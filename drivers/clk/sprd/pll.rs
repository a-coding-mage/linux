// SPDX-License-Identifier: GPL-2.0
//
// Spreadtrum pll clock driver
//
// Copyright (C) 2015~2017 Spreadtrum, Inc.
// Author: Chunyan Zhang <chunyan.zhang@spreadtrum.com>

// Linux kernel dependencies are supplied by the surrounding crate.

const CLK_PLL_1M: u64 = 1_000_000;
const CLK_PLL_10M: u64 = CLK_PLL_1M * 10;

#[inline]
unsafe fn pindex(pll: *const sprd_pll, member: usize) -> usize {
    (*pll).factors[member].shift as usize / (8 * core::mem::size_of::<u32>())
}

#[inline]
unsafe fn pshift(pll: *const sprd_pll, member: usize) -> u32 {
    (*pll).factors[member].shift % (8 * core::mem::size_of::<u32>() as u32)
}

#[inline]
unsafe fn pwidth(pll: *const sprd_pll, member: usize) -> u32 {
    (*pll).factors[member].width
}

#[inline]
unsafe fn pmask(pll: *const sprd_pll, member: usize) -> u32 {
    let width = pwidth(pll, member);
    if width != 0 {
        u32::MAX >> (32 - width) << pshift(pll, member)
    } else {
        0
    }
}

#[inline]
unsafe fn pinternal(pll: *const sprd_pll, cfg: *const u32, member: usize) -> u32 {
    *cfg.add(pindex(pll, member)) & pmask(pll, member)
}

#[inline]
unsafe fn pinternal_val(pll: *const sprd_pll, cfg: *const u32, member: usize) -> u32 {
    pinternal(pll, cfg, member) >> pshift(pll, member)
}

#[inline]
unsafe fn sprd_pll_read(pll: *const sprd_pll, index: u8) -> u32 {
    let common = &(*pll).common;
    let mut val = 0u32;
    if index as u32 >= (*pll).regs_num {
        return 0;
    }
    regmap_read(common.regmap, common.reg + index as u32 * 4, &mut val);
    val
}

#[inline]
unsafe fn sprd_pll_write(pll: *const sprd_pll, index: u8, msk: u32, val: u32) {
    let common = &(*pll).common;
    if index as u32 >= (*pll).regs_num {
        return;
    }
    let offset = common.reg + index as u32 * 4;
    let mut reg = 0u32;
    let ret = regmap_read(common.regmap, offset, &mut reg);
    if ret == 0 {
        regmap_write(common.regmap, offset, (reg & !msk) | val);
    }
}

unsafe fn pll_get_refin(pll: *const sprd_pll) -> u64 {
    let mut refin_id = 3usize;
    let refin = [2u64, 4, 13, 26];
    if pwidth(pll, PLL_REFIN) != 0 {
        let index = pindex(pll, PLL_REFIN);
        let shift = pshift(pll, PLL_REFIN);
        let mask = pmask(pll, PLL_REFIN);
        refin_id = ((sprd_pll_read(pll, index as u8) & mask) >> shift) as usize;
        if refin_id > 3 { refin_id = 3; }
    }
    refin[refin_id]
}

unsafe fn pll_get_ibias(rate: u64, table: *const u64) -> u32 {
    let num = *table as u32;
    let mut i = 0u32;
    while i < num {
        if rate <= *table.add((i + 1) as usize) { break; }
        i += 1;
    }
    if i == num { num - 1 } else { i }
}

unsafe fn _sprd_pll_recalc_rate(pll: *const sprd_pll, parent_rate: u64) -> u64 {
    let regs_num = (*pll).regs_num as usize;
    let mut cfg = alloc_zeroed_u32(regs_num);
    if cfg.is_null() { return parent_rate; }
    for i in 0..regs_num { *cfg.add(i) = sprd_pll_read(pll, i as u8); }
    let mut refin = pll_get_refin(pll);
    if pinternal(pll, cfg, PLL_PREDIV) != 0 { refin *= 2; }
    if pwidth(pll, PLL_POSTDIV) != 0 && (((*pll).fflag == 1 && pinternal(pll, cfg, PLL_POSTDIV) != 0) || ((*pll).fflag == 0 && pinternal(pll, cfg, PLL_POSTDIV) == 0)) { refin /= 2; }
    let rate;
    if pinternal(pll, cfg, PLL_DIV_S) == 0 {
        rate = refin * pinternal_val(pll, cfg, PLL_N) as u64 * CLK_PLL_10M;
    } else {
        let nint = pinternal_val(pll, cfg, PLL_NINT) as u64;
        let kint = if pinternal(pll, cfg, PLL_SDM_EN) != 0 { pinternal_val(pll, cfg, PLL_KINT) as u64 } else { 0 };
        let mask = pmask(pll, PLL_KINT);
        let denom = ((mask >> mask.trailing_zeros()) + 1) as u64;
        rate = ((refin * kint * (*pll).k1 as u64 + denom / 2) / denom) * (*pll).k2 as u64 + refin * nint * CLK_PLL_1M;
    }
    alloc_free_u32(cfg, regs_num);
    rate
}

