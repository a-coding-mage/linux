/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2020 Samsung Electronics Co., Ltd.
 * Copyright 2020 Google LLC.
 * Copyright 2024 Linaro Ltd.
 */

// The C header guard is omitted; Rust items are guarded by the module system.

#[repr(C)]
pub struct acpm_xfer {
    // C: const u32 *txd __counted_by_ptr(txcnt)
    pub txd: *const u32,
    // C: u32 *rxd __counted_by_ptr(rxcnt)
    pub rxd: *mut u32,
    pub txcnt: usize,
    pub rxcnt: usize,
    pub acpm_chan_id: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct acpm_handle {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn acpm_set_xfer(
        xfer: *mut acpm_xfer,
        cmd: *mut u32,
        cmdcnt: usize,
        acpm_chan_id: ::core::ffi::c_uint,
        response: bool,
    );

    pub fn acpm_do_xfer(
        handle: *mut acpm_handle,
        xfer: *const acpm_xfer,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
