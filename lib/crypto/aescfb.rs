// SPDX-License-Identifier: GPL-2.0
/*
 * Minimal library implementation of AES in CFB mode
 *
 * Copyright 2023 Google LLC
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    pub fn aes_encrypt(key: *const aes_enckey, dst: *mut u8, src: *const u8);
    pub fn crypto_xor_cpy(dst: *mut u8, src: *const u8, len: *const u8, n: usize);
    pub fn memzero_explicit(s: *mut core::ffi::c_void, n: usize);
}

#[repr(C)]
pub struct aes_enckey {
    _private: [u8; 0],
}

pub const AES_BLOCK_SIZE: usize = 16;

/**
 * aescfb_encrypt - Perform AES-CFB encryption on a block of data
 *
 * @key: The AES-CFB key schedule
 * @dst: Pointer to the ciphertext output buffer
 * @src: Pointer the plaintext (may equal @dst for encryption in place)
 * @len: The size in bytes of the plaintext and ciphertext.
 * @iv:  The initialization vector (IV) to use for this block of data
 */
pub unsafe fn aescfb_encrypt(
    key: *const aes_enckey,
    mut dst: *mut u8,
    mut src: *const u8,
    mut len: i32,
    iv: *const u8,
) {
    let mut ks = [0u8; AES_BLOCK_SIZE];
    let mut v = iv;

    while len > 0 {
        aes_encrypt(key, ks.as_mut_ptr(), v);
        crypto_xor_cpy(dst, src, ks.as_ptr(), core::cmp::min(len as usize, AES_BLOCK_SIZE));
        v = dst;

        dst = dst.add(AES_BLOCK_SIZE);
        src = src.add(AES_BLOCK_SIZE);
        len -= AES_BLOCK_SIZE as i32;
    }

    memzero_explicit(ks.as_mut_ptr() as *mut core::ffi::c_void, core::mem::size_of_val(&ks));
}

/**
 * aescfb_decrypt - Perform AES-CFB decryption on a block of data
 *
 * @key: The AES-CFB key schedule
 * @dst: Pointer to the plaintext output buffer
 * @src: Pointer the ciphertext (may equal @dst for decryption in place)
 * @len: The size in bytes of the plaintext and ciphertext.
 * @iv:  The initialization vector (IV) to use for this block of data
 */
pub unsafe fn aescfb_decrypt(
    key: *const aes_enckey,
    mut dst: *mut u8,
    mut src: *const u8,
    mut len: i32,
    iv: *const u8,
) {
    let mut ks = [[0u8; AES_BLOCK_SIZE]; 2];

    aes_encrypt(key, ks[0].as_mut_ptr(), iv);

    let mut i = 0usize;
    while len > 0 {
        if len > AES_BLOCK_SIZE as i32 {
            /* Generate the keystream for the next block before performing
             * the XOR, as that may update in place and overwrite ciphertext. */
            aes_encrypt(key, ks[1 - i].as_mut_ptr(), src);
        }

        crypto_xor_cpy(dst, src, ks[i].as_ptr(), core::cmp::min(len as usize, AES_BLOCK_SIZE));

        dst = dst.add(AES_BLOCK_SIZE);
        src = src.add(AES_BLOCK_SIZE);
        len -= AES_BLOCK_SIZE as i32;
        i ^= 1;
    }

    memzero_explicit(ks.as_mut_ptr() as *mut core::ffi::c_void, core::mem::size_of_val(&ks));
}

// EXPORT_SYMBOL(aescfb_encrypt);
// EXPORT_SYMBOL(aescfb_decrypt);
// MODULE_DESCRIPTION("Generic AES-CFB library");
// MODULE_AUTHOR("Ard Biesheuvel <ardb@kernel.org>");
// MODULE_LICENSE("GPL");

// CONFIG_CRYPTO_SELFTESTS conditionally includes the following test code.

#[cfg(feature = "CONFIG_CRYPTO_SELFTESTS")]
#[repr(C)]
struct AescfbTv {
    ptext: [u8; 64],
    ctext: [u8; 64],
    key: [u8; 32],
    iv: [u8; AES_BLOCK_SIZE],
    klen: i32,
    len: i32,
}

#[cfg(feature = "CONFIG_CRYPTO_SELFTESTS")]
static AESCfb_TV: &[AescfbTv] = &[
    AescfbTv {
        key: [0x2b,0x7e,0x15,0x16,0x28,0xae,0xd2,0xa6,0xab,0xf7,0x15,0x88,0x09,0xcf,0x4f,0x3c,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        klen: 16, iv: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15],
        ptext: [0x6b,0xc1,0xbe,0xe2,0x2e,0x40,0x9f,0x96,0xe9,0x3d,0x7e,0x11,0x73,0x93,0x17,0x2a,0xae,0x2d,0x8a,0x57,0x1e,0x03,0xac,0x9c,0x9e,0xb7,0x6f,0xac,0x45,0xaf,0x8e,0x51,0x30,0xc8,0x1c,0x46,0xa3,0x5c,0xe4,0x11,0xe5,0xfb,0xc1,0x19,0x1a,0x0a,0x52,0xef,0xf6,0x9f,0x24,0x45,0xdf,0x4f,0x9b,0x17,0xad,0x2b,0x41,0x7b,0xe6,0x6c,0x37,0x10],
        ctext: [0x3b,0x3f,0xd9,0x2e,0xb7,0x2d,0xad,0x20,0x33,0x34,0x49,0xf8,0xe8,0x3c,0xfb,0x4a,0xc8,0xa6,0x45,0x37,0xa0,0xb3,0xa9,0x3f,0xcd,0xe3,0xcd,0xad,0x9f,0x1c,0xe5,0x8b,0x26,0x75,0x1f,0x67,0xa3,0xcb,0xb1,0x40,0xb1,0x80,0x8c,0xf1,0x87,0xa4,0xf4,0xdf,0xc0,0x4b,0x05,0x35,0x7c,0x5d,0x1c,0x0e,0xea,0xc4,0xc6,0x6f,0x9f,0xf7,0xf2,0xe6], len: 64,
    },
];

// The remaining vectors are identical in structure to the kernel test table;
// their declarations remain conditional on CONFIG_CRYPTO_SELFTESTS.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
