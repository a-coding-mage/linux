// SPDX-License-Identifier: GPL-2.0
/*
 * R-Car Gen4 Clock Pulse Generator
 *
 * Copyright (C) 2021 Renesas Electronics Corp.
 *
 * Based on rcar-gen3-cpg.c
 *
 * Copyright (C) 2015-2018 Glider bvba
 * Copyright (C) 2019 Renesas Electronics Corp.
 */

// External Linux/kernel declarations and macros used below are supplied by
// the surrounding translation unit.

static mut cpg_pll_config: *const rcar_gen4_cpg_pll_config = core::ptr::null();
static mut cpg_clk_extalr: u32 = 0;
static mut cpg_mode: u32 = 0;

const CPG_PLLECR: usize = 0x0820;
const fn cpg_pllecr_pllst(n: usize) -> u32 { 1u32 << (8 + if n < 3 { n - 1 } else if n > 3 { n + 1 } else { n }) }
const CPG_PLL1CR0: usize = 0x830;
const CPG_PLL1CR1: usize = 0x8b0;
const CPG_PLL2CR0: usize = 0x834;
const CPG_PLL2CR1: usize = 0x8b8;
const CPG_PLL3CR0: usize = 0x83c;
const CPG_PLL3CR1: usize = 0x8c0;
const CPG_PLL4CR0: usize = 0x844;
const CPG_PLL4CR1: usize = 0x8c8;
const CPG_PLL6CR0: usize = 0x84c;
const CPG_PLL6CR1: usize = 0x8d8;
const CPG_PLLxCR0_KICK: u32 = 1 << 31;
const CPG_PLLxCR0_SSMODE: u32 = 0x7 << 16;
const CPG_PLLxCR0_SSMODE_FM: u32 = 1 << 18;
const CPG_PLLxCR0_SSMODE_DITH: u32 = 1 << 17;
const CPG_PLLxCR0_SSMODE_CENT: u32 = 1 << 16;
const CPG_PLLxCR0_SSFREQ: u32 = 0x7f << 8;
const CPG_PLLxCR0_SSDEPT: u32 = 0x7f;
const CPG_PLLxCR0_NI8: u32 = 0xff << 20;
const CPG_PLLxCR1_NF25: u32 = 0x1ffffff;
const CPG_PLLxCR0_NI9: u32 = 0x1ff << 20;
const CPG_PLLxCR1_NF24: u32 = 0xffffff;
const CPG_PLLxCR_STC: u32 = 0x7f << 24;
const CPG_RPCCKCR: usize = 0x874;
const CPG_SD0CKCR1: usize = 0x8a4;
const CPG_SD0CKCR1_SDSRC_SEL: u32 = 0x3 << 29;

#[repr(C)]
struct cpg_pll_clk { hw: clk_hw, pllcr0_reg: *mut u8, pllcr1_reg: *mut u8, pllecr_reg: *mut u8, pllecr_pllst_mask: u32 }

unsafe fn cpg_pll_8_25_clk_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let pll_clk = &*(hw as *const cpg_pll_clk);
    let cr0 = readl(pll_clk.pllcr0_reg);
    let ni = (field_get(CPG_PLLxCR0_NI8, cr0) + 1) * 2;
    let mut rate = parent_rate * ni as u64;
    if cr0 & CPG_PLLxCR0_SSMODE_FM != 0 { rate += (parent_rate * field_get(CPG_PLLxCR1_NF25, readl(pll_clk.pllcr1_reg)) as u64) >> 24; }
    rate
}

unsafe fn cpg_pll_8_25_clk_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let pll_clk = &*(hw as *const cpg_pll_clk); let req = &mut *req; let cr0 = readl(pll_clk.pllcr0_reg);
    let prate = req.best_parent_rate * 2;
    let min_mult = core::cmp::max(req.min_rate / prate, 1); let max_mult = core::cmp::min(req.max_rate / prate, 256);
    if max_mult < min_mult { return -22; }
    let (ni, nf) = if cr0 & CPG_PLLxCR0_SSMODE_FM != 0 { let mut ni = req.rate / prate; if ni < min_mult {(min_mult,0)} else {ni=core::cmp::min(ni,max_mult); (ni, ((req.rate - prate*ni)<<24)/req.best_parent_rate)} } else {(core::cmp::min(core::cmp::max((req.rate+prate/2)/prate,min_mult),max_mult),0)};
    req.rate = prate*ni + (req.best_parent_rate*nf)>>24; 0
}

