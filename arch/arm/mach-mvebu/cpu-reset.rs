// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2014 Marvell
 *
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 */

// #define pr_fmt(fmt) "mvebu-cpureset: " fmt
// Dependencies supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: c_ulong,
    pub end: c_ulong,
    pub name: *const c_char,
    pub flags: c_ulong,
    pub desc: c_ulong,
}

extern "C" {
    fn of_address_to_resource(
        np: *mut device_node,
        index: c_int,
        resource: *mut resource,
    ) -> c_int;
    fn request_mem_region(start: c_ulong, size: c_ulong, name: *const c_char) -> *mut resource;
    fn release_mem_region(start: c_ulong, size: c_ulong);
    fn ioremap(offset: c_ulong, size: c_ulong) -> *mut c_void;
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
    fn of_node_put(np: *mut device_node);
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
}

static mut CPU_RESET_BASE: *mut c_void = core::ptr::null_mut();
static mut CPU_RESET_SIZE: usize = 0;

#[inline]
const fn cpu_reset_offset(cpu: usize) -> usize {
    cpu.wrapping_mul(0x8)
}

const CPU_RESET_ASSERT: u32 = 1 << 0;

pub unsafe fn mvebu_cpu_reset_deassert(cpu: c_int) -> c_int {
    let mut reg: u32;

    if CPU_RESET_BASE.is_null() {
        return -19; // -ENODEV
    }

    if cpu_reset_offset(cpu as usize) >= CPU_RESET_SIZE {
        return -22; // -EINVAL
    }

    let offset = cpu_reset_offset(cpu as usize);
    reg = readl(CPU_RESET_BASE.add(offset));
    reg &= !CPU_RESET_ASSERT;
    writel(reg, CPU_RESET_BASE.add(offset));

    0
}

unsafe fn mvebu_cpu_reset_map(np: *mut device_node, res_idx: c_int) -> c_int {
    let mut res: resource = core::mem::zeroed();

    if of_address_to_resource(np, res_idx, &mut res) != 0 {
        // pr_err("unable to get resource\n");
        return -2; // -ENOENT
    }

    let size = res.end.wrapping_sub(res.start).wrapping_add(1);
    if request_mem_region(res.start, size, res.name).is_null() {
        // pr_err("unable to request region\n");
        return -16; // -EBUSY
    }

    CPU_RESET_BASE = ioremap(res.start, size);
    if CPU_RESET_BASE.is_null() {
        // pr_err("unable to map registers\n");
        release_mem_region(res.start, size);
        return -12; // -ENOMEM
    }

    CPU_RESET_SIZE = size as usize;

    0
}

unsafe fn mvebu_cpu_reset_init() -> c_int {
    let np: *mut device_node;
    let res_idx: c_int;
    let ret: c_int;

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        c"marvell,armada-370-cpu-reset".as_ptr(),
    );
    if !np.is_null() {
        res_idx = 0;
    } else {
        /*
         * This code is kept for backward compatibility with
         * old Device Trees.
         */
        np = of_find_compatible_node(
            core::ptr::null_mut(),
            core::ptr::null(),
            c"marvell,armada-370-xp-pmsu".as_ptr(),
        );
        if !np.is_null() {
            // pr_warn(FW_WARN "deprecated pmsu binding\n");
            res_idx = 1;
        } else {
            res_idx = 0;
        }
    }

    /* No reset node found */
    if np.is_null() {
        return -19; // -ENODEV
    }

    ret = mvebu_cpu_reset_map(np, res_idx);
    of_node_put(np);

    ret
}

// early_initcall(mvebu_cpu_reset_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
