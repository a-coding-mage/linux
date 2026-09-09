/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright © 2000-2010 David Woodhouse <dwmw2@infradead.org> et al. */

/* Overhauled routines for dealing with different mmap regions of flash */

// C header dependencies are supplied by the surrounding kernel translation.

pub struct device_node;
pub struct module;
pub struct mtd_info;
pub struct list_head;
pub struct mtd_chip_driver;

#[cfg(feature = "CONFIG_MTD_MAP_BANK_WIDTH_1")]
pub const MAX_MAP_BANKWIDTH: usize = 1;
#[cfg(feature = "CONFIG_MTD_MAP_BANK_WIDTH_2")]
pub const MAX_MAP_BANKWIDTH: usize = 2;
#[cfg(feature = "CONFIG_MTD_MAP_BANK_WIDTH_4")]
pub const MAX_MAP_BANKWIDTH: usize = 4;
#[cfg(feature = "CONFIG_MTD_MAP_BANK_WIDTH_8")]
pub const MAX_MAP_BANKWIDTH: usize = 8;
#[cfg(feature = "CONFIG_MTD_MAP_BANK_WIDTH_16")]
pub const MAX_MAP_BANKWIDTH: usize = 16;
#[cfg(feature = "CONFIG_MTD_MAP_BANK_WIDTH_32")]
pub const MAX_MAP_BANKWIDTH: usize = 32;

#[cfg(feature = "CONFIG_MTD_MAP_BANK_WIDTH_1")]
#[inline] pub unsafe fn map_bankwidth(_map: *mut map_info) -> usize { 1 }
#[cfg(all(not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_1"), feature = "CONFIG_MTD_MAP_BANK_WIDTH_2"))]
#[inline] pub unsafe fn map_bankwidth(_map: *mut map_info) -> usize { 2 }
#[cfg(all(not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_1"), not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_2"), feature = "CONFIG_MTD_MAP_BANK_WIDTH_4"))]
#[inline] pub unsafe fn map_bankwidth(_map: *mut map_info) -> usize { 4 }
#[cfg(all(not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_1"), not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_2"), not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_4"), feature = "CONFIG_MTD_MAP_BANK_WIDTH_8"))]
#[inline] pub unsafe fn map_bankwidth(_map: *mut map_info) -> usize { 8 }
#[cfg(all(not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_1"), not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_2"), not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_4"), not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_8"), feature = "CONFIG_MTD_MAP_BANK_WIDTH_16"))]
#[inline] pub unsafe fn map_bankwidth(_map: *mut map_info) -> usize { 16 }
#[cfg(all(not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_1"), not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_2"), not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_4"), not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_8"), not(feature = "CONFIG_MTD_MAP_BANK_WIDTH_16"), feature = "CONFIG_MTD_MAP_BANK_WIDTH_32"))]
#[inline] pub unsafe fn map_bankwidth(map: *mut map_info) -> usize { (*map).bankwidth as usize }

#[inline] pub unsafe fn map_bankwidth_is_1(map: *mut map_info) -> bool { map_bankwidth(map) == 1 }
#[inline] pub unsafe fn map_bankwidth_is_2(map: *mut map_info) -> bool { map_bankwidth(map) == 2 }
#[inline] pub unsafe fn map_bankwidth_is_4(map: *mut map_info) -> bool { map_bankwidth(map) == 4 }
#[inline] pub unsafe fn map_bankwidth_is_8(map: *mut map_info) -> bool { map_bankwidth(map) == 8 }
#[inline] pub unsafe fn map_bankwidth_is_16(map: *mut map_info) -> bool { map_bankwidth(map) == 16 }
#[inline] pub unsafe fn map_bankwidth_is_32(map: *mut map_info) -> bool { map_bankwidth(map) == 32 }
#[inline] pub unsafe fn map_bankwidth_is_large(map: *mut map_info) -> bool { map_bankwidth(map) > core::mem::size_of::<usize>() }
#[inline] pub unsafe fn map_words(map: *mut map_info) -> usize { (map_bankwidth(map) + core::mem::size_of::<usize>() - 1) / core::mem::size_of::<usize>() }
#[inline] pub unsafe fn map_calc_words(map: *mut map_info) -> usize { map_words(map) }

pub const NO_XIP: usize = usize::MAX;
pub const MAX_MAP_LONGS: usize = (MAX_MAP_BANKWIDTH * 8 + usize::BITS as usize - 1) / usize::BITS as usize;

#[repr(C)]
#[derive(Copy, Clone)]
pub union map_word { pub x: [usize; MAX_MAP_LONGS] }

