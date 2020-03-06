#![no_main]
#![no_std]

use stmlib::{entry, iprint, iprintln, prelude::*};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds, gpioa, rcc, mut itm, mono_timer) = stmlib::init();
    rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());
    gpioa.moder.modify(|_, w| w.moder0().input());
    loop {
        delay.delay_ms(500u16);
        leds[0].on();
        delay.delay_us(100_u32);
        leds[0].off();
        while gpioa.idr.read().idr0().bit_is_clear() {}
        let instant = mono_timer.now();
        while gpioa.idr.read().idr0().bit_is_set() {}

        let elapsed = instant.elapsed();
        let mut dist = ((elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6) / 2.0) / 29.1;
        if dist <= 3.0 {
            iprintln!(&mut itm.stim[0], "Out of Range");
        } else if dist >= 400.0 {
            iprintln!(&mut itm.stim[0], "Out of Range");
        } else {
            iprintln!(&mut itm.stim[0], "Distance = {} cm", dist);
        }

        delay.delay_ms(50_u16);
    }
}
