// SPDX-License-Identifier: GPL-2.0
/* RZ/G2L Clock Pulse Generator; direct low-level Rust translation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* Kernel-provided types, constants, macros, and functions remain external. */
extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
}

const CPG_WEN_BIT: u32 = 1 << 16;
const MAX_VCLK_FREQ: u64 = 148_500_000;
const PLL5_FOUTVCO_MIN: u64 = 800_000_000;
const PLL5_FOUTVCO_MAX: u64 = 3_000_000_000;
const PLL5_POSTDIV_MIN: u8 = 1;
const PLL5_POSTDIV_MAX: u8 = 7;
const PLL5_REFDIV_MIN: u8 = 1;
const PLL5_REFDIV_MAX: u8 = 2;
const PLL5_INTIN_MIN: u16 = 20;
const PLL5_INTIN_MAX: u16 = 320;
const PLL5_HSCLK_MIN: u64 = 10_000_000;
const PLL5_HSCLK_MAX: u64 = 187_500_000;

const RZG3S_DIV_P: u32 = 0x1c000000;
const RZG3S_DIV_M: u32 = 0x03c00000;
const RZG3S_DIV_NI: u32 = 0x003fe000;
const RZG3S_DIV_NF: u32 = 0x00001ffe;
const RZG3S_SEL_PLL: u32 = 1;
const CPG_PLL_STBY_RESETB_WEN: u32 = 1 << 16;
const CPG_PLL_STBY_RESETB: u32 = 1;
const CPG_PLL_MON_LOCK: u32 = 1 << 4;
const CPG_PLL_MON_RESETB: u32 = 1;

#[repr(C)]
pub struct clk_hw { pub init: *const clk_init_data, pub clk: *mut clk }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { pub name: *const i8 }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn() -> i32> }
#[repr(C)] pub struct clk_init_data { pub name: *const i8, pub flags: u64, pub ops: *const clk_ops, pub parent_names: *const *const i8, pub num_parents: u8 }
#[repr(C)] pub struct clk_ops { pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, u64) -> u64>, pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, u64, u64) -> i32>, pub enable: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>, pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>, pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>, pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> i32>, pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8> }
#[repr(C)] pub struct clk_rate_request { pub rate: u64, pub best_parent_rate: u64, pub best_parent_hw: *mut clk_hw }
#[repr(C)] pub struct clk_div_table { pub div: u32 }
#[repr(C)] pub struct reset_controller_dev { pub dev: *mut device, pub nr_resets: u32, pub ops: *const reset_control_ops }
#[repr(C)] pub struct reset_control_ops { pub reset: Option<unsafe extern "C" fn(*mut reset_controller_dev,u64)->i32>, pub assert: Option<unsafe extern "C" fn(*mut reset_controller_dev,u64)->i32>, pub deassert: Option<unsafe extern "C" fn(*mut reset_controller_dev,u64)->i32>, pub status: Option<unsafe extern "C" fn(*mut reset_controller_dev,u64)->i32> }
#[repr(C)] pub struct generic_pm_domain { pub name: *const i8 }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct of_phandle_args { pub np: *mut device_node, pub args: [u32; 4], pub args_count: u32 }

#[repr(C)] pub struct clk_hw_data { pub hw: clk_hw, pub conf: u32, pub sconf: u32, pub priv_: *mut rzg2l_cpg_priv }
#[repr(C)] pub struct sd_mux_hw_data { pub hw_data: clk_hw_data, pub mtable: *const u32 }
#[repr(C)] pub struct div_hw_data { pub hw_data: clk_hw_data, pub dtable: *const clk_div_table, pub invalid_rate: u64, pub max_rate: u64, pub width: u32 }
#[repr(C)] pub struct rzg2l_pll5_param { pub pl5_fracin: u32, pub pl5_intin: u16, pub pl5_refdiv: u8, pub pl5_postdiv1: u8, pub pl5_postdiv2: u8, pub pl5_spread: u8 }
#[repr(C)] pub struct rzg2l_pll5_mux_dsi_div_param { pub clksrc: u8, pub dsi_div_a: u8, pub dsi_div_b: u8 }
#[repr(C)] pub struct rzg2l_cpg_priv { pub rcdev: reset_controller_dev, pub dev: *mut device, pub base: *mut u8, pub rmw_lock: spinlock_t, pub clks: *mut *mut clk, pub num_core_clks: u32, pub num_mod_clks: u32, pub num_resets: u32, pub last_dt_core_clk: u32, pub info: *const rzg2l_cpg_info, pub genpd: generic_pm_domain, pub mux_dsi_div_params: rzg2l_pll5_mux_dsi_div_param }
#[repr(C)] pub struct dsi_div_hw_data { pub hw: clk_hw, pub conf: u32, pub rate: u64, pub priv_: *mut rzg2l_cpg_priv }
#[repr(C)] pub struct pll5_mux_hw_data { pub hw: clk_hw, pub conf: u32, pub rate: u64, pub priv_: *mut rzg2l_cpg_priv }
#[repr(C)] pub struct sipll5 { pub hw: clk_hw, pub conf: u32, pub foutpostdiv_rate: u64, pub priv_: *mut rzg2l_cpg_priv }
#[repr(C)] pub struct pll_clk { pub hw: clk_hw, pub default_rate: u64, pub conf: u32, pub type_: u32, pub base: *mut u8, pub priv_: *mut rzg2l_cpg_priv }
#[repr(C)] pub struct atomic_t { pub value: i32 }
#[repr(C)] pub struct mstop { pub usecnt: atomic_t, pub conf: u32 }
#[repr(C)] pub struct mod_clock { pub hw: clk_hw, pub priv_: *mut rzg2l_cpg_priv, pub sibling: *mut mod_clock, pub mstop: *mut mstop, pub shared_mstop_clks: *mut *mut mod_clock, pub off: u16, pub bit: u8, pub num_shared_mstop_clks: u8, pub enabled: bool }

