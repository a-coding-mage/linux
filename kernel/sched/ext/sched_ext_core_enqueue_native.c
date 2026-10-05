// SPDX-License-Identifier: GPL-2.0
/* F02 native primitive, diagnostic and typed callback boundary. Unqualified C
 * runtime. This is not an independent object or an original-ext.c fallback.
 * F00 owns storage and eventual single-native-envelope composition. */
#error "SOURCE ONLY HOLD: sched_ext enqueue native ABI and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_enqueue_bindings.h"

/* Exact original typed native callback target; comparator algorithm is Rust. */
static bool scx_dsq_priq_less(struct rb_node *node_a, const struct rb_node *node_b)
{
	return lupos_scx_core_enq_priq_less_body(node_a, node_b);
}
struct task_struct *lupos_scx_core_enq_priq_task(struct rb_node *node)
{
	return container_of(node, struct task_struct, scx.dsq_priq);
}
const struct task_struct *lupos_scx_core_enq_priq_task_const(const struct rb_node *node)
{
	return container_of(node, struct task_struct, scx.dsq_priq);
}
bool lupos_scx_core_enq_time_before64(u64 a, u64 b)
{
	return time_before64(a, b);
}
struct rq *lupos_scx_core_enq_local_dsq_rq(struct scx_dispatch_q *dsq)
{
	return container_of(dsq, struct rq, scx.local_dsq);
}
void lupos_scx_core_enq_nr_write_once(struct scx_dispatch_q *dsq, u32 nr)
{
	WRITE_ONCE(dsq->nr, nr);
}
void lupos_scx_core_enq_seq_write_once(struct scx_dispatch_q *dsq, u32 seq)
{
	WRITE_ONCE(dsq->seq, seq);
}
u64 lupos_scx_core_enq_slice_dfl_read_once(struct scx_sched *sch)
{
	return READ_ONCE(sch->slice_dfl);
}
void lupos_scx_core_enq_event_refill(struct scx_sched *sch)
{
	__scx_add_event(sch, SCX_EV_REFILL_SLICE_DFL, 1);
}
void lupos_scx_core_enq_event_slice_denied(struct scx_sched *sch)
{
	__scx_add_event(sch, SCX_EV_SLICE_DENIED, 1);
}
void lupos_scx_core_enq_schedule_reenq_local(struct rq *rq, u64 flags)
{
	/* Existing internal.h inline, not an F02-owned algorithm. Its root RCU
	 * lookup and schedule_dsq_reenq dependency remain native/unqualified. */
	scx_schedule_reenq_local(rq, flags);
}
bool lupos_scx_core_enq_has_dequeue(struct scx_sched *sch)
{
	return SCX_HAS_OP(sch, dequeue);
}
void lupos_scx_core_enq_call_dequeue(struct scx_sched *sch, struct rq *rq,
                                    struct task_struct *p, u64 deq_flags)
{
	SCX_CALL_OP_TASK(sch, dequeue, rq, p, deq_flags); /* ext.c:1513 */
}
void lupos_scx_core_enq_wakeup_preempt(struct rq *rq, struct task_struct *p, int flags)
{
	wakeup_preempt(rq, p, flags);
}
struct scx_dispatch_q *lupos_scx_core_enq_resolve_local(struct scx_sched *sch,
        struct rq *rq, struct task_struct *p, u64 *flags)
{
	/* Sub owner supplies enabled implementation and native disabled inline. */
	return scx_resolve_local_dsq(sch, rq, p, flags);
}
void lupos_scx_core_enq_lock_nested(struct scx_dispatch_q *dsq, u64 enq_flags)
{
	raw_spin_lock_nested(&dsq->lock,
		(enq_flags & SCX_ENQ_NESTED) ? SINGLE_DEPTH_NESTING : 0);
}
void lupos_scx_core_enq_lock(struct scx_dispatch_q *dsq)
{
	raw_spin_lock(&dsq->lock);
}
void lupos_scx_core_enq_unlock(struct scx_dispatch_q *dsq)
{
	raw_spin_unlock(&dsq->lock);
}
int lupos_scx_core_enq_task_cpu(const struct task_struct *p)
{
	return task_cpu(p);
}
bool lupos_scx_core_enq_priq_empty(struct scx_dispatch_q *dsq)
{
	return RB_EMPTY_ROOT(&dsq->priq);
}
void lupos_scx_core_enq_rb_add(struct task_struct *p, struct scx_dispatch_q *dsq)
{
	rb_add(&p->scx.dsq_priq, &dsq->priq, scx_dsq_priq_less);
}
struct rb_node *lupos_scx_core_enq_rb_prev(struct task_struct *p)
{
	return rb_prev(&p->scx.dsq_priq);
}
void lupos_scx_core_enq_rb_erase(struct task_struct *p, struct scx_dispatch_q *dsq)
{
	rb_erase(&p->scx.dsq_priq, &dsq->priq);
}
void lupos_scx_core_enq_rb_clear(struct task_struct *p)
{
	RB_CLEAR_NODE(&p->scx.dsq_priq);
}
void lupos_scx_core_enq_list_add_tail(struct list_head *node, struct list_head *head)
{
	list_add_tail(node, head);
}
struct task_struct *lupos_scx_core_enq_first_plain(struct scx_dispatch_q *dsq)
{
	return dsq->first_task; /* ext.c:1686: plain, deliberately not RCU read */
}
struct task_struct *lupos_scx_core_enq_first_access(struct scx_dispatch_q *dsq)
{
	return rcu_access_pointer(dsq->first_task); /* ext.c:1745 */
}
void lupos_scx_core_enq_first_assign(struct scx_dispatch_q *dsq, struct task_struct *p)
{
	rcu_assign_pointer(dsq->first_task, p);
}
unsigned long lupos_scx_core_enq_opss_read(struct task_struct *p)
{
	return atomic_long_read(&p->scx.ops_state);
}
void lupos_scx_core_enq_opss_set_release(struct task_struct *p, unsigned long state)
{
	atomic_long_set_release(&p->scx.ops_state, state);
}
void lupos_scx_core_enq_assert_dequeue_rq(struct rq *rq)
{
	lockdep_assert_rq_held(rq); /* ext.c:1758 */
}
void lupos_scx_core_enq_assert_locked_task_rq(struct task_struct *p)
{
	lockdep_assert_rq_held(task_rq(p)); /* ext.c:1813 */
}
void lupos_scx_core_enq_assert_locked_dsq(struct scx_dispatch_q *dsq)
{
	lockdep_assert_held(&dsq->lock); /* ext.c:1814 */
}
s32 lupos_scx_core_enq_cpu_ret(struct scx_sched *sch, s32 cpu_or_cid)
{
	return scx_cpu_ret(sch, cpu_or_cid);
}
struct rq *lupos_scx_core_enq_cpu_rq(s32 cpu)
{
	return cpu_rq(cpu);
}
bool lupos_scx_core_enq_is_err_task(struct task_struct *p)
{
	return IS_ERR(p);
}

