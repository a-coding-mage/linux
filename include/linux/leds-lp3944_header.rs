/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * leds-lp3944.h - platform data structure for lp3944 led controller
 *
 * Copyright (C) 2009 Antonio Ospite <ospite@studenti.unina.it>
 */

/* Translated from the C header; the original include guard is omitted. */

pub const LP3944_LED0: i32 = 0;
pub const LP3944_LED1: i32 = 1;
pub const LP3944_LED2: i32 = 2;
pub const LP3944_LED3: i32 = 3;
pub const LP3944_LED4: i32 = 4;
pub const LP3944_LED5: i32 = 5;
pub const LP3944_LED6: i32 = 6;
pub const LP3944_LED7: i32 = 7;
pub const LP3944_LEDS_MAX: usize = 8;

pub const LP3944_LED_STATUS_MASK: u32 = 0x03;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp3944_status {
    LP3944_LED_STATUS_OFF = 0x0,
    LP3944_LED_STATUS_ON = 0x1,
    LP3944_LED_STATUS_DIM0 = 0x2,
    LP3944_LED_STATUS_DIM1 = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lp3944_type {
    LP3944_LED_TYPE_NONE,
    LP3944_LED_TYPE_LED,
    LP3944_LED_TYPE_LED_INVERTED,
}

#[repr(C)]
pub struct lp3944_led {
    pub name: *mut core::ffi::c_char,
    pub r#type: lp3944_type,
    pub status: lp3944_status,
}

#[repr(C)]
pub struct lp3944_platform_data {
    pub leds: [lp3944_led; LP3944_LEDS_MAX],
    pub leds_size: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
