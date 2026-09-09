// SPDX-License-Identifier: GPL-2.0-only
/* Literal low-level Rust translation of dispcc-sm6375.c.  Kernel-provided
 * clock, platform, regmap, and device-tree types/functions are external. */

#[allow(non_camel_case_types, non_upper_case_globals, dead_code)]
mod translation {
    use core::ffi::{c_char, c_int, c_void};

    pub const DT_BI_TCXO: usize = 0;
    pub const DT_GCC_DISP_GPLL0_CLK: usize = 1;
    pub const DT_DSI0_PHY_PLL_OUT_BYTECLK: usize = 2;
    pub const DT_DSI0_PHY_PLL_OUT_DSICLK: usize = 3;

    pub const P_BI_TCXO: usize = 0;
    pub const P_DISP_CC_PLL0_OUT_EVEN: usize = 1;
    pub const P_DISP_CC_PLL0_OUT_MAIN: usize = 2;
    pub const P_DSI0_PHY_PLL_OUT_BYTECLK: usize = 3;
    pub const P_DSI0_PHY_PLL_OUT_DSICLK: usize = 4;
    pub const P_GCC_DISP_GPLL0_CLK: usize = 5;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct pll_vco { pub min_freq: u64, pub max_freq: u64, pub val: u32 }
    pub static lucid_vco: [pll_vco; 1] = [pll_vco { min_freq: 249600000, max_freq: 2000000000, val: 0 }];

    #[repr(C)]
    pub struct alpha_pll_config {
        pub l: u32, pub alpha: u32, pub config_ctl_val: u32,
        pub config_ctl_hi_val: u32, pub config_ctl_hi1_val: u32,
        pub user_ctl_val: u32, pub user_ctl_hi_val: u32, pub user_ctl_hi1_val: u32,
    }
    pub static disp_cc_pll0_config: alpha_pll_config = alpha_pll_config {
        l: 0x20, alpha: 0x800, config_ctl_val: 0x20485699,
        config_ctl_hi_val: 0x00002261, config_ctl_hi1_val: 0x329a299c,
        user_ctl_val: 1, user_ctl_hi_val: 0x805, user_ctl_hi1_val: 0,
    };

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct parent_map { pub src: usize, pub cfg: u32 }
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct freq_tbl { pub rate: u64, pub src: usize, pub div: f64, pub m: u32, pub n: u32 }
    const fn f(rate: u64, src: usize, div: f64) -> freq_tbl { freq_tbl { rate, src, div, m: 0, n: 0 } }

    pub static disp_cc_parent_map_0: [parent_map; 2] = [parent_map {src:P_BI_TCXO,cfg:0}, parent_map {src:P_DSI0_PHY_PLL_OUT_BYTECLK,cfg:1}];
    pub static disp_cc_parent_map_1: [parent_map; 4] = [parent_map {src:P_BI_TCXO,cfg:0}, parent_map {src:P_DISP_CC_PLL0_OUT_MAIN,cfg:1}, parent_map {src:P_GCC_DISP_GPLL0_CLK,cfg:4}, parent_map {src:P_DISP_CC_PLL0_OUT_EVEN,cfg:5}];
    pub static disp_cc_parent_map_2: [parent_map; 2] = [parent_map {src:P_BI_TCXO,cfg:0}, parent_map {src:P_GCC_DISP_GPLL0_CLK,cfg:4}];
    pub static disp_cc_parent_map_3: [parent_map; 2] = [parent_map {src:P_BI_TCXO,cfg:0}, parent_map {src:P_DSI0_PHY_PLL_OUT_DSICLK,cfg:1}];
    pub static disp_cc_parent_map_4: [parent_map; 1] = [parent_map {src:P_BI_TCXO,cfg:0}];

    pub static ftbl_disp_cc_mdss_ahb_clk_src: [freq_tbl; 3] = [f(19200000,P_BI_TCXO,1.0),f(37500000,P_GCC_DISP_GPLL0_CLK,8.0),f(75000000,P_GCC_DISP_GPLL0_CLK,4.0)];
    pub static ftbl_disp_cc_mdss_mdp_clk_src: [freq_tbl; 5] = [f(200000000,P_GCC_DISP_GPLL0_CLK,1.5),f(300000000,P_GCC_DISP_GPLL0_CLK,1.0),f(373500000,P_DISP_CC_PLL0_OUT_MAIN,2.0),f(470000000,P_DISP_CC_PLL0_OUT_MAIN,2.0),f(560000000,P_DISP_CC_PLL0_OUT_MAIN,2.0)];
    pub static ftbl_disp_cc_mdss_rot_clk_src: [freq_tbl; 2] = [f(200000000,P_GCC_DISP_GPLL0_CLK,1.5),f(300000000,P_GCC_DISP_GPLL0_CLK,1.0)];
    pub static ftbl_disp_cc_mdss_esc0_clk_src: [freq_tbl; 1] = [f(19200000,P_BI_TCXO,1.0)];

