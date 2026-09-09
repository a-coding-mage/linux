/* SPDX-License-Identifier: BSD-2-Clause */
/*
 * Copyright 2019 Broadcom.
 */

// Translated from the C header. The Linux `u32` type is supplied by the
// surrounding dependency environment.

extern "C" {
    pub fn tee_bnxt_fw_load() -> ::core::ffi::c_int;
    pub fn tee_bnxt_copy_coredump(
        buf: *mut ::core::ffi::c_void,
        offset: u32,
        size: u32,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
