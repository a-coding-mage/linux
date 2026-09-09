/* SPDX-License-Identifier: GPL-2.0 */
/* RZ/G2L Clock Pulse Generator */

// C dependency: linux/notifier.h and related clock definitions are supplied externally.

pub const CPG_SIPLL5_STBY: u32 = 0x140;
pub const CPG_SIPLL5_CLK1: u32 = 0x144;
pub const CPG_SIPLL5_CLK3: u32 = 0x14C;
pub const CPG_SIPLL5_CLK4: u32 = 0x150;
pub const CPG_SIPLL5_CLK5: u32 = 0x154;
pub const CPG_SIPLL5_MON: u32 = 0x15C;
pub const CPG_PL1_DDIV: u32 = 0x200;
pub const CPG_PL2_DDIV: u32 = 0x204;
pub const CPG_PL3A_DDIV: u32 = 0x208;
pub const CPG_PL6_DDIV: u32 = 0x210;
pub const CPG_PL3C_SDIV: u32 = 0x214;
pub const CPG_CLKSTATUS: u32 = 0x280;
pub const CPG_PL3_SSEL: u32 = 0x408;
pub const CPG_PL6_SSEL: u32 = 0x414;
pub const CPG_PL6_ETH_SSEL: u32 = 0x418;
pub const CPG_PL5_SDIV: u32 = 0x420;
pub const CPG_RST_MON: u32 = 0x680;
pub const CPG_BUS_ACPU_MSTOP: u32 = 0xB60;
pub const CPG_BUS_MCPU1_MSTOP: u32 = 0xB64;
pub const CPG_BUS_MCPU2_MSTOP: u32 = 0xB68;
pub const CPG_BUS_PERI_COM_MSTOP: u32 = 0xB6C;
pub const CPG_BUS_PERI_CPU_MSTOP: u32 = 0xB70;
pub const CPG_BUS_PERI_DDR_MSTOP: u32 = 0xB74;
pub const CPG_BUS_PERI_VIDEO_MSTOP: u32 = 0xB78;
pub const CPG_BUS_REG0_MSTOP: u32 = 0xB7C;
pub const CPG_BUS_REG1_MSTOP: u32 = 0xB80;
pub const CPG_BUS_TZCDDR_MSTOP: u32 = 0xB84;
pub const CPG_MHU_MSTOP: u32 = 0xB88;
pub const CPG_BUS_MCPU3_MSTOP: u32 = 0xB90;
pub const CPG_BUS_PERI_CPU2_MSTOP: u32 = 0xB94;
pub const CPG_OTHERFUNC1_REG: u32 = 0xBE8;

pub const CPG_SIPLL5_STBY_RESETB: u32 = 1 << 0;
pub const CPG_SIPLL5_STBY_RESETB_WEN: u32 = 1 << 16;
pub const CPG_SIPLL5_STBY_SSCG_EN_WEN: u32 = 1 << 18;
pub const CPG_SIPLL5_STBY_DOWNSPREAD_WEN: u32 = 1 << 20;
pub const CPG_SIPLL5_CLK4_RESV_LSB: u32 = 0xFF;
pub const CPG_SIPLL5_MON_PLL5_LOCK: u32 = 1 << 4;
pub const CPG_OTHERFUNC1_REG_RES0_ON_WEN: u32 = 1 << 16;
pub const CPG_PL5_SDIV_DIV_DSI_A_WEN: u32 = 1 << 16;
pub const CPG_PL5_SDIV_DIV_DSI_B_WEN: u32 = 1 << 24;
pub const CPG_CLKSTATUS_SELSDHI0_STS: u32 = 1 << 28;
pub const CPG_CLKSTATUS_SELSDHI1_STS: u32 = 1 << 29;

