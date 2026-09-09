/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency intent from the original header:
// #include <linux/types.h>
// #include <crypto/sm4.h>

pub type sm4_crypt_func = unsafe extern "C" fn(
    rk: *const u32,
    dst: *mut u8,
    src: *const u8,
    iv: *mut u8,
);

extern "C" {
    pub fn sm4_avx_ecb_encrypt(req: *mut skcipher_request) -> i32;
    pub fn sm4_avx_ecb_decrypt(req: *mut skcipher_request) -> i32;

    pub fn sm4_cbc_encrypt(req: *mut skcipher_request) -> i32;
    pub fn sm4_avx_cbc_decrypt(
        req: *mut skcipher_request,
        bsize: u32,
        func: sm4_crypt_func,
    ) -> i32;

    pub fn sm4_avx_ctr_crypt(
        req: *mut skcipher_request,
        bsize: u32,
        func: sm4_crypt_func,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
