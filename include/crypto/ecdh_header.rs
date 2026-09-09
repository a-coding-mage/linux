/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * ECDH params to be used with kpp API
 *
 * Copyright (c) 2016, Intel Corporation
 * Authors: Salvatore Benedetto <salvatore.benedetto@intel.com>
 */

/**
 * DOC: ECDH Helper Functions
 *
 * To use ECDH with the KPP cipher API, the following data structure and
 * functions should be used.
 *
 * The ECC curves known to the ECDH implementation are specified in this
 * header file.
 *
 * To use ECDH with KPP, the following functions should be used to operate on
 * an ECDH private key. The packet private key that can be set with
 * the KPP API function call of crypto_kpp_set_secret.
 */

/* Curves IDs */
pub const ECC_CURVE_NIST_P192: u32 = 0x0001;
pub const ECC_CURVE_NIST_P256: u32 = 0x0002;
pub const ECC_CURVE_NIST_P384: u32 = 0x0003;
pub const ECC_CURVE_NIST_P521: u32 = 0x0004;

/**
 * struct ecdh - define an ECDH private key
 *
 * @key:        Private ECDH key
 * @key_size:   Size of the private ECDH key
 */
#[repr(C)]
pub struct ecdh {
    pub key: *mut core::ffi::c_char,
    pub key_size: u16,
}

extern "C" {
    /**
     * crypto_ecdh_key_len() - Obtain the size of the private ECDH key
     * @params:     private ECDH key
     *
     * This function returns the packet ECDH key size. A caller can use that
     * with the provided ECDH private key reference to obtain the required
     * memory size to hold a packet key.
     *
     * Return: size of the key in bytes
     */
    pub fn crypto_ecdh_key_len(params: *const ecdh) -> u32;

    /**
     * crypto_ecdh_encode_key() - encode the private key
     * @buf:        Buffer allocated by the caller to hold the packet ECDH
     *              private key. The buffer should be at least crypto_ecdh_key_len
     *              bytes in size.
     * @len:        Length of the packet private key buffer
     * @p:          Buffer with the caller-specified private key
     *
     * The ECDH implementations operate on a packet representation of the private
     * key.
     *
     * Return:      -EINVAL if buffer has insufficient size, 0 on success
     */
    pub fn crypto_ecdh_encode_key(
        buf: *mut core::ffi::c_char,
        len: u32,
        p: *const ecdh,
    ) -> i32;

    /**
     * crypto_ecdh_decode_key() - decode a private key
     * @buf:        Buffer holding a packet key that should be decoded
     * @len:        Length of the packet private key buffer
     * @p:          Buffer allocated by the caller that is filled with the
     *              unpacked ECDH private key.
     *
     * The unpacking obtains the private key by pointing @p to the correct location
     * in @buf. Thus, both pointers refer to the same memory.
     *
     * Return:      -EINVAL if buffer has insufficient size, 0 on success
     */
    pub fn crypto_ecdh_decode_key(
        buf: *const core::ffi::c_char,
        len: u32,
        p: *mut ecdh,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
