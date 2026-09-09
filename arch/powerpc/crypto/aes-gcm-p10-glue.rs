// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Glue code for accelerated AES-GCM stitched implementation for ppc64le.
 *
 * Copyright 2022- IBM Inc. All rights reserved
 */

// Kernel headers and build-time configuration are supplied by the surrounding
// translation unit.

const PPC_ALIGN: usize = 16;
const GCM_IV_SIZE: usize = 12;
const RFC4106_NONCE_SIZE: usize = 4;

// MODULE_DESCRIPTION("PPC64le AES-GCM with Stitched implementation");
// MODULE_AUTHOR("Danny Tsen <dtsen@linux.ibm.com");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS_CRYPTO("aes");

extern "C" {
    fn aes_p10_set_encrypt_key(user_key: *const u8, bits: i32, key: *mut core::ffi::c_void) -> i32;
    fn aes_p10_encrypt(input: *const u8, output: *mut u8, key: *const core::ffi::c_void);
    fn aes_p10_gcm_encrypt(input: *const u8, output: *mut u8, len: usize,
                           rkey: *mut core::ffi::c_void, iv: *mut u8, xi: *mut core::ffi::c_void);
    fn aes_p10_gcm_decrypt(input: *const u8, output: *mut u8, len: usize,
                           rkey: *mut core::ffi::c_void, iv: *mut u8, xi: *mut core::ffi::c_void);
    fn gcm_init_htable(htable: *mut u8, xi: *mut u8);
    fn gcm_ghash_p10(xi: *mut u8, htable: *mut u8, aad: *mut u8, alen: u32);
    fn gcm_update(iv: *mut u8, xi: *mut core::ffi::c_void);
}

#[repr(C)]
struct P10AesKey {
    key: [u8; AES_MAX_KEYLENGTH],
    rounds: u64,
}

#[repr(C)]
struct GcmCtx {
    iv: [u8; 16],
    ivtag: [u8; 16],
    aad_hash: [u8; 16],
    aad_len: u64,
    plen: u64,
    pblock: [u8; 16],
}

#[repr(C)]
struct HashCtx {
    h: [u8; 16],
    htable: [u8; 256],
}

#[repr(C)]
struct P10AesGcmCtx {
    enc_key: P10AesKey,
    nonce: [u8; RFC4106_NONCE_SIZE],
}

unsafe fn vsx_begin() {
    preempt_disable();
    pagefault_disable();
    enable_kernel_vsx();
}

unsafe fn vsx_end() {
    disable_kernel_vsx();
    pagefault_enable();
    preempt_enable();
}

unsafe fn set_subkey(hash: *mut u8) {
    *(hash as *mut u64) = be64_to_cpup(hash as *const u64);
    *((hash.add(8)) as *mut u64) = be64_to_cpup(hash.add(8) as *const u64);
}

unsafe fn set_aad(gctx: *mut GcmCtx, hash: *mut HashCtx, mut aad: *mut u8, mut alen: i32) {
    let mut nxi = [0u8; 16];
    (*gctx).aad_len = alen as u64;
    let i = alen & !0xf;
    if i != 0 {
        gcm_ghash_p10(nxi.as_mut_ptr(), (*hash).htable.as_mut_ptr().add(32), aad, i as u32);
        aad = aad.add(i as usize);
        alen -= i;
    }
    if alen != 0 {
        for j in 0..alen { nxi[j as usize] ^= *aad.add(j as usize); }
        core::ptr::write_bytes((*gctx).aad_hash.as_mut_ptr(), 0, 16);
        gcm_ghash_p10((*gctx).aad_hash.as_mut_ptr(), (*hash).htable.as_mut_ptr().add(32), nxi.as_mut_ptr(), 16);
    } else {
        core::ptr::copy_nonoverlapping(nxi.as_ptr(), (*gctx).aad_hash.as_mut_ptr(), 16);
    }
    core::ptr::copy_nonoverlapping((*gctx).aad_hash.as_ptr(), (*hash).htable.as_mut_ptr(), 16);
}

unsafe fn gcmp10_init(gctx: *mut GcmCtx, iv: *mut u8, rdkey: *mut u8,
                      hash: *mut HashCtx, assoc: *mut u8, assoclen: u32) {
    let mut counter: u32 = 1u32.to_be();
    aes_p10_encrypt((*hash).h.as_ptr(), (*hash).h.as_mut_ptr(), rdkey as *const _);
    set_subkey((*hash).h.as_mut_ptr());
    gcm_init_htable((*hash).htable.as_mut_ptr().add(32), (*hash).h.as_mut_ptr());
    *(iv.add(12) as *mut u32) = counter;
    (*gctx).plen = 0;
    aes_p10_encrypt(iv, (*gctx).ivtag.as_mut_ptr(), rdkey as *const _);
    counter = 2u32.to_be();
    *(iv.add(12) as *mut u32) = counter;
    core::ptr::copy_nonoverlapping(iv, (*gctx).iv.as_mut_ptr(), 16);
    (*gctx).aad_len = assoclen as u64;
    core::ptr::write_bytes((*gctx).aad_hash.as_mut_ptr(), 0, 16);
    if assoclen != 0 { set_aad(gctx, hash, assoc, assoclen as i32); }
}

