// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Cryptographic API.
 *
 * Support for VIA PadLock hardware crypto engine.
 *
 * Copyright (c) 2006  Michal Ludvig <michal@logix.cz>
 */

// The following names are supplied by the surrounding kernel translation.

const PADLOCK_SHA_DESCSIZE: usize = 128 + ((PADLOCK_ALIGNMENT - 1) & !(CRYPTO_MINALIGN - 1));

#[repr(C)]
struct padlock_sha_ctx {
    fallback: *mut crypto_ahash,
}

#[inline]
unsafe fn padlock_shash_desc_ctx(desc: *mut shash_desc) -> *mut core::ffi::c_void {
    PTR_ALIGN(shash_desc_ctx(desc), PADLOCK_ALIGNMENT)
}

unsafe fn padlock_sha1_init(desc: *mut shash_desc) -> i32 {
    let sctx = padlock_shash_desc_ctx(desc) as *mut sha1_state;
    *sctx = sha1_state { state: [SHA1_H0, SHA1_H1, SHA1_H2, SHA1_H3, SHA1_H4], ..core::mem::zeroed() };
    0
}

unsafe fn padlock_sha256_init(desc: *mut shash_desc) -> i32 {
    let sctx = padlock_shash_desc_ctx(desc) as *mut crypto_sha256_state;
    sha256_block_init(sctx);
    0
}

unsafe fn padlock_sha_update(desc: *mut shash_desc, data: *const u8, length: u32) -> i32 {
    let state = padlock_shash_desc_ctx(desc) as *mut u8;
    let tfm = (*desc).tfm;
    let remain = length - round_down(length, crypto_shash_blocksize(tfm));
    let ctx = crypto_shash_ctx(tfm) as *mut padlock_sha_ctx;
    let mut req = HASH_REQUEST_ON_STACK((*ctx).fallback);
    ahash_request_set_callback(&mut req, 0, None, core::ptr::null_mut());
    ahash_request_set_virt(&mut req, data, core::ptr::null_mut(), length - remain);
    let mut err = crypto_ahash_import_core(&mut req, state);
    if err == 0 { err = crypto_ahash_update(&mut req); }
    if err == 0 { err = crypto_ahash_export_core(&mut req, state); }
    HASH_REQUEST_ZERO(&mut req);
    if err != 0 { err } else { remain as i32 }
}

unsafe fn padlock_sha_export(desc: *mut shash_desc, out: *mut core::ffi::c_void) -> i32 {
    memcpy(out, padlock_shash_desc_ctx(desc), crypto_shash_coresize((*desc).tfm));
    0
}

unsafe fn padlock_sha_import(desc: *mut shash_desc, input: *const core::ffi::c_void) -> i32 {
    let bs = crypto_shash_blocksize((*desc).tfm);
    let ss = crypto_shash_coresize((*desc).tfm);
    let state = padlock_shash_desc_ctx(desc) as *mut u64;
    memcpy(state as *mut core::ffi::c_void, input, ss);
    // Stop evil imports from generating a fault.
    *state.add(ss / 8 - 1) &= !(bs as u64 - 1);
    0
}

#[inline]
unsafe fn padlock_output_block(mut src: *mut u32, mut dst: *mut u32, mut count: usize) {
    while count != 0 {
        *dst = swab32(*src);
        src = src.add(1); dst = dst.add(1); count -= 1;
    }
}

unsafe fn padlock_sha_finup(desc: *mut shash_desc, input: *const u8, count: u32, out: *mut u8) -> i32 {
    let ctx = crypto_shash_ctx((*desc).tfm) as *mut padlock_sha_ctx;
    let mut req = HASH_REQUEST_ON_STACK((*ctx).fallback);
    ahash_request_set_callback(&mut req, 0, None, core::ptr::null_mut());
    ahash_request_set_virt(&mut req, input, out, count);
    let err = crypto_ahash_import_core(&mut req, padlock_shash_desc_ctx(desc));
    if err != 0 { err } else { crypto_ahash_finup(&mut req) }
}

unsafe fn padlock_sha1_finup(desc: *mut shash_desc, input: *const u8, count: u32, out: *mut u8) -> i32 {
    // We can't store directly to *out as it may be unaligned.
    // BTW Don't reduce the buffer size below 128 Bytes! PadLock microcode needs it that big.
    let state = padlock_shash_desc_ctx(desc) as *mut sha1_state;
    let start = (*state).count;
    if start + count as u64 > ULONG_MAX { return padlock_sha_finup(desc, input, count, out); }
    core::arch::asm!(".byte 0xf3,0x0f,0xa6,0xc8", in("ecx") (start + count as u64) as usize, in("eax") start as usize, in("esi") input, in("edi") state);
    padlock_output_block((*state).state.as_mut_ptr(), out as *mut u32, 5); 0
}

