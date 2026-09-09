// SPDX-License-Identifier: GPL-2.0-only
/* Glue code for AES encryption optimized for sparc64 crypto opcodes.
 *
 * This is based largely upon arch/x86/crypto/aesni-intel_glue.c
 *
 * Copyright (C) 2008, Intel Corp.
 *    Author: Huang Ying <ying.huang@intel.com>
 *
 * Added RFC4106 AES-GCM support for 128-bit keys under the AEAD
 * interface for 64-bit kernels.
 *    Authors: Adrian Hoban <adrian.hoban@intel.com>
 *             Gabriele Paoloni <gabriele.paoloni@intel.com>
 *             Tadeusz Struk (tadeusz.struk@intel.com)
 *             Aidan O'Mahony (aidan.o.mahony@intel.com)
 *    Copyright (c) 2010, Intel Corporation.
 */

// C includes provide the kernel types, constants, helpers, and external symbols used below.

#[repr(C)]
pub struct aes_ops {
    pub load_encrypt_keys: unsafe extern "C" fn(*const u64),
    pub load_decrypt_keys: unsafe extern "C" fn(*const u64),
    pub ecb_encrypt: unsafe extern "C" fn(*const u64, *const u64, *mut u64, u32),
    pub ecb_decrypt: unsafe extern "C" fn(*const u64, *const u64, *mut u64, u32),
    pub cbc_encrypt: unsafe extern "C" fn(*const u64, *const u64, *mut u64, u32, *mut u64),
    pub cbc_decrypt: unsafe extern "C" fn(*const u64, *const u64, *mut u64, u32, *mut u64),
    pub ctr_crypt: unsafe extern "C" fn(*const u64, *const u64, *mut u64, u32, *mut u64),
}

#[repr(C)]
pub struct crypto_sparc64_aes_ctx {
    pub ops: *mut aes_ops,
    pub key: [u64; AES_MAX_KEYLENGTH / core::mem::size_of::<u64>()],
    pub key_length: u32,
    pub expanded_key_length: u32,
}

static mut aes128_ops: aes_ops = aes_ops {
    load_encrypt_keys: aes_sparc64_load_encrypt_keys_128,
    load_decrypt_keys: aes_sparc64_load_decrypt_keys_128,
    ecb_encrypt: aes_sparc64_ecb_encrypt_128,
    ecb_decrypt: aes_sparc64_ecb_decrypt_128,
    cbc_encrypt: aes_sparc64_cbc_encrypt_128,
    cbc_decrypt: aes_sparc64_cbc_decrypt_128,
    ctr_crypt: aes_sparc64_ctr_crypt_128,
};

static mut aes192_ops: aes_ops = aes_ops {
    load_encrypt_keys: aes_sparc64_load_encrypt_keys_192,
    load_decrypt_keys: aes_sparc64_load_decrypt_keys_192,
    ecb_encrypt: aes_sparc64_ecb_encrypt_192,
    ecb_decrypt: aes_sparc64_ecb_decrypt_192,
    cbc_encrypt: aes_sparc64_cbc_encrypt_192,
    cbc_decrypt: aes_sparc64_cbc_decrypt_192,
    ctr_crypt: aes_sparc64_ctr_crypt_192,
};

static mut aes256_ops: aes_ops = aes_ops {
    load_encrypt_keys: aes_sparc64_load_encrypt_keys_256,
    load_decrypt_keys: aes_sparc64_load_decrypt_keys_256,
    ecb_encrypt: aes_sparc64_ecb_encrypt_256,
    ecb_decrypt: aes_sparc64_ecb_decrypt_256,
    cbc_encrypt: aes_sparc64_cbc_encrypt_256,
    cbc_decrypt: aes_sparc64_cbc_decrypt_256,
    ctr_crypt: aes_sparc64_ctr_crypt_256,
};

unsafe fn aes_set_key_skcipher(tfm: *mut crypto_skcipher, in_key: *const u8, key_len: u32) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm);
    match key_len {
        AES_KEYSIZE_128 => { (*ctx).expanded_key_length = 0xb0; (*ctx).ops = &raw mut aes128_ops; }
        AES_KEYSIZE_192 => { (*ctx).expanded_key_length = 0xd0; (*ctx).ops = &raw mut aes192_ops; }
        AES_KEYSIZE_256 => { (*ctx).expanded_key_length = 0xf0; (*ctx).ops = &raw mut aes256_ops; }
        _ => return -EINVAL,
    }
    aes_sparc64_key_expand(in_key as *const u32, (*ctx).key.as_mut_ptr(), key_len);
    (*ctx).key_length = key_len;
    0
}

