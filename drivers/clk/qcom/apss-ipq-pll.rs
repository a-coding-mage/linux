// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
//
// Dependencies supplied by the surrounding kernel/Rust bindings:
// linux/clk-provider.h, linux/module.h, linux/of.h, linux/platform_device.h,
// linux/regmap.h, and clk-alpha-pll.h.

static mut ipq_pll_huayra: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_HUAYRA_APSS],
    flags: SUPPORTS_DYNAMIC_UPDATE,
    clkr: clk_regmap {
        enable_reg: 0x0,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "a53pll",
                parent_data: &clk_parent_data { fw_name: "xo" },
                num_parents: 1,
                ops: &clk_alpha_pll_huayra_ops,
            },
        },
    },
};

static mut ipq_pll_stromer: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0,
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_STROMER],
    flags: SUPPORTS_DYNAMIC_UPDATE,
    clkr: clk_regmap {
        enable_reg: 0x0,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "a53pll",
                parent_data: &clk_parent_data { fw_name: "xo" },
                num_parents: 1,
                ops: &clk_alpha_pll_stromer_ops,
            },
        },
    },
};

static mut ipq_pll_stromer_plus: clk_alpha_pll = clk_alpha_pll {
    offset: 0x0,
    // The register offsets of the Stromer Plus PLL used in IPQ5332
    // are the same as the Stromer PLL's offsets.
    regs: clk_alpha_pll_regs[CLK_ALPHA_PLL_TYPE_STROMER],
    flags: SUPPORTS_DYNAMIC_UPDATE,
    clkr: clk_regmap {
        enable_reg: 0x0,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "a53pll",
                parent_data: &clk_parent_data { fw_name: "xo" },
                num_parents: 1,
                ops: &clk_alpha_pll_stromer_plus_ops,
            },
        },
    },
};

// 1.008 GHz configuration
static ipq5018_pll_config: alpha_pll_config = alpha_pll_config {
    l: 0x2a,
    config_ctl_val: 0x4001075b,
    main_output_mask: BIT(0),
    aux_output_mask: BIT(1),
    early_output_mask: BIT(3),
    status_val: 0x3,
    status_mask: GENMASK(10, 8),
    lock_det: BIT(2),
    test_ctl_hi_val: 0x00400003,
};

static ipq5210_pll_config: alpha_pll_config = alpha_pll_config {
    l: 0x22,
    config_ctl_val: 0x4001075b,
    config_ctl_hi_val: 0x6,
    early_output_mask: BIT(3),
    aux2_output_mask: BIT(2),
    aux_output_mask: BIT(1),
    main_output_mask: BIT(0),
    test_ctl_val: 0x0,
    test_ctl_hi_val: 0x400003,
};

// 1.080 GHz configuration
static ipq5332_pll_config: alpha_pll_config = alpha_pll_config {
    l: 0x2d,
    config_ctl_val: 0x4001075b,
    main_output_mask: BIT(0),
    aux_output_mask: BIT(1),
    early_output_mask: BIT(3),
    status_val: 0x3,
    status_mask: GENMASK(10, 8),
    lock_det: BIT(2),
    test_ctl_hi_val: 0x00400003,
};

static ipq6018_pll_config: alpha_pll_config = alpha_pll_config {
    l: 0x37,
    config_ctl_val: 0x240d4828,
    config_ctl_hi_val: 0x6,
    early_output_mask: BIT(3),
    aux2_output_mask: BIT(2),
    aux_output_mask: BIT(1),
    main_output_mask: BIT(0),
    test_ctl_val: 0x1c0000C0,
    test_ctl_hi_val: 0x4000,
};

static ipq8074_pll_config: alpha_pll_config = alpha_pll_config {
    l: 0x48,
    config_ctl_val: 0x200d4828,
    config_ctl_hi_val: 0x6,
    early_output_mask: BIT(3),
    aux2_output_mask: BIT(2),
    aux_output_mask: BIT(1),
    main_output_mask: BIT(0),
    test_ctl_val: 0x1c000000,
    test_ctl_hi_val: 0x4000,
};

