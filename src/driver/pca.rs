use core::marker::PhantomData;
use embedded_hal::blocking::i2c::{Write, WriteRead};

pub const PCA_I2C_ADDR: u8 = 0b1000000;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct PCA9685<TI2C> {
    phantom: PhantomData<TI2C>,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Led {
    Led0 = register::LED0_ON_L,
    Led1 = register::LED1_ON_L,
    Led2 = register::LED2_ON_L,
    Led3 = register::LED3_ON_L,
    Led4 = register::LED4_ON_L,
    Led5 = register::LED5_ON_L,
    Led6 = register::LED6_ON_L,
    Led7 = register::LED7_ON_L,
    Led8 = register::LED8_ON_L,
    Led9 = register::LED9_ON_L,
    Led10 = register::LED10_ON_L,
    Led11 = register::LED11_ON_L,
    Led12 = register::LED12_ON_L,
    Led13 = register::LED13_ON_L,
    Led14 = register::LED14_ON_L,
    Led15 = register::LED15_ON_L,
    All = register::ALL_LED_ON_L,
}

impl<TI2C: Write + WriteRead> PCA9685<TI2C> {
    pub fn init(i2c: &mut TI2C) -> Result<Self, <TI2C as Write>::Error> {
        i2c.write(PCA_I2C_ADDR, &[register::MODE1, 0b00100001])?;
        Ok(Self {
            phantom: PhantomData,
        })
    }

    pub fn set_multple_pwm(
        &mut self,
        i2c: &mut TI2C,
        leds: &[Led],
        on: u16,
        off: u16,
    ) -> Result<(), <TI2C as Write>::Error> {
        leds.iter()
            .try_for_each(|led| self.set_pwm(i2c, led, on, off))
    }

    pub fn set_pwm(
        &mut self,
        i2c: &mut TI2C,
        led: &Led,
        on: u16,
        off: u16,
    ) -> Result<(), <TI2C as Write>::Error> {
        debug_assert!(off <= 0x0FFF);
        debug_assert!(on <= 0x0FFF);

        let on_l = on as u8;
        let on_h = (on >> 8) as u8;
        let off_l = off as u8;
        let off_h = (off >> 8) as u8;

        i2c.write(PCA_I2C_ADDR, &[*led as u8, on_l, on_h, off_l, off_h])
    }
}

#[allow(dead_code)]
pub mod register {
    pub const MODE1: u8 = 0x00;
    pub const MODE2: u8 = 0x01;

    pub const SUBADR1: u8 = 0x02;
    pub const SUBADR2: u8 = 0x03;
    pub const SUBADR3: u8 = 0x04;

    pub const ALLCALLADR: u8 = 0x05;

    pub const LED0_ON_L: u8 = 0x06;
    pub const LED0_ON_H: u8 = 0x07;
    pub const LED0_OFF_L: u8 = 0x08;
    pub const LED0_OFF_H: u8 = 0x09;

    pub const LED1_ON_L: u8 = 0x0A;
    pub const LED1_ON_H: u8 = 0x0B;
    pub const LED1_OFF_L: u8 = 0x0C;
    pub const LED1_OFF_H: u8 = 0x0D;

    pub const LED2_ON_L: u8 = 0x0E;
    pub const LED2_ON_H: u8 = 0x0F;
    pub const LED2_OFF_L: u8 = 0x10;
    pub const LED2_OFF_H: u8 = 0x11;

    pub const LED3_ON_L: u8 = 0x12;
    pub const LED3_ON_H: u8 = 0x13;
    pub const LED3_OFF_L: u8 = 0x14;
    pub const LED3_OFF_H: u8 = 0x15;

    pub const LED4_ON_L: u8 = 0x16;
    pub const LED4_ON_H: u8 = 0x17;
    pub const LED4_OFF_L: u8 = 0x18;
    pub const LED4_OFF_H: u8 = 0x19;

    pub const LED5_ON_L: u8 = 0x1A;
    pub const LED5_ON_H: u8 = 0x1B;
    pub const LED5_OFF_L: u8 = 0x1C;
    pub const LED5_OFF_H: u8 = 0x1D;

    pub const LED6_ON_L: u8 = 0x1E;
    pub const LED6_ON_H: u8 = 0x1F;
    pub const LED6_OFF_L: u8 = 0x20;
    pub const LED6_OFF_H: u8 = 0x21;

    pub const LED7_ON_L: u8 = 0x22; // 0xFF
    pub const LED7_ON_H: u8 = 0x23; // 0x0F
    pub const LED7_OFF_L: u8 = 0x24; // 0x000
    pub const LED7_OFF_H: u8 = 0x25;

    pub const LED8_ON_L: u8 = 0x26;
    pub const LED8_ON_H: u8 = 0x27;
    pub const LED8_OFF_L: u8 = 0x28;
    pub const LED8_OFF_H: u8 = 0x29;

    pub const LED9_ON_L: u8 = 0x2A;
    pub const LED9_ON_H: u8 = 0x2B;
    pub const LED9_OFF_L: u8 = 0x2C;
    pub const LED9_OFF_H: u8 = 0x2D;

    pub const LED10_ON_L: u8 = 0x2E;
    pub const LED10_ON_H: u8 = 0x2F;
    pub const LED10_OFF_L: u8 = 0x30;
    pub const LED10_OFF_H: u8 = 0x31;

    pub const LED11_ON_L: u8 = 0x32;
    pub const LED11_ON_H: u8 = 0x33;
    pub const LED11_OFF_L: u8 = 0x34;
    pub const LED11_OFF_H: u8 = 0x35;

    pub const LED12_ON_L: u8 = 0x36;
    pub const LED12_ON_H: u8 = 0x37;
    pub const LED12_OFF_L: u8 = 0x38;
    pub const LED12_OFF_H: u8 = 0x39;

    pub const LED13_ON_L: u8 = 0x3A;
    pub const LED13_ON_H: u8 = 0x3B;
    pub const LED13_OFF_L: u8 = 0x3C;
    pub const LED13_OFF_H: u8 = 0x3D;

    pub const LED14_ON_L: u8 = 0x3E;
    pub const LED14_ON_H: u8 = 0x3F;
    pub const LED14_OFF_L: u8 = 0x40;
    pub const LED14_OFF_H: u8 = 0x41;

    pub const LED15_ON_L: u8 = 0x42;
    pub const LED15_ON_H: u8 = 0x43;
    pub const LED15_OFF_L: u8 = 0x44;
    pub const LED15_OFF_H: u8 = 0x45;

    pub const ALL_LED_ON_L: u8 = 0xFA;
    pub const ALL_LED_ON_H: u8 = 0xFB;
    pub const ALL_LED_OFF_L: u8 = 0xFC;
    pub const ALL_LED_OFF_H: u8 = 0xFD;

    pub const PRE_SCALE: u8 = 0xFE;
    pub const TEST_MODE: u8 = 0xFF;
}
