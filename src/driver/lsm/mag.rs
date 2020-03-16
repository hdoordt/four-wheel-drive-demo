use core::convert::TryInto;
use core::marker::PhantomData;
use embedded_hal::blocking::i2c::{Write, WriteRead};

pub const LSM_MAG_I2C_ADDR: u8 = 0b001_1110;

#[allow(non_camel_case_types)]
pub struct LSM303LDHC_MAG<TI2C> {
    phantom: PhantomData<TI2C>,
}

#[derive(Clone, Debug)]
#[repr(u8)]
#[allow(dead_code)]
pub enum DataRate {
    Rate0_75hz = 0b000 << 2,
    Rate1_5Hz = 0b001 << 2,
    Rate3Hz = 0b010 << 2,
    Rate7_5Hz = 0b011 << 2,
    Rate15Hz = 0b100 << 2,
    Rate30Hz = 0b101 << 2,
    Rate75Hz = 0b110 << 2,
    Rate220Hz = 0b111 << 2,
}

#[derive(Clone, Debug)]
pub struct MagData {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(Clone, Debug)]
pub struct TempData(pub i16);

impl<TI2C: WriteRead + Write> LSM303LDHC_MAG<TI2C> {
    pub fn init(i2c: &mut TI2C, data_rate: DataRate) -> Result<Self, <TI2C as Write>::Error> {
        i2c.write(LSM_MAG_I2C_ADDR, &[register::MR_REG_M, 0x00])?;
        i2c.write(
            LSM_MAG_I2C_ADDR,
            &[register::CRA_REG_M, data_rate as u8 | 0x80],
        )?;
        Ok(Self {
            phantom: PhantomData,
        })
    }

    pub fn read_reg(&mut self, i2c: &mut TI2C, reg: u8) -> Result<u8, <TI2C as WriteRead>::Error> {
        let mut buf = [0u8];
        i2c.write_read(LSM_MAG_I2C_ADDR, &[reg], &mut buf)?;

        Ok(buf[0])
    }

    pub fn read_sample(&mut self, i2c: &mut TI2C) -> Result<MagData, <TI2C as WriteRead>::Error> {
        let mut buf = [0u8; 6];
        i2c.write_read(LSM_MAG_I2C_ADDR, &[register::OUT_X_H_M], &mut buf)?;

        let x = i16::from_be_bytes(buf[0..=1].try_into().unwrap());
        let z = i16::from_be_bytes(buf[2..=3].try_into().unwrap());
        let y = i16::from_be_bytes(buf[4..=5].try_into().unwrap());

        Ok(MagData { x, y, z })
    }

    pub fn read_temp(&mut self, _i2c: &mut TI2C) -> Result<TempData, <TI2C as WriteRead>::Error> {
        todo!()
    }
}

#[allow(dead_code)]
pub mod register {
    pub const CRA_REG_M: u8 = 0x00;
    pub const CRB_REG_M: u8 = 0x01;
    pub const MR_REG_M: u8 = 0x02;

    pub const OUT_X_H_M: u8 = 0x03;
    pub const OUT_X_L_M: u8 = 0x04;
    pub const OUT_Z_H_M: u8 = 0x05;
    pub const OUT_Z_L_M: u8 = 0x06;
    pub const OUT_Y_H_M: u8 = 0x07;
    pub const OUT_Y_L_M: u8 = 0x08;

    pub const SR_REG_M: u8 = 0x09;
    pub const IRA_REG_M: u8 = 0x0A;
    pub const IRB_REG_M: u8 = 0x0B;
    pub const IRC_REG_M: u8 = 0x0C;

    pub const TEMP_OUT_H_M: u8 = 0x31;
    pub const TEMP_OUT_L_M: u8 = 0x32;
}
