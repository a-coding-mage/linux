// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP3/4 - specific DPLL control functions
 *
 * This is a source-level Rust translation of dpll3xxx.c.  Kernel clock
 * framework types, globals, constants, and operations are supplied by the
 * surrounding translation unit.
 */

const DPLL_AUTOIDLE_DISABLE: u32 = 0x0;
const DPLL_AUTOIDLE_LOW_POWER_STOP: u32 = 0x1;
const MAX_DPLL_WAIT_TRIES: i32 = 1000000;
const OMAP3XXX_EN_DPLL_LOCKED: u32 = 0x7;

extern "C" {
    static mut ti_clk_ll_ops: *mut TiClkLlOps;
    fn udelay(usecs: u32);
    fn clk_hw_get_name(hw: *mut clk_hw) -> *const core::ffi::c_char;
    fn clk_hw_get_rate(hw: *mut clk_hw) -> u64;
    fn clk_hw_get_parent(hw: *mut clk_hw) -> *mut clk_hw;
    fn __clk_get_name(clk: *mut clk) -> *const core::ffi::c_char;
    fn to_clk_hw_omap(hw: *mut clk_hw) -> *mut clk_hw_omap;
    fn omap2_get_dpll_rate(clk: *mut clk_hw_omap) -> u64;
    fn omap2_dpll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32;
    fn omap2_clk_is_hw_omap(hw: *mut clk_hw) -> bool;
    fn ti_clk_get_features() -> *mut TiClkFeatures;
    fn div_u64(n: u64, d: u32) -> u64;
}

// External kernel types and logging/bit-operation facilities are intentionally
// left as dependencies of the surrounding translation.
extern "C" {
    fn __ffs(v: u32) -> u32;
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
    fn pr_debug(fmt: *const core::ffi::c_char, ...);
    fn WARN_ON(cond: bool) -> bool;
    fn WARN(cond: bool, fmt: *const core::ffi::c_char, ...);
}

const DPLL_LOCKED: u32 = 1;
const DPLL_LOW_POWER_BYPASS: u32 = 4;
const DPLL_LOW_POWER_STOP: u32 = 5;
const DPLL_J_TYPE: u32 = 1 << 0;
const TI_CLK_DPLL_HAS_FREQSEL: u32 = 1 << 0;
const TI_CLK_ERRATA_I810: u32 = 1 << 1;
const TI_CLK_DPLL4_DENY_REPROGRAM: u32 = 1 << 2;
const OMAP3_DPLL5_FREQ_FOR_USBHOST: u64 = 12000000;

// OMAP3/4 non-CORE DPLL clock operations; the structure type is supplied by
// the surrounding clock-framework translation.
extern "C" {
    pub static clkhwops_omap3_dpll: clk_hw_omap_ops;
}

unsafe fn _omap3_dpll_write_clken(clk: *mut clk_hw_omap, clken_bits: u8) {
    let dd = (*clk).dpll_data;
    let mut v = (*ti_clk_ll_ops).clk_readl(&(*dd).control_reg);
    v &= !(*dd).enable_mask;
    v |= (clken_bits as u32) << __ffs((*dd).enable_mask);
    (*ti_clk_ll_ops).clk_writel(v, &(*dd).control_reg);
}

unsafe fn _omap3_wait_dpll_status(clk: *mut clk_hw_omap, mut state: u8) -> i32 {
    let dd = (*clk).dpll_data;
    let mut i = 0;
    let mut ret = -22;
    let name = clk_hw_get_name(&mut (*clk).hw);
    state = state << __ffs((*dd).idlest_mask);
    while (((*ti_clk_ll_ops).clk_readl(&(*dd).idlest_reg) & (*dd).idlest_mask) != state as u32)
        && i < MAX_DPLL_WAIT_TRIES
    {
        i += 1;
        udelay(1);
    }
    if i == MAX_DPLL_WAIT_TRIES {
        pr_err(b"clock: %s failed transition to '%s'\n\0".as_ptr() as _, name,
               if state != 0 { b"locked\0".as_ptr() } else { b"bypassed\0".as_ptr() });
    } else {
        pr_debug(b"clock: %s transition to '%s' in %d loops\n\0".as_ptr() as _, name,
                 if state != 0 { b"locked\0".as_ptr() } else { b"bypassed\0".as_ptr() }, i);
        ret = 0;
    }
    ret
}

