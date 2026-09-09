// SPDX-License-Identifier: GPL-2.0-only
/*
 * SMP initialisation and IPI support
 * Based on arch/arm64/kernel/smp.c
 *
 * Copyright (C) 2012 ARM Ltd.
 * Copyright (C) 2015 Regents of the University of California
 * Copyright (C) 2017 SiFive
 */

// Linux and RISC-V header dependencies are supplied by the surrounding kernel.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ipi_message_type {
    IPI_RESCHEDULE,
    IPI_CALL_FUNC,
    IPI_CPU_STOP,
    IPI_CPU_CRASH_STOP,
    IPI_IRQ_WORK,
    IPI_TIMER,
    IPI_CPU_BACKTRACE,
    IPI_KGDB_ROUNDUP,
    IPI_MAX,
}

static ipi_names: [&'static str; IPI_MAX as usize] = [
    "Rescheduling interrupts",
    "Function call interrupts",
    "CPU stop interrupts",
    "CPU stop (for crash dump) interrupts",
    "IRQ work interrupts",
    "Timer broadcast interrupts",
    "CPU backtrace interrupts",
    "KGDB roundup interrupts",
];

static mut __cpuid_to_hartid_map: [::core::ffi::c_ulong; NR_CPUS] = [INVALID_HARTID; NR_CPUS];

pub unsafe extern "C" fn smp_setup_processor_id() {
    cpuid_to_hartid_map(0) = boot_cpu_hartid;
    pr_info!("Booting Linux on hartid {}\n", boot_cpu_hartid);
}

static mut ipi_dummy_dev: i32 = 0;
static mut ipi_virq_base: i32 = 0;
static mut nr_ipi: i32 = IPI_MAX as i32;
static mut ipi_desc: [*mut irq_desc; IPI_MAX as usize] = [core::ptr::null_mut(); IPI_MAX as usize];

pub unsafe extern "C" fn riscv_hartid_to_cpuid(hartid: ::core::ffi::c_ulong) -> i32 {
    let mut i = 0;
    while i < NR_CPUS as i32 {
        if cpuid_to_hartid_map(i) == hartid { return i; }
        i += 1;
    }
    -ENOENT
}

unsafe fn ipi_stop() {
    set_cpu_online(smp_processor_id(), false);
    loop { wait_for_interrupt(); }
}

#[cfg(CONFIG_KEXEC_CORE)]
static mut waiting_for_crash_ipi: atomic_t = ATOMIC_INIT(0);

#[cfg(CONFIG_KEXEC_CORE)]
unsafe fn ipi_cpu_crash_stop(cpu: u32, regs: *mut pt_regs) {
    crash_save_cpu(regs, cpu);
    atomic_dec(&mut waiting_for_crash_ipi);
    local_irq_disable();
    #[cfg(CONFIG_HOTPLUG_CPU)]
    if cpu_has_hotplug(cpu) { cpu_ops.cpu_stop(); }
    loop { wait_for_interrupt(); }
}

#[cfg(not(CONFIG_KEXEC_CORE))]
unsafe fn ipi_cpu_crash_stop(_cpu: u32, _regs: *mut pt_regs) { unreachable!(); }

unsafe fn send_ipi_mask(mask: *const cpumask, op: ipi_message_type) {
    __ipi_send_mask(ipi_desc[op as usize], mask);
}

unsafe fn send_ipi_single(cpu: i32, op: ipi_message_type) {
    __ipi_send_mask(ipi_desc[op as usize], cpumask_of(cpu));
}

#[cfg(CONFIG_IRQ_WORK)]
pub unsafe extern "C" fn arch_irq_work_raise() { send_ipi_single(smp_processor_id(), IPI_IRQ_WORK); }

unsafe extern "C" fn handle_IPI(irq: i32, _data: *mut core::ffi::c_void) -> irqreturn_t {
    let cpu = smp_processor_id();
    let ipi = irq - ipi_virq_base;
    match ipi {
        x if x == IPI_RESCHEDULE as i32 => scheduler_ipi(),
        x if x == IPI_CALL_FUNC as i32 => generic_smp_call_function_interrupt(),
        x if x == IPI_CPU_STOP as i32 => ipi_stop(),
        x if x == IPI_CPU_CRASH_STOP as i32 => ipi_cpu_crash_stop(cpu as u32, get_irq_regs()),
        x if x == IPI_IRQ_WORK as i32 => irq_work_run(),
        #[cfg(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST)]
        x if x == IPI_TIMER as i32 => tick_receive_broadcast(),
        x if x == IPI_CPU_BACKTRACE as i32 => nmi_cpu_backtrace(get_irq_regs()),
        x if x == IPI_KGDB_ROUNDUP as i32 => kgdb_nmicallback(cpu, get_irq_regs()),
        _ => pr_warn!("CPU{}: unhandled IPI{}\n", cpu, ipi),
    }
    IRQ_HANDLED
}

pub unsafe extern "C" fn riscv_ipi_enable() {
    if WARN_ON_ONCE!(ipi_virq_base == 0) { return; }
    for i in 0..nr_ipi { enable_percpu_irq(ipi_virq_base + i, 0); }
}

pub unsafe extern "C" fn riscv_ipi_disable() {
    if WARN_ON_ONCE!(ipi_virq_base == 0) { return; }
    for i in 0..nr_ipi { disable_percpu_irq(ipi_virq_base + i); }
}

