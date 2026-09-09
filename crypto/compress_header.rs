/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Cryptographic API.
 *
 * Copyright 2015 LG Electronics Inc.
 * Copyright (c) 2016, Intel Corporation
 * Copyright (c) 2023 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependency supplied by "internal.h" in the C source.
use crate::crypto_tfm;

#[repr(C)]
pub struct acomp_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct comp_alg_common {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn crypto_init_scomp_ops_async(tfm: *mut crypto_tfm) -> i32;

    pub fn comp_prepare_alg(alg: *mut comp_alg_common);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
