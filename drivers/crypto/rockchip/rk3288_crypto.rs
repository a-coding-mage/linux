// SPDX-License-Identifier: GPL-2.0-only
/*
 * Crypto acceleration support for Rockchip RK3288
 *
 * Copyright (c) 2015, Fuzhou Rockchip Electronics Co., Ltd
 *
 * Author: Zain Wang <zain.wang@rock-chips.com>
 *
 * Some ideas are from marvell-cesa.c and s5p-sss.c driver.
 */

// Kernel headers and rk3288_crypto.h provide the types, constants, macros,
// functions, and algorithm objects referenced below.

static mut rocklist: rockchip_ip = rockchip_ip {
    dev_list: LIST_HEAD_INIT!(rocklist.dev_list),
    lock: __SPIN_LOCK_UNLOCKED!(rocklist.lock),
};

pub unsafe fn get_rk_crypto() -> *mut rk_crypto_info {
    let first: *mut rk_crypto_info;
    spin_lock(&mut rocklist.lock);
    first = list_first_entry_or_null!(
        &mut rocklist.dev_list, rk_crypto_info, list);
    list_rotate_left(&mut rocklist.dev_list);
    spin_unlock(&mut rocklist.lock);
    first
}

static rk3288_variant: rk_variant = rk_variant {
    num_clks: 4,
    rkclks: [rk_clk { name: "sclk", max: 150000000 }],
};

static rk3328_variant: rk_variant = rk_variant { num_clks: 3 };
static rk3399_variant: rk_variant = rk_variant { num_clks: 3 };

unsafe fn rk_crypto_get_clks(dev: *mut rk_crypto_info) -> i32 {
    let mut err: i32;
    let mut cr: u64;
    (*dev).num_clks = devm_clk_bulk_get_all((*dev).dev, &mut (*dev).clks);
    if (*dev).num_clks < (*(*dev).variant).num_clks {
        dev_err!((*dev).dev, "Missing clocks, got {} instead of {}\n", (*dev).num_clks, (*(*dev).variant).num_clks);
        return -EINVAL;
    }
    for i in 0..(*dev).num_clks {
        cr = clk_get_rate((*dev).clks[i].clk);
        for j in 0..ARRAY_SIZE!((*(*dev).variant).rkclks) {
            if (*dev).variant.rkclks[j].max == 0 { continue; }
            if strcmp((*dev).variant.rkclks[j].name, (*dev).clks[i].id) != 0 { continue; }
            if cr > (*dev).variant.rkclks[j].max {
                err = clk_set_rate((*dev).clks[i].clk, (*dev).variant.rkclks[j].max);
                if err != 0 {
                    dev_err!((*dev).dev, "Fail downclocking %s from %lu to %lu\n", (*dev).variant.rkclks[j].name, cr, (*dev).variant.rkclks[j].max);
                } else {
                    dev_info!((*dev).dev, "Downclocking %s from %lu to %lu\n", (*dev).variant.rkclks[j].name, cr, (*dev).variant.rkclks[j].max);
                }
            }
        }
    }
    0
}

unsafe fn rk_crypto_enable_clk(dev: *mut rk_crypto_info) -> i32 {
    let err = clk_bulk_prepare_enable((*dev).num_clks, (*dev).clks);
    if err != 0 { dev_err!((*dev).dev, "Could not enable clock clks\n"); }
    err
}

unsafe fn rk_crypto_disable_clk(dev: *mut rk_crypto_info) {
    clk_bulk_disable_unprepare((*dev).num_clks, (*dev).clks);
}

/* Power management strategy: suspend until a request is handled; autosuspend is 2s. */
unsafe fn rk_crypto_pm_suspend(dev: *mut device) -> i32 {
    let rkdev = dev_get_drvdata(dev) as *mut rk_crypto_info;
    rk_crypto_disable_clk(rkdev);
    reset_control_assert((*rkdev).rst);
    0
}

unsafe fn rk_crypto_pm_resume(dev: *mut device) -> i32 {
    let rkdev = dev_get_drvdata(dev) as *mut rk_crypto_info;
    let ret = rk_crypto_enable_clk(rkdev);
    if ret != 0 { return ret; }
    reset_control_deassert((*rkdev).rst);
    0
}

static rk_crypto_pm_ops: dev_pm_ops = SET_RUNTIME_PM_OPS!(rk_crypto_pm_suspend, rk_crypto_pm_resume, None);

unsafe fn rk_crypto_pm_init(rkdev: *mut rk_crypto_info) -> i32 {
    pm_runtime_use_autosuspend((*rkdev).dev);
    pm_runtime_set_autosuspend_delay((*rkdev).dev, 2000);
    let err = pm_runtime_set_suspended((*rkdev).dev);
    if err != 0 { return err; }
    pm_runtime_enable((*rkdev).dev);
    err
}

unsafe fn rk_crypto_pm_exit(rkdev: *mut rk_crypto_info) { pm_runtime_disable((*rkdev).dev); }