/* Each diagnostic is its own macro expansion/state object, never an enum-
 * selected shared WARN or a format-string-only Rust substitute. */
void lupos_scx_core_enq_warn_immed_fallback(u64 enq_flags)
{
	WARN_ON_ONCE(!(enq_flags & SCX_ENQ_GDSQ_FALLBACK)); /* 1438 */
}
bool lupos_scx_core_enq_warn_inc_nonlocal(struct scx_dispatch_q *dsq)
{
	return WARN_ON_ONCE(dsq->id != SCX_DSQ_LOCAL); /* 1447 */
}
bool lupos_scx_core_enq_warn_dec_nonlocal(struct scx_dispatch_q *dsq)
{
	return WARN_ON_ONCE(dsq->id != SCX_DSQ_LOCAL); /* 1469 */
}
bool lupos_scx_core_enq_warn_dec_no_immed(struct rq *rq)
{
	return WARN_ON_ONCE(rq->scx.nr_immed <= 0); /* 1470 */
}
void lupos_scx_core_enq_warn_linked_dispatch(struct task_struct *p)
{
	WARN_ON_ONCE(p->scx.dsq || !list_empty(&p->scx.dsq_list.node)); /* 1600 */
}
void lupos_scx_core_enq_warn_priq_dispatch(struct task_struct *p)
{
	WARN_ON_ONCE((p->scx.dsq_flags & SCX_TASK_DSQ_ON_PRIQ) ||
		     !RB_EMPTY_NODE(&p->scx.dsq_priq)); /* 1601 */
}
void lupos_scx_core_enq_warn_unlink_empty(struct task_struct *p)
{
	WARN_ON_ONCE(list_empty(&p->scx.dsq_list.node)); /* 1734 */
}
void lupos_scx_core_enq_warn_holding_linked(struct task_struct *p)
{
	WARN_ON_ONCE(!list_empty(&p->scx.dsq_list.node)); /* 1797 */
}
void lupos_scx_core_enq_warn_ddsp_id(struct task_struct *p)
{
	WARN_ON_ONCE(p->scx.ddsp_dsq_id != SCX_DSQ_INVALID); /* 1879 */
}
void lupos_scx_core_enq_warn_ddsp_flags(struct task_struct *p)
{
	WARN_ON_ONCE(p->scx.ddsp_enq_flags); /* 1880 */
}
void lupos_scx_core_enq_warn_direct_opss(struct task_struct *p, unsigned long opss)
{
	WARN_ONCE(true, "sched_ext: %s[%d] has invalid ops state 0x%lx in direct_dispatch()",
		  p->comm, p->pid, opss); /* 1941-1942 */
}
void lupos_scx_core_enq_warn_linked_direct(struct task_struct *p)
{
	WARN_ON_ONCE(p->scx.dsq || !list_empty(&p->scx.dsq_list.node)); /* 1947 */
}
void lupos_scx_core_enq_error_destroyed(struct scx_sched *sch)
{
	scx_error(sch, "attempting to dispatch to a destroyed dsq");
}
void lupos_scx_core_enq_error_builtin_priq(struct scx_sched *sch)
{
	scx_error(sch, "cannot use vtime ordering for built-in DSQs");
}
void lupos_scx_core_enq_error_existing_fifo(struct scx_sched *sch, struct scx_dispatch_q *dsq)
{
	scx_error(sch, "DSQ ID 0x%016llx already had FIFO-enqueued tasks", dsq->id);
}
void lupos_scx_core_enq_error_existing_priq(struct scx_sched *sch, struct scx_dispatch_q *dsq)
{
	scx_error(sch, "DSQ ID 0x%016llx already had PRIQ-enqueued tasks", dsq->id);
}
void lupos_scx_core_enq_error_missing_dsq(struct scx_sched *sch, u64 dsq_id)
{
	scx_error(sch, "non-existent DSQ 0x%llx", dsq_id);
}
void lupos_scx_core_enq_error_already_dispatched(struct scx_sched *sch, struct task_struct *p)
{
	scx_error(sch, "%s[%d] already direct-dispatched", p->comm, p->pid);
}
void lupos_scx_core_enq_error_wrong_dispatched(struct scx_sched *sch,
        struct task_struct *ddsp_task, struct task_struct *p)
{
	scx_error(sch, "scheduling for %s[%d] but trying to direct-dispatch %s[%d]",
		  ddsp_task->comm, ddsp_task->pid, p->comm, p->pid);
}

