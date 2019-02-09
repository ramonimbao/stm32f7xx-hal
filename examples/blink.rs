//! Blinks the backlight of the LCD

#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
extern crate stm32f7xx_hal as hal;

//use cortex_m::asm;
use crate::hal::prelude::*;
use crate::hal::delay::Delay;
//use crate::sh::hio;
//use core::fmt::Write;
use crate::rt::ExceptionFrame;

#[entry]
fn main() -> ! {
    //let mut hstdout = hio::hstdout().unwrap();

    //writeln!(hstdout, "Hello world!").unwrap();

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Calling this enables HSI and sets it as the system clock.
    // TODO: Implement clock configuration
    let _clocks = rcc.cfgr.freeze(&mut flash.acr);

    // PK3 is connected to the LCD backlight
    let mut gpiok = dp.GPIOK.split(&mut rcc.ahb1);
    let mut led = gpiok.pk3.into_push_pull_output(&mut gpiok.moder, &mut gpiok.otyper);

    let mut timer = Delay::new(cp.SYST);
    loop {
        led.set_high();
        timer.delay_ms(1000_u32);
        led.set_low();
        timer.delay_ms(1000_u32);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}