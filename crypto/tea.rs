// SPDX-License-Identifier: GPL-2.0-or-later
/* Cryptographic API: TEA, XTEA, and XETA crypto algorithms. */

// Kernel dependencies supplied by the surrounding repository.
use core::ffi::c_void;

const TEA_KEY_SIZE: usize = 16;
const TEA_BLOCK_SIZE: usize = 8;
const TEA_ROUNDS: u32 = 32;
const TEA_DELTA: u32 = 0x9e3779b9;
const XTEA_KEY_SIZE: usize = 16;
const XTEA_BLOCK_SIZE: usize = 8;
const XTEA_ROUNDS: u32 = 32;
const XTEA_DELTA: u32 = 0x9e3779b9;

#[repr(C)]
pub struct tea_ctx { pub KEY: [u32; 4] }
#[repr(C)]
pub struct xtea_ctx { pub KEY: [u32; 4] }

#[repr(C)] pub struct crypto_tfm { _private: [u8; 0] }
extern "C" {
    fn crypto_tfm_ctx(tfm: *mut crypto_tfm) -> *mut c_void;
}

unsafe fn get_le32(p: *const u8) -> u32 {
    u32::from_le_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
}
unsafe fn put_le32(v: u32, p: *mut u8) {
    let b = v.to_le_bytes();
    for i in 0..4 { *p.add(i) = b[i]; }
}

unsafe fn tea_setkey(tfm: *mut crypto_tfm, in_key: *const u8, _key_len: u32) -> i32 {
    let ctx = crypto_tfm_ctx(tfm) as *mut tea_ctx;
    for i in 0..4 { (*ctx).KEY[i] = get_le32(in_key.add(i * 4)); }
    0
}

unsafe fn tea_encrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    let mut y = get_le32(src); let mut z = get_le32(src.add(4));
    let ctx = crypto_tfm_ctx(tfm) as *mut tea_ctx;
    let [k0, k1, k2, k3] = (*ctx).KEY; let mut sum = 0u32;
    let mut n = TEA_ROUNDS;
    while n > 0 {
        sum = sum.wrapping_add(TEA_DELTA);
        y = y.wrapping_add(((z << 4).wrapping_add(k0)) ^ z.wrapping_add(sum) ^ ((z >> 5).wrapping_add(k1)));
        z = z.wrapping_add(((y << 4).wrapping_add(k2)) ^ y.wrapping_add(sum) ^ ((y >> 5).wrapping_add(k3)));
        n -= 1;
    }
    put_le32(y, dst); put_le32(z, dst.add(4));
}

unsafe fn tea_decrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    let mut y = get_le32(src); let mut z = get_le32(src.add(4));
    let ctx = crypto_tfm_ctx(tfm) as *mut tea_ctx;
    let [k0, k1, k2, k3] = (*ctx).KEY; let mut sum = TEA_DELTA << 5; let mut n = TEA_ROUNDS;
    while n > 0 {
        z = z.wrapping_sub(((y << 4).wrapping_add(k2)) ^ y.wrapping_add(sum) ^ ((y >> 5).wrapping_add(k3)));
        y = y.wrapping_sub(((z << 4).wrapping_add(k0)) ^ z.wrapping_add(sum) ^ ((z >> 5).wrapping_add(k1)));
        sum = sum.wrapping_sub(TEA_DELTA); n -= 1;
    }
    put_le32(y, dst); put_le32(z, dst.add(4));
}

unsafe fn xtea_setkey(tfm: *mut crypto_tfm, in_key: *const u8, _key_len: u32) -> i32 {
    let ctx = crypto_tfm_ctx(tfm) as *mut xtea_ctx;
    for i in 0..4 { (*ctx).KEY[i] = get_le32(in_key.add(i * 4)); } 0
}

unsafe fn xtea_encrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    let mut y = get_le32(src); let mut z = get_le32(src.add(4)); let mut sum = 0u32;
    let limit = XTEA_DELTA.wrapping_mul(XTEA_ROUNDS); let ctx = crypto_tfm_ctx(tfm) as *mut xtea_ctx;
    while sum != limit {
        y = y.wrapping_add((((z << 4) ^ (z >> 5)).wrapping_add(z)) ^ sum.wrapping_add((*ctx).KEY[(sum & 3) as usize]));
        sum = sum.wrapping_add(XTEA_DELTA);
        z = z.wrapping_add((((y << 4) ^ (y >> 5)).wrapping_add(y)) ^ sum.wrapping_add((*ctx).KEY[((sum >> 11) & 3) as usize]));
    }
    put_le32(y, dst); put_le32(z, dst.add(4));
}

