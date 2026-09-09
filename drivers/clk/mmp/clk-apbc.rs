// SPDX-License-Identifier: GPL-2.0-only
/*
 * mmp APB clock operation source file
 *
 * Copyright (C) 2012 Marvell
 * Chao Xie <xiechao.mail@gmail.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation.

const APBC_APBCLK: u32 = 1 << 0; // APB Bus Clock Enable
const APBC_FNCLK: u32 = 1 << 1; // Functional Clock Enable
const APBC_RST: u32 = 1 << 2; // Reset Generation
const APBC_POWER: u32 = 1 << 7; // Reset Generation

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const core::ffi::c_char,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub parent_names: *const *const core::ffi::c_char,
    pub num_parents: u8,
}

#[repr(C)]
pub struct clk_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    pub unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

extern "C" {
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn udelay(usecs: u32);
    fn kzalloc(size: usize) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn clk_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> *mut clk;
    fn is_err(ptr: *mut clk) -> bool;
}

// APBC_POWER_CTRL, APBC_NO_BUS_CTRL, and CLK_SET_RATE_PARENT are supplied by clk.h.
extern "C" {
    static APBC_POWER_CTRL: u32;
    static APBC_NO_BUS_CTRL: u32;
    static CLK_SET_RATE_PARENT: u32;
}

#[repr(C)]
pub struct clk_apbc {
    pub hw: clk_hw,
    pub base: *mut core::ffi::c_void,
    pub delay: u32,
    pub flags: u32,
    pub lock: *mut spinlock_t,
}

unsafe fn to_clk_apbc(hw: *mut clk_hw) -> *mut clk_apbc {
    hw as *mut clk_apbc
}

unsafe extern "C" fn clk_apbc_prepare(hw: *mut clk_hw) -> i32 {
    let apbc = &mut *to_clk_apbc(hw);
    let mut data: u32;
    let mut flags: usize = 0;

    if !apbc.lock.is_null() {
        spin_lock_irqsave(apbc.lock, &mut flags);
    }

    data = readl_relaxed(apbc.base);
    if apbc.flags & APBC_POWER_CTRL != 0 {
        data |= APBC_POWER;
    }
    data |= APBC_FNCLK;
    writel_relaxed(data, apbc.base);

    if !apbc.lock.is_null() {
        spin_unlock_irqrestore(apbc.lock, flags);
    }

    udelay(apbc.delay);

    if !apbc.lock.is_null() {
        spin_lock_irqsave(apbc.lock, &mut flags);
    }

    data = readl_relaxed(apbc.base);
    data |= APBC_APBCLK;
    writel_relaxed(data, apbc.base);

    if !apbc.lock.is_null() {
        spin_unlock_irqrestore(apbc.lock, flags);
    }

    udelay(apbc.delay);

    if apbc.flags & APBC_NO_BUS_CTRL == 0 {
        if !apbc.lock.is_null() {
            spin_lock_irqsave(apbc.lock, &mut flags);
        }

        data = readl_relaxed(apbc.base);
        data &= !APBC_RST;
        writel_relaxed(data, apbc.base);

        if !apbc.lock.is_null() {
            spin_unlock_irqrestore(apbc.lock, flags);
        }
    }

    0
}

unsafe extern "C" fn clk_apbc_unprepare(hw: *mut clk_hw) {
    let apbc = &mut *to_clk_apbc(hw);
    let mut data: usize;
    let mut flags: usize = 0;

    if !apbc.lock.is_null() {
        spin_lock_irqsave(apbc.lock, &mut flags);
    }

    data = readl_relaxed(apbc.base) as usize;
    if apbc.flags & APBC_POWER_CTRL != 0 {
        data &= !(APBC_POWER as usize);
    }
    data &= !(APBC_FNCLK as usize);
    writel_relaxed(data as u32, apbc.base);

    if !apbc.lock.is_null() {
        spin_unlock_irqrestore(apbc.lock, flags);
    }

    udelay(10);

    if !apbc.lock.is_null() {
        spin_lock_irqsave(apbc.lock, &mut flags);
    }

    data = readl_relaxed(apbc.base) as usize;
    data &= !(APBC_APBCLK as usize);
    writel_relaxed(data as u32, apbc.base);

    if !apbc.lock.is_null() {
        spin_unlock_irqrestore(apbc.lock, flags);
    }
}

static clk_apbc_ops: clk_ops = clk_ops {
    prepare: Some(clk_apbc_prepare),
    unprepare: Some(clk_apbc_unprepare),
};

pub unsafe extern "C" fn mmp_clk_register_apbc(
    name: *const core::ffi::c_char,
    parent_name: *const core::ffi::c_char,
    base: *mut core::ffi::c_void,
    delay: u32,
    apbc_flags: u32,
    lock: *mut spinlock_t,
) -> *mut clk {
    let apbc = kzalloc(core::mem::size_of::<clk_apbc>()) as *mut clk_apbc;
    if apbc.is_null() {
        return core::ptr::null_mut();
    }

    let mut init = clk_init_data {
        name,
        ops: &clk_apbc_ops,
        flags: CLK_SET_RATE_PARENT,
        parent_names: if !parent_name.is_null() {
            &parent_name
        } else {
            core::ptr::null()
        },
        num_parents: if !parent_name.is_null() { 1 } else { 0 },
    };

    (*apbc).base = base;
    (*apbc).delay = delay;
    (*apbc).flags = apbc_flags;
    (*apbc).lock = lock;
    // The C initializer stores the address of init in hw.init; the surrounding
    // clk_hw definition supplies that field.
    let clk = clk_register(core::ptr::null_mut(), &mut (*apbc).hw);
    if is_err(clk) {
        kfree(apbc as *mut core::ffi::c_void);
    }

    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
