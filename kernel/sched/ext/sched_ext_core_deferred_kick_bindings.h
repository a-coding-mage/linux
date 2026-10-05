/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_DEFERRED_KICK_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_DEFERRED_KICK_BINDINGS_H
/* F07 source-only proposal. F00 composes this into the single configured
 * native type universe and native translation unit. No layouts or values are
 * guessed here. Every leaf remains explicit, unqualified C runtime. */
#include "internal.h"
struct scx_kick_syncs; /* Exact definition is F00-owned in the common header. */

/* Typed synchronous owner continuations; native guards/stack nodes never
 * escape these calls. Native callback identities remain original statics. */
void lupos_scx_core_deferred_bal_body(struct rq *rq);
void lupos_scx_core_deferred_irq_body(struct irq_work *irq_work);
void lupos_scx_core_deferred_kick_irq_body(struct irq_work *irq_work);
void lupos_scx_core_deferred_add_local_body(struct rq *rq,
        struct scx_deferred_reenq_local *drl, u64 reenq_flags);
void lupos_scx_core_deferred_add_user_body(struct rq *rq,
        struct scx_deferred_reenq_user *dru, u64 reenq_flags);
struct scx_sched *lupos_scx_core_deferred_pop_local_body(struct rq *rq, u64 *flags);
struct scx_dispatch_q *lupos_scx_core_deferred_pop_user_body(struct rq *rq, u64 *flags);
u32 lupos_scx_core_deferred_reenq_local_body(struct scx_sched *sch, struct rq *rq,
        u64 reenq_flags, struct list_head *tasks);
void lupos_scx_core_deferred_reenq_user_body(struct rq *rq,
        struct scx_dispatch_q *dsq, u64 reenq_flags, struct scx_sched *sch,
        struct scx_dsq_list_node *cursor);

void lupos_scx_core_deferred_add_local_guard(struct rq *rq,
        struct scx_deferred_reenq_local *drl, u64 reenq_flags);
void lupos_scx_core_deferred_add_user_guard(struct rq *rq,
        struct scx_deferred_reenq_user *dru, u64 reenq_flags);
struct scx_sched *lupos_scx_core_deferred_pop_local_guard(struct rq *rq, u64 *flags);
struct scx_dispatch_q *lupos_scx_core_deferred_pop_user_guard(struct rq *rq, u64 *flags);
u32 lupos_scx_core_deferred_with_tasks(struct scx_sched *sch, struct rq *rq, u64 flags);
void lupos_scx_core_deferred_with_cursor(struct rq *rq, struct scx_dispatch_q *dsq,
        u64 reenq_flags, struct scx_sched *sch);

struct rq *lupos_scx_core_deferred_irq_rq(struct irq_work *irq_work);
struct rq *lupos_scx_core_deferred_local_rq(struct scx_dispatch_q *dsq);
struct rq *lupos_scx_core_deferred_this_rq(void);
struct rq *lupos_scx_core_deferred_cpu_rq(s32 cpu);
s32 lupos_scx_core_deferred_cpu_of(struct rq *rq);
s32 lupos_scx_core_deferred_task_cpu(struct task_struct *p);
struct scx_sched *lupos_scx_core_deferred_task_sched(struct task_struct *p);
struct scx_sched_pcpu *lupos_scx_core_deferred_this_pcpu(struct scx_sched *sch);
struct scx_deferred_reenq_local *lupos_scx_core_deferred_local(struct scx_sched *sch,
        struct rq *rq);
struct scx_deferred_reenq_user *lupos_scx_core_deferred_user(struct scx_dispatch_q *dsq,
        struct rq *rq);
struct scx_sched *lupos_scx_core_deferred_local_sched(struct scx_deferred_reenq_local *drl);
struct scx_dispatch_q *lupos_scx_core_deferred_user_dsq(struct scx_deferred_reenq_user *dru);
struct scx_deferred_reenq_local *lupos_scx_core_deferred_first_local(struct rq *rq);
struct scx_deferred_reenq_user *lupos_scx_core_deferred_first_user(struct rq *rq);
struct task_struct *lupos_scx_core_deferred_first_task(struct list_head *head);
struct task_struct *lupos_scx_core_deferred_first_safe_task(struct list_head *head);
struct task_struct *lupos_scx_core_deferred_next_task(struct task_struct *p,
        struct list_head *head);
