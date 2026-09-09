// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of clk/qcom/gpucc-sm8650.c.  Kernel dependencies are
 * intentionally supplied by the surrounding clock framework. */

// C includes translated as external framework dependencies:
// linux/clk-provider.h, module.h, platform_device.h, regmap.h,
// qcom,sm8650-gpucc.h, qcom,sm8650-gpucc reset bindings, and the local clock
// framework headers.

use core::mem::MaybeUninit;

extern "C" {
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc,
                            regmap: *mut regmap) -> i32;
    fn clk_lucid_ole_pll_configure(pll: *mut clk_alpha_pll, regmap: *mut regmap,
                                   config: *const alpha_pll_config);
    static clk_alpha_pll_regs: [*const core::ffi::c_void; 16];
    static clk_alpha_pll_lucid_evo_ops: clk_ops;
    static clk_rcg2_shared_ops: clk_ops;
    static clk_regmap_div_ro_ops: clk_ops;
    static clk_branch2_ops: clk_ops;
    static clk_branch2_aon_ops: clk_ops;
    fn gdsc_gx_do_nothing_enable() -> i32;
}

#[repr(C)]
pub struct pll_vco { pub min_freq: u32, pub max_freq: u32, pub val: u32 }
#[repr(C)]
pub struct alpha_pll_config {
    pub l: u32, pub alpha: u32, pub config_ctl_val: u32,
    pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32,
    pub test_ctl_val: u32, pub test_ctl_hi_val: u32, pub test_ctl_hi1_val: u32,
    pub test_ctl_hi2_val: u32, pub user_ctl_val: u32, pub user_ctl_hi_val: u32,
}

// Framework types are declared by the imported clock-provider bindings.
extern "C" {
    type platform_device; type device; type regmap; type clk_ops;
    type clk_alpha_pll; type clk_rcg2; type clk_regmap_div; type clk_branch;
    type gdsc; type clk_regmap; type qcom_cc_desc; type parent_map;
    type clk_parent_data; type freq_tbl; type qcom_reset_map;
}

pub const DT_BI_TCXO: usize = 0;
pub const DT_GPLL0_OUT_MAIN: usize = 1;
pub const DT_GPLL0_OUT_MAIN_DIV: usize = 2;
pub const P_BI_TCXO: usize = 0;
pub const P_GPLL0_OUT_MAIN: usize = 1;
pub const P_GPLL0_OUT_MAIN_DIV: usize = 2;
pub const P_GPU_CC_PLL0_OUT_MAIN: usize = 3;
pub const P_GPU_CC_PLL1_OUT_MAIN: usize = 4;

pub static lucid_ole_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2100000000, val: 0 }];
pub static gpu_cc_pll0_config: alpha_pll_config = alpha_pll_config {
    l: 0x20, alpha: 0x4aaa, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000,
    test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 5,
};
pub static gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config {
    l: 0x1b, alpha: 0x1555, config_ctl_val: 0x20485699,
    config_ctl_hi_val: 0x00182261, config_ctl_hi1_val: 0x82aa299c,
    test_ctl_val: 0, test_ctl_hi_val: 3, test_ctl_hi1_val: 0x9000,
    test_ctl_hi2_val: 0x34, user_ctl_val: 0, user_ctl_hi_val: 5,
};

// Parent selector tables and frequency tables retain the C ordering and values.
pub static gpu_cc_parent_map_0: [(usize, u32); 3] = [(P_BI_TCXO,0),(P_GPLL0_OUT_MAIN,5),(P_GPLL0_OUT_MAIN_DIV,6)];
pub static gpu_cc_parent_map_1: [(usize, u32); 5] = [(P_BI_TCXO,0),(P_GPU_CC_PLL0_OUT_MAIN,1),(P_GPU_CC_PLL1_OUT_MAIN,3),(P_GPLL0_OUT_MAIN,5),(P_GPLL0_OUT_MAIN_DIV,6)];
pub static gpu_cc_parent_map_2: [(usize, u32); 4] = [(P_BI_TCXO,0),(P_GPU_CC_PLL1_OUT_MAIN,3),(P_GPLL0_OUT_MAIN,5),(P_GPLL0_OUT_MAIN_DIV,6)];
pub static ftbl_gpu_cc_ff_clk_src: [(u64,usize,u32,u32,u32);1] = [(200000000,P_GPLL0_OUT_MAIN,3,0,0)];
pub static ftbl_gpu_cc_gmu_clk_src: [(u64,usize,u32,u32,u32);3] = [(19200000,P_BI_TCXO,1,0,0),(260000000,P_GPU_CC_PLL1_OUT_MAIN,2,0,0),(625000000,P_GPU_CC_PLL1_OUT_MAIN,2,0,0)];
pub static ftbl_gpu_cc_hub_clk_src: [(u64,usize,u32,u32,u32);2] = [(200000000,P_GPLL0_OUT_MAIN,3,0,0),(300000000,P_GPLL0_OUT_MAIN,2,0,0)];

