// SPDX-License-Identifier: GPL-2.0-only
/*
 * Context tracking: Probe on high level context boundaries such as kernel,
 * userspace, guest or idle.
 *
 * This is used by RCU to remove its dependency on the timer tick while a CPU
 * runs in idle, userspace or guest mode.
 *
 * User/guest tracking started by Frederic Weisbecker:
 *
 * Copyright (C) 2012 Red Hat, Inc., Frederic Weisbecker
 *
 * Many thanks to Gilad Ben-Yossef, Paul McKenney, Ingo Molnar, Andrew Morton,
 * Steven Rostedt, Peter Zijlstra for suggestions and improvements.
 *
 * RCU extended quiescent state bits imported from kernel/rcu/tree.c
 * where the relevant authorship may be found.
 */

// Dependencies are supplied by the surrounding kernel translation.

#[cfg(feature = "context_tracking_idle")]
#[inline(always)]
unsafe fn rcu_task_exit() {
    #[cfg(all(feature = "tasks_rcu", feature = "no_hz_full"))]
    { (*current).rcu_tasks_idle_cpu = smp_processor_id(); }
}

#[cfg(feature = "context_tracking_idle")]
#[inline(always)]
unsafe fn rcu_task_enter() {
    #[cfg(all(feature = "tasks_rcu", feature = "no_hz_full"))]
    { (*current).rcu_tasks_idle_cpu = -1; }
}

#[cfg(feature = "context_tracking_idle")]
unsafe fn ct_kernel_exit_state(offset: i32) {
    // RCU is still watching. Better not be in extended quiescent state!
    WARN_ON_ONCE(IS_ENABLED(CONFIG_RCU_EQS_DEBUG) && !rcu_is_watching_curr_cpu());
    let _ = ct_state_inc(offset);
    // RCU is no longer watching.
}

#[cfg(feature = "context_tracking_idle")]
unsafe fn ct_kernel_enter_state(offset: i32) {
    let seq = ct_state_inc(offset);
    // RCU is now watching. Better not be in an extended quiescent state!
    WARN_ON_ONCE(IS_ENABLED(CONFIG_RCU_EQS_DEBUG) && (seq & CT_RCU_WATCHING) == 0);
}

#[cfg(feature = "context_tracking_idle")]
unsafe fn ct_kernel_exit(user: bool, offset: i32) {
    let ct = this_cpu_ptr(&mut context_tracking);
    WARN_ON_ONCE(ct_nmi_nesting() != CT_NESTING_IRQ_NONIDLE);
    (*ct).nmi_nesting = 0;
    WARN_ON_ONCE(IS_ENABLED(CONFIG_RCU_EQS_DEBUG) && ct_nesting() == 0);
    if ct_nesting() != 1 { (*ct).nesting -= 1; return; }
    instrumentation_begin();
    lockdep_assert_irqs_disabled();
    trace_rcu_watching(TPS("End"), ct_nesting(), 0, ct_rcu_watching());
    WARN_ON_ONCE(IS_ENABLED(CONFIG_RCU_EQS_DEBUG) && !user && !is_idle_task(current));
    rcu_preempt_deferred_qs(current);
    instrumentation_atomic_write(&mut (*ct).state, core::mem::size_of_val(&(*ct).state));
    instrumentation_end();
    (*ct).nesting = 0;
    ct_kernel_exit_state(offset);
    rcu_task_exit();
}

#[cfg(feature = "context_tracking_idle")]
unsafe fn ct_kernel_enter(user: bool, offset: i32) {
    let ct = this_cpu_ptr(&mut context_tracking);
    WARN_ON_ONCE(IS_ENABLED(CONFIG_RCU_EQS_DEBUG) && !raw_irqs_disabled());
    let oldval = ct_nesting();
    WARN_ON_ONCE(IS_ENABLED(CONFIG_RCU_EQS_DEBUG) && oldval < 0);
    if oldval != 0 { (*ct).nesting += 1; return; }
    rcu_task_enter();
    ct_kernel_enter_state(offset);
    instrumentation_begin();
    instrumentation_atomic_write(&mut (*ct).state, core::mem::size_of_val(&(*ct).state));
    trace_rcu_watching(TPS("Start"), ct_nesting(), 1, ct_rcu_watching());
    WARN_ON_ONCE(IS_ENABLED(CONFIG_RCU_EQS_DEBUG) && !user && !is_idle_task(current));
    (*ct).nesting = 1;
    WARN_ON_ONCE(ct_nmi_nesting() != 0);
    (*ct).nmi_nesting = CT_NESTING_IRQ_NONIDLE;
    instrumentation_end();
}

