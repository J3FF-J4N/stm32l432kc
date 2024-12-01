//! Blinks an LED

#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
extern crate stm32l4xx_hal as hal;

use core::sync::atomic::AtomicBool;

use crate::hal::timer::{Event, Timer};
use cortex_m::peripheral::syst::SystClkSource;
use hal::interrupt;
use hal::pac::NVIC;
// use hal::timer::Timer;
use sh::hprintln;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::rt::entry;
use crate::rt::ExceptionFrame;

static mut LED_STATUS: AtomicBool = core::sync::atomic::AtomicBool::new(false);

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    // Try a different clock configuration
    // let clocks = rcc.cfgr.hclk(4.MHz()).freeze(&mut flash.acr, &mut pwr);

    // let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    // let mut led = gpiob
    //     .pb3
    //     .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);



    // // Timer::tim15(tim, timeout, clocks, apb)

    // let mut timer = dp.TIM1;

    // //Set as upcounter
    // timer.cr1.write(|bit| {
    //     bit.dir().up()
    // });
    // // timer.cnt.write();
    // timer.cr1.write(|w| {
    //     w.cen().set_bit().arpe().disabled().opm().disabled()
    // });

    // let mut timer = Delay::new(cp.SYST, clocks);



    // let mut flash = dp.FLASH.constrain();
    // let mut rcc = dp.RCC.constrain();
    // let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);
    // // let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);
    let clocks = rcc
        .cfgr
        .sysclk(80.MHz())
        .hclk(8.MHz()) //Attached to sysclck (Core clock and SysTick Clock)
        // .pclk1(32.MHz()) //Attached to hclk
        // .pclk2(32.MHz()) //Attached to hclk
        .freeze(&mut flash.acr, &mut pwr);
    cp.SYST.set_clock_source(SystClkSource::Core);
    cp.SYST.set_reload(8_000_000); //Max Value 0x00ffffff 16777215
    cp.SYST.clear_current();
    cp.SYST.enable_counter();
    cp.SYST.enable_interrupt();


    // unsafe { NVIC::unmask(hal::stm32::Interrupt::TIM7) };

    // let mut timer = Timer::tim2(dp.TIM2, 20.Hz(), clocks, &mut rcc.apb1r1);
    // timer.listen(Event::TimeOut);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob
        .pb3
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);


    

    // unsafe {
    //     cortex_m::interrupt::enable();
    // }

    //led.set_high();

    loop {
        // unsafe { hprintln!("{:#?}",LED_STATUS) };
        // timer.delay_ms(1000_u32);

        unsafe {
            if LED_STATUS.load(core::sync::atomic::Ordering::Relaxed) == true {
                led.set_low();
            } else {
                
                led.set_high();
            }
        }
    }
}

#[interrupt]
fn TIM2() {
    hprintln!("Called from IRQ");
    unsafe {
        if LED_STATUS.load(core::sync::atomic::Ordering::Relaxed) == true {
            LED_STATUS.store(false, core::sync::atomic::Ordering::Relaxed);
        } else {
            LED_STATUS.store(true, core::sync::atomic::Ordering::Relaxed);
        }
    }
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

#[exception]
unsafe fn DefaultHandler(id: i16) {

    hprintln!("Called from Defaulthandler");
    //Exception 15
    //IRQ -1

    if id == 15 { //15 is the systick interrupt number

    }

    if LED_STATUS.load(core::sync::atomic::Ordering::Relaxed) == true{
        LED_STATUS.store(false, core::sync::atomic::Ordering::Relaxed);
    } else {
        LED_STATUS.store(true, core::sync::atomic::Ordering::Relaxed);
    }

    // LED_STATUS.get_mut() =

    // cortex_m::interrupt::InterruptNumber::number(self)


    // if id == interrupt::

}

#[exception]
unsafe fn SysTick() {
    // cortex_m::interrupt::disable();

    hprintln!("Exception from SysTick");

    if LED_STATUS.load(core::sync::atomic::Ordering::Relaxed) == true {
        LED_STATUS.store(false, core::sync::atomic::Ordering::Relaxed);
    } else {
        LED_STATUS.store(true, core::sync::atomic::Ordering::Relaxed);
    }

    // cortex_m::interrupt::enable();
}
