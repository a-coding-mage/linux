/*
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file "COPYING" in the main directory of this
 * archive for more details.
 *
 * Copyright (C) 2000 - 2001 by Kanoj Sarcar (kanoj@sgi.com)
 * Copyright (C) 2000 - 2001 by Silicon Graphics, Inc.
 * Copyright (C) 2000, 2001, 2002 Ralf Baechle
 * Copyright (C) 2000, 2001 Broadcom Corporation
 */

// Dependencies supplied by the corresponding Linux and MIPS headers are
// intentionally left as external Rust symbols.

unsafe extern "C" {
    pub static mut smp_num_siblings: core::ffi::c_int;
    pub static mut cpu_sibling_map: [crate::cpumask_t; crate::NR_CPUS];
    pub static mut cpu_core_map: [crate::cpumask_t; crate::NR_CPUS];
    pub static mut cpu_foreign_map: [crate::cpumask_t; crate::NR_CPUS];
    pub static mut __cpu_number_map: [core::ffi::c_int; crate::CONFIG_MIPS_NR_CPU_NR_MAP];
    pub static mut __cpu_logical_map: [core::ffi::c_int; crate::NR_CPUS];
    pub static mut cpu_coherent_mask: crate::cpumask_t;
    pub static mut smp_max_threads: core::ffi::c_uint;
    pub fn smp_bootstrap();
    pub fn calculate_cpu_foreign_map();
    pub fn start_secondary();
    pub fn mips_smp_ipi_allocate(mask: *const crate::cpumask) -> core::ffi::c_int;
    pub fn mips_smp_ipi_free(mask: *const crate::cpumask) -> core::ffi::c_int;
}

pub unsafe fn raw_smp_processor_id() -> core::ffi::c_int {
    // Under __VDSO__, the C source declares vdso_smp_processor_id() with a
    // compile-time error attribute and calls it here.
    #[cfg(__VDSO__)]
    {
        unsafe extern "C" {
            fn vdso_smp_processor_id() -> core::ffi::c_int;
        }
        return vdso_smp_processor_id();
    }

    #[cfg(not(__VDSO__))]
    {
        crate::current_thread_info().cpu
    }
}

pub const NO_PROC_ID: core::ffi::c_int = -1;

pub const SMP_RESCHEDULE_YOURSELF: core::ffi::c_uint = 0x1; // XXX braindead
pub const SMP_CALL_FUNCTION: core::ffi::c_uint = 0x2;
// Octeon - Tell another core to flush its icache
pub const SMP_ICACHE_FLUSH: core::ffi::c_uint = 0x4;
// Loongson64 - Self IPI for IRQ work
pub const SMP_IRQ_WORK: core::ffi::c_uint = 0x8;

pub const fn cpu_number_map(cpu: usize) -> core::ffi::c_int {
    unsafe { __cpu_number_map[cpu] }
}

pub const fn cpu_logical_map(cpu: usize) -> core::ffi::c_int {
    unsafe { __cpu_logical_map[cpu] }
}

/*
 * this function sends a 'reschedule' IPI to another CPU.
 * it goes straight through and wastes no time serializing
 * anything. Worst case is that we lose a reschedule ...
 */
pub unsafe fn arch_smp_send_reschedule(cpu: core::ffi::c_int) {
    unsafe {
        crate::mp_ops.send_ipi_single(cpu, SMP_RESCHEDULE_YOURSELF);
    }
}

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe fn __cpu_disable() -> core::ffi::c_int {
    unsafe { crate::mp_ops.cpu_disable() }
}

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe fn __cpu_die(cpu: core::ffi::c_uint) {
    unsafe { crate::mp_ops.cpu_die(cpu) }
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe extern "C" {
    pub fn play_dead() -> !;
}

#[cfg(CONFIG_KEXEC_CORE)]
pub unsafe fn kexec_nonboot_cpu() {
    unsafe { crate::mp_ops.kexec_nonboot_cpu() }
}

#[cfg(CONFIG_KEXEC_CORE)]
pub unsafe fn kexec_nonboot_cpu_func() -> *mut core::ffi::c_void {
    unsafe { crate::mp_ops.kexec_nonboot_cpu }
}

pub unsafe fn arch_send_call_function_single_ipi(cpu: core::ffi::c_int) {
    unsafe {
        crate::mp_ops.send_ipi_single(cpu, SMP_CALL_FUNCTION);
    }
}

pub unsafe fn arch_send_call_function_ipi_mask(mask: *const crate::cpumask) {
    unsafe {
        crate::mp_ops.send_ipi_mask(mask, SMP_CALL_FUNCTION);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
