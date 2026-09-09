/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Read-Copy Update mechanism for mutual exclusion (tree-based version)
 *
 * Copyright IBM Corporation, 2008
 *
 * Author: Dipankar Sarma <dipankar@in.ibm.com>
 *         Paul E. McKenney <paulmck@linux.ibm.com> Hierarchical algorithm
 *
 * Based on the original work by Paul McKenney <paulmck@linux.ibm.com>
 * and inputs from Rusty Russell, Andrea Arcangeli and Andi Kleen.
 *
 * For detailed explanation of Read-Copy Update mechanism see -
 *     Documentation/RCU
 */

// Dependencies supplied by other translation units.
#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rcu_gp_seq {
    pub norm: usize,
    pub exp: usize,
}

pub const NUM_ACTIVE_RCU_POLL_FULL_OLDSTATE: usize = 4;

unsafe extern "C" {
    pub fn rcu_softirq_qs();
    pub fn rcu_note_context_switch(preempt: bool);
    pub fn rcu_needs_cpu() -> i32;
    pub fn rcu_cpu_stall_reset();
    pub fn rcu_request_urgent_qs_task(t: *mut task_struct);

    /* Note a virtualization-based context switch.  The caller must have
     * disabled interrupts. */
    pub fn synchronize_rcu_expedited();
    pub fn rcu_barrier();
    pub fn rcu_momentary_eqs();

    pub fn start_poll_synchronize_rcu_expedited() -> usize;
    pub fn start_poll_synchronize_rcu_expedited_full(gsp: *mut rcu_gp_seq);
    pub fn cond_synchronize_rcu_expedited(oldstate: usize);
    pub fn cond_synchronize_rcu_expedited_full(gsp: *mut rcu_gp_seq);
    pub fn get_state_synchronize_rcu() -> usize;
    pub fn get_state_synchronize_rcu_full(gsp: *mut rcu_gp_seq);
    pub fn start_poll_synchronize_rcu() -> usize;
    pub fn start_poll_synchronize_rcu_full(gsp: *mut rcu_gp_seq);
    pub fn poll_state_synchronize_rcu(oldstate: usize) -> bool;
    pub fn poll_state_synchronize_rcu_full(gsp: *mut rcu_gp_seq) -> bool;
    pub fn cond_synchronize_rcu(oldstate: usize);
    pub fn cond_synchronize_rcu_full(gsp: *mut rcu_gp_seq);

    pub fn rcu_irq_exit_check_preempt();
    pub fn rcu_preempt_deferred_qs(t: *mut task_struct);
    pub fn exit_rcu();
    pub fn rcu_scheduler_starting();
    pub static mut rcu_scheduler_active: i32;
    pub fn rcu_end_inkernel_boot();
    pub fn rcu_inkernel_boot_has_ended() -> bool;
    pub fn rcu_is_watching() -> bool;

    pub fn rcutree_prepare_cpu(cpu: u32) -> i32;
    pub fn rcutree_online_cpu(cpu: u32) -> i32;
    pub fn rcutree_report_cpu_starting(cpu: u32);
    pub fn rcutree_migrate_callbacks(cpu: i32);
    pub fn rcutree_report_cpu_dead();
}

#[inline]
pub unsafe fn rcu_virt_note_context_switch() {
    rcu_note_context_switch(false);
}

#[inline]
pub unsafe fn same_state_synchronize_rcu_full(
    rgosp1: *const rcu_gp_seq,
    rgosp2: *const rcu_gp_seq,
) -> bool {
    (*rgosp1).norm == (*rgosp2).norm && (*rgosp1).exp == (*rgosp2).exp
}

// CONFIG_PROVE_RCU controls whether this hook performs its check.

// CONFIG_PREEMPT_RCU controls availability of rcu_all_qs.
unsafe extern "C" {
    pub fn rcu_all_qs();
}

// RCUtree hotplug events.  CONFIG_HOTPLUG_CPU controls these callbacks;
// when disabled, the corresponding C macros expand to NULL.
unsafe extern "C" {
    pub fn rcutree_dead_cpu(cpu: u32) -> i32;
    pub fn rcutree_dying_cpu(cpu: u32) -> i32;
    pub fn rcutree_offline_cpu(cpu: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
