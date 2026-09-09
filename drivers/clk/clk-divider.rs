// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2011 Sascha Hauer, Pengutronix <s.hauer@pengutronix.de>
 * Copyright (C) 2011 Richard Zhao, Linaro <richard.zhao@linaro.org>
 * Copyright (C) 2011-2012 Mike Turquette, Linaro Ltd <mturquette@linaro.org>
 *
 * Adjustable divider clock implementation
 */

// Linux kernel dependencies are supplied by other translation units.

#[inline]
unsafe fn clk_div_readl(divider: *mut clk_divider) -> u32 {
    if (*divider).flags & CLK_DIVIDER_BIG_ENDIAN != 0 {
        return ioread32be((*divider).reg);
    }
    readl((*divider).reg)
}

#[inline]
unsafe fn clk_div_writel(divider: *mut clk_divider, val: u32) {
    if (*divider).flags & CLK_DIVIDER_BIG_ENDIAN != 0 {
        iowrite32be(val, (*divider).reg);
    } else {
        writel(val, (*divider).reg);
    }
}

unsafe fn _get_table_maxdiv(table: *const clk_div_table, width: u8) -> u32 {
    let mut maxdiv = 0;
    let mask = clk_div_mask(width);
    let mut clkt = table;
    while (*clkt).div != 0 {
        if (*clkt).div > maxdiv && (*clkt).val <= mask {
            maxdiv = (*clkt).div;
        }
        clkt = clkt.add(1);
    }
    maxdiv
}

unsafe fn _get_table_mindiv(table: *const clk_div_table) -> u32 {
    let mut mindiv = u32::MAX;
    let mut clkt = table;
    while (*clkt).div != 0 {
        if (*clkt).div < mindiv {
            mindiv = (*clkt).div;
        }
        clkt = clkt.add(1);
    }
    mindiv
}

unsafe fn _get_maxdiv(table: *const clk_div_table, width: u8, flags: c_ulong) -> u32 {
    if flags & CLK_DIVIDER_ONE_BASED != 0 { return clk_div_mask(width); }
    if flags & CLK_DIVIDER_POWER_OF_TWO != 0 { return 1 << clk_div_mask(width); }
    if flags & CLK_DIVIDER_EVEN_INTEGERS != 0 { return 2 * (clk_div_mask(width) + 1); }
    if !table.is_null() { return _get_table_maxdiv(table, width); }
    clk_div_mask(width) + 1
}

unsafe fn _get_table_div(table: *const clk_div_table, val: u32) -> u32 {
    let mut clkt = table;
    while (*clkt).div != 0 {
        if (*clkt).val == val { return (*clkt).div; }
        clkt = clkt.add(1);
    }
    0
}

unsafe fn _get_div(table: *const clk_div_table, val: u32, flags: c_ulong, width: u8) -> u32 {
    if flags & CLK_DIVIDER_ONE_BASED != 0 { return val; }
    if flags & CLK_DIVIDER_POWER_OF_TWO != 0 { return 1 << val; }
    if flags & CLK_DIVIDER_MAX_AT_ZERO != 0 { return if val != 0 { val } else { clk_div_mask(width) + 1 }; }
    if flags & CLK_DIVIDER_EVEN_INTEGERS != 0 { return 2 * (val + 1); }
    if !table.is_null() { return _get_table_div(table, val); }
    val + 1
}

unsafe fn _get_table_val(table: *const clk_div_table, div: u32) -> u32 {
    let mut clkt = table;
    while (*clkt).div != 0 {
        if (*clkt).div == div { return (*clkt).val; }
        clkt = clkt.add(1);
    }
    0
}

unsafe fn _get_val(table: *const clk_div_table, div: u32, flags: c_ulong, width: u8) -> u32 {
    if flags & CLK_DIVIDER_ONE_BASED != 0 { return div; }
    if flags & CLK_DIVIDER_POWER_OF_TWO != 0 { return __ffs(div); }
    if flags & CLK_DIVIDER_MAX_AT_ZERO != 0 { return if div == clk_div_mask(width) + 1 { 0 } else { div }; }
    if flags & CLK_DIVIDER_EVEN_INTEGERS != 0 { return (div >> 1) - 1; }
    if !table.is_null() { return _get_table_val(table, div); }
    div - 1
}

