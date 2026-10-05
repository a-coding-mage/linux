// SPDX-License-Identifier: GPL-2.0
/* F07 primitive, callback, guard and stack-object boundary. No original owner
 * algorithm is retained as a C fallback. These are unqualified C runtime;
 * F00 alone composes the single original build_policy native envelope. */
#error "SOURCE ONLY HOLD: sched_ext deferred/kick native ABI and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_deferred_kick_bindings.h"

static void deferred_bal_cb_workfn(struct rq *rq)
{
	lupos_scx_core_deferred_bal_body(rq);
}
static void deferred_irq_workfn(struct irq_work *irq_work)
{
	lupos_scx_core_deferred_irq_body(irq_work);
}
static void kick_cpus_irq_workfn(struct irq_work *irq_work)
{
	lupos_scx_core_deferred_kick_irq_body(irq_work);
}

/* These guards deliberately retain the pinned counted local_interrupt_*
 * implementation and context-analysis attributes. They are NOT lowered to
 * ordinary irqsave/restore. Owner decisions and mutations remain in Rust. */
void lupos_scx_core_deferred_add_local_guard(struct rq *rq,
        struct scx_deferred_reenq_local *drl, u64 reenq_flags)
{
	guard(raw_spinlock_irqsave)(&rq->scx.deferred_reenq_lock);
	lupos_scx_core_deferred_add_local_body(rq, drl, reenq_flags);
}
void lupos_scx_core_deferred_add_user_guard(struct rq *rq,
        struct scx_deferred_reenq_user *dru, u64 reenq_flags)
{
	guard(raw_spinlock_irqsave)(&rq->scx.deferred_reenq_lock);
	lupos_scx_core_deferred_add_user_body(rq, dru, reenq_flags);
}
struct scx_sched *lupos_scx_core_deferred_pop_local_guard(struct rq *rq, u64 *flags)
{
	struct scx_sched *sch = NULL;

	scoped_guard (raw_spinlock, &rq->scx.deferred_reenq_lock) {
		sch = lupos_scx_core_deferred_pop_local_body(rq, flags);
	}
	return sch;
}
struct scx_dispatch_q *lupos_scx_core_deferred_pop_user_guard(struct rq *rq, u64 *flags)
{
	struct scx_dispatch_q *dsq = NULL;

	scoped_guard (raw_spinlock, &rq->scx.deferred_reenq_lock) {
		dsq = lupos_scx_core_deferred_pop_user_body(rq, flags);
	}
	return dsq;
}
u32 lupos_scx_core_deferred_with_tasks(struct scx_sched *sch, struct rq *rq, u64 flags)
{
	LIST_HEAD(tasks);

	return lupos_scx_core_deferred_reenq_local_body(sch, rq, flags, &tasks);
}
void lupos_scx_core_deferred_with_cursor(struct rq *rq, struct scx_dispatch_q *dsq,
        u64 reenq_flags, struct scx_sched *sch)
{
	struct scx_dsq_list_node cursor = INIT_DSQ_LIST_CURSOR(cursor, dsq, 0);

	lupos_scx_core_deferred_reenq_user_body(rq, dsq, reenq_flags, sch, &cursor);
}

struct rq *lupos_scx_core_deferred_irq_rq(struct irq_work *irq_work)
{
	return container_of(irq_work, struct rq, scx.deferred_irq_work);
}
struct rq *lupos_scx_core_deferred_local_rq(struct scx_dispatch_q *dsq)
{
	return container_of(dsq, struct rq, scx.local_dsq);
}
struct rq *lupos_scx_core_deferred_this_rq(void) { return this_rq(); }
struct rq *lupos_scx_core_deferred_cpu_rq(s32 cpu) { return cpu_rq(cpu); }
s32 lupos_scx_core_deferred_cpu_of(struct rq *rq) { return cpu_of(rq); }
s32 lupos_scx_core_deferred_task_cpu(struct task_struct *p) { return task_cpu(p); }
struct scx_sched *lupos_scx_core_deferred_task_sched(struct task_struct *p)
{
	return scx_task_sched(p);
}
struct scx_sched_pcpu *lupos_scx_core_deferred_this_pcpu(struct scx_sched *sch)
{
	return this_cpu_ptr(sch->pcpu);
}
struct scx_deferred_reenq_local *lupos_scx_core_deferred_local(struct scx_sched *sch,
        struct rq *rq)
{
	struct scx_sched_pcpu *sch_pcpu = per_cpu_ptr(sch->pcpu, cpu_of(rq));

