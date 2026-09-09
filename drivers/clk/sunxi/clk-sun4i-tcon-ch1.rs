// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// C dependencies: <linux/clk-provider.h>, <linux/io.h>, <linux/of.h>,
// <linux/of_address.h>, <linux/slab.h>, and <linux/spinlock.h>.

const TCON_CH1_SCLK2_PARENTS: usize = 4;

const TCON_CH1_SCLK2_GATE_BIT: u32 = 1u32 << 31;
const TCON_CH1_SCLK2_MUX_MASK: u32 = 3;
const TCON_CH1_SCLK2_MUX_SHIFT: u32 = 24;
const TCON_CH1_SCLK2_DIV_MASK: u32 = 0xf;
const TCON_CH1_SCLK2_DIV_SHIFT: u32 = 0;

const TCON_CH1_SCLK1_GATE_BIT: u32 = 1u32 << 15;
const TCON_CH1_SCLK1_HALF_BIT: u32 = 1u32 << 11;

#[repr(C)]
struct TconCh1Clk {
    hw: ClkHw,
    lock: Spinlock,
    reg: *mut core::ffi::c_void,
}

// External kernel types and functions supplied by other translation units.
#[repr(C)] struct ClkHw { _private: [u8; 0] }
#[repr(C)] struct Spinlock { _private: [u8; 0] }
#[repr(C)] struct ClkRateRequest {
    rate: usize,
    best_parent_rate: usize,
    best_parent_hw: *mut ClkHw,
}
#[repr(C)] struct DeviceNode { name: *const core::ffi::c_char }
#[repr(C)] struct ClkInitData {
    name: *const core::ffi::c_char,
    ops: *const ClkOps,
    parent_names: *const *const core::ffi::c_char,
    num_parents: usize,
    flags: u32,
}
#[repr(C)] struct ClkOps {
    disable: Option<unsafe extern "C" fn(*mut ClkHw)>,
    enable: Option<unsafe extern "C" fn(*mut ClkHw) -> i32>,
    is_enabled: Option<unsafe extern "C" fn(*mut ClkHw) -> i32>,
    get_parent: Option<unsafe extern "C" fn(*mut ClkHw) -> u8>,
    set_parent: Option<unsafe extern "C" fn(*mut ClkHw, u8) -> i32>,
    determine_rate: Option<unsafe extern "C" fn(*mut ClkHw, *mut ClkRateRequest) -> i32>,
    recalc_rate: Option<unsafe extern "C" fn(*mut ClkHw, usize) -> usize>,
    set_rate: Option<unsafe extern "C" fn(*mut ClkHw, usize, usize) -> i32>,
}
#[repr(C)] struct Clk { _private: [u8; 0] }
#[repr(C)] struct Resource { start: usize }

extern "C" {
    fn readl(reg: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, reg: *mut core::ffi::c_void);
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
    fn spin_lock_init(lock: *mut Spinlock);
    fn clk_hw_get_num_parents(hw: *mut ClkHw) -> i32;
    fn clk_hw_get_parent_by_index(hw: *mut ClkHw, index: i32) -> *mut ClkHw;
    fn clk_hw_get_rate(hw: *mut ClkHw) -> usize;
    fn of_property_read_string(node: *mut DeviceNode, name: *const u8, out: *mut *const core::ffi::c_char) -> i32;
    fn of_io_request_and_map(node: *mut DeviceNode, index: i32, name: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    fn of_node_full_name(node: *mut DeviceNode) -> *const core::ffi::c_char;
    fn of_clk_parent_fill(node: *mut DeviceNode, parents: *mut *const core::ffi::c_char, count: usize) -> i32;
    fn kzalloc_obj<T>() -> *mut T;
    fn clk_register(dev: *mut core::ffi::c_void, hw: *mut ClkHw) -> *mut Clk;
    fn of_clk_add_provider(node: *mut DeviceNode, get: *const core::ffi::c_void, clk: *mut Clk) -> i32;
    fn clk_unregister(clk: *mut Clk);
    fn kfree(ptr: *mut TconCh1Clk);
    fn iounmap(reg: *mut core::ffi::c_void);
    fn of_address_to_resource(node: *mut DeviceNode, index: i32, res: *mut Resource) -> i32;
    fn resource_size(res: *const Resource) -> usize;
    fn release_mem_region(start: usize, size: usize);
    fn pr_err(fmt: *const u8, ...);
}

unsafe fn hw_to_tclk(hw: *mut ClkHw) -> *mut TconCh1Clk {
    hw as *mut TconCh1Clk
}

unsafe extern "C" fn tcon_ch1_disable(hw: *mut ClkHw) {
    let tclk = hw_to_tclk(hw);
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*tclk).lock, &mut flags);
    let mut reg = readl((*tclk).reg);
    reg &= !(TCON_CH1_SCLK2_GATE_BIT | TCON_CH1_SCLK1_GATE_BIT);
    writel(reg, (*tclk).reg);
    spin_unlock_irqrestore(&mut (*tclk).lock, flags);
}

