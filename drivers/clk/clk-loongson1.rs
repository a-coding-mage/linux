// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Clock driver for Loongson-1 SoC
 *
 * Copyright (C) 2012-2023 Keguang Zhang <keguang.zhang@gmail.com>
 */

// Linux clock, device-tree, I/O, locking, and clock binding declarations are
// supplied by the surrounding kernel translation.

const CLK_PLL_FREQ: usize = 0x0;
const CLK_PLL_DIV: usize = 0x4;

static mut LS1X_CLK_DIV_LOCK: spinlock_t = spinlock_t::new();

#[repr(C)]
struct ls1x_clk_pll_data {
    fixed: u32,
    shift: u8,
    int_shift: u8,
    int_width: u8,
    frac_shift: u8,
    frac_width: u8,
}

#[repr(C)]
struct ls1x_clk_div_data {
    shift: u8,
    width: u8,
    flags: c_ulong,
    table: *const clk_div_table,
    bypass_shift: u8,
    bypass_inv: u8,
    lock: *mut spinlock_t,
}

#[repr(C)]
struct ls1x_clk {
    reg: *mut core::ffi::c_void,
    offset: c_uint,
    hw: clk_hw,
    data: *const core::ffi::c_void,
}

unsafe fn to_ls1x_clk(hw: *mut clk_hw) -> *mut ls1x_clk {
    (hw as *mut u8).sub(core::mem::offset_of!(ls1x_clk, hw)) as *mut ls1x_clk
}

#[inline]
unsafe fn ls1x_pll_rate_part(val: c_uint, shift: c_uint, width: c_uint) -> c_ulong {
    ((val & (((1u32 << (shift + width + 1)) - 1) ^ ((1u32 << shift) - 1))) >> shift) as c_ulong
}

unsafe fn ls1x_pll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let clk = to_ls1x_clk(hw);
    let d = &*((*clk).data as *const ls1x_clk_pll_data);
    let val = readl((*clk).reg as *const _);
    let mut rate = d.fixed as c_ulong;
    rate += ls1x_pll_rate_part(val, d.int_shift as c_uint, d.int_width as c_uint);
    if d.frac_width != 0 {
        rate += ls1x_pll_rate_part(val, d.frac_shift as c_uint, d.frac_width as c_uint);
    }
    rate = rate.wrapping_mul(parent_rate);
    rate >> d.shift
}

static LS1X_PLL_CLK_OPS: clk_ops = clk_ops { recalc_rate: Some(ls1x_pll_recalc_rate), ..clk_ops::ZERO };

unsafe fn ls1x_divider_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let clk = to_ls1x_clk(hw);
    let d = &*((*clk).data as *const ls1x_clk_div_data);
    let val = (readl((*clk).reg as *const _) >> d.shift) & clk_div_mask(d.width);
    divider_recalc_rate(hw, parent_rate, val, d.table, d.flags, d.width)
}

unsafe fn ls1x_divider_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let clk = to_ls1x_clk(hw);
    let d = &*((*clk).data as *const ls1x_clk_div_data);
    divider_determine_rate(hw, req, d.table, d.width, d.flags)
}

unsafe fn ls1x_divider_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    let clk = to_ls1x_clk(hw);
    let d = &*((*clk).data as *const ls1x_clk_div_data);
    let div_val = divider_get_val(rate, parent_rate, d.table, d.width, d.flags);
    if div_val < 0 { return div_val; }
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(d.lock, &mut flags);
    let mut val = readl((*clk).reg as *const _);
    if d.bypass_inv != 0 { val &= !BIT(d.bypass_shift); } else { val |= BIT(d.bypass_shift); }
    writel(val, (*clk).reg as *mut _);
    val = readl((*clk).reg as *const _);
    val &= !(clk_div_mask(d.width) << d.shift);
    val |= (div_val as u32) << d.shift;
    writel(val, (*clk).reg as *mut _);
    val = readl((*clk).reg as *const _);
    if d.bypass_inv != 0 { val |= BIT(d.bypass_shift); } else { val &= !BIT(d.bypass_shift); }
    writel(val, (*clk).reg as *mut _);
    spin_unlock_irqrestore(d.lock, flags);
    0
}

static LS1X_CLK_DIVIDER_OPS: clk_ops = clk_ops {
    recalc_rate: Some(ls1x_divider_recalc_rate),
    determine_rate: Some(ls1x_divider_determine_rate),
    set_rate: Some(ls1x_divider_set_rate),
    ..clk_ops::ZERO
};

// The following declarations preserve the C macro-generated clock objects and
// their exact initialization data; kernel clock types/constants are external.
extern "C" {
    static mut ls1b_clk_pll: ls1x_clk;
    static mut ls1b_clk_cpu: ls1x_clk;
    static mut ls1b_clk_dc: ls1x_clk;
    static mut ls1b_clk_ahb: ls1x_clk;
    static mut ls1b_clk_apb: clk_hw;
    static mut ls1c_clk_pll: ls1x_clk;
    static mut ls1c_clk_cpu: ls1x_clk;
    static mut ls1c_clk_dc: ls1x_clk;
    static mut ls1c_clk_ahb: ls1x_clk;
    static mut ls1c_clk_apb: clk_hw;
    static mut ls1b_clk_hw_data: clk_hw_onecell_data;
    static mut ls1c_clk_hw_data: clk_hw_onecell_data;
}

static LS1C_AHB_DIV_TABLE: [clk_div_table; 5] = [
    clk_div_table { val: 0, div: 2 },
    clk_div_table { val: 1, div: 4 },
    clk_div_table { val: 2, div: 3 },
    clk_div_table { val: 3, div: 3 },
    clk_div_table { val: 0, div: 0 },
];

unsafe fn ls1x_clk_init(np: *mut device_node, hw_data: *mut clk_hw_onecell_data) {
    let reg = of_iomap(np, 0);
    if reg.is_null() { pr_err!("Unable to map base for %pOF\n", np); return; }
    let mut i: c_int = 0;
    while i < (*hw_data).num as c_int {
        if (*hw_data).hws[i as usize].is_null() { i += 1; continue; }
        if i != LS1X_CLKID_APB as c_int {
            let clk = to_ls1x_clk((*hw_data).hws[i as usize]);
            (*clk).reg = (reg as *mut u8).add((*clk).offset) as *mut _;
        }
        if of_clk_hw_register(np, (*hw_data).hws[i as usize]) != 0 { break; }
        i += 1;
    }
    if i == (*hw_data).num as c_int && of_clk_add_hw_provider(np, of_clk_hw_onecell_get, hw_data) == 0 { return; }
    pr_err!("Failed to register %pOF\n", np);
    while i > 0 { i -= 1; clk_hw_unregister((*hw_data).hws[i as usize]); }
    iounmap(reg);
}

unsafe fn ls1b_clk_init(np: *mut device_node) { ls1x_clk_init(np, &raw mut ls1b_clk_hw_data); }
unsafe fn ls1c_clk_init(np: *mut device_node) { ls1x_clk_init(np, &raw mut ls1c_clk_hw_data); }

// CLK_OF_DECLARE(ls1b_clk, "loongson,ls1b-clk", ls1b_clk_init);
// CLK_OF_DECLARE(ls1c_clk, "loongson,ls1c-clk", ls1c_clk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
