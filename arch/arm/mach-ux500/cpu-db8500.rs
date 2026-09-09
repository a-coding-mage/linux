// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008-2009 ST-Ericsson SA
 *
 * Author: Srinidhi KASAGAR <srinidhi.kasagar@stericsson.com>
 */

// Linux and architecture dependencies supplied by the surrounding kernel tree.

extern "C" {
    fn of_find_compatible_node(from: *mut device_node, type_: *const core::ffi::c_char,
                                compatible: *const core::ffi::c_char) -> *mut device_node;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut u8;
    fn of_node_put(np: *mut device_node);
    fn writel_relaxed(value: u32, address: *mut u8);
    fn iounmap(address: *mut u8);
    fn irqchip_init();
    fn db8500_prcmu_early_init();
    fn of_address_to_resource(np: *mut device_node, index: i32, resource: *mut resource);
    fn pr_err(message: *const core::ffi::c_char);
    fn ux500_pm_init(start: u64, size: u64);
    fn db8500_prcmu_system_reset(value: u32);
    fn local_irq_disable();
    fn local_fiq_disable();
    fn of_platform_populate(parent: *mut device_node, matches: *const of_device_id,
                            lookup: *const core::ffi::c_void, root: *mut device);
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: u64,
    pub end: u64,
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub name: *const core::ffi::c_char,
    pub type_: *const core::ffi::c_char,
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

#[repr(C)]
pub struct outer_cache_fns {
    pub write_sec: Option<unsafe extern "C" fn(usize, u32)>,
}

extern "C" {
    static mut outer_cache: outer_cache_fns;
}

const L2X0_LOCKDOWN_WAY_D_BASE: usize = 0x900;
const L2X0_LOCKDOWN_WAY_I_BASE: usize = 0x904;
const L2X0_LOCKDOWN_STRIDE: usize = 0x100;

unsafe extern "C" fn ux500_l2x0_unlock() -> i32 {
    let mut i: i32;
    let np: *mut device_node;
    let l2x0_base: *mut u8;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(),
                                 b"arm,pl310-cache\0".as_ptr() as *const _);
    l2x0_base = of_iomap(np, 0);
    of_node_put(np);
    if l2x0_base.is_null() {
        return -19; // -ENODEV
    }

    /* Unlock Data and Instruction Lock if locked. */
    i = 0;
    while i < 8 {
        writel_relaxed(0, l2x0_base.add(L2X0_LOCKDOWN_WAY_D_BASE +
                                        (i as usize) * L2X0_LOCKDOWN_STRIDE));
        writel_relaxed(0, l2x0_base.add(L2X0_LOCKDOWN_WAY_I_BASE +
                                        (i as usize) * L2X0_LOCKDOWN_STRIDE));
        i += 1;
    }
    iounmap(l2x0_base);
    0
}

unsafe extern "C" fn ux500_l2c310_write_sec(_val: usize, _reg: u32) {
    /* Secure registers cannot be written from non-secure mode. */
}

unsafe extern "C" fn ux500_init_irq() {
    let np: *mut device_node;
    let mut r = resource { start: 0, end: 0, _private: [] };

    irqchip_init();
    db8500_prcmu_early_init();
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(),
                                 b"stericsson,db8500-prcmu\0".as_ptr() as *const _);
    of_address_to_resource(np, 0, &mut r);
    of_node_put(np);
    if r.start == 0 {
        pr_err(b"could not find PRCMU base resource\n\0".as_ptr() as *const _);
        return;
    }
    ux500_pm_init(r.start, r.end - r.start);
    ux500_l2x0_unlock();
    outer_cache.write_sec = Some(ux500_l2c310_write_sec);
}

unsafe extern "C" fn ux500_restart(_mode: i32, _cmd: *const core::ffi::c_char) {
    local_irq_disable();
    local_fiq_disable();
    db8500_prcmu_system_reset(0);
}

#[used]
static U8500_LOCAL_BUS_NODES: [of_device_id; 3] = [
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: b"stericsson,db8500\0".as_ptr() as *const _, data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: b"simple-bus\0".as_ptr() as *const _, data: core::ptr::null() },
    of_device_id { name: core::ptr::null(), type_: core::ptr::null(), compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe extern "C" fn u8500_init_machine() {
    of_platform_populate(core::ptr::null_mut(), U8500_LOCAL_BUS_NODES.as_ptr(),
                         core::ptr::null(), core::ptr::null_mut());
}

static STERICSSON_DT_PLATFORM_COMPAT: [*const core::ffi::c_char; 3] = [
    b"st-ericsson,u8500\0".as_ptr() as *const _,
    b"st-ericsson,u9500\0".as_ptr() as *const _,
    core::ptr::null(),
];

// DT_MACHINE_START(U8500_DT, "ST-Ericsson Ux5x0 platform (Device Tree Support)")
// .l2c_aux_val = 0, .l2c_aux_mask = !0, .init_irq = ux500_init_irq,
// .init_machine = u8500_init_machine, .dt_compat = stericsson_dt_platform_compat,
// .restart = ux500_restart, MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
