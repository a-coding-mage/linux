// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018 Fuzhou Rockchip Electronics Co., Ltd
 */

// Linux clock-provider, I/O, slab, and local clock declarations are supplied
// by the surrounding translation unit.

const fn div_mask(width: u8) -> u32 {
    (1u32 << width) - 1
}

unsafe fn _is_best_half_div(
    rate: libc::c_ulong,
    now: libc::c_ulong,
    best: libc::c_ulong,
    flags: libc::c_ulong,
) -> bool {
    if flags & CLK_DIVIDER_ROUND_CLOSEST as libc::c_ulong != 0 {
        (rate.wrapping_sub(now) as libc::c_long).abs()
            < (rate.wrapping_sub(best) as libc::c_long).abs()
    } else {
        now <= rate && now > best
    }
}

unsafe fn clk_half_divider_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: libc::c_ulong,
) -> libc::c_ulong {
    let divider = to_clk_divider(hw);
    let mut val: u32 = readl((*divider).reg) >> (*divider).shift;
    val &= div_mask((*divider).width);
    val = val * 2 + 3;
    div_round_up_ull((parent_rate as u64) * 2, val as u64) as libc::c_ulong
}

unsafe fn clk_half_divider_bestdiv(
    hw: *mut clk_hw,
    mut rate: libc::c_ulong,
    best_parent_rate: *mut libc::c_ulong,
    width: u8,
    flags: libc::c_ulong,
) -> libc::c_int {
    let mut bestdiv: u32 = 0;
    let mut parent_rate: libc::c_ulong;
    let mut best: libc::c_ulong = 0;
    let mut now: libc::c_ulong;
    let mut maxdiv: libc::c_ulong = div_mask(width) as libc::c_ulong;
    let parent_rate_saved = *best_parent_rate;

    if rate == 0 { rate = 1; }
    if clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT == 0 {
        parent_rate = *best_parent_rate;
        bestdiv = div_round_up_ull((parent_rate as u64) * 2, rate as u64) as u32;
        if bestdiv < 3 { bestdiv = 0; } else { bestdiv = (bestdiv - 3) / 2; }
        bestdiv = if bestdiv as libc::c_ulong > maxdiv { maxdiv as u32 } else { bestdiv };
        return bestdiv as libc::c_int;
    }
    maxdiv = core::cmp::min(libc::c_ulong::MAX / rate, maxdiv);
    for i in 0..=maxdiv {
        if rate as u64 * (i * 2 + 3) as u64 == parent_rate_saved as u64 * 2 {
            *best_parent_rate = parent_rate_saved;
            return i as libc::c_int;
        }
        parent_rate = clk_hw_round_rate((*hw).parent,
            (rate as u64 * (i * 2 + 3) as u64 / 2) as libc::c_ulong);
        now = div_round_up_ull((parent_rate as u64) * 2, (i * 2 + 3) as u64) as libc::c_ulong;
        if _is_best_half_div(rate, now, best, flags) {
            bestdiv = i as u32; best = now; *best_parent_rate = parent_rate;
        }
    }
    if bestdiv == 0 {
        bestdiv = div_mask(width);
        *best_parent_rate = clk_hw_round_rate((*hw).parent, 1);
    }
    bestdiv as libc::c_int
}

unsafe fn clk_half_divider_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> libc::c_int {
    let divider = to_clk_divider(hw);
    let div = clk_half_divider_bestdiv(hw, (*req).rate, &mut (*req).best_parent_rate,
                                       (*divider).width, (*divider).flags);
    (*req).rate = div_round_up_ull(((*req).best_parent_rate as u64) * 2,
                                   (div * 2 + 3) as u64) as libc::c_ulong;
    0
}

