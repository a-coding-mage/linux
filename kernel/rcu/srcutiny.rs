// SPDX-License-Identifier: GPL-2.0+
/*
 * Sleepable Read-Copy Update mechanism for mutual exclusion,
 *	tiny version for non-preemptible single-CPU use.
 *
 * Copyright (C) IBM Corporation, 2017
 *
 * Author: Paul McKenney <paulmck@linux.ibm.com>
 */

// C dependencies supplied by the surrounding kernel translation.

#[cfg(not(CONFIG_TREE_RCU))]
pub static mut rcu_scheduler_active: i32 = 0;
#[cfg(CONFIG_TREE_RCU)]
extern "C" {
    pub static mut rcu_scheduler_active: i32;
}

static mut srcu_boot_list: ListHead = ListHead::new();
static mut srcu_init_done: bool = false;

unsafe fn init_srcu_struct_fields(ssp: *mut srcu_struct) -> i32 {
    (*ssp).srcu_lock_nesting[0] = 0;
    (*ssp).srcu_lock_nesting[1] = 0;
    init_swait_queue_head(&mut (*ssp).srcu_wq);
    (*ssp).srcu_cb_head = core::ptr::null_mut();
    (*ssp).srcu_cb_tail = &mut (*ssp).srcu_cb_head;
    (*ssp).srcu_gp_running = false;
    (*ssp).srcu_gp_waiting = false;
    (*ssp).srcu_idx = 0;
    (*ssp).srcu_idx_max = 0;
    INIT_WORK(&mut (*ssp).srcu_work, srcu_drive_gp);
    INIT_LIST_HEAD(&mut (*ssp).srcu_work.entry);
    init_irq_work(&mut (*ssp).srcu_irq_work, srcu_tiny_irq_work);
    0
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
pub unsafe extern "C" fn init_srcu_struct_lockdep(
    ssp: *mut srcu_struct,
    name: *const core::ffi::c_char,
    key: *mut lock_class_key,
) -> i32 {
    // Don't re-initialize a lock while it is held.
    debug_check_no_locks_freed(ssp.cast(), core::mem::size_of::<srcu_struct>());
    lockdep_init_map(&mut (*ssp).dep_map, name, key, 0);
    init_srcu_struct_fields(ssp)
}

#[cfg(not(CONFIG_DEBUG_LOCK_ALLOC))]
pub unsafe extern "C" fn init_srcu_struct_generic(ssp: *mut srcu_struct) -> i32 {
    init_srcu_struct_fields(ssp)
}

pub unsafe extern "C" fn cleanup_srcu_struct(ssp: *mut srcu_struct) {
    WARN_ON(srcu_readers_active(ssp));
    irq_work_sync(&mut (*ssp).srcu_irq_work);
    flush_work(&mut (*ssp).srcu_work);
    WARN_ON((*ssp).srcu_gp_running);
    WARN_ON((*ssp).srcu_gp_waiting);
    WARN_ON(!(*ssp).srcu_cb_head.is_null());
    WARN_ON(&mut (*ssp).srcu_cb_head != (*ssp).srcu_cb_tail);
    WARN_ON((*ssp).srcu_idx != (*ssp).srcu_idx_max);
    WARN_ON((*ssp).srcu_idx & 0x1 != 0);
}

pub unsafe extern "C" fn __srcu_read_unlock(ssp: *mut srcu_struct, idx: i32) {
    let newval: i32;
    preempt_disable(); // Needed for PREEMPT_LAZY
    newval = READ_ONCE((*ssp).srcu_lock_nesting[idx as usize]) - 1;
    WRITE_ONCE((*ssp).srcu_lock_nesting[idx as usize], newval);
    preempt_enable();
    if newval == 0 && READ_ONCE((*ssp).srcu_gp_waiting) && in_task() && !irqs_disabled() {
        swake_up_one(&mut (*ssp).srcu_wq);
    }
}

pub unsafe extern "C" fn srcu_drive_gp(wp: *mut work_struct) {
    let mut idx: i32;
    let mut lh: *mut rcu_head;
    let mut rhp: *mut rcu_head;
    let ssp = container_of!(wp, srcu_struct, srcu_work);
    preempt_disable(); // Needed for PREEMPT_LAZY
    if (*ssp).srcu_gp_running || ULONG_CMP_GE((*ssp).srcu_idx, READ_ONCE((*ssp).srcu_idx_max)) {
        preempt_enable();
        return; // Already running or nothing to do.
    }
    WRITE_ONCE((*ssp).srcu_gp_running, true);
    local_irq_disable();
    lh = (*ssp).srcu_cb_head;
    (*ssp).srcu_cb_head = core::ptr::null_mut();
    (*ssp).srcu_cb_tail = &mut (*ssp).srcu_cb_head;
    local_irq_enable();
    idx = ((*ssp).srcu_idx & 0x2) / 2;
    WRITE_ONCE((*ssp).srcu_idx, (*ssp).srcu_idx + 1);
    WRITE_ONCE((*ssp).srcu_gp_waiting, true); // srcu_read_unlock() wakes!
    preempt_enable();
    loop {
        // Deadlock issues prevent __srcu_read_unlock() from doing an unconditional wakeup, so polling is required.
        swait_event_timeout_exclusive(&mut (*ssp).srcu_wq, READ_ONCE((*ssp).srcu_lock_nesting[idx as usize]) == 0, HZ / 10);
        if READ_ONCE((*ssp).srcu_lock_nesting[idx as usize]) == 0 { break; }
    }
    preempt_disable(); // Needed for PREEMPT_LAZY
    WRITE_ONCE((*ssp).srcu_gp_waiting, false); // srcu_read_unlock() cheap.
    WRITE_ONCE((*ssp).srcu_idx, (*ssp).srcu_idx + 1);
    preempt_enable();
    while !lh.is_null() {
        rhp = lh;
        lh = (*lh).next;
        debug_rcu_head_callback(rhp);
        local_bh_disable();
        ((*rhp).func)(rhp);
        local_bh_enable();
    }
    preempt_disable(); // Needed for PREEMPT_LAZY
    WRITE_ONCE((*ssp).srcu_gp_running, false);
    idx = ULONG_CMP_LT((*ssp).srcu_idx, READ_ONCE((*ssp).srcu_idx_max)) as i32;
    preempt_enable();
    if idx != 0 { schedule_work(&mut (*ssp).srcu_work); }
}

pub unsafe extern "C" fn srcu_tiny_irq_work(irq_work: *mut irq_work) {
    let ssp = container_of!(irq_work, srcu_struct, srcu_irq_work);
    schedule_work(&mut (*ssp).srcu_work);
}

unsafe fn srcu_gp_start_if_needed(ssp: *mut srcu_struct) {
    let cookie: usize;
    lockdep_assert_preemption_disabled(); // Needed for PREEMPT_LAZY
    cookie = get_state_synchronize_srcu(ssp);
    if ULONG_CMP_GE(READ_ONCE((*ssp).srcu_idx_max), cookie) { return; }
    WRITE_ONCE((*ssp).srcu_idx_max, cookie);
    if !READ_ONCE((*ssp).srcu_gp_running) {
        if likely(srcu_init_done) { irq_work_queue(&mut (*ssp).srcu_irq_work); }
        else if list_empty(&mut (*ssp).srcu_work.entry) { list_add(&mut (*ssp).srcu_work.entry, &mut srcu_boot_list); }
    }
}

pub unsafe extern "C" fn call_srcu(ssp: *mut srcu_struct, rhp: *mut rcu_head, func: rcu_callback_t) {
    let mut flags: usize = 0;
    (*rhp).func = func;
    (*rhp).next = core::ptr::null_mut();
    preempt_disable(); // Needed for PREEMPT_LAZY
    local_irq_save(&mut flags);
    *(*ssp).srcu_cb_tail = rhp;
    (*ssp).srcu_cb_tail = &mut (*rhp).next;
    local_irq_restore(flags);
    srcu_gp_start_if_needed(ssp);
    preempt_enable();
}

pub unsafe extern "C" fn synchronize_srcu(ssp: *mut srcu_struct) {
    let mut rs: rcu_synchronize = core::mem::zeroed();
    srcu_lock_sync(&mut (*ssp).dep_map);
    RCU_LOCKDEP_WARN(lockdep_is_held(ssp) || lock_is_held(&rcu_bh_lock_map) || lock_is_held(&rcu_lock_map) || lock_is_held(&rcu_sched_lock_map), "Illegal synchronize_srcu() in same-type SRCU (or in RCU) read-side critical section");
    if rcu_scheduler_active == RCU_SCHEDULER_INACTIVE { return; }
    might_sleep();
    init_rcu_head_on_stack(&mut rs.head);
    init_completion(&mut rs.completion);
    call_srcu(ssp, &mut rs.head, wakeme_after_rcu);
    wait_for_completion(&mut rs.completion);
    destroy_rcu_head_on_stack(&mut rs.head);
}

pub unsafe extern "C" fn get_state_synchronize_srcu(ssp: *mut srcu_struct) -> usize {
    barrier();
    let ret = (READ_ONCE((*ssp).srcu_idx) + 3) & !0x1;
    barrier();
    ret
}

pub unsafe extern "C" fn start_poll_synchronize_srcu(ssp: *mut srcu_struct) -> usize {
    preempt_disable(); // Needed for PREEMPT_LAZY
    let ret = get_state_synchronize_srcu(ssp);
    srcu_gp_start_if_needed(ssp);
    preempt_enable();
    ret
}

pub unsafe extern "C" fn poll_state_synchronize_srcu(ssp: *mut srcu_struct, cookie: usize) -> bool {
    let cur_s = READ_ONCE((*ssp).srcu_idx);
    barrier();
    cookie == SRCU_GET_STATE_COMPLETED || ULONG_CMP_GE(cur_s, cookie) || ULONG_CMP_LT(cur_s, cookie - 3)
}

#[cfg(not(CONFIG_TREE_RCU))]
pub unsafe extern "C" fn rcu_scheduler_starting() {
    rcu_scheduler_active = RCU_SCHEDULER_RUNNING;
}

pub unsafe extern "C" fn srcu_init() {
    srcu_init_done = true;
    while !list_empty(&mut srcu_boot_list) {
        let ssp = list_first_entry!(&mut srcu_boot_list, srcu_struct, srcu_work.entry);
        list_del_init(&mut (*ssp).srcu_work.entry);
        schedule_work(&mut (*ssp).srcu_work);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