unsafe fn rk_crypto_irq_handle(_irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    let dev = platform_get_drvdata(dev_id) as *mut rk_crypto_info;
    let interrupt_status = CRYPTO_READ!(dev, RK_CRYPTO_INTSTS);
    CRYPTO_WRITE!(dev, RK_CRYPTO_INTSTS, interrupt_status);
    (*dev).status = 1;
    if interrupt_status & 0x0a != 0 { dev_warn!((*dev).dev, "DMA Error\n"); (*dev).status = 0; }
    complete(&mut (*dev).complete);
    IRQ_HANDLED
}

static mut rk_cipher_algs: [*mut rk_crypto_tmp; 9] = [
    &mut rk_ecb_aes_alg, &mut rk_cbc_aes_alg, &mut rk_ecb_des_alg,
    &mut rk_cbc_des_alg, &mut rk_ecb_des3_ede_alg, &mut rk_cbc_des3_ede_alg,
    &mut rk_ahash_sha1, &mut rk_ahash_sha256, &mut rk_ahash_md5,
];

unsafe fn rk_crypto_debugfs_show(seq: *mut seq_file, _v: *mut c_void) -> i32 {
    let mut dd: *mut rk_crypto_info;
    spin_lock(&mut rocklist.lock);
    list_for_each_entry!(dd, &mut rocklist.dev_list, list) {
        seq_printf!(seq, "%s %s requests: %lu\n", dev_driver_string((*dd).dev), dev_name((*dd).dev), (*dd).nreq);
    }
    spin_unlock(&mut rocklist.lock);
    for i in 0..ARRAY_SIZE!(rk_cipher_algs) {
        if (*rk_cipher_algs[i]).dev.is_null() { continue; }
        match (*rk_cipher_algs[i]).type_ {
            CRYPTO_ALG_TYPE_SKCIPHER => {
                seq_printf!(seq, "%s %s reqs=%lu fallback=%lu\n", (*rk_cipher_algs[i]).alg.skcipher.base.base.cra_driver_name, (*rk_cipher_algs[i]).alg.skcipher.base.base.cra_name, (*rk_cipher_algs[i]).stat_req, (*rk_cipher_algs[i]).stat_fb);
                seq_printf!(seq, "\tfallback due to length: %lu\n", (*rk_cipher_algs[i]).stat_fb_len);
                seq_printf!(seq, "\tfallback due to alignment: %lu\n", (*rk_cipher_algs[i]).stat_fb_align);
                seq_printf!(seq, "\tfallback due to SGs: %lu\n", (*rk_cipher_algs[i]).stat_fb_sgdiff);
            },
            CRYPTO_ALG_TYPE_AHASH => seq_printf!(seq, "%s %s reqs=%lu fallback=%lu\n", (*rk_cipher_algs[i]).alg.hash.base.halg.base.cra_driver_name, (*rk_cipher_algs[i]).alg.hash.base.halg.base.cra_name, (*rk_cipher_algs[i]).stat_req, (*rk_cipher_algs[i]).stat_fb),
            _ => (),
        }
    }
    0
}

unsafe fn register_debugfs(_crypto_info: *mut rk_crypto_info) {
    let dbgfs_dir = debugfs_create_dir!("rk3288_crypto", core::ptr::null_mut());
    let _dbgfs_stats = debugfs_create_file!("stats", 0o444, dbgfs_dir, &mut rocklist, &rk_crypto_debugfs_fops);
    // CONFIG_CRYPTO_DEV_ROCKCHIP_DEBUG conditionally stores these entries.
}

unsafe fn rk_crypto_register(crypto_info: *mut rk_crypto_info) -> i32 {
    let mut i = 0;
    while i < ARRAY_SIZE!(rk_cipher_algs) {
        (*rk_cipher_algs[i]).dev = crypto_info;
        let err = match (*rk_cipher_algs[i]).type_ {
            CRYPTO_ALG_TYPE_SKCIPHER => {
                dev_info!((*crypto_info).dev, "Register %s as %s\n", (*rk_cipher_algs[i]).alg.skcipher.base.base.cra_name, (*rk_cipher_algs[i]).alg.skcipher.base.base.cra_driver_name);
                crypto_engine_register_skcipher(&mut (*rk_cipher_algs[i]).alg.skcipher)
            },
            CRYPTO_ALG_TYPE_AHASH => {
                dev_info!((*crypto_info).dev, "Register %s as %s\n", (*rk_cipher_algs[i]).alg.hash.base.halg.base.cra_name, (*rk_cipher_algs[i]).alg.hash.base.halg.base.cra_driver_name);
                crypto_engine_register_ahash(&mut (*rk_cipher_algs[i]).alg.hash)
            },
            _ => { dev_err!((*crypto_info).dev, "unknown algorithm\n"); 0 }
        };
        if err != 0 {
            for k in 0..i {
                if (*rk_cipher_algs[i]).type_ == CRYPTO_ALG_TYPE_SKCIPHER { crypto_engine_unregister_skcipher(&mut (*rk_cipher_algs[k]).alg.skcipher); }
                else { crypto_engine_unregister_ahash(&mut (*rk_cipher_algs[i]).alg.hash); }
            }
            return err;
        }
        i += 1;
    }
    0
}