pub unsafe fn divider_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong, val: u32,
                                  table: *const clk_div_table, flags: c_ulong, width: c_ulong) -> c_ulong {
    let div = _get_div(table, val, flags, width as u8);
    if div == 0 {
        WARN(flags & CLK_DIVIDER_ALLOW_ZERO == 0, "%s: Zero divisor and CLK_DIVIDER_ALLOW_ZERO not set\n", clk_hw_get_name(hw));
        return parent_rate;
    }
    DIV_ROUND_UP_ULL(parent_rate as u64, div as u64)
}

unsafe fn clk_divider_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let divider = to_clk_divider(hw);
    let mut val = clk_div_readl(divider) >> (*divider).shift;
    val &= clk_div_mask((*divider).width);
    divider_recalc_rate(hw, parent_rate, val, (*divider).table, (*divider).flags, (*divider).width as c_ulong)
}

unsafe fn _is_valid_table_div(table: *const clk_div_table, div: u32) -> bool {
    let mut clkt = table;
    while (*clkt).div != 0 {
        if (*clkt).div == div { return true; }
        clkt = clkt.add(1);
    }
    false
}

unsafe fn _is_valid_div(table: *const clk_div_table, div: u32, flags: c_ulong) -> bool {
    if flags & CLK_DIVIDER_POWER_OF_TWO != 0 { return is_power_of_2(div); }
    if !table.is_null() { return _is_valid_table_div(table, div); }
    true
}

unsafe fn _round_up_table(table: *const clk_div_table, div: i32) -> i32 {
    let mut up = i32::MAX;
    let mut clkt = table;
    while (*clkt).div != 0 {
        let value = (*clkt).div as i32;
        if value == div { return value; }
        if value >= div && value - div < up - div { up = value; }
        clkt = clkt.add(1);
    }
    up
}

unsafe fn _round_down_table(table: *const clk_div_table, div: i32) -> i32 {
    let mut down = _get_table_mindiv(table) as i32;
    let mut clkt = table;
    while (*clkt).div != 0 {
        let value = (*clkt).div as i32;
        if value == div { return value; }
        if value <= div && div - value < div - down { down = value; }
        clkt = clkt.add(1);
    }
    down
}

unsafe fn _div_round_up(table: *const clk_div_table, parent_rate: c_ulong, rate: c_ulong, flags: c_ulong) -> i32 {
    let mut div = DIV_ROUND_UP_ULL(parent_rate as u64, rate as u64) as i32;
    if flags & CLK_DIVIDER_POWER_OF_TWO != 0 { div = __roundup_pow_of_two(div as u32) as i32; }
    if !table.is_null() { div = _round_up_table(table, div); }
    div
}

unsafe fn _div_round_closest(table: *const clk_div_table, parent_rate: c_ulong, rate: c_ulong, flags: c_ulong) -> i32 {
    let mut up = DIV_ROUND_UP_ULL(parent_rate as u64, rate as u64) as i32;
    let mut down = (parent_rate / rate) as i32;
    if flags & CLK_DIVIDER_POWER_OF_TWO != 0 {
        up = __roundup_pow_of_two(up as u32) as i32;
        down = __rounddown_pow_of_two(down as u32) as i32;
    } else if !table.is_null() {
        up = _round_up_table(table, up);
        down = _round_down_table(table, down);
    }
    let up_rate = DIV_ROUND_UP_ULL(parent_rate as u64, up as u64);
    let down_rate = DIV_ROUND_UP_ULL(parent_rate as u64, down as u64);
    if rate - up_rate <= down_rate - rate { up } else { down as i32 }
}

unsafe fn _div_round(table: *const clk_div_table, parent_rate: c_ulong, rate: c_ulong, flags: c_ulong) -> i32 {
    if flags & CLK_DIVIDER_ROUND_CLOSEST != 0 { _div_round_closest(table, parent_rate, rate, flags) } else { _div_round_up(table, parent_rate, rate, flags) }
}

unsafe fn _is_best_div(rate: c_ulong, now: c_ulong, best: c_ulong, flags: c_ulong) -> bool {
    if flags & CLK_DIVIDER_ROUND_CLOSEST != 0 { return abs(rate as i64 - now as i64) < abs(rate as i64 - best as i64); }
    now <= rate && now > best
}

unsafe fn _next_div(table: *const clk_div_table, mut div: i32, flags: c_ulong) -> i32 {
    div += 1;
    if flags & CLK_DIVIDER_POWER_OF_TWO != 0 { return __roundup_pow_of_two(div as u32) as i32; }
    if !table.is_null() { return _round_up_table(table, div); }
    div
}

