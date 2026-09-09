// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 */

// Linux kernel dependencies are supplied by other translated files.

const PERIPH_CLK_UART_DIV_ENB: u32 = 1 << 24;

#[inline]
unsafe fn pll_out_override(p: *const tegra_clk_frac_div) -> u32 {
    1u32 << ((*p).shift - 6)
}

#[inline]
unsafe fn div_mask(d: *const tegra_clk_frac_div) -> u32 {
    (1u32 << (*d).width) - 1
}

#[inline]
unsafe fn get_mul(d: *const tegra_clk_frac_div) -> i32 {
    1i32 << (*d).frac_width
}

#[inline]
unsafe fn get_max_div(d: *const tegra_clk_frac_div) -> u32 {
    div_mask(d)
}

unsafe fn get_div(
    divider: *mut tegra_clk_frac_div,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> i32 {
    let div = div_frac_get(
        rate,
        parent_rate,
        (*divider).width,
        (*divider).frac_width,
        (*divider).flags,
    );

    if div < 0 { 0 } else { div }
}

unsafe extern "C" fn clk_frac_div_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: c_ulong,
) -> c_ulong {
    let divider = to_clk_frac_div(hw);
    let reg: u32 = readl_relaxed((*divider).reg);
    let mut rate: u64 = parent_rate as u64;

    if ((*divider).flags & TEGRA_DIVIDER_UART) != 0
        && (reg & PERIPH_CLK_UART_DIV_ENB) == 0
    {
        return rate as c_ulong;
    }

    let div = ((reg >> (*divider).shift) & div_mask(divider)) as u64;
    let mul = get_mul(divider) as u64;

    let div = div + mul;
    rate = rate.wrapping_mul(mul);
    rate = rate.wrapping_add(div - 1);
    rate /= div;

    rate as c_ulong
}

unsafe extern "C" fn clk_frac_div_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let divider = to_clk_frac_div(hw);
    let output_rate = (*req).best_parent_rate;

    if (*req).rate == 0 {
        (*req).rate = output_rate;
        return 0;
    }

    let div = get_div(divider, (*req).rate, output_rate);
    if div < 0 {
        (*req).rate = (*req).best_parent_rate;
        return 0;
    }

    let mul = get_mul(divider) as c_ulong;
    (*req).rate = (output_rate * mul + (div as c_ulong + mul) - 1)
        / (div as c_ulong + mul);

    0
}

unsafe extern "C" fn clk_frac_div_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> i32 {
    let divider = to_clk_frac_div(hw);
    let div = get_div(divider, rate, parent_rate);
    if div < 0 { return div; }

    let mut flags: c_ulong = 0;
    if !(*divider).lock.is_null() {
        spin_lock_irqsave((*divider).lock, &mut flags);
    }

    let mut val = readl_relaxed((*divider).reg);
    val &= !(div_mask(divider) << (*divider).shift);
    val |= (div as u32) << (*divider).shift;

    if ((*divider).flags & TEGRA_DIVIDER_UART) != 0 {
        if div != 0 { val |= PERIPH_CLK_UART_DIV_ENB; }
        else { val &= !PERIPH_CLK_UART_DIV_ENB; }
    }

    if ((*divider).flags & TEGRA_DIVIDER_FIXED) != 0 {
        val |= pll_out_override(divider);
    }

    writel_relaxed(val, (*divider).reg);

    if !(*divider).lock.is_null() {
        spin_unlock_irqrestore((*divider).lock, flags);
    }

    0
}

unsafe extern "C" fn clk_divider_restore_context(hw: *mut clk_hw) {
    let parent = clk_hw_get_parent(hw);
    let parent_rate = clk_hw_get_rate(parent);
    let rate = clk_hw_get_rate(hw);

    if clk_frac_div_set_rate(hw, rate, parent_rate) < 0 {
        WARN_ON(1);
    }
}

pub static tegra_clk_frac_div_ops: clk_ops = clk_ops {
    .recalc_rate = Some(clk_frac_div_recalc_rate),
    .set_rate = Some(clk_frac_div_set_rate),
    .determine_rate = Some(clk_frac_div_determine_rate),
    .restore_context = Some(clk_divider_restore_context),
};

pub unsafe extern "C" fn tegra_clk_register_divider(
    name: *const c_char,
    parent_name: *const c_char,
    reg: *mut c_void,
    flags: c_ulong,
    clk_divider_flags: u8,
    shift: u8,
    width: u8,
    frac_width: u8,
    lock: *mut spinlock_t,
) -> *mut clk {
    let divider = kzalloc_obj::<tegra_clk_frac_div>();
    if divider.is_null() {
        pr_err!("{}: could not allocate fractional divider clk\n", "tegra_clk_register_divider");
        return ERR_PTR(-ENOMEM);
    }

    let mut init = clk_init_data {
        name,
        ops: &tegra_clk_frac_div_ops,
        flags,
        parent_names: if !parent_name.is_null() { &parent_name } else { core::ptr::null() },
        num_parents: if !parent_name.is_null() { 1 } else { 0 },
    };

    (*divider).reg = reg;
    (*divider).shift = shift;
    (*divider).width = width;
    (*divider).frac_width = frac_width;
    (*divider).lock = lock;
    (*divider).flags = clk_divider_flags;

    /* Data in .init is copied by clk_register(), so stack variable OK */
    (*divider).hw.init = &mut init;

    let clk = clk_register(core::ptr::null_mut(), &mut (*divider).hw);
    if IS_ERR(clk) { kfree(divider as *mut c_void); }
    clk
}

static mc_div_table: [clk_div_table; 3] = [
    clk_div_table { val: 0, div: 2 },
    clk_div_table { val: 1, div: 1 },
    clk_div_table { val: 0, div: 0 },
];

pub unsafe extern "C" fn tegra_clk_register_mc(
    name: *const c_char,
    parent_name: *const c_char,
    reg: *mut c_void,
    lock: *mut spinlock_t,
) -> *mut clk {
    clk_register_divider_table(
        core::ptr::null_mut(), name, parent_name, CLK_IS_CRITICAL,
        reg, 16, 1, CLK_DIVIDER_READ_ONLY, mc_div_table.as_ptr(), lock,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
