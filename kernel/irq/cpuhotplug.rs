// SPDX-License-Identifier: GPL-2.0
/*
 * Generic cpu hotunplug interrupt migration code copied from the
 * arch/arm implementation
 *
 * Copyright (C) Russell King
 */

// C dependencies are supplied by the surrounding kernel translation unit.

#[inline]
unsafe fn irq_needs_fixup(d: *mut irq_data) -> bool {
    let mut m = irq_data_get_effective_affinity_mask(d);
    let cpu = smp_processor_id();

    // CONFIG_GENERIC_IRQ_EFFECTIVE_AFF_MASK
    if cpumask_empty(m) {
        m = irq_data_get_affinity_mask(d);
    }
    if cpumask_any_but(m, cpu) < nr_cpu_ids
        && !cpumask_intersects(m, cpu_online_mask)
    {
        pr_warn("Eff. affinity of IRQ {} contains only offline CPUs after offlining CPU {}\n", (*d).irq, cpu);
        return true;
    }
    cpumask_test_cpu(cpu, m)
}

unsafe fn migrate_one_irq(desc: *mut irq_desc) -> bool {
    let d = irq_desc_get_irq_data(desc);
    let chip = irq_data_get_irq_chip(d);
    let maskchip = !irq_can_move_pcntxt(d) && !irqd_irq_masked(d);
    let mut affinity: *const cpumask;
    let mut brokeaff = false;

    if chip.is_null() || (*chip).irq_set_affinity.is_none() {
        pr_debug("IRQ {}: Unable to migrate away\n", (*d).irq);
        return false;
    }

    irq_force_complete_move(desc);

    if irqd_is_per_cpu(d) || !irqd_is_started(d) || !irq_needs_fixup(d) {
        irq_fixup_move_pending(desc, false);
        return false;
    }

    if irq_fixup_move_pending(desc, true) {
        affinity = irq_desc_get_pending_mask(desc);
    } else {
        affinity = irq_data_get_affinity_mask(d);
    }

    if maskchip && (*chip).irq_mask.is_some() {
        ((*chip).irq_mask.unwrap())(d);
    }

    if !cpumask_intersects(affinity, cpu_online_mask) {
        if irqd_affinity_is_managed(d) {
            irqd_set_managed_shutdown(d);
            irq_shutdown_and_deactivate(desc);
            return false;
        }
        affinity = cpu_online_mask;
        brokeaff = true;
    }

    let mut err = irq_do_set_affinity(d, affinity, false);
    if err == -ENOSPC && !irqd_affinity_is_managed(d) && affinity != cpu_online_mask {
        pr_debug("IRQ{}: set affinity failed, re-try with online CPUs\n", (*d).irq);
        affinity = cpu_online_mask;
        brokeaff = true;
        err = irq_do_set_affinity(d, affinity, false);
    }

    if err != 0 {
        pr_warn_ratelimited("IRQ{}: set affinity failed({}).\n", (*d).irq, err);
        brokeaff = false;
    }

    if maskchip && (*chip).irq_unmask.is_some() {
        ((*chip).irq_unmask.unwrap())(d);
    }
    brokeaff
}

pub unsafe fn irq_migrate_all_off_this_cpu() {
    let mut desc: *mut irq_desc;
    let mut irq: u32;

    // for_each_active_irq(irq)
    for irq in for_each_active_irq() {
        desc = irq_to_desc(irq);
        raw_spin_lock(&mut (*desc).lock);
        let affinity_broken = migrate_one_irq(desc);
        if affinity_broken && !(*desc).affinity_notify.is_null() {
            irq_affinity_schedule_notify_work(desc);
        }
        raw_spin_unlock(&mut (*desc).lock);
        if affinity_broken {
            pr_debug_ratelimited("IRQ {}: no longer affine to CPU{}\n", irq, smp_processor_id());
        }
    }
}

unsafe fn hk_should_isolate(data: *mut irq_data, cpu: u32) -> bool {
    if !housekeeping_enabled(HK_TYPE_MANAGED_IRQ) {
        return false;
    }
    let hk_mask = housekeeping_cpumask(HK_TYPE_MANAGED_IRQ);
    if cpumask_subset(irq_data_get_effective_affinity_mask(data), hk_mask) {
        return false;
    }
    cpumask_test_cpu(cpu, hk_mask)
}

unsafe fn irq_restore_affinity_of_irq(desc: *mut irq_desc, cpu: u32) {
    let data = irq_desc_get_irq_data(desc);
    let affinity = irq_data_get_affinity_mask(data);
    if !irqd_affinity_is_managed(data) || (*desc).action.is_null()
        || irq_data_get_irq_chip(data).is_null() || !cpumask_test_cpu(cpu, affinity)
    {
        return;
    }
    if irqd_is_managed_and_shutdown(data) {
        irq_startup_managed(desc);
    }
    if !irqd_is_single_target(data) || hk_should_isolate(data, cpu) {
        irq_set_affinity_locked(data, affinity, false);
    }
}

pub unsafe fn irq_affinity_online_cpu(cpu: u32) -> i32 {
    irq_lock_sparse();
    // for_each_active_irq(irq)
    for irq in for_each_active_irq() {
        let desc = irq_to_desc(irq);
        raw_spin_lock_irq(&mut (*desc).lock);
        irq_restore_affinity_of_irq(desc, cpu);
        raw_spin_unlock_irq(&mut (*desc).lock);
    }
    irq_unlock_sparse();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
