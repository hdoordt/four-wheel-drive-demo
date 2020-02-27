// The no_std and no_main directives you read about.
#![no_std]
#![no_main]

// Import a directive which marks the entrypoint of the application.
use cortex_m_rt::entry;

use core::convert::Infallible;


const PCA_I2C_ADDR:u8 = 0x80;
// Import the embedded_hal trait implementations for the STM32F303.
use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::time::MegaHertz;
use stm32f3xx_hal::hal::digital::v2::OutputPin;


// A panic handler is run when the application encounters an error
// it cannot recover from. The handler defines what it should do
// in that case.
#[panic_handler]
unsafe fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    // On panic, just go do nothing for eternity,
    // or at least until device reset.
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

fn blink_led(led: &mut impl OutputPin<Error = ()>, times: u16) -> Result<(), ()> {
    for _ in 0..times {
        led.set_high()?;
        busy_wait(100);
        led.set_low()?;
        busy_wait(100);
    }
    Ok(())
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

    let mut gpiob = peripherals.GPIOB.split(&mut rcc.ahb);
    let mut gpioe = peripherals.GPIOE.split(&mut rcc.ahb);

    let mut led_n = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_ne = gpioe
        .pe10
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_e = gpioe
        .pe11
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_se = gpioe
        .pe12
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_s = gpioe
        .pe13
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_sw = gpioe
        .pe14
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_w = gpioe
        .pe15
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut led_nw = gpioe
        .pe8
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let mut i2c1 = stm32f3xx_hal::i2c::I2c::i2c1(
        peripherals.I2C1,
        (scl, sda),
        MegaHertz(1),
        clocks,
        &mut rcc.apb1,
    );
    blink_led(&mut led_w, 10).unwrap();

    busy_wait(100);
    i2c1.write(PCA_I2C_ADDR, &[0x00, 0x00]).unwrap();
    blink_led(&mut led_sw, 10).unwrap();
    busy_wait(100);
    i2c1.write(PCA_I2C_ADDR, &[0x01, 0x00]).unwrap();
    blink_led(&mut led_nw, 10).unwrap();

    blink_led(&mut led_s, 2).unwrap();
    match i2c1.write(PCA_I2C_ADDR, &[0x42, 0xFF, 0x0F]) {
        Err(_) => blink_led(&mut led_n, 2).unwrap(),
        Ok(_) => blink_led(&mut led_e, 2).unwrap(),
    }
    
    blink_led(&mut led_s, 2).unwrap();
    match i2c1.write(PCA_I2C_ADDR, &[0x43,  0xFF, 0x0F]) {
        Err(_) => blink_led(&mut led_n, 2).unwrap(),
        Ok(_) => blink_led(&mut led_e, 2).unwrap(),
    }
    
    blink_led(&mut led_s, 2).unwrap();
    match i2c1.write(PCA_I2C_ADDR, &[0x44,  0x00, 0x00, 0x00]) {
        Err(_) => blink_led(&mut led_n, 2).unwrap(),
        Ok(_) => blink_led(&mut led_e, 2).unwrap(),
    }
    
    blink_led(&mut led_s, 2).unwrap();
    match i2c1.write(PCA_I2C_ADDR, &[0x45,  0x00, 0x00, 0x00]) {
        Err(_) => blink_led(&mut led_n, 2).unwrap(),
        Ok(_) => blink_led(&mut led_e, 2).unwrap(),
    }
    
    
    // Loop forever
    loop {
        blink_led(&mut led_n, 1).unwrap();
        blink_led(&mut led_ne, 1).unwrap();
        blink_led(&mut led_e, 1).unwrap();
        blink_led(&mut led_se, 1).unwrap();
        blink_led(&mut led_s, 1).unwrap();
        blink_led(&mut led_sw, 1).unwrap();
        blink_led(&mut led_w, 1).unwrap();
        blink_led(&mut led_nw, 1).unwrap();
    }
}
