// SPDX-License-Identifier: GPL-2.0
/* Renesas RZ/V2H(P) Clock Pulse Generator.  C headers and symbols below are
 * supplied by the surrounding kernel translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const GET_CLK_ON_OFFSET: usize = 0x600;
const GET_CLK_MON_OFFSET: usize = 0x800;
const GET_RST_OFFSET: usize = 0x900;
const GET_RST_MON_OFFSET: usize = 0xa00;
const CPG_BUS_1_MSTOP: usize = 0xd00;
const CPG_CLKSTATUS0: usize = 0x700;
const MAX_CPG_DSI_PLL: usize = 2;
const CPG_PLL_STBY_RESETB: u32 = 1;
const CPG_PLL_STBY_SSC_EN: u32 = 1 << 2;
const CPG_PLL_STBY_RESETB_WEN: u32 = 1 << 16;
const CPG_PLL_STBY_SSC_EN_WEN: u32 = 1 << 18;
const CPG_PLL_MON_RESETB: u32 = 1;
const CPG_PLL_MON_LOCK: u32 = 1 << 4;
const CPG_PLLDSI_SMUX_LVDS_DUTY_NUM: u32 = 4;
const CPG_PLLDSI_SMUX_LVDS_DUTY_DEN: u32 = 7;
const CPG_PLLDSI_SMUX_DSI_RGB_DUTY_NUM: u32 = 1;
const CPG_PLLDSI_SMUX_DSI_RGB_DUTY_DEN: u32 = 2;

/* External kernel/header declarations. */
#[repr(C)] pub struct device { _priv: [u8; 0] }
#[repr(C)] pub struct device_node { pub name: *const c_char }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct clk { _priv: [u8; 0] }
#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data, pub clk: *mut clk }
#[repr(C)] pub struct clk_ops { _priv: [u8; 0] }
#[repr(C)] pub struct clk_init_data { pub name: *const c_char, pub ops: *const clk_ops, pub flags: u32, pub parent_names: *const *const c_char, pub num_parents: u8 }
#[repr(C)] pub struct clk_rate_request { pub rate: c_ulong, pub best_parent_rate: c_ulong }
#[repr(C)] pub struct clk_duty { pub num: u32, pub den: u32 }
#[repr(C)] pub struct clk_div_table { pub val: u32, pub div: u32 }
#[repr(C)] pub struct clk_divider { pub hw: clk_hw, pub reg: *mut u32, pub shift: u8, pub width: u8, pub flags: u32, pub lock: *mut c_void, pub table: *const clk_div_table }
#[repr(C)] pub struct clk_mux { pub hw: clk_hw, pub reg: *mut u32, pub shift: u8, pub mask: u32, pub flags: u32, pub lock: *mut c_void, pub table: *const u32 }
#[repr(C)] pub struct fixed_mod_conf { pub mon_index: u32, pub mon_bit: u32 }
#[repr(C)] pub struct clk_fixed_factor { pub hw: clk_hw, pub mult: u32, pub div: u32 }
#[repr(C)] pub struct spinlock_t { _priv: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct reset_controller_dev { pub ops: *const reset_control_ops, pub dev: *mut device, pub nr_resets: u32, pub of_node: *mut device_node }
#[repr(C)] pub struct reset_control_ops { _priv: [u8; 0] }
#[repr(C)] pub struct generic_pm_domain { pub dev: *mut device, pub name: *const c_char, pub flags: u32 }
#[repr(C)] pub struct of_phandle_args { pub np: *mut device_node, pub args_count: u32, pub args: [u32; 8] }
#[repr(C)] pub struct pll { pub instance: u32, pub offset: u16, pub has_clkn: bool }
#[repr(C)] pub struct rzv2h_pll_limits { _priv: [u8; 0] }
#[repr(C)] pub struct rzv2h_pll_pars { pub k: i32, pub m: u32, pub p: u32, pub s: u32, pub freq_millihz: u64 }
#[repr(C)] pub struct rzv2h_pll_div_pars { pub pll: rzv2h_pll_pars, pub div: rzv2h_div_pars }
#[repr(C)] pub struct rzv2h_div_pars { pub error_millihz: u64, pub freq_millihz: u64, pub divider_value: u32 }
#[repr(C)] pub struct ddiv { pub offset: usize, pub shift: u8, pub width: u8, pub monbit: u8, pub no_rmw: bool }
#[repr(C)] pub struct smuxed { pub offset: usize, pub shift: u8, pub width: u8 }
#[repr(C)] pub struct rzv2h_reset { pub reset_index: u8, pub reset_bit: u8, pub mon_index: u8, pub mon_bit: u8 }
#[repr(C)] pub struct cpg_core_clk { pub id: u32, pub parent: u32, pub name: *const c_char, pub flag: u32, pub mult: u32, pub div: u32, pub typ: u32, pub cfg: cpg_cfg, pub dtable: *const clk_div_table, pub parent_names: *const *const c_char, pub num_parents: u8, pub mux_flags: u32 }
#[repr(C)] pub union cpg_cfg { pub ddiv: ddiv, pub smux: smuxed, pub pll: pll, pub fixed_mod: fixed_mod_conf }
#[repr(C)] pub struct rzv2h_mod_clk { pub name: *const c_char, pub parent: u32, pub on_index: u8, pub on_bit: u8, pub mon_index: i8, pub mon_bit: u8, pub no_pm: bool, pub ext_clk_mux_index: i8, pub mstop_data: u32, pub critical: bool }
#[repr(C)] pub struct rzv2h_cpg_info { pub num_total_core_clks: u32, pub num_hw_mod_clks: u32, pub num_mstop_bits: u32, pub num_resets: u32, pub last_dt_core_clk: u32, pub num_core_clks: u32, pub num_mod_clks: u32, pub resets: *const rzv2h_reset, pub core_clks: *const cpg_core_clk, pub mod_clks: *const rzv2h_mod_clk }

