// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RSA key extract helper
 *
 * Copyright (c) 2015, Intel Corporation
 * Authors: Tadeusz Struk <tadeusz.struk@intel.com>
 */

use core::ffi::c_void;

// Linux kernel dependencies supplied by the surrounding translation unit.
#[repr(C)]
pub struct rsa_key {
    pub n: *const u8,
    pub n_sz: usize,
    pub e: *const u8,
    pub e_sz: usize,
    pub d: *const u8,
    pub d_sz: usize,
    pub p: *const u8,
    pub p_sz: usize,
    pub q: *const u8,
    pub q_sz: usize,
    pub dp: *const u8,
    pub dp_sz: usize,
    pub dq: *const u8,
    pub dq_sz: usize,
    pub qinv: *const u8,
    pub qinv_sz: usize,
}

#[repr(C)]
pub struct Asn1Decoder {
    _private: [u8; 0],
}

extern "C" {
    pub static fips_enabled: bool;
    pub static rsapubkey_decoder: Asn1Decoder;
    pub static rsaprivkey_decoder: Asn1Decoder;
    pub fn asn1_ber_decoder(
        decoder: *const Asn1Decoder,
        context: *mut c_void,
        data: *const c_void,
        data_len: usize,
    ) -> i32;
    pub fn pr_err(message: *const u8, ...);
}

const EINVAL: i32 = 22;

pub unsafe extern "C" fn rsa_get_n(
    context: *mut c_void,
    _hdrlen: usize,
    _tag: u8,
    value: *const c_void,
    vlen: usize,
) -> i32 {
    let key = &mut *(context as *mut rsa_key);
    let mut ptr = value as *const u8;
    let mut n_sz = vlen;

    /* invalid key provided */
    if value.is_null() || vlen == 0 {
        return -EINVAL;
    }

    if fips_enabled {
        while n_sz != 0 && *ptr == 0 {
            ptr = ptr.add(1);
            n_sz -= 1;
        }

        /* In FIPS mode only allow key size 2K and higher */
        if n_sz < 256 {
            pr_err(b"RSA: key size not allowed in FIPS mode\0".as_ptr());
            return -EINVAL;
        }
    }

    key.n = value as *const u8;
    key.n_sz = vlen;
    0
}

pub unsafe extern "C" fn rsa_get_e(
    context: *mut c_void,
    _hdrlen: usize,
    _tag: u8,
    value: *const c_void,
    vlen: usize,
) -> i32 {
    let key = &mut *(context as *mut rsa_key);
    /* invalid key provided */
    if value.is_null() || key.n_sz == 0 || vlen == 0 || vlen > key.n_sz { return -EINVAL; }
    key.e = value as *const u8; key.e_sz = vlen; 0
}

macro_rules! rsa_get_component {
    ($name:ident, $field:ident, $size:ident) => {
        pub unsafe extern "C" fn $name(
            context: *mut c_void, _hdrlen: usize, _tag: u8,
            value: *const c_void, vlen: usize,
        ) -> i32 {
            let key = &mut *(context as *mut rsa_key);
            /* invalid key provided */
            if value.is_null() || vlen == 0 || vlen > key.n_sz { return -EINVAL; }
            key.$field = value as *const u8;
            key.$size = vlen;
            0
        }
    };
}

rsa_get_component!(rsa_get_d, d, d_sz);
rsa_get_component!(rsa_get_p, p, p_sz);
rsa_get_component!(rsa_get_q, q, q_sz);
rsa_get_component!(rsa_get_dp, dp, dp_sz);
rsa_get_component!(rsa_get_dq, dq, dq_sz);
rsa_get_component!(rsa_get_qinv, qinv, qinv_sz);

/**
 * rsa_parse_pub_key() - decodes the BER encoded buffer and stores in the
 *                       provided struct rsa_key, pointers to the raw key as is,
 *                       so that the caller can copy it or MPI parse it, etc.
 *
 * @rsa_key: struct rsa_key key representation
 * @key: key in BER format
 * @key_len: length of key
 *
 * Return: 0 on success or error code in case of error
 */
pub unsafe extern "C" fn rsa_parse_pub_key(
    rsa_key: *mut rsa_key, key: *const c_void, key_len: u32,
) -> i32 {
    asn1_ber_decoder(&rsapubkey_decoder, rsa_key as *mut c_void, key, key_len as usize)
}

/** See `rsa_parse_pub_key` for the corresponding private-key operation. */
pub unsafe extern "C" fn rsa_parse_priv_key(
    rsa_key: *mut rsa_key, key: *const c_void, key_len: u32,
) -> i32 {
    asn1_ber_decoder(&rsaprivkey_decoder, rsa_key as *mut c_void, key, key_len as usize)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
