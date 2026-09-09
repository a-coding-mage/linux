// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by Linux IRQ headers and internals.h are referenced
// directly below.

/**
 * irq_fixup_move_pending - Cleanup irq move pending from a dying CPU
 * @desc:              Interrupt descriptor to clean up
 * @force_clear:       If set clear the move pending bit unconditionally.
 *                     If not set, clear it only when the dying CPU is the
 *                     last one in the pending mask.
 *
 * Returns true if the pending bit was set and the pending mask contains an
 * online CPU other than the dying CPU.
 */
pub unsafe fn irq_fixup_move_pending(desc: *mut irq_desc, force_clear: bool) -> bool {
    let data: *mut irq_data = irq_desc_get_irq_data(desc);

    if !irqd_is_setaffinity_pending(data) {
        return false;
    }

    /*
     * The outgoing CPU might be the last online target in a pending
     * interrupt move. If that's the case clear the pending move bit.
     */
    if !cpumask_intersects((*desc).pending_mask, cpu_online_mask) {
        irqd_clr_move_pending(data);
        return false;
    }
    if force_clear {
        irqd_clr_move_pending(data);
    }
    true
}

pub unsafe fn irq_force_complete_move(desc: *mut irq_desc) {
    let mut d: *mut irq_data = irq_desc_get_irq_data(desc);
    while !d.is_null() {
        if !(*d).chip.is_null() && (*(*d).chip).irq_force_complete_move.is_some() {
            ((*(*d).chip).irq_force_complete_move.unwrap())(d);
            return;
        }
        d = irqd_get_parent_data(d);
    }
}

pub unsafe fn irq_move_masked_irq(idata: *mut irq_data) {
    let desc: *mut irq_desc = irq_data_to_desc(idata);
    let data: *mut irq_data = &mut (*desc).irq_data;
    let chip: *mut irq_chip = (*data).chip;

    if likely(!irqd_is_setaffinity_pending(data)) {
        return;
    }

    irqd_clr_move_pending(data);

    /* Paranoia: cpu-local interrupts shouldn't be calling in here anyway. */
    if irqd_is_per_cpu(data) {
        WARN_ON(1);
        return;
    }

    if unlikely(cpumask_empty((*desc).pending_mask)) {
        return;
    }

    if (*chip).irq_set_affinity.is_none() {
        return;
    }

    assert_raw_spin_locked(&mut (*desc).lock);

    /*
     * If there was a valid mask to work with, please do the disable,
     * re-program, enable sequence. This is *not* particularly important for
     * level triggered but in an edge trigger case, we might be setting rte
     * when an active trigger is coming in. This could cause some ioapics to
     * mal-function. Being paranoid i guess!
     *
     * For correct operation this depends on the caller masking the irqs.
     */
    if cpumask_intersects((*desc).pending_mask, cpu_online_mask) {
        let ret: i32 = irq_do_set_affinity(data, (*desc).pending_mask, false);
        /*
         * If there is a cleanup pending in the underlying vector management,
         * reschedule the move for the next interrupt. Leave pending_mask
         * intact.
         */
        if ret == -EBUSY {
            irqd_set_move_pending(data);
            return;
        }
    }
    cpumask_clear((*desc).pending_mask);
}

pub unsafe fn __irq_move_irq(mut idata: *mut irq_data) {
    let masked: bool;

    /* Get top level irq_data when CONFIG_IRQ_DOMAIN_HIERARCHY is enabled. */
    idata = irq_desc_get_irq_data(irq_data_to_desc(idata));

    if unlikely(irqd_irq_disabled(idata)) {
        return;
    }

    /* Be careful vs. already masked interrupts. */
    masked = irqd_irq_masked(idata);
    if !masked {
        ((*(*idata).chip).irq_mask.unwrap())(idata);
    }
    irq_move_masked_irq(idata);
    if !masked {
        ((*(*idata).chip).irq_unmask.unwrap())(idata);
    }
}

pub unsafe fn irq_can_move_in_process_context(mut data: *mut irq_data) -> bool {
    /* Get the top level irq_data in the hierarchy. */
    data = irq_desc_get_irq_data(irq_data_to_desc(data));
    irq_can_move_pcntxt(data)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
