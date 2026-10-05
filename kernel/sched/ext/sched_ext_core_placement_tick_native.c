// SPDX-License-Identifier: GPL-2.0
/* F05 native primitives, original callback identities and protection envelopes.
 * Unqualified runtime C. No independent object, copied owner algorithm or
 * fallback to ext.c. F00 owns the shared state and single native envelope. */
#error "SOURCE ONLY HOLD: sched_ext placement/tick native ABI and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_placement_tick_bindings.h"

/* F17 must use these exact static class/work callback identities in the one
 * native envelope. No function-pointer casts or replacement callback metadata. */
static int select_task_rq_scx(struct task_struct *p, int prev_cpu, int wake_flags)
{ return lupos_scx_core_pt_select_task_rq_body(p, prev_cpu, wake_flags); }
static void task_woken_scx(struct rq *rq, struct task_struct *p)
{ lupos_scx_core_pt_task_woken_body(rq, p); }
static void set_cpus_allowed_scx(struct task_struct *p, struct affinity_context *ac)
{ lupos_scx_core_pt_set_cpus_allowed_body(p, ac); }
static void rq_online_scx(struct rq *rq)
{ lupos_scx_core_pt_rq_online_body(rq); }
static void rq_offline_scx(struct rq *rq)
{ lupos_scx_core_pt_rq_offline_body(rq); }
static void scx_watchdog_workfn(struct work_struct *work)
{ lupos_scx_core_pt_watchdog_work_body(work); }
static void task_tick_scx(struct rq *rq, struct task_struct *curr, int queued)
{ lupos_scx_core_pt_task_tick_body(rq, curr, queued); }

struct scx_sched *lupos_scx_core_pt_task_sched(struct task_struct *p)
{ return scx_task_sched(p); }
s32 lupos_scx_core_pt_task_cpu(struct task_struct *p) { return task_cpu(p); }
s32 lupos_scx_core_pt_cpu_of(struct rq *rq) { return cpu_of(rq); }
struct rq *lupos_scx_core_pt_this_rq(void) { return this_rq(); }
bool lupos_scx_core_pt_unlikely_exec(int wake_flags)
{ return unlikely(wake_flags & WF_EXEC); }
bool lupos_scx_core_pt_likely_has_select(struct scx_sched *sch)
{ return likely(SCX_HAS_OP(sch, select_cpu)); }
void lupos_scx_core_pt_warn_direct_dispatch(struct task_struct **ddsp_taskp)
{ WARN_ON_ONCE(*ddsp_taskp); }
s32 lupos_scx_core_pt_call_select(struct scx_sched *sch, struct task_struct *p,
                                int prev_cpu, int wake_flags)
{
	/* scx_cpu_arg must be evaluated after kf_tasks[0] is installed by the
	 * task macro, not as an already-converted argument at the Rust boundary. */
	return SCX_CALL_OP_TASK_RET(sch, select_cpu, NULL, p,
				    scx_cpu_arg(prev_cpu), wake_flags);
}
s32 lupos_scx_core_pt_cpu_ret(struct scx_sched *sch, s32 cpu)
{ return scx_cpu_ret(sch, cpu); }
void lupos_scx_core_pt_event_bypass(struct scx_sched *sch)
{ __scx_add_event(sch, SCX_EV_BYPASS_DISPATCH, 1); }
void lupos_scx_core_pt_event_refill(struct scx_sched *sch)
{ __scx_add_event(sch, SCX_EV_REFILL_SLICE_DFL, 1); }
s32 lupos_scx_core_pt_select_default(struct task_struct *p, int prev_cpu,
                                   int wake_flags)
{
	/* Only call the separate idle owner's entry; no selection algorithm. The
	 * original implicit int -> u64 wake_flags conversion remains native. */
	return scx_select_cpu_dfl(p, prev_cpu, wake_flags, NULL, 0);
}
u64 lupos_scx_core_pt_slice_dfl_read_once(struct scx_sched *sch)
{ return READ_ONCE(sch->slice_dfl); }
void lupos_scx_core_pt_set_allowed_common(struct task_struct *p,
                                        struct affinity_context *ac)
{ set_cpus_allowed_common(p, ac); }
bool lupos_scx_core_pt_has_set_cpumask(struct scx_sched *sch)
{ return SCX_HAS_OP(sch, set_cpumask); }
struct rq *lupos_scx_core_pt_task_rq(struct task_struct *p) { return task_rq(p); }
struct scx_sched *lupos_scx_core_pt_root_protected(void)
{ return scx_root_protected(); }
bool lupos_scx_core_pt_enabled(void) { return scx_enabled(); }
void lupos_scx_core_pt_update_topology(struct scx_sched *sch)
{ scx_idle_update_selcpu_topology(&sch->ops); }
void lupos_scx_core_pt_online_ecaps(struct rq *rq) { scx_online_ecaps(rq); }
void lupos_scx_core_pt_offline_ecaps(struct rq *rq) { scx_offline_ecaps(rq); }
s16 *lupos_scx_core_pt_hotplug_cid_table(void)
{
	/* No extra RCU read section: CPU-hotplug serialization pins retirement,
	 * and the checked NULL result is valid after root enable failure. */
	return rcu_dereference_check(scx_cpu_to_cid_tbl, lockdep_is_cpus_held());
}
bool lupos_scx_core_pt_has_cpu_online(struct scx_sched *sch)
{ return SCX_HAS_OP(sch, cpu_online); }
bool lupos_scx_core_pt_has_cpu_offline(struct scx_sched *sch)
{ return SCX_HAS_OP(sch, cpu_offline); }
void lupos_scx_core_pt_call_cpu_online(struct scx_sched *sch, s32 cpu_or_cid)
{ SCX_CALL_OP(sch, cpu_online, NULL, cpu_or_cid); }
void lupos_scx_core_pt_call_cpu_offline(struct scx_sched *sch, s32 cpu_or_cid)
{ SCX_CALL_OP(sch, cpu_offline, NULL, cpu_or_cid); }
void lupos_scx_core_pt_exit_hotplug(struct scx_sched *sch, s32 cpu, bool online)
{
	scx_exit(sch, SCX_EXIT_UNREG_KERN,
		 SCX_ECODE_ACT_RESTART | SCX_ECODE_RSN_HOTPLUG,
		 "cpu %d going %s, exiting scheduler", cpu,
		 online ? "online" : "offline");
}
void lupos_scx_core_pt_rescue_flush(struct rq *rq) { scx_rescue_flush(rq); }