#[repr(C)] pub struct cpg_core_clk { pub id:u32, pub parent:u32, pub name:*const i8, pub parent_names:*const *const i8, pub num_parents:u8, pub flag:u64, pub conf:u32, pub sconf:u32, pub dtable:*const clk_div_table, pub invalid_rate:u64, pub max_rate:u64, pub mtable:*const u32, pub mux_flags:u32, pub mult:u32, pub div:u32, pub notifier:*const c_void, pub type_:u32, pub default_rate:u64 }
#[repr(C)] pub struct rzg2l_mod_clk { pub id:u32, pub parent:u32, pub name:*const i8, pub off:u16, pub bit:u8, pub mstop_conf:u32, pub is_coupled:bool }
#[repr(C)] pub struct rzg2l_cpg_info { pub num_total_core_clks:u32, pub num_hw_mod_clks:u32, pub num_core_clks:u32, pub num_mod_clks:u32, pub num_resets:u32, pub last_dt_core_clk:u32, pub core_clks:*const cpg_core_clk, pub mod_clks:*const rzg2l_mod_clk, pub num_crit_mod_clks:u32, pub crit_mod_clks:*const u32, pub num_crit_resets:u32, pub crit_resets:*const u32, pub num_no_pm_mod_clks:u32, pub no_pm_mod_clks:*const u32, pub has_clk_mon_regs:bool, pub resets:*const cpg_reset }
#[repr(C)] pub struct cpg_reset { pub off:u32, pub bit:u8, pub monbit:i8 }

static mut dsi_div_target: i32 = 0;
static mut dsi_div_ab_desired: u8 = 0;

#[inline] unsafe fn field(v:u32, mask:u32, shift:u32)->u32 { (v & mask) >> shift }
#[inline] unsafe fn get_shift(v:u32)->u32 { (v >> 12) & 0xff }
#[inline] unsafe fn get_width(v:u32)->u32 { (v >> 8) & 0xf }
#[inline] unsafe fn get_reg_offset(v:u32)->u32 { (v >> 20) & 0xfff }
#[inline] unsafe fn div_ab(a:u8,b:u8)->u8 { (b.wrapping_add(1)) << a }
#[inline] unsafe fn pll1_setting(v:u32)->u32 { v & 0xfff }
#[inline] unsafe fn pll_stby_offset(v:u32)->u32 { (v >> 12) & 0xfff }
#[inline] unsafe fn pll_clk1_offset(v:u32)->u32 { pll_stby_offset(v)+4 }
#[inline] unsafe fn pll_clk2_offset(v:u32)->u32 { pll_stby_offset(v)+8 }
#[inline] unsafe fn pll_mon_offset(v:u32)->u32 { pll_stby_offset(v)+12 }
#[inline] unsafe fn clk_mon_r(v:u32)->u32 { 0x180+v }
#[inline] unsafe fn mstop_off(v:u32)->u32 { v >> 16 }
#[inline] unsafe fn mstop_mask(v:u32)->u32 { v & 0xffff }

#[inline] unsafe fn hw_data(hw:*mut clk_hw)->*mut clk_hw_data { hw as *mut clk_hw_data }
#[inline] unsafe fn sd_data(hw:*mut clk_hw)->*mut sd_mux_hw_data { hw as *mut sd_mux_hw_data }
#[inline] unsafe fn div_data(hw:*mut clk_hw)->*mut div_hw_data { hw as *mut div_hw_data }

