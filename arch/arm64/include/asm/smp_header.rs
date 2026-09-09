/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

/* Values for secondary_data.status */
pub const CPU_STUCK_REASON_SHIFT: u32 = 8;
pub const CPU_BOOT_STATUS_MASK: u64 = ((1u64 << CPU_STUCK_REASON_SHIFT) - 1);

pub const CPU_MMU_OFF: i32 = -1;
pub const CPU_BOOT_SUCCESS: i32 = 0;
/* The cpu invoked ops->cpu_die, synchronise it with cpu_kill */
pub const CPU_KILL_ME: i32 = 1;
/* The cpu couldn't die gracefully and is looping in the kernel */
pub const CPU_STUCK_IN_KERNEL: i32 = 2;
/* Fatal system error detected by secondary CPU, crash the system */
pub const CPU_PANIC_KERNEL: i32 = 3;

pub const CPU_STUCK_REASON_52_BIT_VA: u64 = 1u64 << CPU_STUCK_REASON_SHIFT;
pub const CPU_STUCK_REASON_NO_GRAN: u64 = 2u64 << CPU_STUCK_REASON_SHIFT;

/* The following declarations depend on the surrounding kernel bindings. */
pub unsafe fn raw_smp_processor_id() -> ::core::ffi::c_uint {
    current_thread_info().cpu
}

extern "C" {
    pub static mut __cpu_logical_map: [u64; NR_CPUS];
    pub fn cpu_logical_map(cpu: ::core::ffi::c_uint) -> u64;
    pub fn smp_init_cpus();
    pub fn set_smp_ipi_range_percpu(
        ipi_base: ::core::ffi::c_int,
        nr_ipi: ::core::ffi::c_int,
        ncpus: ::core::ffi::c_int,
    );
    /* Called from the secondary holding pen, this is the secondary CPU entry point. */
    pub fn secondary_start_kernel();
    pub static mut secondary_data: secondary_data;
    pub static mut __early_cpu_boot_status: ::core::ffi::c_long;
    pub fn secondary_entry();
    pub fn arch_send_call_function_single_ipi(cpu: ::core::ffi::c_int);
    pub fn arch_send_call_function_ipi_mask(mask: *const cpumask);
    pub fn __cpu_disable() -> ::core::ffi::c_int;
    pub fn cpu_die() -> !;
    pub fn cpu_die_early() -> !;
/*
 * If a secondary CPU enters the kernel but fails to come online,
 * (e.g. due to mismatched features), and cannot exit the kernel,
 * we increment cpus_stuck_in_kernel and leave the CPU in a
 * quiesecent loop within the kernel text. The memory containing
 * this loop must not be re-used for anything else as the 'stuck'
 * core is executing it.
 *
 * This function is used to inhibit features like kexec and hibernate.
 */
pub fn cpus_are_stuck_in_kernel() -> bool;
    pub fn crash_smp_send_stop();
    pub fn smp_crash_stop_failed() -> bool;
}

pub unsafe fn set_cpu_logical_map(cpu: ::core::ffi::c_uint, hwid: u64) {
    __cpu_logical_map[cpu as usize] = hwid;
}

#[repr(C)]
pub struct seq_file;

#[repr(i32)]
pub enum ipi_msg_type {
    IPI_RESCHEDULE,
    IPI_CALL_FUNC,
    IPI_CPU_STOP,
    IPI_CPU_STOP_NMI,
    IPI_TIMER,
    IPI_IRQ_WORK,
    NR_IPI,
    /*
     * Any enum >= NR_IPI and < MAX_IPI is special and not tracable
     * with trace_ipi_*
     */
    IPI_CPU_BACKTRACE = NR_IPI as isize,
    IPI_KGDB_ROUNDUP,
    MAX_IPI,
}

#[repr(C)]
pub struct task_struct;

#[repr(C)]
pub struct secondary_data {
    pub task: *mut task_struct,
    pub status: ::core::ffi::c_long,
}

#[repr(C)]
pub struct cpumask;

pub unsafe fn set_smp_ipi_range(ipi_base: ::core::ffi::c_int, n: ::core::ffi::c_int) {
    set_smp_ipi_range_percpu(ipi_base, n, 0);
}

/* CONFIG_ARM64_ACPI_PARKING_PROTOCOL selects the external wakeup implementation. */
#[cfg(CONFIG_ARM64_ACPI_PARKING_PROTOCOL)]
extern "C" {
    pub fn arch_send_wakeup_ipi(cpu: ::core::ffi::c_uint);
}

#[cfg(not(CONFIG_ARM64_ACPI_PARKING_PROTOCOL))]
pub unsafe fn arch_send_wakeup_ipi(_cpu: ::core::ffi::c_uint) {
    BUILD_BUG!();
}

pub unsafe fn __cpu_die(_cpu: ::core::ffi::c_uint) {}

pub unsafe fn cpu_park_loop() -> ! {
    loop {
        wfe();
        wfi();
    }
}

pub unsafe fn update_cpu_boot_status(val: ::core::ffi::c_int) {
    WRITE_ONCE(secondary_data.status, val);
    /* Ensure the visibility of the status update */
    dsb(ishst);
}

/*
 * The calling secondary CPU has detected serious configuration mismatch,
 * which calls for a kernel panic. Update the boot status and park the calling
 * CPU.
 */
pub unsafe fn cpu_panic_kernel() -> ! {
    update_cpu_boot_status(CPU_PANIC_KERNEL);
    cpu_park_loop();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
