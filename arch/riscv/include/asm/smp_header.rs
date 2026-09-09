/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 */

// The C header includes linux/cpumask.h, linux/irqreturn.h, and
// linux/thread_info.h. Their Rust declarations are supplied externally.

pub const INVALID_HARTID: ::core::ffi::c_ulong = ::core::primitive::usize::MAX as ::core::ffi::c_ulong;

pub enum seq_file {}

extern "C" {
    pub static mut boot_cpu_hartid: ::core::ffi::c_ulong;
}

// CONFIG_SMP is a build-time configuration condition preserved from the C header.
#[cfg(CONFIG_SMP)]
pub mod config_smp {
    use super::seq_file;

    // Mapping between linux logical cpu index and hartid.
    extern "C" {
        pub static mut __cpuid_to_hartid_map: [::core::ffi::c_ulong; NR_CPUS];
    }

    pub unsafe fn cpuid_to_hartid_map(cpu: usize) -> ::core::ffi::c_ulong {
        __cpuid_to_hartid_map[cpu]
    }

    /* print IPI stats */
    extern "C" {
        pub fn show_ipi_stats(p: *mut seq_file, prec: ::core::ffi::c_int);

        /* SMP initialization hook for setup_arch */
        pub fn setup_smp();

        /* Hook for the generic smp_call_function_many() routine. */
        pub fn arch_send_call_function_ipi_mask(mask: *mut cpumask);

        /* Hook for the generic smp_call_function_single() routine. */
        pub fn arch_send_call_function_single_ipi(cpu: ::core::ffi::c_int);

        pub fn riscv_hartid_to_cpuid(hartid: ::core::ffi::c_ulong) -> ::core::ffi::c_int;

        /* Enable IPI for CPU hotplug */
        pub fn riscv_ipi_enable();

        /* Disable IPI for CPU hotplug */
        pub fn riscv_ipi_disable();

        /* Check if IPI interrupt numbers are available */
        pub fn riscv_ipi_have_virq_range() -> bool;

        /* Set the IPI interrupt numbers for arch (called by irqchip drivers) */
        pub fn riscv_ipi_set_virq_range(virq: ::core::ffi::c_int, nr: ::core::ffi::c_int);

        /* Check other CPUs stop or not */
        pub fn smp_crash_stop_failed() -> bool;

        /* Secondary hart entry */
        pub fn smp_callin();

        #[cfg(CONFIG_HOTPLUG_CPU)]
        pub fn __cpu_disable() -> ::core::ffi::c_int;
    }

    /* Obtains the hart ID of the currently executing task. */
    pub unsafe fn raw_smp_processor_id() -> ::core::ffi::c_uint {
        current_thread_info().cpu
    }

    #[cfg(CONFIG_HOTPLUG_CPU)]
    pub unsafe fn __cpu_die(_cpu: ::core::ffi::c_uint) {}
}

// CONFIG_SMP is disabled in this configuration.
#[cfg(not(CONFIG_SMP))]
pub mod config_no_smp {
    use super::{seq_file, boot_cpu_hartid};

    pub unsafe fn show_ipi_stats(_p: *mut seq_file, _prec: ::core::ffi::c_int) {}

    pub unsafe fn riscv_hartid_to_cpuid(hartid: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
        if hartid == boot_cpu_hartid {
            return 0;
        }
        -1
    }

    pub unsafe fn cpuid_to_hartid_map(_cpu: ::core::ffi::c_int) -> ::core::ffi::c_ulong {
        boot_cpu_hartid
    }

    pub unsafe fn riscv_ipi_enable() {}
    pub unsafe fn riscv_ipi_disable() {}

    pub unsafe fn riscv_ipi_have_virq_range() -> bool {
        false
    }

    pub unsafe fn riscv_ipi_set_virq_range(_virq: ::core::ffi::c_int, _nr: ::core::ffi::c_int) {}
}

// CONFIG_HOTPLUG_CPU is a build-time configuration condition preserved from the C header.
#[cfg(CONFIG_HOTPLUG_CPU)]
extern "C" {
    pub fn cpu_has_hotplug(cpu: ::core::ffi::c_uint) -> bool;
}

#[cfg(not(CONFIG_HOTPLUG_CPU))]
pub unsafe fn cpu_has_hotplug(_cpu: ::core::ffi::c_uint) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