	return &sch_pcpu->deferred_reenq_local;
}
struct scx_deferred_reenq_user *lupos_scx_core_deferred_user(struct scx_dispatch_q *dsq,
        struct rq *rq)
{
	struct scx_dsq_pcpu *dsq_pcpu = per_cpu_ptr(dsq->pcpu, cpu_of(rq));

	return &dsq_pcpu->deferred_reenq_user;
}
struct scx_sched *lupos_scx_core_deferred_local_sched(struct scx_deferred_reenq_local *drl)
{
	struct scx_sched_pcpu *sch_pcpu = container_of(drl, struct scx_sched_pcpu,
						     deferred_reenq_local);
	return sch_pcpu->sch;
}
struct scx_dispatch_q *lupos_scx_core_deferred_user_dsq(struct scx_deferred_reenq_user *dru)
{
	struct scx_dsq_pcpu *dsq_pcpu = container_of(dru, struct scx_dsq_pcpu,
						   deferred_reenq_user);
	return dsq_pcpu->dsq;
}
struct scx_deferred_reenq_local *lupos_scx_core_deferred_first_local(struct rq *rq)
{
	return list_first_entry_or_null(&rq->scx.deferred_reenq_locals,
				       struct scx_deferred_reenq_local, node);
}
struct scx_deferred_reenq_user *lupos_scx_core_deferred_first_user(struct rq *rq)
{
	return list_first_entry_or_null(&rq->scx.deferred_reenq_users,
				       struct scx_deferred_reenq_user, node);
}
struct task_struct *lupos_scx_core_deferred_first_task(struct list_head *head)
{
	return list_first_entry_or_null(head, struct task_struct, scx.dsq_list.node);
}
struct task_struct *lupos_scx_core_deferred_first_safe_task(struct list_head *head)
{
	/* list_for_each_entry_safe starts with a plain head->next, unlike
	 * list_first_entry_or_null's READ_ONCE. Map its sentinel to NULL. */
	struct list_head *next = head->next;

	return next == head ? NULL : list_entry(next, struct task_struct, scx.dsq_list.node);
}
struct task_struct *lupos_scx_core_deferred_next_task(struct task_struct *p,
        struct list_head *head)
{
	struct list_head *next = p->scx.dsq_list.node.next;

	return next == head ? NULL : list_entry(next, struct task_struct, scx.dsq_list.node);
}
struct scx_sched_pcpu *lupos_scx_core_deferred_first_kick(struct rq *rq)
{
	struct list_head *head = &rq->scx.sched_pcpus_to_kick;
	struct list_head *next = head->next;

	return next == head ? NULL : list_entry(next, struct scx_sched_pcpu, to_kick_node);
}
struct scx_sched_pcpu *lupos_scx_core_deferred_next_kick(struct scx_sched_pcpu *pcpu,
        struct rq *rq)
{
	struct list_head *next = pcpu->to_kick_node.next;

