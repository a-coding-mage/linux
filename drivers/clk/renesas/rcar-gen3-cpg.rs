// SPDX-License-Identifier: GPL-2.0
/* R-Car Gen3 Clock Pulse Generator */

// Kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_void};

const CPG_PLLECR: usize = 0x00d0;
const CPG_PLL0CR: usize = 0x00d8;
const CPG_PLL2CR: usize = 0x002c;
const CPG_PLL4CR: usize = 0x01f4;
const CPG_PLLNCR_STC_MASK: u32 = 0x7f000000;
const CPG_RCKCR_CKSEL: u32 = 1 << 15;
const CPG_FRQCRB: usize = 0x00000004;
const CPG_FRQCRB_KICK: u32 = 1 << 31;
const CPG_FRQCRC: usize = 0x000000e0;
const RCKCR_CKSEL: u32 = 1 << 1;

#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct raw_notifier_head { _private: [u8; 0] }
#[repr(C)] pub struct clk_init_data { pub name: *const c_char, pub ops: *const clk_ops, pub flags: u32, pub parent_names: *const *const c_char, pub num_parents: u8 }
#[repr(C)] pub struct clk_rate_request { pub rate: usize, pub min_rate: usize, pub max_rate: usize, pub best_parent_rate: usize }
#[repr(C)] pub struct clk_ops { pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>, pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize, usize) -> i32> }
#[repr(C)] pub struct cpg_pll_clk { pub hw: clk_hw, pub pllcr_reg: *mut u8, pub pllecr_reg: *mut u8, pub fixed_mult: u32, pub pllecr_pllst_mask: u32 }
#[repr(C)] pub struct cpg_z_clk { pub hw: clk_hw, pub reg: *mut u8, pub kick_reg: *mut u8, pub max_rate: usize, pub fixed_div: u32, pub mask: u32 }
#[repr(C)] pub struct clk_div_table { pub val: u32, pub div: u32 }
#[repr(C)] pub struct rcar_gen3_cpg_pll_config { pub extal_div: u32, pub pll1_mult: u32, pub pll1_div: u32, pub pll3_mult: u32, pub pll3_div: u32, pub osc_prediv: u32 }
#[repr(C)] pub struct cpg_core_clk { pub name: *const c_char, pub parent: u32, pub type_: u32, pub div: u32, pub offset: u32 }
#[repr(C)] pub struct cpg_mssr_info { _private: [u8; 0] }
#[repr(C)] pub struct cpg_mssr_pub { pub notifiers: raw_notifier_head, pub base0: *mut u8, pub clks: *mut *mut clk }
#[repr(C)] pub struct cpg_simple_notifier { pub reg: *mut u8 }

extern "C" {
    fn readl(addr: *mut u8) -> u32; fn writel(v: u32, addr: *mut u8); fn cpu_relax();
    fn clk_register(_: *mut c_void, _: *mut clk_hw) -> *mut clk;
    fn clk_register_fixed_factor(_: *mut c_void, _: *const c_char, _: *const c_char, _: u32, _: u32, _: u32) -> *mut clk;
    fn clk_hw_get_parent(_: *mut clk_hw) -> *mut clk_hw; fn clk_hw_get_rate(_: *mut clk_hw) -> usize;
    fn clk_hw_round_rate(_: *mut clk_hw, _: usize) -> usize; fn clk_get_rate(_: *mut clk) -> usize;
    fn __clk_get_name(_: *const clk) -> *const c_char; fn cpg_reg_modify(_: *mut u8, _: u32, _: u32);
    fn cpg_simple_notifier_register(_: *mut raw_notifier_head, _: *mut cpg_simple_notifier);
    fn soc_device_match(_: *const c_void) -> *const c_void; fn pr_debug(_: *const c_char, ...);
    fn cpg_sdh_clk_register(_: *const c_char, _: *mut u8, _: *const c_char, _: *mut raw_notifier_head) -> *mut clk;
    fn cpg_sd_clk_register(_: *const c_char, _: *mut u8, _: *const c_char) -> *mut clk;
    fn cpg_rpc_clk_register(_: *const c_char, _: *mut u8, _: *const c_char, _: *mut raw_notifier_head) -> *mut clk;
    fn cpg_rpcd2_clk_register(_: *const c_char, _: *mut u8, _: *const c_char) -> *mut clk;
}