unsafe fn ecb_encrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit();
    let mut nbytes: u32;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, true);
    if err != 0 { return err; }
    let walk = walk.assume_init_mut();
    ((*ctx).ops.as_ref().unwrap().load_encrypt_keys)((*ctx).key.as_ptr());
    while { nbytes = walk.nbytes; nbytes != 0 } {
        ((*ctx).ops.as_ref().unwrap().ecb_encrypt)((*ctx).key.as_ptr(), walk.src.virt.addr, walk.dst.virt.addr, nbytes & !(AES_BLOCK_SIZE - 1));
        err = skcipher_walk_done(walk, nbytes % AES_BLOCK_SIZE);
    }
    fprs_write(0);
    err
}

unsafe fn ecb_decrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm);
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit();
    let mut nbytes: u32;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, true);
    if err != 0 { return err; }
    let walk = walk.assume_init_mut();
    ((*ctx).ops.as_ref().unwrap().load_decrypt_keys)((*ctx).key.as_ptr());
    let key_end = (*ctx).key.as_ptr().add((*ctx).expanded_key_length as usize / core::mem::size_of::<u64>());
    while { nbytes = walk.nbytes; nbytes != 0 } {
        ((*ctx).ops.as_ref().unwrap().ecb_decrypt)(key_end, walk.src.virt.addr, walk.dst.virt.addr, nbytes & !(AES_BLOCK_SIZE - 1));
        err = skcipher_walk_done(walk, nbytes % AES_BLOCK_SIZE);
    }
    fprs_write(0);
    err
}

unsafe fn cbc_encrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(tfm);
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit(); let mut nbytes: u32;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, true); if err != 0 { return err; }
    let walk = walk.assume_init_mut(); ((*ctx).ops.as_ref().unwrap().load_encrypt_keys)((*ctx).key.as_ptr());
    while { nbytes = walk.nbytes; nbytes != 0 } { ((*ctx).ops.as_ref().unwrap().cbc_encrypt)((*ctx).key.as_ptr(), walk.src.virt.addr, walk.dst.virt.addr, nbytes & !(AES_BLOCK_SIZE - 1), walk.iv); err = skcipher_walk_done(walk, nbytes % AES_BLOCK_SIZE); }
    fprs_write(0); err
}

unsafe fn cbc_decrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(tfm);
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit(); let mut nbytes: u32;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, true); if err != 0 { return err; }
    let walk = walk.assume_init_mut(); ((*ctx).ops.as_ref().unwrap().load_decrypt_keys)((*ctx).key.as_ptr());
    let key_end = (*ctx).key.as_ptr().add((*ctx).expanded_key_length as usize / core::mem::size_of::<u64>());
    while { nbytes = walk.nbytes; nbytes != 0 } { ((*ctx).ops.as_ref().unwrap().cbc_decrypt)(key_end, walk.src.virt.addr, walk.dst.virt.addr, nbytes & !(AES_BLOCK_SIZE - 1), walk.iv); err = skcipher_walk_done(walk, nbytes % AES_BLOCK_SIZE); }
    fprs_write(0); err
}

unsafe fn ctr_crypt_final(ctx: *const crypto_sparc64_aes_ctx, walk: *mut skcipher_walk) {
    let ctrblk = (*walk).iv; let mut keystream = [0u64; AES_BLOCK_SIZE / core::mem::size_of::<u64>()];
    let src = (*walk).src.virt.addr; let dst = (*walk).dst.virt.addr; let nbytes = (*walk).nbytes;
    ((*ctx).ops.as_ref().unwrap().ecb_encrypt)((*ctx).key.as_ptr(), ctrblk as *const u64, keystream.as_mut_ptr(), AES_BLOCK_SIZE);
    crypto_xor_cpy(dst, keystream.as_mut_ptr() as *mut u8, src, nbytes); crypto_inc(ctrblk, AES_BLOCK_SIZE);
}

