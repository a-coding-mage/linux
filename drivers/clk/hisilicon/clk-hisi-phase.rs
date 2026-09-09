// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2017 HiSilicon Technologies Co., Ltd.
 *
 * Simple HiSilicon phase clock implementation.
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/err.h, linux/io.h, linux/module.h, linux/platform_device.h,
// linux/slab.h, and "clk.h".

use core::ffi::c_void;

#[repr(C)]
struct clk_hisi_phase {
    hw: clk_hw,
    reg: *mut c_void,
    phase_degrees: *mut u32,
    phase_regvals: *mut u32,
    phase_num: u8,
    mask: u32,
    shift: u8,
    flags: u8,
    lock: *mut spinlock_t,
}

// External kernel types and functions are provided by the translated dependencies.
#[repr(C)] struct clk_hw { init: *mut clk_init_data }
#[repr(C)] struct clk_init_data {
    name: *const core::ffi::c_char,
    ops: *const clk_ops,
    flags: u32,
    parent_names: *const *const core::ffi::c_char,
    num_parents: u8,
}
#[repr(C)] struct clk_ops {
    get_phase: Option<unsafe extern "C" fn(*mut clk_hw) -> i32>,
    set_phase: Option<unsafe extern "C" fn(*mut clk_hw, i32) -> i32>,
}
#[repr(C)] struct device { _private: [u8; 0] }
#[repr(C)] struct spinlock_t { _private: [u8; 0] }
#[repr(C)] struct hisi_phase_clock {
    name: *const core::ffi::c_char,
    flags: u32,
    parent_names: *const core::ffi::c_char,
    offset: usize,
    shift: u8,
    width: u8,
    phase_degrees: *mut u32,
    phase_regvals: *mut u32,
    phase_num: u8,
}

extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: u32) -> *mut c_void;
    fn devm_clk_register(dev: *mut device, hw: *mut clk_hw) -> *mut clk;
}

#[repr(C)] struct clk { _private: [u8; 0] }

const GFP_KERNEL: u32 = 0;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;

unsafe fn to_clk_hisi_phase(hw: *mut clk_hw) -> *mut clk_hisi_phase {
    hw as *mut clk_hisi_phase
}

unsafe fn hisi_phase_regval_to_degrees(phase: *mut clk_hisi_phase, regval: u32) -> i32 {
    let mut i: u8 = 0;
    while i < (*phase).phase_num {
        if *(*phase).phase_regvals.add(i as usize) == regval {
            return *(*phase).phase_degrees.add(i as usize) as i32;
        }
        i = i.wrapping_add(1);
    }
    -EINVAL
}

unsafe extern "C" fn hisi_clk_get_phase(hw: *mut clk_hw) -> i32 {
    let phase = to_clk_hisi_phase(hw);
    let mut regval = readl((*phase).reg);
    regval = (regval & (*phase).mask) >> (*phase).shift;
    hisi_phase_regval_to_degrees(phase, regval)
}

unsafe fn hisi_phase_degrees_to_regval(phase: *mut clk_hisi_phase, degrees: i32) -> i32 {
    let mut i: u8 = 0;
    while i < (*phase).phase_num {
        if *(*phase).phase_degrees.add(i as usize) as i32 == degrees {
            return *(*phase).phase_regvals.add(i as usize) as i32;
        }
        i = i.wrapping_add(1);
    }
    -EINVAL
}

unsafe extern "C" fn hisi_clk_set_phase(hw: *mut clk_hw, degrees: i32) -> i32 {
    let phase = to_clk_hisi_phase(hw);
    let mut flags: usize = 0;
    let regval = hisi_phase_degrees_to_regval(phase, degrees);
    if regval < 0 { return regval; }

    spin_lock_irqsave((*phase).lock, &mut flags);
    let mut val = readl((*phase).reg);
    val &= !(*phase).mask;
    val |= (regval as u32) << (*phase).shift;
    writel(val, (*phase).reg);
    spin_unlock_irqrestore((*phase).lock, flags);
    0
}

static clk_phase_ops: clk_ops = clk_ops {
    get_phase: Some(hisi_clk_get_phase),
    set_phase: Some(hisi_clk_set_phase),
};

#[no_mangle]
pub unsafe extern "C" fn clk_register_hisi_phase(
    dev: *mut device,
    clks: *const hisi_phase_clock,
    base: *mut c_void,
    lock: *mut spinlock_t,
) -> *mut clk {
    let phase = devm_kzalloc(dev, core::mem::size_of::<clk_hisi_phase>(), GFP_KERNEL)
        as *mut clk_hisi_phase;
    if phase.is_null() { return (-ENOMEM) as isize as *mut clk; }

    let clks = &*clks;
    let mut init = clk_init_data {
        name: clks.name,
        ops: &clk_phase_ops,
        flags: clks.flags,
        parent_names: if !clks.parent_names.is_null() { &clks.parent_names } else { core::ptr::null() },
        num_parents: if !clks.parent_names.is_null() { 1 } else { 0 },
    };

    (*phase).reg = base.add(clks.offset);
    (*phase).shift = clks.shift;
    (*phase).mask = (u32::wrapping_shl(1, clks.width as u32).wrapping_sub(1)) << clks.shift;
    (*phase).lock = lock;
    (*phase).phase_degrees = clks.phase_degrees;
    (*phase).phase_regvals = clks.phase_regvals;
    (*phase).phase_num = clks.phase_num;
    (*phase).hw.init = &mut init;
    devm_clk_register(dev, &mut (*phase).hw)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