unsafe fn padlock_sha256_finup(desc: *mut shash_desc, input: *const u8, count: u32, out: *mut u8) -> i32 {
    // We can't store directly to *out as it may be unaligned.
    // BTW Don't reduce the buffer size below 128 Bytes! PadLock microcode needs it that big.
    let state = padlock_shash_desc_ctx(desc) as *mut crypto_sha256_state;
    let start = (*state).count;
    if start + count as u64 > ULONG_MAX { return padlock_sha_finup(desc, input, count, out); }
    core::arch::asm!(".byte 0xf3,0x0f,0xa6,0xd0", in("ecx") (start + count as u64) as usize, in("eax") start as usize, in("esi") input, in("edi") state);
    padlock_output_block((*state).state.as_mut_ptr(), out as *mut u32, 8); 0
}

unsafe fn padlock_init_tfm(hash: *mut crypto_shash) -> i32 {
    let fallback_driver_name = crypto_shash_alg_name(hash);
    let ctx = crypto_shash_ctx(hash) as *mut padlock_sha_ctx;
    let fallback_tfm = crypto_alloc_ahash(fallback_driver_name, 0, CRYPTO_ALG_NEED_FALLBACK | CRYPTO_ALG_ASYNC);
    if IS_ERR(fallback_tfm) { printk(KERN_WARNING, PFX, fallback_driver_name); return PTR_ERR(fallback_tfm); }
    if crypto_shash_statesize(hash) != crypto_ahash_statesize(fallback_tfm) { crypto_free_ahash(fallback_tfm); return -EINVAL; }
    (*ctx).fallback = fallback_tfm; 0
}

unsafe fn padlock_exit_tfm(hash: *mut crypto_shash) { let ctx = crypto_shash_ctx(hash) as *mut padlock_sha_ctx; crypto_free_ahash((*ctx).fallback); }

// Algorithm descriptors, CPU matching, module initialization, registration, and
// metadata are preserved below as declarations using the surrounding kernel ABI.
static mut sha1_alg: shash_alg = shash_alg { digestsize: SHA1_DIGEST_SIZE, init: Some(padlock_sha1_init), update: Some(padlock_sha_update), finup: Some(padlock_sha1_finup), export: Some(padlock_sha_export), import: Some(padlock_sha_import), init_tfm: Some(padlock_init_tfm), exit_tfm: Some(padlock_exit_tfm), descsize: PADLOCK_SHA_DESCSIZE, statesize: SHA1_STATE_SIZE, base: shash_alg_base { cra_name: b"sha1\0".as_ptr(), cra_driver_name: b"sha1-padlock\0".as_ptr(), cra_priority: PADLOCK_CRA_PRIORITY, cra_flags: CRYPTO_ALG_NEED_FALLBACK | CRYPTO_AHASH_ALG_BLOCK_ONLY | CRYPTO_AHASH_ALG_FINUP_MAX, cra_blocksize: SHA1_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<padlock_sha_ctx>(), cra_module: THIS_MODULE } };

// The remaining exported algorithm objects and module entry points retain the
// source interface; their field layouts and external registration functions are
// supplied by the kernel bindings.

unsafe fn padlock_sha1_update_nano(desc: *mut shash_desc, mut src: *const u8, mut len: u32) -> i32 {
    // The PHE requires the output buffer to be 128 bytes and 16-byte aligned.
    let state = padlock_shash_desc_ctx(desc) as *mut sha1_state;
    let blocks = len / SHA1_BLOCK_SIZE;
    len -= blocks * SHA1_BLOCK_SIZE;
    (*state).count += (blocks * SHA1_BLOCK_SIZE) as u64;
    core::arch::asm!(".byte 0xf3,0x0f,0xa6,0xc8", inout("esi") src, inout("edi") state, in("eax") -1i32, in("ecx") blocks);
    len as i32
}

unsafe fn padlock_sha256_update_nano(desc: *mut shash_desc, mut src: *const u8, mut len: u32) -> i32 {
    // The PHE requires the output buffer to be 128 bytes and 16-byte aligned.
    let state = padlock_shash_desc_ctx(desc) as *mut crypto_sha256_state;
    let blocks = len / SHA256_BLOCK_SIZE;
    len -= blocks * SHA256_BLOCK_SIZE;
    (*state).count += (blocks * SHA256_BLOCK_SIZE) as u64;
    core::arch::asm!(".byte 0xf3,0x0f,0xa6,0xd0", inout("esi") src, inout("edi") state, in("eax") -1i32, in("ecx") blocks);
    len as i32
}

