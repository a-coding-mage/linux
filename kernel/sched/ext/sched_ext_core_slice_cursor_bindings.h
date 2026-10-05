/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_SLICE_CURSOR_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_SLICE_CURSOR_BINDINGS_H

/* Source-only F01 native authority, extracted without changing the immutable
 * baseline. These private definitions originate in ext.c, not a public ABI
 * header. The common header includes this file exactly once in the future
 * single native envelope; no second ext.c definition may be composed with it.
 * Canonical generated native bindings are the only Rust type/value authority.
 */
#include "internal.h"

/* ext.c:483-494, exact native enum. */
enum scx_dsq_iter_flags {
	/* iterate in the reverse dispatch order */
	SCX_DSQ_ITER_REV		= 1U << 16,

	__SCX_DSQ_ITER_HAS_SLICE	= 1U << 30,
	__SCX_DSQ_ITER_HAS_VTIME	= 1U << 31,

	__SCX_DSQ_ITER_USER_FLAGS	= SCX_DSQ_ITER_REV,
	__SCX_DSQ_ITER_ALL_FLAGS	= __SCX_DSQ_ITER_USER_FLAGS |
					  __SCX_DSQ_ITER_HAS_SLICE |
					  __SCX_DSQ_ITER_HAS_VTIME,
};

/* ext.c:624-633, exact native types and alignment. F15 retains the original
 * size/alignment/flag BUILD_BUG_ON checks at iterator creation. Rust neither
 * allocates an invented opaque byte array nor supplies a lookalike layout.
 */
struct bpf_iter_scx_dsq_kern {
	struct scx_dsq_list_node	cursor;
	struct scx_dispatch_q		*dsq;
	u64				slice;
	u64				vtime;
} __attribute__((aligned(8)));

struct bpf_iter_scx_dsq {
	u64				__opaque[6];
} __attribute__((aligned(8)));

/* ext.c:1175-1183, exact native enum. The existing six Rust identities must
 * reference these values through the single canonical generated binding set.
 */
enum scx_slice_oob_consts {
	SCX_SLICE_OOB_DUR_BITS	= 43,
	SCX_SLICE_OOB_ID_BITS	= 64 - SCX_SLICE_OOB_DUR_BITS - 1,

	SCX_SLICE_OOB_DUR_MASK	= (1LLU << SCX_SLICE_OOB_DUR_BITS) - 1,
	SCX_SLICE_OOB_ID_SHIFT	= SCX_SLICE_OOB_DUR_BITS,
	SCX_SLICE_OOB_ID_MASK	= (1LLU << SCX_SLICE_OOB_ID_BITS) - 1,
	SCX_SLICE_OOB_PENDING	= 1LLU << 63,
};

/* Every pointer is borrowed synchronously. Callers retain the original DSQ/rq
 * locks, task/scheduler lifetime protection and scheduler context described in
 * the Rust Safety contracts. No leaf acquires ownership or retains a pointer.
 */
void lupos_scx_core_slice_assert_next_dsq(struct scx_dispatch_q *dsq);
void lupos_scx_core_slice_assert_cursor_dsq(struct scx_dispatch_q *dsq);
void lupos_scx_core_slice_assert_lost_rq(struct rq *rq);
void lupos_scx_core_slice_assert_lost_dsq(struct scx_dispatch_q *dsq);
void lupos_scx_core_slice_assert_set_slice(struct task_struct *p);
void lupos_scx_core_slice_assert_ended_rq(struct rq *rq);
void lupos_scx_core_slice_assert_apply_rq(struct rq *rq);
struct scx_sched *lupos_scx_core_slice_task_sched(struct task_struct *p);
struct scx_dsq_list_node *lupos_scx_core_slice_node_lnode(struct list_head *node);
struct task_struct *lupos_scx_core_slice_lnode_task(struct scx_dsq_list_node *node);
u32 lupos_scx_core_slice_cursor_seq(const struct scx_dsq_list_node *cursor);
void lupos_scx_core_slice_bug_invalid_cursor(struct scx_dsq_list_node *cursor);
bool lupos_scx_core_slice_warn_lost_rq(struct rq *rq, struct task_struct *p);
bool lupos_scx_core_slice_warn_head_cursor(struct scx_dsq_list_node *node);
bool lupos_scx_core_slice_list_empty(const struct list_head *head);
void lupos_scx_core_slice_list_add(struct list_head *node, struct list_head *head);
void lupos_scx_core_slice_list_move(struct list_head *node, struct list_head *head);
void lupos_scx_core_slice_list_move_tail(struct list_head *node, struct list_head *head);
void lupos_scx_core_slice_list_del_init(struct list_head *node);
u64 lupos_scx_core_slice_oob_read(struct task_struct *p);
void lupos_scx_core_slice_oob_set(struct task_struct *p, u64 value);
u64 lupos_scx_core_slice_oob_xchg_zero(struct task_struct *p);
u64 lupos_scx_core_slice_missing_base_caps(struct scx_sched *sch, struct rq *rq);
void lupos_scx_core_slice_event_clamped(struct scx_sched *sch);
void lupos_scx_core_slice_event_denied(struct scx_sched *sch);
struct task_struct *lupos_scx_core_slice_rescuee(struct rq *rq);
void lupos_scx_core_slice_rescue_end(struct rq *rq);
void lupos_scx_core_slice_rescue_charge(struct rq *rq, s64 delta_exec);

/* One distinct leaf per original branch marker: no callsite counters merge. */
bool lupos_scx_core_slice_unlikely_cursor_newer(bool condition);
bool lupos_scx_core_slice_unlikely_lost_task(bool condition);
bool lupos_scx_core_slice_unlikely_clear_pending(bool condition);
bool lupos_scx_core_slice_unlikely_protected(bool condition);
bool lupos_scx_core_slice_unlikely_ended_rescue(bool condition);
bool lupos_scx_core_slice_unlikely_clamp(bool condition);
bool lupos_scx_core_slice_likely_no_pending(bool condition);
bool lupos_scx_core_slice_unlikely_empty_exchange(bool condition);
bool lupos_scx_core_slice_unlikely_stale_owner(bool condition);
bool lupos_scx_core_slice_unlikely_missing_caps(bool condition);
bool lupos_scx_core_slice_unlikely_apply_protected(bool condition);
bool lupos_scx_core_slice_unlikely_nonpositive_delta(bool condition);
bool lupos_scx_core_slice_unlikely_current_rescue(bool condition);

#endif /* LUPOS_SCHED_EXT_CORE_SLICE_CURSOR_BINDINGS_H */
