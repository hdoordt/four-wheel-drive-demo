use crate::driver::pca::PCA9685;
use embedded_hal::blocking::i2c::{Write, WriteRead};

use motor_direction::*;
pub struct Motors<TI2C> {
    pca: PCA9685<TI2C>,
}

impl<TI2C: Write + WriteRead> Motors<TI2C> {
    pub fn init(pca: PCA9685<TI2C>, i2c: &mut TI2C) -> Result<Self, <TI2C as Write>::Error> {
        let mut motors = Self { pca };
        motors.all_off(i2c)?;
        Ok(motors)
    }

    /// Set a motor's speed in the given direction.
    /// Switches off the opposite direction, as a motor can't go both
    /// forward and backward at the same time
    pub fn set_motor_speed(
        &mut self,
        i2c: &mut TI2C,
        motor_dir: &MotorDirection,
        speed: u16,
    ) -> Result<(), <TI2C as Write>::Error> {
        self.pca
            .set_pwm(i2c, &motor_dir.ant().into(), 0x000, 0xFFF)?;
        self.pca
            .set_pwm(i2c, &(*motor_dir).into(), speed, 0xFFF - speed)
    }

    pub fn all_off(&mut self, i2c: &mut TI2C) -> Result<(), <TI2C as Write>::Error> {
        self.pca
            .set_pwm(i2c, &MotorDirection::flf().into(), 0x000, 0xFFF)?;
        self.pca
            .set_pwm(i2c, &MotorDirection::flb().into(), 0x000, 0xFFF)?;

        self.pca
            .set_pwm(i2c, &MotorDirection::frf().into(), 0x000, 0xFFF)?;
        self.pca
            .set_pwm(i2c, &MotorDirection::frb().into(), 0x000, 0xFFF)?;

        self.pca
            .set_pwm(i2c, &MotorDirection::rlf().into(), 0x000, 0xFFF)?;
        self.pca
            .set_pwm(i2c, &MotorDirection::rlb().into(), 0x000, 0xFFF)?;

        self.pca
            .set_pwm(i2c, &MotorDirection::rrf().into(), 0x000, 0xFFF)?;
        self.pca
            .set_pwm(i2c, &MotorDirection::rrb().into(), 0x000, 0xFFF)
    }
}

pub mod motor_direction {
    use crate::driver::pca;

    #[derive(Copy, Clone, Debug)]
    pub enum Motor {
        FrontLeft,
        FrontRight,
        RearLeft,
        RearRight,
    }

    #[derive(Copy, Clone, Debug)]
    pub enum Direction {
        Forward,
        Backward,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct MotorDirection {
        motor: Motor,
        dir: Direction,
    }

    impl MotorDirection {
        /// The MotorDirection's antagonist
        pub fn ant(&self) -> Self {
            match self.dir {
                Direction::Forward => Self {
                    motor: self.motor,
                    dir: Direction::Backward,
                },
                Direction::Backward => Self {
                    motor: self.motor,
                    dir: Direction::Forward,
                },
            }
        }

        pub fn flf() -> Self {
            (Motor::FrontLeft, Direction::Forward).into()
        }
        pub fn flb() -> Self {
            (Motor::FrontLeft, Direction::Backward).into()
        }
        pub fn frf() -> Self {
            (Motor::FrontRight, Direction::Forward).into()
        }
        pub fn frb() -> Self {
            (Motor::FrontRight, Direction::Backward).into()
        }
        pub fn rlf() -> Self {
            (Motor::RearLeft, Direction::Forward).into()
        }
        pub fn rlb() -> Self {
            (Motor::RearLeft, Direction::Backward).into()
        }
        pub fn rrf() -> Self {
            (Motor::RearRight, Direction::Forward).into()
        }
        pub fn rrb() -> Self {
            (Motor::RearRight, Direction::Backward).into()
        }
    }

    impl From<(Motor, Direction)> for MotorDirection {
        fn from((motor, dir): (Motor, Direction)) -> Self {
            Self { motor, dir }
        }
    }

    impl From<MotorDirection> for pca::Led {
        fn from(motor_dir: MotorDirection) -> Self {
            use crate::driver::pca::Led::*;
            use Direction::*;
            use Motor::*;
            let MotorDirection { motor, dir } = motor_dir;
            match (motor, dir) {
                (FrontLeft, Forward) => Led2,
                (FrontLeft, Backward) => Led3,
                (FrontRight, Forward) => Led0,
                (FrontRight, Backward) => Led1,
                (RearLeft, Forward) => Led4,
                (RearLeft, Backward) => Led5,
                (RearRight, Forward) => Led7,
                (RearRight, Backward) => Led6,
            }
        }
    }

    /* NOTE: Black swag mobiel mapping
        impl From<MotorDirection> for pca::Led {
        fn from(motor_dir: MotorDirection) -> Self {
        use crate::driver::pca::Led::*;
        use Direction::*;
        use Motor::*;
        let MotorDirection { motor, dir } = motor_dir;
        match (motor, dir) {
        (FrontLeft, Forward) => Led0,
        (FrontLeft, Backward) => Led1,
        (FrontRight, Forward) => Led7,
        (FrontRight, Backward) => Led6,
        (RearLeft, Forward) => Led2,
        (RearLeft, Backward) => Led3,
        (RearRight, Forward) => Led4,
        (RearRight, Backward) => Led5,
    }
    }
    }
         */
}