#[cfg(feature = "context_tracking_idle")]
pub unsafe fn ct_nmi_exit() {
    let ct = this_cpu_ptr(&mut context_tracking);
    instrumentation_begin();
    WARN_ON_ONCE(ct_nmi_nesting() <= 0);
    WARN_ON_ONCE(!rcu_is_watching_curr_cpu());
    if ct_nmi_nesting() != 1 {
        trace_rcu_watching(TPS("--="), ct_nmi_nesting(), ct_nmi_nesting() - 2, ct_rcu_watching());
        (*ct).nmi_nesting = ct_nmi_nesting() - 2;
        instrumentation_end(); return;
    }
    trace_rcu_watching(TPS("Endirq"), ct_nmi_nesting(), 0, ct_rcu_watching());
    (*ct).nmi_nesting = 0;
    instrumentation_atomic_write(&mut (*ct).state, core::mem::size_of_val(&(*ct).state));
    instrumentation_end();
    ct_kernel_exit_state(CT_RCU_WATCHING);
    if !in_nmi() { rcu_task_exit(); }
}

#[cfg(feature = "context_tracking_idle")]
pub unsafe fn ct_nmi_enter() {
    let mut incby: i64 = 2;
    let ct = this_cpu_ptr(&mut context_tracking);
    WARN_ON_ONCE(ct_nmi_nesting() < 0);
    if !rcu_is_watching_curr_cpu() {
        if !in_nmi() { rcu_task_enter(); }
        ct_kernel_enter_state(CT_RCU_WATCHING);
        instrumentation_begin();
        instrumentation_atomic_read(&(*ct).state, core::mem::size_of_val(&(*ct).state));
        instrumentation_atomic_write(&mut (*ct).state, core::mem::size_of_val(&(*ct).state));
        incby = 1;
    } else if !in_nmi() { instrumentation_begin(); rcu_irq_enter_check_tick(); }
    else { instrumentation_begin(); }
    trace_rcu_watching(if incby == 1 { TPS("Startirq") } else { TPS("++=") }, ct_nmi_nesting(), ct_nmi_nesting() + incby, ct_rcu_watching());
    (*ct).nmi_nesting = ct_nmi_nesting() + incby;
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[cfg(feature = "context_tracking_idle")]
pub unsafe fn ct_idle_enter() { WARN_ON_ONCE(IS_ENABLED(CONFIG_RCU_EQS_DEBUG) && !raw_irqs_disabled()); ct_kernel_exit(false, CT_RCU_WATCHING + CT_STATE_IDLE); }
#[cfg(feature = "context_tracking_idle")]
pub unsafe fn ct_idle_exit() { let mut flags = 0; raw_local_irq_save(&mut flags); ct_kernel_enter(false, CT_RCU_WATCHING - CT_STATE_IDLE); raw_local_irq_restore(flags); }
#[cfg(feature = "context_tracking_idle")]
pub unsafe fn ct_irq_enter() { lockdep_assert_irqs_disabled(); ct_nmi_enter(); }
#[cfg(feature = "context_tracking_idle")]
pub unsafe fn ct_irq_exit() { lockdep_assert_irqs_disabled(); ct_nmi_exit(); }
#[cfg(feature = "context_tracking_idle")]
pub unsafe fn ct_irq_enter_irqson() { let mut flags = 0; local_irq_save(&mut flags); ct_irq_enter(); local_irq_restore(flags); }
#[cfg(feature = "context_tracking_idle")]
pub unsafe fn ct_irq_exit_irqson() { let mut flags = 0; local_irq_save(&mut flags); ct_irq_exit(); local_irq_restore(flags); }

#[cfg(not(feature = "context_tracking_idle"))]
#[inline(always)] unsafe fn ct_kernel_exit(_user: bool, _offset: i32) {}
#[cfg(not(feature = "context_tracking_idle"))]
#[inline(always)] unsafe fn ct_kernel_enter(_user: bool, _offset: i32) {}

// User/guest context tracking is supplied under CONFIG_CONTEXT_TRACKING_USER.
#[cfg(feature = "context_tracking_user")]
unsafe fn context_tracking_recursion_enter() -> bool {
    let recursion = __this_cpu_inc_return(context_tracking.recursion);
    if recursion == 1 { return true; }
    WARN_ONCE(recursion < 1, "Invalid context tracking recursion value %d\n", recursion);
    __this_cpu_dec(context_tracking.recursion);
    false
}
#[cfg(feature = "context_tracking_user")]
#[inline(always)] unsafe fn context_tracking_recursion_exit() { __this_cpu_dec(context_tracking.recursion); }

// The remaining user-tracking entry points retain the kernel API and are
// intentionally expressed in terms of the declarations supplied by headers.
#[cfg(feature = "context_tracking_user")]
pub unsafe fn __ct_user_enter(state: ctx_state) {
    let ct = this_cpu_ptr(&mut context_tracking); lockdep_assert_irqs_disabled();
    WARN_ON_ONCE(!(*current).mm); if !context_tracking_recursion_enter() { return; }
    if __ct_state() != state {
        if (*ct).active { if state == CT_STATE_USER { instrumentation_begin(); trace_user_enter(0); vtime_user_enter(current); instrumentation_end(); } rcu_irq_work_resched(); ct_kernel_exit(true, CT_RCU_WATCHING + state); if !IS_ENABLED(CONFIG_CONTEXT_TRACKING_IDLE) { raw_atomic_set(&mut (*ct).state, state); } }
        else if !IS_ENABLED(CONFIG_CONTEXT_TRACKING_IDLE) { raw_atomic_set(&mut (*ct).state, state); }
        else { raw_atomic_add(state, &mut (*ct).state); }
    } context_tracking_recursion_exit();
}

#[cfg(feature = "context_tracking_user")]
pub unsafe fn ct_user_enter(state: ctx_state) { let mut flags = 0; if in_interrupt() { return; } local_irq_save(&mut flags); __ct_user_enter(state); local_irq_restore(flags); }
#[cfg(feature = "context_tracking_user")]
pub unsafe fn user_enter_callable() { user_enter(); }
#[cfg(feature = "context_tracking_user")]
pub unsafe fn __ct_user_exit(state: ctx_state) {
    let ct = this_cpu_ptr(&mut context_tracking); if !context_tracking_recursion_enter() { return; }
    if __ct_state() == state { if (*ct).active { ct_kernel_enter(true, CT_RCU_WATCHING - state); if state == CT_STATE_USER { instrumentation_begin(); vtime_user_exit(current); trace_user_exit(0); instrumentation_end(); } if !IS_ENABLED(CONFIG_CONTEXT_TRACKING_IDLE) { raw_atomic_set(&mut (*ct).state, CT_STATE_KERNEL); } } else if !IS_ENABLED(CONFIG_CONTEXT_TRACKING_IDLE) { raw_atomic_set(&mut (*ct).state, CT_STATE_KERNEL); } else { raw_atomic_sub(state, &mut (*ct).state); } }
    context_tracking_recursion_exit();
}
#[cfg(feature = "context_tracking_user")]
pub unsafe fn ct_user_exit(state: ctx_state) { let mut flags = 0; if in_interrupt() { return; } local_irq_save(&mut flags); __ct_user_exit(state); local_irq_restore(flags); }
#[cfg(feature = "context_tracking_user")]
pub unsafe fn user_exit_callable() { user_exit(); }

#[cfg(feature = "context_tracking_user")]
pub unsafe fn ct_cpu_track_user(cpu: i32) {
    static mut INITIALIZED: bool = false;
    if !per_cpu(context_tracking.active, cpu) {
        per_cpu(context_tracking.active, cpu) = true;
        static_branch_inc(&context_tracking_key);
    }
    if INITIALIZED { return; }
    #[cfg(feature = "have_tif_nohz")]
    set_tsk_thread_flag(&mut init_task, TIF_NOHZ);
    WARN_ON_ONCE(!tasklist_empty());
    INITIALIZED = true;
}

#[cfg(all(feature = "context_tracking_user", feature = "context_tracking_user_force"))]
pub unsafe fn context_tracking_init() {
    let mut cpu = 0;
    for_each_possible_cpu!(cpu) { ct_cpu_track_user(cpu); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
