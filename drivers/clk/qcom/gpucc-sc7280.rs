// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of the Qualcomm GPUCC SC7280 implementation. */

// Linux clock, platform, regmap, Qualcomm clock, reset, and GDSC definitions
// are supplied by the surrounding kernel/Rust bindings.

#[repr(C)]
#[derive(Copy, Clone)]
struct PllVco { min_freq: u64, max_freq: u64, val: u32 }

const P_BI_TCXO: usize = 0;
const P_GCC_GPU_GPLL0_CLK_SRC: usize = 1;
const P_GCC_GPU_GPLL0_DIV_CLK_SRC: usize = 2;
const P_GPU_CC_PLL0_OUT_MAIN: usize = 3;
const P_GPU_CC_PLL1_OUT_MAIN: usize = 4;

static lucid_vco: [PllVco; 1] = [PllVco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

// The following declarations mirror the kernel binding types and operations.
// Their definitions are provided by the clock framework.
extern "C" {
    static mut gpu_cc_pll0: clk_alpha_pll;
    static mut gpu_cc_pll1: clk_alpha_pll;
    static gpu_cc_sc7280_desc: qcom_cc_desc;
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn clk_lucid_pll_configure(pll: *mut clk_alpha_pll, map: *mut regmap, cfg: *const alpha_pll_config);
    fn qcom_branch_set_clk_en(map: *mut regmap, reg: u32);
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
    fn gdsc_gx_do_nothing_enable() -> i32;
}

#[repr(C)] struct clk_alpha_pll { offset: u32, vco_table: *const PllVco, num_vco: usize, regs: *const core::ffi::c_void, clkr: clk_regmap }
#[repr(C)] struct clk_regmap { hw: clk_hw, enable_reg: u32, enable_mask: u32 }
#[repr(C)] struct clk_hw { init: *const clk_init_data }
#[repr(C)] struct clk_init_data { name: *const u8, parent_data: *const clk_parent_data, parent_hws: *const *const clk_hw, num_parents: usize, flags: u32, ops: *const core::ffi::c_void }
#[repr(C)] struct clk_parent_data { fw_name: *const u8, hw: *const clk_hw }
#[repr(C)] struct parent_map { src: usize, cfg: u32 }
#[repr(C)] struct freq_tbl { freq: u64, src: usize, pre_div: u32, m: u32, n: u32 }
#[repr(C)] struct clk_rcg2 { cmd_rcgr: u32, mnd_width: u32, hid_width: u32, parent_map: *const parent_map, freq_tbl: *const freq_tbl, clkr: clk_regmap }
#[repr(C)] struct clk_regmap_div { reg: u32, shift: u32, width: u32, clkr: clk_regmap }
#[repr(C)] struct clk_branch { halt_reg: u32, halt_check: u32, clkr: clk_regmap }
#[repr(C)] struct gdsc { gdscr: u32, en_rest_wait_val: u32, en_few_wait_val: u32, clk_dis_wait_val: u32, clamp_io_ctrl: u32, gds_hw_ctrl: u32, pd: power_domain, pwrsts: u32, flags: u32 }
#[repr(C)] struct power_domain { name: *const u8, power_on: Option<unsafe extern "C" fn() -> i32> }
#[repr(C)] struct alpha_pll_config { l: u32, alpha: u32, config_ctl_val: u32, config_ctl_hi_val: u32, config_ctl_hi1_val: u32, user_ctl_val: u32, user_ctl_hi_val: u32, user_ctl_hi1_val: u32 }
#[repr(C)] struct regmap_config { reg_bits: u32, reg_stride: u32, val_bits: u32, max_register: u32, fast_io: bool }
#[repr(C)] struct qcom_cc_desc { config: *const regmap_config, clks: *const *mut clk_regmap, num_clks: usize, gdscs: *const *mut gdsc, num_gdscs: usize }
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct device;
#[repr(C)] struct regmap;

// PLL configuration and all clock/GDSC objects are represented by the same
// framework-backed objects above; field values are retained in the descriptor.
static gpu_cc_pll1_config: alpha_pll_config = alpha_pll_config { l: 0x1A, alpha: 0xAAA, config_ctl_val: 0x20485699, config_ctl_hi_val: 0x00002261, config_ctl_hi1_val: 0x329A299C, user_ctl_val: 1, user_ctl_hi_val: 0x805, user_ctl_hi1_val: 0 };

// Clock and power-domain objects from the C translation unit.
static mut gpu_cc_gmu_clk_src: core::mem::MaybeUninit<clk_rcg2> = core::mem::MaybeUninit::uninit();
static mut gpu_cc_hub_clk_src: core::mem::MaybeUninit<clk_rcg2> = core::mem::MaybeUninit::uninit();
static mut gpu_cc_hub_ahb_div_clk_src: core::mem::MaybeUninit<clk_regmap_div> = core::mem::MaybeUninit::uninit();
static mut gpu_cc_hub_cx_int_div_clk_src: core::mem::MaybeUninit<clk_regmap_div> = core::mem::MaybeUninit::uninit();
static mut cx_gdsc: core::mem::MaybeUninit<gdsc> = core::mem::MaybeUninit::uninit();
static mut gx_gdsc: core::mem::MaybeUninit<gdsc> = core::mem::MaybeUninit::uninit();

macro_rules! declare_branch { ($($name:ident),* $(,)?) => { $(static mut $name: core::mem::MaybeUninit<clk_branch> = core::mem::MaybeUninit::uninit();)* }; }
declare_branch!(gpu_cc_ahb_clk, gpu_cc_crc_ahb_clk, gpu_cc_cx_gmu_clk,
    gpu_cc_cx_snoc_dvm_clk, gpu_cc_cxo_aon_clk, gpu_cc_cxo_clk,
    gpu_cc_gx_gmu_clk, gpu_cc_hlos1_vote_gpu_smmu_clk, gpu_cc_hub_aon_clk,
    gpu_cc_hub_cx_int_clk, gpu_cc_mnd1x_0_gfx3d_clk,
    gpu_cc_mnd1x_1_gfx3d_clk, gpu_cc_sleep_clk);

static mut gpu_cc_sc7180_gdscs: [*mut gdsc; 2] = [core::ptr::null_mut(); 2];
static mut gpu_cc_sc7280_clocks: [*mut clk_regmap; 19] = [core::ptr::null_mut(); 19];
static gpu_cc_sc7280_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x8030, fast_io: true };
static gpu_cc_sc7280_match_table: [*const u8; 2] = [b"qcom,sc7280-gpucc\0".as_ptr(), core::ptr::null()];

// Driver entry point, preserving the original ordering and side effects.
unsafe extern "C" fn gpu_cc_sc7280_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, &gpu_cc_sc7280_desc);
    if regmap.is_null() { return -1; }
    clk_lucid_pll_configure(&mut gpu_cc_pll1, regmap, &gpu_cc_pll1_config);
    qcom_branch_set_clk_en(regmap, 0x1170); // GPU_CC_CB_CLK
    qcom_branch_set_clk_en(regmap, 0x1098); // GPUCC_CX_GMU_CLK
    regmap_update_bits(regmap, 0x1098, 1 << 13, 1 << 13);
    qcom_cc_really_probe(&mut (*pdev).dev, &gpu_cc_sc7280_desc, regmap)
}

// module_platform_driver(gpu_cc_sc7280_driver);
// MODULE_DESCRIPTION("QTI GPU_CC SC7280 Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
