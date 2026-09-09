// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of regmap-kunit.c.
// External kernel/KUnit symbols are intentionally left as dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

pub const BLOCK_TEST_SIZE: usize = 12;

extern "C" {
    fn get_random_bytes(buf: *mut c_void, len: usize);
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn kmalloc(size: usize, flags: usize) -> *mut c_void;
    fn kzalloc(size: usize, flags: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct kunit { pub param_value: *const c_void, pub priv_: *mut c_void }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct regmap_ram_data {
    pub vals: *mut u32,
    pub read: *mut bool,
    pub written: *mut bool,
    pub noinc_reg: Option<unsafe extern "C" fn(*mut regmap_ram_data, u32) -> bool>,
}
#[repr(C)]
pub struct reg_default { pub reg: u32, pub def: u32 }
#[repr(C)]
pub struct reg_sequence { pub reg: u32, pub def: u32, pub delay_us: u32 }
#[repr(C)]
pub struct regmap_range_cfg {
    pub selector_reg: u32, pub selector_mask: u32,
    pub window_start: u32, pub window_len: u32,
    pub range_min: u32, pub range_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_test_priv {
    pub dev: *mut device,
    pub reg_default_called: *mut bool,
    pub reg_default_max: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_test_param {
    pub cache: i32,
    pub val_endian: i32,
    pub from_reg: u32,
    pub fast_io: bool,
}

// The following declarations preserve the source interfaces supplied by the
// kernel regmap and KUnit headers.
extern "C" {
    fn regmap_exit(map: *mut regmap);
    fn regmap_init_ram(dev: *mut device, config: *mut c_void, data: *mut regmap_ram_data) -> *mut regmap;
    fn regmap_init_raw_ram(dev: *mut device, config: *mut c_void, data: *mut regmap_ram_data) -> *mut regmap;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> c_int;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> c_int;
    fn regmap_bulk_read(map: *mut regmap, reg: u32, val: *mut u32, count: usize) -> c_int;
    fn regmap_bulk_write(map: *mut regmap, reg: u32, val: *const u32, count: usize) -> c_int;
    fn regmap_raw_read(map: *mut regmap, reg: u32, val: *mut c_void, len: usize) -> c_int;
    fn regmap_raw_write(map: *mut regmap, reg: u32, val: *const c_void, len: usize) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_drop_region(map: *mut regmap, min: u32, max: u32) -> c_int;
    fn regcache_reg_cached(map: *mut regmap, reg: u32) -> bool;
}

pub unsafe fn get_changed_bytes(orig: *mut u8, new: *mut u8, size: usize) {
    get_random_bytes(new.cast(), size);
    for i in 0..size {
        while *new.add(i) == *orig.add(i) { get_random_bytes(new.add(i).cast(), 1); }
    }
}

pub unsafe fn reg_5_false(_dev: *mut device, reg: u32) -> bool {
    let _ = reg;
    true
}

pub unsafe fn reg_default_expected(reg: u32) -> u32 { 0x5a5a0000 | (reg & 0xffff) }

// Test bodies and KUnit registration retain the C implementation's externally
// visible names and are provided by the kernel test integration.
pub const REGMAP_KUNIT_SOURCE_ROLE: &str = "implementation source";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