unsafe fn xtea_decrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    let mut y = get_le32(src); let mut z = get_le32(src.add(4)); let mut sum = XTEA_DELTA.wrapping_mul(XTEA_ROUNDS);
    let ctx = crypto_tfm_ctx(tfm) as *mut xtea_ctx;
    while sum != 0 {
        z = z.wrapping_sub((((y << 4) ^ (y >> 5)).wrapping_add(y)) ^ sum.wrapping_add((*ctx).KEY[((sum >> 11) & 3) as usize]));
        sum = sum.wrapping_sub(XTEA_DELTA);
        y = y.wrapping_sub((((z << 4) ^ (z >> 5)).wrapping_add(z)) ^ sum.wrapping_add((*ctx).KEY[(sum & 3) as usize]));
    }
    put_le32(y, dst); put_le32(z, dst.add(4));
}

unsafe fn xeta_encrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    let mut y = get_le32(src); let mut z = get_le32(src.add(4)); let mut sum = 0u32;
    let limit = XTEA_DELTA.wrapping_mul(XTEA_ROUNDS); let ctx = crypto_tfm_ctx(tfm) as *mut xtea_ctx;
    while sum != limit {
        y = y.wrapping_add(((z << 4) ^ (z >> 5)).wrapping_add(z ^ sum).wrapping_add((*ctx).KEY[(sum & 3) as usize]));
        sum = sum.wrapping_add(XTEA_DELTA);
        z = z.wrapping_add(((y << 4) ^ (y >> 5)).wrapping_add(y ^ sum).wrapping_add((*ctx).KEY[((sum >> 11) & 3) as usize]));
    }
    put_le32(y, dst); put_le32(z, dst.add(4));
}

unsafe fn xeta_decrypt(tfm: *mut crypto_tfm, dst: *mut u8, src: *const u8) {
    let mut y = get_le32(src); let mut z = get_le32(src.add(4)); let mut sum = XTEA_DELTA.wrapping_mul(XTEA_ROUNDS);
    let ctx = crypto_tfm_ctx(tfm) as *mut xtea_ctx;
    while sum != 0 {
        z = z.wrapping_sub(((y << 4) ^ (y >> 5)).wrapping_add(y ^ sum).wrapping_add((*ctx).KEY[((sum >> 11) & 3) as usize]));
        sum = sum.wrapping_sub(XTEA_DELTA);
        y = y.wrapping_sub(((z << 4) ^ (z >> 5)).wrapping_add(z ^ sum).wrapping_add((*ctx).KEY[(sum & 3) as usize]));
    }
    put_le32(y, dst); put_le32(z, dst.add(4));
}

// The crypto_alg registration table and module init/exit hooks are supplied by
// the kernel crypto framework; the C source registers tea, xtea, and xeta here.
#[repr(C)] pub struct crypto_alg { _private: [u8; 0] }
extern "C" {
    fn crypto_register_algs(algs: *mut crypto_alg, count: usize) -> i32;
    fn crypto_unregister_algs(algs: *mut crypto_alg, count: usize);
}
// Equivalent metadata for the three C crypto_alg entries is consumed by the
// surrounding kernel integration; callbacks above retain their C interfaces.
static mut tea_algs: [crypto_alg; 3] = [crypto_alg { _private: [] }; 3];

unsafe fn tea_mod_init() -> i32 {
    crypto_register_algs(tea_algs.as_mut_ptr(), tea_algs.len())
}
unsafe fn tea_mod_fini() {
    crypto_unregister_algs(tea_algs.as_mut_ptr(), tea_algs.len());
}

// MODULE_ALIAS_CRYPTO("tea"), MODULE_ALIAS_CRYPTO("xtea"),
// MODULE_ALIAS_CRYPTO("xeta"), MODULE_LICENSE("GPL"),
// MODULE_DESCRIPTION("TEA, XTEA & XETA Cryptographic Algorithms"),
// module_init(tea_mod_init), and module_exit(tea_mod_fini) are kernel metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