bool lupos_scx_core_enq_unlikely_immed_nonlocal(struct scx_dispatch_q *dsq)
{
	return unlikely(dsq->id != SCX_DSQ_LOCAL); /* 1437 */
}
/* These three boolean markers retain direct Rust owner calls. Original
 * build_policy.o defines DISABLE_BRANCH_PROFILING, so no ftrace constant
 * record is repaired by adding C-to-Rust crossings solely for operand text.
 * Cross-language __builtin_expect propagation remains unqualified. */
bool lupos_scx_core_enq_unlikely_immed_wait(bool condition)
{
	return unlikely(condition); /* 1456 */
}
bool lupos_scx_core_enq_unlikely_post_nonlocal(struct scx_dispatch_q *dsq)
{
	return unlikely(dsq->id != SCX_DSQ_LOCAL); /* 1528 */
}
bool lupos_scx_core_enq_likely_set_preempt_slice(bool condition)
{
	return likely(condition); /* 1582 */
}
bool lupos_scx_core_enq_unlikely_destroyed(struct scx_dispatch_q *dsq)
{
	return unlikely(dsq->id == SCX_DSQ_INVALID); /* 1608 */
}
bool lupos_scx_core_enq_unlikely_builtin_priq(struct scx_dispatch_q *dsq, u64 enq_flags)
{
	return unlikely((dsq->id & SCX_DSQ_FLAG_BUILTIN) &&
		     (enq_flags & SCX_ENQ_DSQ_PRIQ)); /* 1617-1618 */
}
bool lupos_scx_core_enq_unlikely_existing_fifo(bool condition)
{
	return unlikely(condition); /* 1645-1646 */
}
bool lupos_scx_core_enq_unlikely_existing_priq(struct scx_dispatch_q *dsq)
{
	return unlikely(!RB_EMPTY_ROOT(&dsq->priq)); /* 1671 */
}
bool lupos_scx_core_enq_unlikely_deferred_linked(struct task_struct *p)
{
	return unlikely(!list_empty(&p->scx.dsq_list.node)); /* 1765 */
}
bool lupos_scx_core_enq_unlikely_missing_dsq(struct scx_dispatch_q *dsq)
{
	return unlikely(!dsq); /* 1847 */
}
bool lupos_scx_core_enq_unlikely_wrong_ddsp_task(struct task_struct *p, struct task_struct *ddsp_task)
{
	return unlikely(p != ddsp_task); /* 1868 */
}

