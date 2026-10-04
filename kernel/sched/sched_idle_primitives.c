// SPDX-License-Identifier: GPL-2.0-only
/* ABI/compiler/architecture/header leaves plus declarative metadata. No idle.c
 * decision body is retained in C. See NATIVE-BOUNDARY.md for dependencies. */
#include "sched_idle_bindings.h"
struct rq *lupos_idle_this_rq(void) { return this_rq(); }
struct task_struct *lupos_idle_current(void) { return current; }
int lupos_idle_cpu(void) { return smp_processor_id(); }
void lupos_idle_irq_disable(void) { local_irq_disable(); }
void lupos_idle_irq_enable(void) { local_irq_enable(); }
void noinstr lupos_idle_raw_irq_disable(void) { raw_local_irq_disable(); }
void noinstr lupos_idle_raw_irq_enable(void) { raw_local_irq_enable(); }
bool lupos_idle_irqs_disabled(void) { return irqs_disabled(); }
void noinstr lupos_idle_cpu_relax(void) { cpu_relax(); }
bool noinstr lupos_idle_tif_need_resched(void) { return tif_need_resched(); }
bool lupos_idle_need_resched(void) { return need_resched(); }
bool lupos_idle_clr_polling_test(void) { return current_clr_polling_and_test(); }
void lupos_idle_set_polling(void) { __current_set_polling(); }
void lupos_idle_clr_polling(void) { __current_clr_polling(); }
bool lupos_idle_cpu_offline(int cpu) { return cpu_is_offline(cpu); }
void lupos_idle_preempt_set_need_resched(void) { preempt_set_need_resched(); }
void lupos_idle_preempt_fold_need_resched(void) { preempt_fold_need_resched(); }
void lupos_idle_preempt_disable(void) { preempt_disable(); }
void lupos_idle_preempt_enable(void) { preempt_enable(); }
void lupos_idle_set_task_need_resched(struct task_struct *p) { set_tsk_need_resched(p); }
void lupos_idle_mb_after_atomic(void) { smp_mb__after_atomic(); }
void lupos_idle_smp_wmb(void) { smp_wmb(); }
void lupos_idle_trace(unsigned int state, unsigned int cpu) { trace_cpu_idle(state, cpu); }
void lupos_idle_stop_critical_timings(void) { stop_critical_timings(); }
void lupos_idle_start_critical_timings(void) { start_critical_timings(); }
/* These leaves carry the exact ct_cpuidle transitions. Their own compiler
 * windows balance locally; Rust places its begin/end at the owning call sites. */
void noinstr lupos_idle_ct_enter(void)
{
	instrumentation_begin();
	lockdep_assert_irqs_disabled();
	trace_hardirqs_on_prepare();
	lockdep_hardirqs_on_prepare();
	instrumentation_end();
	ct_idle_enter();
	lockdep_hardirqs_on(_RET_IP_);
}
void noinstr lupos_idle_ct_exit(void)
{
	lockdep_hardirqs_off(_RET_IP_);
	ct_idle_exit();
}
/* tick_check_broadcast_expired() is a per-CPU bitmap/header access. It can be
 * called while RCU is not watching in the polling window. */