unsafe fn ctr_crypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(tfm);
    let mut walk = core::mem::MaybeUninit::<skcipher_walk>::uninit(); let mut nbytes: u32;
    let mut err = skcipher_walk_virt(walk.as_mut_ptr(), req, true); if err != 0 { return err; }
    let walk = walk.assume_init_mut(); ((*ctx).ops.as_ref().unwrap().load_encrypt_keys)((*ctx).key.as_ptr());
    while { nbytes = walk.nbytes; nbytes >= AES_BLOCK_SIZE } { ((*ctx).ops.as_ref().unwrap().ctr_crypt)((*ctx).key.as_ptr(), walk.src.virt.addr, walk.dst.virt.addr, nbytes & !(AES_BLOCK_SIZE - 1), walk.iv); err = skcipher_walk_done(walk, nbytes % AES_BLOCK_SIZE); }
    if walk.nbytes != 0 { ctr_crypt_final(ctx, walk); err = skcipher_walk_done(walk, 0); }
    fprs_write(0); err
}

static mut skcipher_algs: [skcipher_alg; 3] = [
    skcipher_alg {
        base: crypto_alg { cra_name: "ecb(aes)", cra_driver_name: "ecb-aes-sparc64", cra_priority: SPARC_CR_OPCODE_PRIORITY, cra_blocksize: AES_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<crypto_sparc64_aes_ctx>(), cra_alignmask: 7, cra_module: THIS_MODULE },
        min_keysize: AES_MIN_KEY_SIZE, max_keysize: AES_MAX_KEY_SIZE, ivsize: 0,
        setkey: aes_set_key_skcipher, encrypt: ecb_encrypt, decrypt: ecb_decrypt, chunksize: 0,
    },
    skcipher_alg {
        base: crypto_alg { cra_name: "cbc(aes)", cra_driver_name: "cbc-aes-sparc64", cra_priority: SPARC_CR_OPCODE_PRIORITY, cra_blocksize: AES_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<crypto_sparc64_aes_ctx>(), cra_alignmask: 7, cra_module: THIS_MODULE },
        min_keysize: AES_MIN_KEY_SIZE, max_keysize: AES_MAX_KEY_SIZE, ivsize: AES_BLOCK_SIZE,
        setkey: aes_set_key_skcipher, encrypt: cbc_encrypt, decrypt: cbc_decrypt, chunksize: 0,
    },
    skcipher_alg {
        base: crypto_alg { cra_name: "ctr(aes)", cra_driver_name: "ctr-aes-sparc64", cra_priority: SPARC_CR_OPCODE_PRIORITY, cra_blocksize: 1, cra_ctxsize: core::mem::size_of::<crypto_sparc64_aes_ctx>(), cra_alignmask: 7, cra_module: THIS_MODULE },
        min_keysize: AES_MIN_KEY_SIZE, max_keysize: AES_MAX_KEY_SIZE, ivsize: AES_BLOCK_SIZE,
        setkey: aes_set_key_skcipher, encrypt: ctr_crypt, decrypt: ctr_crypt, chunksize: AES_BLOCK_SIZE,
    },
];

unsafe fn sparc64_has_aes_opcode() -> bool {
    let mut cfr: usize;
    if sparc64_elf_hwcap & HWCAP_SPARC_CRYPTO == 0 { return false; }
    core::arch::asm!("rd %asr26, {0}", out(reg) cfr);
    if cfr & CFR_AES == 0 { return false; }
    true
}

unsafe fn aes_sparc64_mod_init() -> i32 {
    if !sparc64_has_aes_opcode() { pr_info!("sparc64 aes opcodes not available.\n"); return -ENODEV; }
    pr_info!("Using sparc64 aes opcodes optimized AES implementation\n");
    crypto_register_skciphers(skcipher_algs.as_mut_ptr(), ARRAY_SIZE(skcipher_algs))
}

unsafe fn aes_sparc64_mod_fini() { crypto_unregister_skciphers(skcipher_algs.as_mut_ptr(), ARRAY_SIZE(skcipher_algs)); }

// module_init(aes_sparc64_mod_init);
// module_exit(aes_sparc64_mod_fini);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Rijndael (AES) Cipher Algorithm, sparc64 aes opcode accelerated");
// MODULE_ALIAS_CRYPTO("aes");
// #include "crop_devid.c"

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
