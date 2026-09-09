/* SPDX-License-Identifier: GPL-2.0-or-later */
/* keyctl kernel bits
 *
 * Copyright (C) 2016 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependency declarations from <uapi/linux/keyctl.h> are supplied externally. */

#[repr(C)]
pub struct kernel_pkey_query {
    pub supported_ops: __u32, /* Which ops are supported */
    pub key_size: __u32,      /* Size of the key in bits */
    pub max_data_size: __u16, /* Maximum size of raw data to sign in bytes */
    pub max_sig_size: __u16,  /* Maximum size of signature in bytes */
    pub max_enc_size: __u16,  /* Maximum size of encrypted blob in bytes */
    pub max_dec_size: __u16,  /* Maximum size of decrypted blob in bytes */
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum kernel_pkey_operation {
    kernel_pkey_encrypt,
    kernel_pkey_decrypt,
    kernel_pkey_sign,
    kernel_pkey_verify,
}

#[repr(C)]
pub union kernel_pkey_params_lengths {
    pub out_len: __u32, /* Output buffer size (enc/dec/sign) */
    pub in2_len: __u32, /* 2nd input data size (verify) */
}

#[repr(C)]
pub struct kernel_pkey_params {
    pub key: *mut key,
    pub encoding: *const ::std::os::raw::c_char, /* Encoding (eg. "oaep" or "raw" for none) */
    pub hash_algo: *const ::std::os::raw::c_char, /* Digest algorithm used (eg. "sha1") or NULL if N/A */
    pub info: *mut ::std::os::raw::c_char, /* Modified info string to be released later */
    pub in_len: __u32,                       /* Input data size */
    pub lengths: kernel_pkey_params_lengths,
    /* enum kernel_pkey_operation op : 8; */
    pub op: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
