// The no_std and no_main directives you read about.
#![no_std]
#![no_main]
#![feature(never_type)]
#![allow(dead_code)]

// Import a directive which marks the entrypoint of the application.
use cortex_m_rt::entry;

use stm32f3xx_hal::prelude::*;
mod driver;
pub mod time;
mod usart;
mod dispatch;
mod tasks;

use driver::compass::Compass;

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
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let gpioa = peripherals.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = peripherals.GPIOB.split(&mut rcc.ahb);
    let gpioe = peripherals.GPIOE.split(&mut rcc.ahb);

    let mut compass = Compass::init(gpioe);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    compass.blink(North, 2).unwrap();
    let mut i2c1 = stm32f3xx_hal::i2c::I2c::i2c1(
        peripherals.I2C1,
        (scl, sda),
        400.khz(),
        clocks,
        &mut rcc.apb1,
    );
    let t = time::Time::init(peripherals.TIM7, clocks, &mut rcc.apb1);
    let mut dial = driver::compass::Dial::new(compass);
    
    // Loop forever
    loop {
        let d = time::Delay::new(1000, t);
        nb::block!(d.poll(t));
        dial.set_magnitude(8);
        let d = time::Delay::new(1000, t);
        nb::block!(d.poll(t));
        dial.set_magnitude(1);
    }
}