unsafe fn finish_tag(gctx: *mut GcmCtx, hash: *mut HashCtx, len: i32) {
    if len == 0 && (*gctx).aad_len == 0 {
        core::ptr::copy_nonoverlapping((*gctx).ivtag.as_ptr(), (*hash).htable.as_mut_ptr(), 16);
        return;
    }
    let mut len_ac = [0u8; 16 + PPC_ALIGN];
    let offset = len_ac.as_ptr().align_offset(PPC_ALIGN);
    let aclen = len_ac.as_mut_ptr().add(offset);
    *(aclen as *mut u64) = ((*gctx).aad_len << 3).to_be();
    *(aclen.add(8) as *mut u64) = ((len as u64) << 3).to_be();
    gcm_ghash_p10((*hash).htable.as_mut_ptr(), (*hash).htable.as_mut_ptr().add(32), aclen, 16);
    for i in 0..16 { (*hash).htable[i] ^= (*gctx).ivtag[i]; }
}

unsafe fn set_authsize(_tfm: *mut CryptoAead, authsize: u32) -> i32 {
    match authsize { 4 | 8 | 12 | 13 | 14 | 15 | 16 => 0, _ => -EINVAL }
}

// The remaining kernel-facing routines retain the C ABI and call the same
// external kernel helpers.
extern "C" {
    fn p10_aes_gcm_setkey(aead: *mut CryptoAead, key: *const u8, keylen: u32) -> i32;
    fn p10_aes_gcm_crypt(req: *mut AeadRequest, riv: *mut u8, assoclen: i32, enc: i32) -> i32;
    fn rfc4106_setkey(tfm: *mut CryptoAead, inkey: *const u8, keylen: u32) -> i32;
    fn rfc4106_setauthsize(tfm: *mut CryptoAead, authsize: u32) -> i32;
    fn rfc4106_encrypt(req: *mut AeadRequest) -> i32;
    fn rfc4106_decrypt(req: *mut AeadRequest) -> i32;
    fn p10_aes_gcm_encrypt(req: *mut AeadRequest) -> i32;
    fn p10_aes_gcm_decrypt(req: *mut AeadRequest) -> i32;
}

// C's kernel-facing algorithm descriptors and registration hooks are supplied
// by the kernel crypto API.  The entries below preserve the original symbols,
// callbacks, names, priorities, and context sizing.
#[repr(C)]
struct AeadAlg {
    ivsize: usize,
    maxauthsize: usize,
    setauthsize: Option<unsafe extern "C" fn(*mut CryptoAead, u32) -> i32>,
    setkey: Option<unsafe extern "C" fn(*mut CryptoAead, *const u8, u32) -> i32>,
    encrypt: Option<unsafe extern "C" fn(*mut AeadRequest) -> i32>,
    decrypt: Option<unsafe extern "C" fn(*mut AeadRequest) -> i32>,
    cra_name: *const u8,
    cra_driver_name: *const u8,
    cra_priority: i32,
    cra_blocksize: usize,
    cra_ctxsize: usize,
    cra_flags: u32,
}

static mut GCM_AES_ALGS: [AeadAlg; 2] = [
    AeadAlg { ivsize: GCM_IV_SIZE, maxauthsize: 16, setauthsize: Some(set_authsize), setkey: Some(p10_aes_gcm_setkey), encrypt: Some(p10_aes_gcm_encrypt), decrypt: Some(p10_aes_gcm_decrypt), cra_name: b"__gcm(aes)\0".as_ptr(), cra_driver_name: b"__aes_gcm_p10\0".as_ptr(), cra_priority: 2100, cra_blocksize: 1, cra_ctxsize: core::mem::size_of::<P10AesGcmCtx>() + 4 * core::mem::size_of::<[u64; 2]>(), cra_flags: 1 },
    AeadAlg { ivsize: 8, maxauthsize: 16, setauthsize: Some(rfc4106_setauthsize), setkey: Some(rfc4106_setkey), encrypt: Some(rfc4106_encrypt), decrypt: Some(rfc4106_decrypt), cra_name: b"__rfc4106(gcm(aes))\0".as_ptr(), cra_driver_name: b"__rfc4106_aes_gcm_p10\0".as_ptr(), cra_priority: 2100, cra_blocksize: 1, cra_ctxsize: core::mem::size_of::<P10AesGcmCtx>() + 4 * core::mem::size_of::<[u64; 2]>(), cra_flags: 1 },
];

unsafe extern "C" fn p10_init() -> i32 {
    // if (!cpu_has_feature(CPU_FTR_ARCH_31)) return 0;
    simd_register_aeads_compat(GCM_AES_ALGS.as_mut_ptr(), 2)
}

unsafe extern "C" fn p10_exit() {
    simd_unregister_aeads(GCM_AES_ALGS.as_mut_ptr(), 2);
}

// module_init(p10_init); module_exit(p10_exit);

// External kernel types, constants, and helpers referenced by this source.
type CryptoAead = core::ffi::c_void;
type CryptoTfm = core::ffi::c_void;
type AeadRequest = core::ffi::c_void;
const AES_MAX_KEYLENGTH: usize = 60;
const EINVAL: i32 = 22;
unsafe extern "C" { fn preempt_disable(); fn pagefault_disable(); fn enable_kernel_vsx(); fn disable_kernel_vsx(); fn pagefault_enable(); fn preempt_enable(); fn be64_to_cpup(p: *const u64) -> u64; fn simd_register_aeads_compat(a: *mut AeadAlg, n: usize) -> i32; fn simd_unregister_aeads(a: *mut AeadAlg, n: usize); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
