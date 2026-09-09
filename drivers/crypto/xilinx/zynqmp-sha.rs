// SPDX-License-Identifier: GPL-2.0
/*
 * Xilinx ZynqMP SHA Driver.
 * Copyright (c) 2022 Xilinx Inc.
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

const ZYNQMP_DMA_BIT_MASK: u32 = 32;
const ZYNQMP_DMA_ALLOC_FIXED_SIZE: u32 = 0x1000;

#[repr(i32)]
enum zynqmp_sha_op {
    ZYNQMP_SHA3_INIT = 1,
    ZYNQMP_SHA3_UPDATE = 2,
    ZYNQMP_SHA3_FINAL = 4,
}

#[repr(C)]
struct zynqmp_sha_drv_ctx {
    sha3_384: shash_alg,
    dev: *mut device,
}

#[repr(C)]
struct zynqmp_sha_tfm_ctx {
    dev: *mut device,
    fbk_tfm: *mut crypto_shash,
}

static mut update_dma_addr: dma_addr_t = 0;
static mut final_dma_addr: dma_addr_t = 0;
static mut ubuf: *mut i8 = core::ptr::null_mut();
static mut fbuf: *mut i8 = core::ptr::null_mut();

static mut zynqmp_sha_lock: spinlock_t = spinlock_t::new();

unsafe fn zynqmp_sha_init_tfm(hash: *mut crypto_shash) -> i32 {
    let fallback_driver_name = crypto_shash_alg_name(hash);
    let tfm_ctx = crypto_shash_ctx(hash) as *mut zynqmp_sha_tfm_ctx;
    let alg = crypto_shash_alg(hash);
    let fallback_tfm: *mut crypto_shash;
    let drv_ctx: *mut zynqmp_sha_drv_ctx;

    drv_ctx = container_of!(alg, zynqmp_sha_drv_ctx, sha3_384);
    (*tfm_ctx).dev = (*drv_ctx).dev;

    /* Allocate a fallback and abort if it failed. */
    fallback_tfm = crypto_alloc_shash(fallback_driver_name, 0, CRYPTO_ALG_NEED_FALLBACK);
    if IS_ERR(fallback_tfm) {
        return PTR_ERR(fallback_tfm);
    }

    if crypto_shash_descsize(hash) < crypto_shash_statesize((*tfm_ctx).fbk_tfm) {
        crypto_free_shash(fallback_tfm);
        return -EINVAL;
    }

    (*tfm_ctx).fbk_tfm = fallback_tfm;
    0
}

unsafe fn zynqmp_sha_exit_tfm(hash: *mut crypto_shash) {
    let tfm_ctx = crypto_shash_ctx(hash) as *mut zynqmp_sha_tfm_ctx;
    crypto_free_shash((*tfm_ctx).fbk_tfm);
}

unsafe fn zynqmp_sha_continue(desc: *mut shash_desc, fbdesc: *mut shash_desc, mut err: i32) -> i32 {
    if err == 0 {
        err = crypto_shash_export(fbdesc, shash_desc_ctx(desc));
    }
    shash_desc_zero(fbdesc);
    err
}

unsafe fn zynqmp_sha_init(desc: *mut shash_desc) -> i32 {
    let tctx = crypto_shash_ctx((*desc).tfm) as *mut zynqmp_sha_tfm_ctx;
    let fbtfm = (*tctx).fbk_tfm;
    let mut fbdesc = SHASH_DESC_ON_STACK!(fbtfm);
    (*fbdesc).tfm = fbtfm;
    let err = crypto_shash_init(fbdesc);
    zynqmp_sha_continue(desc, fbdesc, err)
}

unsafe fn zynqmp_sha_update(desc: *mut shash_desc, data: *const u8, length: u32) -> i32 {
    let tctx = crypto_shash_ctx((*desc).tfm) as *mut zynqmp_sha_tfm_ctx;
    let fbtfm = (*tctx).fbk_tfm;
    let mut fbdesc = SHASH_DESC_ON_STACK!(fbtfm);
    (*fbdesc).tfm = fbtfm;
    let mut err = crypto_shash_import(fbdesc, shash_desc_ctx(desc));
    if err == 0 {
        err = crypto_shash_update(fbdesc, data, length);
    }
    zynqmp_sha_continue(desc, fbdesc, err)
}

unsafe fn zynqmp_sha_finup(desc: *mut shash_desc, data: *const u8, length: u32, out: *mut u8) -> i32 {
    let tctx = crypto_shash_ctx((*desc).tfm) as *mut zynqmp_sha_tfm_ctx;
    let fbtfm = (*tctx).fbk_tfm;
    let mut fbdesc = SHASH_DESC_ON_STACK!(fbtfm);
    (*fbdesc).tfm = fbtfm;
    let err = crypto_shash_import(fbdesc, shash_desc_ctx(desc));
    if err != 0 { err } else { crypto_shash_finup(fbdesc, data, length, out) }
}

