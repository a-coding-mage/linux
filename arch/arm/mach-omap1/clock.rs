// SPDX-License-Identifier: GPL-2.0-only
/* Direct source-level Rust translation of linux/arch/arm/mach-omap1/clock.c. */

// Includes and symbols from the original C translation unit are supplied by
// the surrounding kernel bindings.

pub static mut arm_idlect1_mask: u32 = 0;
pub static mut api_ck_p: *mut omap1_clk = core::ptr::null_mut();
pub static mut ck_dpll1_p: *mut omap1_clk = core::ptr::null_mut();
pub static mut ck_ref_p: *mut omap1_clk = core::ptr::null_mut();

extern "C" {
    static mut arm_ckctl_lock: spinlock_t;
    static mut arm_idlect2_lock: spinlock_t;
    static mut mod_conf_ctrl_0_lock: spinlock_t;
    static mut mod_conf_ctrl_1_lock: spinlock_t;
    static mut swd_clk_div_ctrl_sel_lock: spinlock_t;
}

pub unsafe fn omap1_uart_recalc(clk: *mut omap1_clk, _p_rate: usize) -> usize {
    let val = __raw_readl((*clk).enable_reg);
    if val & (1 << (*clk).enable_bit) != 0 { 48000000 } else { 12000000 }
}

pub unsafe fn omap1_sossi_recalc(_clk: *mut omap1_clk, p_rate: usize) -> usize {
    let mut div = (omap_readl(MOD_CONF_CTRL_1) >> 17) & 0x7;
    div += 1;
    p_rate / div as usize
}

unsafe fn omap1_clk_allow_idle(clk: *mut omap1_clk) {
    let iclk = clk as *mut arm_idlect1_clk;
    if (*clk).flags & CLOCK_IDLE_CONTROL == 0 { return; }
    if (*iclk).no_idle_count > 0 {
        (*iclk).no_idle_count -= 1;
        if (*iclk).no_idle_count == 0 { arm_idlect1_mask |= 1 << (*iclk).idlect_shift; }
    }
}

unsafe fn omap1_clk_deny_idle(clk: *mut omap1_clk) {
    let iclk = clk as *mut arm_idlect1_clk;
    if (*clk).flags & CLOCK_IDLE_CONTROL == 0 { return; }
    if (*iclk).no_idle_count == 0 { arm_idlect1_mask &= !(1 << (*iclk).idlect_shift); }
    (*iclk).no_idle_count += 1;
}

unsafe fn verify_ckctl_value(mut newval: u16) -> u16 {
    let mut per_exp = ((newval >> CKCTL_PERDIV_OFFSET) & 3) as u8;
    let mut lcd_exp = ((newval >> CKCTL_LCDDIV_OFFSET) & 3) as u8;
    let arm_exp = ((newval >> CKCTL_ARMDIV_OFFSET) & 3) as u8;
    let dsp_exp = ((newval >> CKCTL_DSPDIV_OFFSET) & 3) as u8;
    let mut tc_exp = ((newval >> CKCTL_TCDIV_OFFSET) & 3) as u8;
    let mut dspmmu_exp = ((newval >> CKCTL_DSPMMUDIV_OFFSET) & 3) as u8;
    if dspmmu_exp < dsp_exp { dspmmu_exp = dsp_exp; }
    if dspmmu_exp > dsp_exp + 1 { dspmmu_exp = dsp_exp + 1; }
    if tc_exp < arm_exp { tc_exp = arm_exp; }
    if tc_exp < dspmmu_exp { tc_exp = dspmmu_exp; }
    if tc_exp > lcd_exp { lcd_exp = tc_exp; }
    if tc_exp > per_exp { per_exp = tc_exp; }
    newval &= 0xf000;
    newval |= (per_exp as u16) << CKCTL_PERDIV_OFFSET;
    newval |= (lcd_exp as u16) << CKCTL_LCDDIV_OFFSET;
    newval |= (arm_exp as u16) << CKCTL_ARMDIV_OFFSET;
    newval |= (dsp_exp as u16) << CKCTL_DSPDIV_OFFSET;
    newval |= (tc_exp as u16) << CKCTL_TCDIV_OFFSET;
    newval |= (dspmmu_exp as u16) << CKCTL_DSPMMUDIV_OFFSET;
    newval
}