unsafe fn clk_half_divider_set_rate(hw: *mut clk_hw, rate: libc::c_ulong,
                                    parent_rate: libc::c_ulong) -> libc::c_int {
    let divider = to_clk_divider(hw);
    let mut value = div_round_up_ull((parent_rate as u64) * 2, rate as u64) as u32;
    value = (value - 3) / 2;
    value = core::cmp::min(value, div_mask((*divider).width));
    let mut flags: libc::c_ulong = 0;
    if !(*divider).lock.is_null() { spin_lock_irqsave((*divider).lock, &mut flags); } else { __acquire((*divider).lock); }
    let mut val: u32;
    if (*divider).flags & CLK_DIVIDER_HIWORD_MASK != 0 {
        val = div_mask((*divider).width) << ((*divider).shift + 16);
    } else { val = readl((*divider).reg); val &= !(div_mask((*divider).width) << (*divider).shift); }
    val |= value << (*divider).shift; writel(val, (*divider).reg);
    if !(*divider).lock.is_null() { spin_unlock_irqrestore((*divider).lock, flags); } else { __release((*divider).lock); }
    0
}

static clk_ops clk_half_divider_ops = clk_ops {
    recalc_rate: Some(clk_half_divider_recalc_rate),
    determine_rate: Some(clk_half_divider_determine_rate),
    set_rate: Some(clk_half_divider_set_rate),
};

// Register a clock branch. Most clock branches have a source, mux, gate, and divider.
unsafe fn rockchip_clk_register_halfdiv(
    name: *const libc::c_char, parent_names: *const *const libc::c_char,
    num_parents: u8, base: *mut u8, muxdiv_offset: libc::c_int,
    mux_shift: u8, mux_width: u8, mux_flags: u8, div_shift: u8,
    div_width: u8, div_flags: u8, gate_offset: libc::c_int, gate_shift: u8,
    gate_flags: u8, flags: libc::c_ulong, lock: *mut spinlock_t,
) -> *mut clk {
    let mut hw: *mut clk_hw = err_ptr(-ENOMEM);
    let mut mux: *mut clk_mux = core::ptr::null_mut();
    let mut gate: *mut clk_gate = core::ptr::null_mut();
    let mut div: *mut clk_divider = core::ptr::null_mut();
    let mut mux_ops: *const clk_ops = core::ptr::null();
    let mut div_ops: *const clk_ops = core::ptr::null();
    let mut gate_ops: *const clk_ops = core::ptr::null();

    if num_parents > 1 {
        mux = kzalloc_obj::<clk_mux>();
        if mux.is_null() { return err_ptr(-ENOMEM); }
        (*mux).reg = base.add(muxdiv_offset as usize);
        (*mux).shift = mux_shift;
        (*mux).mask = (1u32 << mux_width) - 1;
        (*mux).flags = mux_flags;
        (*mux).lock = lock;
        mux_ops = if mux_flags & CLK_MUX_READ_ONLY != 0 { &clk_mux_ro_ops } else { &clk_mux_ops };
    }
    if gate_offset >= 0 {
        gate = kzalloc_obj::<clk_gate>();
        if gate.is_null() { goto_err_gate(mux); }
        (*gate).flags = gate_flags;
        (*gate).reg = base.add(gate_offset as usize);
        (*gate).bit_idx = gate_shift;
        (*gate).lock = lock;
        gate_ops = &clk_gate_ops;
    }
    if div_width > 0 {
        div = kzalloc_obj::<clk_divider>();
        if div.is_null() { kfree(gate); goto_err_gate(mux); }
        (*div).flags = div_flags;
        (*div).reg = base.add(muxdiv_offset as usize);
        (*div).shift = div_shift;
        (*div).width = div_width;
        (*div).lock = lock;
        div_ops = &clk_half_divider_ops;
    }
    hw = clk_hw_register_composite(core::ptr::null_mut(), name, parent_names, num_parents,
        if mux.is_null() { core::ptr::null_mut() } else { &mut (*mux).hw }, mux_ops,
        if div.is_null() { core::ptr::null_mut() } else { &mut (*div).hw }, div_ops,
        if gate.is_null() { core::ptr::null_mut() } else { &mut (*gate).hw }, gate_ops, flags);
    if is_err(hw) { kfree(div); kfree(gate); kfree(mux); return err_cast(hw); }
    (*hw).clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
