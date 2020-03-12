// The no_std and no_main directives you read about.
#![no_std]
#![no_main]

// Import a directive which marks the entrypoint of the application.
use cortex_m_rt::entry;
// Import the embedded_hal trait implementations for the STM32F303.
use leds::Leds;
use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::serial::Serial;
mod leds;
mod lsm;
mod pca;
mod usart;

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
    use leds::Led::*;

    // Get a handle to the peripherals.
    let peripherals = stm32f3xx_hal::stm32::Peripherals::take().unwrap();

    // Reset and clock control register
    let mut rcc = peripherals.RCC.constrain();

    let mut flash = peripherals.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = peripherals.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = peripherals.GPIOB.split(&mut rcc.ahb);
    let gpioe = peripherals.GPIOE.split(&mut rcc.ahb);

    let mut leds = Leds::init(gpioe);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let usart1_txd = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    let usart1_rxd = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

    let (usart1_tx, _) = stm32f3xx_hal::serial::Serial::usart1(
        peripherals.USART1,
        (usart1_txd, usart1_rxd),
        115_200.bps(),
        clocks,
        &mut rcc.apb2,
    )
    .split();

    let mut _usart_write = usart::UsartWrite::init(usart1_tx);

    leds.blink(North, 2).unwrap();
    let mut i2c1 = stm32f3xx_hal::i2c::I2c::i2c1(
        peripherals.I2C1,
        (scl, sda),
        400.khz(),
        clocks,
        &mut rcc.apb1,
    );

    // let mut mag = lsm::mag::LSM303LDHC_MAG::init(&mut i2c1).unwrap();
    let mut acc = lsm::acc::LSM303LDHC_ACC::init(&mut i2c1, lsm::acc::DataRate::Rate400Hz).unwrap();
    let mut dial = leds::Dial::new(leds);
    // Loop forever
    loop {
        let lsm::acc::AccData {y,..} = acc.read_sample(&mut i2c1).unwrap();
        
        let mag = y.abs() / 8192;
        dial.set_magnitude(mag as usize).unwrap();
        
        

        // leds.blink(North, 1).unwrap();
        // leds.blink(NorthEast, 1).unwrap();
        // leds.blink(East, 1).unwrap();
        // leds.blink(SouthEast, 1).unwrap();
        // leds.blink(South, 1).unwrap();
        // leds.blink(SouthWest, 1).unwrap();
        // leds.blink(West, 1).unwrap();
        // leds.blink(NorthWest, 1).unwrap();
    }
}