static mut cpg_pll_config: *const rcar_gen3_cpg_pll_config = core::ptr::null();
static mut cpg_clk_extalr: u32 = 0; static mut cpg_mode: u32 = 0; static mut cpg_quirks: u32 = 0;

unsafe extern "C" fn cpg_pll_clk_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    let p = &*(hw as *mut cpg_pll_clk); let mult = ((readl(p.pllcr_reg) & CPG_PLLNCR_STC_MASK) >> 24) + 1;
    parent_rate * mult as usize * p.fixed_mult as usize
}
unsafe extern "C" fn cpg_pll_clk_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 { let p=&*(hw as *mut cpg_pll_clk); let pr=(*req).best_parent_rate*p.fixed_mult as usize; let min=(((*req).min_rate+pr-1)/pr).max(1); let max=((*req).max_rate/pr).min(128); if max<min{return -22} let m=(((*req).rate+pr/2)/pr).clamp(min,max); (*req).rate=pr*m; 0 }
unsafe extern "C" fn cpg_pll_clk_set_rate(hw: *mut clk_hw, rate: usize, parent_rate: usize) -> i32 {
    let p = &*(hw as *mut cpg_pll_clk); let mult = ((rate + parent_rate * p.fixed_mult as usize / 2) / (parent_rate * p.fixed_mult as usize)).clamp(1,128);
    let mut v = readl(p.pllcr_reg) & !CPG_PLLNCR_STC_MASK; v |= ((mult as u32 - 1) << 24) & CPG_PLLNCR_STC_MASK; writel(v,p.pllcr_reg);
    for _ in 0..1000 { if readl(p.pllecr_reg) & p.pllecr_pllst_mask != 0 { return 0 } cpu_relax(); } -110
}
static CPG_PLL_CLK_OPS: clk_ops = clk_ops { recalc_rate: Some(cpg_pll_clk_recalc_rate), determine_rate: Some(cpg_pll_clk_determine_rate), set_rate: Some(cpg_pll_clk_set_rate) };

unsafe fn cpg_pll_clk_register(name: *const c_char, parent_name: *const c_char, base: *mut u8, mult: u32, offset: usize, index: u32) -> *mut clk {
    let p = Box::into_raw(Box::new(cpg_pll_clk { hw: clk_hw { init: core::ptr::null() }, pllcr_reg: base.add(offset), pllecr_reg: base.add(CPG_PLLECR), fixed_mult: mult, pllecr_pllst_mask: 1 << (8+index) }));
    let init = Box::into_raw(Box::new(clk_init_data { name, ops: &CPG_PLL_CLK_OPS, flags: 0, parent_names: &parent_name, num_parents: 1 })); (*p).hw.init=init;
    let c=clk_register(core::ptr::null_mut(), &mut (*p).hw); if c.is_null() { let _=Box::from_raw(p); } c
}

