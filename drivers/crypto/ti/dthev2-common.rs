// SPDX-License-Identifier: GPL-2.0-only
/*
 * K3 DTHE V2 crypto accelerator driver
 *
 * Copyright (C) Texas Instruments 2025 - https://www.ti.com
 * Author: T Pratham <t-pratham@ti.com>
 */

// Dependencies are supplied by the surrounding kernel translation.

const DRIVER_NAME: &str = "dthev2";

static mut DTHE_DEV_LIST: dthe_list = dthe_list {
    dev_list: LIST_HEAD_INIT,
    lock: SPIN_LOCK_UNLOCKED,
};

pub unsafe fn dthe_get_dev(ctx: *mut dthe_tfm_ctx) -> *mut dthe_data {
    let mut dev_data: *mut dthe_data;

    if !(*ctx).dev_data.is_null() {
        return (*ctx).dev_data;
    }

    spin_lock_bh(&mut DTHE_DEV_LIST.lock);
    dev_data = list_first_entry(
        &mut DTHE_DEV_LIST.dev_list,
        core::mem::size_of::<dthe_data>(),
    );
    if !dev_data.is_null() {
        list_move_tail(&mut (*dev_data).list, &mut DTHE_DEV_LIST.dev_list);
    }
    spin_unlock_bh(&mut DTHE_DEV_LIST.lock);

    dev_data
}

pub unsafe fn dthe_copy_sg(
    mut dst: *mut scatterlist,
    mut src: *mut scatterlist,
    mut buflen: i32,
) -> *mut scatterlist {
    let mut sglen: i32;

    while buflen != 0 && !src.is_null() {
        sglen = (*src).length as i32;
        if sglen > buflen {
            sglen = buflen;
        }
        sg_set_buf(dst, sg_virt(src), sglen as usize);
        src = sg_next(src);
        dst = sg_next(dst);
        buflen -= sglen;
    }

    dst
}

unsafe fn dthe_dma_init(dev_data: *mut dthe_data) -> i32 {
    let mut ret: i32;
    let mut cfg: dma_slave_config = core::mem::zeroed();

    (*dev_data).dma_aes_rx = core::ptr::null_mut();
    (*dev_data).dma_aes_tx = core::ptr::null_mut();
    (*dev_data).dma_sha_tx = core::ptr::null_mut();

    (*dev_data).dma_aes_rx = dma_request_chan((*dev_data).dev, "rx");
    if IS_ERR((*dev_data).dma_aes_rx) {
        return dev_err_probe(
            (*dev_data).dev,
            PTR_ERR((*dev_data).dma_aes_rx),
            "Unable to request rx DMA channel\n",
        );
    }

    (*dev_data).dma_aes_tx = dma_request_chan((*dev_data).dev, "tx1");
    if IS_ERR((*dev_data).dma_aes_tx) {
        ret = dev_err_probe(
            (*dev_data).dev,
            PTR_ERR((*dev_data).dma_aes_tx),
            "Unable to request tx1 DMA channel\n",
        );
        dma_release_channel((*dev_data).dma_aes_rx);
        return ret;
    }

    (*dev_data).dma_sha_tx = dma_request_chan((*dev_data).dev, "tx2");
    if IS_ERR((*dev_data).dma_sha_tx) {
        ret = dev_err_probe(
            (*dev_data).dev,
            PTR_ERR((*dev_data).dma_sha_tx),
            "Unable to request tx2 DMA channel\n",
        );
        dma_release_channel((*dev_data).dma_aes_tx);
        dma_release_channel((*dev_data).dma_aes_rx);
        return ret;
    }

    memzero_explicit(&mut cfg, core::mem::size_of::<dma_slave_config>());

    cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    cfg.src_maxburst = 4;

    ret = dmaengine_slave_config((*dev_data).dma_aes_rx, &mut cfg);
    if ret != 0 {
        dev_err((*dev_data).dev, "Can't configure IN dmaengine slave: %d\n", ret);
        dma_release_channel((*dev_data).dma_sha_tx);
        dma_release_channel((*dev_data).dma_aes_tx);
        dma_release_channel((*dev_data).dma_aes_rx);
        return ret;
    }

    cfg.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    cfg.dst_maxburst = 4;

    ret = dmaengine_slave_config((*dev_data).dma_aes_tx, &mut cfg);
    if ret != 0 {
        dev_err((*dev_data).dev, "Can't configure OUT dmaengine slave: %d\n", ret);
        dma_release_channel((*dev_data).dma_sha_tx);
        dma_release_channel((*dev_data).dma_aes_tx);
        dma_release_channel((*dev_data).dma_aes_rx);
        return ret;
    }

    0
}

