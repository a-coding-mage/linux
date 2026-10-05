// SPDX-License-Identifier: GPL-2.0
/* Native primitives and fixed diagnostic/callback ABI; unqualified runtime C. */
#error "SOURCE ONLY HOLD: sched_ext sub rescue native admission is pending"
#include "sched_ext_sub_bindings.h"
#ifdef CONFIG_EXT_SUB_SCHED
/* Preserve separate native invocation sites for the original rq assertions. */
void lupos_scx_sub_assert_rescue_charge_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_sub_assert_rescue_end_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_sub_assert_rescue_keep_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_sub_assert_rescue_accrue_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_sub_assert_rescue_admit_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_sub_assert_rescue_overload_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_sub_assert_rescue_flush_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_sub_assert_reenq_reject_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
s32 lupos_scx_sub_cpu(struct rq *rq) { return cpu_of(rq); }
struct scx_sched *lupos_scx_sub_task_sched(struct task_struct *p) { return scx_task_sched(p); }
struct scx_sched_pcpu *lupos_scx_sub_pcpu(struct scx_sched *sch, s32 cpu) { return per_cpu_ptr(sch->pcpu, cpu); }
bool lupos_scx_sub_bypassing(struct scx_sched *sch, s32 cpu) { return scx_bypassing(sch, cpu); }
bool lupos_scx_sub_list_empty(const struct list_head *list) { return list_empty(list); }
u64 lupos_scx_sub_jiffies64(void) { return get_jiffies_64(); }
unsigned long lupos_scx_sub_jiffies(void) { return jiffies; }
unsigned long lupos_scx_sub_nsecs_jiffies(u64 ns) { return nsecs_to_jiffies(ns); }
unsigned long lupos_scx_sub_msecs_jiffies(u32 ms) { return msecs_to_jiffies(ms); }
bool lupos_scx_sub_time_before(unsigned long a, unsigned long b) { return time_before(a, b); }
bool lupos_scx_sub_time_before64(u64 a, u64 b) { return time_before64(a, b); }
bool lupos_scx_sub_timer_pending(struct timer_list *timer) { return timer_pending(timer); }
void lupos_scx_sub_add_timer(struct timer_list *timer, s32 cpu) { add_timer_on(timer, cpu); }
void lupos_scx_sub_timer_delete(struct timer_list *timer) { timer_delete(timer); }
static void lupos_scx_sub_rescue_timer(struct timer_list *timer)
{
	struct rq *rq = timer_container_of(rq, timer, scx.rescue.timer);
	guard(rq_lock_irqsave)(rq);
	lupos_scx_sub_rescue_timer_locked(rq);
}
void lupos_scx_sub_timer_setup(struct rq *rq)
{
	timer_setup(&rq->scx.rescue.timer, lupos_scx_sub_rescue_timer, TIMER_PINNED);
}
void lupos_scx_sub_set_slice(struct task_struct *p, u64 slice) { scx_set_task_slice(p, slice); }
void lupos_scx_sub_warn_rescue_current(struct rq *rq) { WARN_ON_ONCE(rq->scx.rescue.curr); }
struct task_struct *lupos_scx_sub_dsq_first(const struct scx_dispatch_q *dsq)
{
	return list_first_entry_or_null(&dsq->list, struct task_struct, scx.dsq_list.node);
}
struct task_struct *lupos_scx_sub_dsq_next(const struct scx_dispatch_q *dsq, struct task_struct *p)
{
	return list_is_last(&p->scx.dsq_list.node, &dsq->list) ? NULL :
		list_next_entry(p, scx.dsq_list.node);
}
struct scx_sched *lupos_scx_sub_all_next(struct scx_sched *pos)
{
	struct scx_sched *next;

