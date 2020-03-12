use core::marker::PhantomData;
use embedded_hal::blocking::i2c::{Write, WriteRead};

pub const PCA_I2C_ADDR: u8 = 0b1000000;

#[allow(non_camel_case_types)]
pub struct PCA9685<TI2C: Write + WriteRead> {
    phantom: PhantomData<TI2C>,
}

impl<TI2C: Write + WriteRead> PCA9685<TI2C> {
    pub fn init(i2c: &mut TI2C) -> Result<Self, <TI2C as Write>::Error> {
        todo!()
    }
}

pub mod register {
    // todo
}