static ipq9574_pll_config: alpha_pll_config = alpha_pll_config {
    l: 0x3b,
    config_ctl_val: 0x200d4828,
    config_ctl_hi_val: 0x6,
    early_output_mask: BIT(3),
    aux2_output_mask: BIT(2),
    aux_output_mask: BIT(1),
    main_output_mask: BIT(0),
    test_ctl_val: 0x0,
    test_ctl_hi_val: 0x4000,
};

#[repr(C)]
struct apss_pll_data {
    pll_type: i32,
    pll: *mut clk_alpha_pll,
    pll_config: *const alpha_pll_config,
}

static ipq5018_pll_data: apss_pll_data = apss_pll_data { pll_type: CLK_ALPHA_PLL_TYPE_STROMER, pll: unsafe { &raw mut ipq_pll_stromer }, pll_config: &ipq5018_pll_config };
static ipq5210_pll_data: apss_pll_data = apss_pll_data { pll_type: CLK_ALPHA_PLL_TYPE_HUAYRA, pll: unsafe { &raw mut ipq_pll_huayra }, pll_config: &ipq5210_pll_config };
static ipq5332_pll_data: apss_pll_data = apss_pll_data { pll_type: CLK_ALPHA_PLL_TYPE_STROMER_PLUS, pll: unsafe { &raw mut ipq_pll_stromer_plus }, pll_config: &ipq5332_pll_config };
static ipq8074_pll_data: apss_pll_data = apss_pll_data { pll_type: CLK_ALPHA_PLL_TYPE_HUAYRA, pll: unsafe { &raw mut ipq_pll_huayra }, pll_config: &ipq8074_pll_config };
static ipq6018_pll_data: apss_pll_data = apss_pll_data { pll_type: CLK_ALPHA_PLL_TYPE_HUAYRA, pll: unsafe { &raw mut ipq_pll_huayra }, pll_config: &ipq6018_pll_config };
static ipq9574_pll_data: apss_pll_data = apss_pll_data { pll_type: CLK_ALPHA_PLL_TYPE_HUAYRA, pll: unsafe { &raw mut ipq_pll_huayra }, pll_config: &ipq9574_pll_config };

static ipq_pll_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0x40,
};

unsafe fn apss_ipq_pll_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) { return PTR_ERR(base); }
    let regmap = devm_regmap_init_mmio(dev, base, &ipq_pll_regmap_config);
    if IS_ERR(regmap) { return PTR_ERR(regmap); }
    let data = of_device_get_match_data(dev);
    if data.is_null() { return -ENODEV; }
    if (*data).pll_type == CLK_ALPHA_PLL_TYPE_HUAYRA {
        clk_alpha_pll_configure((*data).pll, regmap, (*data).pll_config);
    } else if (*data).pll_type == CLK_ALPHA_PLL_TYPE_STROMER || (*data).pll_type == CLK_ALPHA_PLL_TYPE_STROMER_PLUS {
        clk_stromer_pll_configure((*data).pll, regmap, (*data).pll_config);
    }
    let ret = devm_clk_register_regmap(dev, &mut (*(*data).pll).clkr);
    if ret != 0 { return ret; }
    devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, &mut (*(*data).pll).clkr.hw)
}

static apss_ipq_pll_match_table: [of_device_id; 7] = [
    of_device_id { compatible: "qcom,ipq5018-a53pll", data: &ipq5018_pll_data },
    of_device_id { compatible: "qcom,ipq5210-a53pll", data: &ipq5210_pll_data },
    of_device_id { compatible: "qcom,ipq5332-a53pll", data: &ipq5332_pll_data },
    of_device_id { compatible: "qcom,ipq6018-a53pll", data: &ipq6018_pll_data },
    of_device_id { compatible: "qcom,ipq8074-a53pll", data: &ipq8074_pll_data },
    of_device_id { compatible: "qcom,ipq9574-a73pll", data: &ipq9574_pll_data },
    of_device_id { ..Default::default() },
];

static mut apss_ipq_pll_driver: platform_driver = platform_driver {
    probe: Some(apss_ipq_pll_probe),
    driver: device_driver {
        name: "qcom-ipq-apss-pll",
        of_match_table: apss_ipq_pll_match_table.as_ptr(),
    },
};

module_platform_driver!(apss_ipq_pll_driver);
module_description!("Qualcomm technology Inc APSS ALPHA PLL Driver");
module_license!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