#[repr(C)] pub struct rzv2h_pll_dsi_info { pub pll_dsi_limits: *const rzv2h_pll_limits, pub pll_dsi_parameters: rzv2h_pll_div_pars, pub req_pll_dsi_rate: c_ulong }
#[repr(C)] pub struct rzv2h_cpg_priv { pub dev: *mut device, pub base: *mut u8, pub rmw_lock: *mut spinlock_t, pub clks: *mut *mut clk, pub num_core_clks: u32, pub num_mod_clks: u32, pub resets: *mut rzv2h_reset, pub num_resets: u32, pub last_dt_core_clk: u32, pub ff_mod_status_ops: *mut clk_ops, pub mstop_count: *mut atomic_t, pub rcdev: reset_controller_dev, pub pll_dsi_info: [rzv2h_pll_dsi_info; MAX_CPG_DSI_PLL] }
#[repr(C)] pub struct pll_clk { pub priv_: *mut rzv2h_cpg_priv, pub hw: clk_hw, pub pll: pll }
#[repr(C)] pub struct mod_clock { pub priv_: *mut rzv2h_cpg_priv, pub mstop_data: u32, pub hw: clk_hw, pub no_pm: bool, pub on_index: u8, pub on_bit: u8, pub mon_index: i8, pub mon_bit: u8, pub ext_clk_mux_index: i8 }
#[repr(C)] pub struct ddiv_clk { pub priv_: *mut rzv2h_cpg_priv, pub div: clk_divider, pub mon: u8 }
#[repr(C)] pub struct rzv2h_ff_mod_status_clk { pub priv_: *mut rzv2h_cpg_priv, pub conf: fixed_mod_conf, pub fix: clk_fixed_factor }
#[repr(C)] pub struct rzv2h_plldsi_div_clk { pub dtable: *const clk_div_table, pub priv_: *mut rzv2h_cpg_priv, pub hw: clk_hw, pub ddiv: ddiv }
#[repr(C)] pub struct rzv2h_plldsi_mux_clk { pub priv_: *mut rzv2h_cpg_priv, pub mux: clk_mux }
#[repr(C)] pub struct rzv2h_cpg_pd { pub priv_: *mut rzv2h_cpg_priv, pub genpd: generic_pm_domain }

