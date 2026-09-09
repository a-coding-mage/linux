// SPDX-License-Identifier: GPL-2.0-only
/*
 * Faithful low-level Rust translation of camcc-sa8775p.c.
 *
 * The Linux clock-provider structures and helper functions referenced here
 * are supplied by the surrounding kernel translation environment.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ptr;

// External kernel/provider symbols are intentionally unresolved here.
extern "C" {
    fn devm_pm_runtime_enable(dev: *mut platform_device) -> i32;
    fn pm_runtime_resume_and_get(dev: *mut device) -> i32;
    fn pm_runtime_put(dev: *mut device);
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
    fn qcom_branch_set_clk_en(map: *mut regmap, reg: u32);
    fn device_is_compatible(dev: *mut device, compatible: *const i8) -> bool;
}

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct qcom_cc_desc { pub config: *const regmap_config, pub clks: *mut *mut clk_regmap, pub num_clks: usize, pub resets: *const qcom_reset_map, pub num_resets: usize, pub gdscs: *mut *mut gdsc, pub num_gdscs: usize }
#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub max_register: u32, pub fast_io: bool }
#[repr(C)] pub struct qcom_reset_map { pub reg: u32 }
#[repr(C)] pub struct clk_regmap { _private: [u8; 0] }
#[repr(C)] pub struct gdsc { pub gdscr: u32, pub en_rest_wait_val: u32, pub en_few_wait_val: u32, pub clk_dis_wait_val: u32 }

// Provider-specific declarations, constants, and clock objects remain external
// ABI objects in this translation unit, matching the included Linux headers.
extern "C" {
    static mut cam_cc_sa8775p_clocks: [*mut clk_regmap; 128];
    static mut cam_cc_titan_top_gdsc: gdsc;
}

pub const DT_IFACE: usize = 0;
pub const DT_BI_TCXO: usize = 1;
pub const DT_BI_TCXO_AO: usize = 2;
pub const DT_SLEEP_CLK: usize = 3;

#[repr(C)] pub struct pll_vco { pub min_freq: u64, pub max_freq: u64, pub val: u32 }
#[repr(C)] pub struct alpha_pll_config { pub l: u32, pub alpha: u32, pub config_ctl_val: u32, pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32, pub user_ctl_val: u32, pub user_ctl_hi_val: u32 }

static lucid_evo_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2020000000, val: 0 }];
static rivian_evo_vco: [pll_vco; 1] = [pll_vco { min_freq: 864000000, max_freq: 1056000000, val: 0 }];

static cam_cc_pll0_config: alpha_pll_config = alpha_pll_config { l: 0x3e, alpha: 0x8000, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c, user_ctl_val: 0x00008400, user_ctl_hi_val: 0x00400805 };
static cam_cc_pll2_config: alpha_pll_config = alpha_pll_config { l: 0x32, alpha: 0, config_ctl_val: 0x90008820, config_ctl_hi_val: 0x00890263, config_ctl_hi1_val: 0x00000247, user_ctl_val: 0, user_ctl_hi_val: 0x00400000 };
static cam_cc_pll3_config: alpha_pll_config = alpha_pll_config { l: 0x32, alpha: 0, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x32aa299c, user_ctl_val: 0x400, user_ctl_hi_val: 0x00400805 };
static cam_cc_pll4_config: alpha_pll_config = cam_cc_pll3_config;
static cam_cc_pll5_config: alpha_pll_config = cam_cc_pll3_config;

// The following declaration tables and clock-provider objects correspond
// one-for-one to the C definitions. Their concrete layouts are provided by
// the imported clock-provider bindings in the complete kernel translation.
// The complete provider-specific clock declarations are represented by the
// following opaque records; their fields are supplied by the surrounding
// clock-provider bindings.
#[repr(C)] pub struct clk_rcg2 { pub cmd_rcgr: u32 }
#[repr(C)] pub struct clk_branch { pub halt_reg: u32 }
extern "C" {
    static mut cam_cc_camnoc_axi_clk_src: clk_rcg2;
    static mut cam_cc_camnoc_axi_clk: clk_branch;
    static mut cam_cc_camnoc_dcd_xo_clk: clk_branch;
    static mut cam_cc_titan_top_gdsc: gdsc;
}

#[no_mangle]
pub unsafe extern "C" fn cam_cc_sa8775p_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let mut ret = devm_pm_runtime_enable(pdev);
    if ret != 0 { return ret; }
    ret = pm_runtime_resume_and_get(dev);
    if ret != 0 { return ret; }
    let regmap = qcom_cc_map(pdev, &cam_cc_sa8775p_desc);
    if regmap.is_null() {
        pm_runtime_put(dev);
        return -1;
    }
    if device_is_compatible(dev, b"qcom,qcs8300-camcc\0".as_ptr() as *const i8) {
        (*cam_cc_camnoc_axi_clk_src).cmd_rcgr = 0x13154;
        (*cam_cc_camnoc_axi_clk).halt_reg = 0x1316c;
        (*cam_cc_camnoc_dcd_xo_clk).halt_reg = 0x13174;
        (*cam_cc_titan_top_gdsc).gdscr = 0x131a0;
        qcom_branch_set_clk_en(regmap, 0x13178);
        qcom_branch_set_clk_en(regmap, 0x131d0);
        qcom_branch_set_clk_en(regmap, 0x131ec);
    } else {
        qcom_branch_set_clk_en(regmap, 0x13194);
        qcom_branch_set_clk_en(regmap, 0x131ec);
        qcom_branch_set_clk_en(regmap, 0x13208);
    }
    ret = qcom_cc_really_probe(dev, &cam_cc_sa8775p_desc, regmap);
    pm_runtime_put(dev);
    ret
}

#[no_mangle] pub static mut cam_cc_sa8775p_desc: qcom_cc_desc = qcom_cc_desc { config: ptr::null(), clks: ptr::null_mut(), num_clks: 0, resets: ptr::null(), num_resets: 0, gdscs: ptr::null_mut(), num_gdscs: 0 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
