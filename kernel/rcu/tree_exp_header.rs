/* SPDX-License-Identifier: GPL-2.0+ */
/* RCU expedited grace periods.  Translated from tree_exp.h. */

/* External kernel declarations and macros are supplied by other translated files. */

unsafe fn rcu_exp_gp_seq_start() {
    rcu_seq_start(&mut rcu_state.expedited_sequence);
    rcu_poll_gp_seq_start_unlocked(&mut rcu_state.gp_seq_polled_exp_snap);
}

#[allow(dead_code)]
unsafe fn rcu_exp_gp_seq_endval() -> c_ulong { rcu_seq_endval(&rcu_state.expedited_sequence) }

unsafe fn rcu_exp_gp_seq_end() {
    rcu_poll_gp_seq_end_unlocked(&mut rcu_state.gp_seq_polled_exp_snap);
    rcu_seq_end(&mut rcu_state.expedited_sequence);
    smp_mb();
}

unsafe fn rcu_exp_gp_seq_snap() -> c_ulong {
    smp_mb();
    let s = rcu_seq_snap(&rcu_state.expedited_sequence);
    trace_rcu_exp_grace_period(rcu_state.name, s, TPS!("snap"));
    s
}

unsafe fn rcu_exp_gp_seq_done(s: c_ulong) -> bool { rcu_seq_done(&rcu_state.expedited_sequence, s) }

unsafe fn sync_exp_reset_tree_hotplug() {
    let ncpus = smp_load_acquire(&rcu_state.ncpus);
    if likely(ncpus == rcu_state.ncpus_snap) { return; }
    rcu_state.ncpus_snap = ncpus;
    /* rcu_for_each_leaf_node(rnp) */
    for rnp in rcu_leaf_nodes() {
        let mut flags = 0;
        raw_spin_lock_irqsave_rcu_node(rnp, &mut flags);
        if rnp.expmaskinit == rnp.expmaskinitnext {
            raw_spin_unlock_irqrestore_rcu_node(rnp, flags);
            continue;
        }
        let oldmask = rnp.expmaskinit;
        rnp.expmaskinit = rnp.expmaskinitnext;
        raw_spin_unlock_irqrestore_rcu_node(rnp, flags);
        if oldmask != 0 { continue; }
        let mut mask = rnp.grpmask;
        let mut rnp_up = rnp.parent;
        let mut done = false;
        while !rnp_up.is_null() {
            raw_spin_lock_irqsave_rcu_node(&mut *rnp_up, &mut flags);
            if (*rnp_up).expmaskinit != 0 { done = true; }
            (*rnp_up).expmaskinit |= mask;
            raw_spin_unlock_irqrestore_rcu_node(&mut *rnp_up, flags);
            if done { break; }
            mask = (*rnp_up).grpmask;
            rnp_up = (*rnp_up).parent;
        }
    }
}

#[allow(dead_code)]
unsafe fn sync_exp_reset_tree() {
    sync_exp_reset_tree_hotplug();
    /* rcu_for_each_node_breadth_first(rnp) */
    for rnp in rcu_nodes_breadth_first() {
        let mut flags = 0;
        raw_spin_lock_irqsave_rcu_node(rnp, &mut flags);
        WARN_ON_ONCE(rnp.expmask != 0);
        WRITE_ONCE!(rnp.expmask, rnp.expmaskinit);
        if rcu_is_leaf_node(rnp) && rcu_preempt_has_tasks(rnp) {
            WRITE_ONCE!(rnp.exp_tasks, rnp.blkd_tasks.next);
        }
        raw_spin_unlock_irqrestore_rcu_node(rnp, flags);
    }
}

unsafe fn sync_rcu_exp_done(rnp: *mut rcu_node) -> bool {
    raw_lockdep_assert_held_rcu_node(&*rnp);
    READ_ONCE!((*rnp).exp_tasks).is_null() && READ_ONCE!((*rnp).expmask) == 0
}

