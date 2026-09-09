// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014 MediaTek Inc.
 * Author: James Liao <jamesjj.liao@mediatek.com>
 */

// Kernel dependencies supplied externally.

const MHZ: u32 = 1000 * 1000;
const REG_CON0: usize = 0;
const REG_CON1: usize = 4;
const CON0_BASE_EN: u32 = 1 << 0;
const CON0_PWR_ON: u32 = 1 << 0;
const CON0_ISO_EN: u32 = 1 << 1;
const PCW_CHG_BIT: u32 = 31;
const AUDPLL_TUNER_EN: u32 = 1 << 31;
const INTEGER_BITS: i32 = 7;

pub unsafe fn mtk_pll_is_prepared(hw: *mut clk_hw) -> i32 {
    let pll = to_mtk_clk_pll(hw);
    (readl((*pll).en_addr) & (1u32 << (*pll).data.pll_en_bit) != 0) as i32
}

unsafe fn mtk_pll_fenc_is_prepared(hw: *mut clk_hw) -> i32 {
    let pll = to_mtk_clk_pll(hw);
    ((readl((*pll).fenc_addr) & (1u32 << (*pll).data.fenc_sta_bit)) != 0) as i32
}

unsafe fn __mtk_pll_recalc_rate(pll: *mut mtk_clk_pll, fin: u32, pcw: u32, postdiv: i32) -> usize {
    let pcwbits = (*pll).data.pcwbits as i32;
    let pcwfbits = if pcwbits > (if (*pll).data.pcwibits != 0 { (*pll).data.pcwibits as i32 } else { INTEGER_BITS }) { pcwbits - (if (*pll).data.pcwibits != 0 { (*pll).data.pcwibits as i32 } else { INTEGER_BITS }) } else { 0 };
    let mut vco = fin as u64 * pcw as u64;
    let mut c: u8 = 0;
    if pcwfbits != 0 && (vco & ((1u64 << pcwfbits) - 1)) != 0 { c = 1; }
    vco >>= pcwfbits;
    if c != 0 { vco += 1; }
    ((vco as usize) + postdiv as usize - 1) / postdiv as usize
}

unsafe fn __mtk_pll_tuner_enable(pll: *mut mtk_clk_pll) {
    let mut r;
    if !(*pll).tuner_en_addr.is_null() { r = readl((*pll).tuner_en_addr) | (1u32 << (*pll).data.tuner_en_bit); writel(r, (*pll).tuner_en_addr); }
    else if !(*pll).tuner_addr.is_null() { r = readl((*pll).tuner_addr) | AUDPLL_TUNER_EN; writel(r, (*pll).tuner_addr); }
}

unsafe fn __mtk_pll_tuner_disable(pll: *mut mtk_clk_pll) {
    let mut r;
    if !(*pll).tuner_en_addr.is_null() { r = readl((*pll).tuner_en_addr) & !(1u32 << (*pll).data.tuner_en_bit); writel(r, (*pll).tuner_en_addr); }
    else if !(*pll).tuner_addr.is_null() { r = readl((*pll).tuner_addr) & !AUDPLL_TUNER_EN; writel(r, (*pll).tuner_addr); }
}

unsafe fn mtk_pll_set_rate_regs(pll: *mut mtk_clk_pll, pcw: u32, postdiv: i32) {
    __mtk_pll_tuner_disable(pll);
    let mut val = readl((*pll).pd_addr);
    val &= !(POSTDIV_MASK << (*pll).data.pd_shift);
    val |= (ffs(postdiv as u32) - 1) << (*pll).data.pd_shift;
    if (*pll).pd_addr != (*pll).pcw_addr { writel(val, (*pll).pd_addr); val = readl((*pll).pcw_addr); }
    val &= !(((1u32 << ((*pll).data.pcw_shift + (*pll).data.pcwbits)) - 1) & !((1u32 << (*pll).data.pcw_shift) - 1));
    val |= pcw << (*pll).data.pcw_shift;
    writel(val, (*pll).pcw_addr);
    let chg = readl((*pll).pcw_chg_addr) | (1u32 << if (*pll).data.pcw_chg_bit != 0 { (*pll).data.pcw_chg_bit } else { PCW_CHG_BIT });
    writel(chg, (*pll).pcw_chg_addr);
    if !(*pll).tuner_addr.is_null() { writel(val + 1, (*pll).tuner_addr); }
    __mtk_pll_tuner_enable(pll); udelay(20);
}

/* mtk_pll_calc_values - calculate good values for a given input frequency. */
pub unsafe fn mtk_pll_calc_values(pll: *mut mtk_clk_pll, pcw: *mut u32, postdiv: *mut u32, mut freq: u32, fin: u32) {
    let fmin = if (*pll).data.fmin != 0 { (*pll).data.fmin } else { 1000 * MHZ };
    let div_table = (*pll).data.div_table;
    if freq > (*pll).data.fmax { freq = (*pll).data.fmax; }
    let mut val = 0u32;
    if !div_table.is_null() {
        if freq > (*div_table).freq { freq = (*div_table).freq; }
        while (*div_table.add((val + 1) as usize)).freq != 0 { if freq > (*div_table.add((val + 1) as usize)).freq { break; } val += 1; }
        *postdiv = 1 << val;
    } else { while val < 5 { *postdiv = 1 << val; if freq as u64 * *postdiv as u64 >= fmin as u64 { break; } val += 1; } }
    let ibits = if (*pll).data.pcwibits != 0 { (*pll).data.pcwibits as i32 } else { INTEGER_BITS };
    let mut v = ((freq as u64) << val) << ((*pll).data.pcwbits as i32 - ibits);
    v /= fin as u64;
    *pcw = v as u32;
}

