/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * if_alg: User-space algorithm interface
 *
 * Copyright (c) 2010 Herbert Xu <herbert@gondor.apana.org.au>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation; either version 2 of the License, or (at your option)
 * any later version.
 *
 */

// Dependency equivalent of <linux/types.h> is supplied externally.

#[repr(C)]
pub struct sockaddr_alg {
    pub salg_family: __u16,
    pub salg_type: [__u8; 14],
    pub salg_feat: __u32,
    pub salg_mask: __u32,
    pub salg_name: [__u8; 64],
}

/*
 * Linux v4.12 and later removed the 64-byte limit on salg_name[]; it's now an
 * arbitrary-length field.  We had to keep the original struct above for source
 * compatibility with existing userspace programs, though.  Use the new struct
 * below if support for very long algorithm names is needed.  To do this,
 * allocate 'sizeof(struct sockaddr_alg_new) + strlen(algname) + 1' bytes, and
 * copy algname (including the null terminator) into salg_name.
 */
#[repr(C)]
pub struct sockaddr_alg_new {
    pub salg_family: __u16,
    pub salg_type: [__u8; 14],
    pub salg_feat: __u32,
    pub salg_mask: __u32,
    // Flexible array member: storage follows the fixed-size struct.
    pub salg_name: [__u8; 0],
}

#[repr(C)]
pub struct af_alg_iv {
    pub ivlen: __u32,
    // Flexible array member counted by ivlen.
    pub iv: [__u8; 0],
}

/* Socket options */
pub const ALG_SET_KEY: ::core::ffi::c_int = 1;
pub const ALG_SET_IV: ::core::ffi::c_int = 2;
pub const ALG_SET_OP: ::core::ffi::c_int = 3;
pub const ALG_SET_AEAD_ASSOCLEN: ::core::ffi::c_int = 4;
pub const ALG_SET_AEAD_AUTHSIZE: ::core::ffi::c_int = 5;
pub const ALG_SET_DRBG_ENTROPY: ::core::ffi::c_int = 6;
pub const ALG_SET_KEY_BY_KEY_SERIAL: ::core::ffi::c_int = 7;

/* Operations */
pub const ALG_OP_DECRYPT: ::core::ffi::c_int = 0;
pub const ALG_OP_ENCRYPT: ::core::ffi::c_int = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