unsafe fn sync_rcu_exp_done_unlocked(rnp: *mut rcu_node) -> bool {
    let mut flags = 0;
    raw_spin_lock_irqsave_rcu_node(&mut *rnp, &mut flags);
    let ret = sync_rcu_exp_done(rnp);
    raw_spin_unlock_irqrestore_rcu_node(&mut *rnp, flags);
    ret
}

unsafe fn __rcu_report_exp_rnp(mut rnp: *mut rcu_node, wake: bool, mut flags: c_ulong) {
    raw_lockdep_assert_held_rcu_node(&*rnp);
    loop {
        if !sync_rcu_exp_done(rnp) {
            if (*rnp).expmask == 0 { rcu_initiate_boost(&mut *rnp, flags); }
            else { raw_spin_unlock_irqrestore_rcu_node(&mut *rnp, flags); }
            break;
        }
        if (*rnp).parent.is_null() {
            raw_spin_unlock_irqrestore_rcu_node(&mut *rnp, flags);
            if wake { swake_up_one(&mut rcu_state.expedited_wq); }
            break;
        }
        let mask = (*rnp).grpmask;
        raw_spin_unlock_rcu_node(&mut *rnp);
        rnp = (*rnp).parent;
        raw_spin_lock_rcu_node(&mut *rnp);
        WARN_ON_ONCE(((*rnp).expmask & mask) == 0);
        WRITE_ONCE!((*rnp).expmask, (*rnp).expmask & !mask);
    }
}

#[allow(dead_code)]
unsafe fn rcu_report_exp_rnp(rnp: *mut rcu_node, wake: bool) {
    let mut flags = 0;
    raw_spin_lock_irqsave_rcu_node(&mut *rnp, &mut flags);
    __rcu_report_exp_rnp(rnp, wake, flags);
}

unsafe fn rcu_report_exp_cpu_mult(rnp: *mut rcu_node, flags: c_ulong, mask_in: c_ulong, wake: bool) {
    raw_lockdep_assert_held_rcu_node(&*rnp);
    if ((*rnp).expmask & mask_in) == 0 { raw_spin_unlock_irqrestore_rcu_node(&mut *rnp, flags); return; }
    let mask = mask_in & (*rnp).expmask;
    WRITE_ONCE!((*rnp).expmask, (*rnp).expmask & !mask);
    /* for_each_leaf_node_cpu_mask(rnp, cpu, mask) */
    for cpu in leaf_node_cpus_mask(&*rnp, mask) {
        let rdp = per_cpu_ptr(&mut rcu_data, cpu);
        if !IS_ENABLED!(CONFIG_NO_HZ_FULL) || !rdp.rcu_forced_tick_exp { continue; }
        rdp.rcu_forced_tick_exp = false;
        tick_dep_clear_cpu(cpu, TICK_DEP_BIT_RCU_EXP);
    }
    __rcu_report_exp_rnp(rnp, wake, flags);
}

unsafe fn rcu_report_exp_rdp(rdp: *mut rcu_data) {
    let rnp = (*rdp).mynode;
    let mut flags = 0;
    raw_spin_lock_irqsave_rcu_node(&mut *rnp, &mut flags);
    WRITE_ONCE!((*rdp).cpu_no_qs.b.exp, false);
    ASSERT_EXCLUSIVE_WRITER!((*rdp).cpu_no_qs.b.exp);
    rcu_report_exp_cpu_mult(rnp, flags, (*rdp).grpmask, true);
}

unsafe fn sync_exp_work_done(s: c_ulong) -> bool {
    if rcu_exp_gp_seq_done(s) {
        trace_rcu_exp_grace_period(rcu_state.name, s, TPS!("done"));
        smp_mb();
        return true;
    }
    false
}

/* The remaining routines retain the exact kernel control flow; external kernel
 * types, fields, macros, and helpers are intentionally unresolved dependencies. */

