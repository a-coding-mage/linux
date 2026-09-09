// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 Western Digital Corporation or its affiliates.
 */

// Declarations supplied by the corresponding kernel headers and other
// translation units are intentionally left external.

#[repr(C)]
pub struct CpuOps {
    pub cpu_stop: Option<unsafe extern "C" fn()>,
    pub cpu_is_stopped: Option<unsafe extern "C" fn(cpu: u32) -> i32>,
}

extern "C" {
    pub static mut cpu_ops: *mut CpuOps;

    fn smp_processor_id() -> u32;
    fn remove_cpu_topology(cpu: u32);
    fn numa_remove_cpu(cpu: u32);
    fn set_cpu_online(cpu: u32, online: bool);
    fn riscv_ipi_disable();
    fn irq_migrate_all_off_this_cpu();
    fn clear_tasks_mm_cpumask(cpu: u32);
    fn idle_task_exit();
    fn cpuhp_ap_report_dead();
    fn bug();
    fn pr_notice(fmt: *const core::ffi::c_char, ...);
    fn pr_warn(fmt: *const core::ffi::c_char, ...);
}

// EOPNOTSUPP is supplied by the kernel errno definitions.
extern "C" {
    static EOPNOTSUPP: i32;
}

pub unsafe fn cpu_has_hotplug(_cpu: u32) -> bool {
    (*cpu_ops).cpu_stop.is_some()
}

/*
 * __cpu_disable runs on the processor to be shutdown.
 */
pub unsafe fn __cpu_disable() -> i32 {
    let cpu: u32 = smp_processor_id();

    if (*cpu_ops).cpu_stop.is_none() {
        return -EOPNOTSUPP;
    }

    remove_cpu_topology(cpu);
    numa_remove_cpu(cpu);
    set_cpu_online(cpu, false);
    riscv_ipi_disable();
    irq_migrate_all_off_this_cpu();

    0
}

/*
 * Called on the thread which is asking for a CPU to be shutdown, if the
 * CPU reported dead to the hotplug core.
 */
pub unsafe fn arch_cpuhp_cleanup_dead_cpu(cpu: u32) {
    let mut ret: i32 = 0;

    pr_notice(b"CPU%u: off\0".as_ptr() as *const core::ffi::c_char, cpu);
    clear_tasks_mm_cpumask(cpu);
    /* Verify from the firmware if the cpu is really stopped*/
    if let Some(cpu_is_stopped) = (*cpu_ops).cpu_is_stopped {
        ret = cpu_is_stopped(cpu);
    }
    if ret == 0 {
        pr_warn(
            b"CPU%u may not have stopped\n\0".as_ptr() as *const core::ffi::c_char,
            cpu,
        );
    }
}

/*
 * Called from the idle thread for the CPU which has been shutdown.
 */
pub unsafe fn arch_cpu_idle_dead() -> ! {
    idle_task_exit();

    cpuhp_ap_report_dead();

    if let Some(cpu_stop) = (*cpu_ops).cpu_stop {
        cpu_stop();
    }
    /* It should never reach here */
    bug();
    core::hint::unreachable_unchecked()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
