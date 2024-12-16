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
use core::u16;


use hal::interrupt;


use hal::pac;
// use hal::timer::Timer;
use sh::hprintln;


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


    // cp.SYST.set_clock_source(SystClkSource::Core);
    // cp.SYST.set_reload(8_000_000); //Max Value 0x00ffffff 16777215
    // cp.SYST.clear_current();
    // cp.SYST.enable_counter();
    // cp.SYST.enable_interrupt();


    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob
        .pb3
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);


    // unsafe { NVIC::unmask(hal::stm32::Interrupt::TIM7) };

    let clocks = rcc
    .cfgr
    // .sysclk(80.MHz())
    // .hclk(8.MHz()) //Attached to sysclck (Core clock and SysTick Clock)
    // .pclk1(8.MHz()) //Attached to hclk
    // .pclk2(8.MHz()) //Attached to hclk
    .freeze(&mut flash.acr, &mut pwr);

    let tim15_period = u16::MAX;

    let mut timer = hal::timer::Timer::tim2(dp.TIM2, 1.Hz(), clocks, &mut rcc.apb1r1);
    
    // hal::timer::Timer::<TIM2>::start(&mut timer, 1.Hz());


    timer.clear_interrupt(hal::timer::Event::TimeOut);
    timer.listen(hal::timer::Event::TimeOut);

    timer.start(1.Hz());


    hprintln!("Init done!");

    unsafe {
        cortex_m::peripheral::NVIC::unmask(hal::pac::Interrupt::TIM2);
    }

    //led.set_high();

    loop {
        // timer.clear_interrupt(Event::TimeOut);
        // hprintln!("{:#?}", hal::timer::Timer::<TIM15>::count() as u16);
        // if let Ok(()) = timer.wait(){
        //     hprintln!("Triggered");
        // }
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
    hprintln!("Called from tim 2 IRQ");
    let dp = unsafe { pac::Peripherals::steal() };

    // Clear the interrupt flag for TIM2
    dp.TIM2.sr.modify(|_, w| w.uif().clear_bit());
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

    if id == 15 { 

    }

    if LED_STATUS.load(core::sync::atomic::Ordering::Relaxed) == true{
        LED_STATUS.store(false, core::sync::atomic::Ordering::Relaxed);
    } else {
        LED_STATUS.store(true, core::sync::atomic::Ordering::Relaxed);
    }

}

#[exception]
unsafe fn SysTick() {

    hprintln!("Exception from SysTick");

    if LED_STATUS.load(core::sync::atomic::Ordering::Relaxed) == true {
        LED_STATUS.store(false, core::sync::atomic::Ordering::Relaxed);
    } else {
        LED_STATUS.store(true, core::sync::atomic::Ordering::Relaxed);
    }

}
