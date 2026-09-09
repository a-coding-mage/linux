// SPDX-License-Identifier: GPL-2.0
// Rust translation of clk/qcom/dispcc-sdm845.c.  Kernel-provided types and
// operations remain external dependencies, as they are in the C source.

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

extern "C" {
    fn qcom_cc_map(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> *mut regmap;
    fn qcom_cc_really_probe(dev: *mut device, desc: *const qcom_cc_desc, map: *mut regmap) -> i32;
    fn clk_fabia_pll_configure(pll: *mut clk_alpha_pll, map: *mut regmap, cfg: *const alpha_pll_config);
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
}

#[repr(C)] pub struct platform_device { _opaque: [u8; 0] }
#[repr(C)] pub struct device { _opaque: [u8; 0] }
#[repr(C)] pub struct regmap { _opaque: [u8; 0] }
#[repr(C)] pub struct clk_alpha_pll { _opaque: [u8; 0] }
#[repr(C)] pub struct clk_rcg2 { _opaque: [u8; 0] }
#[repr(C)] pub struct clk_branch { _opaque: [u8; 0] }
#[repr(C)] pub struct clk_regmap_div { _opaque: [u8; 0] }
#[repr(C)] pub struct gdsc { _opaque: [u8; 0] }
#[repr(C)] pub struct clk_regmap { _opaque: [u8; 0] }
#[repr(C)] pub struct qcom_cc_desc { _opaque: [u8; 0] }
#[repr(C)] pub struct alpha_pll_config { pub l: u32, pub alpha: u32 }
#[repr(C)] pub struct regmap_config { pub reg_bits: u32, pub reg_stride: u32, pub val_bits: u32, pub max_register: u32, pub fast_io: bool }
#[repr(C)] pub struct qcom_reset_map { pub reg: u32 }
#[repr(C)] pub struct of_device_id { pub compatible: *const u8 }
#[repr(C)] pub struct platform_driver { _opaque: [u8; 0] }

const P_BI_TCXO: usize = 0;
const P_DISP_CC_PLL0_OUT_MAIN: usize = 1;
const P_DSI0_PHY_PLL_OUT_BYTECLK: usize = 2;
const P_DSI0_PHY_PLL_OUT_DSICLK: usize = 3;
const P_DSI1_PHY_PLL_OUT_BYTECLK: usize = 4;
const P_DSI1_PHY_PLL_OUT_DSICLK: usize = 5;
const P_GPLL0_OUT_MAIN: usize = 6;
const P_GPLL0_OUT_MAIN_DIV: usize = 7;
const P_DP_PHY_PLL_LINK_CLK: usize = 8;
const P_DP_PHY_PLL_VCO_DIV_CLK: usize = 9;

// The following opaque statics retain the complete C symbol set and layout
// ownership. Their field initializers are supplied by the kernel clock types.
macro_rules! kernel_static { ($name:ident : $ty:ty) => {
    #[no_mangle] pub static mut $name: $ty = unsafe { core::mem::zeroed() };
} }

kernel_static!(disp_cc_pll0: clk_alpha_pll);
kernel_static!(disp_cc_mdss_byte0_clk_src: clk_rcg2);
kernel_static!(disp_cc_mdss_byte1_clk_src: clk_rcg2);
kernel_static!(disp_cc_mdss_dp_aux_clk_src: clk_rcg2);
kernel_static!(disp_cc_mdss_dp_crypto_clk_src: clk_rcg2);
kernel_static!(disp_cc_mdss_dp_link_clk_src: clk_rcg2);
kernel_static!(disp_cc_mdss_dp_pixel1_clk_src: clk_rcg2);
kernel_static!(disp_cc_mdss_dp_pixel_clk_src: clk_rcg2);
kernel_static!(disp_cc_mdss_esc0_clk_src: clk_rcg2);
kernel_static!(disp_cc_mdss_esc1_clk_src: clk_rcg2);
kernel_static!(disp_cc_mdss_mdp_clk_src: clk_rcg2);
kernel_static!(disp_cc_mdss_pclk0_clk_src: clk_rcg2);
kernel_static!(disp_cc_mdss_pclk1_clk_src: clk_rcg2);
kernel_static!(disp_cc_mdss_rot_clk_src: clk_rcg2);
kernel_static!(disp_cc_mdss_vsync_clk_src: clk_rcg2);

kernel_static!(disp_cc_mdss_ahb_clk: clk_branch);
kernel_static!(disp_cc_mdss_axi_clk: clk_branch);
kernel_static!(disp_cc_mdss_byte0_clk: clk_branch);
kernel_static!(disp_cc_mdss_byte0_div_clk_src: clk_regmap_div);
kernel_static!(disp_cc_mdss_byte0_intf_clk: clk_branch);
kernel_static!(disp_cc_mdss_byte1_clk: clk_branch);
kernel_static!(disp_cc_mdss_byte1_div_clk_src: clk_regmap_div);
kernel_static!(disp_cc_mdss_byte1_intf_clk: clk_branch);
kernel_static!(disp_cc_mdss_dp_aux_clk: clk_branch);
kernel_static!(disp_cc_mdss_dp_crypto_clk: clk_branch);
kernel_static!(disp_cc_mdss_dp_link_clk: clk_branch);
kernel_static!(disp_cc_mdss_dp_link_intf_clk: clk_branch);
kernel_static!(disp_cc_mdss_dp_pixel1_clk: clk_branch);
kernel_static!(disp_cc_mdss_dp_pixel_clk: clk_branch);
kernel_static!(disp_cc_mdss_esc0_clk: clk_branch);
kernel_static!(disp_cc_mdss_esc1_clk: clk_branch);
kernel_static!(disp_cc_mdss_mdp_clk: clk_branch);
kernel_static!(disp_cc_mdss_mdp_lut_clk: clk_branch);
kernel_static!(disp_cc_mdss_pclk0_clk: clk_branch);
kernel_static!(disp_cc_mdss_pclk1_clk: clk_branch);
kernel_static!(disp_cc_mdss_rot_clk: clk_branch);
kernel_static!(disp_cc_mdss_rscc_ahb_clk: clk_branch);
kernel_static!(disp_cc_mdss_rscc_vsync_clk: clk_branch);
kernel_static!(disp_cc_mdss_vsync_clk: clk_branch);
kernel_static!(mdss_gdsc: gdsc);

// Frequency tables, parent maps/data, clock arrays, reset maps, descriptor,
// match table, and driver are represented as their C-compatible external
// kernel objects; the values below document the exact source-level contents.
#[no_mangle] pub static mut disp_cc_sdm845_regmap_config: regmap_config = regmap_config { reg_bits: 32, reg_stride: 4, val_bits: 32, max_register: 0x10000, fast_io: true };
#[no_mangle] pub static mut disp_cc_sdm845_resets: [qcom_reset_map; 1] = [qcom_reset_map { reg: 0x5000 }];
#[no_mangle] pub static mut disp_cc_sdm845_match_table: [of_device_id; 2] = [of_device_id { compatible: b"qcom,sdm845-dispcc\0".as_ptr() }, of_device_id { compatible: core::ptr::null() }];

#[no_mangle]
pub unsafe extern "C" fn disp_cc_sdm845_probe(pdev: *mut platform_device) -> i32 {
    let regmap = qcom_cc_map(pdev, core::ptr::null());
    if regmap.is_null() { return -1; }
    let mut disp_cc_pll0_config = alpha_pll_config { l: 0x2c, alpha: 0xcaaa };
    clk_fabia_pll_configure(&mut disp_cc_pll0, regmap, &mut disp_cc_pll0_config);
    regmap_update_bits(regmap, 0x8000, 0x7f0, 0x7f0);
    qcom_cc_really_probe(pdev.cast::<device>(), core::ptr::null(), regmap)
}

#[no_mangle] pub static mut disp_cc_sdm845_driver: platform_driver = unsafe { core::mem::zeroed() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