bool noinstr lupos_idle_broadcast_expired(void) { return tick_check_broadcast_expired(); }
#ifdef CONFIG_GENERIC_CLOCKEVENTS_BROADCAST_IDLE
DEFINE_STATIC_KEY_FALSE(arch_needs_tick_broadcast);
bool lupos_idle_needs_broadcast(void) { return static_branch_unlikely(&arch_needs_tick_broadcast); }
void lupos_idle_tick_broadcast_enter(void) { tick_broadcast_enter(); }
void lupos_idle_tick_broadcast_exit(void) { tick_broadcast_exit(); }
#endif
bool lupos_idle_tick_stopped(void) { return tick_nohz_tick_stopped(); }
void lupos_idle_stop_tick(void) { tick_nohz_idle_stop_tick(); }
void lupos_idle_retain_tick(void) { tick_nohz_idle_retain_tick(); }
void lupos_idle_restart_tick(void) { tick_nohz_idle_restart_tick(); }
void lupos_idle_tick_enter(void) { tick_nohz_idle_enter(); }
void lupos_idle_tick_exit(void) { tick_nohz_idle_exit(); }
bool lupos_idle_got_tick(void) { return tick_nohz_idle_got_tick(); }
struct cpuidle_device *lupos_idle_get_device(void) { return cpuidle_get_device(); }
struct cpuidle_driver *lupos_idle_get_cpu_driver(struct cpuidle_device *dev)
{ return cpuidle_get_cpu_driver(dev); }
bool lupos_idle_not_available(struct cpuidle_driver *drv, struct cpuidle_device *dev)
{ return cpuidle_not_available(drv, dev); }
int lupos_idle_enter_s2idle(struct cpuidle_driver *drv, struct cpuidle_device *dev, u64 max_latency_ns)
{ return cpuidle_enter_s2idle(drv, dev, max_latency_ns); }
int lupos_idle_enter(struct cpuidle_driver *drv, struct cpuidle_device *dev, int state)
{ return cpuidle_enter(drv, dev, state); }
int lupos_idle_find_deepest_state(struct cpuidle_driver *drv, struct cpuidle_device *dev, u64 max_latency_ns)
{ return cpuidle_find_deepest_state(drv, dev, max_latency_ns); }
int lupos_idle_select(struct cpuidle_driver *drv, struct cpuidle_device *dev, bool *stop_tick)
{ return cpuidle_select(drv, dev, stop_tick); }
void lupos_idle_reflect(struct cpuidle_device *dev, int state) { cpuidle_reflect(dev, state); }
void lupos_idle_use_deepest_state(u64 latency_ns) { cpuidle_use_deepest_state(latency_ns); }
bool lupos_idle_should_s2idle(void) { return idle_should_enter_s2idle(); }
s32 lupos_idle_wakeup_latency_qos_limit(void) { return cpu_wakeup_latency_qos_limit(); }
void lupos_idle_nohz_balance(int cpu) { nohz_run_idle_balance(cpu); }
void lupos_idle_rcu_flush(void) { rcu_nocb_flush_deferred_wakeup(); }
void lupos_idle_rcu_sleep_check(void) { rcu_sleep_check(); }
void lupos_idle_flush_smp_queue(void) { flush_smp_call_function_queue(); }
bool lupos_idle_patch_pending(struct task_struct *p) { return klp_patch_pending(p); }
void lupos_idle_update_patch(struct task_struct *p) { klp_update_patch_state(p); }
void lupos_idle_hrtimer_setup(struct hrtimer *timer, enum hrtimer_restart (*fn)(struct hrtimer *))
{ hrtimer_setup_on_stack(timer, fn, CLOCK_MONOTONIC, HRTIMER_MODE_REL_HARD); }
void lupos_idle_hrtimer_start(struct hrtimer *timer, s64 duration)
{ hrtimer_start(timer, ns_to_ktime(duration), HRTIMER_MODE_REL_PINNED_HARD); }
int lupos_idle_task_cpu(struct task_struct *p) { return task_cpu(p); }
bool lupos_idle_scx_enabled(void) { return scx_enabled(); }
bool lupos_idle_smt_active(void) { return sched_smt_active(); }
#ifdef CONFIG_SCHEDSTATS
bool lupos_idle_schedstat_enabled(void) { return schedstat_enabled(); }
#endif
u64 lupos_idle_rq_clock_task(struct rq *rq) { return rq_clock_task(rq); }
u64 lupos_idle_rq_clock(struct rq *rq) { return rq_clock(rq); }
void lupos_idle_assert_clock(struct rq *rq)
{ lockdep_assert_rq_held(rq); assert_clock_updated(rq); }
void lupos_idle_store_clock_idle(struct rq *rq, u64 value) { u64_u32_store(rq->clock_idle, value); }
void lupos_idle_store_clock_pelt_idle(struct rq *rq, u64 value) { u64_u32_store(rq->clock_pelt_idle, value); }
void lupos_idle_rq_lock_irq(struct rq *rq) { raw_spin_rq_lock_irq(rq); }
void lupos_idle_rq_unlock_irq(struct rq *rq) { raw_spin_rq_unlock_irq(rq); }
void lupos_idle_print_bad_schedule(void) { printk(KERN_ERR "bad: scheduling from the idle thread!\n"); }
void lupos_idle_dump_stack(void) { dump_stack(); }
void lupos_idle_cpuhp_report_dead(void) { cpuhp_report_idle_dead(); }
void lupos_idle_cpuhp_online(enum cpuhp_state state) { cpuhp_online_idle(state); }
void __noreturn lupos_idle_bug_switching_to(void) { BUG(); unreachable(); }
void __noreturn lupos_idle_bug_prio_changed(void) { BUG(); unreachable(); }
/* Each original WARN_ON_ONCE site retains independent native once state. */
void lupos_idle_warn_poll_negative(bool condition) { WARN_ON_ONCE(condition); }
bool lupos_idle_warn_irqs_disabled(bool condition) { return WARN_ON_ONCE(condition); }
void lupos_idle_warn_offline_resched(bool condition) { WARN_ON_ONCE(condition); }
void lupos_idle_warn_policy(bool condition) { WARN_ON_ONCE(condition); }
void lupos_idle_warn_affinity(bool condition) { WARN_ON_ONCE(condition); }
void lupos_idle_warn_kthread(bool condition) { WARN_ON_ONCE(condition); }
void lupos_idle_warn_no_setaffinity(bool condition) { WARN_ON_ONCE(condition); }
void lupos_idle_warn_duration(bool condition) { WARN_ON_ONCE(condition); }
void lupos_idle_warn_mm(bool condition) { WARN_ON_ONCE(condition); }
bool lupos_idle_warn_balance(bool condition) { return WARN_ON_ONCE(condition); }

/* Weak-linkage shells retain override semantics; fallback bodies are Rust. */
void __weak arch_cpu_idle_prepare(void) { lupos_idle_arch_prepare_default(); }
void __weak arch_cpu_idle_enter(void) { lupos_idle_arch_enter_default(); }
void __weak arch_cpu_idle_exit(void) { lupos_idle_arch_exit_default(); }
void __weak __noreturn arch_cpu_idle_dead(void) { lupos_idle_arch_dead_default(); }
void __weak arch_cpu_idle(void) { lupos_idle_arch_idle_default(); }
#ifdef CONFIG_GENERIC_IDLE_POLL_SETUP
__setup("nohlt", lupos_idle_poll_setup);
__setup("hlt", lupos_idle_nopoll_setup);
#endif
EXPORT_SYMBOL_GPL(play_idle_precise);
/* Canonical native class layout/section, with all owned callbacks in Rust. */
DEFINE_SCHED_CLASS(idle) = {
	.dequeue_task = lupos_idle_dequeue_task,
	.wakeup_preempt = lupos_idle_wakeup_preempt,
	.pick_task = pick_task_idle,
	.put_prev_task = lupos_idle_put_prev_task,
	.set_next_task = lupos_idle_set_next_task,
	.balance = lupos_idle_balance,
	.select_task_rq = lupos_idle_select_task_rq,
	.set_cpus_allowed = set_cpus_allowed_common,
	.task_tick = lupos_idle_task_tick,
	.prio_changed = lupos_idle_prio_changed,
	.switching_to = lupos_idle_switching_to,
	.update_curr = lupos_idle_update_curr,
};
