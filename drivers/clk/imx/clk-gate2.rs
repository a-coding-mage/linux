// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010-2011 Canonical Ltd <jeremy.kerr@canonical.com>
 * Copyright (C) 2011-2012 Mike Turquette, Linaro Ltd <mturquette@linaro.org>
 *
 * Gated clock implementation
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct clk_gate2 {
    pub hw: clk_hw,
    pub reg: *mut core::ffi::c_void,
    pub bit_idx: u8,
    pub cgr_val: u8,
    pub cgr_mask: u8,
    pub flags: u8,
    pub lock: *mut spinlock_t,
    pub share_count: *mut u32,
}

unsafe fn to_clk_gate2(hw: *mut clk_hw) -> *mut clk_gate2 {
    // `hw` is the first member of `struct clk_gate2`.
    hw as *mut clk_gate2
}

unsafe fn clk_gate2_do_shared_clks(hw: *mut clk_hw, enable: bool) {
    let gate = &mut *to_clk_gate2(hw);
    let mut reg: u32;

    reg = readl(gate.reg);
    reg &= !(u32::from(gate.cgr_mask) << gate.bit_idx);
    if enable {
        reg |= u32::from(gate.cgr_val & gate.cgr_mask) << gate.bit_idx;
    }
    writel(reg, gate.reg);
}

unsafe fn clk_gate2_enable(hw: *mut clk_hw) -> i32 {
    let gate = &mut *to_clk_gate2(hw);
    let mut flags: core::ffi::c_ulong = 0;

    spin_lock_irqsave(gate.lock, &mut flags);

    if !gate.share_count.is_null() {
        let count = &mut *gate.share_count;
        let old = *count;
        *count = count.wrapping_add(1);
        if old > 0 {
            spin_unlock_irqrestore(gate.lock, flags);
            return 0;
        }
    }

    clk_gate2_do_shared_clks(hw, true);
    spin_unlock_irqrestore(gate.lock, flags);
    0
}

unsafe fn clk_gate2_disable(hw: *mut clk_hw) {
    let gate = &mut *to_clk_gate2(hw);
    let mut flags: core::ffi::c_ulong = 0;

    spin_lock_irqsave(gate.lock, &mut flags);

    if !gate.share_count.is_null() {
        let count = &mut *gate.share_count;
        if WARN_ON(*count == 0) {
            spin_unlock_irqrestore(gate.lock, flags);
            return;
        } else {
            *count = count.wrapping_sub(1);
            if *count > 0 {
                spin_unlock_irqrestore(gate.lock, flags);
                return;
            }
        }
    }

    clk_gate2_do_shared_clks(hw, false);
    spin_unlock_irqrestore(gate.lock, flags);
}

unsafe fn clk_gate2_reg_is_enabled(reg: *mut core::ffi::c_void, bit_idx: u8,
                                   cgr_val: u8, cgr_mask: u8) -> i32 {
    let val = readl(reg);

    if (((val >> bit_idx) & u32::from(cgr_mask)) == u32::from(cgr_val)) {
        return 1;
    }
    0
}

unsafe fn clk_gate2_is_enabled(hw: *mut clk_hw) -> i32 {
    let gate = &mut *to_clk_gate2(hw);
    let mut flags: core::ffi::c_ulong = 0;
    let ret: i32;

    spin_lock_irqsave(gate.lock, &mut flags);
    ret = clk_gate2_reg_is_enabled(gate.reg, gate.bit_idx, gate.cgr_val, gate.cgr_mask);
    spin_unlock_irqrestore(gate.lock, flags);
    ret
}

unsafe fn clk_gate2_disable_unused(hw: *mut clk_hw) {
    let gate = &mut *to_clk_gate2(hw);
    let mut flags: core::ffi::c_ulong = 0;

    spin_lock_irqsave(gate.lock, &mut flags);
    if gate.share_count.is_null() || *gate.share_count == 0 {
        clk_gate2_do_shared_clks(hw, false);
    }
    spin_unlock_irqrestore(gate.lock, flags);
}

static clk_gate2_ops: clk_ops = clk_ops {
    enable: Some(clk_gate2_enable),
    disable: Some(clk_gate2_disable),
    disable_unused: Some(clk_gate2_disable_unused),
    is_enabled: Some(clk_gate2_is_enabled),
};

pub unsafe fn clk_hw_register_gate2(
    dev: *mut device, name: *const core::ffi::c_char, parent_name: *const core::ffi::c_char,
    flags: core::ffi::c_ulong, reg: *mut core::ffi::c_void, bit_idx: u8, cgr_val: u8,
    cgr_mask: u8, clk_gate2_flags: u8, lock: *mut spinlock_t, share_count: *mut u32,
) -> *mut clk_hw {
    let gate = kzalloc_obj::<clk_gate2>();
    if gate.is_null() {
        return ERR_PTR(-12);
    }

    (*gate).reg = reg;
    (*gate).bit_idx = bit_idx;
    (*gate).cgr_val = cgr_val;
    (*gate).cgr_mask = cgr_mask;
    (*gate).flags = clk_gate2_flags;
    (*gate).lock = lock;
    (*gate).share_count = share_count;

    let init = clk_init_data {
        name,
        ops: &clk_gate2_ops,
        flags,
        parent_names: if !parent_name.is_null() { &parent_name } else { core::ptr::null() },
        num_parents: if !parent_name.is_null() { 1 } else { 0 },
    };
    (*gate).hw.init = &init;
    let hw = &mut (*gate).hw;

    let ret = clk_hw_register(dev, hw);
    if ret != 0 {
        kfree(gate as *mut core::ffi::c_void);
        return ERR_PTR(ret);
    }
    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