unsafe fn cpg_pll_8_25_clk_set_rate(hw: *mut clk_hw, rate: u64, parent_rate: u64) -> i32 {
    let pll_clk = &*(hw as *const cpg_pll_clk); let prate=parent_rate*2; let cr0=readl(pll_clk.pllcr0_reg);
    let (ni,nf) = if cr0 & CPG_PLLxCR0_SSMODE_FM != 0 { let mut ni=rate/prate; if ni<1 {(1,0)} else {ni=core::cmp::min(ni,256); (ni,((rate-prate*ni)<<24)/parent_rate)} } else {(((rate+prate/2)/prate).clamp(1,256),0)};
    if readl(pll_clk.pllcr0_reg)&CPG_PLLxCR0_KICK != 0{return -16;}
    cpg_reg_modify(pll_clk.pllcr0_reg, CPG_PLLxCR0_NI8, field_prep(CPG_PLLxCR0_NI8,(ni-1) as u32));
    if cr0&CPG_PLLxCR0_SSMODE_FM!=0 {cpg_reg_modify(pll_clk.pllcr1_reg,CPG_PLLxCR1_NF25,field_prep(CPG_PLLxCR1_NF25,nf as u32));}
    cpg_reg_modify(pll_clk.pllcr0_reg,0,CPG_PLLxCR0_KICK); readl_poll_timeout(pll_clk.pllecr_reg, |v| v&pll_clk.pllecr_pllst_mask!=0,0,1000)
}

unsafe fn cpg_pll_9_24_clk_recalc_rate(hw:*mut clk_hw,parent_rate:u64)->u64 { let p=&*(hw as *const cpg_pll_clk); let cr0=readl(p.pllcr0_reg); let ni=field_get(CPG_PLLxCR0_NI9,cr0)+1; let mut rate=parent_rate*ni as u64; if cr0&CPG_PLLxCR0_SSMODE_FM!=0 {rate+=(parent_rate*field_get(CPG_PLLxCR1_NF24,readl(p.pllcr1_reg)) as u64)>>24;} else {rate*=2;} rate }

#[repr(C)] struct cpg_z_clk { hw: clk_hw, reg:*mut u8, kick_reg:*mut u8, max_rate:u64, fixed_div:u32, mask:u32 }
unsafe fn cpg_z_clk_recalc_rate(hw:*mut clk_hw,parent_rate:u64)->u64 {let z=&*(hw as *const cpg_z_clk); let mult=32-field_get(z.mask,readl(z.reg)); (parent_rate*mult as u64+ (32*z.fixed_div as u64)/2)/(32*z.fixed_div as u64)}

// The remaining registration and framework glue retain the C interfaces and
// depend on the corresponding kernel clock types and helpers supplied by
// other translated files.
extern "C" {
    fn cpg_pll_clk_register(name:*const i8,parent_name:*const i8,base:*mut u8,index:u32,ops:*const clk_ops)->*mut clk;
    fn cpg_z_clk_register(name:*const i8,parent_name:*const i8,reg:*mut u8,div:u32,offset:u32)->*mut clk;
    fn rcar_gen4_cpg_clk_register(dev:*mut device,core:*const cpg_core_clk,info:*const cpg_mssr_info,pub_:*mut cpg_mssr_pub)->*mut clk;
}

unsafe fn rcar_gen4_cpg_init(config:*const rcar_gen4_cpg_pll_config, clk_extalr:u32, mode:u32)->i32 { cpg_pll_config=config; cpg_clk_extalr=clk_extalr; cpg_mode=mode; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
