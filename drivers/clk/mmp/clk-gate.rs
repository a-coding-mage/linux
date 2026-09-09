// SPDX-License-Identifier: GPL-2.0-only
/*
 * mmp gate clock operation source file
 *
 * Copyright (C) 2014 Marvell
 * Chao Xie <chao.xie@marvell.com>
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * Some clocks will have multiple bits to enable the clocks, and
 * the bits to disable the clock is not same as enabling bits.
 */

const MMP_CLK_GATE_NEED_DELAY: u32 = /* supplied by clk.h */ 0;

unsafe fn to_clk_mmp_gate(hw: *mut clk_hw) -> *mut mmp_clk_gate {
    // container_of(hw, struct mmp_clk_gate, hw)
    (hw as *mut u8).sub(core::mem::offset_of!(mmp_clk_gate, hw)) as *mut mmp_clk_gate
}

unsafe fn mmp_clk_gate_enable(hw: *mut clk_hw) -> i32 {
    let gate = to_clk_mmp_gate(hw);
    let mut flags: c_ulong = 0;
    let rate: c_ulong;
    let mut tmp: u32;

    if !(*gate).lock.is_null() {
        spin_lock_irqsave((*gate).lock, &mut flags);
    }

    tmp = readl((*gate).reg);
    tmp &= !(*gate).mask;
    tmp |= (*gate).val_enable;
    writel(tmp, (*gate).reg);

    if !(*gate).lock.is_null() {
        spin_unlock_irqrestore((*gate).lock, flags);
    }

    if (*gate).flags & MMP_CLK_GATE_NEED_DELAY != 0 {
        rate = clk_hw_get_rate(hw);
        /* Need delay 2 cycles. */
        udelay(2_000_000 / rate);
    }

    0
}

unsafe fn mmp_clk_gate_disable(hw: *mut clk_hw) {
    let gate = to_clk_mmp_gate(hw);
    let mut flags: c_ulong = 0;
    let mut tmp: u32;

    if !(*gate).lock.is_null() {
        spin_lock_irqsave((*gate).lock, &mut flags);
    }

    tmp = readl((*gate).reg);
    tmp &= !(*gate).mask;
    tmp |= (*gate).val_disable;
    writel(tmp, (*gate).reg);

    if !(*gate).lock.is_null() {
        spin_unlock_irqrestore((*gate).lock, flags);
    }
}

unsafe fn mmp_clk_gate_is_enabled(hw: *mut clk_hw) -> i32 {
    let gate = to_clk_mmp_gate(hw);
    let mut flags: c_ulong = 0;
    let tmp: u32;

    if !(*gate).lock.is_null() {
        spin_lock_irqsave((*gate).lock, &mut flags);
    }

    tmp = readl((*gate).reg);

    if !(*gate).lock.is_null() {
        spin_unlock_irqrestore((*gate).lock, flags);
    }

    if (tmp & (*gate).mask) == (*gate).val_enable { 1 } else { 0 }
}

pub static mmp_clk_gate_ops: clk_ops = clk_ops {
    enable: Some(mmp_clk_gate_enable),
    disable: Some(mmp_clk_gate_disable),
    is_enabled: Some(mmp_clk_gate_is_enabled),
};

pub unsafe fn mmp_clk_register_gate(
    dev: *mut device,
    name: *const c_char,
    parent_name: *const c_char,
    flags: c_ulong,
    reg: *mut core::ffi::c_void,
    mask: u32,
    val_enable: u32,
    val_disable: u32,
    gate_flags: u32,
    lock: *mut spinlock_t,
) -> *mut clk {
    let gate = kzalloc::<mmp_clk_gate>();
    if gate.is_null() {
        return err_ptr(-12);
    }

    let mut init: clk_init_data = core::mem::zeroed();
    init.name = name;
    init.ops = &mmp_clk_gate_ops;
    init.flags = flags;
    init.parent_names = if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
    init.num_parents = if !parent_name.is_null() { 1 } else { 0 };

    /* struct clk_gate assignments */
    (*gate).reg = reg;
    (*gate).mask = mask;
    (*gate).val_enable = val_enable;
    (*gate).val_disable = val_disable;
    (*gate).flags = gate_flags;
    (*gate).lock = lock;
    (*gate).hw.init = &init;

    let clk = clk_register(dev, &mut (*gate).hw);

    if is_err(clk) {
        kfree(gate);
    }

    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