	return next == &rq->scx.sched_pcpus_to_kick ? NULL :
		list_entry(next, struct scx_sched_pcpu, to_kick_node);
}
bool lupos_scx_core_deferred_list_empty(struct list_head *head) { return list_empty(head); }
void lupos_scx_core_deferred_list_del_init(struct list_head *node) { list_del_init(node); }
void lupos_scx_core_deferred_list_move_tail(struct list_head *node, struct list_head *head)
{
	list_move_tail(node, head);
}
void lupos_scx_core_deferred_list_add_tail(struct list_head *node, struct list_head *head)
{
	list_add_tail(node, head);
}
u64 lupos_scx_core_deferred_flags_read_once(u64 *flags) { return READ_ONCE(*flags); }
void lupos_scx_core_deferred_flags_write_once(u64 *flags, u64 value) { WRITE_ONCE(*flags, value); }
u64 lupos_scx_core_deferred_dsq_id_read_once(struct scx_dispatch_q *dsq) { return READ_ONCE(dsq->id); }
void lupos_scx_core_deferred_mb(void) { smp_mb(); }
void lupos_scx_core_deferred_schedule(struct rq *rq)
{
	irq_work_queue_on(&rq->scx.deferred_irq_work, cpu_of(rq));
}
void lupos_scx_core_deferred_queue_kick(struct rq *rq)
{
	irq_work_queue(&rq->scx.kick_cpus_irq_work);
}
void lupos_scx_core_deferred_sync_kick(s32 cpu)
{
	irq_work_sync(&cpu_rq(cpu)->scx.kick_cpus_irq_work);
}

struct cpumask *lupos_scx_core_deferred_kick_mask(struct scx_sched_pcpu *pcpu)
{
	return pcpu->cpus_to_kick;
}
struct cpumask *lupos_scx_core_deferred_idle_mask(struct scx_sched_pcpu *pcpu)
{
	return pcpu->cpus_to_kick_if_idle;
}
struct cpumask *lupos_scx_core_deferred_preempt_mask(struct scx_sched_pcpu *pcpu)
{
	return pcpu->cpus_to_preempt;
}
struct cpumask *lupos_scx_core_deferred_wait_mask(struct scx_sched_pcpu *pcpu)
{
	return pcpu->cpus_to_wait;
}
struct cpumask *lupos_scx_core_deferred_sync_mask(struct rq *rq)
{
	return rq->scx.cpus_to_sync;
}
bool lupos_scx_core_deferred_mask_test(s32 cpu, const struct cpumask *mask)
{
	return cpumask_test_cpu(cpu, mask);
}
void lupos_scx_core_deferred_mask_set(s32 cpu, struct cpumask *mask) { cpumask_set_cpu(cpu, mask); }
void lupos_scx_core_deferred_mask_clear(s32 cpu, struct cpumask *mask) { cpumask_clear_cpu(cpu, mask); }
bool lupos_scx_core_deferred_next_cpu(const struct cpumask *mask, s32 *cpu)
{
	*cpu = find_next_bit(cpumask_bits(mask), small_cpumask_bits, *cpu);
	return *cpu < small_cpumask_bits;
}
bool lupos_scx_core_deferred_next_possible(s32 *cpu)
{
#if NR_CPUS == 1
	return *cpu < 1;
#else
	*cpu = find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits, *cpu);
	return *cpu < small_cpumask_bits;
