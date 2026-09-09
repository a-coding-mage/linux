// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2021 NXP
 *
 * Peng Fan <peng.fan@nxp.com>
 */

// Dependencies supplied by the surrounding clock-provider implementation:
// linux/clk-provider.h, linux/errno.h, linux/export.h, linux/io.h,
// linux/iopoll.h, linux/slab.h, and "clk.h".

const TIMEOUT_US: u32 = 500;

const CCM_DIV_SHIFT: u32 = 0;
const CCM_DIV_WIDTH: u32 = 8;
const CCM_MUX_SHIFT: u32 = 8;
const CCM_MUX_MASK: u32 = 3;
const CCM_OFF_SHIFT: u32 = 24;
const CCM_BUSY_SHIFT: u32 = 28;

const STAT_OFFSET: usize = 0x4;
const AUTHEN_OFFSET: usize = 0x30;
const TZ_NS_SHIFT: u32 = 9;
const TZ_NS_MASK: u32 = 1 << 9;

const WHITE_LIST_SHIFT: u32 = 16;

unsafe fn imx93_clk_composite_wait_ready(hw: *mut clk_hw, reg: *mut core::ffi::c_void) -> i32 {
    let mut val: u32 = 0;
    let ret = readl_poll_timeout_atomic(
        reg.add(STAT_OFFSET),
        &mut val,
        (val & (1u32 << CCM_BUSY_SHIFT)) == 0,
        0,
        TIMEOUT_US,
    );
    if ret != 0 {
        pr_err("Slice[%s] busy timeout\\n", clk_hw_get_name(hw));
    }
    ret
}

unsafe fn imx93_clk_composite_gate_endisable(hw: *mut clk_hw, enable: i32) {
    let gate = to_clk_gate(hw);
    let mut flags: usize = 0;
    let mut reg: u32;

    if !(*gate).lock.is_null() {
        spin_lock_irqsave((*gate).lock, &mut flags);
    }

    reg = readl((*gate).reg);

    if enable != 0 {
        reg &= !(1u32 << (*gate).bit_idx);
    } else {
        reg |= 1u32 << (*gate).bit_idx;
    }

    writel(reg, (*gate).reg);
    imx93_clk_composite_wait_ready(hw, (*gate).reg);

    if !(*gate).lock.is_null() {
        spin_unlock_irqrestore((*gate).lock, flags);
    }
}

unsafe fn imx93_clk_composite_gate_enable(hw: *mut clk_hw) -> i32 {
    imx93_clk_composite_gate_endisable(hw, 1);
    0
}

unsafe fn imx93_clk_composite_gate_disable(hw: *mut clk_hw) {
    /*
     * Skip disable the root clock gate if mcore enabled.
     * The root clock may be used by the mcore.
     */
    if mcore_booted {
        return;
    }
    imx93_clk_composite_gate_endisable(hw, 0);
}

static imx93_clk_composite_gate_ops: clk_ops = clk_ops {
    enable: Some(imx93_clk_composite_gate_enable),
    disable: Some(imx93_clk_composite_gate_disable),
    is_enabled: Some(clk_gate_is_enabled),
};

unsafe fn imx93_clk_composite_divider_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: c_ulong,
) -> c_ulong {
    clk_divider_ops.recalc_rate.unwrap()(hw, parent_rate)
}

unsafe fn imx93_clk_composite_divider_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    clk_divider_ops.determine_rate.unwrap()(hw, req)
}

unsafe fn imx93_clk_composite_divider_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> i32 {
    let divider = to_clk_divider(hw);
    let value = divider_get_val(
        rate,
        parent_rate,
        (*divider).table,
        (*divider).width,
        (*divider).flags,
    );
    if value < 0 {
        return value;
    }

    let mut flags: usize = 0;
    if !(*divider).lock.is_null() {
        spin_lock_irqsave((*divider).lock, &mut flags);
    }

    let mut val = readl((*divider).reg);
    val &= !(clk_div_mask((*divider).width) << (*divider).shift);
    val |= (value as u32) << (*divider).shift;
    writel(val, (*divider).reg);
    let ret = imx93_clk_composite_wait_ready(hw, (*divider).reg);

    if !(*divider).lock.is_null() {
        spin_unlock_irqrestore((*divider).lock, flags);
    }
    ret
}

static imx93_clk_composite_divider_ops: clk_ops = clk_ops {
    recalc_rate: Some(imx93_clk_composite_divider_recalc_rate),
    determine_rate: Some(imx93_clk_composite_divider_determine_rate),
    set_rate: Some(imx93_clk_composite_divider_set_rate),
};

