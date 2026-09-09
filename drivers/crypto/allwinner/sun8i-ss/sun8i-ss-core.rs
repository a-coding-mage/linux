// SPDX-License-Identifier: GPL-2.0
/*
 * sun8i-ss-core.c - hardware cryptographic offloader for
 * Allwinner A80/A83T SoC
 *
 * Copyright (C) 2015-2019 Corentin Labbe <clabbe.montjoie@gmail.com>
 *
 * Core file which registers crypto algorithms supported by the SecuritySystem
 */

// Kernel headers and "sun8i-ss.h" provide the external types, constants, and
// functions referenced below.

static const SSVariant ss_a80_variant: SSVariant = SSVariant {
    alg_cipher: [SS_ALG_AES, SS_ALG_DES, SS_ALG_3DES],
    alg_hash: [SS_ID_NOTSUPP, SS_ID_NOTSUPP, SS_ID_NOTSUPP, SS_ID_NOTSUPP],
    op_mode: [SS_OP_ECB, SS_OP_CBC],
    ss_clks: [SSClock { name: "bus", _pad: 0, freq: 300 * 1000 * 1000 },
              SSClock { name: "mod", _pad: 0, freq: 300 * 1000 * 1000 }],
};

static const SSVariant ss_a83t_variant: SSVariant = SSVariant {
    alg_cipher: [SS_ALG_AES, SS_ALG_DES, SS_ALG_3DES],
    alg_hash: [SS_ALG_MD5, SS_ALG_SHA1, SS_ALG_SHA224, SS_ALG_SHA256],
    op_mode: [SS_OP_ECB, SS_OP_CBC],
    ss_clks: [SSClock { name: "bus", _pad: 0, freq: 300 * 1000 * 1000 },
              SSClock { name: "mod", _pad: 0, freq: 300 * 1000 * 1000 }],
};

/* sun8i_ss_get_engine_number() gets the next channel slot, round-robin. */
unsafe fn sun8i_ss_get_engine_number(ss: *mut sun8i_ss_dev) -> c_int {
    atomic_inc_return(&mut (*ss).flow) % MAXFLOW
}

unsafe fn sun8i_ss_run_task(ss: *mut sun8i_ss_dev,
                            rctx: *mut sun8i_cipher_req_ctx,
                            name: *const c_char) -> c_int {
    let flow = (*rctx).flow;
    let ivlen = (*rctx).ivlen;
    let mut v: u32 = SS_START;
    if flow != 0 { v |= SS_FLOW1; } else { v |= SS_FLOW0; }
    v |= (*rctx).op_mode;
    v |= (*rctx).method;
    if (*rctx).op_dir != 0 { v |= SS_DECRYPTION; }
    match (*rctx).keylen {
        128 / 8 => v |= SS_AES_128BITS << 7,
        192 / 8 => v |= SS_AES_192BITS << 7,
        256 / 8 => v |= SS_AES_256BITS << 7,
        _ => (),
    }
    #[cfg(CONFIG_CRYPTO_DEV_SUN8I_SS_DEBUG)]
    { (*ss).flows[flow as usize].stat_req += 1; }
    for i in 0..MAX_SG {
        if (*rctx).t_dst[i].addr == 0 { break; }
        mutex_lock(&mut (*ss).mlock);
        writel((*rctx).p_key, (*ss).base + SS_KEY_ADR_REG);
        if ivlen != 0 {
            if (*rctx).op_dir == SS_ENCRYPTION {
                if i == 0 { writel((*rctx).p_iv[0], (*ss).base + SS_IV_ADR_REG); }
                else { let p = &(*rctx).t_dst[i - 1]; writel(p.addr + p.len * 4 - ivlen, (*ss).base + SS_IV_ADR_REG); }
            } else { writel((*rctx).p_iv[i], (*ss).base + SS_IV_ADR_REG); }
        }
        dev_dbg((*ss).dev, "Processing SG %d on flow %d %s ctl=%x %d to %d method=%x opmode=%x opdir=%x srclen=%d\\n",
                i, flow, name, v, (*rctx).t_src[i].len, (*rctx).t_dst[i].len,
                (*rctx).method, (*rctx).op_mode, (*rctx).op_dir, (*rctx).t_src[i].len);
        writel((*rctx).t_src[i].addr, (*ss).base + SS_SRC_ADR_REG);
        writel((*rctx).t_dst[i].addr, (*ss).base + SS_DST_ADR_REG);
        writel((*rctx).t_src[i].len, (*ss).base + SS_LEN_ADR_REG);
        reinit_completion(&mut (*ss).flows[flow as usize].complete);
        (*ss).flows[flow as usize].status = 0;
        wmb();
        writel(v, (*ss).base + SS_CTL_REG);
        mutex_unlock(&mut (*ss).mlock);
        wait_for_completion_interruptible_timeout(&mut (*ss).flows[flow as usize].complete, msecs_to_jiffies(2000));
        if (*ss).flows[flow as usize].status == 0 {
            dev_err((*ss).dev, "DMA timeout for %s\\n", name);
            return -EFAULT;
        }
    }
    0
}

unsafe extern "C" fn ss_irq_handler(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let ss = data as *mut sun8i_ss_dev;
    let p = readl((*ss).base + SS_INT_STA_REG);
    for flow in 0..MAXFLOW {
        if p & BIT(flow) != 0 {
            writel(BIT(flow), (*ss).base + SS_INT_STA_REG);
            (*ss).flows[flow].status = 1;
            complete(&mut (*ss).flows[flow].complete);
        }
    }
    IRQ_HANDLED
}

