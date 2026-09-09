// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
 */

// Kernel dependencies supplied by the surrounding translation unit.

const PLL_DIV_MASK: u32 = 0xff;
const PLL_DIV_MAX: u32 = PLL_DIV_MASK;
const PLL_MUL_MIN: u32 = 2;
const PLL_MAX_COUNT: u32 = 0x3f;
const PLL_COUNT_SHIFT: u32 = 8;
const PLL_OUT_SHIFT: u32 = 14;
const PLL_MAX_ID: u8 = 1;

#[inline]
fn pll_status_mask(id: u8) -> u32 { 1u32 << (1 + id) }

#[inline]
fn pll_reg(id: u8) -> u32 { AT91_CKGR_PLLAR + ((id as u32) * 4) }

#[inline]
fn pll_div(reg: u32) -> u8 { (reg & PLL_DIV_MASK) as u8 }

#[inline]
fn pll_mul(reg: u32, layout: *const clk_pll_layout) -> u16 {
    ((reg >> (*layout).mul_shift) & (*layout).mul_mask) as u16
}

#[inline]
fn pll_mul_mask(layout: *const clk_pll_layout) -> u32 { unsafe { (*layout).mul_mask } }

#[inline]
fn pll_mul_max(layout: *const clk_pll_layout) -> u32 { pll_mul_mask(layout) + 1 }

#[inline]
fn pll_icpr_shift(id: u8) -> u32 { (id as u32) * 16 }

#[inline]
fn pll_icpr_mask(id: u8) -> u32 { 0xffffu32 << pll_icpr_shift(id) }

#[repr(C)]
pub struct clk_pll {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub id: u8,
    pub div: u8,
    pub range: u8,
    pub mul: u16,
    pub layout: *const clk_pll_layout,
    pub characteristics: *const clk_pll_characteristics,
    pub pms: at91_clk_pms,
}

#[inline]
unsafe fn clk_pll_ready(regmap: *mut regmap, id: u8) -> bool {
    let mut status: u32 = 0;
    regmap_read(regmap, AT91_PMC_SR, &mut status);
    (status & pll_status_mask(id)) != 0
}

unsafe fn clk_pll_prepare(hw: *mut clk_hw) -> i32 {
    let pll = container_of_clk_pll(hw);
    let regmap = (*pll).regmap;
    let layout = (*pll).layout;
    let characteristics = (*pll).characteristics;
    let id = (*pll).id;
    let mask = pll_status_mask(id);
    let offset = pll_reg(id);
    let mut out: u8 = 0;
    let mut pllr: u32 = 0;
    let mut status: u32 = 0;

    regmap_read(regmap, offset, &mut pllr);
    let div = pll_div(pllr);
    let mul = pll_mul(pllr, layout);
    regmap_read(regmap, AT91_PMC_SR, &mut status);
    if (status & mask) != 0 && div == (*pll).div && mul == (*pll).mul { return 0; }

    if !(*characteristics).out.is_null() { out = *(*characteristics).out.add((*pll).range as usize); }
    if !(*characteristics).icpll.is_null() {
        regmap_update_bits(regmap, AT91_PMC_PLLICPR, pll_icpr_mask(id),
            (*(*characteristics).icpll.add((*pll).range as usize)) << pll_icpr_shift(id));
    }
    regmap_update_bits(regmap, offset, (*layout).pllr_mask,
        (*pll).div as u32 | (PLL_MAX_COUNT << PLL_COUNT_SHIFT) |
        ((out as u32) << PLL_OUT_SHIFT) |
        ((((*pll).mul as u32) & (*layout).mul_mask) << (*layout).mul_shift));
    while !clk_pll_ready(regmap, id) { cpu_relax(); }
    0
}

unsafe fn clk_pll_is_prepared(hw: *mut clk_hw) -> i32 {
    let pll = container_of_clk_pll(hw);
    clk_pll_ready((*pll).regmap, (*pll).id) as i32
}

unsafe fn clk_pll_unprepare(hw: *mut clk_hw) {
    let pll = container_of_clk_pll(hw);
    let mask = (*(*pll).layout).pllr_mask;
    regmap_update_bits((*pll).regmap, pll_reg((*pll).id), mask, !mask);
}

unsafe fn clk_pll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let pll = container_of_clk_pll(hw);
    if (*pll).div == 0 || (*pll).mul == 0 { return 0; }
    (parent_rate / (*pll).div as c_ulong) * ((*pll).mul as c_ulong + 1)
}

