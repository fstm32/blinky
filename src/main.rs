#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f103xx_hal as hal;
#[macro_use]
extern crate nb;

use cortex_m_rt::entry;
use hal::prelude::*;
use hal::stm32f103xx;
use hal::timer::Timer;

#[entry]
fn main() -> ! {

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let mut led = gpioc.pc15.into_push_pull_output(&mut gpioc.crh);

    let mut timer = Timer::syst(cp.SYST, 1.hz(), clocks);

    loop {
        block!(timer.wait()).unwrap();

        led.set_high();

        block!(timer.wait()).unwrap();

        led.set_low();
    }
}