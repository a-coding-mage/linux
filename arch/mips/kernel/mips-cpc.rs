// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2013 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Linux headers supplied by the surrounding translation unit provide these
// types, constants, per-CPU helpers, and external functions.

extern "C" {
    static mut mips_cpc_base: *mut core::ffi::c_void;
    fn mips_cm_present() -> bool;
    fn read_gcr_cpc_status() -> usize;
    fn read_gcr_cpc_base() -> usize;
    fn write_gcr_cpc_base(value: usize);
    fn mips_cm_revision() -> u32;
    fn write_cpc_cl_other(value: usize);
    fn ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
    fn of_find_compatible_node(
        root: *mut device_node,
        from: *mut device_node,
        compatible: *const core::ffi::c_char,
    ) -> *mut device_node;
    fn of_address_to_resource(node: *mut device_node, index: u32, res: *mut resource) -> i32;
    fn of_node_put(node: *mut device_node);
    fn cpu_core(cpu_data: *const cpu_data) -> u32;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn preempt_disable();
    fn preempt_enable();
    static mut of_root: *mut device_node;
    static current_cpu_data: cpu_data;
}

type phys_addr_t = usize;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: usize,
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpu_data {
    _private: [u8; 0],
}

// DEFINE_PER_CPU_ALIGNED(spinlock_t, cpc_core_lock);
static mut cpc_core_lock: spinlock_t = spinlock_t { _private: [] };

// DEFINE_PER_CPU_ALIGNED(unsigned long, cpc_core_lock_flags);
static mut cpc_core_lock_flags: usize = 0;

extern "C" {
    fn for_each_possible_cpu_body(cpu: *mut u32);
}

#[inline]
pub unsafe fn mips_cpc_default_phys_base() -> phys_addr_t {
    let cpc_node: *mut device_node;
    let mut res = resource { start: 0, _private: [] };
    let err: i32;

    cpc_node = of_find_compatible_node(of_root, core::ptr::null_mut(), b"mti,mips-cpc\0".as_ptr() as *const _);
    if !cpc_node.is_null() {
        err = of_address_to_resource(cpc_node, 0, &mut res);
        of_node_put(cpc_node);
        if err == 0 {
            return res.start;
        }
    }

    0
}

/**
 * mips_cpc_phys_base - retrieve the physical base address of the CPC
 *
 * This function returns the physical base address of the Cluster Power
 * Controller memory mapped registers, or 0 if no Cluster Power Controller is
 * present.
 */
unsafe fn mips_cpc_phys_base() -> phys_addr_t {
    let mut cpc_base: usize;

    if !mips_cm_present() {
        return 0;
    }

    if (read_gcr_cpc_status() & CM_GCR_CPC_STATUS_EX) == 0 {
        return 0;
    }

    /* If the CPC is already enabled, leave it so */
    cpc_base = read_gcr_cpc_base();
    if (cpc_base & CM_GCR_CPC_BASE_CPCEN) != 0 {
        return cpc_base & CM_GCR_CPC_BASE_CPCBASE;
    }

    /* Otherwise, use the default address */
    cpc_base = mips_cpc_default_phys_base();
    if cpc_base == 0 {
        return cpc_base;
    }

    /* Enable the CPC, mapped at the default address */
    write_gcr_cpc_base(cpc_base | CM_GCR_CPC_BASE_CPCEN);
    cpc_base
}

pub unsafe fn mips_cpc_probe() -> i32 {
    let addr: phys_addr_t;
    let mut cpu: u32 = 0;

    // for_each_possible_cpu(cpu)
    for_each_possible_cpu_body(&mut cpu);
    spin_lock_init(&mut cpc_core_lock);

    addr = mips_cpc_phys_base();
    if addr == 0 {
        return -ENODEV;
    }

    mips_cpc_base = ioremap(addr, 0x8000);
    if mips_cpc_base.is_null() {
        return -ENXIO;
    }

    0
}

pub unsafe fn mips_cpc_lock_other(core: u32) {
    let curr_core: u32;

    if mips_cm_revision() >= CM_REV_CM3 {
        /* Systems with CM >= 3 lock the CPC via mips_cm_lock_other */
        return;
    }

    preempt_disable();
    curr_core = cpu_core(&current_cpu_data);
    spin_lock_irqsave(&mut cpc_core_lock, &mut cpc_core_lock_flags);
    write_cpc_cl_other(FIELD_PREP(CPC_Cx_OTHER_CORENUM, core as usize));

    /*
     * Ensure the core-other region reflects the appropriate core &
     * VP before any accesses to it occur.
     */
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    let _ = curr_core;
}

pub unsafe fn mips_cpc_unlock_other() {
    let curr_core: u32;

    if mips_cm_revision() >= CM_REV_CM3 {
        /* Systems with CM >= 3 lock the CPC via mips_cm_lock_other */
        return;
    }

    curr_core = cpu_core(&current_cpu_data);
    spin_unlock_irqrestore(&mut cpc_core_lock, cpc_core_lock_flags);
    preempt_enable();
    let _ = curr_core;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