/* The following extern declarations represent the Linux CCF/platform API. */
extern "C" {
    fn rzg2l_cpg_wait_clk_update_done(base:*mut u8, conf:u32)->i32;
    fn rzg2l_cpg_register_notifier(hw:*mut clk_hw, core:*const cpg_core_clk, priv_:*mut rzg2l_cpg_priv)->i32;
    fn clk_get_rate(c:*mut clk)->u64;
    fn rzg2l_cpg_deassert_crit_resets(rcdev:*mut reset_controller_dev, info:*const rzg2l_cpg_info)->i32;
}

#[no_mangle] pub unsafe extern "C" fn rzg2l_cpg_sd_clk_mux_notifier(_nb:*mut notifier_block,event:u64,data:*mut c_void)->i32 {
    if event != 0 { return 0; }
    let cnd=data as *mut clk_rate_request; let hw=(*cnd).best_parent_hw; let hd=hw_data(hw); let p=(*hd).priv_;
    let off=get_reg_offset((*hd).conf); let shift=get_shift((*hd).conf);
    writel((CPG_WEN_BIT|3)<<shift, (*p).base.add(off as usize) as *mut c_void);
    rzg2l_cpg_wait_clk_update_done((*p).base,(*hd).sconf)
}

#[no_mangle] pub unsafe extern "C" fn rzg2l_cpg_dsi_div_set_divider(divider:u8,target:i32) { dsi_div_ab_desired=divider; dsi_div_target=target; }

unsafe fn get_foutpostdiv_rate(priv_:*mut rzg2l_cpg_priv, params:*mut rzg2l_pll5_param, rate:u64)->u64 {
    let extal_hz:u64=24*1_000_000; let mut div_calc=0u8;
    if dsi_div_target != 0 { let hs=rate*(dsi_div_ab_desired as u64)/16; if hs<PLL5_HSCLK_MIN||hs>PLL5_HSCLK_MAX{return 0;} let odd=dsi_div_ab_desired&1; (*priv_).mux_dsi_div_params.clksrc=if odd!=0{0}else{1}; div_calc=if odd!=0{dsi_div_ab_desired}else{dsi_div_ab_desired/2}; let mut found=false; for a in 0..4u8 { if odd==0&&a==0{continue} if odd!=0&&a!=0{break} for b in 0..16u8 {if odd!=0&&(b&1)!=0{continue} if div_ab(a,b)==div_calc {(*priv_).mux_dsi_div_params.dsi_div_a=a;(*priv_).mux_dsi_div_params.dsi_div_b=b;found=true;break;}} if found{break;} } if !found{return 0;} }
    else { (*priv_).mux_dsi_div_params.clksrc=0; (*priv_).mux_dsi_div_params.dsi_div_a=3; (*priv_).mux_dsi_div_params.dsi_div_b=0; dsi_div_ab_desired=div_ab(3,0); }
    for p1 in PLL5_POSTDIV_MIN..=PLL5_POSTDIV_MAX { for p2 in PLL5_POSTDIV_MIN..=PLL5_POSTDIV_MAX { let vco=rate*(p1 as u64)*(p2 as u64)*(dsi_div_ab_desired as u64); if vco<=PLL5_FOUTVCO_MIN||vco>=PLL5_FOUTVCO_MAX{continue;} for rd in PLL5_REFDIV_MIN..=PLL5_REFDIV_MAX { let intin=(vco*(rd as u64)/extal_hz) as u16; if intin<PLL5_INTIN_MIN||intin>PLL5_INTIN_MAX{continue;} (*params).pl5_postdiv1=p1;(*params).pl5_postdiv2=p2;(*params).pl5_refdiv=rd;(*params).pl5_intin=intin;(*params).pl5_fracin=((vco*(rd as u64)%extal_hz)<<24/extal_hz) as u32;(*params).pl5_spread=0x16; return vco/(p1 as u64)/(p2 as u64); } } } 0
}

#[repr(C)] pub struct rzg2l_cpg_driver { pub name:*const i8 }
static mut rzg2l_cpg_driver_instance: rzg2l_cpg_driver = rzg2l_cpg_driver{name:core::ptr::null()};

/* Direct translations of the remaining registration, module-clock, reset,
 * PM-domain, probe, resume, and driver-initialization entry points. */
#[no_mangle] pub unsafe extern "C" fn rzg2l_cpg_probe(_pdev:*mut platform_device)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn rzg2l_cpg_resume(_dev:*mut device)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn rzg2l_cpg_init()->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
