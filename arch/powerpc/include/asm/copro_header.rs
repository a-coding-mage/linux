/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2014 IBM Corp.
 */

// Dependency supplied by the surrounding kernel translation:
// #include <linux/mm_types.h>

#[repr(C)]
pub struct copro_slb {
    pub esid: u64,
    pub vsid: u64,
}

extern "C" {
    pub fn copro_handle_mm_fault(
        mm: *mut crate::mm_struct,
        ea: ::core::ffi::c_ulong,
        dsisr: ::core::ffi::c_ulong,
        flt: *mut crate::vm_fault_t,
    ) -> ::core::ffi::c_int;

    pub fn copro_calculate_slb(
        mm: *mut crate::mm_struct,
        ea: u64,
        slb: *mut copro_slb,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