struct scx_sched_pcpu *lupos_scx_core_deferred_first_kick(struct rq *rq);
struct scx_sched_pcpu *lupos_scx_core_deferred_next_kick(struct scx_sched_pcpu *pcpu,
        struct rq *rq);
bool lupos_scx_core_deferred_list_empty(struct list_head *head);
void lupos_scx_core_deferred_list_del_init(struct list_head *node);
void lupos_scx_core_deferred_list_move_tail(struct list_head *node, struct list_head *head);
void lupos_scx_core_deferred_list_add_tail(struct list_head *node, struct list_head *head);
u64 lupos_scx_core_deferred_flags_read_once(u64 *flags);
void lupos_scx_core_deferred_flags_write_once(u64 *flags, u64 value);
u64 lupos_scx_core_deferred_dsq_id_read_once(struct scx_dispatch_q *dsq);
void lupos_scx_core_deferred_mb(void);
void lupos_scx_core_deferred_schedule(struct rq *rq);
void lupos_scx_core_deferred_queue_kick(struct rq *rq);
void lupos_scx_core_deferred_sync_kick(s32 cpu);

/* Native masks retain both CONFIG_CPUMASK_OFFSTACK representations. */
struct cpumask *lupos_scx_core_deferred_kick_mask(struct scx_sched_pcpu *pcpu);
struct cpumask *lupos_scx_core_deferred_idle_mask(struct scx_sched_pcpu *pcpu);
struct cpumask *lupos_scx_core_deferred_preempt_mask(struct scx_sched_pcpu *pcpu);
struct cpumask *lupos_scx_core_deferred_wait_mask(struct scx_sched_pcpu *pcpu);
struct cpumask *lupos_scx_core_deferred_sync_mask(struct rq *rq);
bool lupos_scx_core_deferred_mask_test(s32 cpu, const struct cpumask *mask);
void lupos_scx_core_deferred_mask_set(s32 cpu, struct cpumask *mask);
void lupos_scx_core_deferred_mask_clear(s32 cpu, struct cpumask *mask);
/* *cpu starts at zero; on success Rust increments it exactly once. These
 * preserve for_each_set_bit's find_next_bit and small_cpumask_bits bound. */
bool lupos_scx_core_deferred_next_cpu(const struct cpumask *mask, s32 *cpu);
bool lupos_scx_core_deferred_next_possible(s32 *cpu);
bool lupos_scx_core_deferred_online(s32 cpu);
bool lupos_scx_core_deferred_curr_idle(struct rq *rq);
bool lupos_scx_core_deferred_trylock(struct rq *rq);
unsigned long lupos_scx_core_deferred_lock_irqsave(struct rq *rq);
void lupos_scx_core_deferred_unlock_irqrestore(struct rq *rq, unsigned long flags);
unsigned long lupos_scx_core_deferred_irq_save(void);
void lupos_scx_core_deferred_irq_restore(unsigned long flags);
void lupos_scx_core_deferred_dsq_lock(struct scx_dispatch_q *dsq);
void lupos_scx_core_deferred_dsq_unlock(struct scx_dispatch_q *dsq);
void lupos_scx_core_deferred_resched(struct rq *rq);
/* Plain array store and plain sequence read match ext.c:8485. F04's paired
 * native predicates can observe this slot during IRQ-enabled polling. This
 * is not an atomic operation or a whole-program memory-model qualification. */
void lupos_scx_core_deferred_kick_sync_snapshot(s32 cpu, unsigned long *ksyncs,
        struct rq *rq);
u64 lupos_scx_core_deferred_missing_caps(struct scx_sched *sch, s32 cpu, u64 caps);
u64 lupos_scx_core_deferred_preempt_caps(struct scx_sched_pcpu *pcpu, struct rq *rq);
bool lupos_scx_core_deferred_revoke(struct rq *rq, struct task_struct *p);
void lupos_scx_core_deferred_reject(struct rq *rq);

