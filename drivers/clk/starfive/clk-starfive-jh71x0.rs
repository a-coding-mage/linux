// SPDX-License-Identifier: GPL-2.0
/*
 * StarFive JH71X0 Clock Generator Driver
 *
 * Copyright (C) 2021-2022 Emil Renner Berthing <kernel@esmil.dk>
 */

// Linux clock-provider, debugfs, device, io, and clk-starfive-jh71x0.h
// declarations are supplied by the surrounding kernel translation.

unsafe fn jh71x0_clk_from(hw: *mut clk_hw) -> *mut jh71x0_clk {
    container_of!(hw, jh71x0_clk, hw)
}

unsafe fn jh71x0_priv_from(clk: *mut jh71x0_clk) -> *mut jh71x0_clk_priv {
    container_of!(clk, jh71x0_clk_priv, reg[(*clk).idx as usize])
}

unsafe fn jh71x0_clk_reg_get(clk: *mut jh71x0_clk) -> u32 {
    let priv_ = jh71x0_priv_from(clk);
    let reg = (*priv_).base.add(4 * (*clk).idx as usize);
    readl_relaxed(reg)
}

unsafe fn jh71x0_clk_reg_rmw(clk: *mut jh71x0_clk, mask: u32, mut value: u32) {
    let priv_ = jh71x0_priv_from(clk);
    let reg = (*priv_).base.add(4 * (*clk).idx as usize);
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&mut (*priv_).rmw_lock, &mut flags);
    value |= readl_relaxed(reg) & !mask;
    writel_relaxed(value, reg);
    spin_unlock_irqrestore(&mut (*priv_).rmw_lock, flags);
}

unsafe extern "C" fn jh71x0_clk_enable(hw: *mut clk_hw) -> c_int {
    let clk = jh71x0_clk_from(hw);
    jh71x0_clk_reg_rmw(clk, JH71X0_CLK_ENABLE, JH71X0_CLK_ENABLE);
    0
}

unsafe extern "C" fn jh71x0_clk_disable(hw: *mut clk_hw) {
    let clk = jh71x0_clk_from(hw);
    jh71x0_clk_reg_rmw(clk, JH71X0_CLK_ENABLE, 0);
}

unsafe extern "C" fn jh71x0_clk_is_enabled(hw: *mut clk_hw) -> c_int {
    let clk = jh71x0_clk_from(hw);
    if jh71x0_clk_reg_get(clk) & JH71X0_CLK_ENABLE != 0 { 1 } else { 0 }
}

unsafe extern "C" fn jh71x0_clk_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let clk = jh71x0_clk_from(hw);
    let div = jh71x0_clk_reg_get(clk) & JH71X0_CLK_DIV_MASK;
    if div != 0 { parent_rate / div as c_ulong } else { 0 }
}

unsafe extern "C" fn jh71x0_clk_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let clk = jh71x0_clk_from(hw);
    let parent = (*req).best_parent_rate;
    let rate = clamp!((*req).rate, (*req).min_rate, (*req).max_rate);
    let div = core::cmp::min(div_round_up(parent, rate), (*clk).max_div as c_ulong);
    let mut result = parent / div;
    if result < (*req).min_rate && div > 1 { result = parent / (div - 1); }
    (*req).rate = result;
    0
}

unsafe extern "C" fn jh71x0_clk_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let clk = jh71x0_clk_from(hw);
    let div = clamp!(div_round_closest(parent_rate, rate), 1, (*clk).max_div as c_ulong);
    jh71x0_clk_reg_rmw(clk, JH71X0_CLK_DIV_MASK, div as u32);
    0
}

unsafe extern "C" fn jh71x0_clk_frac_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let clk = jh71x0_clk_from(hw);
    let reg = jh71x0_clk_reg_get(clk);
    let div100 = 100 * (reg & JH71X0_CLK_INT_MASK) as c_ulong
        + ((reg & JH71X0_CLK_FRAC_MASK) >> JH71X0_CLK_FRAC_SHIFT) as c_ulong;
    if div100 >= JH71X0_CLK_FRAC_MIN as c_ulong { 100 * parent_rate / div100 } else { 0 }
}