bool lupos_scx_core_pt_check_timeouts_locked(struct rq *rq)
{
	struct rq_flags rf;
	bool timed_out;

	/* ext.c:3709/3742 use ordinary explicit rq_lock_irqsave/restore, not a
	 * scoped guard. Keep that exact pair and stack-native rq_flags identity. */
	rq_lock_irqsave(rq, &rf);
	timed_out = lupos_scx_core_pt_check_timeouts_locked_body(rq);
	rq_unlock_irqrestore(rq, &rf);
	return timed_out;
}
struct scx_sched *lupos_scx_core_pt_timeout_root_bh(void)
{ return rcu_dereference_bh(scx_root); }
struct task_struct *lupos_scx_core_pt_runnable_task(struct list_head *node)
{ return list_entry(node, struct task_struct, scx.runnable_node); }
bool lupos_scx_core_pt_unlikely_task_timeout(struct scx_sched *sch,
                                           unsigned long last_runnable)
{
	/* Native unsigned-long addition/wrap, jiffies sampling, READ_ONCE and
	 * signed time_after comparison stay at their original expression. */
	return unlikely(time_after(jiffies,
			last_runnable + READ_ONCE(sch->watchdog_timeout)));
}
struct scx_dispatch_q *lupos_scx_core_pt_task_dsq_read_once(struct task_struct *p)
{ return READ_ONCE(p->scx.dsq); }
u32 lupos_scx_core_pt_duration_ms(unsigned long last)
{ return jiffies_to_msecs(jiffies - last); }
void lupos_scx_core_pt_exit_task_stall(struct scx_sched *sch, struct rq *rq,
                                     struct task_struct *p, u32 dur_ms)
{
	__scx_exit(sch, SCX_EXIT_ERROR_STALL, 0, cpu_of(rq),
		   "%s[%d] failed to run for %u.%03us", p->comm, p->pid,
		   dur_ms / 1000, dur_ms % 1000);
}
unsigned long lupos_scx_core_pt_jiffies(void) { return jiffies; }
bool lupos_scx_core_pt_online_cpu_condition(int *cpu)
{
	/* Exact for_each_online_cpu/for_each_set_bit condition. Rust supplies
	 * initial cpu=0 and the native int-compatible increment after the body.
	 * Do not use cpumask_next: this pinned macro uses small_cpumask_bits. */
#if NR_CPUS == 1
	return *cpu < 1;
#else
	*cpu = find_next_bit(cpumask_bits(cpu_online_mask), small_cpumask_bits, *cpu);
	return *cpu < small_cpumask_bits;
#endif
}
struct rq *lupos_scx_core_pt_cpu_rq(int cpu) { return cpu_rq(cpu); }
void lupos_scx_core_pt_cond_resched(void) { cond_resched(); }
unsigned long lupos_scx_core_pt_ulong_max(void) { return ULONG_MAX; }
void lupos_scx_core_pt_queue_watchdog(struct work_struct *work, unsigned long intv)
{ queue_delayed_work(system_dfl_wq, to_delayed_work(work), intv); }
struct scx_sched *lupos_scx_core_pt_tick_root_bh(void)
{ return rcu_dereference_bh(scx_root); }
bool lupos_scx_core_pt_unlikely_watchdog_timeout(struct scx_sched *root,
                                               unsigned long last_check)
{
	return unlikely(time_after(jiffies,
			last_check + READ_ONCE(root->watchdog_timeout)));
}
void lupos_scx_core_pt_exit_watchdog_stall(struct scx_sched *root, u32 dur_ms)
{
	scx_exit(root, SCX_EXIT_ERROR_STALL, 0,
		 "watchdog failed to check in for %u.%03us",
		 dur_ms / 1000, dur_ms % 1000);
}
void lupos_scx_core_pt_update_other_load_avgs(struct rq *rq)
{ update_other_load_avgs(rq); }
bool lupos_scx_core_pt_has_tick(struct scx_sched *sch)
{ return SCX_HAS_OP(sch, tick); }
void lupos_scx_core_pt_call_tick(struct scx_sched *sch, struct rq *rq,
                               struct task_struct *curr)
{ SCX_CALL_OP_TASK(sch, tick, rq, curr); }
void lupos_scx_core_pt_resched_curr(struct rq *rq) { resched_curr(rq); }
#ifdef CONFIG_NO_HZ_FULL
struct task_struct *lupos_scx_core_pt_rq_curr_plain(struct rq *rq)
{ return rq->curr; /* NOT READ_ONCE, NOT rq->donor. */ }
bool lupos_scx_core_pt_is_ext_task(struct task_struct *p)
{ return p->sched_class == &ext_sched_class; }
bool lupos_scx_core_pt_unlikely_rescuee(struct task_struct *p, struct rq *rq)
{ return unlikely(p == scx_rescuee(rq)); }
#endif
unsigned long lupos_scx_core_pt_refresh_interval_rcu(void)
{
	unsigned long intv;

	/* These are explicit ordinary calls in ext.c:6263-6266, not a guard.
	 * Keep the interval traversal wholly inside the native RCU lifetime. */
	rcu_read_lock();
	intv = lupos_scx_core_pt_refresh_interval_rcu_body();
	rcu_read_unlock();
	return intv;
}
struct scx_sched *lupos_scx_core_pt_sched_first_rcu(struct list_head *head)
{
	/* Exact list_for_each_entry_rcu initializer, including its independent
	 * configured lockdep warning site. The returned head sentinel must only
	 * be examined by sched_is_head, never dereferenced as a scheduler. */
	__list_check_rcu(dummy, 0);
	return list_entry_rcu(head->next, struct scx_sched, all);
}
bool lupos_scx_core_pt_sched_is_head(struct scx_sched *sch, struct list_head *head)
{ return &sch->all == head; }
struct scx_sched *lupos_scx_core_pt_sched_next_rcu(struct scx_sched *sch)
{ return list_entry_rcu(sch->all.next, struct scx_sched, all); }
void lupos_scx_core_pt_mod_watchdog(unsigned long intv)
{ mod_delayed_work(system_dfl_wq, lupos_scx_core_watchdog_work(), intv); }
void lupos_scx_core_pt_cancel_watchdog(void)
{ cancel_delayed_work_sync(lupos_scx_core_watchdog_work()); }
