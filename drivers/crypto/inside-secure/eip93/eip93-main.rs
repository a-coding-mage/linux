// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 - 2021
 *
 * Richard van Schagen <vschagen@icloud.com>
 * Christian Marangi <ansuelsmth@gmail.com>
 */

// Linux and local C dependencies are supplied by the surrounding translation.

static mut EIP93_ALGS: [&mut eip93_alg_template; 32] = [
    &mut eip93_alg_ecb_des, &mut eip93_alg_cbc_des,
    &mut eip93_alg_ecb_des3_ede, &mut eip93_alg_cbc_des3_ede,
    &mut eip93_alg_ecb_aes, &mut eip93_alg_cbc_aes,
    &mut eip93_alg_ctr_aes, &mut eip93_alg_rfc3686_aes,
    &mut eip93_alg_md5, &mut eip93_alg_sha1,
    &mut eip93_alg_sha224, &mut eip93_alg_sha256,
    &mut eip93_alg_hmac_md5, &mut eip93_alg_hmac_sha1,
    &mut eip93_alg_hmac_sha224, &mut eip93_alg_hmac_sha256,
    &mut eip93_alg_authenc_hmac_md5_cbc_des,
    &mut eip93_alg_authenc_hmac_sha1_cbc_des,
    &mut eip93_alg_authenc_hmac_sha224_cbc_des,
    &mut eip93_alg_authenc_hmac_sha256_cbc_des,
    &mut eip93_alg_authenc_hmac_md5_cbc_des3_ede,
    &mut eip93_alg_authenc_hmac_sha1_cbc_des3_ede,
    &mut eip93_alg_authenc_hmac_sha224_cbc_des3_ede,
    &mut eip93_alg_authenc_hmac_sha256_cbc_des3_ede,
    &mut eip93_alg_authenc_hmac_md5_cbc_aes,
    &mut eip93_alg_authenc_hmac_sha1_cbc_aes,
    &mut eip93_alg_authenc_hmac_sha224_cbc_aes,
    &mut eip93_alg_authenc_hmac_sha256_cbc_aes,
    &mut eip93_alg_authenc_hmac_md5_rfc3686_aes,
    &mut eip93_alg_authenc_hmac_sha1_rfc3686_aes,
    &mut eip93_alg_authenc_hmac_sha224_rfc3686_aes,
    &mut eip93_alg_authenc_hmac_sha256_rfc3686_aes,
];

#[inline]
unsafe fn eip93_irq_disable(eip93: *mut eip93_device, mask: u32) { __raw_writel(mask, (*eip93).base.add(EIP93_REG_MASK_DISABLE)); }
#[inline]
unsafe fn eip93_irq_enable(eip93: *mut eip93_device, mask: u32) { __raw_writel(mask, (*eip93).base.add(EIP93_REG_MASK_ENABLE)); }
#[inline]
unsafe fn eip93_irq_clear(eip93: *mut eip93_device, mask: u32) { __raw_writel(mask, (*eip93).base.add(EIP93_REG_INT_CLR)); }

unsafe fn eip93_algo_is_supported(alg_flags: u32, supported_algo_flags: u32) -> i32 {
    if (IS_DES(alg_flags) || IS_3DES(alg_flags)) && supported_algo_flags & EIP93_PE_OPTION_TDES == 0 { return 0; }
    if IS_AES(alg_flags) && supported_algo_flags & EIP93_PE_OPTION_AES == 0 { return 0; }
    if IS_HASH_MD5(alg_flags) && supported_algo_flags & EIP93_PE_OPTION_MD5 == 0 { return 0; }
    if IS_HASH_SHA1(alg_flags) && supported_algo_flags & EIP93_PE_OPTION_SHA_1 == 0 { return 0; }
    if IS_HASH_SHA224(alg_flags) && supported_algo_flags & EIP93_PE_OPTION_SHA_224 == 0 { return 0; }
    if IS_HASH_SHA256(alg_flags) && supported_algo_flags & EIP93_PE_OPTION_SHA_256 == 0 { return 0; }
    1
}

unsafe fn eip93_unregister_algs(supported_algo_flags: u32, i: usize) {
    for j in 0..i {
        if eip93_algo_is_supported(EIP93_ALGS[j].flags, supported_algo_flags) == 0 { continue; }
        match EIP93_ALGS[j].type_ {
            EIP93_ALG_TYPE_SKCIPHER => crypto_unregister_skcipher(&mut EIP93_ALGS[j].alg.skcipher),
            EIP93_ALG_TYPE_AEAD => crypto_unregister_aead(&mut EIP93_ALGS[j].alg.aead),
            EIP93_ALG_TYPE_HASH => crypto_unregister_ahash(&mut EIP93_ALGS[j].alg.ahash),
            _ => (),
        }
    }
}

