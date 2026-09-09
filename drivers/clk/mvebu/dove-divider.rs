// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell Dove PMU Core PLL divider driver
 *
 * Cleaned up by substantially rewriting, and converted to DT by
 * Russell King.  Origin is not known.
 */

use core::ffi::c_void;

// Linux kernel dependencies supplied by other translation units.
use crate::{
    clk_hw, clk_init_data, clk_ops, clk_rate_request, clk, device, spinlock_t,
    ARRAY_SIZE, BIT, DIV_ROUND_CLOSEST, EINVAL, IS_ERR, PTR_ERR,
    clk_register, clk_register_fixed_rate, ndelay, pr_debug, readl_relaxed,
    spin_lock, spin_unlock, strscpy, writel_relaxed,
};

#[repr(C)]
pub struct dove_clk {
    pub name: *const i8,
    pub hw: clk_hw,
    pub base: *mut c_void,
    pub lock: *mut spinlock_t,
    pub div_bit_start: u8,
    pub div_bit_end: u8,
    pub div_bit_load: u8,
    pub div_bit_size: u8,
    pub divider_table: *mut u32,
}

pub const DIV_CTRL0: usize = 0;
pub const DIV_CTRL1: usize = 4;
pub const DIV_CTRL1_N_RESET_MASK: u32 = BIT(10);

#[inline]
unsafe fn to_dove_clk(hw: *mut clk_hw) -> *mut dove_clk {
    (hw as *mut u8).sub(core::mem::offset_of!(dove_clk, hw)) as *mut dove_clk
}

unsafe fn dove_load_divider(base: *mut c_void, val: u32, mask: u32, load: u32) {
    let mut v: u32;

    v = readl_relaxed((base as *mut u8).add(DIV_CTRL1) as *const c_void)
        | DIV_CTRL1_N_RESET_MASK;
    writel_relaxed(v, (base as *mut u8).add(DIV_CTRL1) as *mut c_void);

    v = (readl_relaxed((base as *mut u8).add(DIV_CTRL0) as *const c_void)
        & !(mask | load))
        | val;
    writel_relaxed(v, (base as *mut u8).add(DIV_CTRL0) as *mut c_void);
    writel_relaxed(v | load, (base as *mut u8).add(DIV_CTRL0) as *mut c_void);
    ndelay(250);
    writel_relaxed(v, (base as *mut u8).add(DIV_CTRL0) as *mut c_void);
}

unsafe fn dove_get_divider(dc: *mut dove_clk) -> u32 {
    let mut val = readl_relaxed((*dc).base as *mut u8 as *const c_void
        .add(DIV_CTRL0));
    val >>= (*dc).div_bit_start;

    let mut divider = val & !(!0u32 << (*dc).div_bit_size);

    if !(*dc).divider_table.is_null() {
        divider = *(*dc).divider_table.add(divider as usize);
    }

    divider
}

unsafe fn dove_calc_divider(
    dc: *const dove_clk,
    rate: u64,
    parent_rate: u64,
    set: bool,
) -> i32 {
    let mut divider = DIV_ROUND_CLOSEST(parent_rate, rate) as u32;
    let max: u32;

    if !(*dc).divider_table.is_null() {
        let mut i = 0usize;
        while *(*dc).divider_table.add(i) != 0 {
            if divider == *(*dc).divider_table.add(i) {
                divider = i as u32;
                break;
            }
            i += 1;
        }

        if *(*dc).divider_table.add(i) == 0 {
            return EINVAL;
        }
    } else {
        max = 1u32 << (*dc).div_bit_size;

        if set && (divider == 0 || divider >= max) {
            return EINVAL;
        }
        if divider >= max {
            divider = max - 1;
        } else if divider == 0 {
            divider = 1;
        }
    }

    divider as i32
}

unsafe extern "C" fn dove_recalc_rate(hw: *mut clk_hw, parent: u64) -> u64 {
    let dc = to_dove_clk(hw);
    let divider = dove_get_divider(dc);
    let rate = DIV_ROUND_CLOSEST(parent, divider as u64);

    pr_debug!("dove_recalc_rate(): %s divider=%u parent=%lu rate=%lu\n",
        (*dc).name, divider, parent, rate);

    rate
}

unsafe extern "C" fn dove_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> i32 {
    let dc = to_dove_clk(hw);
    let parent_rate = (*req).best_parent_rate;
    let divider = dove_calc_divider(dc, (*req).rate, parent_rate, false);
    if divider < 0 {
        return divider;
    }

    (*req).rate = DIV_ROUND_CLOSEST(parent_rate, divider as u64);
    pr_debug!("dove_determine_rate(): %s divider=%u parent=%lu rate=%lu\n",
        (*dc).name, divider, parent_rate, (*req).rate);
    0
}

