/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: Hongzhou.Yang <hongzhou.yang@mediatek.com>
 */

macro_rules! MTK_PIN_NO {
    ($x:expr) => { (($x) << 8) };
}

macro_rules! MTK_GET_PIN_NO {
    ($x:expr) => { (($x) >> 8) };
}

macro_rules! MTK_GET_PIN_FUNC {
    ($x:expr) => { (($x) & 0xf) };
}

pub const MTK_PUPD_SET_R1R0_00: i32 = 100;
pub const MTK_PUPD_SET_R1R0_01: i32 = 101;
pub const MTK_PUPD_SET_R1R0_10: i32 = 102;
pub const MTK_PUPD_SET_R1R0_11: i32 = 103;

pub const MTK_PULL_SET_RSEL_000: i32 = 200;
pub const MTK_PULL_SET_RSEL_001: i32 = 201;
pub const MTK_PULL_SET_RSEL_010: i32 = 202;
pub const MTK_PULL_SET_RSEL_011: i32 = 203;
pub const MTK_PULL_SET_RSEL_100: i32 = 204;
pub const MTK_PULL_SET_RSEL_101: i32 = 205;
pub const MTK_PULL_SET_RSEL_110: i32 = 206;
pub const MTK_PULL_SET_RSEL_111: i32 = 207;

pub const MTK_DRIVE_2mA: i32 = 2;
pub const MTK_DRIVE_4mA: i32 = 4;
pub const MTK_DRIVE_6mA: i32 = 6;
pub const MTK_DRIVE_8mA: i32 = 8;
pub const MTK_DRIVE_10mA: i32 = 10;
pub const MTK_DRIVE_12mA: i32 = 12;
pub const MTK_DRIVE_14mA: i32 = 14;
pub const MTK_DRIVE_16mA: i32 = 16;
pub const MTK_DRIVE_20mA: i32 = 20;
pub const MTK_DRIVE_24mA: i32 = 24;
pub const MTK_DRIVE_28mA: i32 = 28;
pub const MTK_DRIVE_32mA: i32 = 32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