unsafe fn eip93_register_algs(eip93: *mut eip93_device, supported_algo_flags: u32) -> i32 {
    let mut i = 0usize;
    while i < EIP93_ALGS.len() {
        let alg_flags = EIP93_ALGS[i].flags;
        EIP93_ALGS[i].eip93 = eip93;
        if eip93_algo_is_supported(alg_flags, supported_algo_flags) == 0 { i += 1; continue; }
        if IS_AES(alg_flags) && !IS_HMAC(alg_flags) {
            if supported_algo_flags & EIP93_PE_OPTION_AES_KEY128 != 0 { EIP93_ALGS[i].alg.skcipher.max_keysize = AES_KEYSIZE_128; }
            if supported_algo_flags & EIP93_PE_OPTION_AES_KEY192 != 0 { EIP93_ALGS[i].alg.skcipher.max_keysize = AES_KEYSIZE_192; }
            if supported_algo_flags & EIP93_PE_OPTION_AES_KEY256 != 0 { EIP93_ALGS[i].alg.skcipher.max_keysize = AES_KEYSIZE_256; }
            if IS_RFC3686(alg_flags) { EIP93_ALGS[i].alg.skcipher.max_keysize += CTR_RFC3686_NONCE_SIZE; }
        }
        let ret = match EIP93_ALGS[i].type_ {
            EIP93_ALG_TYPE_SKCIPHER => crypto_register_skcipher(&mut EIP93_ALGS[i].alg.skcipher),
            EIP93_ALG_TYPE_AEAD => crypto_register_aead(&mut EIP93_ALGS[i].alg.aead),
            EIP93_ALG_TYPE_HASH => crypto_register_ahash(&mut EIP93_ALGS[i].alg.ahash),
            _ => 0,
        };
        if ret != 0 { eip93_unregister_algs(supported_algo_flags, i); return ret; }
        i += 1;
    }
    0
}

unsafe fn eip93_handle_result_descriptor(eip93: *mut eip93_device) {
    let (mut async_req, mut desc_flags, mut crypto_idr, mut err, mut last_entry);
    'get_more: loop {
        let mut handled = 0;
        let mut left = readl((*eip93).base.add(EIP93_REG_PE_RD_COUNT)) & EIP93_PE_RD_COUNT;
        if left == 0 { eip93_irq_clear(eip93, EIP93_INT_RDR_THRESH); eip93_irq_enable(eip93, EIP93_INT_RDR_THRESH); return; }
        last_entry = false; desc_flags = 0; crypto_idr = 0; err = 0;
        while left != 0 {
            let rdesc = eip93_get_descriptor(eip93);
            if IS_ERR(rdesc) { dev_err((*eip93).dev, "Ndesc: %d nreq: %d\n", handled, left); err = -EIO; break; }
            let rdesc = rdesc as *mut eip93_descriptor;
            let (pe_ctrl_stat, pe_length);
            loop { pe_ctrl_stat = READ_ONCE((*rdesc).pe_ctrl_stat_word); pe_length = READ_ONCE((*rdesc).pe_length_word); if FIELD_GET(EIP93_PE_CTRL_PE_READY_DES_TRING_OWN, pe_ctrl_stat) == EIP93_PE_CTRL_PE_READY && FIELD_GET(EIP93_PE_LENGTH_HOST_PE_READY, pe_length) == EIP93_PE_LENGTH_PE_READY { break; } }
            err = (*rdesc).pe_ctrl_stat_word & (EIP93_PE_CTRL_PE_EXT_ERR_CODE | EIP93_PE_CTRL_PE_EXT_ERR | EIP93_PE_CTRL_PE_SEQNUM_ERR | EIP93_PE_CTRL_PE_PAD_ERR | EIP93_PE_CTRL_PE_AUTH_ERR);
            desc_flags = FIELD_GET(EIP93_PE_USER_ID_DESC_FLAGS, (*rdesc).user_id); crypto_idr = FIELD_GET(EIP93_PE_USER_ID_CRYPTO_IDR, (*rdesc).user_id);
            writel(1, (*eip93).base.add(EIP93_REG_PE_RD_COUNT)); eip93_irq_clear(eip93, EIP93_INT_RDR_THRESH); handled += 1; left -= 1;
            if desc_flags & EIP93_DESC_LAST != 0 { last_entry = true; break; }
        }
        if !last_entry { continue 'get_more; }
        async_req = idr_find(&mut (*(*eip93).ring).crypto_async_idr, crypto_idr); idr_remove(&mut (*(*eip93).ring).crypto_async_idr, crypto_idr);
        err = eip93_parse_ctrl_stat_err(eip93, err);
        if desc_flags & EIP93_DESC_SKCIPHER != 0 { eip93_skcipher_handle_result(async_req, err); }
        if desc_flags & EIP93_DESC_AEAD != 0 { eip93_aead_handle_result(async_req, err); }
        if desc_flags & EIP93_DESC_HASH != 0 { eip93_hash_handle_result(async_req, err); }
    }
}

