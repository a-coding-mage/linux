// SPDX-License-Identifier: GPL-2.0
/*
 *  NMI backtrace support
 *
 * Gratuitously copied from arch/x86/kernel/apic/hw_nmi.c by Russell King,
 * with the following header:
 *
 *  HW NMI watchdog support
 *
 *  started by Don Zickus, Copyright (C) 2010 Red Hat, Inc.
 *
 *  Arch specific calls to support NMI watchdog
 *
 *  Bits copied from original nmi.c file
 */

// The C implementation is conditionally compiled when
// `arch_trigger_cpumask_backtrace` is provided by the architecture.

/* For reliability, we're prepared to waste bits here. */
static mut backtrace_mask: [usize; (NR_CPUS + usize::BITS as usize - 1) / usize::BITS as usize] =
    [0; (NR_CPUS + usize::BITS as usize - 1) / usize::BITS as usize];

/* "in progress" flag of arch_trigger_cpumask_backtrace */
static mut backtrace_flag: core::ffi::c_ulong = 0;

const NMI_BT_TIMEOUT_SEC: i32 = 10;

/*
 * When raise() is called it will be passed a pointer to the
 * backtrace_mask. Architectures that call nmi_cpu_backtrace()
 * directly from their raise() functions may rely on the mask
 * they are passed being updated as a side effect of this call.
 */
pub unsafe extern "C" fn nmi_trigger_cpumask_backtrace(
    mask: *const cpumask_t,
    exclude_cpu: i32,
    raise: Option<unsafe extern "C" fn(*mut cpumask_t)>,
) {
    let mut i: i32;
    let this_cpu = get_cpu();

    if test_and_set_bit(0, &raw mut backtrace_flag) {
        /*
         * If there is already a trigger_all_cpu_backtrace() in progress
         * (backtrace_flag == 1), don't output double cpu dump infos.
         */
        put_cpu();
        return;
    }

    cpumask_copy(to_cpumask(&raw mut backtrace_mask), mask);
    if exclude_cpu != -1 {
        cpumask_clear_cpu(exclude_cpu, to_cpumask(&raw mut backtrace_mask));
    }

    /*
     * Don't try to send an NMI to this cpu; it may work on some
     * architectures, but on others it may not, and we'll get
     * information at least as useful just by doing a dump_stack() here.
     * Note that nmi_cpu_backtrace(NULL) will clear the cpu bit.
     */
    if cpumask_test_cpu(this_cpu, to_cpumask(&raw mut backtrace_mask)) {
        nmi_cpu_backtrace(core::ptr::null_mut());
    }

    if !cpumask_empty(to_cpumask(&raw mut backtrace_mask)) {
        pr_info!("Sending NMI from CPU %d to CPUs %*pbl:\n",
            this_cpu, nr_cpumask_bits, to_cpumask(&raw mut backtrace_mask));
        nmi_backtrace_stall_snap(to_cpumask(&raw mut backtrace_mask));
        if let Some(raise) = raise {
            raise(to_cpumask(&raw mut backtrace_mask));
        }
    }

    /* Wait for up to NMI_BT_TIMEOUT_SEC seconds for all CPUs to do the backtrace */
    i = 0;
    while i < NMI_BT_TIMEOUT_SEC * 1000 {
        if cpumask_empty(to_cpumask(&raw mut backtrace_mask)) {
            break;
        }
        mdelay(1);
        touch_softlockup_watchdog();
        i += 1;
    }

    if !cpumask_empty(to_cpumask(&raw mut backtrace_mask)) {
        pr_warn!("After {} seconds, these CPUS still haven't responded to the NMI: %*pbl\n",
            NMI_BT_TIMEOUT_SEC, cpumask_pr_args(to_cpumask(&raw mut backtrace_mask)));

        nmi_backtrace_stall_check(to_cpumask(&raw mut backtrace_mask));
    }

    /*
     * Force flush any remote buffers that might be stuck in IRQ context
     * and therefore could not run their irq_work.
     */
    printk_trigger_flush();

    clear_bit_unlock(0, &raw mut backtrace_flag);
    put_cpu();
}

// Dump stacks even for idle CPUs.
static mut backtrace_idle: bool = false;

pub unsafe extern "C" fn nmi_cpu_backtrace(regs: *mut pt_regs) -> bool {
    let cpu = smp_processor_id();
    let mut flags: core::ffi::c_ulong = 0;

    if cpumask_test_cpu(cpu, to_cpumask(&raw mut backtrace_mask)) {
        /*
         * Allow nested NMI backtraces while serializing
         * against other CPUs.
         */
        printk_cpu_sync_get_irqsave(&mut flags);
        if !core::ptr::read_volatile(&backtrace_idle)
            && !regs.is_null()
            && cpu_in_idle(instruction_pointer(regs))
        {
            pr_warn!("NMI backtrace for cpu %d skipped: idling at %pS\n",
                cpu, instruction_pointer(regs));
        } else {
            pr_warn!("NMI backtrace for cpu %d\n", cpu);
            if !regs.is_null() {
                show_regs(regs);
            } else {
                dump_stack();
            }
        }
        printk_cpu_sync_put_irqrestore(flags);
        cpumask_clear_cpu(cpu, to_cpumask(&raw mut backtrace_mask));
        return true;
    }

    false
}

pub unsafe extern "C" fn cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: i32) {
    arch_trigger_cpumask_backtrace(mask, exclude_cpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