/* Class callback adapter identities consumed by F17's native class table. */
static void enqueue_task_scx(struct rq *rq, struct task_struct *p, int core_enq_flags)
{
	lupos_scx_core_enq_enqueue_task_body(rq, p, core_enq_flags);
}
static bool dequeue_task_scx(struct rq *rq, struct task_struct *p, int core_deq_flags)
{
	return lupos_scx_core_enq_dequeue_task_body(rq, p, core_deq_flags);
}
static void yield_task_scx(struct rq *rq)
{
	lupos_scx_core_enq_yield_task_body(rq);
}
static bool yield_to_task_scx(struct rq *rq, struct task_struct *to)
{
	return lupos_scx_core_enq_yield_to_task_body(rq, to);
}
static void wakeup_preempt_scx(struct rq *rq, struct task_struct *p, int wake_flags)
{
	lupos_scx_core_enq_wakeup_preempt_body(rq, p, wake_flags);
}
int lupos_scx_core_enq_cpu_of(struct rq *rq)
{
	return cpu_of(rq);
}
bool lupos_scx_core_enq_migration_disabled(struct task_struct *p)
{
	return is_migration_disabled(p);
}
bool lupos_scx_core_enq_has_runnable(struct scx_sched *sch)
{
	return SCX_HAS_OP(sch, runnable);
}
bool lupos_scx_core_enq_has_stopping_outer(struct scx_sched *sch)
{
	return SCX_HAS_OP(sch, stopping); /* 2296 */
}
bool lupos_scx_core_enq_has_stopping_inner(struct scx_sched *sch)
{
	return SCX_HAS_OP(sch, stopping); /* 2298 */
}
bool lupos_scx_core_enq_has_quiescent(struct scx_sched *sch)
{
	return SCX_HAS_OP(sch, quiescent);
}
bool lupos_scx_core_enq_has_yield(struct scx_sched *sch)
{
	return SCX_HAS_OP(sch, yield); /* 2332 */
}
bool lupos_scx_core_enq_has_yield_to(struct scx_sched *sch)
{
	return SCX_HAS_OP(sch, yield); /* 2346 */
}

