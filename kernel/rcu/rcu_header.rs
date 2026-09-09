/* SPDX-License-Identifier: GPL-2.0+ */
/* Rust translation of rcu.h.  Kernel-provided names are external dependencies. */

pub const RCU_GET_STATE_COMPLETED: usize = 0x1;
pub const RCU_GET_STATE_NOT_TRACKED: usize = 0x2;
pub const RCU_SEQ_GP: usize = RCU_SEQ_STATE_MASK + 1;

extern "C" {
    pub static mut sysctl_sched_rt_runtime: i32;
}

#[inline]
pub unsafe fn rcu_seq_ctr(s: libc::c_ulong) -> libc::c_ulong { s >> RCU_SEQ_CTR_SHIFT }
#[inline]
pub unsafe fn rcu_seq_state(s: libc::c_ulong) -> i32 { (s & RCU_SEQ_STATE_MASK) as i32 }
#[inline]
pub unsafe fn rcu_seq_set_state(sp: *mut libc::c_ulong, newstate: i32) {
    WARN_ON_ONCE((newstate as libc::c_ulong) & !RCU_SEQ_STATE_MASK);
    WRITE_ONCE(sp, (*sp & !RCU_SEQ_STATE_MASK).wrapping_add(newstate as libc::c_ulong));
}
#[inline]
pub unsafe fn rcu_seq_start(sp: *mut libc::c_ulong) {
    WRITE_ONCE(sp, (*sp).wrapping_add(1));
    smp_mb();
    WARN_ON_ONCE(rcu_seq_state(*sp) != 1);
}
#[inline]
pub unsafe fn rcu_seq_endval(sp: *mut libc::c_ulong) -> libc::c_ulong { ((*sp | RCU_SEQ_STATE_MASK).wrapping_add(1)) }
#[inline]
pub unsafe fn rcu_seq_end(sp: *mut libc::c_ulong) {
    smp_mb();
    WARN_ON_ONCE(rcu_seq_state(*sp) == 0);
    WRITE_ONCE(sp, rcu_seq_endval(sp));
}
#[inline]
pub unsafe fn rcu_seq_snap(sp: *mut libc::c_ulong) -> libc::c_ulong {
    let s = (READ_ONCE(*sp).wrapping_add(2 * RCU_SEQ_STATE_MASK).wrapping_add(1)) & !RCU_SEQ_STATE_MASK;
    smp_mb(); s
}
#[inline] pub unsafe fn rcu_seq_current(sp: *mut libc::c_ulong) -> libc::c_ulong { READ_ONCE(*sp) }
#[inline] pub unsafe fn rcu_seq_started(sp: *mut libc::c_ulong, s: libc::c_ulong) -> bool { ULONG_CMP_LT((s.wrapping_sub(1)) & !RCU_SEQ_STATE_MASK, READ_ONCE(*sp)) }
#[inline] pub unsafe fn rcu_seq_done(sp: *mut libc::c_ulong, s: libc::c_ulong) -> bool { ULONG_CMP_GE(READ_ONCE(*sp), s) }
#[inline] pub unsafe fn rcu_seq_done_exact(sp: *mut libc::c_ulong, s: libc::c_ulong) -> bool {
    let cur_s = READ_ONCE(*sp); ULONG_CMP_GE(cur_s, s) || ULONG_CMP_LT(cur_s, s.wrapping_sub(2 * RCU_SEQ_GP))
}
#[inline] pub unsafe fn rcu_seq_completed_gp(old: libc::c_ulong, new: libc::c_ulong) -> bool { ULONG_CMP_LT(old, new & !RCU_SEQ_STATE_MASK) }
#[inline] pub unsafe fn rcu_seq_new_gp(old: libc::c_ulong, new: libc::c_ulong) -> bool { ULONG_CMP_LT((old.wrapping_add(RCU_SEQ_STATE_MASK)) & !RCU_SEQ_STATE_MASK, new) }
#[inline] pub unsafe fn rcu_seq_diff(new: libc::c_ulong, old: libc::c_ulong) -> libc::c_ulong {
    if old == new { return 0; }
    let rnd_diff = (new & !RCU_SEQ_STATE_MASK).wrapping_sub((old.wrapping_add(RCU_SEQ_STATE_MASK)) & !RCU_SEQ_STATE_MASK)
        .wrapping_add(((new & RCU_SEQ_STATE_MASK != 0) || (old & RCU_SEQ_STATE_MASK != 0)) as libc::c_ulong);
    if ULONG_CMP_GE(RCU_SEQ_STATE_MASK, rnd_diff) { return 1; }
    ((rnd_diff.wrapping_sub(RCU_SEQ_STATE_MASK).wrapping_sub(1)) >> RCU_SEQ_CTR_SHIFT).wrapping_add(2)
}