unsafe fn clk_divider_bestdiv(hw: *mut clk_hw, parent: *mut clk_hw, rate: c_ulong,
                              best_parent_rate: *mut c_ulong, table: *const clk_div_table,
                              width: u8, flags: c_ulong) -> i32 {
    let mut rate = rate;
    let mut bestdiv = 0;
    let mut best = 0;
    let mut parent_rate;
    let maxdiv = _get_maxdiv(table, width, flags) as c_ulong;
    let parent_rate_saved = *best_parent_rate;
    if rate == 0 { rate = 1; }
    if clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT == 0 {
        parent_rate = *best_parent_rate;
        let mut div = _div_round(table, parent_rate, rate, flags);
        if div == 0 { div = 1; }
        if div as c_ulong > maxdiv { div = maxdiv as i32; }
        return div;
    }
    let mut i = _next_div(table, 0, flags);
    while (i as c_ulong) <= maxdiv {
        let mut target_parent_rate = 0;
        let overflow = check_mul_overflow(rate, i as c_ulong, &mut target_parent_rate);
        if !overflow && target_parent_rate == parent_rate_saved { *best_parent_rate = parent_rate_saved; return i; }
        if overflow { target_parent_rate = c_ulong::MAX; }
        parent_rate = clk_hw_round_rate(parent, target_parent_rate);
        let now = DIV_ROUND_UP_ULL(parent_rate as u64, i as u64);
        if _is_best_div(rate, now, best, flags) { bestdiv = i; best = now; *best_parent_rate = parent_rate; }
        if overflow { break; }
        i = _next_div(table, i, flags);
    }
    if bestdiv == 0 { bestdiv = maxdiv as i32; *best_parent_rate = clk_hw_round_rate(parent, 1); }
    bestdiv
}

pub unsafe fn divider_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request,
                                     table: *const clk_div_table, width: u8, flags: c_ulong) -> i32 {
    let div = clk_divider_bestdiv(hw, (*req).best_parent_hw, (*req).rate, &mut (*req).best_parent_rate, table, width, flags);
    (*req).rate = DIV_ROUND_UP_ULL((*req).best_parent_rate as u64, div as u64);
    0
}

pub unsafe fn divider_ro_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request,
                                       table: *const clk_div_table, width: u8, flags: c_ulong, val: u32) -> i32 {
    let div = _get_div(table, val, flags, width);
    if clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT != 0 {
        if (*req).best_parent_hw.is_null() { return -EINVAL; }
        (*req).best_parent_rate = clk_hw_round_rate((*req).best_parent_hw, (*req).rate * div as c_ulong);
    }
    (*req).rate = DIV_ROUND_UP_ULL((*req).best_parent_rate as u64, div as u64);
    0
}

unsafe fn clk_divider_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let divider = to_clk_divider(hw);
    if (*divider).flags & CLK_DIVIDER_READ_ONLY != 0 {
        let mut val = clk_div_readl(divider) >> (*divider).shift;
        val &= clk_div_mask((*divider).width);
        return divider_ro_determine_rate(hw, req, (*divider).table, (*divider).width, (*divider).flags, val);
    }
    divider_determine_rate(hw, req, (*divider).table, (*divider).width, (*divider).flags)
}

pub unsafe fn divider_get_val(rate: c_ulong, parent_rate: c_ulong, table: *const clk_div_table,
                              width: u8, flags: c_ulong) -> i32 {
    let div = DIV_ROUND_UP_ULL(parent_rate as u64, rate as u64) as u32;
    if !_is_valid_div(table, div, flags) { return -EINVAL; }
    let value = _get_val(table, div, flags, width);
    core::cmp::min(value, clk_div_mask(width)) as i32
}

unsafe fn clk_divider_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> i32 {
    let divider = to_clk_divider(hw);
    let value = divider_get_val(rate, parent_rate, (*divider).table, (*divider).width, (*divider).flags);
    if value < 0 { return value; }
    let mut flags = 0;
    if !(*divider).lock.is_null() { spin_lock_irqsave((*divider).lock, &mut flags); } else { __acquire((*divider).lock); }
    let mut val;
    if (*divider).flags & CLK_DIVIDER_HIWORD_MASK != 0 {
        val = clk_div_mask((*divider).width) << ((*divider).shift + 16);
    } else {
        val = clk_div_readl(divider);
        val &= !(clk_div_mask((*divider).width) << (*divider).shift);
    }
    val |= (value as u32) << (*divider).shift;
    clk_div_writel(divider, val);
    if !(*divider).lock.is_null() { spin_unlock_irqrestore((*divider).lock, flags); } else { __release((*divider).lock); }
    0
}