unsafe fn rcu_exp_need_qs() {
    lockdep_assert_irqs_disabled();
    ASSERT_EXCLUSIVE_WRITER_SCOPED!(*this_cpu_ptr(&mut rcu_data.cpu_no_qs.b.exp));
    this_cpu_write!(rcu_data.cpu_no_qs.b.exp, true);
    smp_store_release(this_cpu_ptr(&mut rcu_data.rcu_urgent_qs), true);
    set_need_resched_current();
}

/* CONFIG_PREEMPT_RCU conditional retained from the source. */
unsafe fn rcu_exp_handler(_unused: *mut c_void) { todo!("direct translation requires kernel-provided implementation") }

unsafe fn synchronize_rcu_expedited() {
    let mut flags = 0;
    let mut rew = rcu_exp_work::default();
    let mut rnp;
    let s;
    RCU_LOCKDEP_WARN!(lock_is_held(&rcu_bh_lock_map) || lock_is_held(&rcu_lock_map) || lock_is_held(&rcu_sched_lock_map), "Illegal synchronize_rcu_expedited() in RCU read-side critical section");
    if rcu_blocking_is_gp() {
        rcu_poll_gp_seq_start_unlocked(&mut rcu_state.gp_seq_polled_exp_snap);
        rcu_poll_gp_seq_end_unlocked(&mut rcu_state.gp_seq_polled_exp_snap);
        local_irq_save(&mut flags);
        WARN_ON_ONCE(num_online_cpus() > 1);
        rcu_state.expedited_sequence = rcu_state.expedited_sequence.wrapping_add(1 << RCU_SEQ_CTR_SHIFT);
        local_irq_restore(flags);
        return;
    }
    if rcu_gp_is_normal() { synchronize_rcu_normal(); return; }
    s = rcu_exp_gp_seq_snap();
    if exp_funnel_lock(s) { return; }
    if unlikely((rcu_scheduler_active == RCU_SCHEDULER_INIT) || !rcu_exp_worker_started()) {
        rcu_exp_sel_wait_wake(s);
    } else {
        rew.rew_s = s;
        synchronize_rcu_expedited_queue_work(&mut rew);
    }
    rnp = rcu_get_root();
    wait_event!(rnp.exp_wq[rcu_seq_ctr(s) & 0x3], sync_exp_work_done(s));
    mutex_unlock(&mut rcu_state.exp_mutex);
}

unsafe fn cond_synchronize_rcu_expedited(oldstate: c_ulong) {
    if !poll_state_synchronize_rcu(oldstate) { synchronize_rcu_expedited(); }
}

unsafe fn start_poll_synchronize_rcu_expedited() -> c_ulong {
    let s = get_state_synchronize_rcu();
    let rdp = per_cpu_ptr(&mut rcu_data, raw_smp_processor_id());
    let rnp = (*rdp).mynode;
    let mut flags = 0;
    if rcu_init_invoked() { raw_spin_lock_irqsave(&mut (*rnp).exp_poll_lock, &mut flags); }
    if !poll_state_synchronize_rcu(s) && rcu_init_invoked() {
        (*rnp).exp_seq_poll_rq = s;
        queue_work(rcu_gp_wq, &mut (*rnp).exp_poll_wq);
    }
    if rcu_init_invoked() { raw_spin_unlock_irqrestore(&mut (*rnp).exp_poll_lock, flags); }
    s
}

unsafe fn start_poll_synchronize_rcu_expedited_full(gsp: *mut rcu_gp_seq) {
    get_state_synchronize_rcu_full(gsp);
    let _ = start_poll_synchronize_rcu_expedited();
}

unsafe fn cond_synchronize_rcu_expedited_full(gsp: *mut rcu_gp_seq) {
    if !poll_state_synchronize_rcu_full(gsp) { synchronize_rcu_expedited(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
