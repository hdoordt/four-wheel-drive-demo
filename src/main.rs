// The no_std and no_main directives you read about.
#![no_std]
#![no_main]
#![feature(never_type)]
#![allow(dead_code)]

// Import a directive which marks the entrypoint of the application.
use cortex_m_rt::entry;
//use core::fmt::{self, Write};
use stm32f3xx_hal::prelude::*;
//use stm32f3xx_hal::stm32;

mod dispatch;
mod driver;
mod sample_buffer;
mod tasks;
pub mod time;
mod usart;

use embedded_hal::blocking::i2c::{Write as HalWrite, WriteRead as HalWriteRead};

use driver::compass::Compass;
use driver::motor::Motors;
use sample_buffer::{Sample, SampleBuffer};

const SAMPLES_TO_CHECK: u8 = 5;

// A panic handler is run when the application encounters an error
// it cannot recover from. The handler defines what it should do
// in that case.
#[panic_handler]
unsafe fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    // On panic, first print the error message using semihosting.
    // When that's done, just go do nothing for eternity,
    // or at least until device reset.
    cortex_m_semihosting::hprintln!("ERROR! {:?}", info).unwrap();
    loop {}
}

#[cfg(debug_assertions)]
fn busy_wait(ms: u32) {
    use cortex_m::asm::nop;

    for _ in 0..(ms * 70) {
        nop();
    }
}
#[cfg(not(debug_assertions))]
fn busy_wait(ms: u32) {
    use cortex_m::asm::nop;

    for _ in 0..(ms * 1400) {
        nop();
    }
}

// This is the main function, or entrypoint of our applicaton.
#[entry]
fn main() -> ! {
    use driver::compass::Led::*;
    // Get a handle to the peripherals.
    let peripherals = stm32f3xx_hal::stm32::Peripherals::take().unwrap();

    // Reset and clock control register
    let mut rcc = peripherals.RCC.constrain();

    let mut flash = peripherals.FLASH.constrain();
    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze(&mut flash.acr);

    let mut gpioa = peripherals.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = peripherals.GPIOB.split(&mut rcc.ahb);
    let gpioe = peripherals.GPIOE.split(&mut rcc.ahb);

    let mut compass = Compass::init(gpioe);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let usart1_txd = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    let usart1_rxd = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

    let usart1 = stm32f3xx_hal::serial::Serial::usart1(
        peripherals.USART1,
        (usart1_txd, usart1_rxd),
        9600.bps(),
        clocks,
        &mut rcc.apb2,
    );

    let (usart1_tx, mut _usart1_rx) = usart1.split();

    let mut us = usart::UsartWrite::init(usart1_tx);

    compass.blink(North, 2).unwrap();
    let mut i2c1 = stm32f3xx_hal::i2c::I2c::i2c1(
        peripherals.I2C1,
        (scl, sda),
        400.khz(),
        clocks,
        &mut rcc.apb1,
    );

    let pca = driver::pca::PCA9685::init(&mut i2c1).unwrap();
    let mut motors = Motors::init(pca, &mut i2c1).unwrap();

    let mut mag =
        driver::lsm::mag::LSM303LDHC_MAG::init(&mut i2c1, driver::lsm::mag::DataRate::Rate220Hz)
            .unwrap();

    let mut acc =
        driver::lsm::acc::LSM303LDHC_ACC::init(&mut i2c1, driver::lsm::acc::DataRate::Rate400Hz)
            .unwrap();
    let t = time::Time::init(peripherals.TIM7, clocks, &mut rcc.apb1);
    //let mut dial = driver::compass::Dial::new(compass);

    let mut acc_samples = SampleBuffer::new();

    let mut forward = true;
    compass.set_direction(driver::compass::Led::North).unwrap();

    // Loop forever
    loop {
        //let mag_sample = mag.read_sample(&mut i2c1);
        let acc_sample = acc.read_sample(&mut i2c1).unwrap();

        uprintln!(&mut us, "{:?}", acc_sample);

        let collision_result = has_collided(&acc_sample, &acc_samples.mean_sample());
        if collision_result.0 {
            uprintln!(
                &mut us,
                "Collision! x: {}, y: {}, z: {}",
                collision_result.1.x,
                collision_result.1.y,
                collision_result.1.z
            );

            // Reverse
            forward = !forward;

            //dial.set_magnitude(4).unwrap();
            compass.set_all_high().unwrap();
            stop(&mut i2c1, &mut motors);

            let d = time::Delay::new(1000, t);
            nb::block!(d.poll(t)).unwrap();

            if forward {
                go_forward(&mut i2c1, &mut motors, 0xFFF);
                compass.set_direction(driver::compass::Led::North).unwrap();
            } else {
                go_backward(&mut i2c1, &mut motors, 0xFFF);
                compass.set_direction(driver::compass::Led::South).unwrap();
            }
        }

        acc_samples.push(acc_sample);
    }
}

fn has_collided(a: &Sample, b: &Sample) -> (bool, Sample) {
    let threshold = 10_000;

    let diff_sample = Sample {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    };

    (
        diff_sample.x > threshold || diff_sample.y > threshold || diff_sample.z > threshold,
        diff_sample,
    )
}

fn go_forward<TI2C: HalWrite + HalWriteRead>(
    i2c1: &mut TI2C,
    motors: &mut Motors<TI2C>,
    speed: u16,
) {
    let _ = motors.set_motor_speed(
        i2c1,
        &driver::motor::motor_direction::MotorDirection::flf(),
        speed,
    );
    let _ = motors.set_motor_speed(
        i2c1,
        &driver::motor::motor_direction::MotorDirection::rlf(),
        speed,
    );
    let _ = motors.set_motor_speed(
        i2c1,
        &driver::motor::motor_direction::MotorDirection::frf(),
        speed,
    );
    let _ = motors.set_motor_speed(
        i2c1,
        &driver::motor::motor_direction::MotorDirection::rrf(),
        speed,
    );
}

fn go_backward<TI2C: HalWrite + HalWriteRead>(
    i2c1: &mut TI2C,
    motors: &mut Motors<TI2C>,
    speed: u16,
) {
    let _ = motors.set_motor_speed(
        i2c1,
        &driver::motor::motor_direction::MotorDirection::flb(),
        speed,
    );
    let _ = motors.set_motor_speed(
        i2c1,
        &driver::motor::motor_direction::MotorDirection::rlb(),
        speed,
    );
    let _ = motors.set_motor_speed(
        i2c1,
        &driver::motor::motor_direction::MotorDirection::frb(),
        speed,
    );
    let _ = motors.set_motor_speed(
        i2c1,
        &driver::motor::motor_direction::MotorDirection::rrb(),
        speed,
    );
}

fn stop<TI2C: HalWrite + HalWriteRead>(i2c1: &mut TI2C, motors: &mut Motors<TI2C>) {
    let _ = motors.all_off(i2c1);
}
