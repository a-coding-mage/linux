// SPDX-License-Identifier: GPL-2.0
/*
 * amlgoic-core.c - hardware cryptographic offloader for Amlogic GXL SoC
 *
 * Copyright (C) 2018-2019 Corentin Labbe <clabbe@baylibre.com>
 *
 * Core file which registers crypto algorithms supported by the hardware.
 */

// C dependencies: crypto/engine.h, crypto/internal/skcipher.h, linux/clk.h,
// linux/dma-mapping.h, linux/err.h, linux/interrupt.h, linux/io.h,
// linux/irq.h, linux/kernel.h, linux/module.h, linux/of.h,
// linux/platform_device.h, and amlogic-gxl.h.

unsafe fn meson_irq_handler(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let mc = data as *mut meson_dev;
    let mut flow: i32;
    let mut p: u32;

    flow = 0;
    while flow < MAXFLOW {
        if (*mc).irqs[flow as usize] == irq {
            p = readl((*mc).base.add(((0x04 + flow) << 2) as usize));
            if p != 0 {
                writel_relaxed(0xF, (*mc).base.add(((0x4 + flow) << 2) as usize));
                (*mc).chanlist[flow as usize].status = 1;
                complete(&mut (*mc).chanlist[flow as usize].complete);
                return IRQ_HANDLED;
            }
            dev_err((*mc).dev, "%s %d Got irq for flow %d but ctrl is empty\n", __func__, irq, flow);
        }
        flow += 1;
    }

    dev_err((*mc).dev, "%s %d from unknown irq\n", __func__, irq);
    IRQ_HANDLED
}

