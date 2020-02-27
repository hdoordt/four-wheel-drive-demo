#![no_std]
#![no_main]
use cortex_m_rt::{entry, exception, ExceptionFrame};
use stm32f30x_hal::prelude::*;
use embedded_hal::digital::v2::OutputPin;
use core::panic::PanicInfo;
use cortex_m::asm::nop;

#[panic_handler]
unsafe fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    loop {}
}


#[entry]
fn main() -> ! {
    let device_peripherals = stm32f30x::Peripherals::take().unwrap();

    // Reset and clock control register
    let mut rcc = device_peripherals.RCC.constrain();
    // The compass LEDs are all connected to the GPIO peripheral.
    // TODO what is split?
    let mut gpioe = device_peripherals.GPIOE.split(&mut rcc.ahb);
    
    let mut led = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    
    loop {
        for _ in 0..100_000 {
            nop();
        }
        led.set_high().unwrap();
        for _ in 0..100_000 {
            nop();
        }
        led.set_low().unwrap();
    }
}