unsafe extern "C" fn eip93_done_task(data: usize) { eip93_handle_result_descriptor(data as *mut eip93_device); }

unsafe extern "C" fn eip93_irq_handler(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let eip93 = data as *mut eip93_device; let irq_status = readl((*eip93).base.add(EIP93_REG_INT_MASK_STAT));
    if FIELD_GET(EIP93_INT_RDR_THRESH, irq_status) != 0 { eip93_irq_disable(eip93, EIP93_INT_RDR_THRESH); tasklet_schedule(&mut (*(*eip93).ring).done_task); return IRQ_HANDLED; }
    eip93_irq_clear(eip93, irq_status); if irq_status != 0 { eip93_irq_disable(eip93, irq_status); } IRQ_NONE
}

unsafe fn eip93_initialize(eip93: *mut eip93_device, supported_algo_flags: u32) {
    let mut val = EIP93_PE_CONFIG_RST_PE | EIP93_PE_CONFIG_RST_RING | EIP93_PE_TARGET_AUTO_RING_MODE | EIP93_PE_CONFIG_EN_CDR_UPDATE; writel(val, (*eip93).base.add(EIP93_REG_PE_CONFIG)); usleep_range(10, 20); val = readl((*eip93).base.add(EIP93_REG_PE_CONFIG)) & !(EIP93_PE_CONFIG_RST_PE | EIP93_PE_CONFIG_RST_RING); writel(val, (*eip93).base.add(EIP93_REG_PE_CONFIG));
    val = EIP93_PE_CLOCK_EN_PE_CLK; if supported_algo_flags & EIP93_PE_OPTION_TDES != 0 { val |= EIP93_PE_CLOCK_EN_DES_CLK; } if supported_algo_flags & EIP93_PE_OPTION_AES != 0 { val |= EIP93_PE_CLOCK_EN_AES_CLK; } if supported_algo_flags & (EIP93_PE_OPTION_MD5 | EIP93_PE_OPTION_SHA_1 | EIP93_PE_OPTION_SHA_224 | EIP93_PE_OPTION_SHA_256) != 0 { val |= EIP93_PE_CLOCK_EN_HASH_CLK; } writel(val, (*eip93).base.add(EIP93_REG_PE_CLOCK_CTRL));
    val = FIELD_PREP(EIP93_PE_OUTBUF_THRESH, 128) | FIELD_PREP(EIP93_PE_INBUF_THRESH, 128); writel(val, (*eip93).base.add(EIP93_REG_PE_BUF_THRESH)); eip93_irq_clear(eip93, EIP93_INT_ALL); eip93_irq_disable(eip93, EIP93_INT_ALL); val = FIELD_PREP(EIPR93_PE_CDR_THRESH, EIP93_RING_NUM - EIP93_RING_BUSY) | FIELD_PREP(EIPR93_PE_RD_TIMEOUT, 5) | EIPR93_PE_TIMEROUT_EN; writel(val, (*eip93).base.add(EIP93_REG_PE_RING_THRESH));
}

unsafe fn eip93_desc_free(eip93: *mut eip93_device) { writel(0, (*eip93).base.add(EIP93_REG_PE_RING_CONFIG)); writel(0, (*eip93).base.add(EIP93_REG_PE_CDR_BASE)); writel(0, (*eip93).base.add(EIP93_REG_PE_RDR_BASE)); }

unsafe fn eip93_set_ring(eip93: *mut eip93_device, ring: *mut eip93_desc_ring) -> i32 {
    (*ring).offset = core::mem::size_of::<eip93_descriptor>(); (*ring).base = dmam_alloc_coherent((*eip93).dev, core::mem::size_of::<eip93_descriptor>() * EIP93_RING_NUM, &mut (*ring).base_dma, GFP_KERNEL); if (*ring).base.is_null() { return -ENOMEM; } (*ring).write = (*ring).base; (*ring).base_end = (*ring).base.add(core::mem::size_of::<eip93_descriptor>() * (EIP93_RING_NUM - 1)); (*ring).read = (*ring).base; 0
}

