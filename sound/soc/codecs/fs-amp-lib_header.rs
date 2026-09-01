/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * fs-amp-lib.h --- Common library for FourSemi Audio Amplifiers
 *
 * Copyright (C) 2016-2025 Shanghai FourSemi Semiconductor Co.,Ltd.
 */

use core::ffi::{c_char, c_int, c_uint};

pub const fn HI_U16(a: u16) -> u16 {
    ((a >> 8) & 0xFF) as u16
}

pub const fn LO_U16(a: u16) -> u16 {
    (a & 0xFF) as u16
}

pub const FS_TABLE_NAME_LEN: usize = 4;
pub const FS_SCENE_COUNT_MAX: usize = 16;
pub const FS_CMD_DELAY_MS_MAX: u8 = 100; /* 100ms */

pub const FS_CMD_DELAY: u8 = 0xFF;
pub const FS_CMD_BURST: u8 = 0xFE;
pub const FS_CMD_UPDATE: u8 = 0xFD;

/*
 * C macro:
 * FS_SOC_ENUM_EXT(xname, xhandler_info, xhandler_get, xhandler_put)
 * expands to a designated initializer using SNDRV_CTL_ELEM_IFACE_MIXER and
 * fields .iface, .name, .info, .get, and .put. The target struct type is
 * supplied by ALSA headers outside this isolated file.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum fs_index_type {
    FS_INDEX_INFO = 0,
    FS_INDEX_STCOEF,
    FS_INDEX_SCENE,
    FS_INDEX_MODEL,
    FS_INDEX_REG,
    FS_INDEX_EFFECT,
    FS_INDEX_STRING,
    FS_INDEX_WOOFER,
    FS_INDEX_MAX,
}

pub const FS_INDEX_MAX: usize = fs_index_type::FS_INDEX_MAX as usize;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fs_reg_val {
    pub reg: u8,
    pub val: u16,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fs_reg_bits {
    pub cmd: u8, /* FS_CMD_UPDATE */
    pub reg: u8,
    pub val: u16,
    pub mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fs_cmd_pkg_union {
    pub cmd: u8,
    pub regv: fs_reg_val,
    pub regb: fs_reg_bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_cmd_pkg {
    pub u: fs_cmd_pkg_union,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fs_fwm_index {
    /* Index type */
    pub type_: u16,
    /* Offset address starting from the end of header */
    pub offset: u16,
}

#[repr(C, packed)]
pub struct fs_fwm_table {
    pub name: [c_char; FS_TABLE_NAME_LEN],
    pub size: u16, /* size of buf */
    pub buf: [u8; 0],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fs_scene_index {
    /* Offset address(scene name) in string table */
    pub name: u16,
    /* Offset address(scene reg) in register table */
    pub reg: u16,
    /* Offset address(scene model) in model table */
    pub model: u16,
    /* Offset address(scene effect) in effect table */
    pub effect: u16,
}

#[repr(C, packed)]
pub struct fs_reg_table {
    pub size: u16, /* size of buf */
    pub buf: [u8; 0],
}

#[repr(C, packed)]
pub struct fs_file_table {
    pub name: u16,
    pub size: u16, /* size of buf */
    pub buf: [u8; 0],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct fs_fwm_date {
    /*
     * C bitfields in one u32 storage unit:
     * year:12, month:4, day:5, hour:5, minute:6.
     */
    pub bits: u32,
}

pub const FS_FWM_DATE_YEAR_SHIFT: u32 = 0;
pub const FS_FWM_DATE_YEAR_MASK: u32 = 0x0fff;
pub const FS_FWM_DATE_MONTH_SHIFT: u32 = 12;
pub const FS_FWM_DATE_MONTH_MASK: u32 = 0x000f;
pub const FS_FWM_DATE_DAY_SHIFT: u32 = 16;
pub const FS_FWM_DATE_DAY_MASK: u32 = 0x001f;
pub const FS_FWM_DATE_HOUR_SHIFT: u32 = 21;
pub const FS_FWM_DATE_HOUR_MASK: u32 = 0x001f;
pub const FS_FWM_DATE_MINUTE_SHIFT: u32 = 26;
pub const FS_FWM_DATE_MINUTE_MASK: u32 = 0x003f;

#[repr(C, packed)]
pub struct fs_fwm_header {
    pub version: u16,
    pub project: u16, /* Offset address(project name) in string table */
    pub device: u16,  /* Offset address(device name) in string table */
    pub date: fs_fwm_date,
    pub crc16: u16,
    pub crc_size: u16, /* Starting position for CRC checking */
    pub chip_type: u16,
    pub addr: u16, /* 7-bit i2c address */
    pub spkid: u16,
    pub rsvd: [u16; 6],
    pub params: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_i2s_srate {
    pub srate: u32, /* Sample rate */
    pub i2ssr: u16, /* Value of Bit field[I2SSR] */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_pll_div {
    pub bclk: c_uint, /* Rate of bit clock */
    pub pll1: u16,
    pub pll2: u16,
    pub pll3: u16,
}

#[repr(C)]
pub struct fs_amp_scene {
    pub name: *const c_char,
    pub reg: *const fs_reg_table,
    pub model: *const fs_file_table,
    pub effect: *const fs_file_table,
}

/* External Linux device type supplied by kernel headers outside this file. */
pub enum device {}

#[repr(C)]
pub struct fs_amp_lib {
    pub hdr: *const fs_fwm_header,
    pub table: [*const fs_fwm_table; FS_INDEX_MAX],
    pub scene: *mut fs_amp_scene,
    pub dev: *mut device,
    pub scene_count: c_int,
    pub devid: u16,
}

extern "C" {
    pub fn fs_amp_load_firmware(amp_lib: *mut fs_amp_lib, name: *const c_char) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
