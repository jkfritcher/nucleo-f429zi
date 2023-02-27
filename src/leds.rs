// Copyright (c) 2023, Jason Fritcher <jkf@wolfnet.org>
// All rights reserved.

use stm32f4xx_hal::gpio::{
    gpiob::{PB0, PB14, PB7}, Output, PinMode, PushPull,
};

// TODO Try to genericize the below
// Onboard LEDs
pub struct GreenLed {
    led: PB0<Output<PushPull>>,
}
pub struct BlueLed {
    led: PB7<Output<PushPull>>,
}
pub struct RedLed {
    led: PB14<Output<PushPull>>,
}

impl GreenLed {
    pub fn new(pb0: PB0<impl PinMode>) -> Self {
        Self {
            led: pb0.into_push_pull_output(),
        }
    }

    pub fn set_on(&mut self) {
        self.led.set_high();
    }

    pub fn set_off(&mut self) {
        self.led.set_low();
    }

    pub fn toggle(&mut self) {
        self.led.toggle();
    }
}

impl BlueLed {
    pub fn new(pb7: PB7<impl PinMode>) -> Self {
        Self {
            led: pb7.into_push_pull_output(),
        }
    }

    pub fn set_on(&mut self) {
        self.led.set_high();
    }

    pub fn set_off(&mut self) {
        self.led.set_low();
    }

    pub fn toggle(&mut self) {
        self.led.toggle();
    }
}

impl RedLed {
    pub fn new(pb14: PB14<impl PinMode>) -> Self {
        Self {
            led: pb14.into_push_pull_output(),
        }
    }

    pub fn set_on(&mut self) {
        self.led.set_high();
    }

    pub fn set_off(&mut self) {
        self.led.set_low();
    }

    pub fn toggle(&mut self) {
        self.led.toggle();
    }
}
