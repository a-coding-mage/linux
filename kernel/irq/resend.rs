// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 1992, 1998-2006 Linus Torvalds, Ingo Molnar
 * Copyright (C) 2005-2006, Thomas Gleixner
 *
 * This file contains the IRQ-resend code
 *
 * If the interrupt is waiting to be processed, we try to re-run it.
 * We can't directly run it from here since the caller might be in an
 * interrupt-protected region. Not all irq controller chips can
 * retrigger interrupts at the hardware level, so in those cases
 * we allow the resending of IRQs via a tasklet.
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[cfg(CONFIG_HARDIRQS_SW_RESEND)]
static mut IRQ_RESEND_LIST: HListHead = HListHead::new();
#[cfg(CONFIG_HARDIRQS_SW_RESEND)]
static mut IRQ_RESEND_LOCK: RawSpinLock = RawSpinLock::new();

#[cfg(CONFIG_HARDIRQS_SW_RESEND)]
unsafe fn resend_irqs(_unused: *mut TaskletStruct) {
    let _guard = RawSpinlockIrqGuard::new(&raw mut IRQ_RESEND_LOCK);
    while !hlist_empty(&raw mut IRQ_RESEND_LIST) {
        let desc: *mut IrqDesc;

        desc = hlist_entry((*(&raw mut IRQ_RESEND_LIST)).first, resend_node);
        hlist_del_init(&raw mut (*desc).resend_node);

        raw_spin_unlock(&raw mut IRQ_RESEND_LOCK);
        ((*desc).handle_irq)(desc);
        raw_spin_lock(&raw mut IRQ_RESEND_LOCK);
    }
}

#[cfg(CONFIG_HARDIRQS_SW_RESEND)]
static mut RESEND_TASKLET: TaskletStruct = declare_tasklet!(resend_irqs);

#[cfg(CONFIG_HARDIRQS_SW_RESEND)]
unsafe fn irq_sw_resend(mut desc: *mut IrqDesc) -> c_int {
    /*
     * Validate whether this interrupt can be safely injected from
     * non interrupt context
     */
    if irqd_is_handle_enforce_irqctx(&raw mut (*desc).irq_data) {
        return -EINVAL;
    }

    /*
     * If the interrupt is running in the thread context of the parent
     * irq we need to be careful, because we cannot trigger it
     * directly.
     */
    if irq_settings_is_nested_thread(desc) {
        /* If the parent_irq is valid, we retrigger the parent, otherwise do nothing. */
        if (*desc).parent_irq == 0 {
            return -EINVAL;
        }

        desc = irq_to_desc((*desc).parent_irq);
        if desc.is_null() {
            return -EINVAL;
        }
    }

    /* Add to resend_list and activate the softirq: */
    let _guard = RawSpinlockGuard::new(&raw mut IRQ_RESEND_LOCK);
    if hlist_unhashed(&raw mut (*desc).resend_node) {
        hlist_add_head(&raw mut (*desc).resend_node, &raw mut IRQ_RESEND_LIST);
    }
    tasklet_schedule(&raw mut RESEND_TASKLET);
    0
}

#[cfg(CONFIG_HARDIRQS_SW_RESEND)]
unsafe fn clear_irq_resend(desc: *mut IrqDesc) {
    let _guard = RawSpinlockGuard::new(&raw mut IRQ_RESEND_LOCK);
    hlist_del_init(&raw mut (*desc).resend_node);
}

#[cfg(CONFIG_HARDIRQS_SW_RESEND)]
unsafe fn irq_resend_init(desc: *mut IrqDesc) {
    INIT_HLIST_NODE(&raw mut (*desc).resend_node);
}

#[cfg(not(CONFIG_HARDIRQS_SW_RESEND))]
unsafe fn clear_irq_resend(_desc: *mut IrqDesc) {}
#[cfg(not(CONFIG_HARDIRQS_SW_RESEND))]
unsafe fn irq_resend_init(_desc: *mut IrqDesc) {}

#[cfg(not(CONFIG_HARDIRQS_SW_RESEND))]
unsafe fn irq_sw_resend(_desc: *mut IrqDesc) -> c_int {
    -EINVAL
}

unsafe fn try_retrigger(desc: *mut IrqDesc) -> c_int {
    if let Some(retrigger) = (*(*desc).irq_data.chip).irq_retrigger {
        return retrigger(&raw mut (*desc).irq_data);
    }

    #[cfg(CONFIG_IRQ_DOMAIN_HIERARCHY)]
    {
        return irq_chip_retrigger_hierarchy(&raw mut (*desc).irq_data);
    }
    #[cfg(not(CONFIG_IRQ_DOMAIN_HIERARCHY))]
    {
        0
    }
}

/*
 * IRQ resend
 *
 * Is called with interrupts disabled and desc->lock held.
 */
unsafe fn check_irq_resend(desc: *mut IrqDesc, inject: bool) -> c_int {
    let mut err: c_int = 0;

    /* Do not resend level type interrupts. */
    if irq_settings_is_level(desc) {
        (*desc).istate &= !IRQS_PENDING;
        return -EINVAL;
    }

    if (*desc).istate & IRQS_REPLAY != 0 {
        return -EBUSY;
    }

    if (*desc).istate & IRQS_PENDING == 0 && !inject {
        return 0;
    }

    (*desc).istate &= !IRQS_PENDING;

    if try_retrigger(desc) == 0 {
        err = irq_sw_resend(desc);
    }

    if err == 0 {
        (*desc).istate |= IRQS_REPLAY;
    }
    err
}

#[cfg(CONFIG_GENERIC_IRQ_INJECTION)]
unsafe fn irq_inject_interrupt(irq: c_uint) -> c_int {
    let mut err: c_int = -EINVAL;

    /* Try the state injection hardware interface first */
    if !irq_set_irqchip_state(irq, IRQCHIP_STATE_PENDING, true) {
        return 0;
    }

    /* That failed, try via the resend mechanism */
    let _scope = scoped_irqdesc_get_and_buslock(irq, 0);
    let desc = scoped_irqdesc;

    /* Only try to inject when the interrupt is not NMI type and activated. */
    if !irq_is_nmi(desc) && irqd_is_activated(&raw mut (*desc).irq_data) {
        err = check_irq_resend(desc, true);
    }
    err
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
