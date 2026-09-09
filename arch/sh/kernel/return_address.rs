// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/return_address.c
 *
 * Copyright (C) 2009  Matt Fleming
 * Copyright (C) 2009  Paul Mundt
 */

use core::ffi::c_void;
use core::ptr;

// The C implementation is compiled conditionally by CONFIG_DWARF_UNWINDER.
// This Rust translation uses the corresponding Cargo configuration feature.
#[cfg(feature = "CONFIG_DWARF_UNWINDER")]
#[repr(C)]
pub struct dwarf_frame {
    pub return_addr: usize,
}

#[cfg(feature = "CONFIG_DWARF_UNWINDER")]
unsafe extern "C" {
    fn dwarf_unwind_stack(ra: usize, frame: *mut dwarf_frame) -> *mut dwarf_frame;
    fn dwarf_free_frame(frame: *mut dwarf_frame);
}

#[cfg(feature = "CONFIG_DWARF_UNWINDER")]
pub unsafe fn return_address(depth: u32) -> *mut c_void {
    let mut frame: *mut dwarf_frame;
    let mut ra: usize;
    let mut i: i32;

    i = 0;
    frame = ptr::null_mut();
    ra = 0;
    while i <= depth as i32 {
        let tmp: *mut dwarf_frame;

        tmp = dwarf_unwind_stack(ra, frame);
        if tmp.is_null() {
            return ptr::null_mut();
        }

        if !frame.is_null() {
            dwarf_free_frame(frame);
        }

        frame = tmp;

        if frame.is_null() || (*frame).return_addr == 0 {
            break;
        }

        ra = (*frame).return_addr;
        i += 1;
    }

    // Failed to unwind the stack to the specified depth.
    // Equivalent to WARN_ON(i != depth + 1); the kernel warning facility is external.

    if !frame.is_null() {
        dwarf_free_frame(frame);
    }

    ra as *mut c_void
}

#[cfg(not(feature = "CONFIG_DWARF_UNWINDER"))]
pub unsafe fn return_address(_depth: u32) -> *mut c_void {
    ptr::null_mut()
}

// EXPORT_SYMBOL_GPL(return_address);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
