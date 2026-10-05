/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_ENQUEUE_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_ENQUEUE_BINDINGS_H
/* F02 source-only native boundary. Include only in F00's single canonical
 * binding/native envelope. Layouts and values belong to configured original
 * headers; no storage or alternate type universe is created here. */
#include "internal.h"

bool lupos_scx_core_enq_priq_less_body(struct rb_node *a, const struct rb_node *b);
struct task_struct *lupos_scx_core_enq_priq_task(struct rb_node *node);
const struct task_struct *lupos_scx_core_enq_priq_task_const(const struct rb_node *node);
bool lupos_scx_core_enq_time_before64(u64 a, u64 b);
struct rq *lupos_scx_core_enq_local_dsq_rq(struct scx_dispatch_q *dsq);
void lupos_scx_core_enq_nr_write_once(struct scx_dispatch_q *dsq, u32 nr);
void lupos_scx_core_enq_seq_write_once(struct scx_dispatch_q *dsq, u32 seq);
u64 lupos_scx_core_enq_slice_dfl_read_once(struct scx_sched *sch);
void lupos_scx_core_enq_event_refill(struct scx_sched *sch);
void lupos_scx_core_enq_event_slice_denied(struct scx_sched *sch);
void lupos_scx_core_enq_schedule_reenq_local(struct rq *rq, u64 flags);
bool lupos_scx_core_enq_has_dequeue(struct scx_sched *sch);
void lupos_scx_core_enq_call_dequeue(struct scx_sched *sch, struct rq *rq,
                                    struct task_struct *p, u64 deq_flags);
void lupos_scx_core_enq_wakeup_preempt(struct rq *rq, struct task_struct *p, int flags);
struct scx_dispatch_q *lupos_scx_core_enq_resolve_local(struct scx_sched *sch,
        struct rq *rq, struct task_struct *p, u64 *flags);
void lupos_scx_core_enq_lock_nested(struct scx_dispatch_q *dsq, u64 enq_flags);
void lupos_scx_core_enq_lock(struct scx_dispatch_q *dsq);
void lupos_scx_core_enq_unlock(struct scx_dispatch_q *dsq);
int lupos_scx_core_enq_task_cpu(const struct task_struct *p);
bool lupos_scx_core_enq_priq_empty(struct scx_dispatch_q *dsq);
void lupos_scx_core_enq_rb_add(struct task_struct *p, struct scx_dispatch_q *dsq);
struct rb_node *lupos_scx_core_enq_rb_prev(struct task_struct *p);
void lupos_scx_core_enq_rb_erase(struct task_struct *p, struct scx_dispatch_q *dsq);
void lupos_scx_core_enq_rb_clear(struct task_struct *p);
void lupos_scx_core_enq_list_add_tail(struct list_head *node, struct list_head *head);
struct task_struct *lupos_scx_core_enq_first_plain(struct scx_dispatch_q *dsq);
struct task_struct *lupos_scx_core_enq_first_access(struct scx_dispatch_q *dsq);
void lupos_scx_core_enq_first_assign(struct scx_dispatch_q *dsq, struct task_struct *p);
unsigned long lupos_scx_core_enq_opss_read(struct task_struct *p);
void lupos_scx_core_enq_opss_set_release(struct task_struct *p, unsigned long state);
void lupos_scx_core_enq_assert_dequeue_rq(struct rq *rq);
void lupos_scx_core_enq_assert_locked_task_rq(struct task_struct *p);
void lupos_scx_core_enq_assert_locked_dsq(struct scx_dispatch_q *dsq);
s32 lupos_scx_core_enq_cpu_ret(struct scx_sched *sch, s32 cpu_or_cid);
struct rq *lupos_scx_core_enq_cpu_rq(s32 cpu);
bool lupos_scx_core_enq_is_err_task(struct task_struct *p);