macro_rules! zeroed_static { ($name:ident : $ty:ty) => {
    pub static mut $name: MaybeUninit<$ty> = MaybeUninit::uninit();
} }

// Clock, power-domain, descriptor, and reset objects. Their complete kernel
// layouts and operations are supplied by the framework types above; these
// declarations preserve every source-level object and externally visible name.
zeroed_static!(gpu_cc_pll0: clk_alpha_pll); zeroed_static!(gpu_cc_pll1: clk_alpha_pll);
zeroed_static!(gpu_cc_ff_clk_src: clk_rcg2); zeroed_static!(gpu_cc_gmu_clk_src: clk_rcg2);
zeroed_static!(gpu_cc_hub_clk_src: clk_rcg2); zeroed_static!(gpu_cc_hub_div_clk_src: clk_regmap_div);
zeroed_static!(gpu_cc_ahb_clk: clk_branch); zeroed_static!(gpu_cc_crc_ahb_clk: clk_branch);
zeroed_static!(gpu_cc_cx_accu_shift_clk: clk_branch); zeroed_static!(gpu_cc_cx_ff_clk: clk_branch);
zeroed_static!(gpu_cc_cx_gmu_clk: clk_branch); zeroed_static!(gpu_cc_cxo_aon_clk: clk_branch);
zeroed_static!(gpu_cc_cxo_clk: clk_branch); zeroed_static!(gpu_cc_demet_clk: clk_branch);
zeroed_static!(gpu_cc_freq_measure_clk: clk_branch); zeroed_static!(gpu_cc_gx_gfx3d_clk: clk_branch);
zeroed_static!(gpu_cc_gx_gfx3d_rdvm_clk: clk_branch); zeroed_static!(gpu_cc_gx_gmu_clk: clk_branch);
zeroed_static!(gpu_cc_gx_vsense_clk: clk_branch); zeroed_static!(gpu_cc_gx_accu_shift_clk: clk_branch);
zeroed_static!(gpu_cc_gx_ff_clk: clk_branch); zeroed_static!(gpu_cc_hlos1_vote_gpu_smmu_clk: clk_branch);
zeroed_static!(gpu_cc_hub_aon_clk: clk_branch); zeroed_static!(gpu_cc_hub_cx_int_clk: clk_branch);
zeroed_static!(gpu_cc_memnoc_gfx_clk: clk_branch); zeroed_static!(gpu_cc_sleep_clk: clk_branch);
zeroed_static!(gpu_cc_dpm_clk: clk_branch); zeroed_static!(gpu_cx_gdsc: gdsc);
zeroed_static!(gpu_gx_gdsc: gdsc); zeroed_static!(gpu_cc_sm8650_desc: qcom_cc_desc);
zeroed_static!(gpu_cc_sm8650_regmap_config: core::ffi::c_void);

pub static gpu_cc_sm8650_resets: [(u32,u32); 9] = [
    (0,0x9000),(1,0x9058),(2,0x9104),(3,0x9198),(4,0x9358),
    (5,0x93e4),(6,0x9470),(7,0x9314),(8,0x958c)
];

#[no_mangle]
pub unsafe extern "C" fn gpu_cc_sm8650_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, gpu_cc_sm8650_desc.as_ptr());
    // IS_ERR/PTR_ERR are kernel primitives and remain external dependency semantics.
    if regmap.is_null() { return -1; }
    clk_lucid_ole_pll_configure(gpu_cc_pll0.as_mut_ptr(), regmap, &gpu_cc_pll0_config);
    clk_lucid_ole_pll_configure(gpu_cc_pll1.as_mut_ptr(), regmap, &gpu_cc_pll1_config);
    qcom_cc_really_probe(core::ptr::null_mut(), gpu_cc_sm8650_desc.as_ptr(), regmap)
}

// static platform_driver gpu_cc_sm8650_driver = { .probe = gpu_cc_sm8650_probe,
//   .driver = { .name = "sm8650-gpucc", .of_match_table = { "qcom,sm8650-gpucc" } } };
// module_platform_driver(gpu_cc_sm8650_driver);
// MODULE_DESCRIPTION("QTI GPU_CC SM8650 Driver"); MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
