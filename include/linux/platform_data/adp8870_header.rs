/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Definitions and platform data for Analog Devices
 * Backlight drivers ADP8870
 *
 * Copyright 2009-2010 Analog Devices Inc.
 */

pub const ID_ADP8870: u32 = 8870;

pub const ADP8870_MAX_BRIGHTNESS: u8 = 0x7F;
pub const FLAG_OFFT_SHIFT: u32 = 8;

/*
 * LEDs subdevice platform data
 */

pub const ADP8870_LED_DIS_BLINK: u32 = 0 << FLAG_OFFT_SHIFT;
pub const ADP8870_LED_OFFT_600ms: u32 = 1 << FLAG_OFFT_SHIFT;
pub const ADP8870_LED_OFFT_1200ms: u32 = 2 << FLAG_OFFT_SHIFT;
pub const ADP8870_LED_OFFT_1800ms: u32 = 3 << FLAG_OFFT_SHIFT;

pub const ADP8870_LED_ONT_200ms: u32 = 0;
pub const ADP8870_LED_ONT_600ms: u32 = 1;
pub const ADP8870_LED_ONT_800ms: u32 = 2;
pub const ADP8870_LED_ONT_1200ms: u32 = 3;

pub const ADP8870_LED_D7: u32 = 7;
pub const ADP8870_LED_D6: u32 = 6;
pub const ADP8870_LED_D5: u32 = 5;
pub const ADP8870_LED_D4: u32 = 4;
pub const ADP8870_LED_D3: u32 = 3;
pub const ADP8870_LED_D2: u32 = 2;
pub const ADP8870_LED_D1: u32 = 1;

/*
 * Backlight subdevice platform data
 */

pub const ADP8870_BL_D7: u32 = 1 << 6;
pub const ADP8870_BL_D6: u32 = 1 << 5;
pub const ADP8870_BL_D5: u32 = 1 << 4;
pub const ADP8870_BL_D4: u32 = 1 << 3;
pub const ADP8870_BL_D3: u32 = 1 << 2;
pub const ADP8870_BL_D2: u32 = 1 << 1;
pub const ADP8870_BL_D1: u32 = 1 << 0;

pub const ADP8870_FADE_T_DIS: u8 = 0; /* Fade Timer Disabled */
pub const ADP8870_FADE_T_300ms: u8 = 1; /* 0.3 Sec */
pub const ADP8870_FADE_T_600ms: u8 = 2;
pub const ADP8870_FADE_T_900ms: u8 = 3;
pub const ADP8870_FADE_T_1200ms: u8 = 4;
pub const ADP8870_FADE_T_1500ms: u8 = 5;
pub const ADP8870_FADE_T_1800ms: u8 = 6;
pub const ADP8870_FADE_T_2100ms: u8 = 7;
pub const ADP8870_FADE_T_2400ms: u8 = 8;
pub const ADP8870_FADE_T_2700ms: u8 = 9;
pub const ADP8870_FADE_T_3000ms: u8 = 10;
pub const ADP8870_FADE_T_3500ms: u8 = 11;
pub const ADP8870_FADE_T_4000ms: u8 = 12;
pub const ADP8870_FADE_T_4500ms: u8 = 13;
pub const ADP8870_FADE_T_5000ms: u8 = 14;
pub const ADP8870_FADE_T_5500ms: u8 = 15; /* 5.5 Sec */

pub const ADP8870_FADE_LAW_LINEAR: u8 = 0;
pub const ADP8870_FADE_LAW_SQUARE: u8 = 1;
pub const ADP8870_FADE_LAW_CUBIC1: u8 = 2;
pub const ADP8870_FADE_LAW_CUBIC2: u8 = 3;

pub const ADP8870_BL_AMBL_FILT_80ms: u8 = 0; /* Light sensor filter time */
pub const ADP8870_BL_AMBL_FILT_160ms: u8 = 1;
pub const ADP8870_BL_AMBL_FILT_320ms: u8 = 2;
pub const ADP8870_BL_AMBL_FILT_640ms: u8 = 3;
pub const ADP8870_BL_AMBL_FILT_1280ms: u8 = 4;
pub const ADP8870_BL_AMBL_FILT_2560ms: u8 = 5;
pub const ADP8870_BL_AMBL_FILT_5120ms: u8 = 6;
pub const ADP8870_BL_AMBL_FILT_10240ms: u8 = 7; /* 10.24 sec */

