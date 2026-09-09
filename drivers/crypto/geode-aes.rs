// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2004-2006, Advanced Micro Devices, Inc.
//
// Linux kernel dependencies and the declarations from geode-aes.h are supplied
// by the surrounding translation unit.

use core::ffi::c_void;

static mut _iobase: *mut c_void = core::ptr::null_mut();
static mut lock: spinlock_t = spinlock_t::new();

unsafe fn _writefield(offset: u32, value: *const c_void) {
    for i in 0..4 {
        iowrite32(
            *((value as *const u32).add(i)),
            (_iobase as *mut u8).add(offset as usize + i * 4) as *mut c_void,
        );
    }
}

unsafe fn _readfield(offset: u32, value: *mut c_void) {
    for i in 0..4 {
        *((value as *mut u32).add(i)) =
            ioread32((_iobase as *mut u8).add(offset as usize + i * 4) as *mut c_void);
    }
}

unsafe fn do_crypt(src: *const c_void, dst: *mut c_void, len: u32, flags: u32) -> i32 {
    let mut status: u32;
    let mut counter: u32 = AES_OP_TIMEOUT;

    iowrite32(virt_to_phys(src as *mut c_void), (_iobase as *mut u8).add(AES_SOURCEA_REG as usize) as *mut c_void);
    iowrite32(virt_to_phys(dst), (_iobase as *mut u8).add(AES_DSTA_REG as usize) as *mut c_void);
    iowrite32(len, (_iobase as *mut u8).add(AES_LENA_REG as usize) as *mut c_void);

    iowrite32(AES_CTRL_START | flags, (_iobase as *mut u8).add(AES_CTRLA_REG as usize) as *mut c_void);
    loop {
        status = ioread32((_iobase as *mut u8).add(AES_INTR_REG as usize) as *mut c_void);
        cpu_relax();
        counter = counter.wrapping_sub(1);
        if (status & AES_INTRA_PENDING) != 0 || counter == 0 { break; }
    }
    iowrite32((status & 0xff) | AES_INTRA_PENDING, (_iobase as *mut u8).add(AES_INTR_REG as usize) as *mut c_void);
    if counter != 0 { 0 } else { 1 }
}

unsafe fn geode_aes_crypt(tctx: *const geode_aes_tfm_ctx, src: *const c_void, dst: *mut c_void,
                           len: u32, iv: *mut u8, mode: i32, dir: i32) {
    let mut flags = AES_CTRL_DCA | AES_CTRL_SCA;
    let mut iflags: c_ulong = 0;
    if dir == AES_DIR_ENCRYPT { flags |= AES_CTRL_ENCRYPT; }
    spin_lock_irqsave(&raw mut lock, &mut iflags);
    if mode == AES_MODE_CBC { flags |= AES_CTRL_CBC; _writefield(AES_WRITEIV0_REG, iv as *const c_void); }
    flags |= AES_CTRL_WRKEY;
    _writefield(AES_WRITEKEY0_REG, (*tctx).key.as_ptr() as *const c_void);
    let ret = do_crypt(src, dst, len, flags);
    BUG_ON(ret != 0);
    if mode == AES_MODE_CBC { _readfield(AES_WRITEIV0_REG, iv as *mut c_void); }
    spin_unlock_irqrestore(&raw mut lock, iflags);
}

unsafe fn geode_setkey_cip(tfm: *mut crypto_tfm, key: *const u8, len: c_uint) -> i32 {
    let tctx = crypto_tfm_ctx(tfm);
    (*tctx).keylen = len;
    if len == AES_KEYSIZE_128 { memcpy((*tctx).key.as_mut_ptr() as *mut c_void, key as *const c_void, len as usize); return 0; }
    if len != AES_KEYSIZE_192 && len != AES_KEYSIZE_256 { return -EINVAL; }
    (*tctx).fallback.cip.as_mut().unwrap().base.crt_flags &= !CRYPTO_TFM_REQ_MASK;
    (*tctx).fallback.cip.as_mut().unwrap().base.crt_flags |= (*tfm).crt_flags & CRYPTO_TFM_REQ_MASK;
    crypto_cipher_setkey((*tctx).fallback.cip, key, len)
}