#[inline]
pub const fn cpg_sam_pll_conf(stby: u32) -> u32 { stby << 12 }
#[inline]
pub const fn cpg_pll_conf(stby: u32, setting: u32) -> u32 { (stby << 12) | setting }
#[inline]
pub const fn ddiv_pack(offset: u32, bitpos: u32, size: u32) -> u32 { (offset << 20) | (bitpos << 12) | (size << 8) }
pub const DIVPL1A: u32 = ddiv_pack(CPG_PL1_DDIV, 0, 2);
pub const DIVPL2A: u32 = ddiv_pack(CPG_PL2_DDIV, 0, 3);
pub const DIVDSILPCLK: u32 = ddiv_pack(CPG_PL2_DDIV, 12, 2);
pub const DIVPL3A: u32 = ddiv_pack(CPG_PL3A_DDIV, 0, 3);
pub const DIVPL3B: u32 = ddiv_pack(CPG_PL3A_DDIV, 4, 3);
pub const DIVPL3C: u32 = ddiv_pack(CPG_PL3A_DDIV, 8, 3);
pub const DIVPL3E: u32 = ddiv_pack(CPG_PL3C_SDIV, 8, 5);
pub const DIVGPU: u32 = ddiv_pack(CPG_PL6_DDIV, 0, 2);
#[inline]
pub const fn sel_pll_pack(offset: u32, bitpos: u32, size: u32) -> u32 { ddiv_pack(offset, bitpos, size) }
pub const SEL_PLL3_3: u32 = sel_pll_pack(CPG_PL3_SSEL, 8, 1);
pub const SEL_PLL5_4: u32 = sel_pll_pack(CPG_OTHERFUNC1_REG, 0, 1);
pub const SEL_PLL6_2: u32 = sel_pll_pack(CPG_PL6_ETH_SSEL, 0, 1);
pub const SEL_GPU2: u32 = sel_pll_pack(CPG_PL6_SSEL, 12, 1);
pub const EXTAL_FREQ_IN_MEGA_HZ: u32 = 24;

#[repr(C)]
pub struct cpg_core_clk {
    pub name: *const core::ffi::c_char, pub id: u32, pub parent: u32, pub div: u32, pub mult: u32,
    pub type_: u32, pub conf: u32, pub sconf: u32, pub dtable: *const clk_div_table,
    pub mtable: *const u32, pub invalid_rate: usize, pub max_or_default_rate: usize,
    pub parent_names: *const *const core::ffi::c_char, pub notifier: notifier_fn_t,
    pub flag: u32, pub mux_flags: u32, pub num_parents: i32,
}

#[repr(u32)]
pub enum clk_types { CLK_TYPE_IN, CLK_TYPE_FF, CLK_TYPE_SAM_PLL, CLK_TYPE_G3L_PLL, CLK_TYPE_G3S_PLL, CLK_TYPE_DIV, CLK_TYPE_G3S_DIV, CLK_TYPE_MUX, CLK_TYPE_SD_MUX, CLK_TYPE_SIPLL5, CLK_TYPE_PLL5_4_MUX, CLK_TYPE_DSI_DIV }

#[repr(C)]
pub struct rzg2l_mod_clk { pub name: *const core::ffi::c_char, pub id: u32, pub parent: u32, pub mstop_conf: u32, pub off: u16, pub bit: u8, pub is_coupled: bool }

#[repr(C)]
pub struct rzg2l_reset { pub off: u16, pub bit: u8, pub monbit: i8 }

#[repr(C)]
pub struct rzg2l_cpg_info {
    pub core_clks: *const cpg_core_clk, pub num_core_clks: u32, pub last_dt_core_clk: u32, pub num_total_core_clks: u32,
    pub mod_clks: *const rzg2l_mod_clk, pub num_mod_clks: u32, pub num_hw_mod_clks: u32,
    pub no_pm_mod_clks: *const u32, pub num_no_pm_mod_clks: u32, pub resets: *const rzg2l_reset, pub num_resets: u32,
    pub crit_mod_clks: *const u32, pub num_crit_mod_clks: u32, pub crit_resets: *const u32, pub num_crit_resets: u32,
    pub has_clk_mon_regs: bool,
}

extern "C" {
    pub static r9a07g043_cpg_info: rzg2l_cpg_info;
    pub static r9a07g044_cpg_info: rzg2l_cpg_info;
    pub static r9a07g054_cpg_info: rzg2l_cpg_info;
    pub static r9a08g045_cpg_info: rzg2l_cpg_info;
    pub static r9a08g046_cpg_info: rzg2l_cpg_info;
    pub static r9a09g011_cpg_info: rzg2l_cpg_info;
    pub fn rzg2l_cpg_sd_clk_mux_notifier(nb: *mut notifier_block, event: usize, data: *mut core::ffi::c_void) -> i32;
    pub fn rzg3s_cpg_div_clk_notifier(nb: *mut notifier_block, event: usize, data: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
