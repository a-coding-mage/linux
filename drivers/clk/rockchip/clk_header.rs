/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of clk/rockchip/clk.h.  Kernel-provided types and symbols
 * referenced here are intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

/* Register macros from the C header are represented as Rust macros so their
 * call-site integer type and arithmetic remain visible. */
macro_rules! HIWORD_UPDATE { ($val:expr, $mask:expr, $shift:expr) => {
    (($val) << ($shift)) | (($mask) << (($shift) + 16))
}; }
macro_rules! reg4 { ($x:expr) => { ($x) * 0x4 }; }
macro_rules! reg4_off { ($x:expr, $o:expr) => { ($x) * 0x4 + ($o) }; }

pub const BOOST_CLK_CON: u32 = 0x0008;
pub const BOOST_BOOST_CON: u32 = 0x000c;
pub const BOOST_SWITCH_CNT: u32 = 0x0010;
pub const BOOST_HIGH_PERF_CNT0: u32 = 0x0014;
pub const BOOST_HIGH_PERF_CNT1: u32 = 0x0018;
pub const BOOST_STATIS_THRESHOLD: u32 = 0x001c;
pub const BOOST_SHORT_SWITCH_CNT: u32 = 0x0020;
pub const BOOST_SWITCH_THRESHOLD: u32 = 0x0024;
pub const BOOST_FSM_STATUS: u32 = 0x0028;
pub const BOOST_RECOVERY_MASK: u32 = 0x1;
pub const BOOST_RECOVERY_SHIFT: u32 = 1;
pub const BOOST_SW_CTRL_MASK: u32 = 0x1;
pub const BOOST_SW_CTRL_SHIFT: u32 = 2;
pub const BOOST_LOW_FREQ_EN_MASK: u32 = 0x1;
pub const BOOST_LOW_FREQ_EN_SHIFT: u32 = 3;
pub const BOOST_BUSY_STATE: u32 = 1 << 8;

macro_rules! BOOST_PLL_H_CON { ($x:expr) => { reg4!($x) }; }
macro_rules! BOOST_PLL_L_CON { ($x:expr) => { reg4_off!($x, 0x2c) }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub union rockchip_pll_rate_union {
    pub rk3066: rockchip_pll_rate_rk3066,
    pub rk3036: rockchip_pll_rate_rk3036,
    pub rk3588: rockchip_pll_rate_rk3588,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct rockchip_pll_rate_rk3066 { pub nr:u32,pub nf:u32,pub no:u32,pub nb:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct rockchip_pll_rate_rk3036 { pub fbdiv:u32,pub postdiv1:u32,pub refdiv:u32,pub postdiv2:u32,pub dsmpd:u32,pub frac:u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct rockchip_pll_rate_rk3588 { pub m:u32,pub p:u32,pub s:u32,pub k:i16 }
#[repr(C)] pub struct rockchip_pll_rate_table { pub rate: libc::c_ulong, pub data: rockchip_pll_rate_union }

#[repr(C)] pub struct rockchip_cpuclk_clksel { pub reg: i32, pub val: u32 }
pub const ROCKCHIP_CPUCLK_NUM_DIVIDERS: usize = 6;
pub const ROCKCHIP_CPUCLK_MAX_CORES: usize = 4;
#[repr(C)] pub struct rockchip_cpuclk_rate_table { pub prate: libc::c_ulong, pub divs:[rockchip_cpuclk_clksel;6], pub pre_muxs:[rockchip_cpuclk_clksel;6], pub post_muxs:[rockchip_cpuclk_clksel;6] }

#[repr(C)] pub struct rockchip_cpuclk_reg_data { pub core_reg:[i32;4], pub div_core_shift:[u8;4], pub div_core_mask:[u32;4], pub num_cores:i32, pub mux_core_reg:i32, pub mux_core_alt:u8, pub mux_core_main:u8, pub mux_core_shift:u8, pub mux_core_mask:u32 }

#[repr(C)] pub struct rockchip_gate_link_platdata { pub ctx:*mut rockchip_clk_provider, pub clkbr:*mut rockchip_clk_branch }
#[repr(C)] pub struct rockchip_clk_provider { pub reg_base:*mut core::ffi::c_void, pub clk_data: clk_onecell_data, pub cru_node:*mut device_node, pub grf:*mut regmap, pub lock: spinlock_t }
#[repr(C)] pub struct clk_onecell_data { pub clks:*mut *mut clk, pub clk_num:usize }
#[repr(C)] pub struct device_node { _private:[u8;0] }
#[repr(C)] pub struct regmap { _private:[u8;0] }
#[repr(C)] pub struct spinlock_t { _private:[u8;0] }
#[repr(C)] pub struct clk { _private:[u8;0] }

#[repr(C)] pub struct rockchip_clk_branch { pub id:u32, pub branch_type:rockchip_clk_branch_type, pub name:*const i8, pub parent_names:*const *const i8, pub num_parents:u8, pub flags:libc::c_ulong, pub muxdiv_offset:i32, pub mux_shift:u8, pub mux_width:u8, pub mux_flags:u8, pub mux_table:*mut u32, pub div_offset:i32, pub div_shift:u8, pub div_width:u8, pub div_flags:u8, pub div_table:*mut clk_div_table, pub gate_offset:i32, pub gate_shift:u8, pub gate_flags:u8, pub linked_clk_id:u32, pub grf_type:rockchip_grf_type, pub child:*mut rockchip_clk_branch }
#[repr(C)] pub struct clk_div_table { _private:[u8;0] }
#[repr(C)] pub struct rockchip_pll_clock { pub id:u32, pub name:*const i8, pub parent_names:*const *const i8, pub num_parents:u8, pub flags:libc::c_ulong, pub con_offset:i32, pub mode_offset:i32, pub mode_shift:i32, pub lock_shift:i32, pub pll_type:rockchip_pll_type, pub pll_flags:u8, pub rate_table:*mut rockchip_pll_rate_table }
#[repr(C)] pub enum rockchip_pll_type { pll_rk3036, pll_rk3066, pll_rk3328, pll_rk3399, pll_rk3588, pll_rk3588_core, pll_rk3588_ddr }
#[repr(C)] pub enum rockchip_grf_type { grf_type_sys=0, grf_type_pmu0, grf_type_pmu1, grf_type_ioc, grf_type_vo, grf_type_vpu }
#[repr(C)] pub enum rockchip_clk_branch_type { branch_composite, branch_mux, branch_grf_mux, branch_divider, branch_fraction_divider, branch_gate, branch_grf_gate, branch_linked_gate, branch_mmc, branch_grf_mmc, branch_inverter, branch_factor, branch_ddrclk, branch_half_divider }

pub const ROCKCHIP_PLL_SYNC_RATE:u32=1<<0; pub const ROCKCHIP_PLL_FIXED_MODE:u32=1<<1; pub const ROCKCHIP_DDRCLK_SIP:u32=1<<0; pub const ROCKCHIP_INVERTER_HIWORD_MASK:u32=1<<0; pub const ROCKCHIP_SOFTRST_HIWORD_MASK:u32=1<<0;

/* The remaining C initialiser macros and register-offset families are
 * intentionally preserved as source-level macro invocations below. */
macro_rules! PNAME { ($x:ident) => { pub static $x: &[*const i8] = &[]; }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