/*
 * Blacklight current 0..30mA
 */
#[macro_export]
macro_rules! ADP8870_BL_CUR_mA {
    ($I:expr) => {
        (($I * 127) / 30)
    };
}

/*
 * L2 comparator current 0..1106uA
 */
#[macro_export]
macro_rules! ADP8870_L2_COMP_CURR_uA {
    ($I:expr) => {
        (($I * 255) / 1106)
    };
}

/*
 * L3 comparator current 0..551uA
 */
#[macro_export]
macro_rules! ADP8870_L3_COMP_CURR_uA {
    ($I:expr) => {
        (($I * 255) / 551)
    };
}

/*
 * L4 comparator current 0..275uA
 */
#[macro_export]
macro_rules! ADP8870_L4_COMP_CURR_uA {
    ($I:expr) => {
        (($I * 255) / 275)
    };
}

/*
 * L5 comparator current 0..138uA
 */
#[macro_export]
macro_rules! ADP8870_L5_COMP_CURR_uA {
    ($I:expr) => {
        (($I * 255) / 138)
    };
}

#[repr(C)]
pub struct adp8870_backlight_platform_data {
    pub bl_led_assign: u8, /* 1 = Backlight 0 = Individual LED */
    pub pwm_assign: u8, /* 1 = Enables PWM mode */

    pub bl_fade_in: u8, /* Backlight Fade-In Timer */
    pub bl_fade_out: u8, /* Backlight Fade-Out Timer */
    pub bl_fade_law: u8, /* fade-on/fade-off transfer characteristic */

    pub en_ambl_sens: u8, /* 1 = enable ambient light sensor */
    pub abml_filt: u8, /* Light sensor filter time */

    pub l1_daylight_max: u8, /* use BL_CUR_mA(I) 0 <= I <= 30 mA */
    pub l1_daylight_dim: u8, /* typ = 0, use BL_CUR_mA(I) 0 <= I <= 30 mA */
    pub l2_bright_max: u8, /* use BL_CUR_mA(I) 0 <= I <= 30 mA */
    pub l2_bright_dim: u8, /* typ = 0, use BL_CUR_mA(I) 0 <= I <= 30 mA */
    pub l3_office_max: u8, /* use BL_CUR_mA(I) 0 <= I <= 30 mA */
    pub l3_office_dim: u8, /* typ = 0, use BL_CUR_mA(I) 0 <= I <= 30 mA */
    pub l4_indoor_max: u8, /* use BL_CUR_mA(I) 0 <= I <= 30 mA */
    pub l4_indor_dim: u8, /* typ = 0, use BL_CUR_mA(I) 0 <= I <= 30 mA */
    pub l5_dark_max: u8, /* use BL_CUR_mA(I) 0 <= I <= 30 mA */
    pub l5_dark_dim: u8, /* typ = 0, use BL_CUR_mA(I) 0 <= I <= 30 mA */

    pub l2_trip: u8, /* use L2_COMP_CURR_uA(I) 0 <= I <= 1106 uA */
    pub l2_hyst: u8, /* use L2_COMP_CURR_uA(I) 0 <= I <= 1106 uA */
    pub l3_trip: u8, /* use L3_COMP_CURR_uA(I) 0 <= I <= 551 uA */
    pub l3_hyst: u8, /* use L3_COMP_CURR_uA(I) 0 <= I <= 551 uA */
    pub l4_trip: u8, /* use L4_COMP_CURR_uA(I) 0 <= I <= 275 uA */
    pub l4_hyst: u8, /* use L4_COMP_CURR_uA(I) 0 <= I <= 275 uA */
    pub l5_trip: u8, /* use L5_COMP_CURR_uA(I) 0 <= I <= 138 uA */
    pub l5_hyst: u8, /* use L6_COMP_CURR_uA(I) 0 <= I <= 138 uA */

    /**
     * Independent Current Sinks / LEDS
     * Sinks not assigned to the Backlight can be exposed to
     * user space using the LEDS CLASS interface
     */
    pub num_leds: core::ffi::c_int,
    pub leds: *mut led_info,
    pub led_fade_in: u8, /* LED Fade-In Timer */
    pub led_fade_out: u8, /* LED Fade-Out Timer */
    pub led_fade_law: u8, /* fade-on/fade-off transfer characteristic */
    pub led_on_time: u8,
}

/* Declaration supplied by an external dependency. */
pub enum led_info {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
