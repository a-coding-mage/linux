/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Public Key Signature Algorithm
 *
 * Copyright (c) 2023 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependencies supplied by the surrounding crypto implementation:
// crypto/algapi.h and crypto/sig.h

#[repr(C)]
pub struct SigInstance {
    pub free: Option<unsafe extern "C" fn(inst: *mut SigInstance)>,
    pub variant: SigInstanceVariant,
}

#[repr(C)]
pub union SigInstanceVariant {
    pub common: SigInstanceCommon,
    pub alg: SigAlg,
}

#[repr(C)]
pub struct SigInstanceCommon {
    // C uses char head[offsetof(struct sig_alg, base)] as padding.
    pub head: [core::ffi::c_char; 0],
    pub base: CryptoInstance,
}

#[repr(C)]
pub struct CryptoSigSpawn {
    pub base: CryptoSpawn,
}

#[inline]
pub unsafe fn crypto_sig_ctx(tfm: *mut CryptoSig) -> *mut core::ffi::c_void {
    crypto_tfm_ctx(unsafe { &mut (*tfm).base })
}

/**
 * crypto_register_sig() -- Register public key signature algorithm
 *
 * Function registers an implementation of a public key signature algorithm
 *
 * @alg:\talgorithm definition
 *
 * Return: zero on success; error code in case of error
 */
pub unsafe extern "C" fn crypto_register_sig(alg: *mut SigAlg) -> core::ffi::c_int;

/**
 * crypto_unregister_sig() -- Unregister public key signature algorithm
 *
 * Function unregisters an implementation of a public key signature algorithm
 *
 * @alg:\talgorithm definition
 */
pub unsafe extern "C" fn crypto_unregister_sig(alg: *mut SigAlg);

pub unsafe extern "C" fn sig_register_instance(
    tmpl: *mut CryptoTemplate,
    inst: *mut SigInstance,
) -> core::ffi::c_int;

#[inline]
pub unsafe fn sig_instance(inst: *mut CryptoInstance) -> *mut SigInstance {
    container_of_sig_instance_alg(inst)
}

#[inline]
pub unsafe fn sig_alg_instance(tfm: *mut CryptoSig) -> *mut SigInstance {
    sig_instance(crypto_tfm_alg_instance(unsafe { &mut (*tfm).base }))
}

#[inline]
pub unsafe fn sig_crypto_instance(inst: *mut SigInstance) -> *mut CryptoInstance {
    container_of_sig_crypto_instance_alg(unsafe { &mut (*inst).variant.common.base })
}

#[inline]
pub unsafe fn sig_instance_ctx(inst: *mut SigInstance) -> *mut core::ffi::c_void {
    crypto_instance_ctx(sig_crypto_instance(inst))
}

pub unsafe extern "C" fn crypto_grab_sig(
    spawn: *mut CryptoSigSpawn,
    inst: *mut CryptoInstance,
    name: *const core::ffi::c_char,
    type_: u32,
    mask: u32,
) -> core::ffi::c_int;

#[inline]
pub unsafe fn crypto_spawn_sig(spawn: *mut CryptoSigSpawn) -> *mut CryptoSig {
    crypto_spawn_tfm2(unsafe { &mut (*spawn).base })
}

#[inline]
pub unsafe fn crypto_drop_sig(spawn: *mut CryptoSigSpawn) {
    crypto_drop_spawn(unsafe { &mut (*spawn).base });
}

#[inline]
pub unsafe fn crypto_spawn_sig_alg(spawn: *mut CryptoSigSpawn) -> *mut SigAlg {
    container_of_sig_alg(unsafe { (*spawn).base.alg })
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