unsafe fn _omap3_dpll_compute_freqsel(clk: *mut clk_hw_omap, n: u8) -> u16 {
    let fint = clk_hw_get_rate((*(*clk).dpll_data).clk_ref) / n as u64;
    let mut f = 0;
    if fint >= 750000 && fint <= 1000000 { f = 0x3; }
    else if fint > 1000000 && fint <= 1250000 { f = 0x4; }
    else if fint > 1250000 && fint <= 1500000 { f = 0x5; }
    else if fint > 1500000 && fint <= 1750000 { f = 0x6; }
    else if fint > 1750000 && fint <= 2100000 { f = 0x7; }
    else if fint > 7500000 && fint <= 10000000 { f = 0xB; }
    else if fint > 10000000 && fint <= 12500000 { f = 0xC; }
    else if fint > 12500000 && fint <= 15000000 { f = 0xD; }
    else if fint > 15000000 && fint <= 17500000 { f = 0xE; }
    else if fint > 17500000 && fint <= 21000000 { f = 0xF; }
    f
}

unsafe fn _omap3_noncore_dpll_lock(clk: *mut clk_hw_omap) -> i32 {
    let dd = (*clk).dpll_data;
    let state = 1u32 << __ffs((*dd).idlest_mask);
    if ((*ti_clk_ll_ops).clk_readl(&(*dd).idlest_reg) & (*dd).idlest_mask) == state { return 0; }
    let ai = omap3_dpll_autoidle_read(clk);
    if ai != 0 { omap3_dpll_deny_idle(clk); }
    _omap3_dpll_write_clken(clk, OMAP3XXX_EN_DPLL_LOCKED as u8);
    let r = _omap3_wait_dpll_status(clk, 1);
    if ai != 0 { omap3_dpll_allow_idle(clk); }
    r
}

unsafe fn _omap3_noncore_dpll_bypass(clk: *mut clk_hw_omap) -> i32 {
    if ((*(*clk).dpll_data).modes & (1 << DPLL_LOW_POWER_BYPASS)) == 0 { return -22; }
    let ai = omap3_dpll_autoidle_read(clk);
    _omap3_dpll_write_clken(clk, DPLL_LOW_POWER_BYPASS as u8);
    let r = _omap3_wait_dpll_status(clk, 0);
    if ai != 0 { omap3_dpll_allow_idle(clk); }
    r
}

unsafe fn _omap3_noncore_dpll_stop(clk: *mut clk_hw_omap) -> i32 {
    if ((*(*clk).dpll_data).modes & (1 << DPLL_LOW_POWER_STOP)) == 0 { return -22; }
    let ai = omap3_dpll_autoidle_read(clk);
    _omap3_dpll_write_clken(clk, DPLL_LOW_POWER_STOP as u8);
    if ai != 0 { omap3_dpll_allow_idle(clk); }
    0
}

unsafe fn _lookup_dco(clk: *mut clk_hw_omap, dco: *mut u8, m: u16, n: u8) {
    let clkinp = clk_hw_get_rate(clk_hw_get_parent(&mut (*clk).hw));
    *dco = if (clkinp / n as u64) * m as u64 < 1000000000 { 2 } else { 4 };
}

unsafe fn _lookup_sddiv(clk: *mut clk_hw_omap, sd_div: *mut u8, m: u16, n: u8) {
    let clkinp = clk_hw_get_rate(clk_hw_get_parent(&mut (*clk).hw)) / 100000;
    let denominator = 250 * n as u64;
    let mod1 = (clkinp * m as u64) % denominator;
    let mut sd = (clkinp * m as u64) / denominator;
    let mod2 = sd % 10;
    sd /= 10;
    if mod1 != 0 || mod2 != 0 { sd += 1; }
    *sd_div = sd as u8;
}