extern "C" {
    pub static mut rcu_cpu_stall_suppress_at_boot: i32;
    pub static mut rcu_cpu_stall_notifiers: i32;
    pub fn rcu_inkernel_boot_has_ended() -> bool;
    pub fn rcu_jiffies_till_stall_check() -> i32;
    pub fn rcu_exp_jiffies_till_stall_check() -> i32;
    pub fn rcu_early_boot_tests();
    pub fn rcu_test_sync_prims();
    pub fn resched_cpu(cpu: i32);
}
#[inline] pub unsafe fn rcu_stall_is_suppressed_at_boot() -> bool { rcu_cpu_stall_suppress_at_boot != 0 && !rcu_inkernel_boot_has_ended() }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rcutorture_type { RCU_FLAVOR, RCU_TASKS_FLAVOR, RCU_TASKS_RUDE_FLAVOR, RCU_TASKS_TRACING_FLAVOR, RCU_TRIVIAL_FLAVOR, SRCU_FLAVOR, INVALID_RCU_FLAVOR }
pub const RCU_SCHEDULER_INACTIVE: i32 = 0;
pub const RCU_SCHEDULER_INIT: i32 = 1;
pub const RCU_SCHEDULER_RUNNING: i32 = 2;

extern "C" {
    pub fn rcutorture_gather_gp_seqs() -> u64;
    pub fn rcutorture_format_gp_seqs(seqs: u64, cp: *mut libc::c_char, len: usize);
}

/* Configuration-dependent declarations and macros from the header retain their build-time intent. */
#[cfg(feature = "tiny_rcu")]
#[inline] pub fn rcu_gp_is_normal() -> bool { true }
#[cfg(feature = "tiny_rcu")]
#[inline] pub fn rcu_gp_is_expedited() -> bool { false }
#[cfg(feature = "tiny_rcu")]
#[inline] pub fn rcu_async_should_hurry() -> bool { false }
#[cfg(feature = "tiny_rcu")]
#[inline] pub fn rcu_cpu_online(_cpu: i32) -> bool { true }

/* The remaining declarations depend on Linux kernel types, configuration, and macros supplied by included headers. */
extern "C" {
    pub fn rcu_expedite_gp(); pub fn rcu_unexpedite_gp(); pub fn rcu_async_hurry(); pub fn rcu_async_relax();
    pub fn rcupdate_announce_bootup_oddness();
    pub fn show_rcu_tasks_gp_kthreads();
    pub fn get_rcu_tasks_gp_kthread() -> *mut task_struct;
    pub fn rcu_tasks_get_gp_data(flags: *mut i32, gp_seq: *mut libc::c_ulong);
    pub fn get_rcu_tasks_rude_gp_kthread() -> *mut task_struct;
    pub fn rcu_tasks_rude_get_gp_data(flags: *mut i32, gp_seq: *mut libc::c_ulong);
    pub fn tasks_cblist_init_generic();
    pub fn rcu_get_jiffies_lazy_flush() -> libc::c_ulong;
    pub fn rcu_set_jiffies_lazy_flush(j: libc::c_ulong);
    pub fn rcutorture_get_gp_data(flags: *mut i32, gp_seq: *mut libc::c_ulong);
    pub fn do_trace_rcu_torture_read(name: *const libc::c_char, rhp: *mut rcu_head, secs: libc::c_ulong, c_old: libc::c_ulong, c: libc::c_ulong);
    pub fn rcu_gp_set_torture_wait(duration: i32);
    pub fn rcu_set_gpwrap_lag(lag: libc::c_ulong);
    pub fn rcu_get_gpwrap_count(cpu: i32) -> i32;
    pub fn srcutorture_get_gp_data(sp: *mut srcu_struct, flags: *mut i32, gp_seq: *mut libc::c_ulong);
    pub fn rcu_watching_zero_in_eqs(cpu: i32, vp: *mut i32) -> bool;
    pub fn rcu_get_gp_seq() -> libc::c_ulong;
    pub fn rcu_exp_batches_completed() -> libc::c_ulong;
    pub fn rcu_force_quiescent_state();
    pub fn rcu_check_boost_fail(gp_state: libc::c_ulong, cpup: *mut i32) -> bool;
    pub fn show_rcu_gp_kthreads();
    pub fn rcu_get_gp_kthreads_prio() -> i32;
    pub fn rcu_fwd_progress_check(j: libc::c_ulong);
    pub fn rcu_gp_slow_register(rgssp: *mut atomic_t);
    pub fn rcu_gp_slow_unregister(rgssp: *mut atomic_t);
    pub fn srcu_batches_completed(sp: *mut srcu_struct) -> libc::c_ulong;
    pub fn rcu_bind_current_to_nocb();
    pub fn show_rcu_tasks_classic_gp_kthread();
    pub fn show_rcu_tasks_rude_gp_kthread();
    pub fn rcu_cpu_beenfullyonline(cpu: i32) -> bool;
    pub fn rcu_stall_notifier_call_chain(val: libc::c_ulong, v: *mut libc::c_void) -> i32;
    pub fn synchronize_rcu_trivial_preempt();
    pub fn rcu_is_task_rcu_boosted() -> bool;
}

#[inline] pub unsafe fn rcu_stall_is_suppressed() -> bool { rcu_stall_is_suppressed_at_boot() }
#[inline] pub fn rcu_cpu_beenfullyonline(_cpu: i32) -> bool { true }
#[inline] pub fn rcu_check_boost_fail(_s: libc::c_ulong, _p: *mut i32) -> bool { true }
#[inline] pub fn rcu_is_task_rcu_boosted() -> bool { false }

/* C preprocessor iteration and locking macros are intentionally represented as declarations/comments;
 * their expansion requires the Linux rcu_node, lock, cpumask, and configuration definitions. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
