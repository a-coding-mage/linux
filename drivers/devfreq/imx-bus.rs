// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 NXP
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
struct ImxBus {
    profile: devfreq_dev_profile,
    devfreq: *mut devfreq,
    clk: *mut clk,
    icc_pdev: *mut platform_device,
}

unsafe fn imx_bus_target(dev: *mut device, freq: *mut c_ulong, flags: u32) -> c_int {
    let new_opp: *mut dev_pm_opp;
    let ret: c_int;

    new_opp = devfreq_recommended_opp(dev, freq, flags);
    if is_err(new_opp) {
        ret = ptr_err(new_opp);
        dev_err(dev, "failed to get recommended opp: %d\n", ret);
        return ret;
    }
    dev_pm_opp_put(new_opp);

    dev_pm_opp_set_rate(dev, *freq)
}

unsafe fn imx_bus_get_cur_freq(dev: *mut device, freq: *mut c_ulong) -> c_int {
    let priv_: *mut ImxBus = dev_get_drvdata(dev) as *mut ImxBus;

    *freq = clk_get_rate((*priv_).clk);

    0
}

unsafe fn imx_bus_exit(dev: *mut device) {
    let priv_: *mut ImxBus = dev_get_drvdata(dev) as *mut ImxBus;

    dev_pm_opp_of_remove_table(dev);
    platform_device_unregister((*priv_).icc_pdev);
}

/* imx_bus_init_icc() - register matching icc provider if required */
unsafe fn imx_bus_init_icc(dev: *mut device) -> c_int {
    let priv_: *mut ImxBus = dev_get_drvdata(dev) as *mut ImxBus;
    let icc_driver_name: *const c_char;

    if !of_property_present((*dev).of_node, cstr!("#interconnect-cells")) {
        return 0;
    }
    // CONFIG_INTERCONNECT_IMX is a build-time configuration condition.
    if !IS_ENABLED_CONFIG_INTERCONNECT_IMX {
        dev_warn(dev, "imx interconnect drivers disabled\n");
        return 0;
    }

    icc_driver_name = of_device_get_match_data(dev) as *const c_char;
    if icc_driver_name.is_null() {
        dev_err(dev, "unknown interconnect driver\n");
        return 0;
    }

    (*priv_).icc_pdev = platform_device_register_data(
        dev, icc_driver_name, -1, core::ptr::null(), 0);
    if is_err((*priv_).icc_pdev) {
        dev_err(dev, "failed to register icc provider %s: %ld\n",
            icc_driver_name, ptr_err((*priv_).icc_pdev));
        return ptr_err((*priv_).icc_pdev);
    }

    0
}

unsafe fn imx_bus_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let priv_: *mut ImxBus;
    let gov: *const c_char = DEVFREQ_GOV_USERSPACE;
    let ret: c_int;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<ImxBus>(), GFP_KERNEL);
    if priv_.is_null() {
        return -ENOMEM;
    }

    /*
     * Fetch the clock to adjust but don't explicitly enable.
     *
     * For imx bus clock clk_set_rate is safe no matter if the clock is on
     * or off and some peripheral side-buses might be off unless enabled by
     * drivers for devices on those specific buses.
     *
     * Rate adjustment on a disabled bus clock just takes effect later.
     */
    (*priv_).clk = devm_clk_get(dev, core::ptr::null());
    if is_err((*priv_).clk) {
        ret = ptr_err((*priv_).clk);
        dev_err(dev, "failed to fetch clk: %d\n", ret);
        return ret;
    }
    platform_set_drvdata(pdev, priv_ as *mut c_void);

    ret = dev_pm_opp_of_add_table(dev);
    if ret < 0 {
        dev_err(dev, "failed to get OPP table\n");
        return ret;
    }

    (*priv_).profile.target = Some(imx_bus_target);
    (*priv_).profile.exit = Some(imx_bus_exit);
    (*priv_).profile.get_cur_freq = Some(imx_bus_get_cur_freq);
    (*priv_).profile.initial_freq = clk_get_rate((*priv_).clk);

    (*priv_).devfreq = devm_devfreq_add_device(dev, &mut (*priv_).profile, gov, core::ptr::null_mut());
    if is_err((*priv_).devfreq) {
        ret = ptr_err((*priv_).devfreq);
        dev_err(dev, "failed to add devfreq device: %d\n", ret);
        goto_err: {
            dev_pm_opp_of_remove_table(dev);
            return ret;
        }
    }

    ret = imx_bus_init_icc(dev);
    if ret != 0 {
        dev_pm_opp_of_remove_table(dev);
        return ret;
    }

    0
}

static_of_device_id_array!(imx_bus_of_match, [
    of_device_id { compatible: cstr!("fsl,imx8mq-noc"), data: cstr!("imx8mq-interconnect") },
    of_device_id { compatible: cstr!("fsl,imx8mm-noc"), data: cstr!("imx8mm-interconnect") },
    of_device_id { compatible: cstr!("fsl,imx8mn-noc"), data: cstr!("imx8mn-interconnect") },
    of_device_id { compatible: cstr!("fsl,imx8mp-noc"), data: cstr!("imx8mp-interconnect") },
    of_device_id { compatible: cstr!("fsl,imx8m-noc"), data: core::ptr::null() },
    of_device_id { compatible: cstr!("fsl,imx8m-nic"), data: core::ptr::null() },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
]);

static mut imx_bus_platdrv: platform_driver = platform_driver {
    probe: Some(imx_bus_probe),
    driver: device_driver {
        name: cstr!("imx-bus-devfreq"),
        of_match_table: imx_bus_of_match,
        ..device_driver::ZERO
    },
    ..platform_driver::ZERO
};

module_platform_driver!(imx_bus_platdrv);

module_description!("Generic i.MX bus frequency scaling driver");
module_author!("Leonard Crestez <leonard.crestez@nxp.com>");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
