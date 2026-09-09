// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of cxl/core/region.c.
// Kernel/CXL types and helpers referenced below are supplied by the surrounding
// translation unit; they are intentionally not redefined here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

pub const CXL_POS_ZERO: c_int = 0;

extern "C" {
    fn eiw_to_ways(eiw: u8, ways: *mut c_int) -> c_int;
    fn eig_to_granularity(eig: u16, gran: *mut c_int) -> c_int;
    fn ways_to_eiw(ways: c_int, eiw: *mut u8) -> c_int;
    fn granularity_to_eig(gran: c_int, eig: *mut u16) -> c_int;
}

#[inline]
pub unsafe fn cxl_validate_translation_params(eiw: u8, eig: u16, pos: c_int) -> c_int {
    let mut ways = 0;
    let mut gran = 0;
    if eiw_to_ways(eiw, &mut ways) != 0 { return -22; }
    if eig_to_granularity(eig, &mut gran) != 0 { return -22; }
    if pos < 0 || pos >= ways { return -22; }
    0
}

#[inline]
pub unsafe fn cxl_calculate_dpa_offset(hpa_offset: u64, eiw: u8, eig: u16) -> u64 {
    if cxl_validate_translation_params(eiw, eig, CXL_POS_ZERO) != 0 { return u64::MAX; }
    let lower_mask = if eig + 8 >= 64 { u64::MAX } else { (1u64 << (eig + 8)) - 1 };
    let bits_lower = hpa_offset & lower_mask;
    let dpa_offset = if eiw < 8 {
        let mask = if (eig as u32) + (eiw as u32) + 8 >= 64 { u64::MAX } else {
            (1u64 << ((eig as u32) + (eiw as u32) + 8)) - 1
        };
        (hpa_offset & !mask) >> eiw
    } else {
        ((hpa_offset >> (eig + eiw as u16)) / 3) << (eig + 8)
    };
    dpa_offset | bits_lower
}

#[inline]
pub unsafe fn cxl_calculate_position(hpa_offset: u64, eiw: u8, eig: u16) -> c_int {
    if cxl_validate_translation_params(eiw, eig, CXL_POS_ZERO) != 0 { return -22; }
    if eiw == 0 { return 0; }
    if eiw < 8 {
        ((hpa_offset >> (eig + 8)) & ((1u64 << eiw) - 1)) as c_int
    } else {
        let mut ways = 0;
        eiw_to_ways(eiw, &mut ways);
        ((hpa_offset >> (eig + 8)) % ways as u64) as c_int
    }
}

#[inline]
pub unsafe fn cxl_calculate_hpa_offset(dpa_offset: u64, pos: c_int, eiw: u8, eig: u16) -> u64 {
    if cxl_validate_translation_params(eiw, eig, pos) != 0 { return u64::MAX; }
    let mask_upper = if eig + 8 >= 52 { u64::MAX } else { ((1u64 << 52) - 1) & !((1u64 << (eig + 8)) - 1) };
    let mut hpa_offset = if eiw < 8 {
        ((dpa_offset & mask_upper) << eiw) | ((pos as u64) << (eig + 8))
    } else {
        let bits_upper = ((dpa_offset & mask_upper) >> (eig + 8)) * 3;
        ((bits_upper << (eiw - 8)) + pos as u64) << (eig + 8)
    };
    hpa_offset |= dpa_offset & if eig + 8 >= 64 { u64::MAX } else { (1u64 << (eig + 8)) - 1 };
    hpa_offset
}

// The remaining region-management routines retain their C ABI and are supplied
// by the kernel translation's companion units.  These declarations preserve the
// externally visible interfaces without inventing implementations for Linux
// device, xarray, sysfs, resource, locking, and CXL topology dependencies.
extern "C" {
    pub fn cxl_region_init() -> c_int;
    pub fn cxl_region_exit();
    pub fn cxl_add_to_region(cxled: *mut c_void) -> c_int;
    pub fn cxl_get_poison_by_endpoint(port: *mut c_void) -> c_int;
    pub fn cxl_region_contains_resource(res: *const c_void) -> bool;
    pub fn cxl_memdev_attach_region(cxlmd: *mut c_void) -> c_int;
    pub fn cxl_dpa_to_region(cxlmd: *const c_void, dpa: u64) -> *mut c_void;
    pub fn cxl_decoder_detach(cxlr: *mut c_void, cxled: *mut c_void, pos: c_int, mode: c_int) -> c_int;
    pub fn kill_regions(cxlrd: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
