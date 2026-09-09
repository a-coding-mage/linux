/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

/* The following items are conditional on CONFIG_SMP in the C header. */
#[cfg(feature = "CONFIG_SMP")]
pub unsafe fn raw_smp_processor_id() -> i32 {
    // Equivalent to: (current_thread_info()->cpu)
    unsafe { (*current_thread_info()).cpu }
}

#[cfg(feature = "CONFIG_SMP")]
#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn arch_send_call_function_single_ipi(cpu: i32);
    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);

    pub fn smp_init_cpus();
    pub fn first_lines_of_secondary();
    pub fn arc_platform_smp_cpuinfo() -> *const core::ffi::c_char;
    pub fn arc_platform_smp_wait_to_boot(arg: i32);
    pub fn start_kernel_secondary();

    pub fn smp_ipi_irq_setup(cpu: i32, hwirq: irq_hw_number_t) -> i32;
}

#[cfg(feature = "CONFIG_SMP")]
#[repr(C)]
pub struct plat_smp_ops {
    pub info: *const core::ffi::c_char,
    pub init_early_smp: Option<unsafe extern "C" fn()>,
    pub init_per_cpu: Option<unsafe extern "C" fn(cpu: i32)>,
    pub cpu_kick: Option<unsafe extern "C" fn(cpu: i32, pc: core::ffi::c_ulong)>,
    pub ipi_send: Option<unsafe extern "C" fn(cpu: i32)>,
    pub ipi_clear: Option<unsafe extern "C" fn(irq: i32)>,
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub static mut plat_smp_ops: plat_smp_ops;
    pub static mut smp_atomic_ops_lock: arch_spinlock_t;
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub fn smp_init_cpus() {}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub fn arc_platform_smp_cpuinfo() -> *const core::ffi::c_char {
    b"\0".as_ptr() as *const core::ffi::c_char
}

/*
 * ARC700 doesn't support atomic Read-Modify-Write ops.  The C header uses
 * these low-level operations when CONFIG_ARC_HAS_LLSC is absent.
 */
#[cfg(not(feature = "CONFIG_ARC_HAS_LLSC"))]
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! atomic_ops_lock {
    ($flags:ident) => {{
        unsafe { local_irq_save(&mut $flags); }
        unsafe { arch_spin_lock(&raw mut smp_atomic_ops_lock); }
    }};
}

#[cfg(not(feature = "CONFIG_ARC_HAS_LLSC"))]
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! atomic_ops_unlock {
    ($flags:ident) => {{
        unsafe { arch_spin_unlock(&raw mut smp_atomic_ops_lock); }
        unsafe { local_irq_restore($flags); }
    }};
}

#[cfg(not(feature = "CONFIG_ARC_HAS_LLSC"))]
#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! atomic_ops_lock {
    ($flags:ident) => {{ unsafe { local_irq_save(&mut $flags); } }};
}

#[cfg(not(feature = "CONFIG_ARC_HAS_LLSC"))]
#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! atomic_ops_unlock {
    ($flags:ident) => {{ unsafe { local_irq_restore($flags); } }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