static mut sha256_alg: shash_alg = shash_alg { digestsize: SHA256_DIGEST_SIZE, init: Some(padlock_sha256_init), update: Some(padlock_sha_update), finup: Some(padlock_sha256_finup), export: Some(padlock_sha_export), import: Some(padlock_sha_import), init_tfm: Some(padlock_init_tfm), exit_tfm: Some(padlock_exit_tfm), descsize: PADLOCK_SHA_DESCSIZE, statesize: core::mem::size_of::<crypto_sha256_state>(), base: shash_alg_base { cra_name: b"sha256\0".as_ptr(), cra_driver_name: b"sha256-padlock\0".as_ptr(), cra_priority: PADLOCK_CRA_PRIORITY, cra_flags: CRYPTO_ALG_NEED_FALLBACK | CRYPTO_AHASH_ALG_BLOCK_ONLY | CRYPTO_AHASH_ALG_FINUP_MAX, cra_blocksize: SHA256_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<padlock_sha_ctx>(), cra_module: THIS_MODULE } };

static mut sha1_alg_nano: shash_alg = shash_alg { digestsize: SHA1_DIGEST_SIZE, init: Some(padlock_sha1_init), update: Some(padlock_sha1_update_nano), finup: Some(padlock_sha1_finup), export: Some(padlock_sha_export), import: Some(padlock_sha_import), descsize: PADLOCK_SHA_DESCSIZE, statesize: SHA1_STATE_SIZE, base: shash_alg_base { cra_name: b"sha1\0".as_ptr(), cra_driver_name: b"sha1-padlock-nano\0".as_ptr(), cra_priority: PADLOCK_CRA_PRIORITY, cra_flags: CRYPTO_AHASH_ALG_BLOCK_ONLY | CRYPTO_AHASH_ALG_FINUP_MAX, cra_blocksize: SHA1_BLOCK_SIZE, cra_ctxsize: 0, cra_module: THIS_MODULE } };

static mut sha256_alg_nano: shash_alg = shash_alg { digestsize: SHA256_DIGEST_SIZE, init: Some(padlock_sha256_init), update: Some(padlock_sha256_update_nano), finup: Some(padlock_sha256_finup), export: Some(padlock_sha_export), import: Some(padlock_sha_import), descsize: PADLOCK_SHA_DESCSIZE, statesize: core::mem::size_of::<crypto_sha256_state>(), base: shash_alg_base { cra_name: b"sha256\0".as_ptr(), cra_driver_name: b"sha256-padlock-nano\0".as_ptr(), cra_priority: PADLOCK_CRA_PRIORITY, cra_flags: CRYPTO_AHASH_ALG_BLOCK_ONLY | CRYPTO_AHASH_ALG_FINUP_MAX, cra_blocksize: SHA256_BLOCK_SIZE, cra_ctxsize: 0, cra_module: THIS_MODULE } };

static padlock_sha_ids: [x86_cpu_id; 2] = [X86_MATCH_FEATURE(X86_FEATURE_PHE, core::ptr::null()), x86_cpu_id { feature: 0, model: 0, vendor: 0 }];

unsafe fn padlock_init() -> i32 {
    let c = &mut cpu_data(0);
    if !x86_match_cpu(padlock_sha_ids.as_ptr()) || !boot_cpu_has(X86_FEATURE_PHE_EN) { return -ENODEV; }
    if c.x86 >= 0x07 { return -ENODEV; }
    let (sha1, sha256) = if c.x86_model < 0x0f { (&mut sha1_alg, &mut sha256_alg) } else { (&mut sha1_alg_nano, &mut sha256_alg_nano) };
    let mut rc = crypto_register_shash(sha1);
    if rc != 0 { return rc; }
    rc = crypto_register_shash(sha256);
    if rc != 0 { crypto_unregister_shash(sha1); return rc; }
    printk(KERN_NOTICE, PFX);
    0
}

unsafe fn padlock_fini() {
    let c = &mut cpu_data(0);
    if c.x86_model >= 0x0f { crypto_unregister_shash(&mut sha1_alg_nano); crypto_unregister_shash(&mut sha256_alg_nano); }
    else { crypto_unregister_shash(&mut sha1_alg); crypto_unregister_shash(&mut sha256_alg); }
}

module_init!(padlock_init);
module_exit!(padlock_fini);
MODULE_DESCRIPTION!("VIA PadLock SHA1/SHA256 algorithms support.");
MODULE_LICENSE!("GPL");
MODULE_AUTHOR!("Michal Ludvig");
MODULE_ALIAS_CRYPTO!("sha1-all");
MODULE_ALIAS_CRYPTO!("sha256-all");
MODULE_ALIAS_CRYPTO!("sha1-padlock");
MODULE_ALIAS_CRYPTO!("sha256-padlock");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
