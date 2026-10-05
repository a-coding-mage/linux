// SPDX-License-Identifier: GPL-2.0
/* F03 native primitive/diagnostic boundary. These leaves are unqualified C
 * runtime, never Rust algorithm coverage. F00 alone composes the original
 * single-translation-unit envelope. No old ext.c owner algorithm is called. */
#error "SOURCE ONLY HOLD: sched_ext consume native ABI and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_consume_bindings.h"

/* F07 owns this exact native callback target and its Rust algorithm. This is
 * only a same-TU declaration, not a definition, fallback or independent object.
 * The callback owner must be composed before future admission. */
static void deferred_bal_cb_workfn(struct rq *rq);

struct scx_dispatch_q *lupos_scx_core_consume_resolve_local(struct scx_sched *sch,
        struct rq *rq, struct task_struct *p, u64 *flags)
{
	return scx_resolve_local_dsq(sch, rq, p, flags);
}
void lupos_scx_core_consume_list_add_tail(struct list_head *node, struct list_head *head)
{
	list_add_tail(node, head);
}
s32 lupos_scx_core_consume_cpu_of(struct rq *rq)
{
	return cpu_of(rq);
}
s32 lupos_scx_core_consume_task_cpu(struct task_struct *p)
{
	return task_cpu(p);
}
s32 lupos_scx_core_consume_raw_cpu(void)
{
	return raw_smp_processor_id();
}
void lupos_scx_core_consume_set_task_cpu(struct task_struct *p, struct rq *dst_rq)
{
	/* Native sched.h selects the external SMP or static-inline UP form. */
	set_task_cpu(p, cpu_of(dst_rq));
}
bool lupos_scx_core_consume_task_allowed(struct task_struct *p, s32 cpu)
{
	return task_allowed_on_cpu(p, cpu);
}
void lupos_scx_core_consume_lock(struct scx_dispatch_q *dsq)
{
	raw_spin_lock(&dsq->lock);
}
void lupos_scx_core_consume_unlock(struct scx_dispatch_q *dsq)
{
	raw_spin_unlock(&dsq->lock);
}
struct rq *lupos_scx_core_consume_local_rq(struct scx_dispatch_q *dsq)
{
	return container_of(dsq, struct rq, scx.local_dsq);
}
bool lupos_scx_core_consume_list_empty(struct scx_dispatch_q *dsq)
{
	return list_empty(&dsq->list);
}
void lupos_scx_core_consume_opss_set_release(struct task_struct *p, unsigned long state)
{
	atomic_long_set_release(&p->scx.ops_state, state);
}
unsigned long lupos_scx_core_consume_opss_read(struct task_struct *p)
{
	return atomic_long_read(&p->scx.ops_state);
}
struct scx_dsp_ctx *lupos_scx_core_consume_this_dsp_ctx(struct scx_sched *sch)
{
	return &this_cpu_ptr(sch->pcpu)->dsp_ctx;
}
struct scx_dsp_buf_ent *lupos_scx_core_consume_buf_entry(struct scx_dsp_ctx *dspc, u32 index)
{
	return &dspc->buf[index];
}
void lupos_scx_core_consume_queue_balance(struct rq *rq)
{
	queue_balance_callback(rq, &rq->scx.deferred_bal_cb,
				deferred_bal_cb_workfn);
}

void lupos_scx_core_consume_assert_local_src(struct scx_dispatch_q *src_dsq)
{
	lockdep_assert_held(&src_dsq->lock);
}
void lupos_scx_core_consume_assert_local_dst(struct rq *dst_rq)
{
	lockdep_assert_rq_held(dst_rq);
}
void lupos_scx_core_consume_assert_remote_src(struct rq *src_rq)
{
	lockdep_assert_rq_held(src_rq);
}
void lupos_scx_core_consume_assert_enforce_task_rq(struct task_struct *p)
{
	lockdep_assert_rq_held(task_rq(p));
}
void lupos_scx_core_consume_assert_unlink_dsq(struct scx_dispatch_q *dsq)
{
	lockdep_assert_held(&dsq->lock);
}
void lupos_scx_core_consume_assert_unlink_rq(struct rq *locked_rq)
{
	lockdep_assert_rq_held(locked_rq);
}
void lupos_scx_core_consume_assert_move_dsq(struct scx_dispatch_q *src_dsq)
{
	lockdep_assert_held(&src_dsq->lock);
}
void lupos_scx_core_consume_assert_move_rq(struct rq *src_rq)
{
	lockdep_assert_rq_held(src_rq);
}
void lupos_scx_core_consume_assert_balance_rq(struct rq *rq)
{
	lockdep_assert_rq_held(rq);
}