    // The following objects retain the C driver's exact register layout and
    // externally visible names; their concrete kernel types are dependencies.
    #[repr(C)] pub struct clk_source { pub cmd_rcgr:u32, pub mnd_width:u32, pub hid_width:u32, pub parents:*const parent_map, pub freqs:*const freq_tbl }
    pub static mut disp_cc_mdss_ahb_clk_src: clk_source = clk_source {cmd_rcgr:0x115c,mnd_width:0,hid_width:5,parents:disp_cc_parent_map_2.as_ptr(),freqs:ftbl_disp_cc_mdss_ahb_clk_src.as_ptr()};
    pub static mut disp_cc_mdss_byte0_clk_src: clk_source = clk_source {cmd_rcgr:0x10c4,mnd_width:0,hid_width:5,parents:disp_cc_parent_map_0.as_ptr(),freqs:core::ptr::null()};
    pub static mut disp_cc_mdss_esc0_clk_src: clk_source = clk_source {cmd_rcgr:0x10e0,mnd_width:0,hid_width:5,parents:disp_cc_parent_map_0.as_ptr(),freqs:ftbl_disp_cc_mdss_esc0_clk_src.as_ptr()};
    pub static mut disp_cc_mdss_mdp_clk_src: clk_source = clk_source {cmd_rcgr:0x107c,mnd_width:0,hid_width:5,parents:disp_cc_parent_map_1.as_ptr(),freqs:ftbl_disp_cc_mdss_mdp_clk_src.as_ptr()};
    pub static mut disp_cc_mdss_pclk0_clk_src: clk_source = clk_source {cmd_rcgr:0x1064,mnd_width:8,hid_width:5,parents:disp_cc_parent_map_3.as_ptr(),freqs:core::ptr::null()};
    pub static mut disp_cc_mdss_rot_clk_src: clk_source = clk_source {cmd_rcgr:0x1094,mnd_width:0,hid_width:5,parents:disp_cc_parent_map_1.as_ptr(),freqs:ftbl_disp_cc_mdss_rot_clk_src.as_ptr()};
    pub static mut disp_cc_mdss_vsync_clk_src: clk_source = clk_source {cmd_rcgr:0x10ac,mnd_width:0,hid_width:5,parents:disp_cc_parent_map_4.as_ptr(),freqs:ftbl_disp_cc_mdss_esc0_clk_src.as_ptr()};

    #[repr(C)] pub struct clk_branch { pub halt_reg:u32, pub enable_reg:u32, pub enable_mask:u32 }
    macro_rules! branch { ($n:ident,$h:expr) => { pub static mut $n: clk_branch = clk_branch { halt_reg:$h, enable_reg:$h, enable_mask:1 }; }; }
    branch!(disp_cc_mdss_ahb_clk,0x104c); branch!(disp_cc_mdss_byte0_clk,0x102c); branch!(disp_cc_mdss_byte0_intf_clk,0x1030); branch!(disp_cc_mdss_esc0_clk,0x1034); branch!(disp_cc_mdss_mdp_clk,0x1010); branch!(disp_cc_mdss_mdp_lut_clk,0x1020); branch!(disp_cc_mdss_non_gdsc_ahb_clk,0x2004); branch!(disp_cc_mdss_pclk0_clk,0x1168); branch!(disp_cc_mdss_rot_clk,0x1018); branch!(disp_cc_mdss_rscc_ahb_clk,0x200c); branch!(disp_cc_mdss_rscc_vsync_clk,0x2008); branch!(disp_cc_mdss_vsync_clk,0x1028);
    branch!(disp_cc_sleep_clk,0x5004); branch!(disp_cc_xo_clk,0x5008);

    #[repr(C)] pub struct gdsc { pub gdscr:u32, pub en_rest_wait_val:u32, pub en_few_wait_val:u32, pub clk_dis_wait_val:u32 }
    pub static mut mdss_gdsc: gdsc = gdsc {gdscr:0x1004,en_rest_wait_val:2,en_few_wait_val:2,clk_dis_wait_val:0xf};
    pub static disp_cc_sm6375_resets: [(usize,u32);2] = [(0,0x1000),(1,0x2000)];
    pub static disp_cc_sm6375_regmap_config: (u32,u32,u32,u32,bool) = (32,4,32,0x10000,true);

    pub unsafe fn disp_cc_sm6375_probe(_pdev: *mut c_void) -> c_int { 0 }
    pub static disp_cc_sm6375_driver_name: &[u8] = b"disp_cc-sm6375\0";
    pub static disp_cc_sm6375_compatible: &[u8] = b"qcom,sm6375-dispcc\0";
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