unsafe extern "C" fn jh71x0_clk_frac_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let parent100 = 100 * (*req).best_parent_rate;
    let rate = clamp!((*req).rate, (*req).min_rate, (*req).max_rate);
    let mut div100 = clamp!(div_round_closest(parent100, rate), JH71X0_CLK_FRAC_MIN as c_ulong, JH71X0_CLK_FRAC_MAX as c_ulong);
    let mut result = parent100 / div100;
    if result > (*req).max_rate && div100 < JH71X0_CLK_FRAC_MAX as c_ulong { div100 += 1; result = parent100 / div100; }
    if result < (*req).min_rate && div100 > JH71X0_CLK_FRAC_MIN as c_ulong { result = parent100 / (div100 - 1); }
    (*req).rate = result;
    0
}

unsafe extern "C" fn jh71x0_clk_frac_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let clk = jh71x0_clk_from(hw);
    let div100 = clamp!(div_round_closest(100 * parent_rate, rate), JH71X0_CLK_FRAC_MIN as c_ulong, JH71X0_CLK_FRAC_MAX as c_ulong);
    let value = (((div100 % 100) as u32) << JH71X0_CLK_FRAC_SHIFT) | (div100 / 100) as u32;
    jh71x0_clk_reg_rmw(clk, JH71X0_CLK_DIV_MASK, value);
    0
}

unsafe extern "C" fn jh71x0_clk_get_parent(hw: *mut clk_hw) -> u8 {
    let clk = jh71x0_clk_from(hw);
    ((jh71x0_clk_reg_get(clk) & JH71X0_CLK_MUX_MASK) >> JH71X0_CLK_MUX_SHIFT) as u8
}

unsafe extern "C" fn jh71x0_clk_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    let clk = jh71x0_clk_from(hw);
    jh71x0_clk_reg_rmw(clk, JH71X0_CLK_MUX_MASK, (index as u32) << JH71X0_CLK_MUX_SHIFT);
    0
}

unsafe extern "C" fn jh71x0_clk_get_phase(hw: *mut clk_hw) -> c_int {
    let clk = jh71x0_clk_from(hw);
    if jh71x0_clk_reg_get(clk) & JH71X0_CLK_INVERT != 0 { 180 } else { 0 }
}

unsafe extern "C" fn jh71x0_clk_set_phase(hw: *mut clk_hw, degrees: c_int) -> c_int {
    let clk = jh71x0_clk_from(hw);
    let value = match degrees { 0 => 0, 180 => JH71X0_CLK_INVERT as u32, _ => return -EINVAL };
    jh71x0_clk_reg_rmw(clk, JH71X0_CLK_INVERT, value);
    0
}

// CONFIG_DEBUG_FS selects the debugfs implementation; otherwise this is NULL.
#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe extern "C" fn jh71x0_clk_debug_init(hw: *mut clk_hw, dentry: *mut dentry) {
    static JH71X0_CLK_REG: debugfs_reg32 = debugfs_reg32 { name: "CTRL", offset: 0 };
    let clk = jh71x0_clk_from(hw);
    let priv_ = jh71x0_priv_from(clk);
    let regset = devm_kzalloc((*priv_).dev, core::mem::size_of::<debugfs_regset32>(), GFP_KERNEL) as *mut debugfs_regset32;
    if regset.is_null() { return; }
    (*regset).regs = &JH71X0_CLK_REG;
    (*regset).nregs = 1;
    (*regset).base = (*priv_).base.add(4 * (*clk).idx as usize);
    debugfs_create_regset32("registers", 0o400, dentry, regset);
}

// Operation tables correspond directly to the C clk_ops initializers.
static JH71X0_CLK_GATE_OPS: clk_ops = clk_ops { enable: Some(jh71x0_clk_enable), disable: Some(jh71x0_clk_disable), is_enabled: Some(jh71x0_clk_is_enabled), debug_init: Some(jh71x0_clk_debug_init), ..clk_ops::EMPTY };
static JH71X0_CLK_DIV_OPS: clk_ops = clk_ops { recalc_rate: Some(jh71x0_clk_recalc_rate), determine_rate: Some(jh71x0_clk_determine_rate), set_rate: Some(jh71x0_clk_set_rate), debug_init: Some(jh71x0_clk_debug_init), ..clk_ops::EMPTY };
static JH71X0_CLK_FDIV_OPS: clk_ops = clk_ops { recalc_rate: Some(jh71x0_clk_frac_recalc_rate), determine_rate: Some(jh71x0_clk_frac_determine_rate), set_rate: Some(jh71x0_clk_frac_set_rate), debug_init: Some(jh71x0_clk_debug_init), ..clk_ops::EMPTY };

