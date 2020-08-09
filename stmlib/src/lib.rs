//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_halt; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
pub use f3::hal::stm32f30x::{gpioa, rcc};
pub use f3::{
    hal::{delay::Delay, prelude,time::{Hertz, MonoTimer},
    timer::Timer,},
    led::Leds,
};

use f3::hal::prelude::*;
use f3::hal::stm32f30x::{self, GPIOA, RCC};

pub fn init() -> (
    Delay,
    Leds,
    &'static gpioa::RegisterBlock,
    &'static rcc::RegisterBlock,
    ITM,
    MonoTimer
) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let delay = Delay::new(cp.SYST, clocks);

    let leds = Leds::new(dp.GPIOE.split(&mut rcc.ahb));
    // let timmer = Timer::tim7(dp.TIM7, Hertz(1), clocks, &mut rcc.apb1);

    unsafe { (delay, leds, &*GPIOA::ptr(), &*RCC::ptr(), cp.ITM, MonoTimer::new(cp.DWT, clocks),) }
}
