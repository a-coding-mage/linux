/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Some small helpers for older Cirrus Logic parts.
 *
 * Copyright (C) 2021 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

pub unsafe fn cirrus_read_device_id(
    regmap: *mut regmap,
    reg: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut devid: [u8; 3] = [0; 3];
    let ret: ::core::ffi::c_int;

    ret = unsafe {
        regmap_bulk_read(
            regmap,
            reg,
            devid.as_mut_ptr() as *mut ::core::ffi::c_void,
            devid.len(),
        )
    };
    if ret < 0 {
        return ret;
    }

    ((((devid[0] as ::core::ffi::c_int) & 0xFF) << 12)
        | (((devid[1] as ::core::ffi::c_int) & 0xFF) << 4)
        | (((devid[2] as ::core::ffi::c_int) & 0xF0) >> 4))
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
