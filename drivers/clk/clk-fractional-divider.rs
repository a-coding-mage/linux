// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2014 Intel Corporation
 *
 * Adjustable fractional divider clock implementation.
 * Uses rational best approximation algorithm.
 *
 * Dependencies supplied by the surrounding kernel translation are intentionally
 * referenced but not defined here.
 */

// Linux kernel headers and "clk-fractional-divider.h" provide the external
// types, constants, macros, and functions used below.

#[inline]
unsafe fn clk_fd_readl(fd: *mut clk_fractional_divider) -> u32 {
    if (*fd).flags & CLK_FRAC_DIVIDER_BIG_ENDIAN != 0 {
        return ioread32be((*fd).reg);
    }
    readl((*fd).reg)
}

#[inline]
unsafe fn clk_fd_writel(fd: *mut clk_fractional_divider, val: u32) {
    if (*fd).flags & CLK_FRAC_DIVIDER_BIG_ENDIAN != 0 {
        iowrite32be(val, (*fd).reg);
    } else {
        writel(val, (*fd).reg);
    }
}

unsafe fn clk_fd_get_div(hw: *mut clk_hw, fract: *mut u32_fract) {
    let fd = to_clk_fd(hw);
    let mut flags: c_ulong = 0;
    let m: c_ulong;
    let n: c_ulong;
    let mmask: u32;
    let nmask: u32;
    let val: u32;

    if !(*fd).lock.is_null() {
        spin_lock_irqsave((*fd).lock, &mut flags);
    } else {
        __acquire((*fd).lock);
    }

    val = clk_fd_readl(fd);

    if !(*fd).lock.is_null() {
        spin_unlock_irqrestore((*fd).lock, flags);
    } else {
        __release((*fd).lock);
    }

    mmask = genmask((*fd).mwidth as u32 - 1, 0) << (*fd).mshift;
    nmask = genmask((*fd).nwidth as u32 - 1, 0) << (*fd).nshift;

    m = ((val & mmask) >> (*fd).mshift) as c_ulong;
    n = ((val & nmask) >> (*fd).nshift) as c_ulong;

    let (m, n) = if (*fd).flags & CLK_FRAC_DIVIDER_ZERO_BASED != 0 {
        (m + 1, n + 1)
    } else {
        (m, n)
    };

    (*fract).numerator = m;
    (*fract).denominator = n;
}

unsafe fn clk_fd_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let mut fract = u32_fract { numerator: 0, denominator: 0 };
    clk_fd_get_div(hw, &mut fract);

    if fract.numerator == 0 || fract.denominator == 0 {
        return parent_rate;
    }

    let mut ret = (parent_rate as u64).wrapping_mul(fract.numerator as u64);
    ret /= fract.denominator as u64;
    ret as c_ulong
}

pub unsafe fn clk_fractional_divider_general_approximation(
    hw: *mut clk_hw,
    mut rate: c_ulong,
    parent_rate: *mut c_ulong,
    m: *mut c_ulong,
    n: *mut c_ulong,
) {
    let fd = to_clk_fd(hw);
    let max_m: c_ulong;
    let max_n: c_ulong;

    if (*fd).flags & CLK_FRAC_DIVIDER_POWER_OF_TWO_PS != 0 {
        let scale = fls_long((*parent_rate / rate) - 1);
        if scale > (*fd).nwidth as c_ulong {
            rate <<= scale - (*fd).nwidth as c_ulong;
        }
    }

    if (*fd).flags & CLK_FRAC_DIVIDER_ZERO_BASED != 0 {
        max_m = 1 as c_ulong << (*fd).mwidth;
        max_n = 1 as c_ulong << (*fd).nwidth;
    } else {
        max_m = genmask((*fd).mwidth as u32 - 1, 0) as c_ulong;
        max_n = genmask((*fd).nwidth as u32 - 1, 0) as c_ulong;
    }

    rational_best_approximation(rate, *parent_rate, max_m, max_n, m, n);
}

unsafe fn clk_fd_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let fd = to_clk_fd(hw);
    let mut m: c_ulong = 0;
    let mut n: c_ulong = 0;

    if (*req).rate == 0 || (!clk_hw_can_set_rate_parent(hw) && (*req).rate >= (*req).best_parent_rate) {
        (*req).rate = (*req).best_parent_rate;
        return 0;
    }

    if let Some(approximation) = (*fd).approximation {
        approximation(hw, (*req).rate, &mut (*req).best_parent_rate, &mut m, &mut n);
    } else {
        clk_fractional_divider_general_approximation(
            hw, (*req).rate, &mut (*req).best_parent_rate, &mut m, &mut n,
        );
    }

    let mut ret = ((*req).best_parent_rate as u64).wrapping_mul(m as u64);
    ret /= n as u64;
    (*req).rate = ret as c_ulong;
    0
}

