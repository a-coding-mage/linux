/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_CONSUME_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_CONSUME_BINDINGS_H
/* F03 boundary only. Configured native headers own every layout and constant;
 * include in F00's single canonical binding/native envelope, never separately.
 * No local type aliases, state replicas, generated layouts or callback casts. */
#include "internal.h"

struct scx_dispatch_q *lupos_scx_core_consume_resolve_local(struct scx_sched *sch,
        struct rq *rq, struct task_struct *p, u64 *flags);
void lupos_scx_core_consume_list_add_tail(struct list_head *node, struct list_head *head);
s32 lupos_scx_core_consume_cpu_of(struct rq *rq);
s32 lupos_scx_core_consume_task_cpu(struct task_struct *p);
s32 lupos_scx_core_consume_raw_cpu(void);
void lupos_scx_core_consume_set_task_cpu(struct task_struct *p, struct rq *dst_rq);
bool lupos_scx_core_consume_task_allowed(struct task_struct *p, s32 cpu);
void lupos_scx_core_consume_lock(struct scx_dispatch_q *dsq);
void lupos_scx_core_consume_unlock(struct scx_dispatch_q *dsq);
struct rq *lupos_scx_core_consume_local_rq(struct scx_dispatch_q *dsq);
bool lupos_scx_core_consume_list_empty(struct scx_dispatch_q *dsq);
void lupos_scx_core_consume_opss_set_release(struct task_struct *p, unsigned long state);
unsigned long lupos_scx_core_consume_opss_read(struct task_struct *p);
struct scx_dsp_ctx *lupos_scx_core_consume_this_dsp_ctx(struct scx_sched *sch);
struct scx_dsp_buf_ent *lupos_scx_core_consume_buf_entry(struct scx_dsp_ctx *dspc, u32 index);
void lupos_scx_core_consume_queue_balance(struct rq *rq);

/* Nine distinct native lockdep sites, not coalesced wrapper diagnostics. */
void lupos_scx_core_consume_assert_local_src(struct scx_dispatch_q *src_dsq);
void lupos_scx_core_consume_assert_local_dst(struct rq *dst_rq);
void lupos_scx_core_consume_assert_remote_src(struct rq *src_rq);
void lupos_scx_core_consume_assert_enforce_task_rq(struct task_struct *p);
void lupos_scx_core_consume_assert_unlink_dsq(struct scx_dispatch_q *dsq);
void lupos_scx_core_consume_assert_unlink_rq(struct rq *locked_rq);
void lupos_scx_core_consume_assert_move_dsq(struct scx_dispatch_q *src_dsq);
void lupos_scx_core_consume_assert_move_rq(struct rq *src_rq);
void lupos_scx_core_consume_assert_balance_rq(struct rq *rq);

/* Distinct warning, bug, error and event expansions retain original operands;
 * configured metadata and cross-language attribution remain unqualified. */
void lupos_scx_core_consume_warn_local_holding(struct task_struct *p);
void lupos_scx_core_consume_warn_remote_affinity(struct rq *dst_rq, struct task_struct *p);
void lupos_scx_core_consume_warn_remote_stash(struct rq *dst_rq);
void lupos_scx_core_consume_warn_same_cpu(struct task_struct *p, s32 cpu);
void lupos_scx_core_consume_warn_unlink_holding(struct task_struct *p);
bool lupos_scx_core_consume_warn_unlink_rq_changed(struct rq *src_rq, struct task_struct *p);
bool lupos_scx_core_consume_warn_dispatch_rq_changed(struct rq *src_rq, struct task_struct *p);
void lupos_scx_core_consume_bug_local_source(struct scx_dispatch_q *src_dsq);
void lupos_scx_core_consume_bug_not_queued(struct task_struct *p);
void lupos_scx_core_consume_error_migration_disabled(struct scx_sched *sch,
        struct task_struct *p, s32 cpu);
void lupos_scx_core_consume_error_not_allowed(struct scx_sched *sch,
        struct task_struct *p, s32 cpu);
void lupos_scx_core_consume_event_offline(struct scx_sched *sch);
void lupos_scx_core_consume_event_not_owned(struct scx_sched *sch);

/* Original build_policy.o disables branch profiling; preserve that policy.
 * The three owner-call hints take already-evaluated Rust results. Native field
 * predicates stay inside their hints. Site counts do not qualify optimizer
 * effects or cross-language attribution, nor authorize a new build envelope. */
bool lupos_scx_core_consume_unlikely_migration_disabled(struct task_struct *p);
bool lupos_scx_core_consume_likely_unlink_held(struct task_struct *p, s32 cpu);
bool lupos_scx_core_consume_unlikely_move_disallowed(bool disallowed);
bool lupos_scx_core_consume_unlikely_aborting(struct scx_sched *sch);
bool lupos_scx_core_consume_likely_consumed(bool consumed);
bool lupos_scx_core_consume_likely_dispatch_held(struct task_struct *p);
bool lupos_scx_core_consume_unlikely_dispatch_disallowed(bool disallowed);
bool lupos_scx_core_consume_unlikely_not_owned(struct scx_sched *sch, struct task_struct *p);
bool lupos_scx_core_consume_likely_claim(struct task_struct *p, unsigned long *opss);

#endif /* LUPOS_SCHED_EXT_CORE_CONSUME_BINDINGS_H */
