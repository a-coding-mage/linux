// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Google, Inc.
 */

// C dependencies: linux/clk-provider.h, linux/io.h, linux/kernel.h,
// linux/printk.h, linux/slab.h, and "clk.h".

const PLL_STATUS: u32 = 0x0;
const PLL_STATUS_LOCK: u32 = 1 << 0;
const PLL_CTRL1: u32 = 0x4;
const PLL_CTRL1_REFDIV_SHIFT: u32 = 0;
const PLL_CTRL1_REFDIV_MASK: u32 = 0x3f;
const PLL_CTRL1_FBDIV_SHIFT: u32 = 6;
const PLL_CTRL1_FBDIV_MASK: u32 = 0xfff;
const PLL_INT_CTRL1_POSTDIV1_SHIFT: u32 = 18;
const PLL_INT_CTRL1_POSTDIV1_MASK: u32 = 0x7;
const PLL_INT_CTRL1_POSTDIV2_SHIFT: u32 = 21;
const PLL_INT_CTRL1_POSTDIV2_MASK: u32 = 0x7;
const PLL_INT_CTRL1_PD: u32 = 1 << 24;
const PLL_INT_CTRL1_DSMPD: u32 = 1 << 25;
const PLL_INT_CTRL1_FOUTPOSTDIVPD: u32 = 1 << 26;
const PLL_INT_CTRL1_FOUTVCOPD: u32 = 1 << 27;
const PLL_CTRL2: u32 = 0x8;
const PLL_FRAC_CTRL2_FRAC_SHIFT: u32 = 0;
const PLL_FRAC_CTRL2_FRAC_MASK: u32 = 0xffffff;
const PLL_FRAC_CTRL2_POSTDIV1_SHIFT: u32 = 24;
const PLL_FRAC_CTRL2_POSTDIV1_MASK: u32 = 0x7;
const PLL_FRAC_CTRL2_POSTDIV2_SHIFT: u32 = 27;
const PLL_FRAC_CTRL2_POSTDIV2_MASK: u32 = 0x7;
const PLL_INT_CTRL2_BYPASS: u32 = 1 << 28;
const PLL_CTRL3: u32 = 0xc;
const PLL_FRAC_CTRL3_PD: u32 = 1 << 0;
const PLL_FRAC_CTRL3_DACPD: u32 = 1 << 1;
const PLL_FRAC_CTRL3_DSMPD: u32 = 1 << 2;
const PLL_FRAC_CTRL3_FOUTPOSTDIVPD: u32 = 1 << 3;
const PLL_FRAC_CTRL3_FOUT4PHASEPD: u32 = 1 << 4;
const PLL_FRAC_CTRL3_FOUTVCOPD: u32 = 1 << 5;
const PLL_CTRL4: u32 = 0x10;
const PLL_FRAC_CTRL4_BYPASS: u32 = 1 << 28;
const MIN_PFD: u64 = 9600000;
const MIN_VCO_LA: u64 = 400000000;
const MAX_VCO_LA: u64 = 1600000000;
const MIN_VCO_FRAC_FRAC: u64 = 600000000;
const MAX_VCO_FRAC_FRAC: u64 = 2400000000;
const MIN_OUTPUT_LA: u64 = 8000000;
const MAX_OUTPUT_LA: u64 = 1600000000;
const MIN_OUTPUT_FRAC: u64 = 12000000;
const MAX_OUTPUT_FRAC: u64 = 1600000000;

#[repr(C)]
pub enum pll_mode { PLL_MODE_FRAC, PLL_MODE_INT }

#[repr(C)]
pub struct pistachio_clk_pll {
    pub hw: clk_hw,
    pub base: *mut core::ffi::c_void,
    pub rates: *mut pistachio_pll_rate_table,
    pub nr_rates: u32,
}