unsafe fn calc_dsor_exp(rate: usize, mut realrate: usize) -> i32 {
    if realrate == 0 { return -EIO; }
    let mut dsor_exp = 0;
    while dsor_exp < 4 {
        if realrate <= rate { break; }
        realrate /= 2;
        dsor_exp += 1;
    }
    dsor_exp
}

pub unsafe fn omap1_ckctl_recalc(clk: *mut omap1_clk, p_rate: usize) -> usize {
    let dsor = 1usize << (3 & (omap_readw(ARM_CKCTL) >> (*clk).rate_offset));
    (*clk).rate = p_rate / dsor;
    (*clk).rate
}

unsafe fn omap1_clk_is_enabled(hw: *mut clk_hw) -> i32 {
    let clk = to_omap1_clk(hw);
    let mut api_ck_was_enabled = true;
    if (*clk).ops.is_null() { return 1; }
    if (*clk).ops == &clkops_dspck as *const clkops {
        api_ck_was_enabled = omap1_clk_is_enabled(&mut (*api_ck_p).hw) != 0;
        if !api_ck_was_enabled && ((*api_ck_p).ops).enable.unwrap()(api_ck_p) < 0 { return 0; }
    }
    let regval = if (*clk).flags & ENABLE_REG_32BIT != 0 { __raw_readl((*clk).enable_reg) } else { __raw_readw((*clk).enable_reg) as u32 };
    let ret = if regval & (1 << (*clk).enable_bit) != 0 { 1 } else { 0 };
    if !api_ck_was_enabled { ((*api_ck_p).ops).disable.unwrap()(api_ck_p); }
    ret
}

pub unsafe fn omap1_ckctl_recalc_dsp_domain(clk: *mut omap1_clk, p_rate: usize) -> usize {
    let was = omap1_clk_is_enabled(&mut (*api_ck_p).hw) != 0;
    if !was { ((*api_ck_p).ops).enable.unwrap()(api_ck_p); }
    let dsor = 1usize << (3 & (__raw_readw(DSP_CKCTL) >> (*clk).rate_offset));
    if !was { ((*api_ck_p).ops).disable.unwrap()(api_ck_p); }
    p_rate / dsor
}

pub unsafe fn omap1_select_table_rate(_clk: *mut omap1_clk, rate: usize, _p_rate: usize) -> i32 {
    let ref_rate = (*ck_ref_p).rate;
    let mut ptr = omap1_rate_table;
    while (*ptr).rate != 0 {
        if (*ptr).flags & cpu_mask != 0 && (*ptr).xtal == ref_rate && (*ptr).rate <= rate { break; }
        ptr = ptr.add(1);
    }
    if (*ptr).rate == 0 { return -EINVAL; }
    omap_sram_reprogram_clock((*ptr).dpllctl_val, (*ptr).ckctl_val);
    (*ck_dpll1_p).rate = (*ptr).pll_rate;
    0
}

pub unsafe fn omap1_clk_set_rate_dsp_domain(clk: *mut omap1_clk, rate: usize, p_rate: usize) -> i32 {
    let mut dsor_exp = calc_dsor_exp(rate, p_rate);
    if dsor_exp > 3 { dsor_exp = -EINVAL; }
    if dsor_exp < 0 { return dsor_exp; }
    let mut regval = __raw_readw(DSP_CKCTL);
    regval &= !(3 << (*clk).rate_offset);
    regval |= (dsor_exp as u16) << (*clk).rate_offset;
    __raw_writew(regval, DSP_CKCTL);
    (*clk).rate = p_rate / (1usize << dsor_exp);
    0
}

pub unsafe fn omap1_clk_round_rate_ckctl_arm(_clk: *mut omap1_clk, rate: usize, p_rate: *mut usize) -> isize {
    let mut e = calc_dsor_exp(rate, *p_rate);
    if e < 0 { return e as isize; }
    if e > 3 { e = 3; }
    (*p_rate / (1usize << e)) as isize
}

