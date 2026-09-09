/* SPDX-License-Identifier: GPL-2.0-only OR MIT */
/*
 * Apple SMC (System Management Controller) core definitions
 *
 * Copyright (C) The Asahi Linux Contributors
 */

// Dependency: types supplied by <linux/soc/apple/rtkit.h> and the kernel.

/// Alias for u32 to be used for SMC keys.
pub type smc_key = u32;

/// Convert a FourCC SMC key to an smc_key.
#[inline]
pub const fn __SMC_KEY(a: u8, b: u8, c: u8, d: u8) -> u32 {
    ((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8) | (d as u32)
}

pub const APPLE_SMC_READABLE: u32 = 1 << 7;
pub const APPLE_SMC_WRITABLE: u32 = 1 << 6;
pub const APPLE_SMC_FUNCTION: u32 = 1 << 4;

#[repr(C)]
pub struct apple_smc_key_info {
    pub type_code: u32,
    pub size: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum apple_smc_boot_stage {
    APPLE_SMC_BOOTING,
    APPLE_SMC_INITIALIZED,
    APPLE_SMC_ERROR_NO_SHMEM,
    APPLE_SMC_ERROR_CRASHED,
}

#[repr(C)]
pub struct apple_smc {
    pub dev: *mut device,
    pub key_count: u32,
    pub first_key: smc_key,
    pub last_key: smc_key,
    pub event_handlers: blocking_notifier_head,
    pub rtk: *mut apple_rtkit,
    pub init_done: completion,
    pub boot_stage: apple_smc_boot_stage,
    pub sram: *mut resource,
    pub sram_base: *mut core::ffi::c_void,
    pub shmem: apple_rtkit_shmem,
    pub msg_id: core::ffi::c_uint,
    pub atomic_mode: bool,
    pub atomic_pending: bool,
    pub cmd_done: completion,
    pub cmd_ret: u64,
    pub mutex: mutex,
    pub lock: spinlock_t,
}

extern "C" {
    pub fn apple_smc_read(smc: *mut apple_smc, key: smc_key, buf: *mut core::ffi::c_void, size: usize) -> i32;
    pub fn apple_smc_write(smc: *mut apple_smc, key: smc_key, buf: *const core::ffi::c_void, size: usize) -> i32;
    pub fn apple_smc_enter_atomic(smc: *mut apple_smc) -> i32;
    pub fn apple_smc_write_atomic(smc: *mut apple_smc, key: smc_key, buf: *const core::ffi::c_void, size: usize) -> i32;
    pub fn apple_smc_rw(smc: *mut apple_smc, key: smc_key, wbuf: *const core::ffi::c_void, wsize: usize,
                        rbuf: *mut core::ffi::c_void, rsize: usize) -> i32;
    pub fn apple_smc_get_key_by_index(smc: *mut apple_smc, index: i32, key: *mut smc_key) -> i32;
    pub fn apple_smc_get_key_info(smc: *mut apple_smc, key: smc_key, info: *mut apple_smc_key_info) -> i32;
}

#[inline]
pub unsafe fn apple_smc_key_exists(smc: *mut apple_smc, key: smc_key) -> bool {
    apple_smc_get_key_info(smc, key, core::ptr::null_mut()) >= 0
}

// The C macro APPLE_SMC_TYPE_OPS expands the following typed interfaces.
// Rust identifiers cannot be token-pasted in this macro, so the declarations
// are written explicitly below.

macro_rules! smc_ops {
    ($read:ident, $write:ident, $atomic:ident, $rw:ident, $ty:ty) => {
        #[inline] pub unsafe fn $read(s: *mut apple_smc, k: smc_key, p: *mut $ty) -> i32 { let r = apple_smc_read(s,k,p as *mut _,core::mem::size_of::<$ty>()); if r < 0 { r } else if r != core::mem::size_of::<$ty>() as i32 { -22 } else { 0 } }
        #[inline] pub unsafe fn $write(s: *mut apple_smc, k: smc_key, p: $ty) -> i32 { apple_smc_write(s,k,&p as *const _ as *const _,core::mem::size_of::<$ty>()) }
        #[inline] pub unsafe fn $atomic(s: *mut apple_smc, k: smc_key, p: $ty) -> i32 { apple_smc_write_atomic(s,k,&p as *const _ as *const _,core::mem::size_of::<$ty>()) }
        #[inline] pub unsafe fn $rw(s: *mut apple_smc, k: smc_key, w: $ty, r: *mut $ty) -> i32 { let n=apple_smc_rw(s,k,&w as *const _ as *const _,core::mem::size_of::<$ty>(),r as *mut _,core::mem::size_of::<$ty>()); if n < 0 { n } else if n != core::mem::size_of::<$ty>() as i32 { -22 } else { 0 } }
    };
}

smc_ops!(apple_smc_read_u64, apple_smc_write_u64, apple_smc_write_u64_atomic, apple_smc_rw_u64, u64);
smc_ops!(apple_smc_read_u32, apple_smc_write_u32, apple_smc_write_u32_atomic, apple_smc_rw_u32, u32);
smc_ops!(apple_smc_read_u16, apple_smc_write_u16, apple_smc_write_u16_atomic, apple_smc_rw_u16, u16);
smc_ops!(apple_smc_read_u8, apple_smc_write_u8, apple_smc_write_u8_atomic, apple_smc_rw_u8, u8);
smc_ops!(apple_smc_read_s64, apple_smc_write_s64, apple_smc_write_s64_atomic, apple_smc_rw_s64, i64);
smc_ops!(apple_smc_read_s32, apple_smc_write_s32, apple_smc_write_s32_atomic, apple_smc_rw_s32, i32);
smc_ops!(apple_smc_read_s16, apple_smc_write_s16, apple_smc_write_s16_atomic, apple_smc_rw_s16, i16);
smc_ops!(apple_smc_read_s8, apple_smc_write_s8, apple_smc_write_s8_atomic, apple_smc_rw_s8, i8);

#[inline]
pub unsafe fn apple_smc_read_flag(smc: *mut apple_smc, key: smc_key, flag: *mut bool) -> i32 {
    let mut val: u8 = 0;
    let ret = apple_smc_read_u8(smc, key, &mut val);
    if ret < 0 { return ret; }
    *flag = val != 0;
    ret
}

#[inline] pub unsafe fn apple_smc_write_flag(s: *mut apple_smc, k: smc_key, state: bool) -> i32 { apple_smc_write_u8(s,k,if state {1} else {0}) }
#[inline] pub unsafe fn apple_smc_write_flag_atomic(s: *mut apple_smc, k: smc_key, state: bool) -> i32 { apple_smc_write_u8_atomic(s,k,if state {1} else {0}) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
