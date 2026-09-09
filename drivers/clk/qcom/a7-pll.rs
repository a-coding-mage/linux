// SPDX-License-Identifier: GPL-2.0
/*
 * Qualcomm A7 PLL driver
 *
 * Copyright (c) 2020, Linaro Limited
 * Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
 */

// External Linux clock, platform-device, regmap, and alpha-PLL dependencies
// supplied by the surrounding translation unit.

const LUCID_PLL_OFF_L_VAL: u32 = 0x04;

static lucid_vco: [pll_vco; 1] = [
    pll_vco {
        min_freq: 249600000,
        max_freq: 2000000000,
        val: 0,
    },
];

static mut a7pll: clk_alpha_pll = clk_alpha_pll {
    offset: 0x100,
    vco_table: lucid_vco.as_ptr(),
    num_vco: lucid_vco.len(),
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_LUCID],
    clkr: clk_regmap {
        hw: clk_hw {
            init: &clk_init_data {
                name: "a7pll",
                parent_data: &clk_parent_data {
                    fw_name: "bi_tcxo",
                },
                num_parents: 1,
                ops: &clk_alpha_pll_lucid_ops,
            },
        },
    },
};

static a7pll_config: alpha_pll_config = alpha_pll_config {
    l: 0x39,
    config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x2261,
    config_ctl_hi1_val: 0x029A699C,
    user_ctl_val: 0x1,
    user_ctl_hi_val: 0x805,
};

static a7pll_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0x1000,
};

unsafe fn qcom_a7pll_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = &mut (*pdev).dev;
    let mut regmap: *mut regmap;
    let mut base: *mut core::ffi::c_void;
    let mut l_val: u32 = 0;
    let mut ret: i32;

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    regmap = devm_regmap_init_mmio(dev, base, &a7pll_regmap_config);
    if IS_ERR(regmap) {
        return PTR_ERR(regmap);
    }

    /* Configure PLL only if the l_val is zero */
    regmap_read(regmap, a7pll.offset + LUCID_PLL_OFF_L_VAL, &mut l_val);
    if l_val == 0 {
        clk_lucid_pll_configure(&mut a7pll, regmap, &a7pll_config);
    }

    ret = devm_clk_register_regmap(dev, &mut a7pll.clkr);
    if ret != 0 {
        return ret;
    }

    devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, &mut a7pll.clkr.hw)
}

static qcom_a7pll_match_table: [of_device_id; 2] = [
    of_device_id {
        compatible: "qcom,sdx55-a7pll",
    },
    of_device_id {},
];

// MODULE_DEVICE_TABLE(of, qcom_a7pll_match_table);

static mut qcom_a7pll_driver: platform_driver = platform_driver {
    probe: Some(qcom_a7pll_probe),
    driver: device_driver {
        name: "qcom-a7pll",
        of_match_table: qcom_a7pll_match_table.as_ptr(),
    },
};

// module_platform_driver(qcom_a7pll_driver);
// MODULE_DESCRIPTION("Qualcomm A7 PLL Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
