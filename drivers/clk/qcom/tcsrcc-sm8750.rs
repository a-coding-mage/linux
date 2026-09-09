// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/*
 * Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// Translated from the Linux Qualcomm SM8750 TCSR clock controller driver.

const DT_BI_TCXO_PAD: usize = 0;
const BRANCH_HALT_DELAY: u32 = 0;

extern "C" {
    static clk_branch2_ops: clk_ops;
    fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

#[repr(C)]
pub struct clk_ops {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const u8,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u32,
    pub ops: *const clk_ops,
}

#[repr(C)]
pub struct clk_parent_data {
    pub index: usize,
}

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_regmap {
    pub enable_reg: u32,
    pub enable_mask: u32,
    pub hw: clk_hw,
}

#[repr(C)]
pub struct clk_branch {
    pub halt_reg: u32,
    pub halt_check: u32,
    pub clkr: clk_regmap,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: u32,
    pub reg_stride: u32,
    pub val_bits: u32,
    pub max_register: u32,
    pub fast_io: bool,
}

#[repr(C)]
pub struct qcom_cc_desc {
    pub config: *const regmap_config,
    pub clks: *mut *mut clk_regmap,
    pub num_clks: usize,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const u8,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct platform_device {
    _opaque: [u8; 0],
}

#[repr(C)]
static mut tcsr_pcie_0_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x0,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x0,
        enable_mask: 1u32 << 0,
        hw: clk_hw {
            init: &tcsr_pcie_0_clkref_en_init,
        },
    },
};

static tcsr_pcie_0_clkref_en_init: clk_init_data = clk_init_data {
    name: b"tcsr_pcie_0_clkref_en\0".as_ptr(),
    parent_data: core::ptr::null(),
    num_parents: 0,
    ops: unsafe { &clk_branch2_ops },
};

static tcsr_ufs_clkref_en_parent: clk_parent_data = clk_parent_data { index: DT_BI_TCXO_PAD };
static tcsr_ufs_clkref_en_init: clk_init_data = clk_init_data {
    name: b"tcsr_ufs_clkref_en\0".as_ptr(), parent_data: &tcsr_ufs_clkref_en_parent,
    num_parents: 1, ops: unsafe { &clk_branch2_ops },
};
static mut tcsr_ufs_clkref_en: clk_branch = clk_branch { halt_reg: 0x1000, halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap { enable_reg: 0x1000, enable_mask: 1, hw: clk_hw { init: &tcsr_ufs_clkref_en_init } } };

static tcsr_usb2_clkref_en_parent: clk_parent_data = clk_parent_data { index: DT_BI_TCXO_PAD };
static tcsr_usb2_clkref_en_init: clk_init_data = clk_init_data {
    name: b"tcsr_usb2_clkref_en\0".as_ptr(), parent_data: &tcsr_usb2_clkref_en_parent,
    num_parents: 1, ops: unsafe { &clk_branch2_ops },
};
static mut tcsr_usb2_clkref_en: clk_branch = clk_branch { halt_reg: 0x2000, halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap { enable_reg: 0x2000, enable_mask: 1, hw: clk_hw { init: &tcsr_usb2_clkref_en_init } } };

static tcsr_usb3_clkref_en_parent: clk_parent_data = clk_parent_data { index: DT_BI_TCXO_PAD };
static tcsr_usb3_clkref_en_init: clk_init_data = clk_init_data {
    name: b"tcsr_usb3_clkref_en\0".as_ptr(), parent_data: &tcsr_usb3_clkref_en_parent,
    num_parents: 1, ops: unsafe { &clk_branch2_ops },
};
static mut tcsr_usb3_clkref_en: clk_branch = clk_branch { halt_reg: 0x3000, halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap { enable_reg: 0x3000, enable_mask: 1, hw: clk_hw { init: &tcsr_usb3_clkref_en_init } } };

extern "C" {
    static TCSR_PCIE_0_CLKREF_EN: usize;
    static TCSR_UFS_CLKREF_EN: usize;
    static TCSR_USB2_CLKREF_EN: usize;
    static TCSR_USB3_CLKREF_EN: usize;
}

static mut tcsr_cc_sm8750_clocks: [*mut clk_regmap; 4] = [
    unsafe { &mut tcsr_pcie_0_clkref_en.clkr }, unsafe { &mut tcsr_ufs_clkref_en.clkr },
    unsafe { &mut tcsr_usb2_clkref_en.clkr }, unsafe { &mut tcsr_usb3_clkref_en.clkr },
];

static tcsr_cc_sm8750_regmap_config: regmap_config = regmap_config {
    reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x3000, fast_io: true,
};

static tcsr_cc_sm8750_desc: qcom_cc_desc = qcom_cc_desc {
    config: &tcsr_cc_sm8750_regmap_config,
    clks: unsafe { tcsr_cc_sm8750_clocks.as_mut_ptr() },
    num_clks: 4,
};

static tcsr_cc_sm8750_match_table: [of_device_id; 2] = [
    of_device_id { compatible: b"qcom,sm8750-tcsr\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

unsafe extern "C" fn tcsr_cc_sm8750_probe(pdev: *mut platform_device) -> i32 {
    qcom_cc_probe(pdev, &tcsr_cc_sm8750_desc)
}

static mut tcsr_cc_sm8750_driver: platform_driver = platform_driver {
    probe: Some(tcsr_cc_sm8750_probe),
    driver: device_driver { name: b"tcsr_cc-sm8750\0".as_ptr(), of_match_table: tcsr_cc_sm8750_match_table.as_ptr() },
};

unsafe extern "C" fn tcsr_cc_sm8750_init() -> i32 {
    platform_driver_register(&mut tcsr_cc_sm8750_driver)
}

unsafe extern "C" fn tcsr_cc_sm8750_exit() {
    platform_driver_unregister(&mut tcsr_cc_sm8750_driver);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
