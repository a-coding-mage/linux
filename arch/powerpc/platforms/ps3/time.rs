// SPDX-License-Identifier: GPL-2.0-only
/*
 *  PS3 time and rtc routines.
 *
 *  Copyright (C) 2006 Sony Computer Entertainment Inc.
 *  Copyright 2006 Sony Corp.
 */

// The declarations below are supplied by the corresponding kernel headers and
// other translation units.
extern "C" {
    fn ps3_repository_read_be_tb_freq(lpar: u64, tb_freq: *mut u64) -> i32;
    fn lv1_get_rtc(rtc_val: *mut u64, tb_val: *mut u64) -> i32;
    fn ps3_os_area_get_rtc_diff() -> u64;
    fn firmware_has_feature(feature: u64) -> bool;
    fn platform_device_register_simple(
        name: *const core::ffi::c_char,
        id: i32,
        res: *const core::ffi::c_void,
        num: u32,
    ) -> *mut platform_device;
    fn ptr_err_or_zero(ptr: *mut platform_device) -> i32;
    fn bug_on(condition: bool);
}

// Supplied by the platform/kernel environment.
extern "C" {
    static mut ppc_tb_freq: u64;
    static mut ppc_proc_freq: u64;
}

// These names and types are supplied by the platform headers.
#[allow(non_camel_case_types)]
type time64_t = i64;

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

extern "C" {
    static FW_FEATURE_PS3_LV1: u64;
}

pub unsafe fn ps3_calibrate_decr() {
    let mut result: i32;
    let mut tmp: u64 = 0;

    result = ps3_repository_read_be_tb_freq(0, &mut tmp);
    bug_on(result != 0);

    ppc_tb_freq = tmp;
    ppc_proc_freq = ppc_tb_freq.wrapping_mul(40);
}

unsafe fn read_rtc() -> u64 {
    let mut result: i32;
    let mut rtc_val: u64 = 0;
    let mut tb_val: u64 = 0;

    result = lv1_get_rtc(&mut rtc_val, &mut tb_val);
    bug_on(result != 0);

    rtc_val
}

pub unsafe fn ps3_get_boot_time() -> time64_t {
    (read_rtc() as time64_t).wrapping_add(ps3_os_area_get_rtc_diff() as time64_t)
}

unsafe fn ps3_rtc_init() -> i32 {
    let pdev: *mut platform_device;

    if !firmware_has_feature(FW_FEATURE_PS3_LV1) {
        return -19; // -ENODEV
    }

    pdev = platform_device_register_simple(
        b"rtc-ps3\0".as_ptr() as *const core::ffi::c_char,
        -1,
        core::ptr::null(),
        0,
    );

    ptr_err_or_zero(pdev)
}

// device_initcall(ps3_rtc_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