unsafe extern "C" fn tcon_ch1_enable(hw: *mut ClkHw) -> i32 {
    let tclk = hw_to_tclk(hw);
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*tclk).lock, &mut flags);
    let mut reg = readl((*tclk).reg);
    reg |= TCON_CH1_SCLK2_GATE_BIT | TCON_CH1_SCLK1_GATE_BIT;
    writel(reg, (*tclk).reg);
    spin_unlock_irqrestore(&mut (*tclk).lock, flags);
    0
}

unsafe extern "C" fn tcon_ch1_is_enabled(hw: *mut ClkHw) -> i32 {
    let tclk = hw_to_tclk(hw);
    readl((*tclk).reg) as i32 & (TCON_CH1_SCLK2_GATE_BIT | TCON_CH1_SCLK1_GATE_BIT) as i32
}

unsafe extern "C" fn tcon_ch1_get_parent(hw: *mut ClkHw) -> u8 {
    let tclk = hw_to_tclk(hw);
    let mut reg = readl((*tclk).reg) >> TCON_CH1_SCLK2_MUX_SHIFT;
    reg &= reg >> TCON_CH1_SCLK2_MUX_MASK;
    reg as u8
}

unsafe extern "C" fn tcon_ch1_set_parent(hw: *mut ClkHw, index: u8) -> i32 {
    let tclk = hw_to_tclk(hw);
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*tclk).lock, &mut flags);
    let mut reg = readl((*tclk).reg);
    reg &= !(TCON_CH1_SCLK2_MUX_MASK << TCON_CH1_SCLK2_MUX_SHIFT);
    reg |= (index as u32) << TCON_CH1_SCLK2_MUX_SHIFT;
    writel(reg, (*tclk).reg);
    spin_unlock_irqrestore(&mut (*tclk).lock, flags);
    0
}

unsafe fn tcon_ch1_calc_divider(rate: usize, parent_rate: usize, div: *mut u8, half: *mut bool) -> usize {
    let mut best_rate = 0usize;
    let mut best_m = 0u8;
    let mut is_double = false;
    for m in 1u8..16 {
        for d in 1usize..3 {
            let tmp_rate = parent_rate / m as usize / d;
            if tmp_rate > rate { continue; }
            if best_rate == 0 || rate - tmp_rate < rate - best_rate {
                best_rate = tmp_rate;
                best_m = m;
                is_double = d != 1;
            }
        }
    }
    if !div.is_null() && !half.is_null() {
        *div = best_m;
        *half = is_double;
    }
    best_rate
}

unsafe extern "C" fn tcon_ch1_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    let mut best_rate: i64 = -22;
    for i in 0..clk_hw_get_num_parents(hw) {
        let parent = clk_hw_get_parent_by_index(hw, i);
        if parent.is_null() { continue; }
        let parent_rate = clk_hw_get_rate(parent);
        let tmp_rate = tcon_ch1_calc_divider((*req).rate, parent_rate, core::ptr::null_mut(), core::ptr::null_mut());
        if best_rate < 0 || ((*req).rate - tmp_rate) < ((*req).rate - best_rate as usize) {
            best_rate = tmp_rate as i64;
            (*req).best_parent_rate = parent_rate;
            (*req).best_parent_hw = parent;
        }
    }
    if best_rate < 0 { return best_rate as i32; }
    (*req).rate = best_rate as usize;
    0
}

unsafe extern "C" fn tcon_ch1_recalc_rate(hw: *mut ClkHw, mut parent_rate: usize) -> usize {
    let tclk = hw_to_tclk(hw);
    let reg = readl((*tclk).reg);
    parent_rate /= (reg & TCON_CH1_SCLK2_DIV_MASK) as usize + 1;
    if reg & TCON_CH1_SCLK1_HALF_BIT != 0 { parent_rate /= 2; }
    parent_rate
}