pub static clk_divider_ops: clk_ops = clk_ops { recalc_rate: Some(clk_divider_recalc_rate), determine_rate: Some(clk_divider_determine_rate), set_rate: Some(clk_divider_set_rate) };
pub static clk_divider_ro_ops: clk_ops = clk_ops { recalc_rate: Some(clk_divider_recalc_rate), determine_rate: Some(clk_divider_determine_rate), set_rate: None };

pub unsafe fn __clk_hw_register_divider(dev: *mut device, np: *mut device_node, name: *const c_char,
    parent_name: *const c_char, parent_hw: *const clk_hw, parent_data: *const clk_parent_data,
    flags: c_ulong, reg: *mut c_void, shift: u8, width: u8, clk_divider_flags: c_ulong,
    table: *const clk_div_table, lock: *mut spinlock_t) -> *mut clk_hw {
    if clk_divider_flags & CLK_DIVIDER_HIWORD_MASK != 0 && width + shift > 16 { pr_warn!("divider value exceeds LOWORD field\n"); return ERR_PTR(-EINVAL); }
    let div = kzalloc_obj::<clk_divider>();
    if div.is_null() { return ERR_PTR(-ENOMEM); }
    let mut init: clk_init_data = core::mem::zeroed();
    init.name = name;
    init.ops = if clk_divider_flags & CLK_DIVIDER_READ_ONLY != 0 { &clk_divider_ro_ops } else { &clk_divider_ops };
    init.flags = flags;
    init.parent_names = if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
    init.parent_hws = if !parent_hw.is_null() { &parent_hw } else { core::ptr::null() };
    init.parent_data = parent_data;
    init.num_parents = if !parent_name.is_null() || !parent_hw.is_null() || !parent_data.is_null() { 1 } else { 0 };
    (*div).reg = reg; (*div).shift = shift; (*div).width = width; (*div).flags = clk_divider_flags;
    (*div).lock = lock; (*div).hw.init = &init; (*div).table = table;
    let hw = &mut (*div).hw;
    let ret = clk_hw_register(dev, hw);
    if ret != 0 { kfree(div); return ERR_PTR(ret); }
    hw
}

pub unsafe fn clk_register_divider_table(dev: *mut device, name: *const c_char, parent_name: *const c_char,
    flags: c_ulong, reg: *mut c_void, shift: u8, width: u8, clk_divider_flags: c_ulong,
    table: *const clk_div_table, lock: *mut spinlock_t) -> *mut clk {
    let hw = __clk_hw_register_divider(dev, core::ptr::null_mut(), name, parent_name, core::ptr::null(), core::ptr::null(), flags, reg, shift, width, clk_divider_flags, table, lock);
    if IS_ERR(hw) { return ERR_CAST(hw); }
    (*hw).clk
}

pub unsafe fn clk_unregister_divider(clk: *mut clk) {
    let hw = __clk_get_hw(clk);
    if hw.is_null() { return; }
    let div = to_clk_divider(hw);
    clk_unregister(clk); kfree(div);
}

pub unsafe fn clk_hw_unregister_divider(hw: *mut clk_hw) { let div = to_clk_divider(hw); clk_hw_unregister(hw); kfree(div); }

unsafe fn devm_clk_hw_release_divider(_dev: *mut device, res: *mut c_void) { clk_hw_unregister_divider(*(res as *mut *mut clk_hw)); }

pub unsafe fn __devm_clk_hw_register_divider(dev: *mut device, np: *mut device_node, name: *const c_char,
    parent_name: *const c_char, parent_hw: *const clk_hw, parent_data: *const clk_parent_data, flags: c_ulong,
    reg: *mut c_void, shift: u8, width: u8, clk_divider_flags: c_ulong, table: *const clk_div_table,
    lock: *mut spinlock_t) -> *mut clk_hw {
    let ptr = devres_alloc(devm_clk_hw_release_divider, core::mem::size_of::<*mut clk_hw>(), GFP_KERNEL);
    if ptr.is_null() { return ERR_PTR(-ENOMEM); }
    let hw = __clk_hw_register_divider(dev, np, name, parent_name, parent_hw, parent_data, flags, reg, shift, width, clk_divider_flags, table, lock);
    if !IS_ERR(hw) { *(ptr as *mut *mut clk_hw) = hw; devres_add(dev, ptr); } else { devres_free(ptr); }
    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