/* Independent original lockdep/diagnostic sites keep operand spelling. */
void lupos_scx_core_deferred_assert_schedule(struct rq *rq);
void lupos_scx_core_deferred_assert_ddsp(struct rq *rq);
void lupos_scx_core_deferred_assert_reenq_local(struct rq *rq);
void lupos_scx_core_deferred_assert_pop_locals(struct rq *rq);
void lupos_scx_core_deferred_assert_reenq_user(struct rq *rq);
void lupos_scx_core_deferred_assert_pop_users(struct rq *rq);
void lupos_scx_core_deferred_assert_idle(struct rq *rq);
bool lupos_scx_core_deferred_warn_ddsp(struct scx_dispatch_q *dsq);
bool lupos_scx_core_deferred_warn_tsr(u64 reenq_flags);
bool lupos_scx_core_deferred_warn_local_reason(struct task_struct *p);
bool lupos_scx_core_deferred_warn_user_reason(struct task_struct *p);
void lupos_scx_core_deferred_bug_builtin(u64 dsq_id);
void lupos_scx_core_deferred_error_dsq(struct scx_sched *sch, struct scx_dispatch_q *dsq);
void lupos_scx_core_deferred_error_nmi(struct scx_sched *sch);
void lupos_scx_core_deferred_error_idle_flags(struct scx_sched *sch);
void lupos_scx_core_deferred_event_reenq_denied(struct scx_sched *sch);
void lupos_scx_core_deferred_event_immed(struct task_struct *p);
void lupos_scx_core_deferred_event_preempt_denied(struct scx_sched_pcpu *pcpu);
void lupos_scx_core_deferred_event_slice_denied(struct scx_sched_pcpu *pcpu);
void lupos_scx_core_deferred_event_kick_denied(struct scx_sched_pcpu *pcpu);
void lupos_scx_core_deferred_event_idle_denied(struct scx_sched_pcpu *pcpu);

/* Existing DISABLE_BRANCH_PROFILING applies. Hints retain site expressions
 * where native primitives need them, with no native-to-Rust hint trampoline. */
bool lupos_scx_core_deferred_unlikely_bypass(struct scx_sched *sch);
bool lupos_scx_core_deferred_likely_not_bypass(struct scx_sched *sch);
bool lupos_scx_core_deferred_unlikely_base_missing(struct scx_sched *sch, struct rq *rq);
bool lupos_scx_core_deferred_unlikely_protected(struct rq *rq, struct task_struct *p);
bool lupos_scx_core_deferred_unlikely_trylock_failed(struct rq *task_rq);
bool lupos_scx_core_deferred_unlikely_invalid(u64 dsq_id);
bool lupos_scx_core_deferred_unlikely_preempt_missing(struct scx_sched_pcpu *pcpu,
        s32 cpu, u64 caps);
bool lupos_scx_core_deferred_unlikely_slice_denied(bool denied);
bool lupos_scx_core_deferred_likely_idle_caps(struct scx_sched_pcpu *pcpu, s32 cpu);
bool lupos_scx_core_deferred_unlikely_no_syncs(struct scx_kick_syncs __rcu *ksyncs_pcpu);
bool lupos_scx_core_deferred_unlikely_nmi(void);
bool lupos_scx_core_deferred_unlikely_idle_flags(u64 flags);

/* F00 owns scx_kick_syncs storage and accessors. F07 owns allocation decisions,
 * unwind, IRQ drain ordering, and the native allocator/free primitives. */
struct scx_kick_syncs *lupos_scx_core_deferred_alloc_syncs(s32 cpu);
void lupos_scx_core_deferred_free_syncs(struct scx_kick_syncs *to_free);
unsigned long *lupos_scx_core_deferred_syncs_bh(struct scx_kick_syncs __rcu *ksyncs_pcpu);
void lupos_scx_core_deferred_warn_syncs(struct scx_kick_syncs __rcu **ksyncs);
struct scx_kick_syncs *lupos_scx_core_deferred_replace_syncs(struct scx_kick_syncs __rcu **ksyncs);
void lupos_scx_core_deferred_assign_syncs(struct scx_kick_syncs __rcu **ksyncs,
        struct scx_kick_syncs *new_ksyncs);
#endif
