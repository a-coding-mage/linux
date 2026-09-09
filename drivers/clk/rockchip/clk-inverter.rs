// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015 Heiko Stuebner <heiko@sntech.de>
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/slab.h, linux/clk-provider.h, linux/io.h, linux/spinlock.h,
// linux/kernel.h, and clk.h.

use core::ffi::{c_char, c_int, c_void};

const INVERTER_MASK: u32 = 0x1;

#[repr(C)]
pub struct rockchip_inv_clock {
    pub hw: clk_hw,
    pub reg: *mut c_void,
    pub shift: c_int,
    pub flags: c_int,
    pub lock: *mut spinlock_t,
}

// The following kernel types and functions are provided by the translated dependencies.
#[repr(C)]
pub struct clk_hw {
    pub init: *mut clk_init_data,
}
#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub num_parents: u8,
    pub flags: u32,
    pub parent_names: *const *const c_char,
    pub ops: *const clk_ops,
}
#[repr(C)]
pub struct clk_ops {
    pub get_phase: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub set_phase: Option<unsafe extern "C" fn(*mut clk_hw, c_int) -> c_int>,
}
#[repr(C)]
pub struct clk;
#[repr(C)]
pub struct spinlock_t;

extern "C" {
    fn readl(reg: *mut c_void) -> u32;
    fn writel(value: u32, reg: *mut c_void);
    fn pr_err(fmt: *const c_char, ...);
    fn clk_hw_get_name(hw: *mut clk_hw) -> *const c_char;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn clk_register(dev: *mut c_void, hw: *mut clk_hw) -> *mut clk;
    fn kfree(ptr: *mut c_void);
}

extern "C" {
    static rockchip_inv_clk_ops: clk_ops;
}

const ROCKCHIP_INVERTER_HIWORD_MASK: c_int = 1 << 0;
const CLK_SET_RATE_PARENT: u32 = 1 << 2;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[inline]
unsafe fn to_inv_clock(hw: *mut clk_hw) -> *mut rockchip_inv_clock {
    // `hw` is the first field of rockchip_inv_clock, matching container_of.
    hw as *mut rockchip_inv_clock
}

unsafe extern "C" fn rockchip_inv_get_phase(hw: *mut clk_hw) -> c_int {
    let inv_clock = &*to_inv_clock(hw);
    let mut val = readl(inv_clock.reg) >> inv_clock.shift;
    val &= INVERTER_MASK;
    if val != 0 { 180 } else { 0 }
}

unsafe extern "C" fn rockchip_inv_set_phase(hw: *mut clk_hw, degrees: c_int) -> c_int {
    let inv_clock = &*to_inv_clock(hw);
    let val: u32;

    if degrees % 180 == 0 {
        val = if degrees != 0 { 1 } else { 0 };
    } else {
        // pr_err("%s: unsupported phase %d for %s\n", __func__, degrees, clk_hw_get_name(hw));
        return -EINVAL;
    }

    if inv_clock.flags & ROCKCHIP_INVERTER_HIWORD_MASK != 0 {
        // HIWORD_UPDATE(val, INVERTER_MASK, inv_clock->shift)
        let update = (val << inv_clock.shift) | (INVERTER_MASK << (inv_clock.shift + 16));
        writel(update, inv_clock.reg);
    } else {
        let mut flags: usize = 0;
        let mut reg: u32;

        spin_lock_irqsave(inv_clock.lock, &mut flags);
        reg = readl(inv_clock.reg);
        reg &= !(1u32 << inv_clock.shift);
        reg |= val;
        writel(reg, inv_clock.reg);
        spin_unlock_irqrestore(inv_clock.lock, flags);
    }

    0
}

static ROCKCHIP_INV_CLK_OPS: clk_ops = clk_ops {
    get_phase: Some(rockchip_inv_get_phase),
    set_phase: Some(rockchip_inv_set_phase),
};

pub unsafe extern "C" fn rockchip_clk_register_inverter(
    name: *const c_char,
    parent_names: *const *const c_char,
    num_parents: u8,
    reg: *mut c_void,
    shift: c_int,
    flags: c_int,
    lock: *mut spinlock_t,
) -> *mut clk {
    let inv_clock = libc::malloc(core::mem::size_of::<rockchip_inv_clock>()) as *mut rockchip_inv_clock;
    if inv_clock.is_null() {
        return (-ENOMEM as isize) as *mut clk;
    }

    let mut init = clk_init_data {
        name,
        num_parents,
        flags: CLK_SET_RATE_PARENT,
        parent_names,
        ops: &ROCKCHIP_INV_CLK_OPS,
    };

    (*inv_clock).hw.init = &mut init;
    (*inv_clock).reg = reg;
    (*inv_clock).shift = shift;
    (*inv_clock).flags = flags;
    (*inv_clock).lock = lock;

    let clk = clk_register(core::ptr::null_mut(), &mut (*inv_clock).hw);
    if (clk as isize) < 0 {
        kfree(inv_clock as *mut c_void);
    }
    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