pub unsafe fn mtk_pll_set_rate(hw: *mut clk_hw, rate: usize, parent_rate: usize) -> i32 { let pll = to_mtk_clk_pll(hw); let mut pcw = 0; let mut postdiv = 0; mtk_pll_calc_values(pll, &mut pcw, &mut postdiv, rate as u32, parent_rate as u32); mtk_pll_set_rate_regs(pll, pcw, postdiv as i32); 0 }
pub unsafe fn mtk_pll_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize { let pll = to_mtk_clk_pll(hw); let postdiv = 1 << ((readl((*pll).pd_addr) >> (*pll).data.pd_shift) & POSTDIV_MASK); let pcw = (readl((*pll).pcw_addr) >> (*pll).data.pcw_shift) & ((1 << (*pll).data.pcwbits) - 1); __mtk_pll_recalc_rate(pll, parent_rate as u32, pcw, postdiv as i32) }
pub unsafe fn mtk_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 { let pll = to_mtk_clk_pll(hw); let mut pcw=0; let mut postdiv=0; mtk_pll_calc_values(pll,&mut pcw,&mut postdiv,(*req).rate as u32,(*req).best_parent_rate as u32); (*req).rate=__mtk_pll_recalc_rate(pll,(*req).best_parent_rate as u32,pcw,postdiv as i32); 0 }

pub unsafe fn mtk_pll_prepare(hw: *mut clk_hw) -> i32 { let pll=to_mtk_clk_pll(hw); let mut r=readl((*pll).pwr_addr)|CON0_PWR_ON; writel(r,(*pll).pwr_addr); udelay(1); r=readl((*pll).pwr_addr)&!CON0_ISO_EN; writel(r,(*pll).pwr_addr); udelay(1); r=readl((*pll).en_addr)|(1<<(*pll).data.pll_en_bit); writel(r,(*pll).en_addr); if (*pll).data.en_mask!=0 { r=readl((*pll).base_addr.add(REG_CON0))|(*pll).data.en_mask; writel(r,(*pll).base_addr.add(REG_CON0)); } __mtk_pll_tuner_enable(pll); udelay(20); if (*pll).data.flags&HAVE_RST_BAR!=0 { r=readl((*pll).base_addr.add(REG_CON0))|(*pll).data.rst_bar_mask; writel(r,(*pll).base_addr.add(REG_CON0)); } 0 }
pub unsafe fn mtk_pll_unprepare(hw:*mut clk_hw){let pll=to_mtk_clk_pll(hw);let mut r;if (*pll).data.flags&HAVE_RST_BAR!=0{r=readl((*pll).base_addr.add(REG_CON0))&!(*pll).data.rst_bar_mask;writel(r,(*pll).base_addr.add(REG_CON0));}__mtk_pll_tuner_disable(pll);if (*pll).data.en_mask!=0{r=readl((*pll).base_addr.add(REG_CON0))&!(*pll).data.en_mask;writel(r,(*pll).base_addr.add(REG_CON0));}r=readl((*pll).en_addr)&!(1<<(*pll).data.pll_en_bit);writel(r,(*pll).en_addr);r=readl((*pll).pwr_addr)|CON0_ISO_EN;writel(r,(*pll).pwr_addr);r=readl((*pll).pwr_addr)&!CON0_PWR_ON;writel(r,(*pll).pwr_addr);}

unsafe fn mtk_pll_prepare_setclr(hw:*mut clk_hw)->i32{let pll=to_mtk_clk_pll(hw);writel(1<<(*pll).data.pll_en_bit,(*pll).en_set_addr);udelay(20);0}
unsafe fn mtk_pll_unprepare_setclr(hw:*mut clk_hw){let pll=to_mtk_clk_pll(hw);writel(1<<(*pll).data.pll_en_bit,(*pll).en_clr_addr);}

// The remaining registration and teardown routines retain the kernel ABI and
// external helper calls; their declarations are supplied by the surrounding
// translation unit.
extern "C" { pub fn mtk_clk_register_pll_ops(pll:*mut mtk_clk_pll,data:*const mtk_pll_data,base:*mut u8,pll_ops:*const clk_ops)->*mut clk_hw; pub fn mtk_clk_register_pll(dev:*mut device,data:*const mtk_pll_data,base:*mut u8)->*mut clk_hw; pub fn mtk_clk_unregister_pll(hw:*mut clk_hw); pub fn mtk_clk_register_plls(dev:*mut device,plls:*const mtk_pll_data,num_plls:i32,clk_data:*mut clk_hw_onecell_data)->i32; pub fn mtk_clk_unregister_plls(plls:*const mtk_pll_data,num_plls:i32,clk_data:*mut clk_hw_onecell_data); pub fn mtk_clk_pll_get_base(hw:*mut clk_hw,data:*const mtk_pll_data)->*mut u8; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