unsafe fn eip93_desc_init(eip93: *mut eip93_device) -> i32 { let cdr = &mut (*(*eip93).ring).cdr; let rdr = &mut (*(*eip93).ring).rdr; let mut ret = eip93_set_ring(eip93, cdr); if ret != 0 { return ret; } ret = eip93_set_ring(eip93, rdr); if ret != 0 { return ret; } writel(cdr.base_dma as u32, (*eip93).base.add(EIP93_REG_PE_CDR_BASE)); writel(rdr.base_dma as u32, (*eip93).base.add(EIP93_REG_PE_RDR_BASE)); writel(FIELD_PREP(EIP93_PE_RING_SIZE, EIP93_RING_NUM - 1), (*eip93).base.add(EIP93_REG_PE_RING_CONFIG)); 0 }

unsafe fn eip93_cleanup(eip93: *mut eip93_device) { tasklet_kill(&mut (*(*eip93).ring).done_task); eip93_irq_clear(eip93, EIP93_INT_ALL); eip93_irq_disable(eip93, EIP93_INT_ALL); writel(0, (*eip93).base.add(EIP93_REG_PE_CLOCK_CTRL)); eip93_desc_free(eip93); idr_destroy(&mut (*(*eip93).ring).crypto_async_idr); }

// Platform probe/remove, device match table, driver registration, and module metadata.
// These retain the source-level interfaces and are supplied by the surrounding kernel translation.
unsafe fn eip93_crypto_probe(pdev: *mut platform_device) -> i32 { let dev = &mut (*pdev).dev; let eip93 = devm_kzalloc(dev, struct_size::<eip93_device>(1), GFP_KERNEL); if eip93.is_null() { return -ENOMEM; } (*eip93).dev = dev; platform_set_drvdata(pdev, eip93); (*eip93).base = devm_platform_ioremap_resource(pdev, 0); if IS_ERR((*eip93).base) { return PTR_ERR((*eip93).base); } (*eip93).irq = platform_get_irq(pdev, 0); if (*eip93).irq < 0 { return (*eip93).irq; } let mut ret = devm_request_threaded_irq((*eip93).dev, (*eip93).irq, eip93_irq_handler, None, IRQF_ONESHOT, dev_name((*eip93).dev), eip93); if ret != 0 { return ret; } ret = eip93_desc_init(eip93); if ret != 0 { return ret; } tasklet_init(&mut (*(*eip93).ring).done_task, eip93_done_task, eip93 as usize); spin_lock_init(&mut (*(*eip93).ring).read_lock); spin_lock_init(&mut (*(*eip93).ring).write_lock); spin_lock_init(&mut (*(*eip93).ring).idr_lock); idr_init(&mut (*(*eip93).ring).crypto_async_idr); let algo_flags = readl((*eip93).base.add(EIP93_REG_PE_OPTION_1)); eip93_initialize(eip93, algo_flags); eip93_irq_enable(eip93, EIP93_INT_RDR_THRESH); ret = eip93_register_algs(eip93, algo_flags); if ret != 0 { eip93_cleanup(eip93); return ret; } 0 }
unsafe fn eip93_crypto_remove(pdev: *mut platform_device) { let eip93 = platform_get_drvdata(pdev); let flags = readl((*eip93).base.add(EIP93_REG_PE_OPTION_1)); eip93_unregister_algs(flags, EIP93_ALGS.len()); eip93_cleanup(eip93); }

static EIP93_CRYPTO_OF_MATCH: [of_device_id; 5] = [
    of_device_id { compatible: "inside-secure,safexcel-eip93i" },
    of_device_id { compatible: "inside-secure,safexcel-eip93ie" },
    of_device_id { compatible: "inside-secure,safexcel-eip93is" },
    of_device_id { compatible: "inside-secure,safexcel-eip93ies" },
    of_device_id { compatible: "" },
];
// IW not supported currently, missing AES-XCB-MAC/AES-CCM.

static EIP93_CRYPTO_DRIVER: platform_driver = platform_driver {
    probe: Some(eip93_crypto_probe),
    remove: Some(eip93_crypto_remove),
    driver: device_driver {
        name: "inside-secure-eip93",
        of_match_table: &EIP93_CRYPTO_OF_MATCH,
    },
};

// MODULE_DEVICE_TABLE(of, eip93_crypto_of_match);
// module_platform_driver(eip93_crypto_driver);
// MODULE_AUTHOR("Richard van Schagen <vschagen@cs.com>");
// MODULE_AUTHOR("Christian Marangi <ansuelsmth@gmail.com>");
// MODULE_DESCRIPTION("Mediatek EIP-93 crypto engine driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