void lupos_scx_core_consume_warn_local_holding(struct task_struct *p)
{
	WARN_ON_ONCE(p->scx.holding_cpu >= 0);
}
void lupos_scx_core_consume_warn_remote_affinity(struct rq *dst_rq, struct task_struct *p)
{
	WARN_ON_ONCE(!cpumask_test_cpu(cpu_of(dst_rq), p->cpus_ptr));
}
void lupos_scx_core_consume_warn_remote_stash(struct rq *dst_rq)
{
	WARN_ON_ONCE(dst_rq->scx.remote_activate_enq_flags ||
		     dst_rq->scx.remote_activate_sch);
}
void lupos_scx_core_consume_warn_same_cpu(struct task_struct *p, s32 cpu)
{
	WARN_ON_ONCE(task_cpu(p) == cpu);
}
void lupos_scx_core_consume_warn_unlink_holding(struct task_struct *p)
{
	WARN_ON_ONCE(p->scx.holding_cpu >= 0);
}
bool lupos_scx_core_consume_warn_unlink_rq_changed(struct rq *src_rq, struct task_struct *p)
{
	return WARN_ON_ONCE(src_rq != task_rq(p));
}
bool lupos_scx_core_consume_warn_dispatch_rq_changed(struct rq *src_rq, struct task_struct *p)
{
	return WARN_ON_ONCE(src_rq != task_rq(p));
}
void lupos_scx_core_consume_bug_local_source(struct scx_dispatch_q *src_dsq)
{
	BUG_ON(src_dsq->id == SCX_DSQ_LOCAL);
}
void lupos_scx_core_consume_bug_not_queued(struct task_struct *p)
{
	BUG_ON(!(p->scx.flags & SCX_TASK_QUEUED));
}
void lupos_scx_core_consume_error_migration_disabled(struct scx_sched *sch,
        struct task_struct *p, s32 cpu)
{
	scx_error(sch, "SCX_DSQ_LOCAL[_ON] cannot move migration disabled %s[%d] from CPU %d to %d",
		  p->comm, p->pid, task_cpu(p), cpu);
}
void lupos_scx_core_consume_error_not_allowed(struct scx_sched *sch,
        struct task_struct *p, s32 cpu)
{
	scx_error(sch, "SCX_DSQ_LOCAL[_ON] target CPU %d not allowed for %s[%d]",
		  cpu, p->comm, p->pid);
}
void lupos_scx_core_consume_event_offline(struct scx_sched *sch)
{
	__scx_add_event(sch, SCX_EV_DISPATCH_LOCAL_DSQ_OFFLINE, 1);
}
void lupos_scx_core_consume_event_not_owned(struct scx_sched *sch)
{
	__scx_add_event(sch, SCX_EV_INSERT_NOT_OWNED, 1);
}

bool lupos_scx_core_consume_unlikely_migration_disabled(struct task_struct *p)
{
	return unlikely(is_migration_disabled(p));
}
bool lupos_scx_core_consume_likely_unlink_held(struct task_struct *p, s32 cpu)
{
	return likely(p->scx.holding_cpu == cpu);
}
bool lupos_scx_core_consume_unlikely_move_disallowed(bool disallowed)
{
	return unlikely(disallowed);
}
bool lupos_scx_core_consume_unlikely_aborting(struct scx_sched *sch)
{
	return unlikely(READ_ONCE(sch->aborting));
}
bool lupos_scx_core_consume_likely_consumed(bool consumed)
{
	return likely(consumed);
}
bool lupos_scx_core_consume_likely_dispatch_held(struct task_struct *p)
{
	return likely(p->scx.holding_cpu == raw_smp_processor_id());
}
bool lupos_scx_core_consume_unlikely_dispatch_disallowed(bool disallowed)
{
	return unlikely(disallowed);
}
bool lupos_scx_core_consume_unlikely_not_owned(struct scx_sched *sch, struct task_struct *p)
{
	return unlikely(!scx_task_on_sched(sch, p));
}
bool lupos_scx_core_consume_likely_claim(struct task_struct *p, unsigned long *opss)
{
	return likely(atomic_long_try_cmpxchg(&p->scx.ops_state, opss,
					    SCX_OPSS_DISPATCHING));
}
