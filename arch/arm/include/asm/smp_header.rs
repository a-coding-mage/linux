/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/smp.h
 *
 *  Copyright (C) 2004-2005 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation.

// This header requires CONFIG_SMP.

#[inline(always)]
pub unsafe fn raw_smp_processor_id() -> ::core::ffi::c_int {
    current_thread_info().cpu
}

#[repr(C)]
pub struct seq_file;

/* generate IPI list text */
extern "C" {
    pub fn show_ipi_list(file: *mut seq_file, cpu: ::core::ffi::c_int);

    /* Called from C code, this handles an IPI. */
    pub fn handle_IPI(ipinr: ::core::ffi::c_int, regs: *mut pt_regs);

    /* Setup the set of possible CPUs (via set_cpu_possible) */
    pub fn smp_init_cpus();

    /* Register IPI interrupts with the arch SMP code */
    pub fn set_smp_ipi_range(ipi_base: ::core::ffi::c_int, nr_ipi: ::core::ffi::c_int);

    /* Called from platform specific assembly code, this is the secondary CPU entry point. */
    pub fn secondary_start_kernel(task: *mut task_struct);

    pub static mut secondary_data: secondary_data;
    pub fn secondary_startup();
    pub fn secondary_startup_arm();

    pub fn __cpu_disable() -> ::core::ffi::c_int;

    pub fn arch_send_call_function_single_ipi(cpu: ::core::ffi::c_int);
    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);
    pub fn arch_send_wakeup_ipi_mask(mask: *const cpumask);

    pub fn register_ipi_completion(
        completion: *mut completion,
        cpu: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn smp_set_ops(ops: *const smp_operations);
}

/* Initial data for bringing up a secondary CPU. */
#[repr(C)]
pub union secondary_data_union {
    pub mpu_rgn_info: *mut mpu_rgn_info,
    pub pgdir: u64,
}

#[repr(C)]
pub struct secondary_data {
    pub _union: secondary_data_union,
    pub swapper_pg_dir: ::core::ffi::c_ulong,
    pub stack: *mut ::core::ffi::c_void,
    pub task: *mut task_struct,
}

#[inline(always)]
pub unsafe fn __cpu_die(_cpu: u32) {}

#[repr(C)]
pub struct smp_operations {
    /* Setup the set of possible CPUs (via set_cpu_possible) */
    pub smp_init_cpus: Option<unsafe extern "C" fn()>,
    /* Initialize cpu_possible map, and enable coherency */
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(max_cpus: u32)>,
    /* Perform platform specific initialisation of the specified CPU. */
    pub smp_secondary_init: Option<unsafe extern "C" fn(cpu: u32)>,
    /* Boot a secondary CPU, and assign it the specified idle task.
     * This also gives us the initial stack to use for this CPU. */
    pub smp_boot_secondary:
        Option<unsafe extern "C" fn(cpu: u32, idle: *mut task_struct) -> ::core::ffi::c_int>,
    pub cpu_kill: Option<unsafe extern "C" fn(cpu: u32) -> ::core::ffi::c_int>,
    pub cpu_die: Option<unsafe extern "C" fn(cpu: u32)>,
    pub cpu_can_disable: Option<unsafe extern "C" fn(cpu: u32) -> bool>,
    pub cpu_disable: Option<unsafe extern "C" fn(cpu: u32) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct of_cpu_method {
    pub method: *const ::core::ffi::c_char,
    pub ops: *const smp_operations,
}

// C macro CPU_METHOD_OF_DECLARE(name, _method, _ops):
// creates a used entry in the __cpu_method_of_table linker section.
#[macro_export]
macro_rules! CPU_METHOD_OF_DECLARE {
    ($name:ident, $method:expr, $ops:expr) => {
        #[used]
        #[link_section = "__cpu_method_of_table"]
        static __CPU_METHOD_OF_TABLE_$name: $crate::of_cpu_method = $crate::of_cpu_method {
            method: $method,
            ops: $ops,
        };
    };
}

// External types and current_thread_info are supplied by other translated files.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