unsafe fn clk_pll_get_best_div_mul(pll: *mut clk_pll, rate: c_ulong, parent_rate: c_ulong,
    div: *mut u32, mul: *mut u32, index: *mut u32) -> c_long {
    let layout = (*pll).layout;
    let characteristics = (*pll).characteristics;
    if parent_rate < (*characteristics).input.min { return -ERANGE as c_long; }
    let mut mindiv = (parent_rate * PLL_MUL_MIN as c_ulong) / rate;
    if mindiv == 0 { mindiv = 1; }
    if parent_rate > (*characteristics).input.max {
        let tmpdiv = (parent_rate + (*characteristics).input.max - 1) / (*characteristics).input.max;
        if tmpdiv > PLL_DIV_MAX as c_ulong { return -ERANGE as c_long; }
        if tmpdiv > mindiv { mindiv = tmpdiv; }
    }
    let mut maxdiv = (parent_rate * pll_mul_max(layout) as c_ulong + rate - 1) / rate;
    if maxdiv > PLL_DIV_MAX as c_ulong { maxdiv = PLL_DIV_MAX as c_ulong; }
    let mut bestremainder = c_ulong::MAX;
    let mut bestrate: c_long = -ERANGE as c_long;
    let mut bestdiv = 0;
    let mut bestmul = 0;
    let mut tmpdiv = mindiv;
    while tmpdiv <= maxdiv {
        let tmpmul = (rate + (parent_rate / tmpdiv) / 2) / (parent_rate / tmpdiv);
        let tmprate = (parent_rate / tmpdiv) * tmpmul;
        let remainder = if tmprate > rate { tmprate - rate } else { rate - tmprate };
        if remainder < bestremainder { bestremainder = remainder; bestdiv = tmpdiv; bestmul = tmpmul; bestrate = tmprate as c_long; }
        if remainder == 0 { break; }
        tmpdiv += 1;
    }
    if bestrate < 0 { return bestrate; }
    let mut i = 0;
    while i < (*characteristics).num_output {
        let output = *(*characteristics).output.add(i as usize);
        if bestrate as c_ulong >= output.min && bestrate as c_ulong <= output.max { break; }
        i += 1;
    }
    if i >= (*characteristics).num_output { return -ERANGE as c_long; }
    if !div.is_null() { *div = bestdiv as u32; }
    if !mul.is_null() { *mul = (bestmul - 1) as u32; }
    if !index.is_null() { *index = i as u32; }
    bestrate
}

unsafe fn clk_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let pll = container_of_clk_pll(hw);
    (*req).rate = clk_pll_get_best_div_mul(pll, (*req).rate, (*req).best_parent_rate, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut()) as c_ulong;
    0
}

unsafe fn clk_pll_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> i32 {
    let pll = container_of_clk_pll(hw);
    let mut div = 0; let mut mul = 0; let mut index = 0;
    let ret = clk_pll_get_best_div_mul(pll, rate, parent_rate, &mut div, &mut mul, &mut index);
    if ret < 0 { return ret as i32; }
    (*pll).range = index as u8; (*pll).div = div as u8; (*pll).mul = mul as u16; 0
}

unsafe fn clk_pll_save_context(hw: *mut clk_hw) -> i32 {
    let pll = container_of_clk_pll(hw);
    let parent_hw = clk_hw_get_parent(hw);
    (*pll).pms.parent_rate = clk_hw_get_rate(parent_hw);
    (*pll).pms.rate = clk_pll_recalc_rate(&mut (*pll).hw, (*pll).pms.parent_rate);
    (*pll).pms.status = clk_pll_ready((*pll).regmap, pll_reg((*pll).id));
    0
}

unsafe fn clk_pll_restore_context(hw: *mut clk_hw) {
    let pll = container_of_clk_pll(hw);
    let mut out: u8 = 0;
    if !(*(*pll).characteristics).out.is_null() { out = *(*(*pll).characteristics).out.add((*pll).range as usize); }
    let mut pllr = 0u32;
    regmap_read((*pll).regmap, pll_reg((*pll).id), &mut pllr);
    let calc_rate = ((*pll).pms.parent_rate / pll_div(pllr) as c_ulong) * (pll_mul(pllr, (*pll).layout) as c_ulong + 1);
    let pllr_count = (pllr >> PLL_COUNT_SHIFT) & PLL_MAX_COUNT;
    let pllr_out = (pllr >> PLL_OUT_SHIFT) & out as u32;
    if (*pll).pms.rate != calc_rate || (*pll).pms.status != clk_pll_ready((*pll).regmap, pll_reg((*pll).id)) ||
       pllr_count != PLL_MAX_COUNT || (out != 0 && pllr_out != out as u32) {
        pr_warn("PLLAR was not configured properly by firmware\\n");
    }
}

// The C clk_ops table and registration function use kernel-provided ABI types.
pub unsafe fn at91_clk_register_pll(
    regmap: *mut regmap, name: *const c_char, parent_name: *const c_char, id: u8,
    layout: *const clk_pll_layout,
    characteristics: *const clk_pll_characteristics,
) -> *mut clk_hw {
    if id > PLL_MAX_ID { return ERR_PTR(-EINVAL); }
    let pll = kzalloc_obj::<clk_pll>();
    if pll.is_null() { return ERR_PTR(-ENOMEM); }
    (*pll).id = id;
    (*pll).layout = layout;
    (*pll).characteristics = characteristics;
    (*pll).regmap = regmap;
    let mut pllr = 0u32;
    regmap_read(regmap, pll_reg(id), &mut pllr);
    (*pll).div = pll_div(pllr);
    (*pll).mul = pll_mul(pllr, layout);
    let hw = &mut (*pll).hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 { kfree(pll as *mut c_void); return ERR_PTR(ret); }
    hw
}

pub const at91rm9200_pll_layout: clk_pll_layout = clk_pll_layout { pllr_mask: 0x7ffffff, mul_shift: 16, mul_mask: 0x7ff };
pub const at91sam9g45_pll_layout: clk_pll_layout = clk_pll_layout { pllr_mask: 0xffffff, mul_shift: 16, mul_mask: 0xff };
pub const at91sam9g20_pllb_layout: clk_pll_layout = clk_pll_layout { pllr_mask: 0x3fffff, mul_shift: 16, mul_mask: 0x3f };
pub const sama5d3_pll_layout: clk_pll_layout = clk_pll_layout { pllr_mask: 0x1ffffff, mul_shift: 18, mul_mask: 0x7f };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
