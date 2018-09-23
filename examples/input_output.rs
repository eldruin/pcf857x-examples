//! Uses PCF8575 connected to pins PB6 and PB7 of the STM23F3Discovery
//! board to read the pins P0-P7 and write its state to pins P10-P17.

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate f3;
extern crate panic_semihosting;
extern crate pcf857x;

use f3::hal::delay::Delay;
use f3::hal::prelude::*;
use f3::hal::stm32f30x;
use rt::ExceptionFrame;
use f3::hal::i2c::I2c;
pub use f3::hal::stm32f30x::i2c1;
use pcf857x::{PCF8575, SlaveAddr, PinFlag};

entry!(main);

fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);
    let mut expander = PCF8575::new(i2c, SlaveAddr::default());
    
    loop {
        let inputs = expander.get(&(PinFlag::P0 | PinFlag::P1 | PinFlag::P2 | PinFlag::P3 |
                                    PinFlag::P4 | PinFlag::P5 | PinFlag::P6 | PinFlag::P7)).unwrap();
        expander.set(0b0000_0000_1111_1111 | inputs << 8 ).unwrap();
        delay.delay_ms(100_u16);
    }
}


exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
