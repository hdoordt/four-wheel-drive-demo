// The no_std and no_main directives you read about.
#![no_std]
#![no_main]

// Import a directive which marks the entrypoint of the application.
use cortex_m_rt::entry;

// Import the embedded_hal trait implementations for the STM32F303.
use stm32f3xx_hal::prelude::*;

// We need to explicitly import the v2 OutputPin trait.
// No means No-Op. It's an operation which does nothing.
// We use it in the busy waiting loops to notify Rust that it
// should not optimize these loops out.
use cortex_m::asm::nop;

// A panic handler is run when the application encounters an error
// it cannot recover from. The handler defines what it should do
// in that case.
#[panic_handler]
unsafe fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    // On panic, just go do nothing for eternity,
    // or at least until device reset.
    loop {}
}

// This is the main function, or entrypoint of our applicaton.
#[entry]
fn main() -> ! {
    // Get a handle to the peripherals. Safe Rust allows only a single instance
    // of this handle. That way, accidental concurrent access is avoided.
    let peripherals = stm32f3xx_hal::stm32::Peripherals::take().unwrap();

    // Reset and clock control register. Among other things, this register
    // is for enabling the General Purpose Input/Output peripherals.
    // We constrain full access to the RCC, allowing access per part instead.
    // Individual modules can configure individual parts of the RCC independently
    // from now on. This gets more important in larger applications.
    let mut rcc = peripherals.RCC.constrain();

    // The compass LEDs are all connected to the GPIO E peripheral.
    // Splitting the GPIO provides access to each of the individual pins,
    // so we can configure each of them independently.
    let mut gpioe = peripherals.GPIOE.split(&mut rcc.ahb);
    
    // The Northern LED is connected to pin pe9. To use it four our purpose,
    // we configure it to a push-pull output.
    let mut led = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    
    // Loop forever
    loop {
        // Wait a couple of cycles
        for _ in 0..100_000 {
            nop();
        }
        // Enable the LED
        led.set_high().unwrap();
        // Wait some more
        for _ in 0..100_000 {
            nop();
        }
        // Disable the LED
        led.set_low().unwrap();
    }
}
