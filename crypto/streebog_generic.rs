// SPDX-License-Identifier: GPL-2.0+ OR BSD-2-Clause
// Faithful low-level Rust translation of streebog_generic.c.
// C header dependencies and generated lookup data are supplied by the
// surrounding kernel translation unit.

use core::ptr;

#[repr(C)]
pub struct streebog_uint512 { pub qword: [u64; 8] }
#[repr(C)] pub struct streebog_state { pub h: streebog_uint512, pub N: streebog_uint512, pub Sigma: streebog_uint512, pub hash: streebog_uint512 }
#[repr(C)] pub struct shash_desc { pub tfm: *mut core::ffi::c_void }

extern "C" {
    static C: [streebog_uint512; 12];
    static Ax: [[u64; 256]; 8];
    fn shash_desc_ctx(desc: *mut shash_desc) -> *mut streebog_state;
    fn crypto_shash_digestsize(tfm: *mut core::ffi::c_void) -> u32;
    fn memzero_explicit(p: *mut core::ffi::c_void, n: usize);
}

const STREEBOG_BLOCK_SIZE: usize = 64;
const STREEBOG256_DIGEST_SIZE: u32 = 32;
const STREEBOG512_DIGEST_SIZE: u32 = 64;
static BUFFER0: streebog_uint512 = streebog_uint512 { qword: [0; 8] };
static BUFFER512: streebog_uint512 = streebog_uint512 { qword: [0x200, 0, 0, 0, 0, 0, 0, 0] };

#[inline] unsafe fn streebog_xor(x: *const streebog_uint512, y: *const streebog_uint512, z: *mut streebog_uint512) {
    for i in 0..8 { (*z).qword[i] = (*x).qword[i] ^ (*y).qword[i]; }
}
#[inline] unsafe fn streebog_xlps(x: *const streebog_uint512, y: *const streebog_uint512, data: *mut streebog_uint512) {
    let mut r = [0u64; 8];
    for i in 0..8 { r[i] = (*x).qword[i] ^ (*y).qword[i]; }
    for i in 0..8 {
        (*data).qword[i] = 0;
        for j in 0..8 { (*data).qword[i] ^= Ax[j][((r[j] >> (i * 8)) & 0xff) as usize]; }
    }
}
unsafe fn streebog_round(i: usize, ki: *mut streebog_uint512, data: *mut streebog_uint512) {
    let mut t = streebog_uint512 { qword: [0; 8] };
    streebog_xlps(ki, &C[i], &mut t); *ki = t;
    streebog_xlps(ki, data, &mut t); *data = t;
}
unsafe fn streebog_add512(x: *const streebog_uint512, y: *const streebog_uint512, r: *mut streebog_uint512) {
    let mut carry = 0u64;
    for i in 0..8 { let left = (*x).qword[i]; let sum = left.wrapping_add((*y).qword[i]).wrapping_add(carry); carry = (sum < left) as u64; (*r).qword[i] = sum; }
}
unsafe fn streebog_g(h: *mut streebog_uint512, n: *const streebog_uint512, m: *const streebog_uint512) {
    let mut ki = streebog_uint512 { qword: [0; 8] }; let mut data = streebog_uint512 { qword: [0; 8] };
    streebog_xlps(h, n, &mut data); ki = data; streebog_xlps(&ki, m, &mut data);
    for i in 0..11 { streebog_round(i, &mut ki, &mut data); }
    streebog_xlps(&ki, &C[11], &mut ki); streebog_xor(&ki, &data, &mut data); streebog_xor(&data, h, &mut data); streebog_xor(&data, m, h);
}
unsafe fn streebog_stage2(ctx: *mut streebog_state, data: *const u8) { let mut m = streebog_uint512 { qword: [0; 8] }; ptr::copy_nonoverlapping(data, &mut m as *mut _ as *mut u8, 64); streebog_g(&mut (*ctx).h, &(*ctx).N, &m); streebog_add512(&(*ctx).N, &BUFFER512, &mut (*ctx).N); streebog_add512(&(*ctx).Sigma, &m, &mut (*ctx).Sigma); }
unsafe fn streebog_stage3(ctx: *mut streebog_state, src: *const u8, len: usize) { let mut m = streebog_uint512 { qword: [0; 8] }; ptr::copy_bytes(&mut m as *mut _ as *mut u8, src, len); *( &mut m.qword[0]) = (len as u64 * 8).to_le(); let p = (&mut m as *mut _ as *mut u8).add(len); *p = 1; streebog_g(&mut (*ctx).h, &(*ctx).N, &m); streebog_add512(&(*ctx).N, &streebog_uint512 { qword: [(len as u64 * 8).to_le(),0,0,0,0,0,0,0] }, &mut (*ctx).N); streebog_add512(&(*ctx).Sigma, &m, &mut (*ctx).Sigma); streebog_g(&mut (*ctx).h, &BUFFER0, &(*ctx).N); streebog_g(&mut (*ctx).h, &BUFFER0, &(*ctx).Sigma); (*ctx).hash = (*ctx).h; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
