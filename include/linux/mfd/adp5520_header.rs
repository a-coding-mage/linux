/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Definitions and platform data for Analog Devices
 * ADP5520/ADP5501 MFD PMICs (Backlight, LED, GPIO and Keys)
 *
 * Copyright 2009 Analog Devices Inc.
 */

use core::ffi::{c_int, c_uint};

/* External kernel types supplied by other dependencies. */
pub enum device {}
pub enum led_info {}
pub enum notifier_block {}

pub const ID_ADP5520: c_int = 5520;
pub const ID_ADP5501: c_int = 5501;

/* ADP5520/ADP5501 Register Map */
pub const ADP5520_MODE_STATUS: c_int = 0x00;
pub const ADP5520_INTERRUPT_ENABLE: c_int = 0x01;
pub const ADP5520_BL_CONTROL: c_int = 0x02;
pub const ADP5520_BL_TIME: c_int = 0x03;
pub const ADP5520_BL_FADE: c_int = 0x04;
pub const ADP5520_DAYLIGHT_MAX: c_int = 0x05;
pub const ADP5520_DAYLIGHT_DIM: c_int = 0x06;
pub const ADP5520_OFFICE_MAX: c_int = 0x07;
pub const ADP5520_OFFICE_DIM: c_int = 0x08;
pub const ADP5520_DARK_MAX: c_int = 0x09;
pub const ADP5520_DARK_DIM: c_int = 0x0A;
pub const ADP5520_BL_VALUE: c_int = 0x0B;
pub const ADP5520_ALS_CMPR_CFG: c_int = 0x0C;
pub const ADP5520_L2_TRIP: c_int = 0x0D;
pub const ADP5520_L2_HYS: c_int = 0x0E;
pub const ADP5520_L3_TRIP: c_int = 0x0F;
pub const ADP5520_L3_HYS: c_int = 0x10;
pub const ADP5520_LED_CONTROL: c_int = 0x11;
pub const ADP5520_LED_TIME: c_int = 0x12;
pub const ADP5520_LED_FADE: c_int = 0x13;
pub const ADP5520_LED1_CURRENT: c_int = 0x14;
pub const ADP5520_LED2_CURRENT: c_int = 0x15;
pub const ADP5520_LED3_CURRENT: c_int = 0x16;

pub const ADP5520_GPIO_CFG_1: c_int = 0x17;
pub const ADP5520_GPIO_CFG_2: c_int = 0x18;
pub const ADP5520_GPIO_IN: c_int = 0x19;
pub const ADP5520_GPIO_OUT: c_int = 0x1A;
pub const ADP5520_GPIO_INT_EN: c_int = 0x1B;
pub const ADP5520_GPIO_INT_STAT: c_int = 0x1C;
pub const ADP5520_GPIO_INT_LVL: c_int = 0x1D;
pub const ADP5520_GPIO_DEBOUNCE: c_int = 0x1E;
pub const ADP5520_GPIO_PULLUP: c_int = 0x1F;
pub const ADP5520_KP_INT_STAT_1: c_int = 0x20;
pub const ADP5520_KP_INT_STAT_2: c_int = 0x21;
pub const ADP5520_KR_INT_STAT_1: c_int = 0x22;
pub const ADP5520_KR_INT_STAT_2: c_int = 0x23;
pub const ADP5520_KEY_STAT_1: c_int = 0x24;
pub const ADP5520_KEY_STAT_2: c_int = 0x25;

pub const ADP5520_nSTNBY: c_int = 1 << 7;
pub const ADP5520_BL_EN: c_int = 1 << 6;
pub const ADP5520_DIM_EN: c_int = 1 << 5;
pub const ADP5520_OVP_INT: c_int = 1 << 4;
pub const ADP5520_CMPR_INT: c_int = 1 << 3;
pub const ADP5520_GPI_INT: c_int = 1 << 2;
pub const ADP5520_KR_INT: c_int = 1 << 1;
pub const ADP5520_KP_INT: c_int = 1 << 0;
pub const ADP5520_AUTO_LD_EN: c_int = 1 << 4;
pub const ADP5520_CMPR_IEN: c_int = 1 << 3;
pub const ADP5520_OVP_IEN: c_int = 1 << 2;
pub const ADP5520_KR_IEN: c_int = 1 << 1;
pub const ADP5520_KP_IEN: c_int = 1 << 0;
pub const ADP5520_BL_AUTO_ADJ: c_int = 1 << 3;
pub const ADP5520_OVP_EN: c_int = 1 << 2;
pub const ADP5520_FOVR: c_int = 1 << 1;
pub const ADP5520_KP_BL_EN: c_int = 1 << 0;
pub const ADP5520_L3_OUT: c_int = 1 << 3;
pub const ADP5520_L2_OUT: c_int = 1 << 2;
pub const ADP5520_L3_EN: c_int = 1 << 1;