unsafe fn imx93_clk_composite_mux_get_parent(hw: *mut clk_hw) -> u8 {
    clk_mux_ops.get_parent.unwrap()(hw)
}

unsafe fn imx93_clk_composite_mux_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let mux = to_clk_mux(hw);
    let mut val = clk_mux_index_to_val((*mux).table, (*mux).flags, index);
    let mut flags: usize = 0;
    if !(*mux).lock.is_null() {
        spin_lock_irqsave((*mux).lock, &mut flags);
    }

    let mut reg = readl((*mux).reg);
    reg &= !((*mux).mask << (*mux).shift);
    val <<= (*mux).shift;
    reg |= val;
    writel(reg, (*mux).reg);
    let ret = imx93_clk_composite_wait_ready(hw, (*mux).reg);

    if !(*mux).lock.is_null() {
        spin_unlock_irqrestore((*mux).lock, flags);
    }
    ret
}

unsafe fn imx93_clk_composite_mux_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    clk_mux_ops.determine_rate.unwrap()(hw, req)
}

static imx93_clk_composite_mux_ops: clk_ops = clk_ops {
    get_parent: Some(imx93_clk_composite_mux_get_parent),
    set_parent: Some(imx93_clk_composite_mux_set_parent),
    determine_rate: Some(imx93_clk_composite_mux_determine_rate),
};

unsafe fn imx93_clk_composite_flags(
    name: *const c_char,
    parent_names: *const *const c_char,
    num_parents: i32,
    reg: *mut core::ffi::c_void,
    domain_id: u32,
    flags: c_ulong,
) -> *mut clk_hw {
    let mut hw = err_ptr(-12);
    let mut mux_hw: *mut clk_hw;
    let mut div_hw: *mut clk_hw;
    let mut gate_hw: *mut clk_hw;
    let mut div: *mut clk_divider = core::ptr::null_mut();
    let mut gate: *mut clk_gate = core::ptr::null_mut();
    let mut mux: *mut clk_mux = core::ptr::null_mut();
    let mut clk_ro = false;

    mux = kzalloc_obj::<clk_mux>();
    if mux.is_null() { goto_fail!(); }
    mux_hw = &mut (*mux).hw;
    (*mux).reg = reg;
    (*mux).shift = CCM_MUX_SHIFT;
    (*mux).mask = CCM_MUX_MASK;
    (*mux).lock = &mut imx_ccm_lock;

    div = kzalloc_obj::<clk_divider>();
    if div.is_null() { goto_fail!(); }
    div_hw = &mut (*div).hw;
    (*div).reg = reg;
    (*div).shift = CCM_DIV_SHIFT;
    (*div).width = CCM_DIV_WIDTH;
    (*div).lock = &mut imx_ccm_lock;
    (*div).flags = CLK_DIVIDER_ROUND_CLOSEST;

    let authen = readl(reg.add(AUTHEN_OFFSET));
    if (authen & TZ_NS_MASK) == 0 || (authen & (1u32 << (WHITE_LIST_SHIFT + domain_id))) == 0 {
        clk_ro = true;
    }

    if clk_ro {
        hw = clk_hw_register_composite(core::ptr::null_mut(), name, parent_names, num_parents,
            mux_hw, &clk_mux_ro_ops, div_hw, &clk_divider_ro_ops,
            core::ptr::null_mut(), core::ptr::null(), flags);
    } else {
        gate = kzalloc_obj::<clk_gate>();
        if gate.is_null() { goto_fail!(); }
        gate_hw = &mut (*gate).hw;
        (*gate).reg = reg;
        (*gate).bit_idx = CCM_OFF_SHIFT;
        (*gate).lock = &mut imx_ccm_lock;
        (*gate).flags = CLK_GATE_SET_TO_DISABLE;
        hw = clk_hw_register_composite(core::ptr::null_mut(), name, parent_names, num_parents,
            mux_hw, &imx93_clk_composite_mux_ops, div_hw, &imx93_clk_composite_divider_ops,
            gate_hw, &imx93_clk_composite_gate_ops, flags | CLK_SET_RATE_NO_REPARENT);
    }

    if is_err(hw) { goto_fail!(); }
    return hw;

goto_fail:
    kfree(gate as *mut core::ffi::c_void);
    kfree(div as *mut core::ffi::c_void);
    kfree(mux as *mut core::ffi::c_void);
    err_cast(hw)
}

// EXPORT_SYMBOL_GPL(imx93_clk_composite_flags);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
