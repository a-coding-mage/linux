/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR Linux-OpenIB) */
/*
 * Copyright (c) 2016-2017, Mellanox Technologies. All rights reserved.
 *
 * This software is available to you under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

/* Dependency intent from C: #include <linux/types.h> */

/* TLS socket options */
pub const TLS_TX: u32 = 1; /* Set transmit parameters */
pub const TLS_RX: u32 = 2; /* Set receive parameters */

/* Supported versions */
pub const fn TLS_VERSION_MINOR(ver: u32) -> u32 {
    ver & 0xFF
}

pub const fn TLS_VERSION_MAJOR(ver: u32) -> u32 {
    (ver >> 8) & 0xFF
}

/*
 * C macro intent:
 * #define TLS_VERSION_NUMBER(id) ((((id##_VERSION_MAJOR) & 0xFF) << 8) | \
 *                                ((id##_VERSION_MINOR) & 0xFF))
 */
pub const fn TLS_VERSION_NUMBER(major: u32, minor: u32) -> u32 {
    ((major & 0xFF) << 8) | (minor & 0xFF)
}

pub const TLS_1_2_VERSION_MAJOR: u32 = 0x3;
pub const TLS_1_2_VERSION_MINOR: u32 = 0x3;
pub const TLS_1_2_VERSION: u32 = TLS_VERSION_NUMBER(TLS_1_2_VERSION_MAJOR, TLS_1_2_VERSION_MINOR);

/* Supported ciphers */
pub const TLS_CIPHER_AES_GCM_128: u32 = 51;
pub const TLS_CIPHER_AES_GCM_128_IV_SIZE: usize = 8;
pub const TLS_CIPHER_AES_GCM_128_KEY_SIZE: usize = 16;
pub const TLS_CIPHER_AES_GCM_128_SALT_SIZE: usize = 4;
pub const TLS_CIPHER_AES_GCM_128_TAG_SIZE: usize = 16;
pub const TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE: usize = 8;

pub const TLS_SET_RECORD_TYPE: u32 = 1;
pub const TLS_GET_RECORD_TYPE: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct tls_crypto_info {
    pub version: u16,
    pub cipher_type: u16,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct tls12_crypto_info_aes_gcm_128 {
    pub info: tls_crypto_info,
    pub iv: [u8; TLS_CIPHER_AES_GCM_128_IV_SIZE],
    pub key: [u8; TLS_CIPHER_AES_GCM_128_KEY_SIZE],
    pub salt: [u8; TLS_CIPHER_AES_GCM_128_SALT_SIZE],
    pub rec_seq: [u8; TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE],
}
