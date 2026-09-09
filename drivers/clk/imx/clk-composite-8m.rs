// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2018 NXP
 */

// C dependencies supplied by the surrounding kernel translation.

const PCG_PREDIV_SHIFT: u32 = 16;
const PCG_PREDIV_WIDTH: u32 = 3;
const PCG_PREDIV_MAX: i32 = 8;

const PCG_DIV_SHIFT: u32 = 0;
const PCG_CORE_DIV_WIDTH: u32 = 3;
const PCG_DIV_WIDTH: u32 = 6;
const PCG_DIV_MAX: i32 = 64;

const PCG_PCS_SHIFT: u32 = 24;
const PCG_PCS_MASK: u32 = 0x7;

const PCG_CGC_SHIFT: u32 = 28;

unsafe fn imx8m_clk_composite_divider_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: c_ulong,
) -> c_ulong {
    let divider = to_clk_divider(hw);
    let mut prediv_rate: c_ulong;
    let prediv_value: u32;
    let div_value: u32;

    prediv_value = readl((*divider).reg) >> (*divider).shift;
    prediv_value &= clk_div_mask((*divider).width);

    prediv_rate = divider_recalc_rate(
        hw,
        parent_rate,
        prediv_value,
        core::ptr::null(),
        (*divider).flags,
        (*divider).width,
    );

    div_value = readl((*divider).reg) >> PCG_DIV_SHIFT;
    div_value &= clk_div_mask(PCG_DIV_WIDTH);

    divider_recalc_rate(
        hw,
        prediv_rate,
        div_value,
        core::ptr::null(),
        (*divider).flags,
        PCG_DIV_WIDTH,
    )
}

unsafe fn imx8m_clk_composite_compute_dividers(
    rate: c_ulong,
    parent_rate: c_ulong,
    prediv: *mut c_int,
    postdiv: *mut c_int,
) -> c_int {
    let mut error: c_int = c_int::MAX;
    let mut ret: c_int = -EINVAL;

    *prediv = 1;
    *postdiv = 1;

    let mut div1 = 1;
    while div1 <= PCG_PREDIV_MAX {
        let mut div2 = 1;
        while div2 <= PCG_DIV_MAX {
            let new_error = ((parent_rate / div1 as c_ulong) / div2 as c_ulong) as c_int
                - rate as c_int;

            if new_error.abs() < error.abs() {
                *prediv = div1;
                *postdiv = div2;
                error = new_error;
                ret = 0;
            }
            div2 += 1;
        }
        div1 += 1;
    }
    ret
}

unsafe fn imx8m_clk_composite_divider_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> c_int {
    let divider = to_clk_divider(hw);
    let mut flags: c_ulong = 0;
    let mut prediv_value: c_int = 0;
    let mut div_value: c_int = 0;
    let ret = imx8m_clk_composite_compute_dividers(
        rate,
        parent_rate,
        &mut prediv_value,
        &mut div_value,
    );
    let orig: u32;
    let mut val: u32;

    if ret != 0 {
        return -EINVAL;
    }

    spin_lock_irqsave((*divider).lock, &mut flags);

    orig = readl((*divider).reg);
    val = orig
        & !((clk_div_mask((*divider).width) << (*divider).shift)
            | (clk_div_mask(PCG_DIV_WIDTH) << PCG_DIV_SHIFT));

    val |= (prediv_value - 1) as u32 << (*divider).shift;
    val |= (div_value - 1) as u32 << PCG_DIV_SHIFT;

    if val != orig {
        writel(val, (*divider).reg);
    }

    spin_unlock_irqrestore((*divider).lock, flags);

    ret
}

unsafe fn imx8m_divider_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let divider = to_clk_divider(hw);
    let mut prediv_value: c_int;
    let mut div_value: c_int;

    /* if read only, just return current value */
    if (*divider).flags & CLK_DIVIDER_READ_ONLY != 0 {
        let val = readl((*divider).reg);
        prediv_value = (val >> (*divider).shift) as c_int;
        prediv_value = (prediv_value as u32 & clk_div_mask((*divider).width)) as c_int;
        prediv_value += 1;

        div_value = (val >> PCG_DIV_SHIFT) as c_int;
        div_value = (div_value as u32 & clk_div_mask(PCG_DIV_WIDTH)) as c_int;
        div_value += 1;

        return divider_ro_determine_rate(
            hw,
            req,
            (*divider).table,
            PCG_PREDIV_WIDTH + PCG_DIV_WIDTH,
            (*divider).flags,
            prediv_value * div_value,
        );
    }

    divider_determine_rate(
        hw,
        req,
        (*divider).table,
        PCG_PREDIV_WIDTH + PCG_DIV_WIDTH,
        (*divider).flags,
    )
}

static imx8m_clk_composite_divider_ops: clk_ops = clk_ops {
    recalc_rate: Some(imx8m_clk_composite_divider_recalc_rate),
    set_rate: Some(imx8m_clk_composite_divider_set_rate),
    determine_rate: Some(imx8m_divider_determine_rate),
};

unsafe fn imx8m_clk_composite_mux_get_parent(hw: *mut clk_hw) -> u8 {
    clk_mux_ops.get_parent.unwrap()(hw)
}

