// SPDX-License-Identifier: GPL-2.0
/*
 * Cryptographic API.
 *
 * Support for StarFive hardware cryptographic engine.
 * Copyright (c) 2022 StarFive Technology
 */

// External Linux-kernel and jh7110-cryp.h definitions are supplied by the
// surrounding translation unit.

const DRIVER_NAME: *const u8 = b"jh7110-crypto\0".as_ptr();

#[repr(C)]
struct StarfiveDevList {
    dev_list: ListHead,
    lock: Spinlock,
}

static mut DEV_LIST: StarfiveDevList = StarfiveDevList {
    dev_list: ListHead::new(),
    lock: Spinlock::new(),
};

pub unsafe fn starfive_cryp_find_dev(ctx: *mut StarfiveCrypCtx) -> *mut StarfiveCrypDev {
    let mut cryp: *mut StarfiveCrypDev;

    spin_lock_bh(&mut DEV_LIST.lock);
    if (*ctx).cryp.is_null() {
        (*ctx).cryp = list_first_entry_or_null(
            &mut DEV_LIST.dev_list,
            StarfiveCrypDev::list_offset(),
        );
    }
    cryp = (*ctx).cryp;
    spin_unlock_bh(&mut DEV_LIST.lock);

    cryp
}

static mut SIDE_CHAN: u16 = 0;

unsafe fn starfive_dma_init(cryp: *mut StarfiveCrypDev) -> i32 {
    let mut mask: DmaCapMask = DmaCapMask::zero();
    mask.set(DmaSlave);

    (*cryp).tx = dma_request_chan((*cryp).dev, b"tx\0".as_ptr());
    if is_err((*cryp).tx) {
        return dev_err_probe(
            (*cryp).dev,
            ptr_err((*cryp).tx),
            b"Error requesting tx dma channel.\n\0".as_ptr(),
        );
    }

    (*cryp).rx = dma_request_chan((*cryp).dev, b"rx\0".as_ptr());
    if is_err((*cryp).rx) {
        dma_release_channel((*cryp).tx);
        return dev_err_probe(
            (*cryp).dev,
            ptr_err((*cryp).rx),
            b"Error requesting rx dma channel.\n\0".as_ptr(),
        );
    }

    0
}

unsafe fn starfive_dma_cleanup(cryp: *mut StarfiveCrypDev) {
    dma_release_channel((*cryp).tx);
    dma_release_channel((*cryp).rx);
}

unsafe fn starfive_cryp_probe(pdev: *mut PlatformDevice) -> i32 {
    let cryp = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<StarfiveCrypDev>(),
        GFP_KERNEL,
    );
    if cryp.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, cryp);
    (*cryp).dev = &mut (*pdev).dev;

    let mut res: *mut Resource = core::ptr::null_mut();
    (*cryp).base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if is_err((*cryp).base) {
        return dev_err_probe(
            &mut (*pdev).dev,
            ptr_err((*cryp).base),
            b"Error remapping memory for platform device\n\0".as_ptr(),
        );
    }

    (*cryp).phys_base = (*res).start;
    (*cryp).dma_maxburst = 32;
    (*cryp).side_chan = SIDE_CHAN;

    (*cryp).hclk = devm_clk_get(&mut (*pdev).dev, b"hclk\0".as_ptr());
    if is_err((*cryp).hclk) {
        return dev_err_probe(&mut (*pdev).dev, ptr_err((*cryp).hclk), b"Error getting hardware reference clock\n\0".as_ptr());
    }
    (*cryp).ahb = devm_clk_get(&mut (*pdev).dev, b"ahb\0".as_ptr());
    if is_err((*cryp).ahb) {
        return dev_err_probe(&mut (*pdev).dev, ptr_err((*cryp).ahb), b"Error getting ahb reference clock\n\0".as_ptr());
    }
    (*cryp).rst = devm_reset_control_get_shared((*cryp).dev, core::ptr::null());
    if is_err((*cryp).rst) {
        return dev_err_probe((*cryp).dev, ptr_err((*cryp).rst), b"Error getting hardware reset line\n\0".as_ptr());
    }

    clk_prepare_enable((*cryp).hclk);
    clk_prepare_enable((*cryp).ahb);
    reset_control_deassert((*cryp).rst);

    spin_lock(&mut DEV_LIST.lock);
    list_add(&mut (*cryp).list, &mut DEV_LIST.dev_list);
    spin_unlock(&mut DEV_LIST.lock);

    let mut ret = starfive_dma_init(cryp);
    if ret != 0 { return goto_err_dma_init(cryp, ret); }
    (*cryp).engine = crypto_engine_alloc_init(&mut (*pdev).dev, 1);
    if (*cryp).engine.is_null() { ret = -ENOMEM; return goto_err_engine(cryp, ret); }
    ret = crypto_engine_start((*cryp).engine);
    if ret != 0 { return goto_err_engine_start(cryp, ret); }
    ret = starfive_aes_register_algs();
    if ret != 0 { return goto_err_engine_start(cryp, ret); }
    ret = starfive_hash_register_algs();
    if ret != 0 { starfive_aes_unregister_algs(); return goto_err_engine_start(cryp, ret); }
    ret = starfive_rsa_register_algs();
    if ret != 0 { starfive_hash_unregister_algs(); starfive_aes_unregister_algs(); return goto_err_engine_start(cryp, ret); }
    0
}

// Error labels from the C implementation are represented by the equivalent
// cleanup helpers below.
unsafe fn goto_err_engine_start(cryp: *mut StarfiveCrypDev, ret: i32) -> i32 { crypto_engine_exit((*cryp).engine); goto_err_engine(cryp, ret) }
unsafe fn goto_err_engine(cryp: *mut StarfiveCrypDev, ret: i32) -> i32 { starfive_dma_cleanup(cryp); goto_err_dma_init(cryp, ret) }
unsafe fn goto_err_dma_init(cryp: *mut StarfiveCrypDev, ret: i32) -> i32 {
    spin_lock(&mut DEV_LIST.lock); list_del(&mut (*cryp).list); spin_unlock(&mut DEV_LIST.lock);
    clk_disable_unprepare((*cryp).hclk); clk_disable_unprepare((*cryp).ahb); reset_control_assert((*cryp).rst); ret
}

unsafe fn starfive_cryp_remove(pdev: *mut PlatformDevice) {
    let cryp = platform_get_drvdata(pdev);
    starfive_aes_unregister_algs(); starfive_hash_unregister_algs(); starfive_rsa_unregister_algs();
    crypto_engine_exit((*cryp).engine); starfive_dma_cleanup(cryp);
    spin_lock(&mut DEV_LIST.lock); list_del(&mut (*cryp).list); spin_unlock(&mut DEV_LIST.lock);
    clk_disable_unprepare((*cryp).hclk); clk_disable_unprepare((*cryp).ahb); reset_control_assert((*cryp).rst);
}

// Device-table, platform-driver registration, module parameters, and metadata
// are provided through the kernel bindings corresponding to the C declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