unsafe fn omap3_noncore_dpll_ssc_program(clk: *mut clk_hw_omap) {
    let dd = (*clk).dpll_data;
    let mut ctrl = (*ti_clk_ll_ops).clk_readl(&(*dd).control_reg);
    if (*dd).ssc_modfreq != 0 && (*dd).ssc_deltam != 0 {
        ctrl |= (*dd).ssc_enable_mask;
        if (*dd).ssc_downspread { ctrl |= (*dd).ssc_downspread_mask; } else { ctrl &= !(*dd).ssc_downspread_mask; }
        let ref_rate = clk_hw_get_rate((*dd).clk_ref);
        let mod_freq_divider = (ref_rate / (*dd).last_rounded_n as u64) / (4 * (*dd).ssc_modfreq as u64);
        let mut exponent = 0u32;
        let mut mantissa = mod_freq_divider as u32;
        while mantissa > 127 && exponent < 7 { exponent += 1; mantissa /= 2; }
        if mantissa > 127 { mantissa = 127; }
        let mut v = (*ti_clk_ll_ops).clk_readl(&(*dd).ssc_modfreq_reg);
        v &= !((*dd).ssc_modfreq_mant_mask | (*dd).ssc_modfreq_exp_mask);
        v |= mantissa << __ffs((*dd).ssc_modfreq_mant_mask);
        v |= exponent << __ffs((*dd).ssc_modfreq_exp_mask);
        (*ti_clk_ll_ops).clk_writel(v, &(*dd).ssc_modfreq_reg);
        let mut deltam_step = (*dd).last_rounded_m * (*dd).ssc_deltam;
        deltam_step /= 10;
        if (*dd).ssc_downspread { deltam_step /= 2; }
        deltam_step <<= __ffs((*dd).ssc_deltam_int_mask);
        deltam_step /= 100;
        deltam_step /= mod_freq_divider as u32;
        if deltam_step > 0xFFFFF { deltam_step = 0xFFFFF; }
        let mut deltam_ceil = (deltam_step & (*dd).ssc_deltam_int_mask) >> __ffs((*dd).ssc_deltam_int_mask);
        if deltam_step & (*dd).ssc_deltam_frac_mask != 0 { deltam_ceil += 1; }
        let out = if (*dd).ssc_downspread { (*dd).last_rounded_m - 2 * deltam_ceil < 20 || (*dd).last_rounded_m > 2045 } else { (*dd).last_rounded_m - deltam_ceil < 20 || (*dd).last_rounded_m + deltam_ceil > 2045 };
        let _ = out;
        v = (*ti_clk_ll_ops).clk_readl(&(*dd).ssc_deltam_reg);
        v &= !((*dd).ssc_deltam_int_mask | (*dd).ssc_deltam_frac_mask);
        v |= deltam_step << __ffs((*dd).ssc_deltam_int_mask | (*dd).ssc_deltam_frac_mask);
        (*ti_clk_ll_ops).clk_writel(v, &(*dd).ssc_deltam_reg);
    } else { ctrl &= !(*dd).ssc_enable_mask; }
    (*ti_clk_ll_ops).clk_writel(ctrl, &(*dd).control_reg);
}

unsafe fn omap3_noncore_dpll_program(clk: *mut clk_hw_omap, freqsel: u16) -> i32 {
    let dd = (*clk).dpll_data;
    let mut dco = 0u8; let mut sd_div = 0u8; let mut ai = 0u32;
    let mut v: u32;
    _omap3_noncore_dpll_bypass(clk);
    if (*ti_clk_get_features()).flags & TI_CLK_DPLL_HAS_FREQSEL != 0 {
        v = (*ti_clk_ll_ops).clk_readl(&(*dd).control_reg);
        v &= !(*dd).freqsel_mask; v |= (freqsel as u32) << __ffs((*dd).freqsel_mask);
        (*ti_clk_ll_ops).clk_writel(v, &(*dd).control_reg);
    }
    v = (*ti_clk_ll_ops).clk_readl(&(*dd).mult_div1_reg);
    if (*dd).dcc_mask != 0 { if (*dd).last_rounded_rate >= (*dd).dcc_rate { v |= (*dd).dcc_mask; } else { v &= !(*dd).dcc_mask; } }
    v &= !((*dd).mult_mask | (*dd).div1_mask);
    v |= (*dd).last_rounded_m << __ffs((*dd).mult_mask);
    v |= ((*dd).last_rounded_n - 1) << __ffs((*dd).div1_mask);
    if (*dd).dco_mask != 0 { _lookup_dco(clk, &mut dco, (*dd).last_rounded_m as u16, (*dd).last_rounded_n as u8); v &= !(*dd).dco_mask; v |= (dco as u32) << __ffs((*dd).dco_mask); }
    if (*dd).sddiv_mask != 0 { _lookup_sddiv(clk, &mut sd_div, (*dd).last_rounded_m as u16, (*dd).last_rounded_n as u8); v &= !(*dd).sddiv_mask; v |= (sd_div as u32) << __ffs((*dd).sddiv_mask); }
    let errata_i810 = (*ti_clk_get_features()).flags & TI_CLK_ERRATA_I810 != 0;
    if errata_i810 { ai = omap3_dpll_autoidle_read(clk); if ai != 0 { omap3_dpll_deny_idle(clk); omap3_dpll_autoidle_read(clk); } }
    (*ti_clk_ll_ops).clk_writel(v, &(*dd).mult_div1_reg);
    if (*dd).m4xen_mask != 0 || (*dd).lpmode_mask != 0 {
        v = (*ti_clk_ll_ops).clk_readl(&(*dd).control_reg);
        if (*dd).m4xen_mask != 0 { if (*dd).last_rounded_m4xen { v |= (*dd).m4xen_mask; } else { v &= !(*dd).m4xen_mask; } }
        if (*dd).lpmode_mask != 0 { if (*dd).last_rounded_lpmode { v |= (*dd).lpmode_mask; } else { v &= !(*dd).lpmode_mask; } }
        (*ti_clk_ll_ops).clk_writel(v, &(*dd).control_reg);
    }
    if (*dd).ssc_enable_mask != 0 { omap3_noncore_dpll_ssc_program(clk); }
    _omap3_noncore_dpll_lock(clk);
    if errata_i810 && ai != 0 { omap3_dpll_allow_idle(clk); }
    0
}

