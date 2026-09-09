/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2000, 2001, 2002 Broadcom Corporation
 */
/*
 * Broadcom Common Firmware Environment (CFE)
 *
 * This file contains declarations for doing callbacks to
 * cfe from an application.  It should be the only header
 * needed by the application to use this library
 *
 * Authors:  Mitch Lichtenberg, Chris Demetriou
 */

use core::ffi::{c_char, c_int, c_long, c_ulong};

/*
 * Constants
 */

/* Seal indicating CFE's presence, passed to user program. */
pub const CFE_EPTSEAL: u32 = 0x43464531;

pub const CFE_MI_RESERVED: c_int = 0; /* memory is reserved, do not use */
pub const CFE_MI_AVAILABLE: c_int = 1; /* memory is available */

pub const CFE_FLG_WARMSTART: u32 = 0x00000001;
pub const CFE_FLG_FULL_ARENA: u32 = 0x00000001;
pub const CFE_FLG_ENV_PERMANENT: u32 = 0x00000001;

pub const CFE_CPU_CMD_START: c_int = 1;
pub const CFE_CPU_CMD_STOP: c_int = 0;

pub const CFE_STDHANDLE_CONSOLE: c_int = 0;

pub const CFE_DEV_NETWORK: c_int = 1;
pub const CFE_DEV_DISK: c_int = 2;
pub const CFE_DEV_FLASH: c_int = 3;
pub const CFE_DEV_SERIAL: c_int = 4;
pub const CFE_DEV_CPU: c_int = 5;
pub const CFE_DEV_NVRAM: c_int = 6;
pub const CFE_DEV_CLOCK: c_int = 7;
pub const CFE_DEV_OTHER: c_int = 8;
pub const CFE_DEV_MASK: c_int = 0x0F;

pub const CFE_CACHE_FLUSH_D: c_int = 1;
pub const CFE_CACHE_INVAL_I: c_int = 2;
pub const CFE_CACHE_INVAL_D: c_int = 4;
pub const CFE_CACHE_INVAL_L2: c_int = 8;

pub const CFE_FWI_64BIT: u32 = 0x00000001;
pub const CFE_FWI_32BIT: u32 = 0x00000002;
pub const CFE_FWI_RELOC: u32 = 0x00000004;
pub const CFE_FWI_UNCACHED: u32 = 0x00000008;
pub const CFE_FWI_MULTICPU: u32 = 0x00000010;
pub const CFE_FWI_FUNCSIM: u32 = 0x00000020;
pub const CFE_FWI_RTLSIM: u32 = 0x00000040;

#[repr(C)]
pub struct cfe_fwinfo_t {
    pub fwi_version: i64, /* major, minor, eco version */
    pub fwi_totalmem: i64, /* total installed mem */
    pub fwi_flags: i64, /* various flags */
    pub fwi_boardid: i64, /* board ID */
    pub fwi_bootarea_va: i64, /* VA of boot area */
    pub fwi_bootarea_pa: i64, /* PA of boot area */
    pub fwi_bootarea_size: i64, /* size of boot area */
}

/*
 * Defines and prototypes for functions which take no arguments.
 */
unsafe extern "C" {
    pub fn cfe_getticks() -> i64;

    /*
     * Defines and prototypes for the rest of the functions.
     */
    pub fn cfe_close(handle: c_int) -> c_int;
    pub fn cfe_cpu_start(cpu: c_int, func: Option<unsafe extern "C" fn()>, sp: c_long, gp: c_long, a1: c_long) -> c_int;
    pub fn cfe_cpu_stop(cpu: c_int) -> c_int;
    pub fn cfe_enumenv(idx: c_int, name: *mut c_char, namelen: c_int, val: *mut c_char, vallen: c_int) -> c_int;
    pub fn cfe_enummem(idx: c_int, flags: c_int, start: *mut u64, length: *mut u64, type_: *mut u64) -> c_int;
    pub fn cfe_exit(warm: c_int, status: c_int) -> c_int;
    pub fn cfe_flushcache(flg: c_int) -> c_int;
    pub fn cfe_getdevinfo(name: *mut c_char) -> c_int;
    pub fn cfe_getenv(name: *mut c_char, dest: *mut c_char, destlen: c_int) -> c_int;
    pub fn cfe_getfwinfo(info: *mut cfe_fwinfo_t) -> c_int;
    pub fn cfe_getstdhandle(flg: c_int) -> c_int;
    pub fn cfe_init(handle: u64, ept: u64) -> c_int;
    pub fn cfe_inpstat(handle: c_int) -> c_int;
    pub fn cfe_ioctl(handle: c_int, ioctlnum: u32, buffer: *mut u8, length: c_int, retlen: *mut c_int, offset: u64) -> c_int;
    pub fn cfe_open(name: *mut c_char) -> c_int;
    pub fn cfe_read(handle: c_int, buffer: *mut u8, length: c_int) -> c_int;
    pub fn cfe_readblk(handle: c_int, offset: i64, buffer: *mut u8, length: c_int) -> c_int;
    pub fn cfe_setenv(name: *mut c_char, val: *mut c_char) -> c_int;
    pub fn cfe_write(handle: c_int, buffer: *const c_char, length: c_int) -> c_int;
    pub fn cfe_writeblk(handle: c_int, offset: i64, buffer: *const c_char, length: c_int) -> c_int;
    pub static mut cfe_seal: c_ulong;
    /* __printf(1, 2) */
    pub fn cfe_die(fmt: *mut c_char, ...);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
