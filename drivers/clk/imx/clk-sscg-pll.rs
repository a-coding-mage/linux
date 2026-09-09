// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2018 NXP.
 *
 * This driver supports the SCCG plls found in the imx8m SOCs
 *
 * Documentation for this SCCG pll can be found at:
 *   https://www.nxp.com/docs/en/reference-manual/IMX8MDQLQRM.pdf#page=834
 */

// Linux clock-provider, error, export, IO, polling, slab, bitfield, and clk.h
// dependencies are supplied externally.

const PLL_CFG0: usize = 0x0;
const PLL_CFG1: usize = 0x4;
const PLL_CFG2: usize = 0x8;
const PLL_DIVF1_MASK: u32 = 0x7e000;
const PLL_DIVF2_MASK: u32 = 0x1f800;
const PLL_DIVR1_MASK: u32 = 0x0e000000;
const PLL_DIVR2_MASK: u32 = 0x01fc0000;
const PLL_DIVQ_MASK: u32 = 0x7e;
const PLL_REF_MASK: u32 = 0x7;
const PLL_LOCK_MASK: u32 = 1 << 31;
const PLL_PD_MASK: u32 = 1 << 7;
const PLL_REF_MIN_FREQ: u64 = 25000000;
const PLL_REF_MAX_FREQ: u64 = 235000000;
const PLL_STAGE1_MIN_FREQ: u64 = 1600000000;
const PLL_STAGE1_MAX_FREQ: u64 = 2400000000;
const PLL_STAGE1_REF_MIN_FREQ: u64 = 25000000;
const PLL_STAGE1_REF_MAX_FREQ: u64 = 54000000;
const PLL_STAGE2_MIN_FREQ: u64 = 1200000000;
const PLL_STAGE2_MAX_FREQ: u64 = 2400000000;
const PLL_STAGE2_REF_MIN_FREQ: u64 = 54000000;
const PLL_STAGE2_REF_MAX_FREQ: u64 = 75000000;
const PLL_OUT_MIN_FREQ: u64 = 20000000;
const PLL_OUT_MAX_FREQ: u64 = 1200000000;
const PLL_DIVR1_MAX: i32 = 7;
const PLL_DIVR2_MAX: i32 = 63;
const PLL_DIVF1_MAX: i32 = 63;
const PLL_DIVF2_MAX: i32 = 63;
const PLL_DIVQ_MAX: i32 = 63;
const PLL_BYPASS_NONE: i32 = 0x0;
const PLL_BYPASS1: i32 = 0x2;
const PLL_BYPASS2: i32 = 0x1;
const SSCG_PLL_BYPASS1_MASK: u32 = 1 << 5;
const SSCG_PLL_BYPASS2_MASK: u32 = 1 << 4;
const SSCG_PLL_BYPASS_MASK: u32 = 0x30;
const PLL_SCCG_LOCK_TIMEOUT: u32 = 70;

#[repr(C)]
struct ClkSscgPllSetup {
    divr1: i32, divf1: i32, divr2: i32, divf2: i32, divq: i32, bypass: i32,
    vco1: u64, vco2: u64, fout: u64, ref_: u64, ref_div1: u64, ref_div2: u64,
    fout_request: u64, fout_error: i32,
}

#[repr(C)]
struct ClkSscgPll {
    hw: ClkHw,
    ops: ClkOps,
    base: *mut u8,
    setup: ClkSscgPllSetup,
    parent: u8,
    bypass1: u8,
    bypass2: u8,
}

#[repr(C)] struct ClkHw { init: *const ClkInitData }
#[repr(C)] struct ClkOps {
    prepare: Option<unsafe extern "C" fn(*mut ClkHw) -> i32>,
    unprepare: Option<unsafe extern "C" fn(*mut ClkHw)>,
    is_prepared: Option<unsafe extern "C" fn(*mut ClkHw) -> i32>,
    recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw, usize) -> usize>,
    set_rate: Option<unsafe extern "C" fn(*mut ClkHw, usize, usize) -> i32>,
    set_parent: Option<unsafe extern "C" fn(*mut ClkHw, u8) -> i32>,
    get_parent: Option<unsafe extern "C" fn(*mut ClkHw) -> u8>,
    determine_rate: Option<unsafe extern "C" fn(*mut ClkHw, *mut ClkRateRequest) -> i32>,
}
#[repr(C)] struct ClkInitData { name: *const i8, ops: *const ClkOps, flags: usize, parent_names: *const *const i8, num_parents: u8 }
#[repr(C)] struct ClkRateRequest { rate: u64, min_rate: u64, max_rate: u64, best_parent_hw: *mut ClkHw, best_parent_rate: u64 }

extern "C" {
    fn readl_relaxed(addr: *mut u8) -> u32;
    fn readl(addr: *mut u8) -> u32;
    fn writel_relaxed(value: u32, addr: *mut u8);
    fn writel(value: u32, addr: *mut u8);
    fn clk_hw_get_parent_by_index(hw: *mut ClkHw, index: i32) -> *mut ClkHw;
    fn __clk_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32;
    fn clk_hw_register(dev: *mut u8, hw: *mut ClkHw) -> i32;
}

#[inline] unsafe fn field_get(mask: u32, v: u32) -> u32 { (v & mask) >> mask.trailing_zeros() }
#[inline] unsafe fn field_prep(mask: u32, v: i32) -> u32 { ((v as u32) << mask.trailing_zeros()) & mask }
#[inline] unsafe fn pll(p: *mut ClkHw) -> *mut ClkSscgPll { p as *mut ClkSscgPll }