#[inline]
unsafe fn pll_readl(pll: *mut pistachio_clk_pll, reg: u32) -> u32 {
    readl((*pll).base.add(reg as usize) as *const core::ffi::c_void)
}
#[inline]
unsafe fn pll_writel(pll: *mut pistachio_clk_pll, val: u32, reg: u32) {
    writel(val, (*pll).base.add(reg as usize));
}
#[inline]
unsafe fn pll_lock(pll: *mut pistachio_clk_pll) {
    while pll_readl(pll, PLL_STATUS) & PLL_STATUS_LOCK == 0 { cpu_relax(); }
}
#[inline]
fn do_div_round_closest(mut dividend: u64, divisor: u64) -> u64 {
    dividend = dividend.wrapping_add(divisor / 2); dividend / divisor
}
#[inline]
unsafe fn to_pistachio_pll(hw: *mut clk_hw) -> *mut pistachio_clk_pll {
    (hw as *mut u8).sub(core::mem::offset_of!(pistachio_clk_pll, hw)) as *mut pistachio_clk_pll
}
#[inline]
unsafe fn pll_frac_get_mode(hw: *mut clk_hw) -> pll_mode {
    if pll_readl(to_pistachio_pll(hw), PLL_CTRL3) & PLL_FRAC_CTRL3_DSMPD != 0 { pll_mode::PLL_MODE_INT } else { pll_mode::PLL_MODE_FRAC }
}
#[inline]
unsafe fn pll_frac_set_mode(hw: *mut clk_hw, mode: pll_mode) {
    let pll = to_pistachio_pll(hw); let mut val = pll_readl(pll, PLL_CTRL3);
    if matches!(mode, pll_mode::PLL_MODE_INT) { val |= PLL_FRAC_CTRL3_DSMPD | PLL_FRAC_CTRL3_DACPD; } else { val &= !(PLL_FRAC_CTRL3_DSMPD | PLL_FRAC_CTRL3_DACPD); }
    pll_writel(pll, val, PLL_CTRL3);
}

// The remaining declarations and operations retain the kernel clock-provider
// types and callbacks supplied by the included headers.
extern "C" {
    fn readl(addr: *const core::ffi::c_void) -> u32;
    fn writel(val: u32, addr: *mut core::ffi::c_void);
    fn cpu_relax();
}
unsafe fn pll_get_params(pll: *mut pistachio_clk_pll, fref: usize, fout: usize) -> *mut pistachio_pll_rate_table {
    for i in 0..(*pll).nr_rates as usize { let r = (*pll).rates.add(i); if (*r).fref == fref && (*r).fout == fout { return r; } } core::ptr::null_mut()
}

unsafe fn pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let pll = to_pistachio_pll(hw);
    for i in 0..(*pll).nr_rates as usize { let r = (*pll).rates.add(i); if i > 0 && (*r).fref == (*req).best_parent_rate && (*r).fout <= (*req).rate { (*req).rate = (*pll).rates.add(i-1).read().fout; return 0; } }
    (*req).rate = (*pll).rates.read().fout; 0
}

unsafe fn pll_gf40lp_frac_enable(hw: *mut clk_hw) -> i32 { let p=to_pistachio_pll(hw); let mut v=pll_readl(p,PLL_CTRL3); v &= !(PLL_FRAC_CTRL3_PD|PLL_FRAC_CTRL3_FOUTPOSTDIVPD|PLL_FRAC_CTRL3_FOUT4PHASEPD|PLL_FRAC_CTRL3_FOUTVCOPD); pll_writel(p,v,PLL_CTRL3); v=pll_readl(p,PLL_CTRL4)&!PLL_FRAC_CTRL4_BYPASS; pll_writel(p,v,PLL_CTRL4); pll_lock(p); 0 }
unsafe fn pll_gf40lp_frac_disable(hw:*mut clk_hw){let p=to_pistachio_pll(hw);pll_writel(p,pll_readl(p,PLL_CTRL3)|PLL_FRAC_CTRL3_PD,PLL_CTRL3)}
unsafe fn pll_gf40lp_frac_is_enabled(hw:*mut clk_hw)->i32{if pll_readl(to_pistachio_pll(hw),PLL_CTRL3)&PLL_FRAC_CTRL3_PD==0{1}else{0}}
unsafe fn pll_gf40lp_frac_recalc_rate(hw:*mut clk_hw,parent:u64)->u64{let p=to_pistachio_pll(hw);let a=pll_readl(p,PLL_CTRL1);let pre=((a>>PLL_CTRL1_REFDIV_SHIFT)&PLL_CTRL1_REFDIV_MASK)as u64;let fb=((a>>PLL_CTRL1_FBDIV_SHIFT)&PLL_CTRL1_FBDIV_MASK)as u64;let b=pll_readl(p,PLL_CTRL2);let d1=((b>>24)&7)as u64;let d2=((b>>27)&7)as u64;let f=(b&0xffffff)as u64;let mut r=parent;if matches!(pll_frac_get_mode(hw),pll_mode::PLL_MODE_FRAC){r=r.wrapping_mul((fb<<24)+f)}else{r=r.wrapping_mul(fb<<24)};do_div_round_closest(r,(pre*d1*d2)<<24)}