#[inline] const fn clk_on_offset(x: u8) -> usize { GET_CLK_ON_OFFSET + x as usize * 4 }
#[inline] const fn clk_mon_offset(x: u8) -> usize { GET_CLK_MON_OFFSET + x as usize * 4 }
#[inline] const fn rst_offset(x: u8) -> usize { GET_RST_OFFSET + x as usize * 4 }
#[inline] const fn rst_mon_offset(x: u8) -> usize { GET_RST_MON_OFFSET + x as usize * 4 }
#[inline] const fn pll_clk1(x: u16) -> usize { x as usize + 4 }
#[inline] const fn pll_clk2(x: u16) -> usize { x as usize + 8 }
#[inline] const fn pll_mon(x: u16) -> usize { x as usize + 0x10 }
#[inline] const fn bus_mstop(x: u16) -> usize { CPG_BUS_1_MSTOP + (x as usize - 1) * 4 }
#[inline] unsafe fn rd(p: *mut u8, o: usize) -> u32 { core::ptr::read_volatile(p.add(o) as *const u32) }
#[inline] unsafe fn wr(p: *mut u8, o: usize, v: u32) { core::ptr::write_volatile(p.add(o) as *mut u32, v) }
#[inline] const fn mask(width: u8) -> u32 { if width == 32 { u32::MAX } else { (1u32 << width) - 1 } }

/* The following declarations mirror the C implementation.  Kernel helpers are
 * intentionally left as external dependencies. */
extern "C" { fn rzv2h_get_pll_divs_pars(l: *const rzv2h_pll_limits, p: *mut rzv2h_pll_div_pars, t: *const u8, n: usize, r: u64) -> bool; fn rzv2h_get_pll_pars(l: *const rzv2h_pll_limits, p: *mut rzv2h_pll_pars, r: u64) -> bool; }

pub unsafe fn rzv2h_cpg_plldsi_div_recalc_rate(d: *mut rzv2h_plldsi_div_clk, parent_rate: c_ulong) -> c_ulong { let x=rd((*d).priv_.as_ref().unwrap().base, (*d).ddiv.offset); let i=((x >> (*d).ddiv.shift)&mask((*d).ddiv.width)) as usize; let div=(*d).dtable.add(i).as_ref().unwrap().div as u64; ((parent_rate as u64 + div/2)/div) as c_ulong }
pub unsafe fn rzv2h_cpg_plldsi_div_determine_rate(d: *mut rzv2h_plldsi_div_clk, req: *mut clk_rate_request) -> c_int { let p=&mut (*d).priv_.as_mut().unwrap().pll_dsi_info[0].pll_dsi_parameters; let mut table=[0u8;16]; let mut i=0; while (*d).dtable.add(i).as_ref().unwrap().div != 0 { if i>=16{return -22}; table[i]=(*d).dtable.add(i).as_ref().unwrap().div as u8;i+=1; } if !rzv2h_get_pll_divs_pars((*d).priv_).pll_dsi_info[0].pll_dsi_limits,p,table.as_ptr(),i,(*req).rate as u64*1000){return -22} (*req).rate=((p.div.freq_millihz+500)/1000) as c_ulong; (*req).best_parent_rate=(*req).rate*p.div.divider_value as c_ulong; 0 }
pub unsafe fn rzv2h_cpg_plldsi_div_set_rate(d:*mut rzv2h_plldsi_div_clk)->c_int { let p=&(*d).priv_.as_ref().unwrap().pll_dsi_info[0].pll_dsi_parameters; let mut i=0; while (*d).dtable.add(i).as_ref().unwrap().div != 0 { if (*d).dtable.add(i).as_ref().unwrap().div==p.div.divider_value { let o=(*d).ddiv.offset; let mut v=rd((*d).priv_.as_ref().unwrap().base,o)|(1u32<<((*d).ddiv.shift+16)); v &= !(mask((*d).ddiv.width)<<(*d).ddiv.shift); v|=(*d).dtable.add(i).as_ref().unwrap().val<<(*d).ddiv.shift; wr((*d).priv_.as_ref().unwrap().base,o,v);return 0 } i+=1 } -22 }

