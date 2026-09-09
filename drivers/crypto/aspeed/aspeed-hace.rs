// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (c) 2021 Aspeed Technology Inc.
 */

// C dependencies supplied by the surrounding kernel translation.
use crate::*;

#[cfg(feature = "CONFIG_CRYPTO_DEV_ASPEED_DEBUG")]
macro_rules! HACE_DBG {
    ($d:expr, $fmt:expr $(, $arg:expr)*) => {
        dev_info(($d).dev, concat!(module_path!(), "() ", $fmt) $(, $arg)*);
    };
}

#[cfg(not(feature = "CONFIG_CRYPTO_DEV_ASPEED_DEBUG"))]
macro_rules! HACE_DBG {
    ($d:expr, $fmt:expr $(, $arg:expr)*) => {
        dev_dbg(($d).dev, concat!(module_path!(), "() ", $fmt) $(, $arg)*);
    };
}

/* HACE interrupt service routine */
unsafe extern "C" fn aspeed_hace_irq(_irq: i32, dev: *mut core::ffi::c_void) -> irqreturn_t {
    let hace_dev = &mut *(dev as *mut aspeed_hace_dev);
    let crypto_engine = &mut hace_dev.crypto_engine;
    let hash_engine = &mut hace_dev.hash_engine;
    let sts: u32;

    sts = ast_hace_read(hace_dev, ASPEED_HACE_STS);
    ast_hace_write(hace_dev, sts, ASPEED_HACE_STS);

    HACE_DBG!(hace_dev, "irq status: 0x%x\n", sts);

    if sts & HACE_HASH_ISR != 0 {
        if hash_engine.flags & CRYPTO_FLAGS_BUSY != 0 {
            tasklet_schedule(&mut hash_engine.done_task);
        } else {
            dev_warn(hace_dev.dev, "HASH no active requests.\n");
        }
    }

    if sts & HACE_CRYPTO_ISR != 0 {
        if crypto_engine.flags & CRYPTO_FLAGS_BUSY != 0 {
            tasklet_schedule(&mut crypto_engine.done_task);
        } else {
            dev_warn(hace_dev.dev, "CRYPTO no active requests.\n");
        }
    }

    IRQ_HANDLED
}

unsafe extern "C" fn aspeed_hace_crypto_done_task(data: c_ulong) {
    let hace_dev = &mut *(data as *mut aspeed_hace_dev);
    let crypto_engine = &mut hace_dev.crypto_engine;
    (crypto_engine.resume)(hace_dev);
}

unsafe extern "C" fn aspeed_hace_hash_done_task(data: c_ulong) {
    let hace_dev = &mut *(data as *mut aspeed_hace_dev);
    let hash_engine = &mut hace_dev.hash_engine;
    (hash_engine.resume)(hace_dev);
}

unsafe fn aspeed_hace_register(hace_dev: *mut aspeed_hace_dev) {
    #[cfg(feature = "CONFIG_CRYPTO_DEV_ASPEED_HACE_HASH")]
    aspeed_register_hace_hash_algs(hace_dev);
    #[cfg(feature = "CONFIG_CRYPTO_DEV_ASPEED_HACE_CRYPTO")]
    aspeed_register_hace_crypto_algs(hace_dev);
}

unsafe fn aspeed_hace_unregister(hace_dev: *mut aspeed_hace_dev) {
    #[cfg(feature = "CONFIG_CRYPTO_DEV_ASPEED_HACE_HASH")]
    aspeed_unregister_hace_hash_algs(hace_dev);
    #[cfg(feature = "CONFIG_CRYPTO_DEV_ASPEED_HACE_CRYPTO")]
    aspeed_unregister_hace_crypto_algs(hace_dev);
}

static aspeed_hace_of_matches: [of_device_id; 3] = [
    of_device_id { compatible: c_str!("aspeed,ast2500-hace"), data: 5 as *mut core::ffi::c_void },
    of_device_id { compatible: c_str!("aspeed,ast2600-hace"), data: 6 as *mut core::ffi::c_void },
    of_device_id::default(),
];