static mut mc_algs: [meson_alg_template; 2] = [
    meson_alg_template {
        r#type: CRYPTO_ALG_TYPE_SKCIPHER,
        blockmode: MESON_OPMODE_CBC,
        alg: meson_alg_union { skcipher: meson_skcipher_alg {
            base: skcipher_base { base: crypto_alg {
                cra_name: c"cbc(aes)", cra_driver_name: c"cbc-aes-gxl",
                cra_priority: 400, cra_blocksize: AES_BLOCK_SIZE,
                cra_flags: CRYPTO_ALG_TYPE_SKCIPHER | CRYPTO_ALG_ASYNC |
                    CRYPTO_ALG_ALLOCATES_MEMORY | CRYPTO_ALG_NEED_FALLBACK,
                cra_ctxsize: core::mem::size_of::<meson_cipher_tfm_ctx>(),
                cra_module: THIS_MODULE, cra_alignmask: 0xf,
                cra_init: Some(meson_cipher_init), cra_exit: Some(meson_cipher_exit),
            }, min_keysize: AES_MIN_KEY_SIZE, max_keysize: AES_MAX_KEY_SIZE,
            ivsize: AES_BLOCK_SIZE, setkey: meson_aes_setkey,
            encrypt: meson_skencrypt, decrypt: meson_skdecrypt,
        }, op: crypto_engine_op { do_one_request: meson_handle_cipher_request } }
    },
    meson_alg_template {
        r#type: CRYPTO_ALG_TYPE_SKCIPHER,
        blockmode: MESON_OPMODE_ECB,
        alg: meson_alg_union { skcipher: meson_skcipher_alg {
            base: skcipher_base { base: crypto_alg {
                cra_name: c"ecb(aes)", cra_driver_name: c"ecb-aes-gxl",
                cra_priority: 400, cra_blocksize: AES_BLOCK_SIZE,
                cra_flags: CRYPTO_ALG_TYPE_SKCIPHER | CRYPTO_ALG_ASYNC |
                    CRYPTO_ALG_ALLOCATES_MEMORY | CRYPTO_ALG_NEED_FALLBACK,
                cra_ctxsize: core::mem::size_of::<meson_cipher_tfm_ctx>(),
                cra_module: THIS_MODULE, cra_alignmask: 0xf,
                cra_init: Some(meson_cipher_init), cra_exit: Some(meson_cipher_exit),
            }, min_keysize: AES_MIN_KEY_SIZE, max_keysize: AES_MAX_KEY_SIZE,
            ivsize: 0, setkey: meson_aes_setkey,
            encrypt: meson_skencrypt, decrypt: meson_skdecrypt,
        }, op: crypto_engine_op { do_one_request: meson_handle_cipher_request } }
    },
];

unsafe fn meson_debugfs_show(seq: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let mc = (*seq).private as *mut meson_dev;
    let mut i = 0;
    while i < MAXFLOW {
        seq_printf(seq, c"Channel %d: nreq %lu\n", i,
            #[cfg(feature = "CONFIG_CRYPTO_DEV_AMLOGIC_GXL_DEBUG")] { (*mc).chanlist[i].stat_req },
            #[cfg(not(feature = "CONFIG_CRYPTO_DEV_AMLOGIC_GXL_DEBUG"))] { 0u64 });
        i += 1;
    }
    i = 0;
    while i < mc_algs.len() {
        if mc_algs[i].r#type == CRYPTO_ALG_TYPE_SKCIPHER {
            seq_printf(seq, c"%s %s %lu %lu\n", mc_algs[i].alg.skcipher.base.base.cra_driver_name,
                mc_algs[i].alg.skcipher.base.base.cra_name, 0u64, 0u64);
        }
        i += 1;
    }
    0
}
DEFINE_SHOW_ATTRIBUTE!(meson_debugfs);

unsafe fn meson_free_chanlist(mc: *mut meson_dev, mut i: i32) {
    while i >= 0 {
        crypto_engine_exit((*mc).chanlist[i as usize].engine);
        if !(*mc).chanlist[i as usize].tl.is_null() {
            dma_free_coherent((*mc).dev, core::mem::size_of::<meson_desc>() * MAXDESC,
                (*mc).chanlist[i as usize].tl, (*mc).chanlist[i as usize].t_phy);
        }
        i -= 1;
    }
}

/* Allocate the channel list structure */
unsafe fn meson_allocate_chanlist(mc: *mut meson_dev) -> i32 {
    let mut i: i32;
    let mut err: i32;
    (*mc).chanlist = devm_kcalloc((*mc).dev, MAXFLOW, core::mem::size_of::<meson_flow>(), GFP_KERNEL);
    if (*mc).chanlist.is_null() { return -ENOMEM; }
    i = 0;
    while i < MAXFLOW {
        init_completion(&mut (*mc).chanlist[i as usize].complete);
        (*mc).chanlist[i as usize].engine = crypto_engine_alloc_init((*mc).dev, true);
        if (*mc).chanlist[i as usize].engine.is_null() {
            dev_err((*mc).dev, c"Cannot allocate engine\n"); i -= 1; err = -ENOMEM; goto error_engine;
        }
        err = crypto_engine_start((*mc).chanlist[i as usize].engine);
        if err != 0 { dev_err((*mc).dev, c"Cannot start engine\n"); goto error_engine; }
        (*mc).chanlist[i as usize].tl = dma_alloc_coherent((*mc).dev,
            core::mem::size_of::<meson_desc>() * MAXDESC,
            &mut (*mc).chanlist[i as usize].t_phy, GFP_KERNEL);
        if (*mc).chanlist[i as usize].tl.is_null() { err = -ENOMEM; goto error_engine; }
        i += 1;
    }
    return 0;
error_engine:
    meson_free_chanlist(mc, i); err
}

unsafe fn meson_register_algs(mc: *mut meson_dev) -> i32 {
    let mut i = 0;
    while i < mc_algs.len() {
        mc_algs[i].mc = mc;
        if mc_algs[i].r#type == CRYPTO_ALG_TYPE_SKCIPHER {
            let err = crypto_engine_register_skcipher(&mut mc_algs[i].alg.skcipher);
            if err != 0 { dev_err((*mc).dev, c"Fail to register %s\n", mc_algs[i].alg.skcipher.base.base.cra_name); mc_algs[i].mc = core::ptr::null_mut(); return err; }
        }
        i += 1;
    }
    0
}

unsafe fn meson_unregister_algs(mc: *mut meson_dev) {
    let mut i = 0;
    while i < mc_algs.len() {
        if mc_algs[i].mc.is_null() { i += 1; continue; }
        if mc_algs[i].r#type == CRYPTO_ALG_TYPE_SKCIPHER { crypto_engine_unregister_skcipher(&mut mc_algs[i].alg.skcipher); }
        i += 1;
    }
}

unsafe fn meson_crypto_probe(pdev: *mut platform_device) -> i32 {
    let mc = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<meson_dev>(), GFP_KERNEL) as *mut meson_dev;
    if mc.is_null() { return -ENOMEM; }
    (*mc).dev = &mut (*pdev).dev; platform_set_drvdata(pdev, mc as *mut core::ffi::c_void);
    (*mc).base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*mc).base) { return PTR_ERR((*mc).base); }
    (*mc).busclk = devm_clk_get(&mut (*pdev).dev, c"blkmv");
    if IS_ERR((*mc).busclk) { let err = PTR_ERR((*mc).busclk); dev_err(&mut (*pdev).dev, c"Cannot get core clock err=%d\n", err); return err; }
    let mut i = 0;
    while i < MAXFLOW { (*mc).irqs[i as usize] = platform_get_irq(pdev, i); if (*mc).irqs[i as usize] < 0 { return (*mc).irqs[i as usize]; } let err = devm_request_irq(&mut (*pdev).dev, (*mc).irqs[i as usize], Some(meson_irq_handler), 0, c"gxl-crypto", mc as *mut core::ffi::c_void); if err < 0 { return err; } i += 1; }
    let mut err = clk_prepare_enable((*mc).busclk); if err != 0 { dev_err(&mut (*pdev).dev, c"Cannot prepare_enable busclk\n"); return err; }
    err = meson_allocate_chanlist(mc); if err != 0 { goto error_flow; }
    err = meson_register_algs(mc); if err != 0 { goto error_alg; }
    if IS_ENABLED(CONFIG_CRYPTO_DEV_AMLOGIC_GXL_DEBUG) { let dbgfs_dir = debugfs_create_dir(c"gxl-crypto", core::ptr::null_mut()); debugfs_create_file(c"stats", 0o444, dbgfs_dir, mc as *mut core::ffi::c_void, &meson_debugfs_fops); (*mc).dbgfs_dir = dbgfs_dir; }
    return 0;