/* Independent diagnostic expansions, preserving original expression evaluation. */
void lupos_scx_core_enq_warn_immed_fallback(u64 enq_flags);
bool lupos_scx_core_enq_warn_inc_nonlocal(struct scx_dispatch_q *dsq);
bool lupos_scx_core_enq_warn_dec_nonlocal(struct scx_dispatch_q *dsq);
bool lupos_scx_core_enq_warn_dec_no_immed(struct rq *rq);
void lupos_scx_core_enq_warn_linked_dispatch(struct task_struct *p);
void lupos_scx_core_enq_warn_priq_dispatch(struct task_struct *p);
void lupos_scx_core_enq_warn_unlink_empty(struct task_struct *p);
void lupos_scx_core_enq_warn_holding_linked(struct task_struct *p);
void lupos_scx_core_enq_warn_ddsp_id(struct task_struct *p);
void lupos_scx_core_enq_warn_ddsp_flags(struct task_struct *p);
void lupos_scx_core_enq_warn_direct_opss(struct task_struct *p, unsigned long opss);
void lupos_scx_core_enq_warn_linked_direct(struct task_struct *p);
void lupos_scx_core_enq_error_destroyed(struct scx_sched *sch);
void lupos_scx_core_enq_error_builtin_priq(struct scx_sched *sch);
void lupos_scx_core_enq_error_existing_fifo(struct scx_sched *sch, struct scx_dispatch_q *dsq);
void lupos_scx_core_enq_error_existing_priq(struct scx_sched *sch, struct scx_dispatch_q *dsq);
void lupos_scx_core_enq_error_missing_dsq(struct scx_sched *sch, u64 dsq_id);
void lupos_scx_core_enq_error_already_dispatched(struct scx_sched *sch, struct task_struct *p);
void lupos_scx_core_enq_error_wrong_dispatched(struct scx_sched *sch,
        struct task_struct *ddsp_task, struct task_struct *p);

/* One distinct native branch expansion per original site. Three marker leaves
 * keep direct Rust helper calls rather than adding native-to-Rust round trips.
 * Original build_policy.o disables branch profiling; optimization remains held. */
bool lupos_scx_core_enq_unlikely_immed_nonlocal(struct scx_dispatch_q *dsq);
bool lupos_scx_core_enq_unlikely_immed_wait(bool condition);
bool lupos_scx_core_enq_unlikely_post_nonlocal(struct scx_dispatch_q *dsq);
bool lupos_scx_core_enq_likely_set_preempt_slice(bool condition);
bool lupos_scx_core_enq_unlikely_destroyed(struct scx_dispatch_q *dsq);
bool lupos_scx_core_enq_unlikely_builtin_priq(struct scx_dispatch_q *dsq, u64 enq_flags);
bool lupos_scx_core_enq_unlikely_existing_fifo(bool condition);
bool lupos_scx_core_enq_unlikely_existing_priq(struct scx_dispatch_q *dsq);
bool lupos_scx_core_enq_unlikely_deferred_linked(struct task_struct *p);
bool lupos_scx_core_enq_unlikely_missing_dsq(struct scx_dispatch_q *dsq);
bool lupos_scx_core_enq_unlikely_wrong_ddsp_task(struct task_struct *p, struct task_struct *ddsp_task);

/* 1963-2374 continuation. Native class adapters preserve exact C ABI. */
bool lupos_scx_core_enq_task_runnable_body(const struct task_struct *p);
void lupos_scx_core_enq_enqueue_task_body(struct rq *rq, struct task_struct *p, int flags);
bool lupos_scx_core_enq_dequeue_task_body(struct rq *rq, struct task_struct *p, int flags);
void lupos_scx_core_enq_yield_task_body(struct rq *rq);
bool lupos_scx_core_enq_yield_to_task_body(struct rq *rq, struct task_struct *to);
void lupos_scx_core_enq_wakeup_preempt_body(struct rq *rq, struct task_struct *p, int flags);
int lupos_scx_core_enq_cpu_of(struct rq *rq);
bool lupos_scx_core_enq_migration_disabled(struct task_struct *p);
bool lupos_scx_core_enq_has_runnable(struct scx_sched *sch);
bool lupos_scx_core_enq_has_stopping_outer(struct scx_sched *sch);
bool lupos_scx_core_enq_has_stopping_inner(struct scx_sched *sch);
bool lupos_scx_core_enq_has_quiescent(struct scx_sched *sch);
bool lupos_scx_core_enq_has_yield(struct scx_sched *sch);
bool lupos_scx_core_enq_has_yield_to(struct scx_sched *sch);
void lupos_scx_core_enq_call_enqueue(struct scx_sched *sch, struct rq *rq,
                                    struct task_struct *p, u64 enq_flags);
