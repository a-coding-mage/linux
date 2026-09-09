// SPDX-License-Identifier: GPL-2.0
/*
 *  HW NMI watchdog support
 *
 *  started by Don Zickus, Copyright (C) 2010 Red Hat, Inc.
 *
 *  Arch specific calls to support NMI watchdog
 *
 *  Bits copied from original nmi.c file
 *
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[cfg(CONFIG_HARDLOCKUP_DETECTOR_PERF)]
pub unsafe fn hw_nmi_get_sample_period(watchdog_thresh: i32) -> u64 {
    (cpu_khz as u64)
        .wrapping_mul(1000)
        .wrapping_mul(watchdog_thresh as u64)
}

// Preserves the source conditional `#ifdef arch_trigger_cpumask_backtrace`.
#[cfg(arch_trigger_cpumask_backtrace)]
unsafe fn nmi_raise_cpu_backtrace(mask: *mut cpumask_t) {
    __apic_send_IPI_mask(mask, NMI_VECTOR);
}

#[cfg(arch_trigger_cpumask_backtrace)]
pub unsafe fn arch_trigger_cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: i32) {
    nmi_trigger_cpumask_backtrace(mask, exclude_cpu, nmi_raise_cpu_backtrace);
}

#[cfg(arch_trigger_cpumask_backtrace)]
unsafe fn nmi_cpu_backtrace_handler(cmd: u32, regs: *mut pt_regs) -> i32 {
    if nmi_cpu_backtrace(regs) != 0 {
        return NMI_HANDLED;
    }

    NMI_DONE
}

// NOKPROBE_SYMBOL(nmi_cpu_backtrace_handler);

#[cfg(arch_trigger_cpumask_backtrace)]
unsafe fn register_nmi_cpu_backtrace_handler() -> i32 {
    register_nmi_handler(NMI_LOCAL, nmi_cpu_backtrace_handler, 0, "arch_bt");
    0
}

// early_initcall(register_nmi_cpu_backtrace_handler);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