	/* Retain list_for_each_entry_rcu()'s entry check and READ_ONCE sites;
	 * its head-derived sentinel must stay inside native list handling.
	 */
	if (!pos) {
		__list_check_rcu(dummy, 0);
		next = list_entry_rcu(scx_sched_all.next, struct scx_sched, all);
	} else {
		next = list_entry_rcu(pos->all.next, struct scx_sched, all);
	}
	return &next->all == &scx_sched_all ? NULL : next;
}
s32 lupos_scx_sub_exit_kind(struct scx_sched *sch) { return atomic_read(&sch->exit_kind); }
void lupos_scx_sub_exit_rescue(struct scx_sched *victim, s32 cpu, u64 avg, struct task_struct *p, unsigned long duration)
{
	u32 dur_ms = jiffies_to_msecs(duration);
	__scx_exit(victim, SCX_EXIT_ERROR_RESCUE, 0, cpu,
		   "used too much rescue CPU time (%llums) while %s[%d] waited %u.%03us to be rescued",
		   div_u64(avg, NSEC_PER_MSEC), p->comm, p->pid, dur_ms / 1000, dur_ms % 1000);
}
bool lupos_scx_sub_ext_above_current(struct rq *rq) { return sched_class_above(&ext_sched_class, rq->curr->sched_class); }
void lupos_scx_sub_resched(struct rq *rq) { resched_curr(rq); }
bool lupos_scx_sub_cpu_active(s32 cpu) { return cpu_active(cpu); }
void lupos_scx_sub_dump_rescue(struct seq_buf *s, struct rq *rq)
{
	struct task_struct *p = rq->scx.rescue.curr;
	scx_dump_line(s, "          rescue=%u budget=%lldus rescuing=%s[%d]",
		      rq->scx.rescue.dsq.nr, div_s64(rq->scx.rescue.budget, NSEC_PER_USEC),
		      p ? p->comm : "none", p ? p->pid : -1);
}
void lupos_scx_sub_warn_timeout(struct scx_sched *sch, unsigned long threshold)
{
	pr_warn("sched_ext: %s: watchdog timeout %ums <= rescue overload threshold %ums\n",
		sch->ops.name, jiffies_to_msecs(sch->watchdog_timeout), jiffies_to_msecs(threshold));
}
void lupos_scx_sub_warn_funding(struct scx_sched *sch, s64 period, unsigned long threshold)
{
	pr_warn("sched_ext: %s: rescue funding period %lldms > overload threshold %ums / 2\n",
		sch->ops.name, div_s64(period, NSEC_PER_MSEC), jiffies_to_msecs(threshold));
}
struct sched_ext_ops *lupos_scx_sub_ops(struct scx_sched *sch) { return &sch->ops; }
void lupos_scx_sub_bug_init_dsq(s32 ret) { BUG_ON(ret); }
bool lupos_scx_sub_has_subs(void) { return scx_has_subs(); }
s32 lupos_scx_sub_cpu_cid(s32 cpu) { return __scx_cpu_to_cid(cpu); }
u64 lupos_scx_sub_caps_for_enq(u64 flags) { return scx_caps_for_enq(flags); }
u64 lupos_scx_sub_caps_for_preempt(struct scx_sched *sch, struct rq *rq, u64 flags) { return scx_caps_for_preempt(sch, rq, flags); }
u64 lupos_scx_sub_missing_caps(struct scx_sched *sch, s32 cpu, u64 caps) { return scx_missing_caps(sch, cpu, caps); }
bool lupos_scx_sub_rq_online(struct rq *rq) { return scx_rq_online(rq); }
bool lupos_scx_sub_migration_disabled(struct task_struct *p) { return is_migration_disabled(p); }
void lupos_scx_sub_event_forced(struct scx_sched *sch) { __scx_add_event(sch, SCX_EV_SUB_FORCED_ADMIT, 1); }
void lupos_scx_sub_event_rescue(struct scx_sched *sch) { __scx_add_event(sch, SCX_EV_SUB_RESCUE, 1); }
struct task_struct *lupos_scx_sub_rescuee(struct rq *rq) { return scx_rescuee(rq); }
u64 lupos_scx_sub_caps_for_task(struct task_struct *p) { return scx_caps_for_task(p); }
void lupos_scx_sub_with_reject_list(struct rq *rq)
{
	LIST_HEAD(tasks);
	lupos_scx_sub_reject_with_list(rq, &tasks);
}
bool lupos_scx_sub_warn_migration_pending(struct task_struct *p) { return WARN_ON_ONCE(p->migration_pending); }
bool lupos_scx_sub_warn_reenq_flags(struct task_struct *p) { return WARN_ON_ONCE(p->scx.flags & SCX_TASK_REENQ_REASON_MASK); }
void lupos_scx_sub_task_list_add(struct task_struct *p, struct list_head *head) { list_add_tail(&p->scx.dsq_list.node, head); }
void lupos_scx_sub_task_list_del(struct task_struct *p) { list_del_init(&p->scx.dsq_list.node); }
struct task_struct *lupos_scx_sub_task_list_first(struct list_head *head)
{
	return list_first_entry_or_null(head, struct task_struct, scx.dsq_list.node);
}
struct task_struct *lupos_scx_sub_task_list_next(struct list_head *head, struct task_struct *p)
{
	return list_is_last(&p->scx.dsq_list.node, head) ? NULL : list_next_entry(p, scx.dsq_list.node);
}
#endif
