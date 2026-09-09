// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 1992, 1998-2006 Linus Torvalds, Ingo Molnar
 * Copyright (C) 2005-2006, Thomas Gleixner, Russell King
 *
 * This file contains the core interrupt handling code. Detailed
 * information is available in Documentation/core-api/genericirq.rst
 */

// Linux kernel headers and local internals supplied by other translation units.

#[cfg(CONFIG_GENERIC_IRQ_MULTI_HANDLER)]
pub static mut handle_arch_irq: Option<unsafe extern "C" fn(*mut pt_regs)> = None;

/**
 * handle_bad_irq - handle spurious and unhandled irqs
 * @desc:      description of the interrupt
 *
 * Handles spurious and unhandled IRQ's. It also prints a debugmessage.
 */
pub unsafe extern "C" fn handle_bad_irq(desc: *mut irq_desc) {
    let irq: c_uint = irq_desc_get_irq(desc);

    print_irq_desc(irq, desc);
    kstat_incr_irqs_this_cpu(desc);
    ack_bad_irq(irq);
}

/*
 * Special, empty irq handler:
 */
pub unsafe extern "C" fn no_action(_cpl: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    IRQ_NONE
}

unsafe fn warn_no_thread(irq: c_uint, action: *mut irqaction) {
    if test_and_set_bit(IRQTF_WARNED, &mut (*action).thread_flags) != 0 {
        return;
    }

    printk(
        KERN_WARNING,
        "IRQ %d device %s returned IRQ_WAKE_THREAD but no thread function available.",
        irq,
        (*action).name,
    );
}

pub unsafe extern "C" fn __irq_wake_thread(desc: *mut irq_desc, action: *mut irqaction) {
    if (*(*action).thread).flags & PF_EXITING != 0 {
        return;
    }

    if test_and_set_bit(IRQTF_RUNTHREAD, &mut (*action).thread_flags) != 0 {
        return;
    }

    (*desc).threads_oneshot |= (*action).thread_mask;
    atomic_inc(&mut (*desc).threads_active);
    wake_up_state((*action).thread, TASK_INTERRUPTIBLE);
}

static mut irqhandler_duration_check_enabled: bool = false;
static mut irqhandler_duration_threshold_ns: u64 = 0;

unsafe extern "C" fn irqhandler_duration_check_setup(arg: *mut c_char) -> c_int {
    let mut val: c_ulong = 0;
    let ret = kstrtoul(arg, 0, &mut val);
    if ret != 0 {
        pr_err(
            "Unable to parse irqhandler.duration_warn_us setting: ret=%d\n",
            ret,
        );
        return 0;
    }

    if val == 0 {
        pr_err("Invalid irqhandler.duration_warn_us setting, must be > 0\n");
        return 0;
    }

    irqhandler_duration_threshold_ns = val * 1000;
    static_branch_enable(&mut irqhandler_duration_check_enabled);

    1
}

unsafe fn irqhandler_duration_check(
    ts_start: u64,
    irq: c_uint,
    action: *const irqaction,
) {
    let delta_ns = local_clock() - ts_start;

    if delta_ns > irqhandler_duration_threshold_ns {
        pr_warn_ratelimited(
            "[CPU%u] long duration of IRQ[%u:%ps], took: %llu us\n",
            smp_processor_id(),
            irq,
            (*action).handler,
            div_u64(delta_ns, NSEC_PER_USEC),
        );
    }
}

pub unsafe extern "C" fn __handle_irq_event_percpu(desc: *mut irq_desc) -> irqreturn_t {
    let mut retval: irqreturn_t = IRQ_NONE;
    let irq: c_uint = (*desc).irq_data.irq;
    let mut action: *mut irqaction = core::ptr::null_mut();

    // for_each_action_of_desc(desc, action)
    while for_each_action_of_desc(desc, &mut action) {
        let res: irqreturn_t;

        if irq_settings_can_thread(desc)
            && (*action).flags & (IRQF_NO_THREAD | IRQF_PERCPU | IRQF_ONESHOT) == 0
        {
            lockdep_hardirq_threaded();
        }

        trace_irq_handler_entry(irq, action);

        if static_branch_unlikely(&irqhandler_duration_check_enabled) {
            let ts_start = local_clock();
            res = ((*action).handler)(irq, (*action).dev_id);
            irqhandler_duration_check(ts_start, irq, action);
        } else {
            res = ((*action).handler)(irq, (*action).dev_id);
        }

        trace_irq_handler_exit(irq, action, res);

        if WARN_ONCE(
            !irqs_disabled(),
            "irq %u handler %pS enabled interrupts\n",
            irq,
            (*action).handler,
        ) {
            local_irq_disable();
        }

        if res == IRQ_WAKE_THREAD {
            if (*action).thread_fn.is_none() {
                warn_no_thread(irq, action);
            } else {
                __irq_wake_thread(desc, action);
            }
        }

        retval |= res;
    }

    retval
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
