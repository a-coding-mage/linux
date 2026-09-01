/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8731.h  --  WM8731 Soc Audio driver
 *
 * Copyright 2005 Openedhand Ltd.
 *
 * Author: Richard Purdie <richard@openedhand.com>
 *
 * Based on wm8753.h
 */

/* C dependencies:
 * #include <linux/mutex.h>
 * #include <linux/regmap.h>
 * #include <linux/regulator/consumer.h>
 */

/* C forward declarations:
 * struct clk;
 * struct snd_pcm_hw_constraint_list;
 */

/* WM8731 register space */

pub const WM8731_LINVOL: u32 = 0x00;
pub const WM8731_RINVOL: u32 = 0x01;
pub const WM8731_LOUT1V: u32 = 0x02;
pub const WM8731_ROUT1V: u32 = 0x03;
pub const WM8731_APANA: u32 = 0x04;
pub const WM8731_APDIGI: u32 = 0x05;
pub const WM8731_PWR: u32 = 0x06;
pub const WM8731_IFACE: u32 = 0x07;
pub const WM8731_SRATE: u32 = 0x08;
pub const WM8731_ACTIVE: u32 = 0x09;
pub const WM8731_RESET: u32 = 0x0f;

pub const WM8731_CACHEREGNUM: u32 = 10;

pub const WM8731_SYSCLK_MCLK: u32 = 0;
pub const WM8731_SYSCLK_XTAL: u32 = 1;

pub const WM8731_DAI: u32 = 0;

pub const WM8731_NUM_SUPPLIES: usize = 4;

/* codec private data */
#[repr(C)]
pub struct wm8731_priv {
    pub regmap: *mut regmap,
    pub mclk: *mut clk,
    pub supplies: [regulator_bulk_data; WM8731_NUM_SUPPLIES],
    pub constraints: *const snd_pcm_hw_constraint_list,
    pub sysclk: ::core::ffi::c_uint,
    pub sysclk_type: ::core::ffi::c_int,
    pub playback_fs: ::core::ffi::c_int,
    pub deemph: bool,

    pub lock: mutex,
}

unsafe extern "C" {
    pub static wm8731_regmap: regmap_config;

    pub fn wm8731_init(dev: *mut device, wm8731: *mut wm8731_priv) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