unsafe fn imx8m_clk_composite_mux_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    let mux = to_clk_mux(hw);
    let mut val = clk_mux_index_to_val((*mux).table, (*mux).flags, index);
    let mut flags: c_ulong = 0;

    if !(*mux).lock.is_null() {
        spin_lock_irqsave((*mux).lock, &mut flags);
    }

    let mut reg = readl((*mux).reg);
    reg &= !((*mux).mask << (*mux).shift);
    val <<= (*mux).shift;
    reg |= val;
    /*
     * write twice to make sure non-target interface
     * SEL_A/B point the same clk input.
     */
    writel(reg, (*mux).reg);
    writel(reg, (*mux).reg);

    if !(*mux).lock.is_null() {
        spin_unlock_irqrestore((*mux).lock, flags);
    }
    0
}

unsafe fn imx8m_clk_composite_mux_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    clk_mux_ops.determine_rate.unwrap()(hw, req)
}

static imx8m_clk_composite_mux_ops: clk_ops = clk_ops {
    get_parent: Some(imx8m_clk_composite_mux_get_parent),
    set_parent: Some(imx8m_clk_composite_mux_set_parent),
    determine_rate: Some(imx8m_clk_composite_mux_determine_rate),
};

unsafe fn imx8m_clk_composite_gate_enable(hw: *mut clk_hw) -> c_int {
    let gate = to_clk_gate(hw);
    let mut flags: c_ulong = 0;
    spin_lock_irqsave((*gate).lock, &mut flags);
    let mut val = readl((*gate).reg);
    val |= BIT((*gate).bit_idx);
    writel(val, (*gate).reg);
    spin_unlock_irqrestore((*gate).lock, flags);
    0
}

unsafe fn imx8m_clk_composite_gate_disable(_hw: *mut clk_hw) {
    /* composite clk requires the disable hook */
}

static imx8m_clk_composite_gate_ops: clk_ops = clk_ops {
    enable: Some(imx8m_clk_composite_gate_enable),
    disable: Some(imx8m_clk_composite_gate_disable),
    is_enabled: Some(clk_gate_is_enabled),
};

unsafe fn __imx8m_clk_hw_composite(
    name: *const c_char,
    parent_names: *const *const c_char,
    num_parents: c_int,
    reg: *mut core::ffi::c_void,
    composite_flags: u32,
    mut flags: c_ulong,
) -> *mut clk_hw {
    let mut hw: *mut clk_hw = ERR_PTR(-ENOMEM as isize);
    let mux = kzalloc_obj::<clk_mux>();
    if mux.is_null() {
        return ERR_CAST(hw);
    }
    let mux_hw = &mut (*mux).hw;
    (*mux).reg = reg;
    (*mux).shift = PCG_PCS_SHIFT;
    (*mux).mask = PCG_PCS_MASK;
    (*mux).lock = &mut imx_ccm_lock;

    let div = kzalloc_obj::<clk_divider>();
    if div.is_null() {
        kfree(mux as *mut core::ffi::c_void);
        return ERR_CAST(hw);
    }
    let div_hw = &mut (*div).hw;
    (*div).reg = reg;
    let divider_ops: *const clk_ops;
    let mux_ops: *const clk_ops;
    if composite_flags & IMX_COMPOSITE_CORE != 0 {
        (*div).shift = PCG_DIV_SHIFT;
        (*div).width = PCG_CORE_DIV_WIDTH;
        divider_ops = &clk_divider_ops;
        mux_ops = &imx8m_clk_composite_mux_ops;
    } else if composite_flags & IMX_COMPOSITE_BUS != 0 {
        (*div).shift = PCG_PREDIV_SHIFT;
        (*div).width = PCG_PREDIV_WIDTH;
        divider_ops = &imx8m_clk_composite_divider_ops;
        mux_ops = &imx8m_clk_composite_mux_ops;
    } else {
        (*div).shift = PCG_PREDIV_SHIFT;
        (*div).width = PCG_PREDIV_WIDTH;
        divider_ops = &imx8m_clk_composite_divider_ops;
        mux_ops = &clk_mux_ops;
        if composite_flags & IMX_COMPOSITE_FW_MANAGED == 0 {
            flags |= CLK_SET_PARENT_GATE;
        }
    }
    (*div).lock = &mut imx_ccm_lock;
    (*div).flags = CLK_DIVIDER_ROUND_CLOSEST;

    /* skip registering the gate ops if M4 is enabled */
    let gate = kzalloc_obj::<clk_gate>();
    if gate.is_null() {
        kfree(div as *mut core::ffi::c_void);
        kfree(mux as *mut core::ffi::c_void);
        return ERR_CAST(hw);
    }
    let gate_hw = &mut (*gate).hw;
    (*gate).reg = reg;
    (*gate).bit_idx = PCG_CGC_SHIFT;
    (*gate).lock = &mut imx_ccm_lock;
    let gate_ops: *const clk_ops = if !mcore_booted {
        &clk_gate_ops
    } else {
        &imx8m_clk_composite_gate_ops
    };

    hw = clk_hw_register_composite(
        core::ptr::null_mut(),
        name,
        parent_names,
        num_parents,
        mux_hw,
        mux_ops,
        div_hw,
        divider_ops,
        gate_hw,
        gate_ops,
        flags,
    );
    if IS_ERR(hw) {
        kfree(gate as *mut core::ffi::c_void);
        kfree(div as *mut core::ffi::c_void);
        kfree(mux as *mut core::ffi::c_void);
        return ERR_CAST(hw);
    }
    hw
}

// EXPORT_SYMBOL_GPL(__imx8m_clk_hw_composite);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
