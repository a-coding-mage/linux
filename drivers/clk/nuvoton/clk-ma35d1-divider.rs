// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 Nuvoton Technology Corp.
 * Author: Chi-Fang Li <cfli0@nuvoton.com>
 */

// Dependencies supplied by the Linux clock, device, regmap, spinlock, and
// MA35D1 clock headers are intentionally left as external Rust symbols.

#[repr(C)]
pub struct ma35d1_adc_clk_div {
    pub hw: clk_hw,
    pub reg: *mut core::ffi::c_void,
    pub shift: u8,
    pub width: u8,
    pub mask: u32,
    pub table: *const clk_div_table,
    // protects concurrent access to clock divider registers
    pub lock: *mut spinlock_t,
}

#[inline]
unsafe fn to_ma35d1_adc_clk_div(hw: *mut clk_hw) -> *mut ma35d1_adc_clk_div {
    // Equivalent to container_of(hw, struct ma35d1_adc_clk_div, hw).
    (hw as *mut u8).sub(core::mem::offset_of!(ma35d1_adc_clk_div, hw))
        as *mut ma35d1_adc_clk_div
}

unsafe fn ma35d1_clkdiv_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: libc::c_ulong,
) -> libc::c_ulong {
    let dclk = to_ma35d1_adc_clk_div(hw);
    let mut val: u32 = readl_relaxed((*dclk).reg) >> (*dclk).shift;
    val &= clk_div_mask((*dclk).width);
    val = val.wrapping_add(1);
    divider_recalc_rate(
        hw,
        parent_rate,
        val,
        (*dclk).table,
        CLK_DIVIDER_ROUND_CLOSEST,
        (*dclk).width,
    )
}

unsafe fn ma35d1_clkdiv_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> libc::c_int {
    let dclk = to_ma35d1_adc_clk_div(hw);
    divider_determine_rate(
        hw,
        req,
        (*dclk).table,
        (*dclk).width,
        CLK_DIVIDER_ROUND_CLOSEST,
    )
}

unsafe fn ma35d1_clkdiv_set_rate(
    hw: *mut clk_hw,
    rate: libc::c_ulong,
    parent_rate: libc::c_ulong,
) -> libc::c_int {
    let dclk = to_ma35d1_adc_clk_div(hw);
    let value = divider_get_val(
        rate,
        parent_rate,
        (*dclk).table,
        (*dclk).width,
        CLK_DIVIDER_ROUND_CLOSEST,
    );
    let mut flags: libc::c_ulong = 0;

    spin_lock_irqsave((*dclk).lock, &mut flags);
    let mut data = readl_relaxed((*dclk).reg);
    data &= !(clk_div_mask((*dclk).width) << (*dclk).shift);
    data |= (value - 1) << (*dclk).shift;
    data |= (*dclk).mask;
    writel_relaxed(data, (*dclk).reg);
    spin_unlock_irqrestore((*dclk).lock, flags);
    0
}

static ma35d1_adc_clkdiv_ops: clk_ops = clk_ops {
    recalc_rate: Some(ma35d1_clkdiv_recalc_rate),
    determine_rate: Some(ma35d1_clkdiv_determine_rate),
    set_rate: Some(ma35d1_clkdiv_set_rate),
};

pub unsafe fn ma35d1_reg_adc_clkdiv(
    dev: *mut device,
    name: *const libc::c_char,
    parent_hw: *mut clk_hw,
    lock: *mut spinlock_t,
    flags: libc::c_ulong,
    reg: *mut core::ffi::c_void,
    shift: u8,
    width: u8,
    mask_bit: u32,
) -> *mut clk_hw {
    let div = devm_kzalloc(dev, core::mem::size_of::<ma35d1_adc_clk_div>(), GFP_KERNEL)
        as *mut ma35d1_adc_clk_div;
    if div.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    let max_div = clk_div_mask(width).wrapping_add(1);
    let min_div: u32 = 1;
    let table = devm_kcalloc(
        dev,
        (max_div + 1) as usize,
        core::mem::size_of::<clk_div_table>(),
        GFP_KERNEL,
    ) as *mut clk_div_table;
    if table.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    for i in 0..max_div {
        (*table.add(i as usize)).val = min_div + i;
        (*table.add(i as usize)).div = 2 * (*table.add(i as usize)).val;
    }
    (*table.add(max_div as usize)).val = 0;
    (*table.add(max_div as usize)).div = 0;

    let mut init: clk_init_data = core::mem::zeroed();
    init.name = name;
    init.ops = &ma35d1_adc_clkdiv_ops;
    init.flags |= flags;
    let mut pdata = clk_parent_data { index: 0, hw: parent_hw };
    init.parent_data = &mut pdata;
    init.num_parents = 1;

    (*div).reg = reg;
    (*div).shift = shift;
    (*div).width = width;
    (*div).mask = if mask_bit != 0 { 1u32 << mask_bit } else { 0 };
    (*div).lock = lock;
    (*div).hw.init = &mut init;
    (*div).table = table;

    let hw = &mut (*div).hw as *mut clk_hw;
    let ret = devm_clk_hw_register(dev, hw);
    if ret != 0 {
        return ERR_PTR(ret);
    }
    hw
}

// EXPORT_SYMBOL_GPL(ma35d1_reg_adc_clkdiv)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