pub unsafe fn omap3_dpll_recalc(hw: *mut clk_hw, _parent_rate: u64) -> u64 { omap2_get_dpll_rate(to_clk_hw_omap(hw)) }

pub unsafe fn omap3_noncore_dpll_enable(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_hw_omap(hw); let dd = (*clk).dpll_data; if dd.is_null() { return -22; }
    if !(*clk).clkdm.is_null() { let r = (*ti_clk_ll_ops).clkdm_clk_enable((*clk).clkdm, (*hw).clk); if r != 0 { return r; } }
    let parent = clk_hw_get_parent(hw);
    if clk_hw_get_rate(hw) == clk_hw_get_rate((*dd).clk_bypass) { let _ = WARN_ON(parent != (*dd).clk_bypass); _omap3_noncore_dpll_bypass(clk) } else { let _ = WARN_ON(parent != (*dd).clk_ref); _omap3_noncore_dpll_lock(clk) }
}

pub unsafe fn omap3_noncore_dpll_disable(hw: *mut clk_hw) { let clk = to_clk_hw_omap(hw); _omap3_noncore_dpll_stop(clk); if !(*clk).clkdm.is_null() { (*ti_clk_ll_ops).clkdm_clk_disable((*clk).clkdm, (*hw).clk); } }

pub unsafe fn omap3_noncore_dpll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let clk = to_clk_hw_omap(hw); if (*req).rate == 0 || (*clk).dpll_data.is_null() { return -22; }
    let dd = (*clk).dpll_data;
    if clk_hw_get_rate((*dd).clk_bypass) == (*req).rate && (*dd).modes & (1 << DPLL_LOW_POWER_BYPASS) != 0 { (*req).best_parent_hw = (*dd).clk_bypass; } else { let ret = omap2_dpll_determine_rate(hw, req); if ret != 0 { return ret; } (*req).best_parent_hw = (*dd).clk_ref; }
    (*req).best_parent_rate = (*req).rate; 0
}

pub unsafe fn omap3_noncore_dpll_set_parent(hw: *mut clk_hw, index: u8) -> i32 { if hw.is_null() { return -22; } let clk = to_clk_hw_omap(hw); if index != 0 { _omap3_noncore_dpll_bypass(clk) } else { _omap3_noncore_dpll_lock(clk) } }

pub unsafe fn omap3_noncore_dpll_set_rate(hw: *mut clk_hw, _rate: u64, _parent_rate: u64) -> i32 {
    let clk = to_clk_hw_omap(hw); if hw.is_null() || _rate == 0 || (*clk).dpll_data.is_null() { return -22; }
    let dd = (*clk).dpll_data; if clk_hw_get_parent(hw) != (*dd).clk_ref || (*dd).last_rounded_rate == 0 { return -22; }
    let mut freqsel = 0; if (*ti_clk_get_features()).flags & TI_CLK_DPLL_HAS_FREQSEL != 0 { freqsel = _omap3_dpll_compute_freqsel(clk, (*dd).last_rounded_n as u8); }
    omap3_noncore_dpll_program(clk, freqsel)
}

