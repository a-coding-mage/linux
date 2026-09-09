// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2000-2003 Deep Blue Solutions Ltd
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

// The kernel's raw spinlock object and MMIO/memory-management interfaces are
// provided externally.  These declarations preserve the C file's interfaces.
extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn raw_spin_lock_irqsave(lock: *mut RawSpinLock, flags: *mut c_ulong);
    fn raw_spin_unlock_irqrestore(lock: *mut RawSpinLock, flags: c_ulong);
    fn of_find_matching_node(from: *mut DeviceNode, matches: *const OfDeviceId) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: c_int) -> *mut c_void;
    fn memblock_reserve(base: usize, size: usize) -> c_int;
    fn __pa(address: *mut c_void) -> usize;
    static mut swapper_pg_dir: c_void;
}

#[repr(C)]
pub struct RawSpinLock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const u8,
}

type c_ulong = usize;
type c_int = i32;

// Supplied by the platform headers.
// These are preprocessor constants in the C headers; their concrete values
// are supplied by the target platform translation.
const INTEGRATOR_HDR_CTRL_OFFSET: usize = 0;
const INTEGRATOR_HDR_IC_OFFSET: usize = 0;
const IRQ_ENABLE_CLEAR: usize = 0;
const PHYS_OFFSET: usize = 0;

static mut cm_base: *mut c_void = core::ptr::null_mut();
static mut cm_lock: RawSpinLock = RawSpinLock { _private: [] };

/**
 * cm_get - get the value from the CM_CTRL register
 */
pub unsafe fn cm_get() -> u32 {
    readl(cm_base.add(INTEGRATOR_HDR_CTRL_OFFSET))
}

/**
 * cm_control - update the CM_CTRL register.
 * @mask: bits to change
 * @set: bits to set
 */
pub unsafe fn cm_control(mask: u32, set: u32) {
    let mut flags: c_ulong = 0;
    let mut val: u32;

    raw_spin_lock_irqsave(&mut cm_lock, &mut flags);
    val = readl(cm_base.add(INTEGRATOR_HDR_CTRL_OFFSET)) & !mask;
    writel(
        val | set,
        cm_base.add(INTEGRATOR_HDR_CTRL_OFFSET),
    );
    raw_spin_unlock_irqrestore(&mut cm_lock, flags);
}

pub unsafe fn cm_clear_irqs() {
    /* disable core module IRQs */
    writel(
        0xffff_ffff_u32,
        cm_base.add(INTEGRATOR_HDR_IC_OFFSET + IRQ_ENABLE_CLEAR),
    );
}

static cm_match: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"arm,core-module-integrator\0".as_ptr(),
    },
    OfDeviceId {
        compatible: core::ptr::null(),
    },
];

pub unsafe fn cm_init() {
    let cm = of_find_matching_node(core::ptr::null_mut(), cm_match.as_ptr());

    if cm.is_null() {
        // pr_crit("no core module node found in device tree\n");
        return;
    }
    cm_base = of_iomap(cm, 0);
    if cm_base.is_null() {
        // pr_crit("could not remap core module\n");
        return;
    }
    cm_clear_irqs();
}

/*
 * We need to stop things allocating the low memory; ideally we need a
 * better implementation of GFP_DMA which does not assume that DMA-able
 * memory starts at zero.
 */
pub unsafe fn integrator_reserve() {
    memblock_reserve(
        PHYS_OFFSET,
        __pa(core::ptr::addr_of_mut!(swapper_pg_dir)) - PHYS_OFFSET,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