#endif
}
bool lupos_scx_core_deferred_online(s32 cpu) { return cpu_online(cpu); }
bool lupos_scx_core_deferred_curr_idle(struct rq *rq) { return is_idle_task(rq->curr); }
bool lupos_scx_core_deferred_trylock(struct rq *rq) { return raw_spin_rq_trylock(rq); }
unsigned long lupos_scx_core_deferred_lock_irqsave(struct rq *rq)
{
	unsigned long flags;

	raw_spin_rq_lock_irqsave(rq, flags);
	return flags;
}
void lupos_scx_core_deferred_unlock_irqrestore(struct rq *rq, unsigned long flags)
{
	raw_spin_rq_unlock_irqrestore(rq, flags);
}
unsigned long lupos_scx_core_deferred_irq_save(void)
{
	unsigned long irq_flags;

	local_irq_save(irq_flags);
	return irq_flags;
}
void lupos_scx_core_deferred_irq_restore(unsigned long irq_flags) { local_irq_restore(irq_flags); }
void lupos_scx_core_deferred_dsq_lock(struct scx_dispatch_q *dsq) { raw_spin_lock(&dsq->lock); }
void lupos_scx_core_deferred_dsq_unlock(struct scx_dispatch_q *dsq) { raw_spin_unlock(&dsq->lock); }
void lupos_scx_core_deferred_resched(struct rq *rq) { resched_curr(rq); }
void lupos_scx_core_deferred_kick_sync_snapshot(s32 cpu, unsigned long *ksyncs,
        struct rq *rq)
{
	/* Preserve both original plain operands, including the native unsigned
	 * long width. IRQ-enabled F04 polling may overlap this store. */
	ksyncs[cpu] = rq->scx.kick_sync;
}
u64 lupos_scx_core_deferred_missing_caps(struct scx_sched *sch, s32 cpu, u64 caps)
{
	return scx_missing_caps(sch, cpu, caps);
}
u64 lupos_scx_core_deferred_preempt_caps(struct scx_sched_pcpu *pcpu, struct rq *rq)
{
	return scx_caps_for_preempt(pcpu->sch, rq, 0);
}
bool lupos_scx_core_deferred_revoke(struct rq *rq, struct task_struct *p)
{
	return scx_task_reenq_on_cap_revoke(rq, p);
}
void lupos_scx_core_deferred_reject(struct rq *rq) { scx_reenq_reject(rq); }