unsafe fn rk_crypto_unregister() {
    for i in 0..ARRAY_SIZE!(rk_cipher_algs) {
        if (*rk_cipher_algs[i]).type_ == CRYPTO_ALG_TYPE_SKCIPHER { crypto_engine_unregister_skcipher(&mut (*rk_cipher_algs[i]).alg.skcipher); }
        else { crypto_engine_unregister_ahash(&mut (*rk_cipher_algs[i]).alg.hash); }
    }
}

static crypto_of_id_table: [of_device_id; 4] = [
    of_device_id { compatible: "rockchip,rk3288-crypto", data: &rk3288_variant },
    of_device_id { compatible: "rockchip,rk3328-crypto", data: &rk3328_variant },
    of_device_id { compatible: "rockchip,rk3399-crypto", data: &rk3399_variant },
    of_device_id { compatible: "", data: core::ptr::null() },
];

unsafe fn rk_crypto_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let crypto_info = devm_kzalloc!(dev, core::mem::size_of::<rk_crypto_info>(), GFP_KERNEL) as *mut rk_crypto_info;
    if crypto_info.is_null() { dev_err!(dev, "Crypto Accelerator not successfully registered\n"); return -ENOMEM; }
    (*crypto_info).dev = dev;
    platform_set_drvdata(pdev, crypto_info);
    (*crypto_info).variant = of_device_get_match_data(dev);
    if (*crypto_info).variant.is_null() { dev_err!(dev, "Missing variant\n"); return -EINVAL; }
    (*crypto_info).rst = devm_reset_control_array_get_exclusive(dev);
    if IS_ERR!((*crypto_info).rst) { dev_err!(dev, "Crypto Accelerator not successfully registered\n"); return PTR_ERR!((*crypto_info).rst); }
    reset_control_assert((*crypto_info).rst); usleep_range(10, 20); reset_control_deassert((*crypto_info).rst);
    (*crypto_info).reg = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR!((*crypto_info).reg) { return PTR_ERR!((*crypto_info).reg); }
    let mut err = rk_crypto_get_clks(crypto_info); if err != 0 { return err; }
    (*crypto_info).irq = platform_get_irq(pdev, 0); if (*crypto_info).irq < 0 { return (*crypto_info).irq; }
    err = devm_request_irq!(dev, (*crypto_info).irq, rk_crypto_irq_handle, IRQF_SHARED, "rk-crypto", pdev); if err != 0 { return err; }
    (*crypto_info).engine = crypto_engine_alloc_init(dev, true); if (*crypto_info).engine.is_null() { return -ENOMEM; }
    crypto_engine_start((*crypto_info).engine); init_completion(&mut (*crypto_info).complete);
    err = rk_crypto_pm_init(crypto_info); if err != 0 { crypto_engine_exit((*crypto_info).engine); return err; }
    spin_lock(&mut rocklist.lock); let first = list_first_entry_or_null!(&mut rocklist.dev_list, rk_crypto_info, list); list_add_tail!(&mut (*crypto_info).list, &mut rocklist.dev_list); spin_unlock(&mut rocklist.lock);
    if first.is_null() { err = rk_crypto_register(crypto_info); if err != 0 { rk_crypto_pm_exit(crypto_info); crypto_engine_exit((*crypto_info).engine); return err; } register_debugfs(crypto_info); }
    0
}

unsafe fn rk_crypto_remove(pdev: *mut platform_device) {
    let crypto_tmp = platform_get_drvdata(pdev) as *mut rk_crypto_info;
    spin_lock_bh(&mut rocklist.lock); list_del(&mut (*crypto_tmp).list); let first = list_first_entry_or_null!(&mut rocklist.dev_list, rk_crypto_info, list); spin_unlock_bh(&mut rocklist.lock);
    if first.is_null() { rk_crypto_unregister(); }
    rk_crypto_pm_exit(crypto_tmp); crypto_engine_exit((*crypto_tmp).engine);
}

static crypto_driver: platform_driver = platform_driver {
    probe: rk_crypto_probe, remove: rk_crypto_remove,
    driver: device_driver { name: "rk3288-crypto", pm: &rk_crypto_pm_ops, of_match_table: crypto_of_id_table.as_ptr() },
};

// The remaining kernel-only registration wrappers are intentionally expressed
// through their corresponding declarations/macros supplied by the dependency.
DEFINE_SHOW_ATTRIBUTE!(rk_crypto_debugfs);
MODULE_DEVICE_TABLE!(of, crypto_of_id_table);
module_platform_driver!(crypto_driver);
MODULE_AUTHOR!("Zain Wang <zain.wang@rock-chips.com>");
MODULE_DESCRIPTION!("Support for Rockchip's cryptographic engine");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
