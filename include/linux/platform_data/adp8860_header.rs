/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Definitions and platform data for Analog Devices
 * Backlight drivers ADP8860
 *
 * Copyright 2009-2010 Analog Devices Inc.
 */

// C dependencies: linux/leds.h and linux/types.h

pub const ID_ADP8860: i32 = 8860;

pub const ADP8860_MAX_BRIGHTNESS: i32 = 0x7F;
pub const FLAG_OFFT_SHIFT: i32 = 8;

/* LEDs subdevice platform data */

pub const ADP8860_LED_DIS_BLINK: i32 = 0 << FLAG_OFFT_SHIFT;
pub const ADP8860_LED_OFFT_600ms: i32 = 1 << FLAG_OFFT_SHIFT;
pub const ADP8860_LED_OFFT_1200ms: i32 = 2 << FLAG_OFFT_SHIFT;
pub const ADP8860_LED_OFFT_1800ms: i32 = 3 << FLAG_OFFT_SHIFT;

pub const ADP8860_LED_ONT_200ms: i32 = 0;
pub const ADP8860_LED_ONT_600ms: i32 = 1;
pub const ADP8860_LED_ONT_800ms: i32 = 2;
pub const ADP8860_LED_ONT_1200ms: i32 = 3;

pub const ADP8860_LED_D7: i32 = 7;
pub const ADP8860_LED_D6: i32 = 6;
pub const ADP8860_LED_D5: i32 = 5;
pub const ADP8860_LED_D4: i32 = 4;
pub const ADP8860_LED_D3: i32 = 3;
pub const ADP8860_LED_D2: i32 = 2;
pub const ADP8860_LED_D1: i32 = 1;

/* Backlight subdevice platform data */

pub const ADP8860_BL_D7: i32 = 1 << 6;
pub const ADP8860_BL_D6: i32 = 1 << 5;
pub const ADP8860_BL_D5: i32 = 1 << 4;
pub const ADP8860_BL_D4: i32 = 1 << 3;
pub const ADP8860_BL_D3: i32 = 1 << 2;
pub const ADP8860_BL_D2: i32 = 1 << 1;
pub const ADP8860_BL_D1: i32 = 1 << 0;

pub const ADP8860_FADE_T_DIS: i32 = 0; /* Fade Timer Disabled */
pub const ADP8860_FADE_T_300ms: i32 = 1; /* 0.3 Sec */
pub const ADP8860_FADE_T_600ms: i32 = 2;
pub const ADP8860_FADE_T_900ms: i32 = 3;
pub const ADP8860_FADE_T_1200ms: i32 = 4;
pub const ADP8860_FADE_T_1500ms: i32 = 5;
pub const ADP8860_FADE_T_1800ms: i32 = 6;
pub const ADP8860_FADE_T_2100ms: i32 = 7;
pub const ADP8860_FADE_T_2400ms: i32 = 8;
pub const ADP8860_FADE_T_2700ms: i32 = 9;
pub const ADP8860_FADE_T_3000ms: i32 = 10;
pub const ADP8860_FADE_T_3500ms: i32 = 11;
pub const ADP8860_FADE_T_4000ms: i32 = 12;
pub const ADP8860_FADE_T_4500ms: i32 = 13;
pub const ADP8860_FADE_T_5000ms: i32 = 14;
pub const ADP8860_FADE_T_5500ms: i32 = 15; /* 5.5 Sec */

pub const ADP8860_FADE_LAW_LINEAR: i32 = 0;
pub const ADP8860_FADE_LAW_SQUARE: i32 = 1;
pub const ADP8860_FADE_LAW_CUBIC1: i32 = 2;
pub const ADP8860_FADE_LAW_CUBIC2: i32 = 3;

