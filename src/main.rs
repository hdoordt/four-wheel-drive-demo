// The no_std and no_main directives you read about.
#![no_std]
#![no_main]
#![feature(never_type)]
#![allow(dead_code)]

// Import a directive which marks the entrypoint of the application.
use cortex_m_rt::entry;

use stm32f3xx_hal::prelude::*;

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

    for _ in 0..(ms * 70) {
        
    }
}
#[cfg(not(debug_assertions))]
fn busy_wait(ms: u32) {
    for _ in 0..(ms * 1400) {
        
    }
}

// This is the main function, or entrypoint of our applicaton.
#[entry]
fn main() -> ! {

    // Get a handle to the peripherals.
    let peripherals = stm32f3xx_hal::stm32::Peripherals::take().unwrap();

    // Reset and clock control register
    let mut rcc = peripherals.RCC.constrain();

    let mut flash = peripherals.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let gpioe = peripherals.GPIOE.split(&mut rcc.ahb);

    // TODO find out which pins the LEDs are connected to


    loop {
        // TODO blink an LED    
    }
}
