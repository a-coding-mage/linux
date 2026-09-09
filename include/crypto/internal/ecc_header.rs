/*
 * Copyright (c) 2013, Kenneth MacKay
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *  * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

// Dependencies: crypto/ecc_curve.h and linux/unaligned.h.

/* One digit is u64 qword. */
pub const ECC_CURVE_NIST_P192_DIGITS: usize = 3;
pub const ECC_CURVE_NIST_P256_DIGITS: usize = 4;
pub const ECC_CURVE_NIST_P384_DIGITS: usize = 6;
pub const ECC_CURVE_NIST_P521_DIGITS: usize = 9;
pub const ECC_MAX_DIGITS: usize = (521 + 64 - 1) / 64;
pub const ECC_DIGITS_TO_BYTES_SHIFT: usize = 3;
pub const ECC_MAX_BYTES: usize = ECC_MAX_DIGITS << ECC_DIGITS_TO_BYTES_SHIFT;

#[repr(C)]
pub struct ecdsa_raw_sig {
    pub r: [u64; ECC_MAX_DIGITS],
    pub s: [u64; ECC_MAX_DIGITS],
}

extern "C" {
    fn get_unaligned_be64(p: *const u8) -> u64;

    pub fn ecc_digits_from_bytes(input: *const u8, nbytes: u32,
                                 out: *mut u64, ndigits: u32);
    pub fn ecc_is_key_valid(curve_id: u32, ndigits: u32,
                            private_key: *const u64, private_key_len: u32) -> i32;
    pub fn ecc_gen_privkey(curve_id: u32, ndigits: u32, private_key: *mut u64) -> i32;
    pub fn ecc_make_pub_key(curve_id: u32, ndigits: u32,
                            private_key: *const u64, public_key: *mut u64) -> i32;
    pub fn crypto_ecdh_shared_secret(curve_id: u32, ndigits: u32,
                                     private_key: *const u64, public_key: *const u64,
                                     secret: *mut u64) -> i32;
    pub fn ecc_is_pubkey_valid_partial(curve: *const ecc_curve,
                                       pk: *mut ecc_point) -> i32;
    pub fn ecc_is_pubkey_valid_full(curve: *const ecc_curve, pk: *mut ecc_point) -> i32;
    pub fn vli_is_zero(vli: *const u64, ndigits: u32) -> bool;
    pub fn vli_cmp(left: *const u64, right: *const u64, ndigits: u32) -> i32;
    pub fn vli_sub(result: *mut u64, left: *const u64, right: *const u64,
                   ndigits: u32) -> u64;
    pub fn vli_from_be64(dest: *mut u64, src: *const core::ffi::c_void, ndigits: u32);
    pub fn vli_from_le64(dest: *mut u64, src: *const core::ffi::c_void, ndigits: u32);
    pub fn vli_mod_inv(result: *mut u64, input: *const u64, modulus: *const u64,
                       ndigits: u32);
    pub fn vli_mod_mult_slow(result: *mut u64, left: *const u64, right: *const u64,
                             modulus: *const u64, ndigits: u32);
    pub fn vli_num_bits(vli: *const u64, ndigits: u32) -> u32;
    pub fn ecc_alloc_point(ndigits: u32) -> *mut ecc_point;
    pub fn ecc_free_point(p: *mut ecc_point);
    pub fn ecc_point_is_zero(point: *const ecc_point) -> bool;
    pub fn ecc_point_mult_shamir(result: *const ecc_point, x: *const u64,
                                 p: *const ecc_point, y: *const u64,
                                 q: *const ecc_point, curve: *const ecc_curve);

    pub static mut ecdsa_x962_tmpl: crypto_template;
    pub static mut ecdsa_p1363_tmpl: crypto_template;
}

// Supplied by crypto/ecc_curve.h and the surrounding kernel interfaces.
#[repr(C)] pub struct ecc_curve { _private: [u8; 0] }
#[repr(C)] pub struct ecc_point { _private: [u8; 0] }
#[repr(C)] pub struct crypto_template { _private: [u8; 0] }

/// Copy ndigits from big endian array to native array.
pub unsafe fn ecc_swap_digits(input: *const core::ffi::c_void,
                              out: *mut u64, ndigits: u32) {
    let src = input as *const u8;
    let mut i: u32 = 0;
    while i < ndigits {
        *out.add(i as usize) = get_unaligned_be64(src.add(((ndigits - 1 - i) * 8) as usize));
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