unsafe fn pll_gf40lp_frac_set_rate(hw:*mut clk_hw,rate:u64,parent:u64)->i32{let p=to_pistachio_pll(hw);if rate<MIN_OUTPUT_FRAC||rate>MAX_OUTPUT_FRAC{return -22}let q=pll_get_params(p,parent as usize,rate as usize);if q.is_null()||(*q).refdiv==0{return -22}let mut v=pll_readl(p,PLL_CTRL1);v&=!((PLL_CTRL1_REFDIV_MASK<<PLL_CTRL1_REFDIV_SHIFT)|(PLL_CTRL1_FBDIV_MASK<<PLL_CTRL1_FBDIV_SHIFT));v|=(*q).refdiv<<PLL_CTRL1_REFDIV_SHIFT|(*q).fbdiv<<PLL_CTRL1_FBDIV_SHIFT;pll_writel(p,v,PLL_CTRL1);v=pll_readl(p,PLL_CTRL2);v&=!((PLL_FRAC_CTRL2_FRAC_MASK<<PLL_FRAC_CTRL2_FRAC_SHIFT)|(PLL_FRAC_CTRL2_POSTDIV1_MASK<<24)|(PLL_FRAC_CTRL2_POSTDIV2_MASK<<27));v|=(*q).frac|(*q).postdiv1<<24|(*q).postdiv2<<27;pll_writel(p,v,PLL_CTRL2);pll_frac_set_mode(hw,if (*q).frac!=0{pll_mode::PLL_MODE_FRAC}else{pll_mode::PLL_MODE_INT});if pll_gf40lp_frac_is_enabled(hw)!=0{pll_lock(p)};0}

unsafe fn pll_gf40lp_laint_set_rate(hw:*mut clk_hw,rate:u64,parent:u64)->i32{let p=to_pistachio_pll(hw);if rate<MIN_OUTPUT_LA||rate>MAX_OUTPUT_LA{return -22}let q=pll_get_params(p,parent as usize,rate as usize);if q.is_null()||(*q).refdiv==0{return -22}let mut v=pll_readl(p,PLL_CTRL1);v&=!((PLL_CTRL1_REFDIV_MASK<<0)|(PLL_CTRL1_FBDIV_MASK<<6)|(7<<18)|(7<<21));v|=(*q).refdiv|(*q).fbdiv<<6|(*q).postdiv1<<18|(*q).postdiv2<<21;pll_writel(p,v,PLL_CTRL1);if pll_gf40lp_laint_is_enabled(hw)!=0{pll_lock(p)};0}

unsafe fn pll_gf40lp_laint_enable(hw:*mut clk_hw)->i32{let p=to_pistachio_pll(hw);let mut v=pll_readl(p,PLL_CTRL1)&!(PLL_INT_CTRL1_PD|PLL_INT_CTRL1_FOUTPOSTDIVPD|PLL_INT_CTRL1_FOUTVCOPD);pll_writel(p,v,PLL_CTRL1);v=pll_readl(p,PLL_CTRL2)&!PLL_INT_CTRL2_BYPASS;pll_writel(p,v,PLL_CTRL2);pll_lock(p);0}
unsafe fn pll_gf40lp_laint_disable(hw:*mut clk_hw){let p=to_pistachio_pll(hw);pll_writel(p,pll_readl(p,PLL_CTRL1)|PLL_INT_CTRL1_PD,PLL_CTRL1)}
unsafe fn pll_gf40lp_laint_is_enabled(hw:*mut clk_hw)->i32{if pll_readl(to_pistachio_pll(hw),PLL_CTRL1)&PLL_INT_CTRL1_PD==0{1}else{0}}
unsafe fn pll_gf40lp_laint_recalc_rate(hw:*mut clk_hw,parent:u64)->u64{let p=to_pistachio_pll(hw);let v=pll_readl(p,PLL_CTRL1);let a=((v>>0)&0x3f)as u64;let b=((v>>6)&0xfff)as u64;let c=((v>>18)&7)as u64;let d=((v>>21)&7)as u64;do_div_round_closest(parent.wrapping_mul(b),a*c*d)}

// External kernel types/functions are intentionally referenced, not reimplemented.
#[allow(dead_code)]
pub unsafe fn pistachio_clk_register_pll(p: *mut pistachio_clk_provider, pll: *mut pistachio_pll, num: u32) {
    for i in 0..num as usize {
        let item = pll.add(i);
        let clk = pll_register((*item).name, (*item).parent, 0, (*p).base.add((*item).reg_base as usize), (*item).type_, (*item).rates, (*item).nr_rates);
        (*p).clk_data.clks[(*item).id as usize] = clk;
    }
}

// External declarations corresponding to symbols from clk.h and the kernel.
extern "C" {
    fn pll_register(name: *const i8, parent: *const i8, flags: u64, base: *mut core::ffi::c_void, type_: pistachio_pll_type, rates: *mut pistachio_pll_rate_table, nr_rates: u32) -> *mut clk;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
