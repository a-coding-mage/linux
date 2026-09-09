// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022, Qualcomm Innovation Center, Inc. All rights reserved.
 * Copyright (c) 2023, Linaro Limited
 */

// Translated from the Linux Qualcomm X1E80100 TCSR clock controller driver.

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
mod translation {
    use core::ffi::{c_char, c_int, c_void};

    pub const DT_BI_TCXO_PAD: usize = 0;
    pub const BRANCH_HALT_DELAY: u32 = 1;

    #[repr(C)]
    pub struct clk_parent_data { pub index: usize }
    #[repr(C)]
    pub struct clk_init_data {
        pub name: *const c_char,
        pub parent_data: *const clk_parent_data,
        pub num_parents: usize,
        pub ops: *const c_void,
    }
    #[repr(C)]
    pub struct clk_regmap { pub _private: [u8; 0] }
    #[repr(C)]
    pub struct clk_branch {
        pub halt_reg: u32,
        pub halt_check: u32,
        pub clkr: clk_regmap_data,
    }
    #[repr(C)]
    pub struct clk_regmap_data {
        pub enable_reg: u32,
        pub enable_mask: u32,
        pub init: *const clk_init_data,
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
        pub clks: *const *mut clk_regmap,
        pub num_clks: usize,
    }
    #[repr(C)]
    pub struct of_device_id { pub compatible: *const c_char }
    #[repr(C)]
    pub struct platform_device { pub _private: [u8; 0] }
    #[repr(C)]
    pub struct platform_driver {
        pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
        pub name: *const c_char,
        pub of_match_table: *const of_device_id,
    }

    unsafe extern "C" {
        pub static clk_branch2_ops: c_void;
        pub fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> c_int;
        pub fn platform_driver_register(driver: *mut platform_driver) -> c_int;
        pub fn platform_driver_unregister(driver: *mut platform_driver);
    }

    macro_rules! branch {
        ($name:ident, $reg:expr, $text:literal) => {
            static mut $name: clk_branch = clk_branch {
                halt_reg: $reg,
                halt_check: BRANCH_HALT_DELAY,
                clkr: clk_regmap_data {
                    enable_reg: $reg,
                    enable_mask: 1u32 << 0,
                    init: &clk_init_data {
                        name: concat!($text, "\0").as_ptr() as *const c_char,
                        parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                        num_parents: 1,
                        ops: unsafe { &clk_branch2_ops as *const c_void },
                    },
                },
            };
        };
    }

    branch!(tcsr_edp_clkref_en, 0x15130, "tcsr_edp_clkref_en");
    branch!(tcsr_pcie_2l_4_clkref_en, 0x15100, "tcsr_pcie_2l_4_clkref_en");
    branch!(tcsr_pcie_2l_5_clkref_en, 0x15104, "tcsr_pcie_2l_5_clkref_en");
    branch!(tcsr_pcie_8l_clkref_en, 0x15108, "tcsr_pcie_8l_clkref_en");
    branch!(tcsr_usb3_mp0_clkref_en, 0x1510c, "tcsr_usb3_mp0_clkref_en");
    branch!(tcsr_usb3_mp1_clkref_en, 0x15110, "tcsr_usb3_mp1_clkref_en");
    branch!(tcsr_usb2_1_clkref_en, 0x15114, "tcsr_usb2_1_clkref_en");
    branch!(tcsr_ufs_phy_clkref_en, 0x15118, "tcsr_ufs_phy_clkref_en");
    branch!(tcsr_usb4_1_clkref_en, 0x15120, "tcsr_usb4_1_clkref_en");
    branch!(tcsr_usb4_2_clkref_en, 0x15124, "tcsr_usb4_2_clkref_en");
    branch!(tcsr_usb2_2_clkref_en, 0x15128, "tcsr_usb2_2_clkref_en");
    branch!(tcsr_pcie_4l_clkref_en, 0x1512c, "tcsr_pcie_4l_clkref_en");

