// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Low-level power-management support for Alpine platform.
 *
 * Copyright (C) 2015 Annapurna Labs Ltd.
 */

// C dependencies supplied by the surrounding kernel code are intentionally
// left as external Rust declarations.

use core::ffi::{c_char, c_int, c_uint, c_void};

/* NB registers */
const AL_SYSFAB_POWER_CONTROL_BASE: usize = 0x2000;

#[inline]
const fn al_sysfab_power_control(cpu: usize) -> usize {
    AL_SYSFAB_POWER_CONTROL_BASE + cpu * 0x100 + 0x20
}

#[repr(C)]
pub struct Regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct AlCpuResumePerCpu {
    pub resume_addr: u32,
}

#[repr(C)]
pub struct AlCpuResumeRegs {
    pub watermark: u32,
    pub per_cpu: [AlCpuResumePerCpu; 0],
}

extern "C" {
    fn syscon_regmap_lookup_by_compatible(compatible: *const c_char) -> *mut Regmap;
    fn of_find_compatible_node(
        from: *mut DeviceNode,
        ty: *const c_char,
        compatible: *const c_char,
    ) -> *mut DeviceNode;
    fn of_iomap(node: *mut DeviceNode, index: c_int) -> *mut AlCpuResumeRegs;
    fn regmap_write(map: *mut Regmap, reg: usize, val: u32) -> c_int;
    fn readl(addr: *const u32) -> u32;
    fn writel(value: u32, addr: *mut u32);
    fn is_err(ptr: *const c_void) -> bool;
}

const ENOSYS: c_int = 38;
extern "C" {
    static AL_CPU_RESUME_MAGIC_NUM_MASK: u32;
    static AL_CPU_RESUME_MAGIC_NUM: u32;
}

static mut al_sysfabric: *mut Regmap = core::ptr::null_mut();
static mut al_cpu_resume_regs: *mut AlCpuResumeRegs = core::ptr::null_mut();
static mut wakeup_supported: c_int = 0;

pub unsafe fn alpine_cpu_wakeup(phys_cpu: c_uint, phys_resume_addr: u32) -> c_int {
    if wakeup_supported == 0 {
        return -ENOSYS;
    }

    /*
     * Set CPU resume address -
     * secure firmware running on boot will jump to this address
     * after setting proper CPU mode, and initializing e.g. secure
     * regs (the same mode all CPUs are booted to - usually HYP)
     */
    let resume_addr = (*al_cpu_resume_regs)
        .per_cpu
        .as_mut_ptr()
        .add(phys_cpu as usize)
        .cast::<AlCpuResumePerCpu>();
    writel(phys_resume_addr, &mut (*resume_addr).resume_addr);

    /* Power-up the CPU */
    regmap_write(
        al_sysfabric,
        al_sysfab_power_control(phys_cpu as usize),
        0,
    );

    0
}

pub unsafe fn alpine_cpu_pm_init() {
    let mut np: *mut DeviceNode;
    let mut watermark: u32;

    al_sysfabric = syscon_regmap_lookup_by_compatible(
        b"al,alpine-sysfabric-service\0".as_ptr().cast(),
    );

    np = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null(),
        b"al,alpine-cpu-resume\0".as_ptr().cast(),
    );
    al_cpu_resume_regs = of_iomap(np, 0);

    wakeup_supported = (!is_err(al_sysfabric.cast()) && !al_cpu_resume_regs.is_null()) as c_int;

    if wakeup_supported != 0 {
        watermark = readl(&(*al_cpu_resume_regs).watermark);
        wakeup_supported =
            ((watermark & AL_CPU_RESUME_MAGIC_NUM_MASK) == AL_CPU_RESUME_MAGIC_NUM) as c_int;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