unsafe fn dthe_register_algs() -> i32 {
    dthe_register_aes_algs()
}

unsafe fn dthe_unregister_algs() {
    dthe_unregister_aes_algs();
}

unsafe fn dthe_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let dev_data = devm_kzalloc(dev, core::mem::size_of::<dthe_data>(), GFP_KERNEL);
    let mut ret: i32;

    if dev_data.is_null() {
        return -ENOMEM;
    }

    (*dev_data).dev = dev;
    (*dev_data).regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*dev_data).regs) {
        return PTR_ERR((*dev_data).regs);
    }

    platform_set_drvdata(pdev, dev_data);

    spin_lock(&mut DTHE_DEV_LIST.lock);
    list_add(&mut (*dev_data).list, &mut DTHE_DEV_LIST.dev_list);
    spin_unlock(&mut DTHE_DEV_LIST.lock);

    ret = dthe_dma_init(dev_data);
    if ret != 0 {
        spin_lock(&mut DTHE_DEV_LIST.lock);
        list_del(&mut (*dev_data).list);
        spin_unlock(&mut DTHE_DEV_LIST.lock);
        return ret;
    }

    (*dev_data).engine = crypto_engine_alloc_init(dev, 1);
    if (*dev_data).engine.is_null() {
        ret = -ENOMEM;
        dma_release_channel((*dev_data).dma_aes_rx);
        dma_release_channel((*dev_data).dma_aes_tx);
        dma_release_channel((*dev_data).dma_sha_tx);
        spin_lock(&mut DTHE_DEV_LIST.lock);
        list_del(&mut (*dev_data).list);
        spin_unlock(&mut DTHE_DEV_LIST.lock);
        return ret;
    }

    ret = crypto_engine_start((*dev_data).engine);
    if ret != 0 {
        dev_err(dev, "Failed to start crypto engine\n");
        crypto_engine_exit((*dev_data).engine);
    } else {
        ret = dthe_register_algs();
        if ret != 0 {
            dev_err(dev, "Failed to register algs\n");
            crypto_engine_exit((*dev_data).engine);
        } else {
            return 0;
        }
    }

    dma_release_channel((*dev_data).dma_aes_rx);
    dma_release_channel((*dev_data).dma_aes_tx);
    dma_release_channel((*dev_data).dma_sha_tx);
    spin_lock(&mut DTHE_DEV_LIST.lock);
    list_del(&mut (*dev_data).list);
    spin_unlock(&mut DTHE_DEV_LIST.lock);
    ret
}

unsafe fn dthe_remove(pdev: *mut platform_device) {
    let dev_data = platform_get_drvdata(pdev);

    spin_lock(&mut DTHE_DEV_LIST.lock);
    list_del(&mut (*dev_data).list);
    spin_unlock(&mut DTHE_DEV_LIST.lock);

    dthe_unregister_algs();
    crypto_engine_exit((*dev_data).engine);
    dma_release_channel((*dev_data).dma_aes_rx);
    dma_release_channel((*dev_data).dma_aes_tx);
    dma_release_channel((*dev_data).dma_sha_tx);
}

static DTHE_OF_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: "ti,am62l-dthev2" },
    of_device_id { compatible: core::ptr::null() },
];

static mut DTHE_DRIVER: platform_driver = platform_driver {
    probe: Some(dthe_probe),
    remove: Some(dthe_remove),
    driver: driver {
        name: DRIVER_NAME,
        of_match_table: DTHE_OF_MATCH.as_ptr(),
    },
};

// Equivalent of module_platform_driver(dthe_driver).
module_platform_driver!(&mut DTHE_DRIVER);

module_author!("T Pratham <t-pratham@ti.com>");
module_description!("Texas Instruments DTHE V2 driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
