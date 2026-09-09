// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * crypto_sig wrapper around ML-DSA library.
 */

// Kernel headers and externally supplied ML-DSA symbols are dependencies of
// this translation and are intentionally not implemented here.

#[repr(C)]
pub struct CryptoMldsaCtx {
    pub pk: [u8; MAX_MLDSA_PUBLIC_KEY_SIZE],
    pub pk_len: u32,
    pub strength: MldsaAlg,
    pub key_set: bool,
}

// Corresponds to enum mldsa_alg from crypto/mldsa.h.
#[repr(C)]
#[derive(Copy, Clone)]
pub enum MldsaAlg {
    Mldsa44,
    Mldsa65,
    Mldsa87,
}

// MAX(MAX(MLDSA44_PUBLIC_KEY_SIZE, MLDSA65_PUBLIC_KEY_SIZE),
//     MLDSA87_PUBLIC_KEY_SIZE), supplied by crypto/mldsa.h.
pub const MAX_MLDSA_PUBLIC_KEY_SIZE: usize = max3(
    MLDSA44_PUBLIC_KEY_SIZE,
    MLDSA65_PUBLIC_KEY_SIZE,
    MLDSA87_PUBLIC_KEY_SIZE,
);

const fn max3(a: usize, b: usize, c: usize) -> usize {
    if a > b {
        if a > c { a } else { c }
    } else if b > c {
        b
    } else {
        c
    }
}

// External constants and kernel/API types and functions are supplied by the
// surrounding kernel translation unit.
extern "C" {
    static MLDSA44_PUBLIC_KEY_SIZE: usize;
    static MLDSA65_PUBLIC_KEY_SIZE: usize;
    static MLDSA87_PUBLIC_KEY_SIZE: usize;
    static MLDSA44_SIGNATURE_SIZE: usize;
    static MLDSA65_SIGNATURE_SIZE: usize;
    static MLDSA87_SIGNATURE_SIZE: usize;

    fn crypto_sig_ctx(tfm: *mut CryptoSig) -> *mut CryptoMldsaCtx;
    fn mldsa_verify(
        strength: MldsaAlg,
        sig: *const core::ffi::c_void,
        sig_len: u32,
        msg: *const core::ffi::c_void,
        msg_len: u32,
        pk: *const u8,
        pk_len: u32,
    ) -> i32;
    fn crypto_register_sig(alg: *mut SigAlg) -> i32;
    fn crypto_unregister_sig(alg: *mut SigAlg);
}

#[repr(C)]
pub struct CryptoSig {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SigAlg {
    _private: [u8; 0],
}

const EOPNOTSUPP: i32 = -95;
const EINVAL: i32 = -22;

pub unsafe fn crypto_mldsa_sign(
    _tfm: *mut CryptoSig,
    _msg: *const core::ffi::c_void,
    _msg_len: u32,
    _sig: *mut core::ffi::c_void,
    _sig_len: u32,
) -> i32 {
    -EOPNOTSUPP
}

pub unsafe fn crypto_mldsa_verify(
    tfm: *mut CryptoSig,
    sig: *const core::ffi::c_void,
    sig_len: u32,
    msg: *const core::ffi::c_void,
    msg_len: u32,
) -> i32 {
    let ctx = &*crypto_sig_ctx(tfm);
    if !ctx.key_set {
        return -EINVAL;
    }
    mldsa_verify(strength_value(ctx.strength), sig, sig_len, msg, msg_len,
                 ctx.pk.as_ptr(), ctx.pk_len)
}

fn strength_value(strength: MldsaAlg) -> MldsaAlg { strength }

pub unsafe fn crypto_mldsa_key_size(tfm: *mut CryptoSig) -> u32 {
    match (*crypto_sig_ctx(tfm)).strength {
        MldsaAlg::Mldsa44 => MLDSA44_PUBLIC_KEY_SIZE as u32,
        MldsaAlg::Mldsa65 => MLDSA65_PUBLIC_KEY_SIZE as u32,
        MldsaAlg::Mldsa87 => MLDSA87_PUBLIC_KEY_SIZE as u32,
    }
}

pub unsafe fn crypto_mldsa_set_pub_key(
    tfm: *mut CryptoSig,
    key: *const core::ffi::c_void,
    keylen: u32,
) -> i32 {
    let ctx = &mut *crypto_sig_ctx(tfm);
    let expected_len = crypto_mldsa_key_size(tfm);
    if keylen != expected_len { return -EINVAL; }
    ctx.pk_len = keylen;
    core::ptr::copy_nonoverlapping(key as *const u8, ctx.pk.as_mut_ptr(), keylen as usize);
    ctx.key_set = true;
    0
}

pub unsafe fn crypto_mldsa_set_priv_key(
    _tfm: *mut CryptoSig,
    _key: *const core::ffi::c_void,
    _keylen: u32,
) -> i32 { -EOPNOTSUPP }

pub unsafe fn crypto_mldsa_max_size(tfm: *mut CryptoSig) -> u32 {
    match (*crypto_sig_ctx(tfm)).strength {
        MldsaAlg::Mldsa44 => MLDSA44_SIGNATURE_SIZE as u32,
        MldsaAlg::Mldsa65 => MLDSA65_SIGNATURE_SIZE as u32,
        MldsaAlg::Mldsa87 => MLDSA87_SIGNATURE_SIZE as u32,
    }
}

pub unsafe fn crypto_mldsa44_alg_init(tfm: *mut CryptoSig) -> i32 {
    let ctx = &mut *crypto_sig_ctx(tfm); ctx.strength = MldsaAlg::Mldsa44; ctx.key_set = false; 0
}
pub unsafe fn crypto_mldsa65_alg_init(tfm: *mut CryptoSig) -> i32 {
    let ctx = &mut *crypto_sig_ctx(tfm); ctx.strength = MldsaAlg::Mldsa65; ctx.key_set = false; 0
}
pub unsafe fn crypto_mldsa87_alg_init(tfm: *mut CryptoSig) -> i32 {
    let ctx = &mut *crypto_sig_ctx(tfm); ctx.strength = MldsaAlg::Mldsa87; ctx.key_set = false; 0
}

pub unsafe fn crypto_mldsa_alg_exit(_tfm: *mut CryptoSig) {}

// The sig_alg array and module registration metadata are supplied by the
// kernel ABI; the three entries correspond to mldsa44, mldsa65, and mldsa87.
pub static mut CRYPTO_MLDSA_ALGS: [SigAlg; 3] = unsafe { core::mem::zeroed() };

pub unsafe fn mldsa_init() -> i32 {
    let mut ret: i32 = 0;
    let mut i: isize = 0;
    while (i as usize) < CRYPTO_MLDSA_ALGS.len() {
        ret = crypto_register_sig(&mut CRYPTO_MLDSA_ALGS[i as usize]);
        if ret < 0 {
            while i > 0 { i -= 1; crypto_unregister_sig(&mut CRYPTO_MLDSA_ALGS[i as usize]); }
            return ret;
        }
        i += 1;
    }
    ret
}

pub unsafe fn mldsa_exit() {
    let mut i = 0;
    while i < CRYPTO_MLDSA_ALGS.len() {
        crypto_unregister_sig(&mut CRYPTO_MLDSA_ALGS[i]);
        i += 1;
    }
}

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Crypto API support for ML-DSA signature verification");
// MODULE_ALIAS_CRYPTO("mldsa44");
// MODULE_ALIAS_CRYPTO("mldsa65");
// MODULE_ALIAS_CRYPTO("mldsa87");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
