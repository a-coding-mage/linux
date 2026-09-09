// SPDX-License-Identifier: GPL-2.0-only
/*
 * Suspend support specific for mips.
 *
 * Copyright (C) 2009 Lemote Inc.
 * Author: Hu Hongbing <huhb@lemote.com>
 *	   Wu Zhangjin <wuzhangjin@gmail.com>
 */

// Dependencies supplied by the corresponding kernel headers.
extern "C" {
    fn read_c0_status() -> u32;
    fn write_c0_status(status: u32);
    fn is_fpu_owner() -> bool;
    fn save_fp(task: *mut core::ffi::c_void);
    fn restore_fp(task: *mut core::ffi::c_void);
    fn save_dsp(task: *mut core::ffi::c_void);
    fn restore_dsp(task: *mut core::ffi::c_void);
    static mut current: *mut core::ffi::c_void;
    static mut __nosave_begin: u8;
    static mut __nosave_end: u8;
    fn __pa(address: usize) -> usize;
    static PAGE_SHIFT: usize;
}

static mut saved_status: u32 = 0;

#[repr(C)]
pub struct pt_regs {
    _opaque: [u8; 0],
}

pub static mut saved_regs: pt_regs = pt_regs { _opaque: [] };

pub unsafe fn save_processor_state() {
    saved_status = read_c0_status();

    if is_fpu_owner() {
        save_fp(current);
    }

    save_dsp(current);
}

pub unsafe fn restore_processor_state() {
    write_c0_status(saved_status);

    if is_fpu_owner() {
        restore_fp(current);
    }

    restore_dsp(current);
}

pub unsafe fn pfn_is_nosave(pfn: usize) -> i32 {
    let nosave_begin_pfn = (__pa((&raw const __nosave_begin) as usize)) >> PAGE_SHIFT;
    let nosave_end_pfn =
        ((__pa((&raw const __nosave_end) as usize) + ((1usize << PAGE_SHIFT) - 1)) >> PAGE_SHIFT);

    if (pfn >= nosave_begin_pfn) && (pfn < nosave_end_pfn) {
        1
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
