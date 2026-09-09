/* SPDX-License-Identifier: GPL-2.0+ OR BSD-2-Clause */
/*
 * Copyright (c) 2013 Alexey Degtyarev <alexey@renatasystems.org>
 * Copyright (c) 2018 Vitaly Chikunov <vt@altlinux.org>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation; either version 2 of the License, or (at your option)
 * any later version.
 */

// The original header includes <linux/types.h> for the __le64 type.

pub const STREEBOG256_DIGEST_SIZE: usize = 32;
pub const STREEBOG512_DIGEST_SIZE: usize = 64;
pub const STREEBOG_BLOCK_SIZE: usize = 64;

#[repr(C)]
pub struct streebog_uint512 {
    pub qword: [u64; 8],
}

#[repr(C)]
pub struct streebog_state {
    pub hash: streebog_uint512,
    pub h: streebog_uint512,
    pub N: streebog_uint512,
    pub Sigma: streebog_uint512,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