unsafe extern "C" fn tcon_ch1_set_rate(hw: *mut ClkHw, rate: usize, parent_rate: usize) -> i32 {
    let tclk = hw_to_tclk(hw);
    let mut half = false;
    let mut div_m = 0u8;
    tcon_ch1_calc_divider(rate, parent_rate, &mut div_m, &mut half);
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*tclk).lock, &mut flags);
    let mut reg = readl((*tclk).reg);
    reg &= !(TCON_CH1_SCLK2_DIV_MASK | TCON_CH1_SCLK1_HALF_BIT);
    reg |= (div_m.wrapping_sub(1) as u32) & TCON_CH1_SCLK2_DIV_MASK;
    if half { reg |= TCON_CH1_SCLK1_HALF_BIT; }
    writel(reg, (*tclk).reg);
    spin_unlock_irqrestore(&mut (*tclk).lock, flags);
    0
}

static TCON_CH1_OPS: ClkOps = ClkOps {
    disable: Some(tcon_ch1_disable), enable: Some(tcon_ch1_enable), is_enabled: Some(tcon_ch1_is_enabled),
    get_parent: Some(tcon_ch1_get_parent), set_parent: Some(tcon_ch1_set_parent),
    determine_rate: Some(tcon_ch1_determine_rate), recalc_rate: Some(tcon_ch1_recalc_rate), set_rate: Some(tcon_ch1_set_rate),
};

unsafe extern "C" fn tcon_ch1_setup(node: *mut DeviceNode) {
    let mut parents: [*const core::ffi::c_char; TCON_CH1_SCLK2_PARENTS] = [core::ptr::null(); TCON_CH1_SCLK2_PARENTS];
    let mut clk_name = (*node).name;
    let mut init: ClkInitData = core::mem::zeroed();
    let mut res: Resource = core::mem::zeroed();
    let mut ret: i32;

    of_property_read_string(node, b"clock-output-names\0".as_ptr(), &mut clk_name);
    let reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if reg as isize == -1 {
        pr_err(b"%s: Could not map the clock registers\n\0".as_ptr(), clk_name);
        return;
    }

    ret = of_clk_parent_fill(node, parents.as_mut_ptr(), TCON_CH1_SCLK2_PARENTS);
    if ret != TCON_CH1_SCLK2_PARENTS as i32 {
        pr_err(b"%s Could not retrieve the parents\n\0".as_ptr(), clk_name);
        iounmap(reg);
        of_address_to_resource(node, 0, &mut res);
        release_mem_region(res.start, resource_size(&res));
        return;
    }

    let tclk = kzalloc_obj::<TconCh1Clk>();
    if tclk.is_null() {
        iounmap(reg);
        of_address_to_resource(node, 0, &mut res);
        release_mem_region(res.start, resource_size(&res));
        return;
    }

    init.name = clk_name;
    init.ops = &TCON_CH1_OPS;
    init.parent_names = parents.as_ptr();
    init.num_parents = TCON_CH1_SCLK2_PARENTS;
    init.flags = 1; // CLK_SET_RATE_PARENT

    (*tclk).reg = reg;
    // C: tclk->hw.init = &init;
    spin_lock_init(&mut (*tclk).lock);

    let clk = clk_register(core::ptr::null_mut(), &mut (*tclk).hw);
    if clk as isize == -1 {
        pr_err(b"%s: Couldn't register the clock\n\0".as_ptr(), clk_name);
        kfree(tclk);
        iounmap(reg);
        of_address_to_resource(node, 0, &mut res);
        release_mem_region(res.start, resource_size(&res));
        return;
    }

    ret = of_clk_add_provider(node, core::ptr::null(), clk);
    if ret != 0 {
        pr_err(b"%s: Couldn't register our clock provider\n\0".as_ptr(), clk_name);
        clk_unregister(clk);
        kfree(tclk);
        iounmap(reg);
        of_address_to_resource(node, 0, &mut res);
        release_mem_region(res.start, resource_size(&res));
    }
}

// CLK_OF_DECLARE(tcon_ch1, "allwinner,sun4i-a10-tcon-ch1-clk", tcon_ch1_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