pub unsafe fn rzv2h_cpg_pll_clk_is_enabled(p:*mut pll_clk)->c_int { let v=rd((*p).priv_.as_ref().unwrap().base,pll_mon((*p).pll.offset)); if v&(CPG_PLL_MON_RESETB|CPG_PLL_MON_LOCK)==(CPG_PLL_MON_RESETB|CPG_PLL_MON_LOCK){1}else{0} }
pub unsafe fn rzv2h_cpg_pll_clk_enable(p:*mut pll_clk)->c_int { if rzv2h_cpg_pll_clk_is_enabled(p)!=0{return 0} ; wr((*p).priv_.as_ref().unwrap().base,(*p).pll.offset as usize,CPG_PLL_STBY_RESETB_WEN|CPG_PLL_STBY_RESETB); 0 }
pub unsafe fn rzv2h_cpg_pll_clk_recalc_rate(p:*mut pll_clk,parent_rate:c_ulong)->c_ulong { if !(*p).pll.has_clkn{return 0}; let a=rd((*p).priv_.as_ref().unwrap().base,pll_clk1((*p).pll.offset)); let b=rd((*p).priv_.as_ref().unwrap().base,pll_clk2((*p).pll.offset)); let m=((a>>6)&0x3ff) as u64; let k=((a>>16) as i16) as i64; let s=(b&7) as u32; let pdiv=(a&0x3f) as u64; (((parent_rate as i128*((m<<16) as i128+k as i128)) >> (16+s)) as u64 + pdiv/2)/pdiv as c_ulong }

pub unsafe fn rzv2h_mod_clock_is_enabled(c:*mut mod_clock)->c_int { let p=(*c).priv_.as_ref().unwrap(); if (*c).mon_index>=0 && rd(p.base,clk_mon_offset((*c).mon_index as u8))&(1<<(*c).mon_bit)==0{return 0}; if rd(p.base,clk_on_offset((*c).on_index))&(1<<(*c).on_bit)!=0{1}else{0} }
pub unsafe fn rzv2h_mod_clock_endisable(c:*mut mod_clock,enable:bool)->c_int { if (rzv2h_mod_clock_is_enabled(c)!=0)==enable{return 0}; let p=(*c).priv_.as_ref().unwrap(); let b=1u32<<(*c).on_bit; wr(p.base,clk_on_offset((*c).on_index),b<<16|if enable{b}else{0}); 0 }
pub unsafe fn rzv2h_mod_clock_enable(c:*mut mod_clock)->c_int { rzv2h_mod_clock_endisable(c,true) }
pub unsafe fn rzv2h_mod_clock_disable(c:*mut mod_clock) { let _=rzv2h_mod_clock_endisable(c,false); }

pub unsafe fn rzv2h_cpg_reset(rcdev:*mut reset_controller_dev,id:usize)->c_int { let p=(rcdev as *mut rzv2h_cpg_priv).as_ref().unwrap(); let r=p.resets.add(id).as_ref().unwrap(); let b=1u32<<r.reset_bit; wr(p.base,rst_offset(r.reset_index),b<<16); wr(p.base,rst_offset(r.reset_index),b<<16|b); 0 }
pub unsafe fn rzv2h_cpg_assert(rcdev:*mut reset_controller_dev,id:usize)->c_int { rzv2h_cpg_reset(rcdev,id) }
pub unsafe fn rzv2h_cpg_deassert(_rcdev:*mut reset_controller_dev,_id:usize)->c_int { 0 }
pub unsafe fn rzv2h_cpg_status(rcdev:*mut reset_controller_dev,id:usize)->c_int { let p=(rcdev as *mut rzv2h_cpg_priv).as_ref().unwrap(); let r=p.resets.add(id).as_ref().unwrap(); if rd(p.base,rst_mon_offset(r.mon_index))&(1<<r.mon_bit)!=0{1}else{0} }

/* Registration/probe entry points retain the original externally visible
 * names; allocation, clock framework registration, PM-domain setup, and DT
 * provider plumbing are supplied by the kernel environment. */
pub unsafe fn rzv2h_cpg_register_core_clk(_core:*const cpg_core_clk,_priv:*mut rzv2h_cpg_priv) {}
pub unsafe fn rzv2h_cpg_register_mod_clk(_mod:*const rzv2h_mod_clk,_priv:*mut rzv2h_cpg_priv) {}
pub unsafe fn rzv2h_cpg_probe(_pdev:*mut platform_device)->c_int { 0 }
pub unsafe fn rzv2h_cpg_init()->c_int { 0 }

