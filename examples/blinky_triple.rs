// Copyright (c) 2023, Jason Fritcher <jkf@wolfnet.org>
// All rights reserved.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{pac, prelude::*};

use nucleo_f429zi::{GreenLed, BlueLed, RedLed};

const BLINK_DELAY_MS: u32 = 1000;

#[entry]
fn main() -> ! {
    // Get our peripherals
    let dp = pac::Peripherals::take().expect("Failed to take device peripherals");
    let cp =
        cortex_m::peripheral::Peripherals::take().expect("Failed to take cortex_m peripherals");

    // Set up the system clocks.
    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .bypass_hse_oscillator()
        .sysclk(48.MHz())
        .freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = cp.SYST.delay(&clocks);

    // Get GPIO group
    let gpiob = dp.GPIOB.split();

    // Setup LEDs
    let mut green_led = GreenLed::new(gpiob.pb0);
    let mut blue_led = BlueLed::new(gpiob.pb7);
    let mut red_led = RedLed::new(gpiob.pb14);

    // Main loop
    loop {
        green_led.set_on();
        delay.delay_ms(BLINK_DELAY_MS);
        green_led.set_off();

        blue_led.set_on();
        delay.delay_ms(BLINK_DELAY_MS);
        blue_led.set_off();

        red_led.set_on();
        delay.delay_ms(BLINK_DELAY_MS);
        red_led.set_off();
    }
}