#[repr(C)]
pub struct map_info {
    pub name: *const i8,
    pub size: usize,
    pub phys: usize,
    pub virt: *mut u8,
    pub cached: *mut core::ffi::c_void,
    pub swap: i32,
    pub bankwidth: i32,
    #[cfg(feature = "CONFIG_MTD_COMPLEX_MAPPINGS")]
    pub read: Option<unsafe extern "C" fn(*mut map_info, usize) -> map_word>,
    #[cfg(feature = "CONFIG_MTD_COMPLEX_MAPPINGS")]
    pub copy_from: Option<unsafe extern "C" fn(*mut map_info, *mut core::ffi::c_void, usize, isize)>,
    #[cfg(feature = "CONFIG_MTD_COMPLEX_MAPPINGS")]
    pub write: Option<unsafe extern "C" fn(*mut map_info, map_word, usize)>,
    #[cfg(feature = "CONFIG_MTD_COMPLEX_MAPPINGS")]
    pub copy_to: Option<unsafe extern "C" fn(*mut map_info, usize, *const core::ffi::c_void, isize)>,
    pub inval_cache: Option<unsafe extern "C" fn(*mut map_info, usize, isize)>,
    pub set_vpp: Option<unsafe extern "C" fn(*mut map_info, i32)>,
    pub pfow_base: usize,
    pub map_priv_1: usize,
    pub map_priv_2: usize,
    pub device_node: *mut device_node,
    pub fldrv_priv: *mut core::ffi::c_void,
    pub fldrv: *mut mtd_chip_driver,
}

#[repr(C)]
pub struct mtd_chip_driver {
    pub probe: Option<unsafe extern "C" fn(*mut map_info) -> *mut mtd_info>,
    pub destroy: Option<unsafe extern "C" fn(*mut mtd_info)>,
    pub module: *mut module,
    pub name: *mut i8,
    pub list: list_head,
}

unsafe extern "C" {
    pub fn register_mtd_chip_driver(driver: *mut mtd_chip_driver);
    pub fn unregister_mtd_chip_driver(driver: *mut mtd_chip_driver);
    pub fn do_map_probe(name: *const i8, map: *mut map_info) -> *mut mtd_info;
    pub fn map_destroy(mtd: *mut mtd_info);
}

#[inline] pub unsafe fn map_word_equal(map: *mut map_info, val1: map_word, val2: map_word) -> i32 { let mut i = 0; while i < map_words(map) { if val1.x[i] != val2.x[i] { return 0; } i += 1; } 1 }
#[inline] pub unsafe fn map_word_and(map: *mut map_info, val1: map_word, val2: map_word) -> map_word { let mut r = map_word { x: [0; MAX_MAP_LONGS] }; let mut i = 0; while i < map_words(map) { r.x[i] = val1.x[i] & val2.x[i]; i += 1; } r }
#[inline] pub unsafe fn map_word_clr(map: *mut map_info, val1: map_word, val2: map_word) -> map_word { let mut r = map_word { x: [0; MAX_MAP_LONGS] }; let mut i = 0; while i < map_words(map) { r.x[i] = val1.x[i] & !val2.x[i]; i += 1; } r }
#[inline] pub unsafe fn map_word_or(map: *mut map_info, val1: map_word, val2: map_word) -> map_word { let mut r = map_word { x: [0; MAX_MAP_LONGS] }; let mut i = 0; while i < map_words(map) { r.x[i] = val1.x[i] | val2.x[i]; i += 1; } r }
#[inline] pub unsafe fn map_word_andequal(map: *mut map_info, val1: map_word, val2: map_word, val3: map_word) -> i32 { let mut i = 0; while i < map_words(map) { if (val1.x[i] & val2.x[i]) != val3.x[i] { return 0; } i += 1; } 1 }
#[inline] pub unsafe fn map_word_bitsset(map: *mut map_info, val1: map_word, val2: map_word) -> i32 { let mut i = 0; while i < map_words(map) { if val1.x[i] & val2.x[i] != 0 { return 1; } i += 1; } 0 }

// The remaining inline I/O helpers depend on the kernel's __raw_* and memcpy_{from,to}io primitives.
// They are declared here with the same interfaces; implementations are supplied by the surrounding translation.
pub unsafe fn map_word_load(_map: *mut map_info, _ptr: *const core::ffi::c_void) -> map_word { todo!("kernel I/O dependency") }
pub unsafe fn map_word_load_partial(_map: *mut map_info, orig: map_word, _buf: *const u8, _start: i32, _len: i32) -> map_word { orig }
pub unsafe fn map_word_ff(_map: *mut map_info) -> map_word { map_word { x: [usize::MAX; MAX_MAP_LONGS] } }
pub unsafe fn inline_map_read(_map: *mut map_info, _ofs: usize) -> map_word { todo!("kernel I/O dependency") }
pub unsafe fn inline_map_write(_map: *mut map_info, _datum: map_word, _ofs: usize) { todo!("kernel I/O dependency") }
pub unsafe fn inline_map_copy_from(_map: *mut map_info, _to: *mut core::ffi::c_void, _from: usize, _len: isize) { todo!("kernel I/O dependency") }
pub unsafe fn inline_map_copy_to(_map: *mut map_info, _to: usize, _from: *const core::ffi::c_void, _len: isize) { todo!("kernel I/O dependency") }

#[cfg(feature = "CONFIG_MTD_COMPLEX_MAPPINGS")]
unsafe extern "C" { pub fn simple_map_init(map: *mut map_info); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