unsafe fn _sprd_pll_set_rate(pll: *const sprd_pll, rate: u64, _parent_rate: u64) -> i32 {
    let regs_num = (*pll).regs_num as usize;
    let cfg = alloc_zeroed_reg_cfg(regs_num);
    if cfg.is_null() { return -12; }
    let mut refin = pll_get_refin(pll);
    let mut mask = pmask(pll, PLL_PREDIV); let mut index = pindex(pll, PLL_PREDIV); let width = pwidth(pll, PLL_PREDIV);
    if width != 0 && sprd_pll_read(pll, index as u8) & mask != 0 { refin *= 2; }
    mask = pmask(pll, PLL_POSTDIV); index = pindex(pll, PLL_POSTDIV); let width = pwidth(pll, PLL_POSTDIV);
    (*cfg.add(index)).msk = mask;
    let mut fvco = rate;
    if width != 0 && (((*pll).fflag == 1 && fvco <= (*pll).fvco) || ((*pll).fflag == 0 && fvco > (*pll).fvco)) { (*cfg.add(index)).val |= mask; }
    if width != 0 && fvco <= (*pll).fvco { fvco *= 2; }
    mask = pmask(pll, PLL_DIV_S); index = pindex(pll, PLL_DIV_S); (*cfg.add(index)).val |= mask; (*cfg.add(index)).msk |= mask;
    mask = pmask(pll, PLL_SDM_EN); index = pindex(pll, PLL_SDM_EN); (*cfg.add(index)).val |= mask; (*cfg.add(index)).msk |= mask;
    let nint = fvco / (refin * CLK_PLL_1M); fvco %= refin * CLK_PLL_1M;
    mask = pmask(pll, PLL_NINT); index = pindex(pll, PLL_NINT); let shift = pshift(pll, PLL_NINT); (*cfg.add(index)).val |= ((nint as u32) << shift) & mask; (*cfg.add(index)).msk |= mask;
    mask = pmask(pll, PLL_KINT); index = pindex(pll, PLL_KINT); let shift = pshift(pll, PLL_KINT); let tmp = (fvco / 10000) * ((mask >> shift) as u64 + 1); let kint = (tmp + refin * 50) / (refin * 100);
    (*cfg.add(index)).val |= ((kint as u32) << shift) & mask; (*cfg.add(index)).msk |= mask;
    let ibias_val = pll_get_ibias(rate, (*pll).itable);
    mask = pmask(pll, PLL_IBIAS); index = pindex(pll, PLL_IBIAS); let shift = pshift(pll, PLL_IBIAS); (*cfg.add(index)).val |= ibias_val << shift & mask; (*cfg.add(index)).msk |= mask;
    let mut ret = 0i32;
    for i in 0..regs_num { if (*cfg.add(i)).msk != 0 { sprd_pll_write(pll, i as u8, (*cfg.add(i)).msk, (*cfg.add(i)).val); if sprd_pll_read(pll, i as u8) & (*cfg.add(i)).msk != (*cfg.add(i)).val { ret |= -14; } } }
    if ret == 0 { udelay((*pll).udelay); }
    alloc_free_reg_cfg(cfg, regs_num); ret
}

unsafe fn sprd_pll_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 { _sprd_pll_recalc_rate(hw_to_sprd_pll(hw), parent_rate) }
unsafe fn sprd_pll_set_rate(hw: *mut clk_hw, rate: u64, parent_rate: u64) -> i32 { _sprd_pll_set_rate(hw_to_sprd_pll(hw), rate, parent_rate) }
unsafe fn sprd_pll_clk_prepare(hw: *mut clk_hw) -> i32 { udelay((*hw_to_sprd_pll(hw)).udelay); 0 }

// External types, constants, functions, and allocator helpers are supplied by pll.h and the kernel bindings.
pub static sprd_pll_ops: clk_ops = clk_ops { prepare: Some(sprd_pll_clk_prepare), recalc_rate: Some(sprd_pll_recalc_rate), determine_rate: Some(clk_determine_rate_noop), set_rate: Some(sprd_pll_set_rate) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