#[inline] pub const fn ADP5520_BL_LVL(x: c_int) -> c_int { x << 5 }
#[inline] pub const fn ADP5520_BL_LAW(x: c_int) -> c_int { x << 4 }
pub const ADP5020_MAX_BRIGHTNESS: c_int = 0x7F;
#[inline] pub const fn FADE_VAL(input: c_int, output: c_int) -> c_int { (0xF & input) | ((0xF & output) << 4) }
#[inline] pub const fn BL_CTRL_VAL(law: c_int, auto_: c_int) -> c_int { ((1 & auto_) << 3) | ((0x3 & law) << 4) }
#[inline] pub const fn ALS_CMPR_CFG_VAL(filt: c_int, l3_en: c_int) -> c_int { ((0x7 & filt) << 5) | l3_en }

pub const ADP5520_01_MAXLEDS: c_int = 3;
pub const ADP5520_FLAG_LED_MASK: c_int = 0x3;
pub const ADP5520_FLAG_OFFT_SHIFT: c_int = 8;
pub const ADP5520_FLAG_OFFT_MASK: c_int = 0x3;
pub const ADP5520_R3_MODE: c_int = 1 << 5;
pub const ADP5520_C3_MODE: c_int = 1 << 4;
pub const ADP5520_LED_LAW: c_int = 1 << 3;
pub const ADP5520_LED3_EN: c_int = 1 << 2;
pub const ADP5520_LED2_EN: c_int = 1 << 1;
pub const ADP5520_LED1_EN: c_int = 1 << 0;

pub const ADP5520_MAXGPIOS: c_int = 8;
pub const ADP5520_GPIO_C3: c_int = 1 << 7;
pub const ADP5520_GPIO_C2: c_int = 1 << 6;
pub const ADP5520_GPIO_C1: c_int = 1 << 5;
pub const ADP5520_GPIO_C0: c_int = 1 << 4;
pub const ADP5520_GPIO_R3: c_int = 1 << 3;
pub const ADP5520_GPIO_R2: c_int = 1 << 2;
pub const ADP5520_GPIO_R1: c_int = 1 << 1;
pub const ADP5520_GPIO_R0: c_int = 1 << 0;

#[repr(C)] pub struct adp5520_gpio_platform_data { pub gpio_start: c_uint, pub gpio_en_mask: u8, pub gpio_pullup_mask: u8 }

pub const ADP5520_MAXKEYS: c_int = 16;
pub const ADP5520_COL_C3: c_int = 1 << 7;
pub const ADP5520_COL_C2: c_int = 1 << 6;
pub const ADP5520_COL_C1: c_int = 1 << 5;
pub const ADP5520_COL_C0: c_int = 1 << 4;
pub const ADP5520_ROW_R3: c_int = 1 << 3;
pub const ADP5520_ROW_R2: c_int = 1 << 2;
pub const ADP5520_ROW_R1: c_int = 1 << 1;
pub const ADP5520_ROW_R0: c_int = 1 << 0;
#[inline] pub const fn ADP5520_KEY(row: c_int, col: c_int) -> c_int { col + row * 4 }
pub const ADP5520_KEYMAPSIZE: c_int = ADP5520_MAXKEYS;

#[repr(C)] pub struct adp5520_keys_platform_data { pub rows_en_mask: c_int, pub cols_en_mask: c_int, pub keymap: *const u16, pub keymapsize: u16, pub repeat: u8 }

pub const FLAG_ID_ADP5520_LED1_ADP5501_LED0: c_int = 1;
pub const FLAG_ID_ADP5520_LED2_ADP5501_LED1: c_int = 2;
pub const FLAG_ID_ADP5520_LED3_ADP5501_LED2: c_int = 3;
pub const ADP5520_LED_DIS_BLINK: c_int = 0 << ADP5520_FLAG_OFFT_SHIFT;
pub const ADP5520_LED_OFFT_600ms: c_int = 1 << ADP5520_FLAG_OFFT_SHIFT;
pub const ADP5520_LED_OFFT_800ms: c_int = 2 << ADP5520_FLAG_OFFT_SHIFT;
pub const ADP5520_LED_OFFT_1200ms: c_int = 3 << ADP5520_FLAG_OFFT_SHIFT;
pub const ADP5520_LED_ONT_200ms: c_int = 0;
pub const ADP5520_LED_ONT_600ms: c_int = 1;
pub const ADP5520_LED_ONT_800ms: c_int = 2;
pub const ADP5520_LED_ONT_1200ms: c_int = 3;

