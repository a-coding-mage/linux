// SPDX-License-Identifier: GPL-2.0-only
/*
 * Clkout driver for Rockchip RK808
 *
 * Copyright (c) 2014, Fuzhou Rockchip Electronics Co., Ltd
 *
 * Author:Chris Zhong <zyw@rock-chips.com>
 */

// External Linux kernel declarations and constants are supplied by other files.

#[repr(C)]
pub struct rk808_clkout {
    pub regmap: *mut regmap,
    pub clkout1_hw: clk_hw,
    pub clkout2_hw: clk_hw,
}

unsafe fn rk808_clkout_recalc_rate(_hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    32768
}

unsafe fn rk808_clkout2_enable(hw: *mut clk_hw, enable: bool) -> c_int {
    let rk808_clkout = container_of!(hw, rk808_clkout, clkout2_hw);
    regmap_update_bits(
        (*rk808_clkout).regmap,
        RK808_CLK32OUT_REG,
        CLK32KOUT2_EN,
        if enable { CLK32KOUT2_EN } else { 0 },
    )
}

unsafe fn rk808_clkout2_prepare(hw: *mut clk_hw) -> c_int {
    rk808_clkout2_enable(hw, true)
}

unsafe fn rk808_clkout2_unprepare(hw: *mut clk_hw) {
    rk808_clkout2_enable(hw, false);
}

unsafe fn rk808_clkout2_is_prepared(hw: *mut clk_hw) -> c_int {
    let rk808_clkout = container_of!(hw, rk808_clkout, clkout2_hw);
    let mut val: u32 = 0;

    let ret = regmap_read((*rk808_clkout).regmap, RK808_CLK32OUT_REG, &mut val);

    if ret < 0 {
        return ret;
    }

    if val & CLK32KOUT2_EN != 0 { 1 } else { 0 }
}

static rk808_clkout1_ops: clk_ops = clk_ops {
    recalc_rate: Some(rk808_clkout_recalc_rate),
};

static rk808_clkout2_ops: clk_ops = clk_ops {
    prepare: Some(rk808_clkout2_prepare),
    unprepare: Some(rk808_clkout2_unprepare),
    is_prepared: Some(rk808_clkout2_is_prepared),
    recalc_rate: Some(rk808_clkout_recalc_rate),
};

unsafe fn of_clk_rk808_get(
    clkspec: *mut of_phandle_args,
    data: *mut c_void,
) -> *mut clk_hw {
    let rk808_clkout = data as *mut rk808_clkout;
    let idx = (*clkspec).args[0];

    if idx >= 2 {
        pr_err!("%s: invalid index %u\n", "of_clk_rk808_get", idx);
        return ERR_PTR(-EINVAL);
    }

    if idx != 0 {
        &mut (*rk808_clkout).clkout2_hw
    } else {
        &mut (*rk808_clkout).clkout1_hw
    }
}

unsafe fn rk817_clkout2_enable(hw: *mut clk_hw, enable: bool) -> c_int {
    let rk808_clkout = container_of!(hw, rk808_clkout, clkout2_hw);
    regmap_update_bits(
        (*rk808_clkout).regmap,
        RK817_SYS_CFG(1),
        RK817_CLK32KOUT2_EN,
        if enable { RK817_CLK32KOUT2_EN } else { 0 },
    )
}

unsafe fn rk817_clkout2_prepare(hw: *mut clk_hw) -> c_int {
    rk817_clkout2_enable(hw, true)
}

unsafe fn rk817_clkout2_unprepare(hw: *mut clk_hw) {
    rk817_clkout2_enable(hw, false);
}

unsafe fn rk817_clkout2_is_prepared(hw: *mut clk_hw) -> c_int {
    let rk808_clkout = container_of!(hw, rk808_clkout, clkout2_hw);
    let mut val: c_uint = 0;

    let ret = regmap_read((*rk808_clkout).regmap, RK817_SYS_CFG(1), &mut val);

    if ret < 0 {
        return 0;
    }

    if val & RK817_CLK32KOUT2_EN != 0 { 1 } else { 0 }
}

static rk817_clkout2_ops: clk_ops = clk_ops {
    prepare: Some(rk817_clkout2_prepare),
    unprepare: Some(rk817_clkout2_unprepare),
    is_prepared: Some(rk817_clkout2_is_prepared),
    recalc_rate: Some(rk808_clkout_recalc_rate),
};

unsafe fn rkpmic_get_ops(variant: c_long) -> *const clk_ops {
    match variant {
        RK809_ID | RK817_ID => &rk817_clkout2_ops,
        /*
         * For the default case, it match the following PMIC type.
         * RK805_ID
         * RK808_ID
         * RK818_ID
         */
        _ => &rk808_clkout2_ops,
    }
}

unsafe fn rk808_clkout_probe(pdev: *mut platform_device) -> c_int {
    let rk808 = dev_get_drvdata((*pdev).dev.parent) as *mut rk808;
    let dev = &mut (*pdev).dev;
    let mut init: clk_init_data = core::mem::zeroed();
    let rk808_clkout: *mut rk808_clkout;
    let ret: c_int;

    device_set_of_node_from_dev(dev, (*dev).parent);

    rk808_clkout = devm_kzalloc(dev, core::mem::size_of::<rk808_clkout>(), GFP_KERNEL)
        as *mut rk808_clkout;
    if rk808_clkout.is_null() {
        return -ENOMEM;
    }

    (*rk808_clkout).regmap = dev_get_regmap((*pdev).dev.parent, core::ptr::null());
    if (*rk808_clkout).regmap.is_null() {
        return -ENODEV;
    }

    init.parent_names = core::ptr::null();
    init.num_parents = 0;
    init.name = c"rk808-clkout1".as_ptr();
    init.ops = &rk808_clkout1_ops;
    (*rk808_clkout).clkout1_hw.init = &init;

    /* optional override of the clockname */
    of_property_read_string_index(dev.of_node, c"clock-output-names".as_ptr(), 0, &mut init.name);

    ret = devm_clk_hw_register(dev, &mut (*rk808_clkout).clkout1_hw);
    if ret != 0 {
        return ret;
    }

    init.name = c"rk808-clkout2".as_ptr();
    init.ops = rkpmic_get_ops((*rk808).variant);
    (*rk808_clkout).clkout2_hw.init = &init;

    /* optional override of the clockname */
    of_property_read_string_index(dev.of_node, c"clock-output-names".as_ptr(), 1, &mut init.name);

    ret = devm_clk_hw_register(dev, &mut (*rk808_clkout).clkout2_hw);
    if ret != 0 {
        return ret;
    }

    devm_of_clk_add_hw_provider(pdev, Some(of_clk_rk808_get), rk808_clkout as *mut c_void)
}

static mut rk808_clkout_driver: platform_driver = platform_driver {
    probe: Some(rk808_clkout_probe),
    driver: driver {
        name: c"rk808-clkout".as_ptr(),
    },
};

module_platform_driver!(rk808_clkout_driver);

// MODULE_DESCRIPTION("Clkout driver for the rk808 series PMICs");
// MODULE_AUTHOR("Chris Zhong <zyw@rock-chips.com>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:rk808-clkout");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
