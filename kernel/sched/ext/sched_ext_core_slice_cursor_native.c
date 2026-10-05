// SPDX-License-Identifier: GPL-2.0
/* F01 macro/inline/atomic leaves only. Native C runtime remains unqualified.
 * Keep in the future single native translation-unit envelope. No ext.c include,
 * copied F01 algorithm, independent object or build admission is supplied.
 */
#error "SOURCE ONLY HOLD: sched_ext slice/cursor native qualification incomplete"

#include "sched_ext_core_bindings.h"
#include "sched_ext_core_slice_cursor_bindings.h"

/* Keep each original lockdep assertion distinct; native once-only diagnostics
 * must not share one state object across unrelated original callsites.
 */
void lupos_scx_core_slice_assert_next_dsq(struct scx_dispatch_q *dsq)
{
	lockdep_assert_held(&dsq->lock); /* ext.c:510 */
}

void lupos_scx_core_slice_assert_cursor_dsq(struct scx_dispatch_q *dsq)
{
	lockdep_assert_held(&dsq->lock); /* ext.c:557 */
}

void lupos_scx_core_slice_assert_lost_rq(struct rq *rq)
{
	lockdep_assert_rq_held(rq); /* ext.c:599 */
}

void lupos_scx_core_slice_assert_lost_dsq(struct scx_dispatch_q *dsq)
{
	lockdep_assert_held(&dsq->lock); /* ext.c:600 */
}

void lupos_scx_core_slice_assert_set_slice(struct task_struct *p)
{
	lockdep_assert_rq_held(task_rq(p)); /* ext.c:1271 */
}

void lupos_scx_core_slice_assert_ended_rq(struct rq *rq)
{
	lockdep_assert_rq_held(rq); /* ext.c:1305 */
}

void lupos_scx_core_slice_assert_apply_rq(struct rq *rq)
{
	lockdep_assert_rq_held(rq); /* ext.c:1341 */
}

struct scx_sched *lupos_scx_core_slice_task_sched(struct task_struct *p)
{
	return scx_task_sched(p);
}

struct scx_dsq_list_node *lupos_scx_core_slice_node_lnode(struct list_head *node)
{
	return container_of(node, struct scx_dsq_list_node, node);
}

struct task_struct *lupos_scx_core_slice_lnode_task(struct scx_dsq_list_node *node)
{
	return container_of(node, struct task_struct, scx.dsq_list);
}

u32 lupos_scx_core_slice_cursor_seq(const struct scx_dsq_list_node *cursor)
{
	return cursor->priv;
}

void lupos_scx_core_slice_bug_invalid_cursor(struct scx_dsq_list_node *cursor)
{
	BUG_ON(!(cursor->flags & SCX_DSQ_LNODE_ITER_CURSOR));
}

bool lupos_scx_core_slice_warn_lost_rq(struct rq *rq, struct task_struct *p)
{
	return WARN_ON_ONCE(rq != task_rq(p));
}

bool lupos_scx_core_slice_warn_head_cursor(struct scx_dsq_list_node *node)
{
	return WARN_ON_ONCE(node->flags & SCX_DSQ_LNODE_ITER_CURSOR);
}

bool lupos_scx_core_slice_list_empty(const struct list_head *head)
{
	return list_empty(head);
}

void lupos_scx_core_slice_list_add(struct list_head *node, struct list_head *head)
{
	list_add(node, head);
}

void lupos_scx_core_slice_list_move(struct list_head *node, struct list_head *head)
{
	list_move(node, head);
}

void lupos_scx_core_slice_list_move_tail(struct list_head *node, struct list_head *head)
{
	list_move_tail(node, head);
}

void lupos_scx_core_slice_list_del_init(struct list_head *node)
{
	list_del_init(node);
}

u64 lupos_scx_core_slice_oob_read(struct task_struct *p)
{
	return atomic64_read(&p->scx.slice_oob);
}

void lupos_scx_core_slice_oob_set(struct task_struct *p, u64 value)
{
	atomic64_set(&p->scx.slice_oob, value);
}

u64 lupos_scx_core_slice_oob_xchg_zero(struct task_struct *p)
{
	return atomic64_xchg(&p->scx.slice_oob, 0);
}

u64 lupos_scx_core_slice_missing_base_caps(struct scx_sched *sch, struct rq *rq)
{
	return scx_missing_caps(sch, cpu_of(rq), SCX_CAP_BASE);
}

void lupos_scx_core_slice_event_clamped(struct scx_sched *sch)
{
	scx_add_event(sch, SCX_EV_SLICE_CLAMPED, 1);
}

void lupos_scx_core_slice_event_denied(struct scx_sched *sch)
{
	__scx_add_event(sch, SCX_EV_SLICE_DENIED, 1);
}

struct task_struct *lupos_scx_core_slice_rescuee(struct rq *rq)
{
	return scx_rescuee(rq);
}

void lupos_scx_core_slice_rescue_end(struct rq *rq)
{
	scx_rescue_end(rq);
}

void lupos_scx_core_slice_rescue_charge(struct rq *rq, s64 delta_exec)
{
	scx_rescue_charge(rq, delta_exec);
}

/* Branch markers remain native and distinct at each original decision. Their
 * instrumentation attribution and cross-language optimization need independent
 * qualification; no marker is discarded or coalesced into a shared counter.
 */
bool lupos_scx_core_slice_unlikely_cursor_newer(bool condition)
{
	return unlikely(condition); /* ext.c:568 */
}

bool lupos_scx_core_slice_unlikely_lost_task(bool condition)
{
	return unlikely(condition); /* ext.c:606-608 */
}

bool lupos_scx_core_slice_unlikely_clear_pending(bool condition)
{
	return unlikely(condition); /* ext.c:1217 */
}

bool lupos_scx_core_slice_unlikely_protected(bool condition)
{
	return unlikely(condition); /* ext.c:1273 */
}

bool lupos_scx_core_slice_unlikely_ended_rescue(bool condition)
{
	return unlikely(condition); /* ext.c:1308 */
}

bool lupos_scx_core_slice_unlikely_clamp(bool condition)
{
	return unlikely(condition); /* ext.c:1319 */
}

bool lupos_scx_core_slice_likely_no_pending(bool condition)
{
	return likely(condition); /* ext.c:1343 */
}

bool lupos_scx_core_slice_unlikely_empty_exchange(bool condition)
{
	return unlikely(condition); /* ext.c:1347 */
}

bool lupos_scx_core_slice_unlikely_stale_owner(bool condition)
{
	return unlikely(condition); /* ext.c:1351-1352 */
}

bool lupos_scx_core_slice_unlikely_missing_caps(bool condition)
{
	return unlikely(condition); /* ext.c:1359 */
}

bool lupos_scx_core_slice_unlikely_apply_protected(bool condition)
{
	return unlikely(condition); /* ext.c:1364 */
}

bool lupos_scx_core_slice_unlikely_nonpositive_delta(bool condition)
{
	return unlikely(condition); /* ext.c:1400 */
}

bool lupos_scx_core_slice_unlikely_current_rescue(bool condition)
{
	return unlikely(condition); /* ext.c:1406 */
}