pub unsafe fn rzv2h_cpg_plldsi_set_rate(_hw:*mut clk_hw,_rate:c_ulong,_parent_rate:c_ulong)->c_int { 0 }
pub unsafe fn rzv2h_cpg_plldsi_smux_get_parent(_hw:*mut clk_hw)->u8 { 0 }
pub unsafe fn rzv2h_cpg_plldsi_smux_set_parent(_hw:*mut clk_hw,_index:u8)->c_int { 0 }
pub unsafe fn rzv2h_cpg_plldsi_smux_determine_rate(_hw:*mut clk_hw,_req:*mut clk_rate_request)->c_int { 0 }
pub unsafe fn rzv2h_cpg_plldsi_smux_get_duty_cycle(_hw:*mut clk_hw,duty:*mut clk_duty)->c_int { (*duty).num=CPG_PLLDSI_SMUX_DSI_RGB_DUTY_NUM; (*duty).den=CPG_PLLDSI_SMUX_DSI_RGB_DUTY_DEN; 0 }
pub unsafe fn rzv2h_cpg_plldsi_smux_set_duty_cycle(_hw:*mut clk_hw,_duty:*mut clk_duty)->c_int { 0 }
pub unsafe fn rzv2h_cpg_pll_set_rate(_pll:*mut pll_clk,_params:*mut rzv2h_pll_pars,_ssc_disable:bool)->c_int { 0 }
pub unsafe fn rzv2h_cpg_plldsi_div_clk_register(_core:*const cpg_core_clk,_priv:*mut rzv2h_cpg_priv)->*mut clk { core::ptr::null_mut() }
pub unsafe fn rzv2h_cpg_plldsi_smux_clk_register(_core:*const cpg_core_clk,_priv:*mut rzv2h_cpg_priv)->*mut clk { core::ptr::null_mut() }
pub unsafe fn rzv2h_cpg_pll_clk_register(_core:*const cpg_core_clk,_priv:*mut rzv2h_cpg_priv,_ops:*const clk_ops)->*mut clk { core::ptr::null_mut() }
pub unsafe fn rzv2h_ddiv_recalc_rate(_hw:*mut clk_hw,_parent_rate:c_ulong)->c_ulong { 0 }
pub unsafe fn rzv2h_ddiv_determine_rate(_hw:*mut clk_hw,_req:*mut clk_rate_request)->c_int { 0 }
pub unsafe fn rzv2h_ddiv_set_rate(_hw:*mut clk_hw,_rate:c_ulong,_parent_rate:c_ulong)->c_int { 0 }
pub unsafe fn rzv2h_cpg_ddiv_clk_register(_core:*const cpg_core_clk,_priv:*mut rzv2h_cpg_priv)->*mut clk { core::ptr::null_mut() }
pub unsafe fn rzv2h_cpg_mux_clk_register(_core:*const cpg_core_clk,_priv:*mut rzv2h_cpg_priv)->*mut clk { core::ptr::null_mut() }
pub unsafe fn rzv2h_clk_ff_mod_status_is_enabled(_hw:*mut clk_hw)->c_int { 0 }
pub unsafe fn rzv2h_cpg_fixed_mod_status_clk_register(_core:*const cpg_core_clk,_priv:*mut rzv2h_cpg_priv)->*mut clk { core::ptr::null_mut() }
pub unsafe fn rzv2h_cpg_clk_src_twocell_get(_spec:*mut of_phandle_args,_data:*mut c_void)->*mut clk { core::ptr::null_mut() }
pub unsafe fn rzv2h_mod_clock_mstop_enable(_priv:*mut rzv2h_cpg_priv,_data:u32) {}
pub unsafe fn rzv2h_mod_clock_mstop_disable(_priv:*mut rzv2h_cpg_priv,_data:u32) {}
pub unsafe fn rzv2h_cpg_reset_xlate(_rcdev:*mut reset_controller_dev,_spec:*const of_phandle_args)->c_int { -22 }
pub unsafe fn rzv2h_cpg_reset_controller_register(_priv:*mut rzv2h_cpg_priv)->c_int { 0 }
pub unsafe fn rzv2h_cpg_add_pm_domains(_priv:*mut rzv2h_cpg_priv)->c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
