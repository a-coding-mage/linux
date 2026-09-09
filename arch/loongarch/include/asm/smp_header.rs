/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

/* CONFIG_SMP */

#[repr(C)]
pub struct smp_ops {
    pub init_ipi: Option<unsafe extern "C" fn()>,
    pub send_ipi_single: Option<unsafe extern "C" fn(cpu: core::ffi::c_int, action: core::ffi::c_uint)>,
    pub send_ipi_mask: Option<unsafe extern "C" fn(mask: *const cpumask, action: core::ffi::c_uint)>,
}

extern "C" {
    pub static mut mp_ops: smp_ops;

    pub static mut smp_num_siblings: core::ffi::c_int;
    pub static mut num_processors: core::ffi::c_int;
    pub static mut disabled_cpus: core::ffi::c_int;
    pub static mut cpu_sibling_map: [cpumask_t; 0];
    pub static mut cpu_llc_shared_map: [cpumask_t; 0];
    pub static mut cpu_core_map: [cpumask_t; 0];
    pub static mut cpu_foreign_map: [cpumask_t; 0];

    pub fn loongson_smp_setup();
    pub fn loongson_prepare_cpus(max_cpus: core::ffi::c_uint);
    pub fn loongson_boot_secondary(cpu: core::ffi::c_int, idle: *mut task_struct);
    pub fn loongson_init_secondary();
    pub fn loongson_smp_finish();

    /* CONFIG_HOTPLUG_CPU */
    pub fn loongson_cpu_disable() -> core::ffi::c_int;
    pub fn loongson_cpu_die(cpu: core::ffi::c_uint);

    pub fn current_thread_info() -> *mut thread_info;

    pub static mut __cpu_number_map: [core::ffi::c_int; 0];
    pub static mut __cpu_logical_map: [core::ffi::c_int; 0];

    pub fn smpboot_entry();
    pub fn start_secondary();
    pub fn calculate_cpu_foreign_map();
    pub fn show_ipi_list(p: *mut seq_file, prec: core::ffi::c_int);
}

#[repr(C)]
pub struct cpumask_t {
    _private: [u8; 0],
}
pub type cpumask = cpumask_t;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread_info {
    pub cpu: core::ffi::c_int,
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct secondary_data {
    pub task: core::ffi::c_ulong,
    pub stack: core::ffi::c_ulong,
    pub offset: core::ffi::c_ulong,
}

extern "C" {
    pub static mut cpuboot_data: secondary_data;
}

#[inline]
pub unsafe fn plat_smp_setup() {
    loongson_smp_setup();
}

#[inline]
pub unsafe fn raw_smp_processor_id() -> core::ffi::c_int {
    /* __VDSO__ builds use vdso_smp_processor_id(), which is a compile-time error if called. */
    (*current_thread_info()).cpu
}

#[inline]
pub unsafe fn cpu_number_map(cpu: usize) -> core::ffi::c_int {
    __cpu_number_map[cpu]
}

#[inline]
pub unsafe fn cpu_logical_map(cpu: usize) -> core::ffi::c_int {
    __cpu_logical_map[cpu]
}

#[inline]
pub unsafe fn cpu_physical_id(cpu: usize) -> core::ffi::c_int {
    cpu_logical_map(cpu)
}

pub const ACTION_BOOT_CPU: core::ffi::c_uint = 0;
pub const ACTION_RESCHEDULE: core::ffi::c_uint = 1;
pub const ACTION_CALL_FUNCTION: core::ffi::c_uint = 2;
pub const ACTION_IRQ_WORK: core::ffi::c_uint = 3;
pub const ACTION_CLEAR_VECTOR: core::ffi::c_uint = 4;
pub const SMP_BOOT_CPU: core::ffi::c_uint = 1 << ACTION_BOOT_CPU;
pub const SMP_RESCHEDULE: core::ffi::c_uint = 1 << ACTION_RESCHEDULE;
pub const SMP_CALL_FUNCTION: core::ffi::c_uint = 1 << ACTION_CALL_FUNCTION;
pub const SMP_IRQ_WORK: core::ffi::c_uint = 1 << ACTION_IRQ_WORK;
pub const SMP_CLEAR_VECTOR: core::ffi::c_uint = 1 << ACTION_CLEAR_VECTOR;

#[inline]
pub unsafe fn arch_send_call_function_single_ipi(cpu: core::ffi::c_int) {
    if let Some(send_ipi_single) = mp_ops.send_ipi_single {
        send_ipi_single(cpu, ACTION_CALL_FUNCTION);
    }
}

#[inline]
pub unsafe fn arch_send_call_function_ipi_mask(mask: *const cpumask) {
    if let Some(send_ipi_mask) = mp_ops.send_ipi_mask {
        send_ipi_mask(mask, ACTION_CALL_FUNCTION);
    }
}

/* CONFIG_HOTPLUG_CPU */
#[inline]
pub unsafe fn __cpu_disable() -> core::ffi::c_int {
    loongson_cpu_disable()
}

#[inline]
pub unsafe fn __cpu_die(cpu: core::ffi::c_uint) {
    loongson_cpu_die(cpu);
}

/* !CONFIG_SMP: cpu_logical_map(cpu) expands to 0. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