/* Each SCX_CALL macro remains a separate native site. Its original task/rq
 * expressions are evaluated inside the callback envelope, with real ops field
 * typing, task guards, owner checks and nested locked-rq save/restore. */
void lupos_scx_core_enq_call_enqueue(struct scx_sched *sch, struct rq *rq,
                                    struct task_struct *p, u64 enq_flags)
{
	SCX_CALL_OP_TASK(sch, enqueue, rq, p, enq_flags); /* 2059 */
}
void lupos_scx_core_enq_call_runnable(struct scx_sched *sch, struct rq *rq,
                                     struct task_struct *p, u64 enq_flags)
{
	SCX_CALL_OP_TASK(sch, runnable, rq, p, enq_flags); /* 2170 */
}
void lupos_scx_core_enq_call_stopping(struct scx_sched *sch, struct rq *rq, struct task_struct *p)
{
	SCX_CALL_OP_TASK(sch, stopping, rq, p, false); /* 2299 */
}
void lupos_scx_core_enq_call_quiescent(struct scx_sched *sch, struct rq *rq,
                                      struct task_struct *p, u64 deq_flags)
{
	SCX_CALL_OP_TASK(sch, quiescent, rq, p, deq_flags); /* 2303 */
}
void lupos_scx_core_enq_call_yield(struct scx_sched *sch, struct rq *rq, struct task_struct *p)
{
	SCX_CALL_OP_2TASKS_RET(sch, yield, rq, p, NULL); /* 2333, result unused */
}
bool lupos_scx_core_enq_call_yield_to(struct scx_sched *sch, struct rq *rq,
                                     struct task_struct *from, struct task_struct *to)
{
	return SCX_CALL_OP_2TASKS_RET(sch, yield, rq, from, to); /* 2347 */
}
void lupos_scx_core_enq_opss_set(struct task_struct *p, unsigned long state)
{
	atomic_long_set(&p->scx.ops_state, state);
}
bool lupos_scx_core_enq_opss_try_none(struct task_struct *p, unsigned long *opss)
{
	return atomic_long_try_cmpxchg(&p->scx.ops_state, opss, SCX_OPSS_NONE);
}
void lupos_scx_core_enq_runnable_cpu_write_once(struct task_struct *p, s32 cpu)
{
	WRITE_ONCE(p->scx.runnable_cpu, cpu);
}
void lupos_scx_core_enq_assert_runnable_rq(struct rq *rq)
{
	lockdep_assert_rq_held(rq); /* 2107 */
}
bool lupos_scx_core_enq_task_current(struct rq *rq, struct task_struct *p)
{
	return task_current(rq, p);
}
bool lupos_scx_core_enq_on_rq_migrating(struct task_struct *p)
{
	return task_on_rq_migrating(p);
}
void lupos_scx_core_enq_add_nr_running(struct rq *rq)
{
	add_nr_running(rq, 1);
}
void lupos_scx_core_enq_sub_nr_running(struct rq *rq)
{
	sub_nr_running(rq, 1);
}
struct task_struct *lupos_scx_core_enq_rq_donor(struct rq *rq)
{
	return rq->donor; /* Plain access to configured native union/field. */
}
void lupos_scx_core_enq_event_reenq_repeat(struct scx_sched *sch)
{
	__scx_add_event(sch, SCX_EV_REENQ_REPEAT, 1);
}
void lupos_scx_core_enq_event_bypass(struct scx_sched *sch)
{
	__scx_add_event(sch, SCX_EV_BYPASS_DISPATCH, 1);
}
void lupos_scx_core_enq_event_skip_exiting(struct scx_sched *sch)
{
	__scx_add_event(sch, SCX_EV_ENQ_SKIP_EXITING, 1);
}
void lupos_scx_core_enq_event_skip_migration(struct scx_sched *sch)
{
	__scx_add_event(sch, SCX_EV_ENQ_SKIP_MIGRATION_DISABLED, 1);
}
void lupos_scx_core_enq_event_select_fallback(struct scx_sched *sch)
{
	__scx_add_event(sch, SCX_EV_SELECT_CPU_FALLBACK, 1);
}
void lupos_scx_core_enq_exit_reenq(struct scx_sched *sch, struct rq *rq, struct task_struct *p)
{
	__scx_exit(sch, SCX_EXIT_ERROR_REENQ, 0, cpu_of(rq),
		   "%s[%d] reenqueued %u times without running",
		   p->comm, p->pid, p->scx.reenq_cnt);
}
void lupos_scx_core_enq_warn_not_queued(struct task_struct *p)
{
	WARN_ON_ONCE(!(p->scx.flags & SCX_TASK_QUEUED)); /* 1983 */
}
void lupos_scx_core_enq_warn_enqueue_opss(struct task_struct *p)
{
	WARN_ON_ONCE(atomic_long_read(&p->scx.ops_state) != SCX_OPSS_NONE); /* 2052 */
}
void lupos_scx_core_enq_warn_enqueue_ddsp(struct task_struct **ddsp_taskp)
{
	WARN_ON_ONCE(*ddsp_taskp); /* 2056 */
}
/* Exact original helper identity in the warning's stringified expression.
 * This typed adapter does not duplicate the Rust runnable-membership body. */
