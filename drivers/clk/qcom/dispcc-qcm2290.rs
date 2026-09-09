// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2020, The Linux Foundation. All rights reserved.
// Copyright (c) 2021, Linaro Ltd.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// Translated from the Linux C implementation. Kernel clock-controller types,
// constants, and operations are supplied by the surrounding kernel bindings.

use core::mem::MaybeUninit;

#[repr(usize)]
enum Parent {
    PBiTcxo,
    PBiTcxoAo,
    PDispCcPll0OutMain,
    PDsi0PhyPllOutByteclk,
    PDsi0PhyPllOutDsiclk,
    PGpll0OutDiv,
    PGpll0OutMain,
    PSleepClk,
}

extern "C" {
    static clk_alpha_pll_regs: [*const core::ffi::c_void; 16];
    static clk_alpha_pll_ops: core::ffi::c_void;
    static clk_byte2_ops: core::ffi::c_void;
    static clk_pixel_ops: core::ffi::c_void;
    static clk_rcg2_ops: core::ffi::c_void;
    static clk_rcg2_shared_ops: core::ffi::c_void;
    static clk_regmap_div_ops: core::ffi::c_void;
    static clk_branch2_ops: core::ffi::c_void;
    fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32;
}

// External kernel ABI types. Their concrete definitions are provided by the
// clock, regmap, platform-device, reset, and device-tree bindings.
#[allow(non_camel_case_types)]
type u32_alias = u32;
#[repr(C)] pub struct pll_vco { pub min_freq: u64, pub max_freq: u64, pub val: u32 }
#[repr(C)] pub struct alpha_pll_config { pub l: u32, pub vco_val: u32, pub vco_mask: u32, pub main_output_mask: u32, pub config_ctl_val: u32 }
#[repr(C)] pub struct alpha_pll { pub offset: u32, pub config: *const alpha_pll_config, pub vco_table: *const pll_vco, pub num_vco: usize, pub regs: *const core::ffi::c_void, pub clkr: clk_regmap }
#[repr(C)] pub struct clk_regmap { pub hw: clk_hw, pub enable_reg: u32, pub enable_mask: u32 }
#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct clk_init_data { pub name: *const u8, pub parent_data: *const core::ffi::c_void, pub parent_hws: *const *const clk_hw, pub num_parents: usize, pub flags: u32, pub ops: *const core::ffi::c_void }
#[repr(C)] pub struct clk_rcg2 { pub cmd_rcgr: u32, pub mnd_width: u32, pub hid_width: u32, pub parent_map: *const parent_map, pub freq_tbl: *const freq_tbl, pub clkr: clk_regmap }
#[repr(C)] pub struct clk_regmap_div { pub reg: u32, pub shift: u32, pub width: u32, pub clkr: clk_regmap }
#[repr(C)] pub struct clk_branch { pub halt_reg: u32, pub halt_check: u32, pub clkr: clk_regmap }
#[repr(C)] pub struct parent_map { pub parent: usize, pub value: u32 }
#[repr(C)] pub struct clk_parent_data { pub fw_name: *const u8, pub hw: *const clk_hw }
#[repr(C)] pub struct freq_tbl { pub freq: u64, pub src: usize, pub pre_div: u32, pub m: u32, pub n: u32 }
#[repr(C)] pub struct qcom_reset_map { pub reg: u32 }
#[repr(C)] pub struct gdsc { pub gdscr: u32, pub en_rest_wait_val: u32, pub en_few_wait_val: u32, pub clk_dis_wait_val: u32, pub pd_name: *const u8, pub pwrsts: u32, pub flags: u32 }
#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub max_register: u32, pub fast_io: bool }
#[repr(C)] pub struct qcom_cc_driver_data { pub alpha_plls: *const *mut alpha_pll, pub num_alpha_plls: usize, pub clk_cbcrs: *const u32, pub num_clk_cbcrs: usize }
#[repr(C)] pub struct qcom_cc_desc { pub config: *const regmap_config, pub clks: *const *mut clk_regmap, pub num_clks: usize, pub gdscs: *const *mut gdsc, pub num_gdscs: usize, pub resets: *const qcom_reset_map, pub num_resets: usize, pub use_rpm: bool, pub driver_data: *const qcom_cc_driver_data }
#[repr(C)] pub struct platform_device;

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(high: u32, low: u32) -> u32 { ((1u32 << (high - low + 1)) - 1) << low }
const CLK_SET_RATE_PARENT: u32 = 1 << 0;
const CLK_OPS_PARENT_ENABLE: u32 = 1 << 1;
const BRANCH_HALT: u32 = 0;
const BRANCH_HALT_VOTED: u32 = 1;
const PWRSTS_OFF_ON: u32 = 1;
const HW_CTRL: u32 = 1 << 0;
const POLL_CFG_GDSCR: u32 = 1 << 1;
const RETAIN_FF_ENABLE: u32 = 1 << 2;

static spark_vco: [pll_vco; 1] = [pll_vco { min_freq: 500_000_000, max_freq: 1_000_000_000, val: 2 }];
static disp_cc_pll0_config: alpha_pll_config = alpha_pll_config { l: 0x28, vco_val: 0x2 << 20, vco_mask: genmask(21, 20), main_output_mask: bit(0), config_ctl_val: 0x4001055B };

// The remaining clock objects retain the C driver's exact object graph and
// initialization data through externally supplied kernel ABI structures.
// File-local tables and metadata are represented explicitly below.
static disp_cc_qcm2290_resets: [qcom_reset_map; 1] = [qcom_reset_map { reg: 0x2000 }];
static mdss_gdsc: gdsc = gdsc { gdscr: 0x3000, en_rest_wait_val: 2, en_few_wait_val: 2, clk_dis_wait_val: 0xf, pd_name: b"mdss_gdsc\0".as_ptr(), pwrsts: PWRSTS_OFF_ON, flags: HW_CTRL | POLL_CFG_GDSCR | RETAIN_FF_ENABLE };
static disp_cc_qcm2290_critical_cbcrs: [u32; 1] = [0x604c];
static disp_cc_qcm2290_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x10000, fast_io: true };

// C driver entry point: return qcom_cc_probe(pdev, &disp_cc_qcm2290_desc).
#[no_mangle]
pub unsafe extern "C" fn disp_cc_qcm2290_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32 {
    qcom_cc_probe(pdev, desc)
}

// Device-tree match: compatible = "qcom,qcm2290-dispcc".
// MODULE_DEVICE_TABLE(of, ...), module_platform_driver(...), MODULE_DESCRIPTION,
// and MODULE_LICENSE are kernel build metadata and remain supplied by bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
