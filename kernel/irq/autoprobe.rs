// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 1992, 1998-2004 Linus Torvalds, Ingo Molnar
 *
 * This file contains the interrupt probing code and driver APIs.
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * Autodetection depends on the fact that any interrupt that
 * comes in on to an unassigned handler will get stuck with
 * "IRQS_WAITING" cleared and the interrupt disabled.
 */
static probing_active: Mutex = DEFINE_MUTEX!();

/**
 * probe_irq_on - begin an interrupt autodetect
 *
 * Commence probing for an interrupt. The interrupts are scanned
 * and a mask of potential interrupt lines is returned.
 */
pub unsafe fn probe_irq_on() -> c_ulong {
    let mut desc: *mut irq_desc;
    let mut mask: c_ulong = 0;
    let mut i: c_int;

    /* quiesce the kernel, or at least the asynchronous portion */
    async_synchronize_full();
    mutex_lock(&probing_active);

    /*
     * something may have generated an irq long ago and we want
     * to flush such a longstanding irq before considering it as spurious.
     */
    for_each_irq_desc_reverse!(i, desc, {
        guard!(raw_spinlock_irq, (*desc).lock);
        if (*desc).action.is_null() && irq_settings_can_probe(desc) {
            /* Some chips need to know about probing in progress. */
            if !(*(*desc).irq_data.chip).irq_set_type.is_none() {
                ((*(*desc).irq_data.chip).irq_set_type.unwrap())(
                    &mut (*desc).irq_data,
                    IRQ_TYPE_PROBE,
                );
            }
            irq_activate_and_startup(desc, IRQ_NORESEND);
        }
    });

    /* Wait for longstanding interrupts to trigger. */
    msleep(20);

    /*
     * enable any unassigned irqs
     * (we must startup again here because if a longstanding irq
     * happened in the previous stage, it may have masked itself)
     */
    for_each_irq_desc_reverse!(i, desc, {
        guard!(raw_spinlock_irq, (*desc).lock);
        if (*desc).action.is_null() && irq_settings_can_probe(desc) {
            (*desc).istate |= IRQS_AUTODETECT | IRQS_WAITING;
            if irq_activate_and_startup(desc, IRQ_NORESEND) != 0 {
                (*desc).istate |= IRQS_PENDING;
            }
        }
    });

    /* Wait for spurious interrupts to trigger. */
    msleep(100);

    /* Now filter out any obviously spurious interrupts. */
    for_each_irq_desc!(i, desc, {
        guard!(raw_spinlock_irq, (*desc).lock);
        if (*desc).istate & IRQS_AUTODETECT != 0 {
            /* It triggered already - consider it spurious. */
            if (*desc).istate & IRQS_WAITING == 0 {
                (*desc).istate &= !IRQS_AUTODETECT;
                irq_shutdown_and_deactivate(desc);
            } else if i < 32 {
                mask |= 1 << i;
            }
        }
    });

    mask
}

EXPORT_SYMBOL!(probe_irq_on);

/**
 * probe_irq_mask - scan a bitmap of interrupt lines
 * @val: mask of interrupts to consider
 *
 * Scan the interrupt lines and return a bitmap of active autodetect
 * interrupts. The interrupt probe logic state is then returned to its
 * previous value.
 */
pub unsafe fn probe_irq_mask(val: c_ulong) -> c_uint {
    let mut mask: c_uint = 0;
    let mut desc: *mut irq_desc;
    let mut i: c_int;

    for_each_irq_desc!(i, desc, {
        guard!(raw_spinlock_irq, (*desc).lock);
        if (*desc).istate & IRQS_AUTODETECT != 0 {
            if i < 16 && (*desc).istate & IRQS_WAITING == 0 {
                mask |= 1 << i;
            }
            (*desc).istate &= !IRQS_AUTODETECT;
            irq_shutdown_and_deactivate(desc);
        }
    });
    mutex_unlock(&probing_active);

    mask & val as c_uint
}

EXPORT_SYMBOL!(probe_irq_mask);

/**
 * probe_irq_off - end an interrupt autodetect
 * @val: mask of potential interrupts (unused)
 *
 * Scans the unused interrupt lines and returns the line which appears to
 * have triggered the interrupt. If no interrupt was found then zero is
 * returned. If more than one interrupt is found then minus the first
 * candidate is returned to indicate their is doubt.
 */
pub unsafe fn probe_irq_off(_val: c_ulong) -> c_int {
    let mut irq_found: c_int = 0;
    let mut nr_of_irqs: c_int = 0;
    let mut desc: *mut irq_desc;
    let mut i: c_int;

    for_each_irq_desc!(i, desc, {
        guard!(raw_spinlock_irq, (*desc).lock);
        if (*desc).istate & IRQS_AUTODETECT != 0 {
            if (*desc).istate & IRQS_WAITING == 0 {
                if nr_of_irqs == 0 {
                    irq_found = i;
                }
                nr_of_irqs += 1;
            }
            (*desc).istate &= !IRQS_AUTODETECT;
            irq_shutdown_and_deactivate(desc);
        }
    });
    mutex_unlock(&probing_active);

    if nr_of_irqs > 1 {
        irq_found = -irq_found;
    }

    irq_found
}

EXPORT_SYMBOL!(probe_irq_off);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