    // The TCSR_* indices are supplied by dt-bindings/clock/qcom,x1e80100-tcsr.h.
    unsafe extern "C" {
        pub static TCSR_EDP_CLKREF_EN: usize;
        pub static TCSR_PCIE_2L_4_CLKREF_EN: usize;
        pub static TCSR_PCIE_2L_5_CLKREF_EN: usize;
        pub static TCSR_PCIE_8L_CLKREF_EN: usize;
        pub static TCSR_USB3_MP0_CLKREF_EN: usize;
        pub static TCSR_USB3_MP1_CLKREF_EN: usize;
        pub static TCSR_USB2_1_CLKREF_EN: usize;
        pub static TCSR_UFS_PHY_CLKREF_EN: usize;
        pub static TCSR_USB4_1_CLKREF_EN: usize;
        pub static TCSR_USB4_2_CLKREF_EN: usize;
        pub static TCSR_USB2_2_CLKREF_EN: usize;
        pub static TCSR_PCIE_4L_CLKREF_EN: usize;
    }

    static mut tcsr_cc_x1e80100_clocks: [*mut clk_regmap; 12] = [
        unsafe { &mut tcsr_edp_clkref_en.clkr as *mut _ as *mut clk_regmap },
        unsafe { &mut tcsr_pcie_2l_4_clkref_en.clkr as *mut _ as *mut clk_regmap },
        unsafe { &mut tcsr_pcie_2l_5_clkref_en.clkr as *mut _ as *mut clk_regmap },
        unsafe { &mut tcsr_pcie_8l_clkref_en.clkr as *mut _ as *mut clk_regmap },
        unsafe { &mut tcsr_usb3_mp0_clkref_en.clkr as *mut _ as *mut clk_regmap },
        unsafe { &mut tcsr_usb3_mp1_clkref_en.clkr as *mut _ as *mut clk_regmap },
        unsafe { &mut tcsr_usb2_1_clkref_en.clkr as *mut _ as *mut clk_regmap },
        unsafe { &mut tcsr_ufs_phy_clkref_en.clkr as *mut _ as *mut clk_regmap },
        unsafe { &mut tcsr_usb4_1_clkref_en.clkr as *mut _ as *mut clk_regmap },
        unsafe { &mut tcsr_usb4_2_clkref_en.clkr as *mut _ as *mut clk_regmap },
        unsafe { &mut tcsr_usb2_2_clkref_en.clkr as *mut _ as *mut clk_regmap },
        unsafe { &mut tcsr_pcie_4l_clkref_en.clkr as *mut _ as *mut clk_regmap },
    ];

    static tcsr_cc_x1e80100_regmap_config: regmap_config = regmap_config {
        reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x2f000, fast_io: true,
    };
    static tcsr_cc_x1e80100_desc: qcom_cc_desc = qcom_cc_desc {
        config: &tcsr_cc_x1e80100_regmap_config,
        clks: unsafe { tcsr_cc_x1e80100_clocks.as_ptr() },
        num_clks: 12,
    };
    static tcsr_cc_x1e80100_match_table: [of_device_id; 2] = [
        of_device_id { compatible: b"qcom,x1e80100-tcsr\0".as_ptr() as *const c_char },
        of_device_id { compatible: core::ptr::null() },
    ];

    unsafe extern "C" fn tcsr_cc_x1e80100_probe(pdev: *mut platform_device) -> c_int {
        qcom_cc_probe(pdev, &tcsr_cc_x1e80100_desc)
    }
    static mut tcsr_cc_x1e80100_driver: platform_driver = platform_driver {
        probe: Some(tcsr_cc_x1e80100_probe),
        name: b"tcsrcc-x1e80100\0".as_ptr() as *const c_char,
        of_match_table: tcsr_cc_x1e80100_match_table.as_ptr(),
    };

    unsafe extern "C" fn tcsr_cc_x1e80100_init() -> c_int {
        platform_driver_register(&mut tcsr_cc_x1e80100_driver)
    }
    unsafe extern "C" fn tcsr_cc_x1e80100_exit() {
        platform_driver_unregister(&mut tcsr_cc_x1e80100_driver)
    }

    // C registration annotations: subsys_initcall(tcsr_cc_x1e80100_init), module_exit(...).
    // MODULE_DEVICE_TABLE(of, ...), MODULE_DESCRIPTION, and MODULE_LICENSE are metadata.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
