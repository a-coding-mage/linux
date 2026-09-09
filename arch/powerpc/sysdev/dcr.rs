// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * (c) Copyright 2006 Benjamin Herrenschmidt, IBM Corp.
 *                    <benh@kernel.crashing.org>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/export.h, linux/of_address.h, and asm/dcr.h.

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

// External kernel types and functions supplied by other translated files.
extern "C" {
    pub fn of_get_property(
        np: *const device_node,
        name: *const core::ffi::c_char,
        length: *mut u32,
    ) -> *const u32;
}

pub type spinlock_t = core::ffi::c_ulong;

pub unsafe fn dcr_resource_start(np: *const device_node, index: u32) -> u32 {
    let mut ds: u32 = 0;
    let dr: *const u32 = of_get_property(
        np,
        b"dcr-reg\0".as_ptr() as *const core::ffi::c_char,
        &mut ds,
    );

    if dr.is_null() || (ds & 1) != 0 || index >= (ds / 8) {
        return 0;
    }

    *dr.add((index * 2) as usize)
}

// EXPORT_SYMBOL_GPL(dcr_resource_start);

pub unsafe fn dcr_resource_len(np: *const device_node, index: u32) -> u32 {
    let mut ds: u32 = 0;
    let dr: *const u32 = of_get_property(
        np,
        b"dcr-reg\0".as_ptr() as *const core::ffi::c_char,
        &mut ds,
    );

    if dr.is_null() || (ds & 1) != 0 || index >= (ds / 8) {
        return 0;
    }

    *dr.add((index * 2 + 1) as usize)
}

// EXPORT_SYMBOL_GPL(dcr_resource_len);

// DEFINE_SPINLOCK(dcr_ind_lock);
pub static mut dcr_ind_lock: spinlock_t = 0;
// EXPORT_SYMBOL_GPL(dcr_ind_lock);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
