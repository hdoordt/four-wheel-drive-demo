use embedded_hal::blocking::i2c::{Write, WriteRead};

use crate::driver::compass::Dial;
use crate::driver::lsm::{acc, mag};
use crate::driver::motor;
use crate::driver::pca;
use crate::time::{Time};

pub struct Context<TI2C> {
    time: Time,
    i2c: TI2C,
    dial: Dial,
    mag: mag::LSM303LDHC_MAG<TI2C>,
    acc: acc::LSM303LDHC_ACC<TI2C>,
    motors: motor::Motors<TI2C>,
}

impl<TI2C: WriteRead + Write> Context<TI2C> {
    pub fn init(mut i2c: TI2C, time: Time, dial: Dial) -> Result<Self, <TI2C as Write>::Error> {
        let mag = mag::LSM303LDHC_MAG::init(&mut i2c, mag::DataRate::Rate75Hz)?;
        let acc = acc::LSM303LDHC_ACC::init(&mut i2c, acc::DataRate::Rate400Hz)?;
        let pca = pca::PCA9685::init(&mut i2c)?;
        let motors = motor::Motors::init(pca, &mut i2c)?;

        Ok(Self {
            time,
            i2c,
            dial,
            mag,
            acc,
            motors,
        })
    }
}

pub struct Dispatch<TI2C> {
    context: Context<TI2C>,
}

impl<TI2C: Write + WriteRead> Dispatch<TI2C> {
    pub fn init(context: Context<TI2C>) -> Self {
        Self { context }
    }

    pub fn run() -> ! {
        loop {
            // TODO
        }
    }
}
