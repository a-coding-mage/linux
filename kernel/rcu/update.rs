// SPDX-License-Identifier: GPL-2.0+
/*
 * Read-Copy Update mechanism for mutual exclusion
 *
 * Copyright IBM Corporation, 2001
 *
 * Authors: Dipankar Sarma <dipankar@in.ibm.com>
 *          Manfred Spraul <manfred@colorfullife.com>
 *
 * Based on the original work by Paul McKenney <paulmck@linux.ibm.com>
 * and inputs from Rusty Russell, Andrea Arcangeli and Andi Kleen.
 * Papers:
 * http://www.rdrop.com/users/paulmck/paper/rclockpdcsproof.pdf
 * http://lse.sourceforge.net/locking/rclock_OLS.2001.05.01c.sc.pdf (OLS2001)
 *
 * For detailed explanation of Read-Copy Update mechanism see -
 *      http://lse.sourceforge.net/locking/rcupdate.html
 */

// Linux kernel dependencies supplied by other translation units.

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
unsafe fn rcu_read_lock_held_common(ret: *mut bool) -> bool {
    if !debug_lockdep_rcu_enabled() { *ret = true; return true; }
    if !rcu_is_watching() { *ret = false; return true; }
    if !rcu_lockdep_current_cpu_online() { *ret = false; return true; }
    false
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_sched_held() -> i32 {
    let mut ret = false;
    if rcu_read_lock_held_common(&mut ret) { return ret as i32; }
    (lock_is_held(&rcu_sched_lock_map) || !preemptible()) as i32
}

#[cfg(not(CONFIG_TINY_RCU))]
#[no_mangle]
pub unsafe extern "C" fn rcu_gp_is_normal() -> bool {
    READ_ONCE(rcu_normal) && rcu_scheduler_active != RCU_SCHEDULER_INIT
}

#[cfg(not(CONFIG_TINY_RCU))]
static mut rcu_async_hurry_nesting: AtomicInt = AtomicInt::init(1);

#[cfg(not(CONFIG_TINY_RCU))]
#[no_mangle]
pub unsafe extern "C" fn rcu_async_should_hurry() -> bool {
    !cfg!(CONFIG_RCU_LAZY) || rcu_async_hurry_nesting.read() != 0
}

#[cfg(not(CONFIG_TINY_RCU))]
pub unsafe extern "C" fn rcu_async_hurry() {
    if cfg!(CONFIG_RCU_LAZY) { rcu_async_hurry_nesting.inc(); }
}

#[cfg(not(CONFIG_TINY_RCU))]
pub unsafe extern "C" fn rcu_async_relax() {
    if cfg!(CONFIG_RCU_LAZY) { rcu_async_hurry_nesting.dec(); }
}

#[cfg(not(CONFIG_TINY_RCU))]
static mut rcu_expedited_nesting: AtomicInt = AtomicInt::init(1);

#[cfg(not(CONFIG_TINY_RCU))]
pub unsafe extern "C" fn rcu_gp_is_expedited() -> bool {
    rcu_expedited || rcu_expedited_nesting.read() != 0
}

#[cfg(not(CONFIG_TINY_RCU))]
pub unsafe extern "C" fn rcu_expedite_gp() { rcu_expedited_nesting.inc(); }

#[cfg(not(CONFIG_TINY_RCU))]
pub unsafe extern "C" fn rcu_unexpedite_gp() { rcu_expedited_nesting.dec(); }

#[cfg(not(CONFIG_TINY_RCU))]
static mut rcu_boot_ended: bool = false;

#[cfg(not(CONFIG_TINY_RCU))]
pub unsafe extern "C" fn rcu_end_inkernel_boot() {
    rcu_unexpedite_gp(); rcu_async_relax();
    if rcu_normal_after_boot { WRITE_ONCE(rcu_normal, 1); }
    rcu_boot_ended = true;
}

#[cfg(not(CONFIG_TINY_RCU))]
pub unsafe extern "C" fn rcu_inkernel_boot_has_ended() -> bool { rcu_boot_ended }

pub unsafe extern "C" fn rcu_test_sync_prims() {
    if !cfg!(CONFIG_PROVE_RCU) { return; }
    pr_info("Running RCU synchronous self tests\n");
    synchronize_rcu(); synchronize_rcu_expedited();
}

#[cfg(not(CONFIG_TINY_RCU))]
unsafe extern "C" fn rcu_set_runtime_mode() -> i32 {
    rcu_test_sync_prims();
    rcu_scheduler_active = RCU_SCHEDULER_RUNNING;
    kfree_rcu_scheduler_running();
    rcu_test_sync_prims();
    0
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
pub unsafe extern "C" fn debug_lockdep_rcu_enabled() -> i32 {
    (rcu_scheduler_active != RCU_SCHEDULER_INACTIVE && READ_ONCE(debug_locks) &&
     (*current).lockdep_recursion == 0) as i32
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
pub unsafe extern "C" fn rcu_read_lock_held() -> i32 {
    let mut ret = false;
    if rcu_read_lock_held_common(&mut ret) { return ret as i32; }
    lock_is_held(&rcu_lock_map) as i32
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
pub unsafe extern "C" fn rcu_read_lock_bh_held() -> i32 {
    let mut ret = false;
    if rcu_read_lock_held_common(&mut ret) { return ret as i32; }
    (in_softirq() || irqs_disabled()) as i32
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
pub unsafe extern "C" fn rcu_read_lock_any_held() -> i32 {
    let mut ret = false;
    if rcu_read_lock_held_common(&mut ret) { return ret as i32; }
    (lock_is_held(&rcu_lock_map) || lock_is_held(&rcu_bh_lock_map) ||
     lock_is_held(&rcu_sched_lock_map) || !preemptible()) as i32
}

pub unsafe extern "C" fn wakeme_after_rcu(head: *mut rcu_head) {
    let rcu = container_of!(head, rcu_synchronize, head);
    complete(&mut (*rcu).completion);
}

pub unsafe extern "C" fn __wait_rcu_gp(checktiny: bool, state: u32, n: i32,
                                        crcu_array: *mut call_rcu_func_t,
                                        rs_array: *mut rcu_synchronize) {
    for i in 0..n {
        if checktiny && (*crcu_array.add(i as usize)) == call_rcu { might_sleep(); continue; }
        let mut j = 0; while j < i && (*crcu_array.add(j as usize)) != (*crcu_array.add(i as usize)) { j += 1; }
        if j == i { init_rcu_head_on_stack(&mut (*rs_array.add(i as usize)).head); init_completion(&mut (*rs_array.add(i as usize)).completion); (*crcu_array.add(i as usize))(&mut (*rs_array.add(i as usize)).head, wakeme_after_rcu); }
    }
    for i in 0..n {
        if checktiny && (*crcu_array.add(i as usize)) == call_rcu { continue; }
        let mut j = 0; while j < i && (*crcu_array.add(j as usize)) != (*crcu_array.add(i as usize)) { j += 1; }
        if j == i { wait_for_completion_state(&mut (*rs_array.add(i as usize)).completion, state); destroy_rcu_head_on_stack(&mut (*rs_array.add(i as usize)).head); }
    }
}

pub unsafe extern "C" fn finish_rcuwait(w: *mut rcuwait) { rcu_assign_pointer!((*w).task, core::ptr::null_mut()); __set_current_state(TASK_RUNNING); }

#[cfg(CONFIG_DEBUG_OBJECTS_RCU_HEAD)]
pub unsafe extern "C" fn init_rcu_head(head: *mut rcu_head) { debug_object_init(head, &rcuhead_debug_descr); }
#[cfg(CONFIG_DEBUG_OBJECTS_RCU_HEAD)]
pub unsafe extern "C" fn destroy_rcu_head(head: *mut rcu_head) { debug_object_free(head, &rcuhead_debug_descr); }
#[cfg(CONFIG_DEBUG_OBJECTS_RCU_HEAD)]
unsafe fn rcuhead_is_static_object(_addr: *mut core::ffi::c_void) -> bool { true }
#[cfg(CONFIG_DEBUG_OBJECTS_RCU_HEAD)]
pub unsafe extern "C" fn init_rcu_head_on_stack(head: *mut rcu_head) { debug_object_init_on_stack(head, &rcuhead_debug_descr); }
#[cfg(CONFIG_DEBUG_OBJECTS_RCU_HEAD)]
pub unsafe extern "C" fn destroy_rcu_head_on_stack(head: *mut rcu_head) { debug_object_free(head, &rcuhead_debug_descr); }

#[cfg(any(CONFIG_TREE_RCU, CONFIG_RCU_TRACE))]
pub unsafe extern "C" fn do_trace_rcu_torture_read(name: *const c_char, rhp: *mut rcu_head, secs: c_ulong, old: c_ulong, c: c_ulong) { trace_rcu_torture_read(name, rhp, secs, old, c); }

#[cfg(any(CONFIG_RCU_TORTURE_TEST, CONFIG_LOCK_TORTURE_TEST))]
pub unsafe extern "C" fn torture_sched_setaffinity(pid: pid_t, mask: *const cpumask, dowarn: bool) -> c_long { let ret = sched_setaffinity(pid, mask); WARN_ONCE(dowarn && ret != 0, "%s: sched_setaffinity(%d) returned %d\n", "torture_sched_setaffinity", pid, ret); ret }

#[cfg(CONFIG_TRIVIAL_PREEMPT_RCU)]
pub unsafe extern "C" fn synchronize_rcu_trivial_preempt() { smp_mb(); rcu_read_lock(); for_each_process_thread!((_g, t) => { if t == current { return; } while (*t).rcu_trivial_preempt_nesting.load_acquire() != 0 {} }); rcu_read_unlock(); }

pub static mut rcu_cpu_stall_notifiers: i32 = 0;
#[cfg(CONFIG_RCU_STALL_COMMON)]
pub static mut rcu_cpu_stall_ftrace_dump: i32 = 0;
#[cfg(CONFIG_RCU_STALL_COMMON)]
pub static mut rcu_cpu_stall_suppress: i32 = 0;
#[cfg(CONFIG_RCU_STALL_COMMON)]
pub static mut rcu_cpu_stall_timeout: i32 = CONFIG_RCU_CPU_STALL_TIMEOUT;
#[cfg(CONFIG_RCU_STALL_COMMON)]
pub static mut rcu_exp_cpu_stall_timeout: i32 = CONFIG_RCU_EXP_CPU_STALL_TIMEOUT;
#[cfg(CONFIG_RCU_STALL_COMMON)]
pub static mut rcu_cpu_stall_cputime: i32 = cfg!(CONFIG_RCU_CPU_STALL_CPUTIME) as i32;
#[cfg(CONFIG_RCU_STALL_COMMON)]
pub static mut rcu_exp_stall_task_details: bool = false;
pub static mut rcu_cpu_stall_suppress_at_boot: i32 = 0;

pub unsafe extern "C" fn get_completed_synchronize_rcu() -> c_ulong { RCU_GET_STATE_COMPLETED }

#[cfg(CONFIG_PROVE_RCU)]
pub unsafe extern "C" fn rcu_early_boot_tests() { pr_info("Running RCU self tests\n"); if rcu_self_test { early_boot_test_call_rcu(); } rcu_test_sync_prims(); }
#[cfg(not(CONFIG_PROVE_RCU))]
pub unsafe extern "C" fn rcu_early_boot_tests() {}

#[cfg(not(CONFIG_TINY_RCU))]
pub unsafe extern "C" fn rcupdate_announce_bootup_oddness() {
    if rcu_normal { pr_info("\tNo expedited grace period (rcu_normal).\n"); }
    else if rcu_normal_after_boot { pr_info("\tNo expedited grace period (rcu_normal_after_boot).\n"); }
    else if rcu_expedited { pr_info("\tAll grace periods are expedited (rcu_expedited).\n"); }
    if rcu_cpu_stall_suppress != 0 { pr_info("\tRCU CPU stall warnings suppressed (rcu_cpu_stall_suppress).\n"); }
    if rcu_cpu_stall_timeout != CONFIG_RCU_CPU_STALL_TIMEOUT { pr_info("\tRCU CPU stall warnings timeout set to %d (rcu_cpu_stall_timeout).\n", rcu_cpu_stall_timeout); }
    rcu_tasks_bootup_oddness();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