#[repr(C)] pub struct adp5520_leds_platform_data { pub num_leds: c_int, pub leds: *mut led_info, pub fade_in: u8, pub fade_out: u8, pub led_on_time: u8 }

pub const ADP5520_FADE_T_DIS: c_int = 0;
pub const ADP5520_FADE_T_300ms: c_int = 1;
pub const ADP5520_FADE_T_600ms: c_int = 2;
pub const ADP5520_FADE_T_900ms: c_int = 3;
pub const ADP5520_FADE_T_1200ms: c_int = 4;
pub const ADP5520_FADE_T_1500ms: c_int = 5;
pub const ADP5520_FADE_T_1800ms: c_int = 6;
pub const ADP5520_FADE_T_2100ms: c_int = 7;
pub const ADP5520_FADE_T_2400ms: c_int = 8;
pub const ADP5520_FADE_T_2700ms: c_int = 9;
pub const ADP5520_FADE_T_3000ms: c_int = 10;
pub const ADP5520_FADE_T_3500ms: c_int = 11;
pub const ADP5520_FADE_T_4000ms: c_int = 12;
pub const ADP5520_FADE_T_4500ms: c_int = 13;
pub const ADP5520_FADE_T_5000ms: c_int = 14;
pub const ADP5520_FADE_T_5500ms: c_int = 15;
pub const ADP5520_BL_LAW_LINEAR: c_int = 0;
pub const ADP5520_BL_LAW_SQUARE: c_int = 1;
pub const ADP5520_BL_LAW_CUBIC1: c_int = 2;
pub const ADP5520_BL_LAW_CUBIC2: c_int = 3;
pub const ADP5520_BL_AMBL_FILT_80ms: c_int = 0;
pub const ADP5520_BL_AMBL_FILT_160ms: c_int = 1;
pub const ADP5520_BL_AMBL_FILT_320ms: c_int = 2;
pub const ADP5520_BL_AMBL_FILT_640ms: c_int = 3;
pub const ADP5520_BL_AMBL_FILT_1280ms: c_int = 4;
pub const ADP5520_BL_AMBL_FILT_2560ms: c_int = 5;
pub const ADP5520_BL_AMBL_FILT_5120ms: c_int = 6;
pub const ADP5520_BL_AMBL_FILT_10240ms: c_int = 7;
#[inline] pub const fn ADP5520_BL_CUR_mA(i: c_int) -> c_int { (i * 127) / 30 }
#[inline] pub const fn ADP5520_L2_COMP_CURR_uA(i: c_int) -> c_int { (i * 255) / 1000 }
#[inline] pub const fn ADP5520_L3_COMP_CURR_uA(i: c_int) -> c_int { (i * 255) / 127 }

#[repr(C)] pub struct adp5520_backlight_platform_data {
    pub fade_in: u8, pub fade_out: u8, pub fade_led_law: u8,
    pub en_ambl_sens: u8, pub abml_filt: u8, pub l1_daylight_max: u8, pub l1_daylight_dim: u8,
    pub l2_office_max: u8, pub l2_office_dim: u8, pub l3_dark_max: u8, pub l3_dark_dim: u8,
    pub l2_trip: u8, pub l2_hyst: u8, pub l3_trip: u8, pub l3_hyst: u8,
}

#[repr(C)] pub struct adp5520_platform_data {
    pub keys: *mut adp5520_keys_platform_data,
    pub gpio: *mut adp5520_gpio_platform_data,
    pub leds: *mut adp5520_leds_platform_data,
    pub backlight: *mut adp5520_backlight_platform_data,
}

extern "C" {
    pub fn adp5520_read(dev: *mut device, reg: c_int, val: *mut u8) -> c_int;
    pub fn adp5520_write(dev: *mut device, reg: c_int, val: u8) -> c_int;
    pub fn adp5520_clr_bits(dev: *mut device, reg: c_int, bit_mask: u8) -> c_int;
    pub fn adp5520_set_bits(dev: *mut device, reg: c_int, bit_mask: u8) -> c_int;
    pub fn adp5520_register_notifier(dev: *mut device, nb: *mut notifier_block, events: c_uint) -> c_int;
    pub fn adp5520_unregister_notifier(dev: *mut device, nb: *mut notifier_block, events: c_uint) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