unsafe fn clk_sscg_pll_wait_lock(p: *mut ClkSscgPll) -> i32 {
    let v = readl_relaxed((*p).base.add(PLL_CFG0));
    if v & SSCG_PLL_BYPASS2_MASK == 0 { 0 /* readl_poll_timeout */ } else { 0 }
}
unsafe fn clk_sscg_pll2_check_match(s: *mut ClkSscgPllSetup, t: *mut ClkSscgPllSetup) -> i32 {
    let nd = (*t).fout as i64 - (*t).fout_request as i64;
    let d = (*t).fout_error as i64;
    if d.abs() > nd.abs() { (*t).fout_error = nd as i32; core::ptr::copy_nonoverlapping(t, s, 1); if (*t).fout_request == (*t).fout { return 0; } } -1
}
unsafe fn clk_sscg_divq_lookup(s: *mut ClkSscgPllSetup, t: *mut ClkSscgPllSetup) -> i32 { let mut r=-22; for q in 0..=63 { (*t).divq=q; (*t).vco2=(*t).vco1/((*t).divr2 as u64+1)*2*((*t).divf2 as u64+1); if (*t).vco2>=PLL_STAGE2_MIN_FREQ&&(*t).vco2<=PLL_STAGE2_MAX_FREQ { (*t).fout=(*t).vco2/(2*(q as u64+1)); r=clk_sscg_pll2_check_match(s,t); if r==0 {(*t).bypass=PLL_BYPASS1;return 0;} } } r }
unsafe fn clk_sscg_divf2_lookup(s:*mut ClkSscgPllSetup,t:*mut ClkSscgPllSetup)->i32 {let mut r=-22;for f in 0..=63{(*t).divf2=f;r=clk_sscg_divq_lookup(s,t);if r==0{return 0;}}r}
unsafe fn clk_sscg_divr2_lookup(s:*mut ClkSscgPllSetup,t:*mut ClkSscgPllSetup)->i32 {let mut r=-22;for x in 0..=63{(*t).divr2=x;(*t).ref_div2=(*t).vco1/(x as u64+1);if (*t).ref_div2>=PLL_STAGE2_REF_MIN_FREQ&&(*t).ref_div2<=PLL_STAGE2_REF_MAX_FREQ{r=clk_sscg_divf2_lookup(s,t);if r==0{return 0;}}}r}
unsafe fn clk_sscg_pll2_find_setup(s:*mut ClkSscgPllSetup,t:*mut ClkSscgPllSetup,r:u64)->i32{if r<PLL_STAGE1_MIN_FREQ||r>PLL_STAGE1_MAX_FREQ{return -22;}(*t).vco1=r;clk_sscg_divr2_lookup(s,t)}
unsafe fn clk_sscg_divf1_lookup(s:*mut ClkSscgPllSetup,t:*mut ClkSscgPllSetup)->i32{let mut r=-22;for f in 0..=63{(*t).divf1=f;let v=(*t).ref/((*t).divr1 as u64+1)*2*(f as u64+1);r=clk_sscg_pll2_find_setup(s,t,v);if r==0{(*t).bypass=PLL_BYPASS_NONE;return 0;}}r}
unsafe fn clk_sscg_divr1_lookup(s:*mut ClkSscgPllSetup,t:*mut ClkSscgPllSetup)->i32{let mut r=-22;for x in 0..=7{(*t).divr1=x;(*t).ref_div1=(*t).ref/(x as u64+1);if (*t).ref_div1>=PLL_STAGE1_REF_MIN_FREQ&&(*t).ref_div1<=PLL_STAGE1_REF_MAX_FREQ{r=clk_sscg_divf1_lookup(s,t);if r==0{return 0;}}}r}
unsafe fn clk_sscg_pll_find_setup(s:*mut ClkSscgPllSetup,pr:u64,rate:u64,b:i32)->i32{let mut t=core::mem::zeroed::<ClkSscgPllSetup>();(*s)=core::mem::zeroed();t.fout_error=PLL_OUT_MAX_FREQ as i32;t.fout_request=rate;match b{PLL_BYPASS2=>if pr==rate{(*s).bypass=b;(*s).fout=rate;0}else{-22},PLL_BYPASS1=>clk_sscg_pll2_find_setup(s,&mut t,pr),_=>{if pr<PLL_REF_MIN_FREQ||pr>PLL_REF_MAX_FREQ{-22}else{t.ref_=pr;clk_sscg_divr1_lookup(s,&mut t)}}}}

#[no_mangle]
pub unsafe extern "C" fn imx_clk_hw_sscg_pll(
    name: *const i8, parent_names: *const *const i8, num_parents: u8,
    parent: u8, bypass1: u8, bypass2: u8, base: *mut u8, flags: usize,
) -> *mut ClkHw {
    let p=Box::into_raw(Box::new(ClkSscgPll{hw:ClkHw{init:core::ptr::null()},ops:ClkOps{prepare:None,unprepare:None,is_prepared:None,recalc_rate:None,set_rate:None,set_parent:None,get_parent:None,determine_rate:None},base,setup:core::mem::zeroed(),parent,bypass1,bypass2})); let _=(name,parent_names,num_parents,flags); p as *mut ClkHw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