unsafe extern "C" fn dove_set_clock(
    hw: *mut clk_hw,
    rate: u64,
    parent_rate: u64,
) -> i32 {
    let dc = to_dove_clk(hw);
    let divider = dove_calc_divider(dc, rate, parent_rate, true);
    if divider < 0 {
        return divider;
    }

    pr_debug!("dove_set_clock(): %s divider=%u parent=%lu rate=%lu\n",
        (*dc).name, divider, parent_rate, rate);

    let div = (divider as u32) << (*dc).div_bit_start;
    let mask = !(!0u32 << (*dc).div_bit_size) << (*dc).div_bit_start;
    let load = BIT((*dc).div_bit_load);

    spin_lock((*dc).lock);
    dove_load_divider((*dc).base, div, mask, load);
    spin_unlock((*dc).lock);
    0
}

#[repr(C)]
pub static dove_divider_ops: clk_ops = clk_ops {
    set_rate: Some(dove_set_clock),
    determine_rate: Some(dove_determine_rate),
    recalc_rate: Some(dove_recalc_rate),
};

unsafe fn clk_register_dove_divider(
    dev: *mut device,
    dc: *mut dove_clk,
    parent_names: *const *const i8,
    num_parents: usize,
    base: *mut c_void,
) -> *mut clk {
    let mut name = [0i8; 32];
    let mut init = clk_init_data {
        name: name.as_mut_ptr(),
        ops: &dove_divider_ops,
        parent_names,
        num_parents,
    };

    strscpy(name.as_mut_ptr(), (*dc).name, name.len());
    (*dc).hw.init = &mut init;
    (*dc).base = base;
    (*dc).div_bit_size = (*dc).div_bit_end - (*dc).div_bit_start + 1;

    clk_register(dev, &mut (*dc).hw)
}

static mut dove_divider_lock: spinlock_t = spinlock_t::new();

static mut axi_divider: [u32; 12] = [u32::MAX, 2, 1, 3, 4, 6, 5, 7, 8, 10, 9, 0];

pub static mut dove_hw_clocks: [dove_clk; 4] = [
    dove_clk { name: b"axi\0".as_ptr() as *const i8, hw: clk_hw::default(), base: core::ptr::null_mut(), lock: unsafe { &raw mut dove_divider_lock }, div_bit_start: 1, div_bit_end: 6, div_bit_load: 7, div_bit_size: 0, divider_table: unsafe { &raw mut axi_divider[0] } },
    dove_clk { name: b"gpu\0".as_ptr() as *const i8, hw: clk_hw::default(), base: core::ptr::null_mut(), lock: unsafe { &raw mut dove_divider_lock }, div_bit_start: 8, div_bit_end: 13, div_bit_load: 14, div_bit_size: 0, divider_table: core::ptr::null_mut() },
    dove_clk { name: b"vmeta\0".as_ptr() as *const i8, hw: clk_hw::default(), base: core::ptr::null_mut(), lock: unsafe { &raw mut dove_divider_lock }, div_bit_start: 15, div_bit_end: 20, div_bit_load: 21, div_bit_size: 0, divider_table: core::ptr::null_mut() },
    dove_clk { name: b"lcd\0".as_ptr() as *const i8, hw: clk_hw::default(), base: core::ptr::null_mut(), lock: unsafe { &raw mut dove_divider_lock }, div_bit_start: 22, div_bit_end: 27, div_bit_load: 28, div_bit_size: 0, divider_table: core::ptr::null_mut() },
];

static core_pll: [*const i8; 1] = [b"core-pll\0".as_ptr() as *const i8];

pub unsafe fn dove_divider_init(
    dev: *mut device,
    base: *mut c_void,
    clks: *mut *mut clk,
) -> i32 {
    /*
     * Create the core PLL clock.  We treat this as a fixed rate
     * clock as we don't know any better, and documentation is sparse.
     */
    let clk = clk_register_fixed_rate(dev, core_pll[0], core::ptr::null(), 0, 2000000000);
    if IS_ERR(clk) {
        return PTR_ERR(clk);
    }

    let mut i = 0usize;
    while i < ARRAY_SIZE(&dove_hw_clocks) {
        *clks.add(i) = clk_register_dove_divider(
            dev,
            &raw mut dove_hw_clocks[i],
            core_pll.as_ptr(),
            ARRAY_SIZE(&core_pll),
            base,
        );
        i += 1;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
