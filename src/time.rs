use core::cell::RefCell;
use core::ops::DerefMut;
use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m::interrupt::free as interrupt_free;
use cortex_m::interrupt::Mutex;

use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::rcc::{Clocks, APB1};
use stm32f3xx_hal::stm32;
use stm32f3xx_hal::timer::{Event, Timer};

use stm32::{interrupt, Interrupt};

static CURRENT_TIME: AtomicU32 = AtomicU32::new(0);
static mut CURRENT_TIME_UNSAFE: u32 = 0;

static TIMER_TIM7: Mutex<RefCell<Option<Timer<stm32::TIM7>>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn TIM7() {
    interrupt_free(|cs| {
        if let Some(ref mut tim7) = TIMER_TIM7.borrow(cs).borrow_mut().deref_mut() {
            tim7.clear_update_interrupt_flag();
        }
        CURRENT_TIME.fetch_add(1, Ordering::Relaxed);
    });
}
static TIOC: AtomicU32 = AtomicU32::new(0);

#[derive(Copy, Clone)]
pub struct Time {}

impl Time {
    pub fn init(tim7: stm32::TIM7, clocks: Clocks, apb1: &mut APB1) -> Self {
        let mut tim7 = Timer::tim7(tim7, 1.khz(), clocks, apb1);
        tim7.listen(Event::Update);

        interrupt_free(|cs| {
            TIMER_TIM7.borrow(cs).replace(Some(tim7));
        });

        unsafe {
            stm32::NVIC::unmask(Interrupt::TIM7);
        }

        Self {}
    }

    pub fn current_time(self) -> u32 {
        CURRENT_TIME.load(Ordering::Relaxed)
    }
}

pub struct Delay {
    target_time: u32,
}

impl Delay {
    pub fn new(delay_ms: u32, time: Time) -> Self {
        Self {
            target_time: time.current_time() + delay_ms,
        }
    }

    pub fn poll(&self, time: Time) -> nb::Result<(), core::convert::Infallible> {
        if self.target_time <= time.current_time() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

pub struct Interval {
    interval_ms: u32,
    delay: Delay,
}

impl Interval{
    pub fn new(interval_ms: u32, time: Time) -> Self {
        Self {
            interval_ms,
            delay: Delay::new(interval_ms, time)
        }
    }

    #[allow(unused_variables)]
    pub fn poll(&self, time: Time) -> nb::Result<bool, core::convert::Infallible> {
        todo!()
    }
}