unsafe fn clk_fd_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let fd = to_clk_fd(hw);
    let mut flags: c_ulong = 0;
    let mut m: c_ulong = 0;
    let mut n: c_ulong = 0;
    let max_m: c_ulong;
    let max_n: c_ulong;

    if (*fd).flags & CLK_FRAC_DIVIDER_ZERO_BASED != 0 {
        max_m = 1 as c_ulong << (*fd).mwidth;
        max_n = 1 as c_ulong << (*fd).nwidth;
    } else {
        max_m = genmask((*fd).mwidth as u32 - 1, 0) as c_ulong;
        max_n = genmask((*fd).nwidth as u32 - 1, 0) as c_ulong;
    }
    rational_best_approximation(rate, parent_rate, max_m, max_n, &mut m, &mut n);

    if (*fd).flags & CLK_FRAC_DIVIDER_ZERO_BASED != 0 {
        m -= 1;
        n -= 1;
    }

    let mmask = genmask((*fd).mwidth as u32 - 1, 0) << (*fd).mshift;
    let nmask = genmask((*fd).nwidth as u32 - 1, 0) << (*fd).nshift;

    if !(*fd).lock.is_null() { spin_lock_irqsave((*fd).lock, &mut flags); } else { __acquire((*fd).lock); }
    let mut val = clk_fd_readl(fd);
    val &= !(mmask | nmask);
    val |= ((m as u32) << (*fd).mshift) | ((n as u32) << (*fd).nshift);
    clk_fd_writel(fd, val);
    if !(*fd).lock.is_null() { spin_unlock_irqrestore((*fd).lock, flags); } else { __release((*fd).lock); }
    0
}

pub const clk_fractional_divider_ops: clk_ops = clk_ops {
    recalc_rate: Some(clk_fd_recalc_rate),
    determine_rate: Some(clk_fd_determine_rate),
    set_rate: Some(clk_fd_set_rate),
    // debug_init is provided by CONFIG_DEBUG_FS in the kernel build.
};

pub unsafe fn clk_hw_register_fractional_divider(
    dev: *mut device,
    name: *const c_char,
    parent_name: *const c_char,
    flags: c_ulong,
    reg: *mut c_void,
    mshift: u8,
    mwidth: u8,
    nshift: u8,
    nwidth: u8,
    clk_divider_flags: u8,
    lock: *mut spinlock_t,
) -> *mut clk_hw {
    let fd = kzalloc::<clk_fractional_divider>();
    if fd.is_null() {
        return err_ptr(-12);
    }

    let mut init = clk_init_data {
        name,
        ops: &clk_fractional_divider_ops,
        flags,
        parent_names: if !parent_name.is_null() { &parent_name } else { core::ptr::null() },
        num_parents: if !parent_name.is_null() { 1 } else { 0 },
    };

    (*fd).reg = reg;
    (*fd).mshift = mshift;
    (*fd).mwidth = mwidth;
    (*fd).nshift = nshift;
    (*fd).nwidth = nwidth;
    (*fd).flags = clk_divider_flags;
    (*fd).lock = lock;
    (*fd).hw.init = &mut init;

    let hw = &mut (*fd).hw as *mut clk_hw;
    let ret = clk_hw_register(dev, hw);
    if ret != 0 {
        kfree(fd as *mut c_void);
        return err_ptr(ret);
    }
    hw
}

pub unsafe fn clk_register_fractional_divider(
    dev: *mut device,
    name: *const c_char,
    parent_name: *const c_char,
    flags: c_ulong,
    reg: *mut c_void,
    mshift: u8,
    mwidth: u8,
    nshift: u8,
    nwidth: u8,
    clk_divider_flags: u8,
    lock: *mut spinlock_t,
) -> *mut clk {
    let hw = clk_hw_register_fractional_divider(
        dev, name, parent_name, flags, reg, mshift, mwidth, nshift, nwidth,
        clk_divider_flags, lock,
    );
    if is_err(hw) {
        return err_cast(hw);
    }
    (*hw).clk
}

pub unsafe fn clk_hw_unregister_fractional_divider(hw: *mut clk_hw) {
    let fd = to_clk_fd(hw);
    clk_hw_unregister(hw);
    kfree(fd as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