// The remaining operation tables retain the exact C field combinations.
static JH71X0_CLK_GDIV_OPS: clk_ops = clk_ops { enable: Some(jh71x0_clk_enable), disable: Some(jh71x0_clk_disable), is_enabled: Some(jh71x0_clk_is_enabled), recalc_rate: Some(jh71x0_clk_recalc_rate), determine_rate: Some(jh71x0_clk_determine_rate), set_rate: Some(jh71x0_clk_set_rate), debug_init: Some(jh71x0_clk_debug_init), ..clk_ops::EMPTY };
static JH71X0_CLK_MUX_OPS: clk_ops = clk_ops { determine_rate: Some(__clk_mux_determine_rate), set_parent: Some(jh71x0_clk_set_parent), get_parent: Some(jh71x0_clk_get_parent), debug_init: Some(jh71x0_clk_debug_init), ..clk_ops::EMPTY };
static JH71X0_CLK_GMUX_OPS: clk_ops = clk_ops { enable: Some(jh71x0_clk_enable), disable: Some(jh71x0_clk_disable), is_enabled: Some(jh71x0_clk_is_enabled), determine_rate: Some(__clk_mux_determine_rate), set_parent: Some(jh71x0_clk_set_parent), get_parent: Some(jh71x0_clk_get_parent), debug_init: Some(jh71x0_clk_debug_init), ..clk_ops::EMPTY };
static JH71X0_CLK_MDIV_OPS: clk_ops = clk_ops { recalc_rate: Some(jh71x0_clk_recalc_rate), determine_rate: Some(jh71x0_clk_determine_rate), get_parent: Some(jh71x0_clk_get_parent), set_parent: Some(jh71x0_clk_set_parent), set_rate: Some(jh71x0_clk_set_rate), debug_init: Some(jh71x0_clk_debug_init), ..clk_ops::EMPTY };
static JH71X0_CLK_GMD_OPS: clk_ops = clk_ops { enable: Some(jh71x0_clk_enable), disable: Some(jh71x0_clk_disable), is_enabled: Some(jh71x0_clk_is_enabled), recalc_rate: Some(jh71x0_clk_recalc_rate), determine_rate: Some(jh71x0_clk_determine_rate), get_parent: Some(jh71x0_clk_get_parent), set_parent: Some(jh71x0_clk_set_parent), set_rate: Some(jh71x0_clk_set_rate), debug_init: Some(jh71x0_clk_debug_init), ..clk_ops::EMPTY };
static JH71X0_CLK_INV_OPS: clk_ops = clk_ops { get_phase: Some(jh71x0_clk_get_phase), set_phase: Some(jh71x0_clk_set_phase), debug_init: Some(jh71x0_clk_debug_init), ..clk_ops::EMPTY };

#[no_mangle]
pub unsafe extern "C" fn starfive_jh71x0_clk_ops(max: u32) -> *const clk_ops {
    if max & JH71X0_CLK_DIV_MASK != 0 {
        if max & JH71X0_CLK_MUX_MASK != 0 { return if max & JH71X0_CLK_ENABLE != 0 { &JH71X0_CLK_GMD_OPS } else { &JH71X0_CLK_MDIV_OPS }; }
        if max & JH71X0_CLK_ENABLE != 0 { return &JH71X0_CLK_GDIV_OPS; }
        if max == JH71X0_CLK_FRAC_MAX { return &JH71X0_CLK_FDIV_OPS; }
        return &JH71X0_CLK_DIV_OPS;
    }
    if max & JH71X0_CLK_MUX_MASK != 0 { return if max & JH71X0_CLK_ENABLE != 0 { &JH71X0_CLK_GMUX_OPS } else { &JH71X0_CLK_MUX_OPS }; }
    if max & JH71X0_CLK_ENABLE != 0 { &JH71X0_CLK_GATE_OPS } else { &JH71X0_CLK_INV_OPS }
}

#[no_mangle]
pub unsafe extern "C" fn jh71x0_clk_get(clkspec: *mut of_phandle_args, data: *mut c_void) -> *mut clk_hw {
    let priv_ = data as *mut jh71x0_clk_priv;
    let idx = (*clkspec).args[0] as usize;
    if idx < (*priv_).num_reg as usize { &mut (*priv_).reg[idx].hw }
    else { err_ptr(-EINVAL) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
