/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * leds-bd2802.h - RGB LED Driver
 *
 * Copyright (C) 2009 Samsung Electronics
 * Kim Kyuwon <q1.kim@samsung.com>
 *
 * Datasheet: http://www.rohm.com/products/databook/driver/pdf/bd2802gu-e.pdf
 */

#[repr(C)]
pub struct bd2802_led_platform_data {
    pub rgb_time: u8,
}

#[macro_export]
macro_rules! RGB_TIME {
    ($slopedown:expr, $slopeup:expr, $waveform:expr) => {
        (($slopedown) << 6 | ($slopeup) << 4 | ($waveform))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
