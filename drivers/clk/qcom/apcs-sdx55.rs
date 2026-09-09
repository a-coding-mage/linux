// SPDX-License-Identifier: GPL-2.0
/*
 * Qualcomm SDX55 APCS clock controller driver
 *
 * Copyright (c) 2020, Linaro Limited
 * Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
 */

// Linux kernel dependencies supplied by the surrounding build.

static APCS_MUX_CLK_PARENT_MAP: [u32; 3] = [0, 1, 5];

static PDATA: [clk_parent_data; 3] = [
    clk_parent_data { fw_name: "ref" },
    clk_parent_data { fw_name: "aux" },
    clk_parent_data { fw_name: "pll" },
];

/*
 * We use the notifier function for switching to a temporary safe configuration
 * (mux and divider), while the A7 PLL is reconfigured.
 */
unsafe extern "C" fn a7cc_notifier_cb(
    nb: *mut notifier_block,
    event: c_ulong,
    data: *mut c_void,
) -> c_int {
    let mut ret: c_int = 0;
    let md: *mut clk_regmap_mux_div = container_of!(
        nb,
        clk_regmap_mux_div,
        clk_nb
    );

    if event == PRE_RATE_CHANGE {
        /* set the mux and divider to safe frequency (400mhz) */
        ret = mux_div_set_src_div(md, 1, 2);
    }

    notifier_from_errno(ret)
}

unsafe extern "C" fn qcom_apcs_sdx55_clk_probe(
    pdev: *mut platform_device,
) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let parent: *mut device = (*dev).parent;
    let mut cpu_dev: *mut device;
    let a7cc: *mut clk_regmap_mux_div;
    let regmap: *mut regmap;
    let mut init: clk_init_data = core::mem::zeroed();
    let mut ret: c_int;

    regmap = dev_get_regmap(parent, core::ptr::null());
    if regmap.is_null() {
        dev_err(dev, "Failed to get parent regmap\n");
        return -ENODEV;
    }

    a7cc = devm_kzalloc(dev, core::mem::size_of::<clk_regmap_mux_div>(), GFP_KERNEL);
    if a7cc.is_null() {
        return -ENOMEM;
    }

    init.name = "a7mux";
    init.parent_data = PDATA.as_ptr();
    init.num_parents = ARRAY_SIZE!(PDATA);
    init.ops = &clk_regmap_mux_div_ops;

    (*a7cc).clkr.hw.init = &init;
    (*a7cc).clkr.regmap = regmap;
    (*a7cc).reg_offset = 0x8;
    (*a7cc).hid_width = 5;
    (*a7cc).hid_shift = 0;
    (*a7cc).src_width = 3;
    (*a7cc).src_shift = 8;
    (*a7cc).parent_map = APCS_MUX_CLK_PARENT_MAP.as_ptr();

    (*a7cc).pclk = devm_clk_get(parent, "pll");
    if IS_ERR!((*a7cc).pclk) {
        return dev_err_probe(dev, PTR_ERR!((*a7cc).pclk), "Failed to get PLL clk\n");
    }

    (*a7cc).clk_nb.notifier_call = Some(a7cc_notifier_cb);
    ret = clk_notifier_register((*a7cc).pclk, &mut (*a7cc).clk_nb);
    if ret != 0 {
        return dev_err_probe(dev, ret, "Failed to register clock notifier\n");
    }

    ret = devm_clk_register_regmap(dev, &mut (*a7cc).clkr);
    if ret != 0 {
        dev_err_probe(dev, ret, "Failed to register regmap clock\n");
        goto_err: {
            clk_notifier_unregister((*a7cc).pclk, &mut (*a7cc).clk_nb);
            return ret;
        }
    }

    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, &mut (*a7cc).clkr.hw);
    if ret != 0 {
        dev_err_probe(dev, ret, "Failed to add clock provider\n");
        clk_notifier_unregister((*a7cc).pclk, &mut (*a7cc).clk_nb);
        return ret;
    }

    platform_set_drvdata(pdev, a7cc);

    /*
     * Attach the power domain to cpudev. Since there is no dedicated driver
     * for CPUs and the SDX55 platform lacks hardware specific CPUFreq
     * driver, there seems to be no better place to do this. So do it here!
     */
    cpu_dev = get_cpu_device(0);
    ret = dev_pm_domain_attach(cpu_dev, PD_FLAG_ATTACH_POWER_ON);
    if ret != 0 {
        dev_err_probe(dev, ret, "can't get PM domain: %d\n", ret);
        clk_notifier_unregister((*a7cc).pclk, &mut (*a7cc).clk_nb);
        return ret;
    }

    0
}

unsafe extern "C" fn qcom_apcs_sdx55_clk_remove(pdev: *mut platform_device) {
    let cpu_dev: *mut device = get_cpu_device(0);
    let a7cc: *mut clk_regmap_mux_div = platform_get_drvdata(pdev);

    clk_notifier_unregister((*a7cc).pclk, &mut (*a7cc).clk_nb);
    dev_pm_domain_detach(cpu_dev, true);
}

static mut QCOM_APCS_SDX55_CLK_DRIVER: platform_driver = platform_driver {
    probe: Some(qcom_apcs_sdx55_clk_probe),
    remove: Some(qcom_apcs_sdx55_clk_remove),
    driver: device_driver {
        name: "qcom-sdx55-acps-clk",
    },
};

module_platform_driver!(QCOM_APCS_SDX55_CLK_DRIVER);

module_author!("Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>");
module_license!("GPL v2");
module_description!("Qualcomm SDX55 APCS clock driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