unsafe fn __zynqmp_sha_digest(_desc: *mut shash_desc, mut data: *const u8, len: u32, out: *mut u8) -> i32 {
    let mut remaining_len = len;
    let mut update_size: u32;
    let ret = zynqmp_pm_sha_hash(0, 0, ZYNQMP_SHA3_INIT as u32);
    if ret != 0 { return ret; }

    while remaining_len != 0 {
        memzero_explicit(ubuf as *mut _, ZYNQMP_DMA_ALLOC_FIXED_SIZE as usize);
        if remaining_len >= ZYNQMP_DMA_ALLOC_FIXED_SIZE {
            update_size = ZYNQMP_DMA_ALLOC_FIXED_SIZE;
            remaining_len -= ZYNQMP_DMA_ALLOC_FIXED_SIZE;
        } else {
            update_size = remaining_len;
            remaining_len = 0;
        }
        memcpy(ubuf as *mut _, data, update_size as usize);
        flush_icache_range(ubuf as usize, ubuf as usize + update_size as usize);
        let ret = zynqmp_pm_sha_hash(update_dma_addr, update_size, ZYNQMP_SHA3_UPDATE as u32);
        if ret != 0 { return ret; }
        data = data.add(update_size as usize);
    }

    let ret = zynqmp_pm_sha_hash(final_dma_addr, SHA3_384_DIGEST_SIZE, ZYNQMP_SHA3_FINAL as u32);
    memcpy(out as *mut _, fbuf as *const _, SHA3_384_DIGEST_SIZE as usize);
    memzero_explicit(fbuf as *mut _, SHA3_384_DIGEST_SIZE as usize);
    ret
}

unsafe fn zynqmp_sha_digest(desc: *mut shash_desc, data: *const u8, len: u32, out: *mut u8) -> i32 {
    let _guard = spinlock_bh_guard(&mut zynqmp_sha_lock);
    __zynqmp_sha_digest(desc, data, len, out)
}

static mut sha3_drv_ctx: zynqmp_sha_drv_ctx = zynqmp_sha_drv_ctx {
    sha3_384: shash_alg {
        init: Some(zynqmp_sha_init), update: Some(zynqmp_sha_update),
        finup: Some(zynqmp_sha_finup), digest: Some(zynqmp_sha_digest),
        init_tfm: Some(zynqmp_sha_init_tfm), exit_tfm: Some(zynqmp_sha_exit_tfm),
        descsize: SHA3_384_EXPORT_SIZE, digestsize: SHA3_384_DIGEST_SIZE,
        base: crypto_alg { cra_name: c"sha3-384", cra_driver_name: c"zynqmp-sha3-384",
            cra_priority: 300, cra_flags: CRYPTO_ALG_KERN_DRIVER_ONLY | CRYPTO_ALG_NEED_FALLBACK,
            cra_blocksize: SHA3_384_BLOCK_SIZE, cra_ctxsize: core::mem::size_of::<zynqmp_sha_tfm_ctx>(),
            cra_module: THIS_MODULE },
    },
    dev: core::ptr::null_mut(),
};

unsafe fn zynqmp_sha_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let mut v: u32 = 0;
    let mut err = zynqmp_pm_get_api_version(&mut v);
    if err != 0 { return err; }
    err = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(ZYNQMP_DMA_BIT_MASK));
    if err < 0 { dev_err(dev, c"No usable DMA configuration\n"); return err; }
    err = crypto_register_shash(&mut sha3_drv_ctx.sha3_384);
    if err < 0 { dev_err(dev, c"Failed to register shash alg.\n"); return err; }
    sha3_drv_ctx.dev = dev;
    platform_set_drvdata(pdev, &mut sha3_drv_ctx as *mut _);
    ubuf = dma_alloc_coherent(dev, ZYNQMP_DMA_ALLOC_FIXED_SIZE, &mut update_dma_addr, GFP_KERNEL);
    if ubuf.is_null() { err = -ENOMEM; crypto_unregister_shash(&mut sha3_drv_ctx.sha3_384); return err; }
    fbuf = dma_alloc_coherent(dev, SHA3_384_DIGEST_SIZE, &mut final_dma_addr, GFP_KERNEL);
    if fbuf.is_null() {
        dma_free_coherent(sha3_drv_ctx.dev, ZYNQMP_DMA_ALLOC_FIXED_SIZE, ubuf, update_dma_addr);
        crypto_unregister_shash(&mut sha3_drv_ctx.sha3_384);
        return -ENOMEM;
    }
    0
}

unsafe fn zynqmp_sha_remove(pdev: *mut platform_device) {
    sha3_drv_ctx.dev = platform_get_drvdata(pdev);
    dma_free_coherent(sha3_drv_ctx.dev, ZYNQMP_DMA_ALLOC_FIXED_SIZE, ubuf, update_dma_addr);
    dma_free_coherent(sha3_drv_ctx.dev, SHA3_384_DIGEST_SIZE, fbuf, final_dma_addr);
    crypto_unregister_shash(&mut sha3_drv_ctx.sha3_384);
}

static mut zynqmp_sha_driver: platform_driver = platform_driver {
    probe: Some(zynqmp_sha_probe), remove: Some(zynqmp_sha_remove),
    driver: device_driver { name: c"zynqmp-sha3-384" },
};

// module_platform_driver(zynqmp_sha_driver);
// MODULE_DESCRIPTION("ZynqMP SHA3 hardware acceleration support.");
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Harsha <harsha.harsha@xilinx.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
