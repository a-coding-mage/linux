/* SPDX-License-Identifier: GPL-2.0+ */
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

// Dependency supplied by the surrounding translation unit.
// #include <asm/param.h> /* for HZ */

// Maximum number of rcu_gp_seq values corresponding to
// not-yet-completed RCU grace periods.
pub const NUM_ACTIVE_RCU_POLL_FULL_OLDSTATE: usize = 2;

// `HZ` and `rcu_gp_seq` are supplied by the included kernel dependencies.

pub unsafe fn same_state_synchronize_rcu_full(
    rgosp1: *mut rcu_gp_seq,
    rgosp2: *mut rcu_gp_seq,
) -> bool {
    unsafe { (*rgosp1).norm == (*rgosp2).norm }
}

extern "C" {
    pub fn get_state_synchronize_rcu() -> ::core::ffi::c_ulong;
}

pub unsafe fn get_state_synchronize_rcu_full(gsp: *mut rcu_gp_seq) {
    unsafe { (*gsp).norm = get_state_synchronize_rcu(); }
}

extern "C" {
    pub fn start_poll_synchronize_rcu() -> ::core::ffi::c_ulong;
}

pub unsafe fn start_poll_synchronize_rcu_full(gsp: *mut rcu_gp_seq) {
    unsafe { (*gsp).norm = start_poll_synchronize_rcu(); }
}

extern "C" {
    pub fn poll_state_synchronize_rcu(oldstate: ::core::ffi::c_ulong) -> bool;
    pub fn might_sleep();
    pub fn synchronize_rcu();
    pub fn rcu_barrier();
    pub fn rcu_qs();
    pub fn rcu_scheduler_starting();
    pub fn barrier();
}

pub unsafe fn poll_state_synchronize_rcu_full(gsp: *mut rcu_gp_seq) -> bool {
    unsafe { poll_state_synchronize_rcu((*gsp).norm) }
}

pub unsafe fn cond_synchronize_rcu(_oldstate: ::core::ffi::c_ulong) {
    unsafe { might_sleep(); }
}

pub unsafe fn cond_synchronize_rcu_full(gsp: *mut rcu_gp_seq) {
    unsafe { cond_synchronize_rcu((*gsp).norm); }
}

pub unsafe fn start_poll_synchronize_rcu_expedited() -> ::core::ffi::c_ulong {
    unsafe { start_poll_synchronize_rcu() }
}

pub unsafe fn start_poll_synchronize_rcu_expedited_full(gsp: *mut rcu_gp_seq) {
    unsafe { (*gsp).norm = start_poll_synchronize_rcu_expedited(); }
}

pub unsafe fn cond_synchronize_rcu_expedited(oldstate: ::core::ffi::c_ulong) {
    unsafe { cond_synchronize_rcu(oldstate); }
}

pub unsafe fn cond_synchronize_rcu_expedited_full(gsp: *mut rcu_gp_seq) {
    unsafe { cond_synchronize_rcu_expedited((*gsp).norm); }
}

pub unsafe fn synchronize_rcu_expedited() {
    unsafe { synchronize_rcu(); }
}

pub unsafe fn rcu_softirq_qs() {
    unsafe { rcu_qs(); }
}

#[macro_export]
macro_rules! rcu_note_context_switch {
    ($preempt:expr) => {{
        unsafe {
            rcu_qs();
            rcu_tasks_qs(current, $preempt);
        }
    }};
}

pub const fn rcu_needs_cpu() -> ::core::ffi::c_int { 0 }

pub unsafe fn rcu_request_urgent_qs_task(_t: *mut task_struct) {}
pub unsafe fn rcu_virt_note_context_switch() {}
pub unsafe fn rcu_cpu_stall_reset() {}
pub const fn rcu_jiffies_till_stall_check() -> ::core::ffi::c_int { 21 * HZ as ::core::ffi::c_int }
pub unsafe fn rcu_irq_exit_check_preempt() {}
pub unsafe fn exit_rcu() {}
pub unsafe fn rcu_preempt_need_deferred_qs(_t: *mut task_struct) -> bool { false }
pub unsafe fn rcu_preempt_deferred_qs(_t: *mut task_struct) {}
pub unsafe fn rcu_end_inkernel_boot() {}
pub const fn rcu_inkernel_boot_has_ended() -> bool { true }
pub const fn rcu_is_watching() -> bool { true }
pub unsafe fn rcu_momentary_eqs() {}

/* Avoid RCU read-side critical sections leaking across. */
pub unsafe fn rcu_all_qs() { unsafe { barrier(); } }

/* RCUtree hotplug events */
pub const rcutree_prepare_cpu: *mut ::core::ffi::c_void = ::core::ptr::null_mut();
pub const rcutree_online_cpu: *mut ::core::ffi::c_void = ::core::ptr::null_mut();
pub const rcutree_offline_cpu: *mut ::core::ffi::c_void = ::core::ptr::null_mut();
pub const rcutree_dead_cpu: *mut ::core::ffi::c_void = ::core::ptr::null_mut();
pub const rcutree_dying_cpu: *mut ::core::ffi::c_void = ::core::ptr::null_mut();
pub unsafe fn rcutree_report_cpu_starting(_cpu: ::core::ffi::c_uint) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
