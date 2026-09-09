// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2010-2011 Canonical Ltd <jeremy.kerr@canonical.com>
 * Copyright (C) 2011-2012 Mike Turquette, Linaro Ltd <mturquette@linaro.org>
 *
 * Gated clock implementation
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn clk_gate_readl(gate: *mut clk_gate) -> u32 {
    unsafe {
        if (*gate).flags & CLK_GATE_BIG_ENDIAN != 0 {
            return ioread32be((*gate).reg);
        }
        readl((*gate).reg)
    }
}

unsafe fn clk_gate_writel(gate: *mut clk_gate, val: u32) {
    unsafe {
        if (*gate).flags & CLK_GATE_BIG_ENDIAN != 0 {
            iowrite32be(val, (*gate).reg);
        } else {
            writel(val, (*gate).reg);
        }
    }
}

/*
 * It works on following logic:
 *
 * For enabling clock, enable = 1
 * set2dis = 1 -> clear bit -> set = 0
 * set2dis = 0 -> set bit -> set = 1
 *
 * For disabling clock, enable = 0
 * set2dis = 1 -> set bit -> set = 1
 * set2dis = 0 -> clear bit -> set = 0
 *
 * So, result is always: enable xor set2dis.
 */
unsafe fn clk_gate_endisable(hw: *mut clk_hw, enable: i32) {
    let gate = to_clk_gate(hw);
    let mut set: i32;
    let mut flags: c_ulong = 0;
    let reg: u32;

    unsafe {
        set = if (*gate).flags & CLK_GATE_SET_TO_DISABLE != 0 { 1 } else { 0 };
        set ^= enable;

        if !(*gate).lock.is_null() {
            spin_lock_irqsave((*gate).lock, &mut flags);
        } else {
            __acquire((*gate).lock);
        }

        if (*gate).flags & CLK_GATE_HIWORD_MASK != 0 {
            reg = BIT((*gate).bit_idx as u32 + 16);
            let mut value = reg;
            if set != 0 {
                value |= BIT((*gate).bit_idx as u32);
            }
            clk_gate_writel(gate, value);
        } else {
            let mut value = clk_gate_readl(gate);
            if set != 0 {
                value |= BIT((*gate).bit_idx as u32);
            } else {
                value &= !BIT((*gate).bit_idx as u32);
            }
            clk_gate_writel(gate, value);
        }

        if !(*gate).lock.is_null() {
            spin_unlock_irqrestore((*gate).lock, flags);
        } else {
            __release((*gate).lock);
        }
    }
}

unsafe fn clk_gate_enable(hw: *mut clk_hw) -> i32 {
    clk_gate_endisable(hw, 1);
    0
}

unsafe fn clk_gate_disable(hw: *mut clk_hw) {
    clk_gate_endisable(hw, 0);
}

pub unsafe fn clk_gate_is_enabled(hw: *mut clk_hw) -> i32 {
    let gate = to_clk_gate(hw);
    let mut reg;
    unsafe {
        reg = clk_gate_readl(gate);
        /* if a set bit disables this clk, flip it before masking */
        if (*gate).flags & CLK_GATE_SET_TO_DISABLE != 0 {
            reg ^= BIT((*gate).bit_idx as u32);
        }
        reg &= BIT((*gate).bit_idx as u32);
    }
    if reg != 0 { 1 } else { 0 }
}

pub static clk_gate_ops: clk_ops = clk_ops {
    enable: Some(clk_gate_enable),
    disable: Some(clk_gate_disable),
    is_enabled: Some(clk_gate_is_enabled),
};

pub unsafe fn __clk_hw_register_gate(
    dev: *mut device,
    np: *mut device_node,
    name: *const c_char,
    parent_name: *const c_char,
    parent_hw: *const clk_hw,
    parent_data: *const clk_parent_data,
    flags: c_ulong,
    reg: *mut c_void,
    bit_idx: u8,
    clk_gate_flags: u8,
    lock: *mut spinlock_t,
) -> *mut clk_hw {
    let mut gate = kzalloc::<clk_gate>(GFP_KERNEL);
    let mut hw: *mut clk_hw;
    let mut init: clk_init_data = core::mem::zeroed();
    let mut ret: i32 = -EINVAL;

    if clk_gate_flags & CLK_GATE_HIWORD_MASK != 0 && bit_idx > 15 {
        pr_err!("gate bit exceeds LOWORD field\n");
        return ERR_PTR(-EINVAL);
    }
    if gate.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    init.name = name;
    init.ops = &clk_gate_ops;
    init.flags = flags;
    init.parent_names = if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
    init.parent_hws = if !parent_hw.is_null() { &parent_hw } else { core::ptr::null() };
    init.parent_data = parent_data;
    init.num_parents = if !parent_name.is_null() || !parent_hw.is_null() || !parent_data.is_null() { 1 } else { 0 };

    (*gate).reg = reg;
    (*gate).bit_idx = bit_idx;
    (*gate).flags = clk_gate_flags;
    (*gate).lock = lock;
    (*gate).hw.init = &init;

    hw = &mut (*gate).hw;
    if !dev.is_null() || np.is_null() {
        ret = clk_hw_register(dev, hw);
    } else if !np.is_null() {
        ret = of_clk_hw_register(np, hw);
    }
    if ret != 0 {
        kfree(gate as *mut c_void);
        hw = ERR_PTR(ret);
    }
    hw
}

pub unsafe fn clk_register_gate(dev: *mut device, name: *const c_char, parent_name: *const c_char, flags: c_ulong, reg: *mut c_void, bit_idx: u8, clk_gate_flags: u8, lock: *mut spinlock_t) -> *mut clk {
    let hw = clk_hw_register_gate(dev, name, parent_name, flags, reg, bit_idx, clk_gate_flags, lock);
    if IS_ERR(hw) { return ERR_CAST(hw); }
    (*hw).clk
}

pub unsafe fn clk_unregister_gate(clk: *mut clk) {
    let hw = __clk_get_hw(clk);
    if hw.is_null() { return; }
    let gate = to_clk_gate(hw);
    clk_unregister(clk);
    kfree(gate as *mut c_void);
}

pub unsafe fn clk_hw_unregister_gate(hw: *mut clk_hw) {
    let gate = to_clk_gate(hw);
    clk_hw_unregister(hw);
    kfree(gate as *mut c_void);
}

unsafe fn devm_clk_hw_release_gate(_dev: *mut device, res: *mut c_void) {
    clk_hw_unregister_gate(*(res as *mut *mut clk_hw));
}

pub unsafe fn __devm_clk_hw_register_gate(dev: *mut device, np: *mut device_node, name: *const c_char, parent_name: *const c_char, parent_hw: *const clk_hw, parent_data: *const clk_parent_data, flags: c_ulong, reg: *mut c_void, bit_idx: u8, clk_gate_flags: u8, lock: *mut spinlock_t) -> *mut clk_hw {
    let ptr = devres_alloc(Some(devm_clk_hw_release_gate), core::mem::size_of::<*mut clk_hw>(), GFP_KERNEL);
    if ptr.is_null() { return ERR_PTR(-ENOMEM); }
    let hw = __clk_hw_register_gate(dev, np, name, parent_name, parent_hw, parent_data, flags, reg, bit_idx, clk_gate_flags, lock);
    if !IS_ERR(hw) {
        *(ptr as *mut *mut clk_hw) = hw;
        devres_add(dev, ptr);
    } else {
        devres_free(ptr);
    }
    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
