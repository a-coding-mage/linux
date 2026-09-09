// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019, Linaro Ltd.
 */

// Linux kernel headers and the qcom clock-controller headers are external
// dependencies of this translation.

use core::ffi::{c_char, c_int, c_void};

// External types, operations, constants, and functions supplied by the kernel
// clock framework and the associated qcom clock-controller sources.
#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const c_void,
}

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_regmap {
    pub hw: clk_hw,
    pub enable_reg: u32,
    pub enable_mask: u32,
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
    pub clks: *const *mut clk_regmap,
    pub num_clks: usize,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: driver,
}

#[repr(C)]
pub struct driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

extern "C" {
    static clk_branch2_aon_ops: c_void;
    static clk_branch2_ops: c_void;
    fn devm_pm_runtime_enable(dev: *mut device) -> c_int;
    fn devm_pm_clk_create(dev: *mut device) -> c_int;
    fn pm_clk_add(dev: *mut device, name: *const c_char) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> c_int;
    fn pm_runtime_put(dev: *mut device);
    fn pm_runtime_put_sync(dev: *mut device);
    fn pm_clk_suspend(dev: *mut device) -> c_int;
    fn pm_clk_resume(dev: *mut device) -> c_int;
}

const BRANCH_HALT: u32 = 0;
const TURING_WRAPPER_AON_CLK: usize = 0;
const TURING_Q6SS_AHBM_AON_CLK: usize = 1;
const TURING_Q6SS_Q6_AXIM_CLK: usize = 2;
const TURING_Q6SS_AHBS_AON_CLK: usize = 3;
const TURING_WRAPPER_QOS_AHBS_AON_CLK: usize = 4;

static mut turing_wrapper_aon_cbcr: clk_branch = clk_branch {
    halt_reg: 0x5098,
    halt_check: BRANCH_HALT,
    clkr: clk_regmap { hw: clk_hw { init: &turing_wrapper_aon_init }, enable_reg: 0x5098, enable_mask: 1 << 0 },
};
static turing_wrapper_aon_init: clk_init_data = clk_init_data { name: b"turing_wrapper_aon_clk\0".as_ptr() as *const c_char, ops: unsafe { &clk_branch2_aon_ops } };

static mut turing_q6ss_ahbm_aon_cbcr: clk_branch = clk_branch {
    halt_reg: 0x9000,
    halt_check: BRANCH_HALT,
    clkr: clk_regmap { hw: clk_hw { init: &turing_q6ss_ahbm_aon_init }, enable_reg: 0x9000, enable_mask: 1 << 0 },
};
static turing_q6ss_ahbm_aon_init: clk_init_data = clk_init_data { name: b"turing_q6ss_ahbm_aon_cbcr\0".as_ptr() as *const c_char, ops: unsafe { &clk_branch2_ops } };

static mut turing_q6ss_q6_axim_clk: clk_branch = clk_branch {
    halt_reg: 0xb000,
    halt_check: BRANCH_HALT,
    clkr: clk_regmap { hw: clk_hw { init: &turing_q6ss_q6_axim_init }, enable_reg: 0xb000, enable_mask: 1 << 0 },
};
static turing_q6ss_q6_axim_init: clk_init_data = clk_init_data { name: b"turing_q6ss_q6_axim_clk\0".as_ptr() as *const c_char, ops: unsafe { &clk_branch2_aon_ops } };

static mut turing_q6ss_ahbs_aon_cbcr: clk_branch = clk_branch {
    halt_reg: 0x10000,
    halt_check: BRANCH_HALT,
    clkr: clk_regmap { hw: clk_hw { init: &turing_q6ss_ahbs_aon_init }, enable_reg: 0x10000, enable_mask: 1 << 0 },
};
static turing_q6ss_ahbs_aon_init: clk_init_data = clk_init_data { name: b"turing_q6ss_ahbs_aon_clk\0".as_ptr() as *const c_char, ops: unsafe { &clk_branch2_aon_ops } };

static mut turing_wrapper_qos_ahbs_aon_cbcr: clk_branch = clk_branch {
    halt_reg: 0x11014,
    halt_check: BRANCH_HALT,
    clkr: clk_regmap { hw: clk_hw { init: &turing_wrapper_qos_ahbs_aon_init }, enable_reg: 0x11014, enable_mask: 1 << 0 },
};
static turing_wrapper_qos_ahbs_aon_init: clk_init_data = clk_init_data { name: b"turing_wrapper_qos_ahbs_aon_clk\0".as_ptr() as *const c_char, ops: unsafe { &clk_branch2_aon_ops } };

static mut turingcc_clocks: [*mut clk_regmap; 5] = [
    unsafe { &raw mut turing_wrapper_aon_cbcr.clkr },
    unsafe { &raw mut turing_q6ss_ahbm_aon_cbcr.clkr },
    unsafe { &raw mut turing_q6ss_q6_axim_clk.clkr },
    unsafe { &raw mut turing_q6ss_ahbs_aon_cbcr.clkr },
    unsafe { &raw mut turing_wrapper_qos_ahbs_aon_cbcr.clkr },
];

static turingcc_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x23004, fast_io: true };
static turingcc_desc: qcom_cc_desc = qcom_cc_desc { config: &turingcc_regmap_config, clks: turingcc_clocks.as_ptr(), num_clks: turingcc_clocks.len() };

unsafe extern "C" fn turingcc_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let mut ret = devm_pm_runtime_enable(dev);
    if ret != 0 { return ret; }
    ret = devm_pm_clk_create(dev);
    if ret != 0 { return ret; }
    ret = pm_clk_add(dev, core::ptr::null());
    if ret < 0 { return ret; }
    ret = pm_runtime_resume_and_get(dev);
    if ret != 0 { return ret; }
    ret = qcom_cc_probe(pdev, &turingcc_desc);
    if ret < 0 { pm_runtime_put_sync(dev); return ret; }
    pm_runtime_put(dev);
    0
}

static turingcc_pm_ops: dev_pm_ops = dev_pm_ops { suspend: Some(pm_clk_suspend), resume: Some(pm_clk_resume) };
static turingcc_match_table: [of_device_id; 2] = [
    of_device_id { compatible: b"qcom,qcs404-turingcc\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];
static mut turingcc_driver: platform_driver = platform_driver {
    probe: Some(turingcc_probe),
    driver: driver { name: b"qcs404-turingcc\0".as_ptr() as *const c_char, of_match_table: turingcc_match_table.as_ptr(), pm: &turingcc_pm_ops },
};

// module_platform_driver(turingcc_driver);
// MODULE_DEVICE_TABLE(of, turingcc_match_table);
// MODULE_DESCRIPTION("Qualcomm QCS404 Turing Clock Controller");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
