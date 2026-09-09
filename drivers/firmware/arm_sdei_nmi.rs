// SPDX-License-Identifier: GPL-2.0
/*
 * arm64 SDEI-based cross-CPU NMI service.
 *
 * Delivering an "NMI-shaped" event to an EL1 context that has locally
 * masked interrupts, on silicon without FEAT_NMI, can be done two ways:
 *
 *   - pseudo-NMI: mask "interrupts" via the GIC priority register
 *     (ICC_PMR_EL1) instead of PSTATE.DAIF, leaving a high-priority band
 *     deliverable. Functionally this works -- but it reimplements every
 *     local_irq_disable()/enable() and exception entry/exit as a PMR
 *     write plus synchronisation, a cost paid on that hot path forever,
 *     whether or not an NMI is ever delivered.
 *
 *   - SDEI: leave interrupt masking as the cheap PSTATE.DAIF operation
 *     and have the firmware bounce an EL3-routed Group-0 SGI back to
 *     NS-EL1 as an event callback. The cost is a firmware round-trip,
 *     but only at the rare moment delivery is actually needed.
 *
 * This driver takes the second path: it keeps the IRQ-mask hot path
 * free and pays only when it fires, which is what makes cross-CPU NMI
 * affordable on hardware where the pseudo-NMI tax isn't, until FEAT_NMI
 * makes NMI masking cheap in the architecture itself.
 *
 * Capabilities provided:
 *
 *   - sdei_nmi_trigger_cpumask_backtrace() — override for arm64's
 *     arch_trigger_cpumask_backtrace(), so sysrq-l, RCU stall dumps,
 *     hardlockup_all_cpu_backtrace, soft-lockup/hung-task secondary
 *     dumps all reach interrupt-masked CPUs.
 *
 *   - sdei_nmi_stop_cpus() — the last rung of smp_send_stop()'s
 *     escalation (reboot/halt and the panic/kdump crash stop alike),
 *     reaching CPUs that ignored the stop IPIs; on the kdump path the
 *     wedged context is captured into the vmcore before the CPU parks.
 */

// Dependencies supplied by the surrounding kernel translation.

static mut SDEI_NMI_AVAILABLE: bool = false;
const SDEI_NMI_EVENT: u32 = 0;
static mut SDEI_NMI_STOPPING: bool = false;

unsafe fn sdei_nmi_handler(event: u32, regs: *mut pt_regs, arg: *mut core::ffi::c_void) -> i32 {
    /*
     * No smp_rmb() pairing sdei_nmi_stop_cpus()'s dsb(ishst): the flag is
     * the only shared value, and this handler runs only because firmware
     * delivered the event -- a round-trip past that store -- so the read
     * cannot be stale and there is no second load for a barrier to order.
     */
    if core::ptr::read_volatile(core::ptr::addr_of!(SDEI_NMI_STOPPING)) {
        /* unreachable after arm64_nmi_cpu_stop */
        arm64_nmi_cpu_stop(regs, false);
    }

    nmi_cpu_backtrace(regs);
    SDEI_EV_HANDLED
}

unsafe fn sdei_nmi_fire(target_cpu: u32) {
    let err: i32 = sdei_event_signal(SDEI_NMI_EVENT, cpu_logical_map(target_cpu));

    if err != 0 {
        pr_warn("SDEI_EVENT_SIGNAL to CPU %u failed: %d\n", target_cpu, err);
    }
}

/*
 * Raise callback for nmi_trigger_cpumask_backtrace(): signal event 0
 * at every CPU still pending in @mask. The framework excludes the local
 * CPU from @mask before calling us.
 */
unsafe fn sdei_nmi_raise_backtrace(mask: *mut cpumask_t) {
    let mut cpu: u32;

    dsb(ishst);

    for_each_cpu!(cpu, mask, {
        sdei_nmi_fire(cpu);
    });
}

pub unsafe fn sdei_nmi_trigger_cpumask_backtrace(mask: *const cpumask_t, exclude_cpu: i32) -> bool {
    if !SDEI_NMI_AVAILABLE {
        return false;
    }

    nmi_trigger_cpumask_backtrace(mask, exclude_cpu, sdei_nmi_raise_backtrace);
    true
}

pub unsafe fn sdei_nmi_active() -> bool {
    SDEI_NMI_AVAILABLE
}

/*
 * Last rung of the stop escalation in smp_send_stop(). The caller runs
 * the regular stop IPI first; @mask holds whatever stayed online through
 * those -- typically CPUs wedged with interrupts masked, unreachable by
 * an IPI. Mark the stop in progress and signal event 0 at each target.
 */
pub unsafe fn sdei_nmi_stop_cpus(mask: *const cpumask_t) {
    WRITE_ONCE!(SDEI_NMI_STOPPING, true);
    dsb(ishst);

    for_each_cpu!(cpu, mask, {
        sdei_nmi_fire(cpu);
    });
}

/* device_initcall (after arch_initcall(sdei_init)): probe and enable SDEI. */
unsafe fn sdei_nmi_init() -> i32 {
    let mut err: i32;

    if !sdei_is_present() {
        return 0;
    }

    err = sdei_event_register(SDEI_NMI_EVENT, sdei_nmi_handler, core::ptr::null_mut());
    if err != 0 {
        pr_err("sdei_event_register(%u) failed: %d\n", SDEI_NMI_EVENT, err);
        return 0;
    }

    err = sdei_event_enable(SDEI_NMI_EVENT);
    if err != 0 {
        pr_err("sdei_event_enable(%u) failed: %d\n", SDEI_NMI_EVENT, err);
        sdei_event_unregister(SDEI_NMI_EVENT);
        return 0;
    }

    SDEI_NMI_AVAILABLE = true;
    pr_info("using SDEI cross-CPU NMI (SDEI_EVENT_SIGNAL, event %u)\n", SDEI_NMI_EVENT);
    0
}

// device_initcall(sdei_nmi_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