void lupos_scx_core_deferred_assert_schedule(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_core_deferred_assert_ddsp(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_core_deferred_assert_reenq_local(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_core_deferred_assert_pop_locals(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_core_deferred_assert_reenq_user(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_core_deferred_assert_pop_users(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_core_deferred_assert_idle(struct rq *rq) { lockdep_assert_rq_held(rq); }
bool lupos_scx_core_deferred_warn_ddsp(struct scx_dispatch_q *dsq)
{
	return WARN_ON_ONCE(dsq->id != SCX_DSQ_LOCAL);
}
bool lupos_scx_core_deferred_warn_tsr(u64 reenq_flags)
{
	return WARN_ON_ONCE(reenq_flags & __SCX_REENQ_TSR_MASK);
}
bool lupos_scx_core_deferred_warn_local_reason(struct task_struct *p)
{
	return WARN_ON_ONCE(p->scx.flags & SCX_TASK_REENQ_REASON_MASK);
}
bool lupos_scx_core_deferred_warn_user_reason(struct task_struct *p)
{
	return WARN_ON_ONCE(p->scx.flags & SCX_TASK_REENQ_REASON_MASK);
}
void lupos_scx_core_deferred_bug_builtin(u64 dsq_id) { BUG_ON(dsq_id & SCX_DSQ_FLAG_BUILTIN); }
void lupos_scx_core_deferred_error_dsq(struct scx_sched *sch, struct scx_dispatch_q *dsq)
{
	scx_error(sch, "DSQ 0x%llx not allowed for reenq", dsq->id);
}
void lupos_scx_core_deferred_error_nmi(struct scx_sched *sch)
{
	scx_error(sch, "scx_bpf_kick_cpu() called from NMI");
}
void lupos_scx_core_deferred_error_idle_flags(struct scx_sched *sch)
{
	scx_error(sch, "PREEMPT/WAIT cannot be used with SCX_KICK_IDLE");
}
void lupos_scx_core_deferred_event_reenq_denied(struct scx_sched *sch)
{
	__scx_add_event(sch, SCX_EV_SUB_REENQ_DENIED, 1);
}
void lupos_scx_core_deferred_event_immed(struct task_struct *p)
{
	__scx_add_event(scx_task_sched(p), SCX_EV_REENQ_IMMED, 1);
}
void lupos_scx_core_deferred_event_preempt_denied(struct scx_sched_pcpu *pcpu)
{
	__scx_add_event(pcpu->sch, SCX_EV_SUB_PREEMPT_DENIED, 1);
}
void lupos_scx_core_deferred_event_slice_denied(struct scx_sched_pcpu *pcpu)
{
	__scx_add_event(pcpu->sch, SCX_EV_SLICE_DENIED, 1);
}
void lupos_scx_core_deferred_event_kick_denied(struct scx_sched_pcpu *pcpu)
{
	__scx_add_event(pcpu->sch, SCX_EV_SUB_KICK_DENIED, 1);
}
void lupos_scx_core_deferred_event_idle_denied(struct scx_sched_pcpu *pcpu)
{
	__scx_add_event(pcpu->sch, SCX_EV_SUB_KICK_DENIED, 1);
}
bool lupos_scx_core_deferred_unlikely_bypass(struct scx_sched *sch)
{
	return unlikely(READ_ONCE(sch->bypass_depth));
}
bool lupos_scx_core_deferred_likely_not_bypass(struct scx_sched *sch)
{
	return likely(!READ_ONCE(sch->bypass_depth));
}
bool lupos_scx_core_deferred_unlikely_base_missing(struct scx_sched *sch, struct rq *rq)
{
	return unlikely(scx_missing_caps(sch, cpu_of(rq), SCX_CAP_BASE));
}
bool lupos_scx_core_deferred_unlikely_protected(struct rq *rq, struct task_struct *p)
{
	return unlikely((p->scx.flags & SCX_TASK_PROTECTED) || p == scx_rescuee(rq));
}
bool lupos_scx_core_deferred_unlikely_trylock_failed(struct rq *task_rq)
{
	return unlikely(!raw_spin_rq_trylock(task_rq));
}
bool lupos_scx_core_deferred_unlikely_invalid(u64 dsq_id)
{
	return unlikely(dsq_id == SCX_DSQ_INVALID);
}
bool lupos_scx_core_deferred_unlikely_preempt_missing(struct scx_sched_pcpu *pcpu,
        s32 cpu, u64 caps)
{
	return unlikely(scx_missing_caps(pcpu->sch, cpu, caps));
}
bool lupos_scx_core_deferred_unlikely_slice_denied(bool denied) { return unlikely(denied); }
bool lupos_scx_core_deferred_likely_idle_caps(struct scx_sched_pcpu *pcpu, s32 cpu)
{
	return likely(!scx_missing_caps(pcpu->sch, cpu, SCX_CAP_BASE));
}
bool lupos_scx_core_deferred_unlikely_no_syncs(struct scx_kick_syncs __rcu *ksyncs_pcpu)
{
	return unlikely(!ksyncs_pcpu);
}
bool lupos_scx_core_deferred_unlikely_nmi(void) { return unlikely(in_nmi()); }
bool lupos_scx_core_deferred_unlikely_idle_flags(u64 flags)
{
	return unlikely(flags & (SCX_KICK_PREEMPT | SCX_KICK_WAIT));
}

struct scx_kick_syncs *lupos_scx_core_deferred_alloc_syncs(s32 cpu)
{
	struct scx_kick_syncs *new_ksyncs;

	return kvzalloc_node(struct_size(new_ksyncs, syncs, nr_cpu_ids),
			    GFP_KERNEL, cpu_to_node(cpu));
}
void lupos_scx_core_deferred_free_syncs(struct scx_kick_syncs *to_free)
{
	kvfree_rcu(to_free, rcu);
}
unsigned long *lupos_scx_core_deferred_syncs_bh(struct scx_kick_syncs __rcu *ksyncs_pcpu)
{
	return rcu_dereference_bh(ksyncs_pcpu)->syncs;
}
void lupos_scx_core_deferred_warn_syncs(struct scx_kick_syncs __rcu **ksyncs)
{
	WARN_ON_ONCE(rcu_access_pointer(*ksyncs));
}
struct scx_kick_syncs *lupos_scx_core_deferred_replace_syncs(struct scx_kick_syncs __rcu **ksyncs)
{
	return rcu_replace_pointer(*ksyncs, NULL, true);
}
void lupos_scx_core_deferred_assign_syncs(struct scx_kick_syncs __rcu **ksyncs,
        struct scx_kick_syncs *new_ksyncs)
{
	rcu_assign_pointer(*ksyncs, new_ksyncs);
}
