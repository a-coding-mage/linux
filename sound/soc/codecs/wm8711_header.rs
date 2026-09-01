/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8711.h  --  WM8711 Soc Audio driver
 *
 * Copyright 2006 Wolfson Microelectronics
 *
 * Author: Mike Arthur <linux@wolfsonmicro.com>
 *
 * Based on wm8731.h
 */

/* WM8711 register space */

pub const WM8711_LOUT1V: u32 = 0x02;
pub const WM8711_ROUT1V: u32 = 0x03;
pub const WM8711_APANA: u32 = 0x04;
pub const WM8711_APDIGI: u32 = 0x05;
pub const WM8711_PWR: u32 = 0x06;
pub const WM8711_IFACE: u32 = 0x07;
pub const WM8711_SRATE: u32 = 0x08;
pub const WM8711_ACTIVE: u32 = 0x09;
pub const WM8711_RESET: u32 = 0x0f;

pub const WM8711_CACHEREGNUM: u32 = 8;

pub const WM8711_SYSCLK: u32 = 0;
pub const WM8711_DAI: u32 = 0;

#[repr(C)]
pub struct wm8711_setup_data {
    pub i2c_address: u16,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