pub unsafe fn omap3_noncore_dpll_set_rate_and_parent(hw: *mut clk_hw, rate: u64, parent_rate: u64, index: u8) -> i32 { if hw.is_null() || rate == 0 { return -22; } if index != 0 { omap3_noncore_dpll_set_parent(hw, index) } else { omap3_noncore_dpll_set_rate(hw, rate, parent_rate) } }

unsafe fn omap3_dpll_autoidle_read(clk: *mut clk_hw_omap) -> u32 { if clk.is_null() || (*clk).dpll_data.is_null() { return u32::MAX; } let dd = (*clk).dpll_data; if (*dd).autoidle_mask == 0 { return u32::MAX; } ((*ti_clk_ll_ops).clk_readl(&(*dd).autoidle_reg) & (*dd).autoidle_mask) >> __ffs((*dd).autoidle_mask) }
unsafe fn omap3_dpll_allow_idle(clk: *mut clk_hw_omap) { if clk.is_null() || (*clk).dpll_data.is_null() { return; } let dd = (*clk).dpll_data; if (*dd).autoidle_mask == 0 { return; } let mut v = (*ti_clk_ll_ops).clk_readl(&(*dd).autoidle_reg); v &= !(*dd).autoidle_mask; v |= DPLL_AUTOIDLE_LOW_POWER_STOP << __ffs((*dd).autoidle_mask); (*ti_clk_ll_ops).clk_writel(v, &(*dd).autoidle_reg); }
unsafe fn omap3_dpll_deny_idle(clk: *mut clk_hw_omap) { if clk.is_null() || (*clk).dpll_data.is_null() { return; } let dd = (*clk).dpll_data; if (*dd).autoidle_mask == 0 { return; } let mut v = (*ti_clk_ll_ops).clk_readl(&(*dd).autoidle_reg); v &= !(*dd).autoidle_mask; v |= DPLL_AUTOIDLE_DISABLE << __ffs((*dd).autoidle_mask); (*ti_clk_ll_ops).clk_writel(v, &(*dd).autoidle_reg); }

unsafe fn omap3_find_clkoutx2_dpll(mut hw: *mut clk_hw) -> *mut clk_hw_omap {
    let mut pclk = core::ptr::null_mut();
    loop { loop { hw = clk_hw_get_parent(hw); if hw.is_null() || omap2_clk_is_hw_omap(hw) { break; } } if hw.is_null() { break; } pclk = to_clk_hw_omap(hw); if !(*pclk).dpll_data.is_null() { break; } }
    if pclk.is_null() { let _ = WARN_ON(true); } pclk
}

pub unsafe fn omap3_clkoutx2_recalc(hw: *mut clk_hw, parent_rate: u64) -> u64 { if parent_rate == 0 { return 0; } let pclk = omap3_find_clkoutx2_dpll(hw); if pclk.is_null() { return 0; } let dd = (*pclk).dpll_data; let mut v = ((*ti_clk_ll_ops).clk_readl(&(*dd).control_reg) & (*dd).enable_mask) >> __ffs((*dd).enable_mask); if v != OMAP3XXX_EN_DPLL_LOCKED || (*dd).flags & DPLL_J_TYPE != 0 { parent_rate } else { parent_rate * 2 } }

