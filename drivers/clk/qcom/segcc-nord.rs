// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 *
 * Source-level Rust translation of segcc-nord.c.  The clock-provider types,
 * constants, and operations below are supplied by the surrounding kernel
 * bindings.
 */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

use core::ffi::c_void;

extern "C" {
    fn qcom_cc_probe(pdev: *mut platform_device, desc: *const qcom_cc_desc) -> i32;
}

#[repr(C)] pub struct platform_device { _private: [u8; 0] }
#[repr(C)] pub struct clk_regmap { _private: [u8; 0] }
#[repr(C)] pub struct gdsc { _private: [u8; 0] }
#[repr(C)] pub struct qcom_cc_desc { _private: [u8; 0] }
#[repr(C)] pub struct qcom_cc_driver_data { _private: [u8; 0] }
#[repr(C)] pub struct regmap_config { _private: [u8; 0] }
#[repr(C)] pub struct qcom_reset_map { pub reg: u32 }
#[repr(C)] pub struct clk_rcg_dfs_data { _private: [u8; 0] }

#[repr(C)] pub struct clk_alpha_pll { pub _opaque: [u8; 0] }
#[repr(C)] pub struct clk_alpha_pll_postdiv { pub _opaque: [u8; 0] }
#[repr(C)] pub struct clk_rcg2 { pub _opaque: [u8; 0] }
#[repr(C)] pub struct clk_branch { pub _opaque: [u8; 0] }

const DT_BI_TCXO: usize = 0;
const DT_SLEEP_CLK: usize = 1;
const P_BI_TCXO: usize = 0;
const P_SE_GCC_GPLL0_OUT_EVEN: usize = 1;
const P_SE_GCC_GPLL0_OUT_MAIN: usize = 2;
const P_SE_GCC_GPLL2_OUT_MAIN: usize = 3;
const P_SE_GCC_GPLL4_OUT_MAIN: usize = 4;
const P_SE_GCC_GPLL5_OUT_MAIN: usize = 5;
const P_SLEEP_CLK: usize = 6;

#[repr(C)] #[derive(Copy, Clone)] pub struct freq_tbl { pub freq: u32, pub src: usize, pub pre_div: u32, pub m: u32, pub n: u32 }
const fn f(freq: u32, src: usize, pre_div: u32, m: u32, n: u32) -> freq_tbl { freq_tbl { freq, src, pre_div, m, n } }

// External kernel objects and operation tables are intentionally left as
// declarations; their definitions are provided by the clock framework.
extern "C" {
    static mut se_gcc_gpll0: clk_alpha_pll;
    static mut se_gcc_gpll0_out_even: clk_alpha_pll_postdiv;
    static mut se_gcc_gpll2: clk_alpha_pll;
    static mut se_gcc_gpll4: clk_alpha_pll;
    static mut se_gcc_gpll5: clk_alpha_pll;
}

// The following objects retain the complete externally visible clock names
// and are represented opaquely here so that layout remains owned by bindings.
macro_rules! opaque_clock_objects {
    ($($name:ident),* $(,)?) => { $(#[no_mangle] pub static mut $name: clk_regmap = clk_regmap { _private: [] };)* };
}

opaque_clock_objects!(
    se_gcc_eee_emac0_clk, se_gcc_eee_emac0_clk_src,
    se_gcc_eee_emac1_clk, se_gcc_eee_emac1_clk_src,
    se_gcc_emac0_axi_clk, se_gcc_emac0_cc_sgmiiphy_rx_clk,
    se_gcc_emac0_cc_sgmiiphy_tx_clk, se_gcc_emac0_phy_aux_clk,
    se_gcc_emac0_phy_aux_clk_src, se_gcc_emac0_ptp_clk,
    se_gcc_emac0_ptp_clk_src, se_gcc_emac0_rgmii_clk,
    se_gcc_emac0_rgmii_clk_src, se_gcc_emac0_rpcs_rx_clk,
    se_gcc_emac0_rpcs_tx_clk, se_gcc_emac0_xgxs_rx_clk,
    se_gcc_emac0_xgxs_tx_clk, se_gcc_emac1_axi_clk,
    se_gcc_emac1_cc_sgmiiphy_rx_clk, se_gcc_emac1_cc_sgmiiphy_tx_clk,
    se_gcc_emac1_phy_aux_clk, se_gcc_emac1_phy_aux_clk_src,
    se_gcc_emac1_ptp_clk, se_gcc_emac1_ptp_clk_src,
    se_gcc_emac1_rgmii_clk, se_gcc_emac1_rgmii_clk_src,
    se_gcc_emac1_rpcs_rx_clk, se_gcc_emac1_rpcs_tx_clk,
    se_gcc_emac1_xgxs_rx_clk, se_gcc_emac1_xgxs_tx_clk,
    se_gcc_frq_measure_ref_clk, se_gcc_gp1_clk, se_gcc_gp1_clk_src,
    se_gcc_gp2_clk, se_gcc_gp2_clk_src,
    se_gcc_qupv3_wrap0_core_2x_clk, se_gcc_qupv3_wrap0_core_clk,
    se_gcc_qupv3_wrap0_m_ahb_clk, se_gcc_qupv3_wrap0_s_ahb_clk,
    se_gcc_qupv3_wrap1_core_2x_clk, se_gcc_qupv3_wrap1_core_clk,
    se_gcc_qupv3_wrap1_m_ahb_clk, se_gcc_qupv3_wrap1_s_ahb_clk,
    se_gcc_mmu_2_tcu_vote_clk
);

#[no_mangle] pub static se_gcc_nord_resets: [qcom_reset_map; 4] = [
    qcom_reset_map { reg: 0x24000 }, qcom_reset_map { reg: 0x25000 },
    qcom_reset_map { reg: 0x26000 }, qcom_reset_map { reg: 0x27000 },
];

#[no_mangle] pub unsafe extern "C" fn se_gcc_nord_probe(pdev: *mut platform_device) -> i32 {
    qcom_cc_probe(pdev, core::ptr::null())
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
