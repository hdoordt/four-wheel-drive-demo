use stm32f3xx_hal::gpio::{gpioe::*, Output, PushPull};
use stm32f3xx_hal::hal::digital::v2::OutputPin;

pub struct Leds {
    north_west: PE8<Output<PushPull>>,
    north: PE9<Output<PushPull>>,
    north_east: PE10<Output<PushPull>>,
    east: PE11<Output<PushPull>>,
    south_east: PE12<Output<PushPull>>,
    south: PE13<Output<PushPull>>,
    south_west: PE14<Output<PushPull>>,
    west: PE15<Output<PushPull>>,
}

#[derive(Clone, Copy, Debug)]
pub enum Led {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

impl Led {
    fn all() -> [Led; 8] {
        [
            Self::North,
            Self::NorthEast,
            Self::East,
            Self::SouthEast,
            Self::South,
            Self::SouthWest,
            Self::West,
            Self::NorthWest,
        ]
    }
}
impl Leds {
    pub fn init(mut gpioe: Parts) -> Self {
        let north_west = gpioe
            .pe8
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let north = gpioe
            .pe9
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let north_east = gpioe
            .pe10
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let east = gpioe
            .pe11
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let south_east = gpioe
            .pe12
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let south = gpioe
            .pe13
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let south_west = gpioe
            .pe14
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let west = gpioe
            .pe15
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

        Self {
            north_west,
            north,
            north_east,
            east,
            south_east,
            south,
            south_west,
            west,
        }
    }

    pub fn set_all_high(&mut self) -> Result<(), ()> {
        Led::all().iter().try_for_each(|l| self.set_high(*l))
    }

    pub fn set_all_low(&mut self) -> Result<(), ()> {
        Led::all().iter().try_for_each(|l| self.set_low(*l))
    }

    pub fn set_high(&mut self, led: Led) -> Result<(), ()> {
        match led {
            Led::NorthWest => self.north_west.set_high(),
            Led::North => self.north.set_high(),
            Led::NorthEast => self.north_east.set_high(),
            Led::East => self.east.set_high(),
            Led::SouthEast => self.south_east.set_high(),
            Led::South => self.south.set_high(),
            Led::SouthWest => self.south_west.set_high(),
            Led::West => self.west.set_high(),
        }
    }

    pub fn set_low(&mut self, led: Led) -> Result<(), ()> {
        match led {
            Led::NorthWest => self.north_west.set_low(),
            Led::North => self.north.set_low(),
            Led::NorthEast => self.north_east.set_low(),
            Led::East => self.east.set_low(),
            Led::SouthEast => self.south_east.set_low(),
            Led::South => self.south.set_low(),
            Led::SouthWest => self.south_west.set_low(),
            Led::West => self.west.set_low(),
        }
    }

    pub fn blink(&mut self, led: Led, times: u16) -> Result<(), ()> {
        match led {
            Led::NorthWest => blink_led(&mut self.north_west, times),
            Led::North => blink_led(&mut self.north, times),
            Led::NorthEast => blink_led(&mut self.north_east, times),
            Led::East => blink_led(&mut self.east, times),
            Led::SouthEast => blink_led(&mut self.south_east, times),
            Led::South => blink_led(&mut self.south, times),
            Led::SouthWest => blink_led(&mut self.south_west, times),
            Led::West => blink_led(&mut self.west, times),
        }
    }
}

pub struct Dial {
    leds: Leds
}

impl Dial {
    pub fn new(leds: Leds) -> Self {
        Self { leds }
    } 

    pub fn reset(&mut self ) -> Result<(), ()> {
        self.leds.set_all_low()
    }

    pub fn set_magnitude(&mut self, mag: usize) -> Result<(), ()> {
        debug_assert!(mag <= 8);
        self.reset()?;
        Led::all().iter().take(mag).try_for_each(|l| self.leds.set_high(*l))
    }
}

fn blink_led(led: &mut impl OutputPin<Error = ()>, times: u16) -> Result<(), ()> {
    for _ in 0..times {
        led.set_high()?;
        crate::busy_wait(100);
        led.set_low()?;
        crate::busy_wait(100);
    }
    Ok(())
}