void lupos_scx_core_enq_call_runnable(struct scx_sched *sch, struct rq *rq,
                                     struct task_struct *p, u64 enq_flags);
void lupos_scx_core_enq_call_stopping(struct scx_sched *sch, struct rq *rq, struct task_struct *p);
void lupos_scx_core_enq_call_quiescent(struct scx_sched *sch, struct rq *rq,
                                      struct task_struct *p, u64 deq_flags);
void lupos_scx_core_enq_call_yield(struct scx_sched *sch, struct rq *rq, struct task_struct *p);
bool lupos_scx_core_enq_call_yield_to(struct scx_sched *sch, struct rq *rq,
                                     struct task_struct *from, struct task_struct *to);
void lupos_scx_core_enq_opss_set(struct task_struct *p, unsigned long state);
bool lupos_scx_core_enq_opss_try_none(struct task_struct *p, unsigned long *opss);
void lupos_scx_core_enq_runnable_cpu_write_once(struct task_struct *p, s32 cpu);
void lupos_scx_core_enq_assert_runnable_rq(struct rq *rq);
bool lupos_scx_core_enq_task_current(struct rq *rq, struct task_struct *p);
bool lupos_scx_core_enq_on_rq_migrating(struct task_struct *p);
void lupos_scx_core_enq_add_nr_running(struct rq *rq);
void lupos_scx_core_enq_sub_nr_running(struct rq *rq);
struct task_struct *lupos_scx_core_enq_rq_donor(struct rq *rq);
void lupos_scx_core_enq_event_reenq_repeat(struct scx_sched *sch);
void lupos_scx_core_enq_event_bypass(struct scx_sched *sch);
void lupos_scx_core_enq_event_skip_exiting(struct scx_sched *sch);
void lupos_scx_core_enq_event_skip_migration(struct scx_sched *sch);
void lupos_scx_core_enq_event_select_fallback(struct scx_sched *sch);
void lupos_scx_core_enq_exit_reenq(struct scx_sched *sch, struct rq *rq, struct task_struct *p);
void lupos_scx_core_enq_warn_not_queued(struct task_struct *p);
void lupos_scx_core_enq_warn_enqueue_opss(struct task_struct *p);
void lupos_scx_core_enq_warn_enqueue_ddsp(struct task_struct **ddsp_taskp);
void lupos_scx_core_enq_warn_queued_not_runnable(const struct task_struct *p);
void lupos_scx_core_enq_warn_unqueued_runnable(const struct task_struct *p);
__noreturn void lupos_scx_core_enq_bug_queueing(void);
void lupos_scx_core_enq_bug_after_dispatch(struct task_struct *p);
bool lupos_scx_core_enq_likely_online(struct rq *rq);
bool lupos_scx_core_enq_unlikely_reenq_limit(struct task_struct *p);
bool lupos_scx_core_enq_unlikely_exiting(struct task_struct *p);
bool lupos_scx_core_enq_unlikely_no_enqueue(struct scx_sched *sch);
bool lupos_scx_core_enq_unlikely_restore(u64 enq_flags);
bool lupos_scx_core_enq_unlikely_selected_fallback(struct rq *rq, struct task_struct *p);
bool lupos_scx_core_enq_unlikely_lost_custody(struct task_struct *p);
bool lupos_scx_core_enq_unlikely_stopping_rescue(struct task_struct *p, struct rq *rq);

#endif /* LUPOS_SCHED_EXT_CORE_ENQUEUE_BINDINGS_H */