unsafe extern "C" fn cpg_z_clk_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize { let z=&*(hw as *mut cpg_z_clk); let mult=32-((readl(z.reg)&z.mask).trailing_zeros()); (parent_rate*mult as usize + (32*z.fixed_div as usize)/2)/(32*z.fixed_div as usize) }
unsafe extern "C" fn cpg_z_clk_determine_rate(hw:*mut clk_hw, req:*mut clk_rate_request)->i32 { let z=&*(hw as *mut cpg_z_clk); let rate=(*req).rate.min((*req).max_rate); let pr=if rate<=z.max_rate{z.max_rate}else{rate}; (*req).best_parent_rate=clk_hw_round_rate(clk_hw_get_parent(hw),pr*z.fixed_div as usize); let p=(*req).best_parent_rate/z.fixed_div as usize; let min=(((*req).min_rate*32+p-1)/p).max(1); let max=((*req).max_rate*32/p).min(32); if max<min{return -22} let m=((rate*32+p/2)/p).clamp(min,max); (*req).rate=(p*m+16)/32; 0 }
unsafe extern "C" fn cpg_z_clk_set_rate(hw:*mut clk_hw, rate:usize, parent_rate:usize)->i32 { let z=&*(hw as *mut cpg_z_clk); let mult=((rate*32*z.fixed_div as usize+parent_rate/2)/parent_rate).clamp(1,32); if readl(z.kick_reg)&CPG_FRQCRB_KICK!=0{return -16} cpg_reg_modify(z.reg,z.mask, ((32-mult as u32)<<z.mask.trailing_zeros())); cpg_reg_modify(z.kick_reg,0,CPG_FRQCRB_KICK); for _ in 0..1000 {if readl(z.kick_reg)&CPG_FRQCRB_KICK==0{return 0} cpu_relax()} -110 }
static CPG_Z_CLK_OPS: clk_ops=clk_ops{recalc_rate:Some(cpg_z_clk_recalc_rate),determine_rate:Some(cpg_z_clk_determine_rate),set_rate:Some(cpg_z_clk_set_rate)};

// Clock registration and SoC initialization entry points retain their C ABI and external dependencies.
pub unsafe fn rcar_gen3_cpg_clk_register(_: *mut device, core: *const cpg_core_clk, _: *const cpg_mssr_info, pub_: *mut cpg_mssr_pub) -> *mut clk { let c=&*core; let p=*(*pub_).clks.add((c.parent&0xffff) as usize); if p.is_null(){return core::ptr::null_mut()} let base=(*pub_).base0; let mut mult=1; let mut div=1; match c.type_ { 0=>div=(*cpg_pll_config).extal_div, 1=>return cpg_pll_clk_register(c.name,__clk_get_name(p),base,2,CPG_PLL0CR,0), 2=>{mult=(*cpg_pll_config).pll1_mult;div=(*cpg_pll_config).pll1_div}, 3=>return cpg_pll_clk_register(c.name,__clk_get_name(p),base,2,CPG_PLL2CR,2), 4=>{mult=(*cpg_pll_config).pll3_mult;div=(*cpg_pll_config).pll3_div}, 5=>{mult=((readl(base.add(CPG_PLL4CR))>>24)&0x7f+1)*2}, 6=>return cpg_sdh_clk_register(c.name,base.add(c.offset as usize),__clk_get_name(p),&mut (*pub_).notifiers), 7=>return cpg_sd_clk_register(c.name,base.add(c.offset as usize),__clk_get_name(p)), 8=>{if cpg_mode&(1<<28)!=0{p=*(*pub_).clks.add(cpg_clk_extalr as usize)}}, 9=>{if cpg_mode&(1<<c.offset)!=0{div=c.div&0xffff}else{p=*(*pub_).clks.add((c.parent>>16) as usize);div=c.div>>16}}, 10=>return core::ptr::null_mut(), 11=>return core::ptr::null_mut(), 12=>{if readl(base.add(0x20))&CPG_RCKCR_CKSEL!=0{div=c.div&0xffff}else{p=*(*pub_).clks.add((c.parent>>16) as usize);div=c.div>>16}}, 13=>{let v=(readl(base.add(0x240))>>3)&3;div=match v{0=>5,1=>3,2=>{p=*(*pub_).clks.add((c.parent>>16) as usize);c.div},_=>2}}, 14=>return cpg_rpc_clk_register(c.name,base.add(0x240),__clk_get_name(p),&mut (*pub_).notifiers), 15=>return cpg_rpcd2_clk_register(c.name,base.add(0x240),__clk_get_name(p)), _=>return core::ptr::null_mut()} clk_register_fixed_factor(core::ptr::null_mut(),c.name,__clk_get_name(p),0,mult,div) }
pub unsafe fn rcar_gen3_cpg_init(config:*const rcar_gen3_cpg_pll_config, extalr:u32, mode:u32)->i32 { cpg_pll_config=config;cpg_clk_extalr=extalr;cpg_mode=mode; cpg_quirks=0; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
