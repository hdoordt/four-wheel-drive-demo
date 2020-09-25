use crate::sample_buffer::Sample;
use core::convert::TryInto;
use core::marker::PhantomData;
use embedded_hal::blocking::i2c::{Write, WriteRead};

pub const LSM_ACC_I2C_ADDR: u8 = 0b0011001;

#[allow(non_camel_case_types)]
pub struct LSM303LDHC_ACC<TI2C> {
    phantom: PhantomData<TI2C>,
}

#[derive(Clone, Debug)]
#[repr(u8)]
#[allow(dead_code)]
pub enum DataRate {
    PowerDown = 0b0000 << 4,
    Rate1hz = 0b0001 << 4,
    Rate10Hz = 0b0010 << 4,
    Rate25Hz = 0b0011 << 4,
    Rate50Hz = 0b0100 << 4,
    Rate100Hz = 0b0101 << 4,
    Rate200Hz = 0b0110 << 4,
    Rate400Hz = 0b0111 << 4,
}

impl<TI2C: WriteRead + Write> LSM303LDHC_ACC<TI2C> {
    pub fn init(i2c: &mut TI2C, data_rate: DataRate) -> Result<Self, <TI2C as Write>::Error> {
        // set data rate (table 20)
        // Optionally set low-power mode, z, y , x axes enamble

        i2c.write(
            LSM_ACC_I2C_ADDR,
            &[register::CTRL_REG1_A, data_rate as u8 | 0b0111],
        )?;

        Ok(Self {
            phantom: PhantomData,
        })
    }

    pub fn read_sample(&mut self, i2c: &mut TI2C) -> Result<Sample, <TI2C as WriteRead>::Error> {
        use register::OUT_X_L_A;

        let mut buf = [0u8; 6];

        // We bitwise set the last bit to 1 to enable auto-increment
        i2c.write_read(LSM_ACC_I2C_ADDR, &[OUT_X_L_A | 0b0000001], &mut buf)?;

        let x = i16::from_le_bytes(buf[0..=1].try_into().unwrap());
        let y = i16::from_le_bytes(buf[2..=3].try_into().unwrap());
        let z = i16::from_le_bytes(buf[4..=5].try_into().unwrap());

        Ok(Sample { x, y, z })
    }
    /*
    pub fn read_sample(&mut self, i2c: &mut TI2C) -> Result<Sample, <TI2C as WriteRead>::Error> {
        use register::OUT_X_L_A;
        use register::OUT_Y_L_A;
        use register::OUT_Z_L_A;

        use register::OUT_X_H_A;
        use register::OUT_Y_H_A;
        use register::OUT_Z_H_A;

        let mut buf_x_l = [0u8; 1];
        let mut buf_y_l = [0u8; 1];
        let mut buf_z_l = [0u8; 1];

        let mut buf_x_h = [0u8; 1];
        let mut buf_y_h = [0u8; 1];
        let mut buf_z_h = [0u8; 1];

        i2c.write_read(LSM_ACC_I2C_ADDR, &[OUT_X_L_A], &mut buf_x_l)?;
        i2c.write_read(LSM_ACC_I2C_ADDR, &[OUT_Y_L_A], &mut buf_y_l)?;
        i2c.write_read(LSM_ACC_I2C_ADDR, &[OUT_Z_L_A], &mut buf_z_l)?;

        i2c.write_read(LSM_ACC_I2C_ADDR, &[OUT_X_H_A], &mut buf_x_h)?;
        i2c.write_read(LSM_ACC_I2C_ADDR, &[OUT_Y_H_A], &mut buf_y_h)?;
        i2c.write_read(LSM_ACC_I2C_ADDR, &[OUT_Z_H_A], &mut buf_z_h)?;

        let buf_x = [buf_x_l[0], buf_x_h[0]];
        let buf_y = [buf_y_l[0], buf_y_h[0]];
        let buf_z = [buf_z_l[0], buf_z_h[0]];

        let x = i16::from_le_bytes(buf_x);
        let y = i16::from_le_bytes(buf_y);
        let z = i16::from_le_bytes(buf_z);

        Ok(Sample { x, y, z })
    }
    */
}

#[allow(dead_code)]
pub mod register {
    pub const CTRL_REG1_A: u8 = 0x20;
    pub const CTRL_REG2_A: u8 = 0x21;
    pub const CTRL_REG3_A: u8 = 0x22;
    pub const CTRL_REG4_A: u8 = 0x23;
    pub const CTRL_REG5_A: u8 = 0x24;
    pub const CTRL_REG6_A: u8 = 0x25;

    pub const REFERENCE_A: u8 = 0x26;
    pub const STATUS_REG_A: u8 = 0x27;

    pub const OUT_X_L_A: u8 = 0x28;
    pub const OUT_X_H_A: u8 = 0x29;
    pub const OUT_Y_L_A: u8 = 0x2A;
    pub const OUT_Y_H_A: u8 = 0x2B;
    pub const OUT_Z_L_A: u8 = 0x2C;
    pub const OUT_Z_H_A: u8 = 0x2D;

    pub const FIFO_CTRL_REG_A: u8 = 0x2E;
    pub const FIFO_SRC_REG_A: u8 = 0x2F;

    pub const INT1_CFG_A: u8 = 0x30;
    pub const INT1_SRC_A: u8 = 0x31;
    pub const INT1_THS_A: u8 = 0x32;
    pub const INT1_DURATION_A: u8 = 0x33;

    pub const INT2_CFG_A: u8 = 0x34;
    pub const INT2_SRC_A: u8 = 0x35;
    pub const INT2_THS_A: u8 = 0x36;
    pub const INT2_DURATION_A: u8 = 0x37;

    pub const CLICK_CFG_A: u8 = 0x38;
    pub const CLICK_SRC_A: u8 = 0x39;
    pub const CLICK_THS_A: u8 = 0x3A;

    pub const TIME_LIMIT_A: u8 = 0x3B;
    pub const TIME_LATENCY_A: u8 = 0x3C;
    pub const TIME_WINDOW_A: u8 = 0x3;
}
