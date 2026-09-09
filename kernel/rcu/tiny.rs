// SPDX-License-Identifier: GPL-2.0+
/*
 * Read-Copy Update mechanism for mutual exclusion, the Bloatwatch edition.
 *
 * Copyright IBM Corporation, 2008
 *
 * Author: Paul E. McKenney <paulmck@linux.ibm.com>
 *
 * For detailed explanation of Read-Copy Update mechanism see -
 *		Documentation/RCU
 */

// Declarations below are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct rcu_ctrlblk {
    pub rcucblist: *mut rcu_head,
    pub donetail: *mut *mut rcu_head,
    pub curtail: *mut *mut rcu_head,
    pub gp_seq: usize,
}

static mut rcu_ctrlblk: rcu_ctrlblk = rcu_ctrlblk {
    rcucblist: core::ptr::null_mut(),
    donetail: core::ptr::null_mut(),
    curtail: core::ptr::null_mut(),
    gp_seq: 0usize.wrapping_sub(300),
};

pub unsafe fn rcu_barrier() {
    wait_rcu_gp(call_rcu_hurry);
}

pub unsafe fn rcu_qs() {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    if rcu_ctrlblk.donetail != rcu_ctrlblk.curtail {
        rcu_ctrlblk.donetail = rcu_ctrlblk.curtail;
        raise_softirq_irqoff(RCU_SOFTIRQ);
    }
    core::ptr::write_volatile(&mut rcu_ctrlblk.gp_seq, rcu_ctrlblk.gp_seq.wrapping_add(2));
    local_irq_restore(flags);
}

pub unsafe fn rcu_sched_clock_irq(user: i32) {
    if user != 0 {
        rcu_qs();
    } else if rcu_ctrlblk.donetail != rcu_ctrlblk.curtail {
        set_need_resched_current();
    }
}

unsafe fn rcu_reclaim_tiny(head: *mut rcu_head) -> bool {
    rcu_lock_acquire(&rcu_callback_map);
    trace_rcu_invoke_callback("", head);
    let f = (*head).func;
    debug_rcu_head_callback(head);
    core::ptr::write_volatile(&mut (*head).func, None);
    f.unwrap()(head);
    rcu_lock_release(&rcu_callback_map);
    false
}

unsafe fn rcu_process_callbacks() {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    if rcu_ctrlblk.donetail == &mut rcu_ctrlblk.rcucblist {
        local_irq_restore(flags);
        return;
    }
    let mut list = rcu_ctrlblk.rcucblist;
    rcu_ctrlblk.rcucblist = *rcu_ctrlblk.donetail;
    *rcu_ctrlblk.donetail = core::ptr::null_mut();
    if rcu_ctrlblk.curtail == rcu_ctrlblk.donetail {
        rcu_ctrlblk.curtail = &mut rcu_ctrlblk.rcucblist;
    }
    rcu_ctrlblk.donetail = &mut rcu_ctrlblk.rcucblist;
    local_irq_restore(flags);

    while !list.is_null() {
        let next = (*list).next;
        prefetch(next);
        debug_rcu_head_unqueue(list);
        rcu_reclaim_tiny(list);
        list = next;
    }
}

pub unsafe fn synchronize_rcu() {
    RCU_LOCKDEP_WARN(
        lock_is_held(&rcu_bh_lock_map) || lock_is_held(&rcu_lock_map) || lock_is_held(&rcu_sched_lock_map),
        "Illegal synchronize_rcu() in RCU read-side critical section",
    );
    preempt_disable();
    core::ptr::write_volatile(&mut rcu_ctrlblk.gp_seq, rcu_ctrlblk.gp_seq.wrapping_add(2));
    preempt_enable();
}

pub unsafe fn call_rcu(head: *mut rcu_head, func: rcu_callback_t) {
    static mut doublefrees: atomic_t = atomic_t { value: 0 };
    let mut flags: usize = 0;
    if debug_rcu_head_queue(head) {
        if atomic_inc_return(&mut doublefrees) < 4 {
            pr_err_double_free(head);
            mem_dump_obj(head);
        }
        return;
    }
    (*head).func = Some(func);
    (*head).next = core::ptr::null_mut();
    local_irq_save(&mut flags);
    *rcu_ctrlblk.curtail = head;
    rcu_ctrlblk.curtail = &mut (*head).next;
    local_irq_restore(flags);
    if is_idle_task(current) {
        resched_cpu(0);
    }
}

pub unsafe fn get_completed_synchronize_rcu_full(gsp: *mut rcu_gp_seq) {
    (*gsp).norm = RCU_GET_STATE_COMPLETED;
}

pub unsafe fn get_state_synchronize_rcu() -> usize {
    core::ptr::read_volatile(&rcu_ctrlblk.gp_seq)
}

pub unsafe fn start_poll_synchronize_rcu() -> usize {
    let gp_seq = get_state_synchronize_rcu();
    if is_idle_task(current) { resched_cpu(0); }
    gp_seq
}

pub unsafe fn poll_state_synchronize_rcu(oldstate: usize) -> bool {
    oldstate == RCU_GET_STATE_COMPLETED || core::ptr::read_volatile(&rcu_ctrlblk.gp_seq) != oldstate
}

#[cfg(feature = "CONFIG_RCU_TORTURE_TEST")]
pub unsafe fn rcutorture_gather_gp_seqs() -> u64 {
    core::ptr::read_volatile(&rcu_ctrlblk.gp_seq) as u64 & 0xffffu64
}

#[cfg(feature = "CONFIG_RCU_TORTURE_TEST")]
pub unsafe fn rcutorture_format_gp_seqs(seqs: u64, cp: *mut u8, len: usize) {
    snprintf(cp, len, b"g%04llx\0".as_ptr(), seqs & 0xffffu64);
}

pub unsafe fn rcu_init() {
    rcu_ctrlblk.donetail = &mut rcu_ctrlblk.rcucblist;
    rcu_ctrlblk.curtail = &mut rcu_ctrlblk.rcucblist;
    open_softirq(RCU_SOFTIRQ, rcu_process_callbacks);
    rcu_early_boot_tests();
    tasks_cblist_init_generic();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