pub const ADP8860_BL_AMBL_FILT_80ms: i32 = 0; /* Light sensor filter time */
pub const ADP8860_BL_AMBL_FILT_160ms: i32 = 1;
pub const ADP8860_BL_AMBL_FILT_320ms: i32 = 2;
pub const ADP8860_BL_AMBL_FILT_640ms: i32 = 3;
pub const ADP8860_BL_AMBL_FILT_1280ms: i32 = 4;
pub const ADP8860_BL_AMBL_FILT_2560ms: i32 = 5;
pub const ADP8860_BL_AMBL_FILT_5120ms: i32 = 6;
pub const ADP8860_BL_AMBL_FILT_10240ms: i32 = 7; /* 10.24 sec */

/* Blacklight current 0..30mA */
#[macro_export]
macro_rules! ADP8860_BL_CUR_mA {
    ($i:expr) => (($i * 127) / 30);
}

/* L2 comparator current 0..1106uA */
#[macro_export]
macro_rules! ADP8860_L2_COMP_CURR_uA {
    ($i:expr) => (($i * 255) / 1106);
}

/* L3 comparator current 0..138uA */
#[macro_export]
macro_rules! ADP8860_L3_COMP_CURR_uA {
    ($i:expr) => (($i * 255) / 138);
}

#[repr(C)]
pub struct adp8860_backlight_platform_data {
    pub bl_led_assign: u8, /* 1 = Backlight 0 = Individual LED */

    pub bl_fade_in: u8, /* Backlight Fade-In Timer */
    pub bl_fade_out: u8, /* Backlight Fade-Out Timer */
    pub bl_fade_law: u8, /* fade-on/fade-off transfer characteristic */

    pub en_ambl_sens: u8, /* 1 = enable ambient light sensor */
    pub abml_filt: u8, /* Light sensor filter time */

    pub l1_daylight_max: u8, /* use BL_CUR_mA(I) 0 <= I <= 30 mA */
    pub l1_daylight_dim: u8, /* typ = 0, use BL_CUR_mA(I) 0 <= I <= 30 mA */
    pub l2_office_max: u8, /* use BL_CUR_mA(I) 0 <= I <= 30 mA */
    pub l2_office_dim: u8, /* typ = 0, use BL_CUR_mA(I) 0 <= I <= 30 mA */
    pub l3_dark_max: u8, /* use BL_CUR_mA(I) 0 <= I <= 30 mA */
    pub l3_dark_dim: u8, /* typ = 0, use BL_CUR_mA(I) 0 <= I <= 30 mA */

    pub l2_trip: u8, /* use L2_COMP_CURR_uA(I) 0 <= I <= 1106 uA */
    pub l2_hyst: u8, /* use L2_COMP_CURR_uA(I) 0 <= I <= 1106 uA */
    pub l3_trip: u8, /* use L3_COMP_CURR_uA(I) 0 <= I <= 551 uA */
    pub l3_hyst: u8, /* use L3_COMP_CURR_uA(I) 0 <= I <= 551 uA */

    /**
     * Independent Current Sinks / LEDS
     * Sinks not assigned to the Backlight can be exposed to
     * user space using the LEDS CLASS interface
     */
    pub num_leds: i32,
    pub leds: *mut led_info,
    pub led_fade_in: u8, /* LED Fade-In Timer */
    pub led_fade_out: u8, /* LED Fade-Out Timer */
    pub led_fade_law: u8, /* fade-on/fade-off transfer characteristic */
    pub led_on_time: u8,

    /**
     * Gain down disable. Setting this option does not allow the
     * charge pump to switch to lower gains. NOT AVAILABLE on ADP8860
     * 1 = the charge pump doesn't switch down in gain until all LEDs are 0.
     *  The charge pump switches up in gain as needed. This feature is
     *  useful if the ADP8863 charge pump is used to drive an external load.
     *  This feature must be used when utilizing small fly capacitors
     *  (0402 or smaller).
     * 0 = the charge pump automatically switches up and down in gain.
     *  This provides optimal efficiency, but is not suitable for driving
     *  loads that are not connected through the ADP8863 diode drivers.
     *  Additionally, the charge pump fly capacitors should be low ESR
     * and sized 0603 or greater.
     */
    pub gdwn_dis: u8,
}

// External dependency declaration supplied by linux/leds.h.
pub struct led_info;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