unsafe fn geode_setkey_skcipher(tfm: *mut crypto_skcipher, key: *const u8, len: c_uint) -> i32 {
    let tctx = crypto_skcipher_ctx(tfm);
    (*tctx).keylen = len;
    if len == AES_KEYSIZE_128 { memcpy((*tctx).key.as_mut_ptr() as *mut c_void, key as *const c_void, len as usize); return 0; }
    if len != AES_KEYSIZE_192 && len != AES_KEYSIZE_256 { return -EINVAL; }
    crypto_skcipher_clear_flags((*tctx).fallback.skcipher, CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_set_flags((*tctx).fallback.skcipher, crypto_skcipher_get_flags(tfm) & CRYPTO_TFM_REQ_MASK);
    crypto_skcipher_setkey((*tctx).fallback.skcipher, key, len)
}

unsafe fn geode_encrypt(tfm: *mut crypto_tfm, out: *mut u8, input: *const u8) {
    let tctx = crypto_tfm_ctx(tfm);
    if (*tctx).keylen != AES_KEYSIZE_128 { crypto_cipher_encrypt_one((*tctx).fallback.cip, out, input); return; }
    geode_aes_crypt(tctx, input as *const c_void, out as *mut c_void, AES_BLOCK_SIZE, core::ptr::null_mut(), AES_MODE_ECB, AES_DIR_ENCRYPT);
}

unsafe fn geode_decrypt(tfm: *mut crypto_tfm, out: *mut u8, input: *const u8) {
    let tctx = crypto_tfm_ctx(tfm);
    if (*tctx).keylen != AES_KEYSIZE_128 { crypto_cipher_decrypt_one((*tctx).fallback.cip, out, input); return; }
    geode_aes_crypt(tctx, input as *const c_void, out as *mut c_void, AES_BLOCK_SIZE, core::ptr::null_mut(), AES_MODE_ECB, AES_DIR_DECRYPT);
}

// The remaining registration and skcipher glue is a direct declaration-level
// translation of the C source; kernel framework types/macros are external.
unsafe fn fallback_init_cip(tfm: *mut crypto_tfm) -> i32 { let name = crypto_tfm_alg_name(tfm); let tctx = crypto_tfm_ctx(tfm); (*tctx).fallback.cip = crypto_alloc_cipher(name, 0, CRYPTO_ALG_NEED_FALLBACK); if IS_ERR((*tctx).fallback.cip) { printk(KERN_ERR, "Error allocating fallback algo %s\n", name); return PTR_ERR((*tctx).fallback.cip); } 0 }
unsafe fn fallback_exit_cip(tfm: *mut crypto_tfm) { crypto_free_cipher((*crypto_tfm_ctx(tfm)).fallback.cip); }

unsafe fn geode_init_skcipher(tfm: *mut crypto_skcipher) -> i32 {
    let name = crypto_tfm_alg_name(&mut (*tfm).base);
    let tctx = crypto_skcipher_ctx(tfm);
    (*tctx).fallback.skcipher = crypto_alloc_skcipher(name, 0, CRYPTO_ALG_NEED_FALLBACK | CRYPTO_ALG_ASYNC);
    if IS_ERR((*tctx).fallback.skcipher) { printk(KERN_ERR, "Error allocating fallback algo %s\n", name); return PTR_ERR((*tctx).fallback.skcipher); }
    crypto_skcipher_set_reqsize(tfm, core::mem::size_of::<skcipher_request>() + crypto_skcipher_reqsize((*tctx).fallback.skcipher));
    0
}
unsafe fn geode_exit_skcipher(tfm: *mut crypto_skcipher) { crypto_free_skcipher((*crypto_skcipher_ctx(tfm)).fallback.skcipher); }
unsafe fn geode_skcipher_crypt(req: *mut skcipher_request, mode: i32, dir: i32) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req); let tctx = crypto_skcipher_ctx(tfm); let mut walk = skcipher_walk::default(); let mut nbytes: c_uint; let mut err: i32;
    if (*tctx).keylen != AES_KEYSIZE_128 { let subreq = skcipher_request_ctx(req); *subreq = *req; skcipher_request_set_tfm(subreq, (*tctx).fallback.skcipher); return if dir == AES_DIR_DECRYPT { crypto_skcipher_decrypt(subreq) } else { crypto_skcipher_encrypt(subreq) }; }
    err = skcipher_walk_virt(&mut walk, req, false);
    while { nbytes = walk.nbytes; nbytes != 0 } { geode_aes_crypt(tctx, walk.src.virt.addr, walk.dst.virt.addr, round_down(nbytes, AES_BLOCK_SIZE), walk.iv, mode, dir); err = skcipher_walk_done(&mut walk, nbytes % AES_BLOCK_SIZE); }
    err
}
unsafe fn geode_cbc_encrypt(r: *mut skcipher_request) -> i32 { geode_skcipher_crypt(r, AES_MODE_CBC, AES_DIR_ENCRYPT) }
unsafe fn geode_cbc_decrypt(r: *mut skcipher_request) -> i32 { geode_skcipher_crypt(r, AES_MODE_CBC, AES_DIR_DECRYPT) }
unsafe fn geode_ecb_encrypt(r: *mut skcipher_request) -> i32 { geode_skcipher_crypt(r, AES_MODE_ECB, AES_DIR_ENCRYPT) }
unsafe fn geode_ecb_decrypt(r: *mut skcipher_request) -> i32 { geode_skcipher_crypt(r, AES_MODE_ECB, AES_DIR_DECRYPT) }