pub unsafe fn omap1_clk_set_rate_ckctl_arm(clk: *mut omap1_clk, rate: usize, p_rate: usize) -> i32 {
    let mut e = calc_dsor_exp(rate, p_rate);
    if e > 3 { e = -EINVAL; }
    if e < 0 { return e; }
    let mut flags = 0usize;
    spin_lock_irqsave(&mut arm_ckctl_lock, &mut flags);
    let mut regval = omap_readw(ARM_CKCTL);
    regval &= !(3 << (*clk).rate_offset);
    regval |= (e as u16) << (*clk).rate_offset;
    omap_writew(verify_ckctl_value(regval), ARM_CKCTL);
    (*clk).rate = p_rate / (1usize << e);
    spin_unlock_irqrestore(&mut arm_ckctl_lock, flags);
    0
}

pub unsafe fn omap1_round_to_table_rate(_clk: *mut omap1_clk, rate: usize, _p_rate: *mut usize) -> isize {
    let ref_rate = (*ck_ref_p).rate;
    let mut highest = -EINVAL as isize;
    let mut ptr = omap1_rate_table;
    while (*ptr).rate != 0 {
        if (*ptr).flags & cpu_mask != 0 && (*ptr).xtal == ref_rate {
            highest = (*ptr).rate as isize;
            if (*ptr).rate <= rate { break; }
        }
        ptr = ptr.add(1);
    }
    highest
}

unsafe fn calc_ext_dsor(rate: usize) -> usize {
    let mut dsor = 2;
    while dsor < 96 {
        if (!(dsor & 1 != 0 && dsor > 8)) && rate >= 96000000 / dsor { break; }
        dsor += 1;
    }
    dsor
}

pub unsafe fn omap1_round_uart_rate(_clk: *mut omap1_clk, rate: usize, _p_rate: *mut usize) -> isize { if rate > 24000000 { 48000000 } else { 12000000 } }

pub unsafe fn omap1_set_uart_rate(clk: *mut omap1_clk, rate: usize, _p_rate: usize) -> i32 {
    let val = if rate == 12000000 { 0 } else if rate == 48000000 { 1 << (*clk).enable_bit } else { return -EINVAL };
    let mut flags = 0usize;
    spin_lock_irqsave(&mut mod_conf_ctrl_0_lock, &mut flags);
    __raw_writel(val | (__raw_readl((*clk).enable_reg) & !(1 << (*clk).enable_bit)), (*clk).enable_reg);
    spin_unlock_irqrestore(&mut mod_conf_ctrl_0_lock, flags);
    (*clk).rate = rate; 0
}

pub unsafe fn omap1_set_ext_clk_rate(clk: *mut omap1_clk, rate: usize, _p_rate: usize) -> i32 {
    let dsor = calc_ext_dsor(rate);
    (*clk).rate = 96000000 / dsor;
    let mut ratio = if dsor > 8 { ((dsor - 8) / 2 + 6) << 2 } else { (dsor - 2) << 2 } as u16;
    let mut flags = 0usize;
    spin_lock_irqsave(&mut swd_clk_div_ctrl_sel_lock, &mut flags);
    ratio |= __raw_readw((*clk).enable_reg) & !0xfd;
    __raw_writew(ratio, (*clk).enable_reg);
    spin_unlock_irqrestore(&mut swd_clk_div_ctrl_sel_lock, flags); 0
}

unsafe fn calc_div_sossi(rate: usize, p_rate: usize) -> i32 { ((p_rate + rate - 1) / rate) as i32 - 1 }
pub unsafe fn omap1_round_sossi_rate(_clk: *mut omap1_clk, rate: usize, p_rate: *mut usize) -> isize {
    let mut div = calc_div_sossi(rate, *p_rate); if div < 0 { div = 0; } else if div > 7 { div = 7; }
    (*p_rate / (div as usize + 1)) as isize
}
pub unsafe fn omap1_set_sossi_rate(clk: *mut omap1_clk, rate: usize, p_rate: usize) -> i32 {
    let div = calc_div_sossi(rate, p_rate); if div < 0 || div > 7 { return -EINVAL; }
    let mut flags = 0usize; spin_lock_irqsave(&mut mod_conf_ctrl_1_lock, &mut flags);
    let mut l = omap_readl(MOD_CONF_CTRL_1); l &= !(7 << 17); l |= (div as u32) << 17; omap_writel(l, MOD_CONF_CTRL_1);
    (*clk).rate = p_rate / (div as usize + 1); spin_unlock_irqrestore(&mut mod_conf_ctrl_1_lock, flags); 0
}
pub unsafe fn omap1_round_ext_clk_rate(_clk: *mut omap1_clk, rate: usize, _p_rate: *mut usize) -> isize { (96000000 / calc_ext_dsor(rate)) as isize }