unsafe extern "C" fn aspeed_hace_probe(pdev: *mut platform_device) -> i32 {
    let mut rc: i32;
    let hace_dev = devm_kzalloc((*pdev).dev, core::mem::size_of::<aspeed_hace_dev>(), GFP_KERNEL)
        as *mut aspeed_hace_dev;
    if hace_dev.is_null() { return -ENOMEM; }

    (*hace_dev).version = device_get_match_data((*pdev).dev) as usize;
    if (*hace_dev).version == 0 {
        dev_err((*pdev).dev, "Failed to match hace dev id\n");
        return -EINVAL;
    }

    (*hace_dev).dev = (*pdev).dev;
    let hash_engine = &mut (*hace_dev).hash_engine;
    let crypto_engine = &mut (*hace_dev).crypto_engine;
    platform_set_drvdata(pdev, hace_dev as *mut core::ffi::c_void);

    (*hace_dev).regs = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if IS_ERR((*hace_dev).regs) { return PTR_ERR((*hace_dev).regs); }

    (*hace_dev).irq = platform_get_irq(pdev, 0);
    if (*hace_dev).irq < 0 { return (*hace_dev).irq; }
    rc = devm_request_irq((*pdev).dev, (*hace_dev).irq, aspeed_hace_irq, 0,
                          dev_name((*pdev).dev), hace_dev as *mut core::ffi::c_void);
    if rc != 0 { return rc; }

    (*hace_dev).clk = devm_clk_get((*pdev).dev, core::ptr::null());
    if IS_ERR((*hace_dev).clk) {
        dev_err((*pdev).dev, "Failed to get clk\n");
        return -ENODEV;
    }
    rc = clk_prepare_enable((*hace_dev).clk);
    if rc != 0 {
        dev_err((*pdev).dev, "Failed to enable clock 0x%x\n", rc);
        return rc;
    }

    (*hace_dev).crypt_engine_hash = crypto_engine_alloc_init((*hace_dev).dev, true);
    if (*hace_dev).crypt_engine_hash.is_null() { rc = -ENOMEM; goto clk_exit; }
    rc = crypto_engine_start((*hace_dev).crypt_engine_hash);
    if rc != 0 { goto err_engine_hash_start; }
    tasklet_init(&mut hash_engine.done_task, aspeed_hace_hash_done_task, hace_dev as c_ulong);

    (*hace_dev).crypt_engine_crypto = crypto_engine_alloc_init((*hace_dev).dev, true);
    if (*hace_dev).crypt_engine_crypto.is_null() { rc = -ENOMEM; goto err_engine_hash_start; }
    rc = crypto_engine_start((*hace_dev).crypt_engine_crypto);
    if rc != 0 { goto err_engine_crypto_start; }
    tasklet_init(&mut crypto_engine.done_task, aspeed_hace_crypto_done_task, hace_dev as c_ulong);

    hash_engine.ahash_src_addr = dmam_alloc_coherent((*pdev).dev, ASPEED_HASH_SRC_DMA_BUF_LEN,
        &mut hash_engine.ahash_src_dma_addr, GFP_KERNEL);
    if hash_engine.ahash_src_addr.is_null() {
        dev_err((*pdev).dev, "Failed to allocate dma buffer\n"); rc = -ENOMEM; goto err_engine_crypto_start;
    }
    crypto_engine.cipher_ctx = dmam_alloc_coherent((*pdev).dev, PAGE_SIZE,
        &mut crypto_engine.cipher_ctx_dma, GFP_KERNEL);
    if crypto_engine.cipher_ctx.is_null() {
        dev_err((*pdev).dev, "Failed to allocate cipher ctx dma\n"); rc = -ENOMEM; goto err_engine_crypto_start;
    }
    crypto_engine.cipher_addr = dmam_alloc_coherent((*pdev).dev, ASPEED_CRYPTO_SRC_DMA_BUF_LEN,
        &mut crypto_engine.cipher_dma_addr, GFP_KERNEL);
    if crypto_engine.cipher_addr.is_null() {
        dev_err((*pdev).dev, "Failed to allocate cipher addr dma\n"); rc = -ENOMEM; goto err_engine_crypto_start;
    }
    if (*hace_dev).version == AST2600_VERSION {
        crypto_engine.dst_sg_addr = dmam_alloc_coherent((*pdev).dev, ASPEED_CRYPTO_DST_DMA_BUF_LEN,
            &mut crypto_engine.dst_sg_dma_addr, GFP_KERNEL);
        if crypto_engine.dst_sg_addr.is_null() {
            dev_err((*pdev).dev, "Failed to allocate dst_sg dma\n"); rc = -ENOMEM; goto err_engine_crypto_start;
        }
    }
    aspeed_hace_register(hace_dev);
    dev_info((*pdev).dev, "Aspeed Crypto Accelerator successfully registered\n");
    return 0;

err_engine_crypto_start:
    crypto_engine_exit((*hace_dev).crypt_engine_crypto);
err_engine_hash_start:
    crypto_engine_exit((*hace_dev).crypt_engine_hash);
clk_exit:
    clk_disable_unprepare((*hace_dev).clk);
    rc
}

unsafe extern "C" fn aspeed_hace_remove(pdev: *mut platform_device) {
    let hace_dev = platform_get_drvdata(pdev) as *mut aspeed_hace_dev;
    aspeed_hace_unregister(hace_dev);
    crypto_engine_exit((*hace_dev).crypt_engine_hash);
    crypto_engine_exit((*hace_dev).crypt_engine_crypto);
    tasklet_kill(&mut (*hace_dev).hash_engine.done_task);
    tasklet_kill(&mut (*hace_dev).crypto_engine.done_task);
    clk_disable_unprepare((*hace_dev).clk);
}

MODULE_DEVICE_TABLE!(of, aspeed_hace_of_matches);

static mut aspeed_hace_driver: platform_driver = platform_driver {
    probe: Some(aspeed_hace_probe),
    remove: Some(aspeed_hace_remove),
    driver: device_driver {
        name: KBUILD_MODNAME,
        of_match_table: aspeed_hace_of_matches.as_ptr(),
    },
};

module_platform_driver!(aspeed_hace_driver);
MODULE_AUTHOR!("Neal Liu <neal_liu@aspeedtech.com>");
MODULE_DESCRIPTION!("Aspeed HACE driver Crypto Accelerator");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
