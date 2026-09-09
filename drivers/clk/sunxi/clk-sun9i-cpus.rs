// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2015 Chen-Yu Tsai
 *
 * Chen-Yu Tsai <wens@csie.org>
 *
 * Allwinner A80 CPUS clock driver
 */

// Linux clock, MMIO, allocation, spinlock, device-tree, and resource APIs
// are supplied by external dependencies.

static mut sun9i_a80_cpus_lock: spinlock_t = DEFINE_SPINLOCK!();

const SUN9I_CPUS_MAX_PARENTS: usize = 4;
const SUN9I_CPUS_MUX_PARENT_PLL4: u8 = 3;
const SUN9I_CPUS_MUX_SHIFT: u32 = 16;
const SUN9I_CPUS_MUX_MASK: u32 = (1 << 17) | (1 << 16);
#[inline]
fn sun9i_cpus_mux_get_parent(reg: u32) -> u8 {
    ((reg & SUN9I_CPUS_MUX_MASK) >> SUN9I_CPUS_MUX_SHIFT) as u8
}

const SUN9I_CPUS_DIV_SHIFT: u32 = 4;
const SUN9I_CPUS_DIV_MASK: u32 = (1 << 5) | (1 << 4);
#[inline]
fn sun9i_cpus_div_get(reg: u32) -> u8 {
    ((reg & SUN9I_CPUS_DIV_MASK) >> SUN9I_CPUS_DIV_SHIFT) as u8
}
#[inline]
fn sun9i_cpus_div_set(reg: u32, div: u8) -> u32 {
    (reg & !SUN9I_CPUS_DIV_MASK) | ((div as u32) << SUN9I_CPUS_DIV_SHIFT)
}

const SUN9I_CPUS_PLL4_DIV_SHIFT: u32 = 8;
const SUN9I_CPUS_PLL4_DIV_MASK: u32 = (1 << 12) | (1 << 11) | (1 << 10) | (1 << 9) | (1 << 8);
#[inline]
fn sun9i_cpus_pll4_div_get(reg: u32) -> u8 {
    ((reg & SUN9I_CPUS_PLL4_DIV_MASK) >> SUN9I_CPUS_PLL4_DIV_SHIFT) as u8
}
#[inline]
fn sun9i_cpus_pll4_div_set(reg: u32, div: u8) -> u32 {
    (reg & !SUN9I_CPUS_PLL4_DIV_MASK) | ((div as u32) << SUN9I_CPUS_PLL4_DIV_SHIFT)
}

#[repr(C)]
struct sun9i_a80_cpus_clk {
    hw: clk_hw,
    reg: *mut core::ffi::c_void,
}

unsafe fn to_sun9i_a80_cpus_clk(hw: *mut clk_hw) -> *mut sun9i_a80_cpus_clk {
    hw.cast()
}

unsafe fn sun9i_a80_cpus_clk_recalc_rate(hw: *mut clk_hw, mut parent_rate: usize) -> usize {
    let cpus = &*to_sun9i_a80_cpus_clk(hw);
    let reg = readl(cpus.reg);

    if sun9i_cpus_mux_get_parent(reg) == SUN9I_CPUS_MUX_PARENT_PLL4 {
        parent_rate /= sun9i_cpus_pll4_div_get(reg) as usize + 1;
    }
    parent_rate / (sun9i_cpus_div_get(reg) as usize + 1)
}

unsafe fn sun9i_a80_cpus_clk_round(
    mut rate: usize, divp: *mut u8, pre_divp: *mut u8, parent: u8, parent_rate: usize,
) -> isize {
    let mut div: u8;
    let mut pre_div: u8 = 1;
    if parent_rate != 0 && rate > parent_rate { rate = parent_rate; }
    div = ((parent_rate + rate - 1) / rate) as u8;
    if parent == SUN9I_CPUS_MUX_PARENT_PLL4 && div > 4 {
        if div < 32 { pre_div = div; div = 1; }
        else if div < 64 { pre_div = ((div as usize + 1) / 2) as u8; div = 2; }
        else if div < 96 { pre_div = ((div as usize + 2) / 3) as u8; div = 3; }
        else { pre_div = ((div as usize + 3) / 4) as u8; div = 4; }
    }
    if !divp.is_null() {
        *divp = div - 1;
        *pre_divp = pre_div - 1;
    }
    (parent_rate / pre_div as usize / div as usize) as isize
}