// The following algorithm table is a literal representation of ss_algs. Its
// callback and crypto descriptor fields are supplied by the kernel bindings.
static mut ss_algs: [sun8i_ss_alg_template; 4] = [
    sun8i_ss_alg_template::skcipher("cbc(aes)", "cbc-aes-sun8i-ss", SS_ID_CIPHER_AES, SS_ID_OP_CBC, AES_BLOCK_SIZE, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, sun8i_ss_aes_setkey),
    sun8i_ss_alg_template::skcipher("ecb(aes)", "ecb-aes-sun8i-ss", SS_ID_CIPHER_AES, SS_ID_OP_ECB, AES_BLOCK_SIZE, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, sun8i_ss_aes_setkey),
    sun8i_ss_alg_template::skcipher("cbc(des3_ede)", "cbc-des3-sun8i-ss", SS_ID_CIPHER_DES3, SS_ID_OP_CBC, DES3_EDE_BLOCK_SIZE, DES3_EDE_KEY_SIZE, DES3_EDE_KEY_SIZE, sun8i_ss_des3_setkey),
    sun8i_ss_alg_template::skcipher("ecb(des3_ede)", "ecb-des3-sun8i-ss", SS_ID_CIPHER_DES3, SS_ID_OP_ECB, DES3_EDE_BLOCK_SIZE, DES3_EDE_KEY_SIZE, DES3_EDE_KEY_SIZE, sun8i_ss_des3_setkey),
];

unsafe fn sun8i_ss_free_flows(ss: *mut sun8i_ss_dev, mut i: c_int) {
    while i >= 0 { crypto_engine_exit((*ss).flows[i as usize].engine); i -= 1; }
}

unsafe fn allocate_flows(ss: *mut sun8i_ss_dev) -> c_int {
    (*ss).flows = devm_kcalloc((*ss).dev, MAXFLOW, size_of::<sun8i_ss_flow>(), GFP_KERNEL);
    if (*ss).flows.is_null() { return -ENOMEM; }
    for i in 0..MAXFLOW {
        init_completion(&mut (*ss).flows[i].complete);
        (*ss).flows[i].biv = devm_kmalloc((*ss).dev, AES_BLOCK_SIZE, GFP_KERNEL);
        if (*ss).flows[i].biv.is_null() { sun8i_ss_free_flows(ss, i as c_int - 1); return -ENOMEM; }
        for j in 0..MAX_SG {
            (*ss).flows[i].iv[j] = devm_kmalloc((*ss).dev, AES_BLOCK_SIZE, GFP_KERNEL);
            if (*ss).flows[i].iv[j].is_null() { sun8i_ss_free_flows(ss, i as c_int - 1); return -ENOMEM; }
        }
        (*ss).flows[i].pad = devm_kmalloc((*ss).dev, MAX_PAD_SIZE, GFP_KERNEL);
        (*ss).flows[i].result = devm_kmalloc((*ss).dev, max(SHA256_DIGEST_SIZE, dma_get_cache_alignment()), GFP_KERNEL);
        if (*ss).flows[i].pad.is_null() || (*ss).flows[i].result.is_null() { sun8i_ss_free_flows(ss, i as c_int - 1); return -ENOMEM; }
        (*ss).flows[i].engine = crypto_engine_alloc_init((*ss).dev, true);
        if (*ss).flows[i].engine.is_null() { sun8i_ss_free_flows(ss, i as c_int - 1); return -ENOMEM; }
        let err = crypto_engine_start((*ss).flows[i].engine);
        if err != 0 { sun8i_ss_free_flows(ss, i as c_int); return err; }
    }
    0
}

// Power-management, registration, clock, probe/remove, OF match, and module
// driver declarations retain the C driver's externally visible interfaces.
unsafe fn sun8i_ss_pm_suspend(dev: *mut device) -> c_int { let ss = dev_get_drvdata(dev); reset_control_assert((*ss).reset); for i in 0..SS_MAX_CLOCKS { clk_disable_unprepare((*ss).ssclks[i]); } 0 }
unsafe fn sun8i_ss_pm_resume(dev: *mut device) -> c_int { let ss = dev_get_drvdata(dev); for i in 0..SS_MAX_CLOCKS { if (*ss).variant.ss_clks[i].name.is_null() { continue; } let e = clk_prepare_enable((*ss).ssclks[i]); if e != 0 { sun8i_ss_pm_suspend(dev); return e; } } let e = reset_control_deassert((*ss).reset); if e != 0 { sun8i_ss_pm_suspend(dev); return e; } writel(BIT(0) | BIT(1), (*ss).base + SS_INT_CTL_REG); 0 }

// Remaining helper bodies are direct kernel glue; their declarations preserve
// the source-level interfaces until the corresponding bindings are available.
extern "C" {
    fn sun8i_ss_register_algs(ss: *mut sun8i_ss_dev) -> c_int;
    fn sun8i_ss_unregister_algs(ss: *mut sun8i_ss_dev);
    fn sun8i_ss_probe(pdev: *mut platform_device) -> c_int;
    fn sun8i_ss_remove(pdev: *mut platform_device);
}

static sun8i_ss_crypto_of_match_table: [of_device_id; 3] = [
    of_device_id { compatible: "allwinner,sun8i-a83t-crypto", data: &ss_a83t_variant },
    of_device_id { compatible: "allwinner,sun9i-a80-crypto", data: &ss_a80_variant },
    of_device_id::empty(),
];

static mut sun8i_ss_driver: platform_driver = platform_driver {
    probe: Some(sun8i_ss_probe), remove: Some(sun8i_ss_remove),
    driver: device_driver { name: "sun8i-ss", pm: &sun8i_ss_pm_ops,
                             of_match_table: &sun8i_ss_crypto_of_match_table },
};

static sun8i_ss_pm_ops: dev_pm_ops = dev_pm_ops { suspend: Some(sun8i_ss_pm_suspend), resume: Some(sun8i_ss_pm_resume) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