pub unsafe fn omap3_core_dpll_save_context(hw: *mut clk_hw) -> i32 { let clk = to_clk_hw_omap(hw); let dd = (*clk).dpll_data; let v = (*ti_clk_ll_ops).clk_readl(&(*dd).control_reg); (*clk).context = (v & (*dd).enable_mask) >> __ffs((*dd).enable_mask); if (*clk).context == DPLL_LOCKED { let v = (*ti_clk_ll_ops).clk_readl(&(*dd).mult_div1_reg); (*dd).last_rounded_m = (v & (*dd).mult_mask) >> __ffs((*dd).mult_mask); (*dd).last_rounded_n = ((v & (*dd).div1_mask) >> __ffs((*dd).div1_mask)) + 1; } 0 }
pub unsafe fn omap3_core_dpll_restore_context(hw: *mut clk_hw) { let clk = to_clk_hw_omap(hw); let dd = (*clk).dpll_data; if (*clk).context == DPLL_LOCKED { _omap3_dpll_write_clken(clk, 4); _omap3_wait_dpll_status(clk, 0); let mut v = (*ti_clk_ll_ops).clk_readl(&(*dd).mult_div1_reg); v &= !((*dd).mult_mask | (*dd).div1_mask); v |= (*dd).last_rounded_m << __ffs((*dd).mult_mask); v |= ((*dd).last_rounded_n - 1) << __ffs((*dd).div1_mask); (*ti_clk_ll_ops).clk_writel(v, &(*dd).mult_div1_reg); _omap3_dpll_write_clken(clk, DPLL_LOCKED as u8); _omap3_wait_dpll_status(clk, 1); } else { _omap3_dpll_write_clken(clk, (*clk).context as u8); } }
pub unsafe fn omap3_noncore_dpll_save_context(hw: *mut clk_hw) -> i32 { omap3_core_dpll_save_context(hw) }
pub unsafe fn omap3_noncore_dpll_restore_context(hw: *mut clk_hw) { let clk = to_clk_hw_omap(hw); let dd = (*clk).dpll_data; let ctrl = (*ti_clk_ll_ops).clk_readl(&(*dd).control_reg); let mult = (*ti_clk_ll_ops).clk_readl(&(*dd).mult_div1_reg); if (*clk).context == ((ctrl & (*dd).enable_mask) >> __ffs((*dd).enable_mask)) && (*dd).last_rounded_m == ((mult & (*dd).mult_mask) >> __ffs((*dd).mult_mask)) && (*dd).last_rounded_n == ((mult & (*dd).div1_mask) >> __ffs((*dd).div1_mask)) + 1 { return; } if (*clk).context == DPLL_LOCKED { omap3_noncore_dpll_program(clk, 0); } else { _omap3_dpll_write_clken(clk, (*clk).context as u8); } }

pub unsafe fn omap3_dpll4_set_rate(hw: *mut clk_hw, rate: u64, parent_rate: u64) -> i32 { if (*ti_clk_get_features()).flags & TI_CLK_DPLL4_DENY_REPROGRAM != 0 { return -22; } omap3_noncore_dpll_set_rate(hw, rate, parent_rate) }
pub unsafe fn omap3_dpll4_set_rate_and_parent(hw: *mut clk_hw, rate: u64, parent_rate: u64, index: u8) -> i32 { if (*ti_clk_get_features()).flags & TI_CLK_DPLL4_DENY_REPROGRAM != 0 { return -22; } omap3_noncore_dpll_set_rate_and_parent(hw, rate, parent_rate, index) }

#[repr(C)]
struct Omap3Dpll5Settings { rate: u32, m: u32, n: u32 }

unsafe fn omap3_dpll5_apply_errata(hw: *mut clk_hw, parent_rate: u64) -> bool {
    const PRECOMPUTED: [Omap3Dpll5Settings; 5] = [
        Omap3Dpll5Settings { rate: 12000000, m: 80, n: 1 }, Omap3Dpll5Settings { rate: 13000000, m: 443, n: 6 },
        Omap3Dpll5Settings { rate: 19200000, m: 50, n: 1 }, Omap3Dpll5Settings { rate: 26000000, m: 443, n: 12 },
        Omap3Dpll5Settings { rate: 38400000, m: 25, n: 1 },
    ];
    let mut i = 0; while i < PRECOMPUTED.len() && parent_rate != PRECOMPUTED[i].rate as u64 { i += 1; }
    if i == PRECOMPUTED.len() { return false; }
    let d = PRECOMPUTED[i]; let clk = to_clk_hw_omap(hw); let dd = (*clk).dpll_data;
    (*dd).last_rounded_m = d.m; (*dd).last_rounded_n = d.n; (*dd).last_rounded_rate = div_u64(parent_rate * d.m as u64, d.n); omap3_noncore_dpll_program(clk, 0); true
}

pub unsafe fn omap3_dpll5_set_rate(hw: *mut clk_hw, rate: u64, parent_rate: u64) -> i32 { if rate == OMAP3_DPLL5_FREQ_FOR_USBHOST * 8 && omap3_dpll5_apply_errata(hw, parent_rate) { return 0; } omap3_noncore_dpll_set_rate(hw, rate, parent_rate) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
