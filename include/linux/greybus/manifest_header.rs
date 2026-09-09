/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Greybus manifest parsing
 *
 * Copyright 2014 Google Inc.
 * Copyright 2014 Linaro Ltd.
 */

// Dependency equivalent of: #include <linux/types.h>

#[repr(C)]
pub struct gb_interface {
    _private: [u8; 0],
}

pub unsafe extern "C" fn gb_manifest_parse(
    intf: *mut gb_interface,
    data: *mut core::ffi::c_void,
    size: usize,
) -> bool;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
