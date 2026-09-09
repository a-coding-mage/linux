// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2016 Freescale Semiconductor, Inc.
 * Copyright 2017~2018 NXP
 */

// Dependencies are supplied by the surrounding kernel translation.

const PCG_PR_MASK: u32 = 1u32 << 31;
const PCG_PCS_SHIFT: u32 = 24;
const PCG_PCS_MASK: u32 = 0x7;
const PCG_CGC_SHIFT: u32 = 30;
const PCG_FRAC_SHIFT: u32 = 3;
const PCG_FRAC_WIDTH: u32 = 1;
const PCG_PCD_SHIFT: u32 = 0;
const PCG_PCD_WIDTH: u32 = 3;

const SW_RST: u32 = 1u32 << 28;

unsafe fn pcc_gate_enable(hw: *mut clk_hw) -> i32 {
    let gate = to_clk_gate(hw);
    let mut flags: ulong = 0;
    let mut val: u32;
    let ret: i32;

    ret = (clk_gate_ops.enable.unwrap())(hw);
    if ret != 0 {
        return ret;
    }

    /* Make sure the IP's clock is ready before release reset */
    udelay(1);

    spin_lock_irqsave((*gate).lock, &mut flags);
    /*
     * release the sw reset for peripherals associated with
     * with this pcc clock.
     */
    val = readl((*gate).reg);
    val |= SW_RST;
    writel(val, (*gate).reg);

    spin_unlock_irqrestore((*gate).lock, flags);

    /*
     * Read back the register to make sure the previous write has been
     * done in the target HW register. For IP like GPU, after deassert
     * the reset, need to wait for a while to make sure the sync reset
     * is done
     */
    readl((*gate).reg);
    udelay(1);

    0
}

unsafe fn pcc_gate_disable(hw: *mut clk_hw) {
    (clk_gate_ops.disable.unwrap())(hw);
}

unsafe fn pcc_gate_is_enabled(hw: *mut clk_hw) -> i32 {
    (clk_gate_ops.is_enabled.unwrap())(hw)
}

static pcc_gate_ops: clk_ops = clk_ops {
    enable: Some(pcc_gate_enable),
    disable: Some(pcc_gate_disable),
    is_enabled: Some(pcc_gate_is_enabled),
};

unsafe fn imx_ulp_clk_hw_composite(
    name: *const c_char,
    parent_names: *const *const c_char,
    num_parents: i32,
    mux_present: bool,
    rate_present: bool,
    gate_present: bool,
    reg: *mut c_void,
    has_swrst: bool,
) -> *mut clk_hw {
    let mut mux_hw: *mut clk_hw = core::ptr::null_mut();
    let mut fd_hw: *mut clk_hw = core::ptr::null_mut();
    let mut gate_hw: *mut clk_hw = core::ptr::null_mut();
    let mut fd: *mut clk_fractional_divider = core::ptr::null_mut();
    let mut gate: *mut clk_gate = core::ptr::null_mut();
    let mut mux: *mut clk_mux = core::ptr::null_mut();
    let hw: *mut clk_hw;
    let mut val: u32;

    val = readl(reg);
    if (val & PCG_PR_MASK) == 0 {
        pr_info("PCC PR is 0 for clk:%s, bypass\\0", name);
        return core::ptr::null_mut();
    }

    if mux_present {
        mux = kzalloc_obj();
        if mux.is_null() {
            return ERR_PTR(-12);
        }
        mux_hw = &mut (*mux).hw;
        (*mux).reg = reg;
        (*mux).shift = PCG_PCS_SHIFT;
        (*mux).mask = PCG_PCS_MASK;
        if has_swrst {
            (*mux).lock = &mut imx_ccm_lock;
        }
    }

    if rate_present {
        fd = kzalloc_obj();
        if fd.is_null() {
            kfree(mux as *mut c_void);
            return ERR_PTR(-12);
        }
        fd_hw = &mut (*fd).hw;
        (*fd).reg = reg;
        (*fd).mshift = PCG_FRAC_SHIFT;
        (*fd).mwidth = PCG_FRAC_WIDTH;
        (*fd).nshift = PCG_PCD_SHIFT;
        (*fd).nwidth = PCG_PCD_WIDTH;
        (*fd).flags = CLK_FRAC_DIVIDER_ZERO_BASED;
        if has_swrst {
            (*fd).lock = &mut imx_ccm_lock;
        }
    }

    if gate_present {
        gate = kzalloc_obj();
        if gate.is_null() {
            kfree(mux as *mut c_void);
            kfree(fd as *mut c_void);
            return ERR_PTR(-12);
        }
        gate_hw = &mut (*gate).hw;
        (*gate).reg = reg;
        (*gate).bit_idx = PCG_CGC_SHIFT;
        if has_swrst {
            (*gate).lock = &mut imx_ccm_lock;
        }
        /* Make sure clock is gated during clock tree initialization. */
        val = readl_relaxed(reg);
        val &= !(1u32 << PCG_CGC_SHIFT);
        writel_relaxed(val, reg);
    }

    hw = clk_hw_register_composite(
        core::ptr::null_mut(), name, parent_names, num_parents,
        mux_hw, &clk_mux_ops, fd_hw, &clk_fractional_divider_ops,
        gate_hw, if has_swrst { &pcc_gate_ops } else { &clk_gate_ops },
        CLK_SET_RATE_GATE | CLK_SET_PARENT_GATE | CLK_SET_RATE_NO_REPARENT,
    );
    if IS_ERR(hw) {
        kfree(mux as *mut c_void);
        kfree(fd as *mut c_void);
        kfree(gate as *mut c_void);
    }

    hw
}

pub unsafe fn imx7ulp_clk_hw_composite(
    name: *const c_char, parent_names: *const *const c_char, num_parents: i32,
    mux_present: bool, rate_present: bool, gate_present: bool, reg: *mut c_void,
) -> *mut clk_hw {
    imx_ulp_clk_hw_composite(name, parent_names, num_parents, mux_present,
        rate_present, gate_present, reg, false)
}

pub unsafe fn imx8ulp_clk_hw_composite(
    name: *const c_char, parent_names: *const *const c_char, num_parents: i32,
    mux_present: bool, rate_present: bool, gate_present: bool, reg: *mut c_void,
    has_swrst: bool,
) -> *mut clk_hw {
    imx_ulp_clk_hw_composite(name, parent_names, num_parents, mux_present,
        rate_present, gate_present, reg, has_swrst)
}

// EXPORT_SYMBOL_GPL(imx8ulp_clk_hw_composite);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
