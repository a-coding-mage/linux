// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// C dependencies supplied by the surrounding kernel clock framework are intentionally
// referenced as external types, constants, and functions below.

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
const DT_BI_TCXO_PAD: u32 = 0;

extern "C" {
    static clk_branch2_ops: clk_ops;
    fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

#[repr(C)]
struct clk_ops;

#[repr(C)]
struct clk_init_data {
    name: *const u8,
    parent_data: *const clk_parent_data,
    num_parents: u32,
    ops: *const clk_ops,
}

#[repr(C)]
struct clk_parent_data {
    index: u32,
}

#[repr(C)]
struct clk_hw {
    init: *const clk_init_data,
}

#[repr(C)]
struct clk_regmap {
    enable_reg: u32,
    enable_mask: u32,
    hw: clk_hw,
}

#[repr(C)]
struct clk_branch {
    halt_reg: u32,
    halt_check: u32,
    clkr: clk_regmap,
}

#[repr(C)]
struct regmap_config {
    reg_bits: u32,
    reg_stride: u32,
    val_bits: u32,
    max_register: u32,
    fast_io: bool,
}

#[repr(C)]
struct qcom_cc_desc {
    config: *const regmap_config,
    clks: *const *mut clk_regmap,
    num_clks: usize,
}

#[repr(C)]
struct of_device_id {
    compatible: *const u8,
}

#[repr(C)]
struct platform_device;

#[repr(C)]
struct device_driver {
    name: *const u8,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    driver: device_driver,
}

const BRANCH_HALT_DELAY: u32 = 1;
const TCSR_PCIE_0_CLKREF_EN: usize = 0;
const TCSR_UFS_CLKREF_EN: usize = 1;
const TCSR_USB2_CLKREF_EN: usize = 2;
const TCSR_USB3_CLKREF_EN: usize = 3;

static mut tcsr_pcie_0_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x15044,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x15044,
        enable_mask: 1 << 0,
        hw: clk_hw {
            init: &clk_init_data {
                name: b"tcsr_pcie_0_clkref_en\0".as_ptr(),
                parent_data: core::ptr::null(),
                num_parents: 0,
                ops: unsafe { &clk_branch2_ops },
            },
        },
    },
};

static mut tcsr_usb3_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x1504c,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x1504c,
        enable_mask: 1 << 0,
        hw: clk_hw {
            init: &clk_init_data {
                name: b"tcsr_usb3_clkref_en\0".as_ptr(),
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: unsafe { &clk_branch2_ops },
            },
        },
    },
};

static mut tcsr_ufs_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x15054,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x15054,
        enable_mask: 1 << 0,
        hw: clk_hw {
            init: &clk_init_data {
                name: b"tcsr_ufs_clkref_en\0".as_ptr(),
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: unsafe { &clk_branch2_ops },
            },
        },
    },
};

static mut tcsr_usb2_clkref_en: clk_branch = clk_branch {
    halt_reg: 0x1505c,
    halt_check: BRANCH_HALT_DELAY,
    clkr: clk_regmap {
        enable_reg: 0x1505c,
        enable_mask: 1 << 0,
        hw: clk_hw {
            init: &clk_init_data {
                name: b"tcsr_usb2_clkref_en\0".as_ptr(),
                parent_data: &clk_parent_data { index: DT_BI_TCXO_PAD },
                num_parents: 1,
                ops: unsafe { &clk_branch2_ops },
            },
        },
    },
};

static mut tcsr_cc_kaanapali_clocks: [*mut clk_regmap; 4] = [
    unsafe { &mut tcsr_pcie_0_clkref_en.clkr },
    unsafe { &mut tcsr_ufs_clkref_en.clkr },
    unsafe { &mut tcsr_usb2_clkref_en.clkr },
    unsafe { &mut tcsr_usb3_clkref_en.clkr },
];

static tcsr_cc_kaanapali_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: 0x3d000,
    fast_io: true,
};

static tcsr_cc_kaanapali_desc: qcom_cc_desc = qcom_cc_desc {
    config: &tcsr_cc_kaanapali_regmap_config,
    clks: tcsr_cc_kaanapali_clocks.as_ptr(),
    num_clks: tcsr_cc_kaanapali_clocks.len(),
};

static tcsr_cc_kaanapali_match_table: [of_device_id; 2] = [
    of_device_id { compatible: b"qcom,kaanapali-tcsr\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

unsafe extern "C" fn tcsr_cc_kaanapali_probe(pdev: *mut platform_device) -> i32 {
    qcom_cc_probe(pdev, &tcsr_cc_kaanapali_desc)
}

static mut tcsr_cc_kaanapali_driver: platform_driver = platform_driver {
    probe: Some(tcsr_cc_kaanapali_probe),
    driver: device_driver {
        name: b"tcsr_cc-kaanapali\0".as_ptr(),
        of_match_table: tcsr_cc_kaanapali_match_table.as_ptr(),
    },
};

unsafe extern "C" fn tcsr_cc_kaanapali_init() -> i32 {
    platform_driver_register(&mut tcsr_cc_kaanapali_driver)
}

unsafe extern "C" fn tcsr_cc_kaanapali_exit() {
    platform_driver_unregister(&mut tcsr_cc_kaanapali_driver);
}

// subsys_initcall(tcsr_cc_kaanapali_init);
// module_exit(tcsr_cc_kaanapali_exit);
// MODULE_DESCRIPTION("QTI TCSR_CC Kaanapali Driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
