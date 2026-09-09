/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *	LED driver for TI lp3952 controller
 *
 *	Copyright (C) 2016, DAQRI, LLC.
 *	Author: Tony Makkiel <tony.makkiel@daqri.com>
 */

pub const LP3952_NAME: &str = "lp3952";
pub const LP3952_CMD_REG_COUNT: usize = 8;
pub const LP3952_BRIGHT_MAX: usize = 4;
pub const LP3952_LABEL_MAX_LEN: usize = 15;

pub const LP3952_REG_LED_CTRL: u8 = 0x00;
pub const LP3952_REG_R1_BLNK_TIME_CTRL: u8 = 0x01;
pub const LP3952_REG_R1_BLNK_CYCLE_CTRL: u8 = 0x02;
pub const LP3952_REG_G1_BLNK_TIME_CTRL: u8 = 0x03;
pub const LP3952_REG_G1_BLNK_CYCLE_CTRL: u8 = 0x04;
pub const LP3952_REG_B1_BLNK_TIME_CTRL: u8 = 0x05;
pub const LP3952_REG_B1_BLNK_CYCLE_CTRL: u8 = 0x06;
pub const LP3952_REG_ENABLES: u8 = 0x0b;
pub const LP3952_REG_PAT_GEN_CTRL: u8 = 0x11;
pub const LP3952_REG_RGB1_MAX_I_CTRL: u8 = 0x12;
pub const LP3952_REG_RGB2_MAX_I_CTRL: u8 = 0x13;
pub const LP3952_REG_CMD_0: u8 = 0x50;
pub const LP3952_REG_RESET: u8 = 0x60;
pub const REG_MAX: u8 = LP3952_REG_RESET;

pub const LP3952_PATRN_LOOP: u8 = 1 << 1;
pub const LP3952_PATRN_GEN_EN: u8 = 1 << 2;
pub const LP3952_INT_B00ST_LDR: u8 = 1 << 2;
pub const LP3952_ACTIVE_MODE: u8 = 1 << 6;
pub const LP3952_LED_MASK_ALL: u8 = 0x3f;

/* Transition Time in ms */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp3952_tt {
    TT0,
    TT55,
    TT110,
    TT221,
    TT422,
    TT885,
    TT1770,
    TT3539,
}

/* Command Execution Time in ms */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp3952_cet {
    CET197,
    CET393,
    CET590,
    CET786,
    CET1180,
    CET1376,
    CET1573,
    CET1769,
    CET1966,
    CET2163,
    CET2359,
    CET2556,
    CET2763,
    CET2949,
    CET3146,
}

/* Max Current in % */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp3952_colour_I_log_0 {
    I0,
    I7,
    I14,
    I21,
    I32,
    I46,
    I71,
    I100,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum lp3952_leds {
    LP3952_BLUE_2,
    LP3952_GREEN_2,
    LP3952_RED_2,
    LP3952_BLUE_1,
    LP3952_GREEN_1,
    LP3952_RED_1,
    LP3952_LED_ALL,
}

#[repr(C)]
pub struct lp3952_ctrl_hdl {
    pub cdev: led_classdev,
    pub name: [core::ffi::c_char; LP3952_LABEL_MAX_LEN],
    pub channel: lp3952_leds,
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct ptrn_gen_cmd_bits {
    pub tt: u16,
    pub b: u16,
    pub cet: u16,
    pub g: u16,
    pub r: u16,
}

#[repr(C)]
pub struct ptrn_gen_cmd_bytes {
    pub lsb: u8,
    pub msb: u8,
}

#[repr(C)]
pub union ptrn_gen_cmd_data {
    pub bits: ptrn_gen_cmd_bits,
    pub bytes: ptrn_gen_cmd_bytes,
}

#[repr(C, packed)]
pub struct ptrn_gen_cmd {
    pub data: ptrn_gen_cmd_data,
}

#[repr(C)]
pub struct lp3952_led_array {
    pub regmap: *mut regmap,
    pub client: *mut i2c_client,
    pub enable_gpio: *mut gpio_desc,
    pub leds: [lp3952_ctrl_hdl; LP3952_LED_ALL as usize],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
