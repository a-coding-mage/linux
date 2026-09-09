// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Maxime Ripard
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn ccu_phase_get_phase(hw: *mut clk_hw) -> i32 {
    let phase: *mut ccu_phase = hw_to_ccu_phase(hw);
    let mut parent: *mut clk_hw;
    let mut grandparent: *mut clk_hw;
    let mut parent_rate: u32;
    let mut grandparent_rate: u32;
    let mut step: u16;
    let mut parent_div: u16;
    let mut reg: u32;
    let mut delay: u8;

    reg = readl((*(*phase).common.base).add((*phase).common.reg as usize));
    delay = (reg >> (*phase).shift) as u8;
    delay &= ((1u32 << (*phase).width) - 1) as u8;

    if delay == 0 {
        return 180;
    }

    /* Get our parent clock, it's the one that can adjust its rate */
    parent = clk_hw_get_parent(hw);
    if parent.is_null() {
        return -EINVAL;
    }

    /* And its rate */
    parent_rate = clk_hw_get_rate(parent);
    if parent_rate == 0 {
        return -EINVAL;
    }

    /* Now, get our parent's parent (most likely some PLL) */
    grandparent = clk_hw_get_parent(parent);
    if grandparent.is_null() {
        return -EINVAL;
    }

    /* And its rate */
    grandparent_rate = clk_hw_get_rate(grandparent);
    if grandparent_rate == 0 {
        return -EINVAL;
    }

    /* Get our parent clock divider */
    parent_div = (grandparent_rate / parent_rate) as u16;

    step = div_round_closest(360, parent_div);
    (delay as u16 * step) as i32
}

unsafe fn ccu_phase_set_phase(hw: *mut clk_hw, degrees: i32) -> i32 {
    let phase: *mut ccu_phase = hw_to_ccu_phase(hw);
    let mut parent: *mut clk_hw;
    let mut grandparent: *mut clk_hw;
    let mut parent_rate: u32;
    let mut grandparent_rate: u32;
    let mut flags: ulong;
    let mut reg: u32;
    let mut delay: u8;

    /* Get our parent clock, it's the one that can adjust its rate */
    parent = clk_hw_get_parent(hw);
    if parent.is_null() {
        return -EINVAL;
    }

    /* And its rate */
    parent_rate = clk_hw_get_rate(parent);
    if parent_rate == 0 {
        return -EINVAL;
    }

    /* Now, get our parent's parent (most likely some PLL) */
    grandparent = clk_hw_get_parent(parent);
    if grandparent.is_null() {
        return -EINVAL;
    }

    /* And its rate */
    grandparent_rate = clk_hw_get_rate(grandparent);
    if grandparent_rate == 0 {
        return -EINVAL;
    }

    if degrees != 180 {
        let step: u16;
        let parent_div: u16;

        /* Get our parent divider */
        parent_div = (grandparent_rate / parent_rate) as u16;

        /*
         * We can only outphase the clocks by multiple of the
         * PLL's period.
         *
         * Since our parent clock is only a divider, and the
         * formula to get the outphasing in degrees is deg =
         * 360 * delta / period
         *
         * If we simplify this formula, we can see that the
         * only thing that we're concerned about is the number
         * of period we want to outphase our clock from, and
         * the divider set by our parent clock.
         */
        step = div_round_closest(360, parent_div);
        delay = div_round_closest(degrees, step) as u8;
    } else {
        delay = 0;
    }

    flags = 0;
    spin_lock_irqsave((*phase).common.lock, &mut flags);
    reg = readl((*(*phase).common.base).add((*phase).common.reg as usize));
    reg &= !genmask((*phase).width + (*phase).shift - 1, (*phase).shift);
    writel(
        reg | ((delay as u32) << (*phase).shift),
        (*(*phase).common.base).add((*phase).common.reg as usize),
    );
    spin_unlock_irqrestore((*phase).common.lock, flags);

    0
}

pub static ccu_phase_ops: clk_ops = clk_ops {
    get_phase: Some(ccu_phase_get_phase),
    set_phase: Some(ccu_phase_set_phase),
};

// External declarations and kernel-provided types/macros.
extern "C" {
    static EINVAL: i32;
    fn hw_to_ccu_phase(hw: *mut clk_hw) -> *mut ccu_phase;
    fn readl(addr: *mut u32) -> u32;
    fn writel(value: u32, addr: *mut u32);
    fn clk_hw_get_parent(hw: *mut clk_hw) -> *mut clk_hw;
    fn clk_hw_get_rate(hw: *mut clk_hw) -> u32;
    fn div_round_closest(value: i32, divisor: u16) -> u16;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
