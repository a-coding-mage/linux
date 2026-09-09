/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

use core::ffi::{c_int, c_ulong, c_ulonglong, c_void};

extern "C" {
    pub fn phys_mapping(phys: c_ulong, offset_out: *mut c_ulonglong) -> c_int;

    pub static mut uml_physmem: c_ulong;
}

pub unsafe fn uml_to_phys(virt: *mut c_void) -> c_ulong {
    (virt as usize as c_ulong).wrapping_sub(uml_physmem)
}

pub unsafe fn uml_to_virt(phys: c_ulong) -> *mut c_void {
    (uml_physmem.wrapping_add(phys) as usize) as *mut c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
