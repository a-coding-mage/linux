// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Hisilicon clock separated gate driver
 *
 * Copyright (c) 2012-2013 Hisilicon Limited.
 * Copyright (c) 2012-2013 Linaro Limited.
 *
 * Author: Haojian Zhuang <haojian.zhuang@linaro.org>
 *         Xin Li <li.xin@linaro.org>
 */

// Dependencies supplied by the surrounding kernel/Rust environment:
// linux/kernel.h, linux/clk-provider.h, linux/io.h, linux/slab.h, and clk.h.

/* clock separated gate register offset */
const CLKGATE_SEPARATED_ENABLE: usize = 0x0;
const CLKGATE_SEPARATED_DISABLE: usize = 0x4;
const CLKGATE_SEPARATED_STATUS: usize = 0x8;

#[repr(C)]
struct clkgate_separated {
    hw: clk_hw,
    enable: *mut core::ffi::c_void, /* enable register */
    bit_idx: u8,                     /* bits in enable/disable register */
    flags: u8,
    lock: *mut spinlock_t,
}

unsafe fn clkgate_separated_enable(hw: *mut clk_hw) -> i32 {
    let sclk: *mut clkgate_separated = container_of!(hw, clkgate_separated, hw);
    let mut flags: c_ulong = 0;
    let reg: u32;

    if !(*sclk).lock.is_null() {
        spin_lock_irqsave((*sclk).lock, &mut flags);
    }
    reg = BIT((*sclk).bit_idx);
    writel_relaxed(reg, (*sclk).enable);
    readl_relaxed((*sclk).enable.add(CLKGATE_SEPARATED_STATUS));
    if !(*sclk).lock.is_null() {
        spin_unlock_irqrestore((*sclk).lock, flags);
    }
    0
}

unsafe fn clkgate_separated_disable(hw: *mut clk_hw) {
    let sclk: *mut clkgate_separated = container_of!(hw, clkgate_separated, hw);
    let mut flags: c_ulong = 0;
    let reg: u32;

    if !(*sclk).lock.is_null() {
        spin_lock_irqsave((*sclk).lock, &mut flags);
    }
    reg = BIT((*sclk).bit_idx);
    writel_relaxed(reg, (*sclk).enable.add(CLKGATE_SEPARATED_DISABLE));
    readl_relaxed((*sclk).enable.add(CLKGATE_SEPARATED_STATUS));
    if !(*sclk).lock.is_null() {
        spin_unlock_irqrestore((*sclk).lock, flags);
    }
}

unsafe fn clkgate_separated_is_enabled(hw: *mut clk_hw) -> i32 {
    let sclk: *mut clkgate_separated = container_of!(hw, clkgate_separated, hw);
    let mut reg: u32;

    reg = readl_relaxed((*sclk).enable.add(CLKGATE_SEPARATED_STATUS));
    reg &= BIT((*sclk).bit_idx);

    if reg != 0 { 1 } else { 0 }
}

static clkgate_separated_ops: clk_ops = clk_ops {
    enable: Some(clkgate_separated_enable),
    disable: Some(clkgate_separated_disable),
    is_enabled: Some(clkgate_separated_is_enabled),
};

unsafe fn hisi_register_clkgate_sep(
    dev: *mut device,
    name: *const c_char,
    parent_name: *const c_char,
    flags: c_ulong,
    reg: *mut core::ffi::c_void,
    bit_idx: u8,
    clk_gate_flags: u8,
    lock: *mut spinlock_t,
) -> *mut clk {
    let sclk: *mut clkgate_separated;
    let clk: *mut clk;
    let mut init: clk_init_data;

    sclk = kzalloc_obj::<clkgate_separated>();
    if sclk.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    init.name = name;
    init.ops = &clkgate_separated_ops;
    init.flags = flags;
    init.parent_names = if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
    init.num_parents = if !parent_name.is_null() { 1 } else { 0 };

    (*sclk).enable = reg.add(CLKGATE_SEPARATED_ENABLE);
    (*sclk).bit_idx = bit_idx;
    (*sclk).flags = clk_gate_flags;
    (*sclk).hw.init = &init;
    (*sclk).lock = lock;

    clk = clk_register(dev, &mut (*sclk).hw);
    if IS_ERR(clk) {
        kfree(sclk);
    }
    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
