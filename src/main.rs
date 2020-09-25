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

mod driver;
mod time;
mod usart;

use embedded_hal::blocking::i2c::{Write as HalWrite, WriteRead as HalWriteRead};

use driver::compass::Compass;
use driver::motor::Motors;

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

    // Init the compass using the GPIOE block
    let mut compass = Compass::init(gpioe);

    // // Configure USART pins
    // let usart1_txd = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    // let usart1_rxd = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

    // // Configure USART1 peripheral
    // let usart1 = stm32f3xx_hal::serial::Serial::usart1(
    //     peripherals.USART1,
    //     (usart1_txd, usart1_rxd),
    //     9600.bps(),
    //     clocks,
    //     &mut rcc.apb2,
    // );

    // // Split USART1 peripheral in transmitter and receiver
    // let (usart1_tx, mut _usart1_rx) = usart1.split();
    // // Create wrapper around USART transmitter for ease of use
    // let mut usart = usart::UsartWrite::init(usart1_tx);

    // // Blink North LED on compass to show that we've come this far
    // compass.blink(North, 2).unwrap();

    // // Configure I2C pins
    // let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    // let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    // // Configure I2C1 peripherals
    // let mut i2c1 = stm32f3xx_hal::i2c::I2c::i2c1(
    //     peripherals.I2C1,
    //     (scl, sda),
    //     400.khz(),
    //     clocks,
    //     &mut rcc.apb1,
    // );

    // // Initialize PCA9685 PWM driver using the I2C1 peripheral
    // let pca = driver::pca::PCA9685::init(&mut i2c1).unwrap();

    // // Initialize the Motors wrapper around the PWM driver to ease controlling the motors
    // let mut motors = Motors::init(pca, &mut i2c1).unwrap();

    // // Inititialize the LSM303LDHC magnetometor driver using the I2C1 peripheral
    // let mut mag =
    //     driver::lsm::mag::LSM303LDHC_MAG::init(&mut i2c1, driver::lsm::mag::DataRate::Rate220Hz)
    //         .unwrap();

    // // Inititialize the LSM303LDHC accelerometer driver using the I2C1 peripheral
    // let mut acc =
    //     driver::lsm::acc::LSM303LDHC_ACC::init(&mut i2c1, driver::lsm::acc::DataRate::Rate400Hz)
    //         .unwrap();

    // Loop forever
    loop {
        // TODO: Do something cool with the provided hardware
        compass.set_all_high().unwrap();
        busy_wait(500);
        compass.set_all_low().unwrap();
        busy_wait(500);
    }
}
