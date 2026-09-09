/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 Western Digital Corporation or its affiliates.
 */

// Dependency intent from <asm-generic/sections.h> and <linux/mm.h>.

use core::ffi::c_char;

extern "C" {
    static mut _start: [c_char; 0];
    static mut _start_kernel: [c_char; 0];
    static mut __init_data_begin: [c_char; 0];
    static mut __init_data_end: [c_char; 0];
    static mut __init_text_begin: [c_char; 0];
    static mut __init_text_end: [c_char; 0];
    static mut __alt_start: [c_char; 0];
    static mut __alt_end: [c_char; 0];
    static mut __exittext_begin: [c_char; 0];
    static mut __exittext_end: [c_char; 0];

    // Supplied by the Linux memory-management dependency.
    fn lm_alias(addr: *mut c_char) -> *mut c_char;
}

pub unsafe fn is_va_kernel_text(va: usize) -> bool {
    let start = core::ptr::addr_of_mut!(_start) as usize;
    let end = core::ptr::addr_of_mut!(__init_data_begin) as usize;

    va >= start && va < end
}

pub unsafe fn is_va_kernel_lm_alias_text(va: usize) -> bool {
    let start = lm_alias(core::ptr::addr_of_mut!(_start) as *mut c_char) as usize;
    let end = lm_alias(core::ptr::addr_of_mut!(__init_data_begin) as *mut c_char) as usize;

    va >= start && va < end
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
