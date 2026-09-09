// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/clk-provider.h, linux/slab.h, linux/io.h, linux/delay.h,
// linux/err.h, soc/tegra/fuse.h, and "clk.h".

static mut PERIPH_REF_LOCK: Spinlock = DEFINE_SPINLOCK();

/* Macros to assist peripheral gate clock */
#[inline]
unsafe fn read_enb(gate: *mut tegra_clk_periph_gate) -> u32 {
    readl_relaxed((*gate).clk_base.add((*gate).regs.enb_reg as usize))
}

#[inline]
unsafe fn write_enb_set(val: u32, gate: *mut tegra_clk_periph_gate) {
    writel_relaxed(val, (*gate).clk_base.add((*gate).regs.enb_set_reg as usize));
}

#[inline]
unsafe fn write_enb_clr(val: u32, gate: *mut tegra_clk_periph_gate) {
    writel_relaxed(val, (*gate).clk_base.add((*gate).regs.enb_clr_reg as usize));
}

#[inline]
unsafe fn read_rst(gate: *mut tegra_clk_periph_gate) -> u32 {
    readl_relaxed((*gate).clk_base.add((*gate).regs.rst_reg as usize))
}

#[inline]
unsafe fn write_rst_clr(val: u32, gate: *mut tegra_clk_periph_gate) {
    writel_relaxed(val, (*gate).clk_base.add((*gate).regs.rst_clr_reg as usize));
}

#[inline]
unsafe fn periph_clk_to_bit(gate: *mut tegra_clk_periph_gate) -> u32 {
    1u32 << ((*gate).clk_num % 32)
}

const LVL2_CLK_GATE_OVRE: usize = 0x554;

/* Peripheral gate clock ops */
unsafe fn clk_periph_is_enabled(hw: *mut clk_hw) -> i32 {
    let gate = to_clk_periph_gate(hw);
    let mut state = 1;

    if (read_enb(gate) & periph_clk_to_bit(gate)) == 0 {
        state = 0;
    }

    if ((*gate).flags & TEGRA_PERIPH_NO_RESET) == 0
        && (read_rst(gate) & periph_clk_to_bit(gate)) != 0
    {
        state = 0;
    }

    state
}

unsafe fn clk_periph_enable_locked(hw: *mut clk_hw) {
    let gate = to_clk_periph_gate(hw);

    write_enb_set(periph_clk_to_bit(gate), gate);
    udelay(2);

    if ((*gate).flags & TEGRA_PERIPH_WAR_1005168) != 0 {
        writel_relaxed(0, (*gate).clk_base.add(LVL2_CLK_GATE_OVRE));
        writel_relaxed(BIT(22), (*gate).clk_base.add(LVL2_CLK_GATE_OVRE));
        udelay(1);
        writel_relaxed(0, (*gate).clk_base.add(LVL2_CLK_GATE_OVRE));
    }
}

unsafe fn clk_periph_disable_locked(hw: *mut clk_hw) {
    let gate = to_clk_periph_gate(hw);

    /*
     * If peripheral is in the APB bus then read the APB bus to
     * flush the write operation in apb bus. This will avoid the
     * peripheral access after disabling clock
     */
    if ((*gate).flags & TEGRA_PERIPH_ON_APB) != 0 {
        tegra_read_chipid();
    }

    write_enb_clr(periph_clk_to_bit(gate), gate);
}

unsafe fn clk_periph_enable(hw: *mut clk_hw) -> i32 {
    let gate = to_clk_periph_gate(hw);
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&raw mut PERIPH_REF_LOCK, &mut flags);

    let count = &mut *(*gate).enable_refcnt.add((*gate).clk_num as usize);
    if *count == 0 {
        clk_periph_enable_locked(hw);
    }
    *count += 1;

    spin_unlock_irqrestore(&raw mut PERIPH_REF_LOCK, flags);

    0
}

unsafe fn clk_periph_disable(hw: *mut clk_hw) {
    let gate = to_clk_periph_gate(hw);
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&raw mut PERIPH_REF_LOCK, &mut flags);

    let count = &mut *(*gate).enable_refcnt.add((*gate).clk_num as usize);
    WARN_ON(*count == 0);
    *count -= 1;
    if *count == 0 {
        clk_periph_disable_locked(hw);
    }

    spin_unlock_irqrestore(&raw mut PERIPH_REF_LOCK, flags);
}

unsafe fn clk_periph_disable_unused(hw: *mut clk_hw) {
    let gate = to_clk_periph_gate(hw);
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&raw mut PERIPH_REF_LOCK, &mut flags);

    /*
     * Some clocks are duplicated and some of them are marked as critical,
     * like fuse and fuse_burn for example, thus the enable_refcnt will
     * be non-zero here if the "unused" duplicate is disabled by CCF.
     */
    if *(*gate).enable_refcnt.add((*gate).clk_num as usize) == 0 {
        clk_periph_disable_locked(hw);
    }

    spin_unlock_irqrestore(&raw mut PERIPH_REF_LOCK, flags);
}

const TEGRA_CLK_PERIPH_GATE_OPS: clk_ops = clk_ops {
    is_enabled: Some(clk_periph_is_enabled),
    enable: Some(clk_periph_enable),
    disable: Some(clk_periph_disable),
    disable_unused: Some(clk_periph_disable_unused),
};

unsafe fn tegra_clk_register_periph_gate(
    name: *const c_char,
    parent_name: *const c_char,
    gate_flags: u8,
    clk_base: *mut c_void,
    flags: c_ulong,
    clk_num: i32,
    enable_refcnt: *mut i32,
) -> *mut clk {
    let mut gate: *mut tegra_clk_periph_gate;
    let mut clk: *mut clk;
    let mut init: clk_init_data;
    let pregs: *const tegra_clk_periph_regs;

    pregs = get_reg_bank(clk_num);
    if pregs.is_null() {
        return ERR_PTR(-EINVAL);
    }

    gate = kzalloc_obj::<tegra_clk_periph_gate>();
    if gate.is_null() {
        pr_err!("%s: could not allocate periph gate clk\\n", __func__);
        return ERR_PTR(-ENOMEM);
    }

    init.name = name;
    init.flags = flags;
    init.parent_names = if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
    init.num_parents = if !parent_name.is_null() { 1 } else { 0 };
    init.ops = &TEGRA_CLK_PERIPH_GATE_OPS;

    (*gate).magic = TEGRA_CLK_PERIPH_GATE_MAGIC;
    (*gate).clk_base = clk_base;
    (*gate).clk_num = clk_num;
    (*gate).flags = gate_flags;
    (*gate).enable_refcnt = enable_refcnt;
    (*gate).regs = pregs;

    /* Data in .init is copied by clk_register(), so stack variable OK */
    (*gate).hw.init = &init;

    clk = clk_register(core::ptr::null_mut(), &mut (*gate).hw);
    if IS_ERR(clk) {
        kfree(gate);
    }

    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
