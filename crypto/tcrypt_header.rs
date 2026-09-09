/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Quick & dirty crypto benchmarking module.
 *
 * This will only exist until we have a better benchmarking mechanism
 * (e.g. a char device).
 *
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 * Copyright (c) 2002 Jean-Francois Dive <jef@linuxbe.org>
 * Copyright (c) 2007 Nokia Siemens Networks
 */

use core::ffi::c_char;

#[repr(C)]
struct cipher_speed_template {
    key: *const c_char,
    klen: u32,
}

#[repr(C)]
struct aead_speed_template {
    key: *const c_char,
    klen: u32,
}

#[repr(C)]
struct hash_speed {
    blen: u32, /* buffer length */
    plen: u32, /* per-update length */
}

/* DES test vectors. */
const DES3_SPEED_VECTORS: u32 = 1;

static DES3_KEY: [u8; 25] = [
    0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
    0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
    0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 0x00,
];

static mut des3_speed_template: [cipher_speed_template; 1] = [cipher_speed_template {
    key: DES3_KEY.as_ptr() as *const c_char,
    klen: 24,
}];

/* Cipher speed tests */
static mut speed_template_8: [u8; 2] = [8, 0];
static mut speed_template_16: [u8; 2] = [16, 0];
static mut speed_template_24: [u8; 2] = [24, 0];
static mut speed_template_8_16: [u8; 3] = [8, 16, 0];
static mut speed_template_8_32: [u8; 3] = [8, 32, 0];
static mut speed_template_16_32: [u8; 3] = [16, 32, 0];
static mut speed_template_16_24_32: [u8; 4] = [16, 24, 32, 0];
static mut speed_template_20_28_36: [u8; 4] = [20, 28, 36, 0];
static mut speed_template_32_40_48: [u8; 4] = [32, 40, 48, 0];
static mut speed_template_32_48: [u8; 3] = [32, 48, 0];
static mut speed_template_32_48_64: [u8; 4] = [32, 48, 64, 0];
static mut speed_template_32_64: [u8; 3] = [32, 64, 0];
static mut speed_template_32: [u8; 2] = [32, 0];

/* AEAD speed tests */
static mut aead_speed_template_19: [u8; 2] = [19, 0];
static mut aead_speed_template_20_28_36: [u8; 4] = [20, 28, 36, 0];
static mut aead_speed_template_36: [u8; 2] = [36, 0];

/* Digest speed tests */
static mut generic_hash_speed_template: [hash_speed; 23] = [
    hash_speed { blen: 16, plen: 16 },
    hash_speed { blen: 64, plen: 16 },
    hash_speed { blen: 64, plen: 64 },
    hash_speed { blen: 256, plen: 16 },
    hash_speed { blen: 256, plen: 64 },
    hash_speed { blen: 256, plen: 256 },
    hash_speed { blen: 1024, plen: 16 },
    hash_speed { blen: 1024, plen: 256 },
    hash_speed { blen: 1024, plen: 1024 },
    hash_speed { blen: 2048, plen: 16 },
    hash_speed { blen: 2048, plen: 256 },
    hash_speed { blen: 2048, plen: 1024 },
    hash_speed { blen: 2048, plen: 2048 },
    hash_speed { blen: 4096, plen: 16 },
    hash_speed { blen: 4096, plen: 256 },
    hash_speed { blen: 4096, plen: 1024 },
    hash_speed { blen: 4096, plen: 4096 },
    hash_speed { blen: 8192, plen: 16 },
    hash_speed { blen: 8192, plen: 256 },
    hash_speed { blen: 8192, plen: 1024 },
    hash_speed { blen: 8192, plen: 4096 },
    hash_speed { blen: 8192, plen: 8192 },
    /* End marker */
    hash_speed { blen: 0, plen: 0 },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
