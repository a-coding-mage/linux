// SPDX-License-Identifier: GPL-2.0-only
/*
 * Hisilicon hi6220 SoC divider clock driver
 *
 * Copyright (c) 2015 Hisilicon Limited.
 *
 * Author: Bintian Wang <bintian.wang@huawei.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[inline]
const fn div_mask(width: u32) -> u32 {
    (1u32 << width).wrapping_sub(1)
}

#[repr(C)]
pub struct hi6220_clk_divider {
    pub hw: clk_hw,
    pub reg: *mut core::ffi::c_void,
    pub shift: u8,
    pub width: u8,
    pub mask: u32,
    pub lock: *mut spinlock_t,
    pub table: [clk_div_table; 0],
}

#[inline]
unsafe fn to_hi6220_clk_divider(hw: *mut clk_hw) -> *mut hi6220_clk_divider {
    (hw as *mut u8).sub(core::mem::offset_of!(hi6220_clk_divider, hw))
        as *mut hi6220_clk_divider
}

unsafe fn hi6220_clkdiv_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: libc::c_ulong,
) -> libc::c_ulong {
    let dclk = &*to_hi6220_clk_divider(hw);
    let mut val: u32 = readl_relaxed(dclk.reg) >> dclk.shift;
    val &= div_mask(dclk.width as u32);

    divider_recalc_rate(
        hw,
        parent_rate,
        val,
        dclk.table.as_ptr(),
        CLK_DIVIDER_ROUND_CLOSEST,
        dclk.width,
    )
}

unsafe fn hi6220_clkdiv_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> libc::c_int {
    let dclk = &*to_hi6220_clk_divider(hw);

    divider_determine_rate(
        hw,
        req,
        dclk.table.as_ptr(),
        dclk.width,
        CLK_DIVIDER_ROUND_CLOSEST,
    )
}

unsafe fn hi6220_clkdiv_set_rate(
    hw: *mut clk_hw,
    rate: libc::c_ulong,
    parent_rate: libc::c_ulong,
) -> libc::c_int {
    let dclk = &*to_hi6220_clk_divider(hw);
    let value = divider_get_val(
        rate,
        parent_rate,
        dclk.table.as_ptr(),
        dclk.width,
        CLK_DIVIDER_ROUND_CLOSEST,
    );
    let mut flags: libc::c_ulong = 0;

    if !dclk.lock.is_null() {
        spin_lock_irqsave(dclk.lock, &mut flags);
    }

    let mut data = readl_relaxed(dclk.reg);
    data &= !(div_mask(dclk.width as u32) << dclk.shift);
    data |= value << dclk.shift;
    data |= dclk.mask;

    writel_relaxed(data, dclk.reg);

    if !dclk.lock.is_null() {
        spin_unlock_irqrestore(dclk.lock, flags);
    }

    0
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe fn(*mut clk_hw, libc::c_ulong) -> libc::c_ulong>,
    pub determine_rate: Option<unsafe fn(*mut clk_hw, *mut clk_rate_request) -> libc::c_int>,
    pub set_rate: Option<unsafe fn(*mut clk_hw, libc::c_ulong, libc::c_ulong) -> libc::c_int>,
}

static HI6220_CLKDIV_OPS: clk_ops = clk_ops {
    recalc_rate: Some(hi6220_clkdiv_recalc_rate),
    determine_rate: Some(hi6220_clkdiv_determine_rate),
    set_rate: Some(hi6220_clkdiv_set_rate),
};

pub unsafe fn hi6220_register_clkdiv(
    dev: *mut device,
    name: *const libc::c_char,
    parent_name: *const libc::c_char,
    flags: libc::c_ulong,
    reg: *mut core::ffi::c_void,
    shift: u8,
    width: u8,
    mask_bit: u32,
    lock: *mut spinlock_t,
) -> *mut clk {
    let max_div = div_mask(width as u32).wrapping_add(1);
    let min_div = 1u32;
    let size = core::mem::size_of::<hi6220_clk_divider>()
        .wrapping_add((max_div as usize + 1).wrapping_mul(core::mem::size_of::<clk_div_table>()));
    let div = kzalloc(size, GFP_KERNEL) as *mut hi6220_clk_divider;
    if div.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    for i in 0..max_div {
        let table = &mut (*div).table.as_mut_ptr().add(i as usize);
        (*table).div = min_div + i;
        (*table).val = (*table).div - 1;
    }

    let mut init: clk_init_data = core::mem::zeroed();
    init.name = name;
    init.ops = &HI6220_CLKDIV_OPS;
    init.flags = flags;
    init.parent_names = if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
    init.num_parents = if !parent_name.is_null() { 1 } else { 0 };

    (*div).reg = reg;
    (*div).shift = shift;
    (*div).width = width;
    (*div).mask = if mask_bit != 0 { BIT(mask_bit) } else { 0 };
    (*div).lock = lock;
    (*div).hw.init = &init;

    let clk = clk_register(dev, &mut (*div).hw);
    if IS_ERR(clk) {
        kfree(div as *mut core::ffi::c_void);
    }

    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