pub unsafe extern "C" fn riscv_ipi_have_virq_range() -> bool { ipi_virq_base != 0 }

pub unsafe extern "C" fn riscv_ipi_set_virq_range(virq: i32, nr: i32) {
    if WARN_ON!(ipi_virq_base != 0) { return; }
    WARN_ON!(nr < IPI_MAX as i32);
    nr_ipi = core::cmp::min(nr, IPI_MAX as i32);
    ipi_virq_base = virq;
    for i in 0..nr_ipi {
        let err = request_percpu_irq(ipi_virq_base + i, handle_IPI, ipi_names[i as usize].as_ptr(), &mut ipi_dummy_dev);
        WARN_ON!(err != 0);
        ipi_desc[i as usize] = irq_to_desc(ipi_virq_base + i);
        irq_set_status_flags(ipi_virq_base + i, IRQ_HIDDEN);
    }
    riscv_ipi_enable();
}

pub unsafe extern "C" fn show_ipi_stats(p: *mut seq_file, prec: i32) {
    for i in 0..IPI_MAX as usize {
        seq_printf!(p, "%*s%u:", prec - 1, "IPI", i);
        for_each_online_cpu!(cpu => seq_printf!(p, "%10u ", irq_desc_kstat_cpu(ipi_desc[i], cpu)));
        seq_printf!(p, " %s\n", ipi_names[i].as_ptr());
    }
}

pub unsafe extern "C" fn arch_send_call_function_ipi_mask(mask: *mut cpumask) { send_ipi_mask(mask, IPI_CALL_FUNC); }
pub unsafe extern "C" fn arch_send_call_function_single_ipi(cpu: i32) { send_ipi_single(cpu, IPI_CALL_FUNC); }

#[cfg(CONFIG_GENERIC_CLOCKEVENTS_BROADCAST)]
pub unsafe extern "C" fn tick_broadcast(mask: *const cpumask) { send_ipi_mask(mask, IPI_TIMER); }

pub unsafe extern "C" fn smp_send_stop() {
    let mut timeout: ::core::ffi::c_ulong;
    if num_online_cpus() > 1 {
        let mut mask: cpumask_t = core::mem::zeroed();
        cpumask_copy(&mut mask, cpu_online_mask);
        cpumask_clear_cpu(smp_processor_id(), &mut mask);
        if system_state <= SYSTEM_RUNNING { pr_crit!("SMP: stopping secondary CPUs\n"); }
        send_ipi_mask(&mask, IPI_CPU_STOP);
    }
    timeout = USEC_PER_SEC;
    while num_online_cpus() > 1 && timeout > 0 { timeout -= 1; udelay(1); }
    if num_online_cpus() > 1 { pr_warn!("SMP: failed to stop secondary CPUs %*pbl\n", cpumask_pr_args(cpu_online_mask)); }
}

#[cfg(CONFIG_KEXEC_CORE)]
unsafe fn num_other_online_cpus() -> u32 { num_online_cpus() - cpu_online(smp_processor_id()) }

#[cfg(CONFIG_KEXEC_CORE)]
pub unsafe extern "C" fn crash_smp_send_stop() {
    static mut cpus_stopped: i32 = 0;
    let mut mask: cpumask_t = core::mem::zeroed();
    let mut timeout: ::core::ffi::c_ulong;
    if cpus_stopped != 0 { return; }
    cpus_stopped = 1;
    if num_other_online_cpus() == 0 { return; }
    cpumask_copy(&mut mask, cpu_online_mask);
    cpumask_clear_cpu(smp_processor_id(), &mut mask);
    atomic_set(&mut waiting_for_crash_ipi, num_other_online_cpus() as i32);
    pr_crit!("SMP: stopping secondary CPUs\n");
    send_ipi_mask(&mask, IPI_CPU_CRASH_STOP);
    timeout = USEC_PER_SEC;
    while atomic_read(&waiting_for_crash_ipi) > 0 && timeout > 0 { timeout -= 1; udelay(1); }
    if atomic_read(&waiting_for_crash_ipi) > 0 { pr_warn!("SMP: failed to stop secondary CPUs %*pbl\n", cpumask_pr_args(&mask)); }
}

#[cfg(CONFIG_KEXEC_CORE)]
pub unsafe extern "C" fn smp_crash_stop_failed() -> bool { atomic_read(&waiting_for_crash_ipi) > 0 }

pub unsafe extern "C" fn arch_smp_send_reschedule(cpu: i32) { send_ipi_single(cpu, IPI_RESCHEDULE); }

unsafe fn riscv_backtrace_ipi(mask: *mut cpumask) { send_ipi_mask(mask, IPI_CPU_BACKTRACE); }
pub unsafe extern "C" fn arch_trigger_cpumask_backtrace(mask: *const cpumask, exclude_cpu: i32) { nmi_trigger_cpumask_backtrace(mask, exclude_cpu, riscv_backtrace_ipi); }

#[cfg(CONFIG_KGDB)]
pub unsafe extern "C" fn kgdb_roundup_cpus() {
    let this_cpu = raw_smp_processor_id();
    for_each_online_cpu!(cpu => {
        if cpu != this_cpu { send_ipi_single(cpu, IPI_KGDB_ROUNDUP); }
    });
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