static bool task_runnable(const struct task_struct *p)
{
	return lupos_scx_core_enq_task_runnable_body(p);
}
void lupos_scx_core_enq_warn_queued_not_runnable(const struct task_struct *p)
{
	WARN_ON_ONCE(!task_runnable(p)); /* 2160 */
}
void lupos_scx_core_enq_warn_unqueued_runnable(const struct task_struct *p)
{
	WARN_ON_ONCE(task_runnable(p)); /* 2277 */
}
__noreturn void lupos_scx_core_enq_bug_queueing(void)
{
	BUG(); /* 2208 */
}
void lupos_scx_core_enq_bug_after_dispatch(struct task_struct *p)
{
	BUG_ON(atomic_long_read(&p->scx.ops_state) != SCX_OPSS_NONE); /* 2243 */
}
bool lupos_scx_core_enq_likely_online(struct rq *rq)
{
	return likely((rq->scx.flags & SCX_RQ_ONLINE) && cpu_active(cpu_of(rq))); /* 1972 */
}
bool lupos_scx_core_enq_unlikely_reenq_limit(struct task_struct *p)
{
	return unlikely(p->scx.reenq_cnt > SCX_REENQ_MAX_REPEAT); /* 2007 */
}
bool lupos_scx_core_enq_unlikely_exiting(struct task_struct *p)
{
	return unlikely(p->flags & PF_EXITING); /* 2033 */
}
bool lupos_scx_core_enq_unlikely_no_enqueue(struct scx_sched *sch)
{
	return unlikely(!SCX_HAS_OP(sch, enqueue)); /* 2046 */
}
bool lupos_scx_core_enq_unlikely_restore(u64 enq_flags)
{
	return unlikely(enq_flags & ENQUEUE_RESTORE); /* 2154 */
}
bool lupos_scx_core_enq_unlikely_selected_fallback(struct rq *rq, struct task_struct *p)
{
	return unlikely(cpu_of(rq) != p->scx.selected_cpu); /* 2184 */
}
bool lupos_scx_core_enq_unlikely_lost_custody(struct task_struct *p)
{
	return unlikely(!(READ_ONCE(p->scx.flags) & SCX_TASK_IN_CUSTODY)); /* 2219 */
}
bool lupos_scx_core_enq_unlikely_stopping_rescue(struct task_struct *p, struct rq *rq)
{
	return unlikely(p == scx_rescuee(rq)); /* 2296 */
}