error_alg: meson_unregister_algs(mc); meson_free_chanlist(mc, MAXFLOW - 1);
error_flow: clk_disable_unprepare((*mc).busclk); err
}

unsafe fn meson_crypto_remove(pdev: *mut platform_device) {
    let mc = platform_get_drvdata(pdev) as *mut meson_dev;
    debugfs_remove_recursive((*mc).dbgfs_dir); meson_unregister_algs(mc);
    meson_free_chanlist(mc, MAXFLOW - 1); clk_disable_unprepare((*mc).busclk);
}

static meson_crypto_of_match_table: [of_device_id; 2] = [of_device_id { compatible: c"amlogic,gxl-crypto" }, of_device_id { compatible: core::ptr::null() }];
MODULE_DEVICE_TABLE!(of, meson_crypto_of_match_table);
static mut meson_crypto_driver: platform_driver = platform_driver { probe: Some(meson_crypto_probe), remove: Some(meson_crypto_remove), driver: driver { name: c"gxl-crypto", of_match_table: meson_crypto_of_match_table.as_ptr() } };
module_platform_driver!(meson_crypto_driver);
MODULE_DESCRIPTION!(c"Amlogic GXL cryptographic offloader");
MODULE_LICENSE!(c"GPL");
MODULE_AUTHOR!(c"Corentin Labbe <clabbe@baylibre.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
