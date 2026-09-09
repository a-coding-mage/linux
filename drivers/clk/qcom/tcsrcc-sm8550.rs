// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 * Copyright (c) 2022, Qualcomm Innovation Center, Inc. All rights reserved.
 * Copyright (c) 2022, Linaro Limited
 */

// Dependencies supplied by the surrounding kernel clock-driver environment.

#[repr(C)]
pub enum DtBiTcxoPad {
    DT_BI_TCXO_PAD,
}

pub static mut tcsr_pcie_0_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x15100,
    halt_check: BRANCH_HALT_SKIP,
    clkr: clk_regmap {
        enable_reg: 0x15100,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "tcsr_pcie_0_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD as usize },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

pub static mut tcsr_pcie_1_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x15114,
    halt_check: BRANCH_HALT_SKIP,
    clkr: clk_regmap {
        enable_reg: 0x15114,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "tcsr_pcie_1_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD as usize },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

pub static mut tcsr_ufs_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x15110,
    halt_check: BRANCH_HALT_SKIP,
    clkr: clk_regmap {
        enable_reg: 0x15110,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "tcsr_ufs_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD as usize },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

pub static mut tcsr_ufs_pad_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x15104,
    halt_check: BRANCH_HALT_SKIP,
    clkr: clk_regmap {
        enable_reg: 0x15104,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "tcsr_ufs_pad_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD as usize },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

pub static mut tcsr_usb2_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x15118,
    halt_check: BRANCH_HALT_SKIP,
    clkr: clk_regmap {
        enable_reg: 0x15118,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "tcsr_usb2_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD as usize },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

pub static mut tcsr_usb3_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x15108,
    halt_check: BRANCH_HALT_SKIP,
    clkr: clk_regmap {
        enable_reg: 0x15108,
        enable_mask: BIT(0),
        hw: clk_hw {
            init: &clk_init_data {
                name: "tcsr_usb3_clkref_en",
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD as usize },
                num_parents: 1,
                ops: &clk_branch2_ops,
            },
        },
    },
};

pub static mut tcsr_cc_sar2130p_clocks: [*mut clk_regmap; 6] = [
    tcsr_pcie_0_clkref_en.clkr_ptr(), tcsr_pcie_1_clkref_en.clkr_ptr(),
    core::ptr::null_mut(), tcsr_usb2_clkref_en.clkr_ptr(),
    tcsr_usb3_clkref_en.clkr_ptr(), core::ptr::null_mut(),
];

pub static mut tcsr_cc_sm8550_clocks: [*mut clk_regmap; 6] = [
    tcsr_pcie_0_clkref_en.clkr_ptr(), tcsr_pcie_1_clkref_en.clkr_ptr(),
    tcsr_ufs_clkref_en.clkr_ptr(), tcsr_ufs_pad_clkref_en.clkr_ptr(),
    tcsr_usb2_clkref_en.clkr_ptr(), tcsr_usb3_clkref_en.clkr_ptr(),
];

pub static tcsr_cc_sm8550_regmap_config: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x2f000, fast_io: true,
};

pub static tcsr_cc_sar2130p_desc: qcom_cc_desc = qcom_cc_desc {
    config: &tcsr_cc_sm8550_regmap_config, clks: tcsr_cc_sar2130p_clocks.as_ptr(),
    num_clks: tcsr_cc_sar2130p_clocks.len(),
};

pub static tcsr_cc_sm8550_desc: qcom_cc_desc = qcom_cc_desc {
    config: &tcsr_cc_sm8550_regmap_config, clks: tcsr_cc_sm8550_clocks.as_ptr(),
    num_clks: tcsr_cc_sm8550_clocks.len(),
};

pub static tcsr_cc_sm8550_match_table: [of_device_id; 3] = [
    of_device_id { compatible: "qcom,sar2130p-tcsr", data: &tcsr_cc_sar2130p_desc },
    of_device_id { compatible: "qcom,sm8550-tcsr", data: &tcsr_cc_sm8550_desc },
    of_device_id::default(),
];

unsafe extern "C" {
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn of_device_get_match_data(dev: *const device) -> *const qcom_cc_desc;
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

pub unsafe extern "C" fn tcsr_cc_sm8550_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, of_device_get_match_data((*pdev).dev.as_ptr()));
    if IS_ERR(regmap) { return PTR_ERR(regmap); }
    qcom_cc_really_probe((*pdev).dev.as_ptr(), &tcsr_cc_sm8550_desc, regmap)
}

pub static mut tcsr_cc_sm8550_driver: platform_driver = platform_driver {
    probe: Some(tcsr_cc_sm8550_probe),
    driver: driver { name: "tcsr_cc-sm8550", of_match_table: tcsr_cc_sm8550_match_table.as_ptr() },
};

pub unsafe extern "C" fn tcsr_cc_sm8550_init() -> i32 {
    platform_driver_register(&mut tcsr_cc_sm8550_driver)
}

pub unsafe extern "C" fn tcsr_cc_sm8550_exit() {
    platform_driver_unregister(&mut tcsr_cc_sm8550_driver);
}

// subsys_initcall(tcsr_cc_sm8550_init);
// module_exit(tcsr_cc_sm8550_exit);
// MODULE_DESCRIPTION("QTI TCSRCC SM8550 Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
