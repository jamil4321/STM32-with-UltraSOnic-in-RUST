#![no_main]
#![no_std]

use stmlib::{entry, iprint, iprintln, prelude::*};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds, gpioa, rcc, mut itm) = stmlib::init();
    rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());
    gpioa.moder.modify(|_, w| w.moder0().input());
    let mut time = 0;
    let mut clearTime = 0;
    loop {
        delay.delay_ms(500u16);
        leds[0].on();
        delay.delay_us(10_u32);
        leds[0].off();
        while gpioa.idr.read().idr0().bit_is_clear(){
        };
        while gpioa.idr.read().idr0().bit_is_set() {
            iprintln!(&mut itm.stim[0], "Bit set {}ms", time);
            time = time + 1;
        }

        // // else{
        // iprintln!(&mut itm.stim[0], "Bit low");
        // // }

        delay.delay_ms(50_u16);
    }
}

// for curr in 0..8 {
//     let next = (curr + 1) % 8;

//     leds[next].on();
//     delay.delay_ms(ms);
//     leds[curr].off();
//     delay.delay_ms(ms);
// }

// if gpioa.idr.read().idr0().bit_is_set() {
//     iprintln!(&mut itm.stim[0], "Bit set ");
// }
// if gpioa.idr.read().idr0().bit_is_clear() {
//     iprintln!(&mut itm.stim[0], "Bit Clear");
// }
// while gpioa.idr.read().idr0().bit_is_set() {
//     iprintln!(&mut itm.stim[0], "Bit set ");
// }