unsafe fn sun9i_a80_cpus_clk_determine_rate(clk: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let mut best_parent: *mut clk_hw = core::ptr::null_mut();
    let mut best = 0usize;
    let mut best_child_rate = 0usize;
    let rate = (*req).rate;
    let num_parents = clk_hw_get_num_parents(clk);
    for i in 0..num_parents {
        let parent = clk_hw_get_parent_by_index(clk, i);
        if parent.is_null() { continue; }
        let parent_rate = if clk_hw_get_flags(clk) & CLK_SET_RATE_PARENT != 0 {
            clk_hw_round_rate(parent, rate)
        } else { clk_hw_get_rate(parent) };
        let child_rate = sun9i_a80_cpus_clk_round(rate, core::ptr::null_mut(), core::ptr::null_mut(), i as u8, parent_rate) as usize;
        if child_rate <= rate && child_rate > best_child_rate {
            best_parent = parent; best = parent_rate; best_child_rate = child_rate;
        }
    }
    if best_parent.is_null() { return -22; }
    (*req).best_parent_hw = best_parent;
    (*req).best_parent_rate = best;
    (*req).rate = best_child_rate;
    0
}

unsafe fn sun9i_a80_cpus_clk_set_rate(hw: *mut clk_hw, rate: usize, parent_rate: usize) -> i32 {
    let cpus = &*to_sun9i_a80_cpus_clk(hw);
    let reg = readl(cpus.reg);
    let parent = sun9i_cpus_mux_get_parent(reg);
    let mut div = 0u8;
    let mut pre_div = 0u8;
    sun9i_a80_cpus_clk_round(rate, &mut div, &mut pre_div, parent, parent_rate);
    let reg = sun9i_cpus_pll4_div_set(sun9i_cpus_div_set(reg, div), pre_div);
    writel(reg, cpus.reg);
    0
}

// The setup routine and clock registration descriptor are retained below;
// their Linux framework types and functions are external dependencies.
unsafe fn sun9i_a80_cpus_setup(node: *mut device_node) {
    let mut clk_name = (*node).name;
    let mut parents: [*const core::ffi::c_char; SUN9I_CPUS_MAX_PARENTS] = [core::ptr::null(); SUN9I_CPUS_MAX_PARENTS];
    let mut res = core::mem::MaybeUninit::<resource>::uninit();
    let cpus = kzalloc_obj::<sun9i_a80_cpus_clk>();
    if cpus.is_null() { return; }
    (*cpus).reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if is_err((*cpus).reg) { kfree(cpus); return; }
    of_property_read_string(node, b"clock-output-names\0".as_ptr().cast(), &mut clk_name);
    let ret = of_clk_parent_fill(node, parents.as_mut_ptr(), SUN9I_CPUS_MAX_PARENTS);
    let mux = kzalloc_obj::<clk_mux>();
    if mux.is_null() { iounmap((*cpus).reg); kfree(cpus); return; }
    (*mux).reg = (*cpus).reg;
    (*mux).shift = SUN9I_CPUS_MUX_SHIFT as u8;
    (*mux).mask = (SUN9I_CPUS_MUX_MASK >> SUN9I_CPUS_MUX_SHIFT) as u32;
    (*mux).lock = &raw mut sun9i_a80_cpus_lock;
    let clk = clk_register_composite(core::ptr::null_mut(), clk_name, parents.as_ptr(), ret, &mut (*mux).hw, &clk_mux_ops, &mut (*cpus).hw, &sun9i_a80_cpus_clk_ops, core::ptr::null(), core::ptr::null(), 0);
    if is_err(clk) { kfree(mux); iounmap((*cpus).reg); kfree(cpus); return; }
    if of_clk_add_provider(node, of_clk_src_simple_get, clk) != 0 { clk_unregister(clk); kfree(mux); iounmap((*cpus).reg); of_address_to_resource(node, 0, res.as_mut_ptr()); release_mem_region((*res.as_ptr()).start, resource_size(res.as_ptr())); kfree(cpus); }
}

// CLK_OF_DECLARE(sun9i_a80_cpus, "allwinner,sun9i-a80-cpus-clk", sun9i_a80_cpus_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