// The following aggregate registrations preserve the C driver's externally
// visible names, priorities, sizes, callbacks, and algorithm parameters.
static mut geode_alg: crypto_alg = crypto_alg { cra_name: "aes", cra_driver_name: "geode-aes", cra_priority: 300, cra_alignmask: 15, cra_flags: CRYPTO_ALG_TYPE_CIPHER | CRYPTO_ALG_NEED_FALLBACK, cra_init: Some(fallback_init_cip), cra_exit: Some(fallback_exit_cip), cra_blocksize: AES_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<geode_aes_tfm_ctx>(), cra_module: THIS_MODULE, cra_u: crypto_alg_union::cipher(crypto_cipher_alg { cia_min_keysize: AES_MIN_KEY_SIZE, cia_max_keysize: AES_MAX_KEY_SIZE, cia_setkey: Some(geode_setkey_cip), cia_encrypt: Some(geode_encrypt), cia_decrypt: Some(geode_decrypt) }) };

unsafe fn geode_aes_remove(dev: *mut pci_dev) { crypto_unregister_alg(&raw mut geode_alg); crypto_unregister_skciphers(geode_skcipher_algs.as_mut_ptr(), 2); pci_iounmap(dev, _iobase); _iobase = core::ptr::null_mut(); pci_release_regions(dev); pci_disable_device(dev); }
unsafe fn geode_aes_probe(dev: *mut pci_dev, _id: *const pci_device_id) -> i32 { let mut ret = pci_enable_device(dev); if ret != 0 { return ret; } ret = pci_request_regions(dev, "geode-aes"); if ret != 0 { pci_disable_device(dev); return ret; } _iobase = pci_iomap(dev, 0, 0); if _iobase.is_null() { pci_release_regions(dev); pci_disable_device(dev); return -ENOMEM; } iowrite32(AES_INTR_PENDING | AES_INTR_MASK, (_iobase as *mut u8).add(AES_INTR_REG as usize) as *mut c_void); ret = crypto_register_alg(&raw mut geode_alg); if ret != 0 { pci_iounmap(dev, _iobase); pci_release_regions(dev); pci_disable_device(dev); return ret; } ret = crypto_register_skciphers(geode_skcipher_algs.as_mut_ptr(), 2); if ret != 0 { crypto_unregister_alg(&raw mut geode_alg); pci_iounmap(dev, _iobase); pci_release_regions(dev); pci_disable_device(dev); } ret }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