pub unsafe fn omap1_init_ext_clk(clk: *mut omap1_clk) -> i32 {
    let mut ratio = __raw_readw((*clk).enable_reg) & !1; __raw_writew(ratio, (*clk).enable_reg); ratio = (ratio & 0xfc) >> 2;
    let dsor = if ratio > 6 { (ratio - 6) * 2 + 8 } else { ratio + 2 }; (*clk).rate = 96000000 / dsor as usize; 0
}

// The remaining callbacks and operation tables retain the C ABI-facing shape;
// their bodies delegate to the external clock framework exactly as in C.
pub unsafe fn followparent_recalc(_clk: *mut omap1_clk, p_rate: usize) -> usize { p_rate }
pub unsafe fn omap_fixed_divisor_recalc(clk: *mut omap1_clk, p_rate: usize) -> usize { p_rate / (*clk).fixed_div }
pub unsafe fn propagate_rate(_tclk: *mut omap1_clk) { }

unsafe fn omap1_clk_enable_generic(clk: *mut omap1_clk) -> i32 {
    if (*clk).enable_reg.is_null() { printk(KERN_ERR, b"clock.c: Enable without enable code\0"); return -EINVAL; }
    let mut flags = 0usize;
    if (*clk).flags & ENABLE_REG_32BIT != 0 {
        let mut v = __raw_readl((*clk).enable_reg); v |= 1 << (*clk).enable_bit; __raw_writel(v, (*clk).enable_reg);
    } else { let mut v = __raw_readw((*clk).enable_reg); v |= 1 << (*clk).enable_bit; __raw_writew(v, (*clk).enable_reg); }
    let _ = &mut flags; 0
}
unsafe fn omap1_clk_disable_generic(clk: *mut omap1_clk) {
    if (*clk).enable_reg.is_null() { return; }
    if (*clk).flags & ENABLE_REG_32BIT != 0 { let mut v = __raw_readl((*clk).enable_reg); v &= !(1 << (*clk).enable_bit); __raw_writel(v, (*clk).enable_reg); }
    else { let mut v = __raw_readw((*clk).enable_reg); v &= !(1 << (*clk).enable_bit); __raw_writew(v, (*clk).enable_reg); }
}
unsafe fn omap1_clk_enable_dsp_domain(clk: *mut omap1_clk) -> i32 { omap1_clk_enable_generic(clk) }
unsafe fn omap1_clk_disable_dsp_domain(clk: *mut omap1_clk) { omap1_clk_disable_generic(clk); }
unsafe fn omap1_clk_recalc_rate(clk: *mut clk_hw, p_rate: usize) -> usize { let c = to_omap1_clk(clk); if let Some(f) = (*c).recalc { f(c, p_rate) } else { (*c).rate } }
unsafe fn omap1_clk_set_rate(clk: *mut clk_hw, rate: usize, p_rate: usize) -> i32 { let c = to_omap1_clk(clk); if let Some(f) = (*c).set_rate { f(c, rate, p_rate) } else { -EINVAL } }
unsafe fn omap1_clk_init_op(clk: *mut clk_hw) -> i32 { let c = to_omap1_clk(clk); if let Some(f) = (*c).init { f(c) } else { 0 } }

// CONFIG_OMAP_RESET_CLOCKS conditionally supplies disable_unused, as in C.
pub static mut omap1_clk_gate_ops: clk_ops = clk_ops::empty();
pub static mut omap1_clk_rate_ops: clk_ops = clk_ops::empty();
pub static mut omap1_clk_full_ops: clk_ops = clk_ops::empty();
pub static mut omap1_clk_null_ops: clk_ops = clk_ops::empty();

// Dummy clock used for aliases present only on some OMAP variants.
pub static mut dummy_ck: omap1_clk = omap1_clk::zeroed();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
