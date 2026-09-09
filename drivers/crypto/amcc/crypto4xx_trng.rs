// SPDX-License-Identifier: GPL-2.0-only
/*
 * Generic PowerPC 44x RNG driver
 *
 * Copyright 2011 IBM Corporation
 */

const PPC4XX_TRNG_CTRL: usize = 0x0008;
const PPC4XX_TRNG_CTRL_DALM: u32 = 0x20;
const PPC4XX_TRNG_STAT: usize = 0x0004;
const PPC4XX_TRNG_STAT_B: u32 = 0x1;
const PPC4XX_TRNG_DATA: usize = 0x0000;

unsafe fn ppc4xx_trng_data_present(rng: *mut hwrng, wait: i32) -> i32 {
    let dev = (*rng).priv_ as *mut crypto4xx_device;
    let mut present = 0;

    for _i in 0..20 {
        let busy = in_le32((*dev).trng_base.add(PPC4XX_TRNG_STAT)) & PPC4XX_TRNG_STAT_B;
        if busy == 0 || wait == 0 {
            present = 1;
            break;
        }
        udelay(10);
    }
    present
}

unsafe fn ppc4xx_trng_data_read(rng: *mut hwrng, data: *mut u32) -> i32 {
    let dev = (*rng).priv_ as *mut crypto4xx_device;
    *data = in_le32((*dev).trng_base.add(PPC4XX_TRNG_DATA));
    4
}

unsafe fn ppc4xx_trng_enable(dev: *mut crypto4xx_device, enable: bool) {
    let mut device_ctrl = readl((*dev).ce_base.add(CRYPTO4XX_DEVICE_CTRL));
    if enable {
        device_ctrl |= PPC4XX_TRNG_EN;
    } else {
        device_ctrl &= !PPC4XX_TRNG_EN;
    }
    writel(device_ctrl, (*dev).ce_base.add(CRYPTO4XX_DEVICE_CTRL));
}

static ppc4xx_trng_match: [of_device_id; 4] = [
    of_device_id { compatible: c"ppc4xx-rng".as_ptr() },
    of_device_id { compatible: c"amcc,ppc460ex-rng".as_ptr() },
    of_device_id { compatible: c"amcc,ppc440epx-rng".as_ptr() },
    of_device_id { ..Default::default() },
];

unsafe fn ppc4xx_trng_probe(core_dev: *mut crypto4xx_core_device) {
    let dev = (*core_dev).dev;
    let mut trng: *mut device_node = core::ptr::null_mut();
    let mut rng: *mut hwrng = core::ptr::null_mut();
    let err: i32;

    /* Find the TRNG device node and map it */
    trng = of_find_matching_node(core::ptr::null_mut(), ppc4xx_trng_match.as_ptr());
    if trng.is_null() || !of_device_is_available(trng) {
        of_node_put(trng);
        return;
    }

    (*dev).trng_base = of_iomap(trng, 0);
    of_node_put(trng);
    if (*dev).trng_base.is_null() {
        goto err_out;
    }

    rng = kzalloc_obj::<hwrng>();
    if rng.is_null() {
        goto err_out;
    }

    (*rng).name = KBUILD_MODNAME;
    (*rng).data_present = Some(ppc4xx_trng_data_present);
    (*rng).data_read = Some(ppc4xx_trng_data_read);
    (*rng).priv_ = dev as unsigned_long;
    (*core_dev).trng = rng;
    ppc4xx_trng_enable(dev, true);
    out_le32((*dev).trng_base.add(PPC4XX_TRNG_CTRL), PPC4XX_TRNG_CTRL_DALM);
    err = devm_hwrng_register((*core_dev).device, (*core_dev).trng);
    if err != 0 {
        ppc4xx_trng_enable(dev, false);
        dev_err((*core_dev).device, "failed to register hwrng (%d).\n", err);
        goto err_out;
    }
    return;

err_out:
    iounmap((*dev).trng_base);
    kfree(rng);
    (*dev).trng_base = core::ptr::null_mut();
    (*core_dev).trng = core::ptr::null_mut();
}

unsafe fn ppc4xx_trng_remove(core_dev: *mut crypto4xx_core_device) {
    if !core_dev.is_null() && !(*core_dev).trng.is_null() {
        let dev = (*core_dev).dev;

        devm_hwrng_unregister((*core_dev).device, (*core_dev).trng);
        ppc4xx_trng_enable(dev, false);
        iounmap((*dev).trng_base);
        kfree((*core_dev).trng);
    }
}

module_alias!("ppc4xx_rng");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
