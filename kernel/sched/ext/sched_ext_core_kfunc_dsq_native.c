// SPDX-License-Identifier: GPL-2.0
/* F15 exact native kfunc ABI plus narrow, unqualified primitive/protection
 * boundaries. The future F00 composition retains the original build_policy
 * single translation unit and DISABLE_BRANCH_PROFILING policy. F13 alone owns
 * BTF metadata; F01 owns iterator types/cursor algorithms; no old C fallback. */
#error "SOURCE ONLY HOLD: sched_ext DSQ kfunc native ABI and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_kfunc_dsq_bindings.h"

/* The first two original groups remain separate around F13-owned BTF sets. */
__bpf_kfunc_start_defs();

__bpf_kfunc u32 scx_bpf_reenqueue_local(const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	return lupos_scx_core_kq_reenqueue_local_body(aux);
}

__bpf_kfunc_end_defs();

__bpf_kfunc_start_defs();

__bpf_kfunc s32 scx_bpf_create_dsq(u64 dsq_id, s32 node, const struct bpf_prog_aux *aux)
{
	return lupos_scx_core_kq_create_dsq_body(dsq_id, node, aux);
}

__bpf_kfunc_end_defs();

/* Original third scope crosses F07's non-kfunc scx_kick_cpu. Reconcile it in
 * the same-TU composition; this scope includes only F15's native kfunc shells. */
__bpf_kfunc_start_defs();

__bpf_kfunc bool scx_bpf_task_set_slice(struct task_struct *p, u64 slice,
					const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	return lupos_scx_core_kq_task_set_slice_body(p, slice, aux);
}

__bpf_kfunc bool scx_bpf_task_set_dsq_vtime(struct task_struct *p, u64 vtime,
					    const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	return lupos_scx_core_kq_task_set_dsq_vtime_body(p, vtime, aux);
}

__bpf_kfunc void scx_bpf_kick_cpu(s32 cpu, u64 flags, const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	lupos_scx_core_kq_kick_cpu_body(cpu, flags, aux);
}

__bpf_kfunc void scx_bpf_kick_cid(s32 cid, u64 flags, const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	lupos_scx_core_kq_kick_cid_body(cid, flags, aux);
}

__bpf_kfunc s32 scx_bpf_dsq_nr_queued(u64 dsq_id, const struct bpf_prog_aux *aux)
{
	s32 ret;

	/* Original explicit pair, not a newly scoped preempt guard. */
	preempt_disable();
	ret = lupos_scx_core_kq_dsq_nr_queued_body(dsq_id, aux);
	preempt_enable();
	return ret;
}

__bpf_kfunc void scx_bpf_destroy_dsq(u64 dsq_id, const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	lupos_scx_core_kq_destroy_dsq_body(dsq_id, aux);
}

__bpf_kfunc int bpf_iter_scx_dsq_new(struct bpf_iter_scx_dsq *it, u64 dsq_id,
				     u64 flags, const struct bpf_prog_aux *aux)
{
	struct bpf_iter_scx_dsq_kern *kit = (void *)it;

	BUILD_BUG_ON(sizeof(struct bpf_iter_scx_dsq_kern) >
		     sizeof(struct bpf_iter_scx_dsq));
	BUILD_BUG_ON(__alignof__(struct bpf_iter_scx_dsq_kern) !=
		     __alignof__(struct bpf_iter_scx_dsq));
	BUILD_BUG_ON(__SCX_DSQ_ITER_ALL_FLAGS &
		     ((1U << __SCX_DSQ_LNODE_PRIV_SHIFT) - 1));

	return lupos_scx_core_kq_iter_new_body(kit, dsq_id, flags, aux);
}

__bpf_kfunc struct task_struct *bpf_iter_scx_dsq_next(struct bpf_iter_scx_dsq *it)
{
	struct bpf_iter_scx_dsq_kern *kit = (void *)it;

	return lupos_scx_core_kq_iter_next_body(kit);
}

__bpf_kfunc void bpf_iter_scx_dsq_destroy(struct bpf_iter_scx_dsq *it)
{
	struct bpf_iter_scx_dsq_kern *kit = (void *)it;

	lupos_scx_core_kq_iter_destroy_body(kit);
}

__bpf_kfunc struct task_struct *scx_bpf_dsq_peek(u64 dsq_id,
						 const struct bpf_prog_aux *aux)
{
	/* Original KF_RCU_PROTECTED caller lifetime, with no additional guard. */
	return lupos_scx_core_kq_dsq_peek_body(dsq_id, aux);
}

__bpf_kfunc void scx_bpf_dsq_reenq(u64 dsq_id, u64 reenq_flags,
				   const struct bpf_prog_aux *aux)
{
	struct rq *locked_rq = scx_locked_rq();

	guard(preempt)();
	lupos_scx_core_kq_dsq_reenq_body(dsq_id, reenq_flags, aux, locked_rq);
}

__bpf_kfunc void scx_bpf_reenqueue_local___v2(const struct bpf_prog_aux *aux)
{
	lupos_scx_core_kq_reenqueue_local_v2_body(aux);
}

__bpf_kfunc_end_defs();

/* Existing header-owned helpers remain explicit native dependencies; none is
 * claimed as newly implemented Rust algorithm coverage. */
struct scx_sched *lupos_scx_core_kq_prog_sched(const struct bpf_prog_aux *aux)
{ return scx_prog_sched(aux); }
int lupos_scx_core_kq_processor_id(void) { return smp_processor_id(); }
struct rq *lupos_scx_core_kq_cpu_rq(s32 cpu) { return cpu_rq(cpu); }
struct rq *lupos_scx_core_kq_this_rq(void) { return this_rq(); }
int lupos_scx_core_kq_cpu_of(struct rq *rq) { return cpu_of(rq); }
void lupos_scx_core_kq_assert_reenqueue_rq(struct rq *rq)
{ lockdep_assert_rq_held(rq); }
struct scx_dispatch_q *lupos_scx_core_kq_alloc_dsq(s32 node)
{ return kmalloc_node(sizeof(struct scx_dispatch_q), GFP_KERNEL, node); }
void lupos_scx_core_kq_free_dsq(struct scx_dispatch_q *dsq) { kfree(dsq); }

s32 lupos_scx_core_kq_create_rcu(struct scx_dispatch_q *dsq,
                               const struct bpf_prog_aux *aux)
{
	s32 ret;

	/* Allocation/init and exit/free remain outside this exact explicit pair. */
	rcu_read_lock();
	ret = lupos_scx_core_kq_create_rcu_body(dsq, aux);
	rcu_read_unlock();
	return ret;
}

s32 lupos_scx_core_kq_hash_insert(struct scx_sched *sch, struct scx_dispatch_q *dsq)
{
	/* Same native by-value parameter object; F00 alone owns its storage. */
	return rhashtable_lookup_insert_fast(&sch->dsq_hash, &dsq->hash_node,
					    *lupos_scx_core_dsq_hash_params());
}
s32 lupos_scx_core_kq_runnable_cpu_read_once(struct task_struct *p)
{ return READ_ONCE(p->scx.runnable_cpu); }
bool lupos_scx_core_kq_task_current(struct rq *rq, struct task_struct *p)
{ return task_current(rq, p); }
void lupos_scx_core_kq_event_slice_denied(struct scx_sched *sch)
{ __scx_add_event(sch, SCX_EV_SLICE_DENIED, 1); }
s32 lupos_scx_core_kq_cid_to_cpu(struct scx_sched *sch, s32 cid)
{ return scx_cid_to_cpu(sch, cid); }
s32 lupos_scx_core_kq_cpu_ret(struct scx_sched *sch, s32 cpu_or_cid)
{ return scx_cpu_ret(sch, cpu_or_cid); }
/* Preserve separate READ_ONCE sites and native u32 count type. */
u32 lupos_scx_core_kq_local_nr_read_once(struct rq *rq)
{ return READ_ONCE(rq->scx.local_dsq.nr); }
u32 lupos_scx_core_kq_local_on_nr_read_once(s32 cpu)
{ return READ_ONCE(cpu_rq(cpu)->scx.local_dsq.nr); }
u32 lupos_scx_core_kq_user_nr_read_once(struct scx_dispatch_q *dsq)
{ return READ_ONCE(dsq->nr); }
bool lupos_scx_core_kq_iter_bad_flags(u64 flags)
{ return flags & ~__SCX_DSQ_ITER_USER_FLAGS; }
void lupos_scx_core_kq_init_cursor(struct bpf_iter_scx_dsq_kern *kit, u64 flags)
{
	/* Native self-address, u32 flag conversion and READ_ONCE(dsq->seq). */
	kit->cursor = INIT_DSQ_LIST_CURSOR(kit->cursor, kit->dsq, flags);
}

struct task_struct *lupos_scx_core_kq_iter_next_guard(struct bpf_iter_scx_dsq_kern *kit)
{
	guard(raw_spinlock_irqsave)(&kit->dsq->lock);
	return lupos_scx_core_kq_iter_next_locked_body(kit);
}
bool lupos_scx_core_kq_cursor_empty(struct bpf_iter_scx_dsq_kern *kit)
{ return list_empty(&kit->cursor.node); }
void lupos_scx_core_kq_cursor_unlink_locked(struct bpf_iter_scx_dsq_kern *kit)
{
	unsigned long flags;

	/* Original explicit IRQ-save pair, distinct from next()'s scoped guard. */
	raw_spin_lock_irqsave(&kit->dsq->lock, flags);
	list_del_init(&kit->cursor.node);
	raw_spin_unlock_irqrestore(&kit->dsq->lock, flags);
}
struct task_struct *lupos_scx_core_kq_first_task_rcu(struct scx_dispatch_q *dsq)
{ return rcu_dereference(dsq->first_task); }
void lupos_scx_core_kq_error_peek_builtin(struct scx_sched *sch, u64 dsq_id)
{ scx_error(sch, "peek disallowed on builtin DSQ 0x%llx", dsq_id); }
void lupos_scx_core_kq_error_peek_missing(struct scx_sched *sch, u64 dsq_id)
{ scx_error(sch, "peek on non-existent DSQ 0x%llx", dsq_id); }
void lupos_scx_core_kq_error_reenq_flags(struct scx_sched *sch, u64 reenq_flags)
{ scx_error(sch, "invalid SCX_REENQ flags 0x%llx", reenq_flags); }
void lupos_scx_core_kq_call_dsq_reenq(u64 dsq_id, u64 reenq_flags,
                                    const struct bpf_prog_aux *aux)
{ scx_bpf_dsq_reenq(dsq_id, reenq_flags, aux); }

bool lupos_scx_core_kq_unlikely_reenqueue_no_sched(struct scx_sched *sch)
{ return unlikely(!sch); }
bool lupos_scx_core_kq_unlikely_bad_node(s32 node)
{ return unlikely(node >= (int)nr_node_ids || (node < 0 && node != NUMA_NO_NODE)); }
bool lupos_scx_core_kq_unlikely_create_builtin(u64 dsq_id)
{ return unlikely(dsq_id & SCX_DSQ_FLAG_BUILTIN); }
bool lupos_scx_core_kq_unlikely_slice_unauthorized(struct scx_sched *sch, struct task_struct *p)
{ return unlikely(!sch || !scx_task_on_sched(sch, p)); }
bool lupos_scx_core_kq_unlikely_slice_missing_base(struct scx_sched *sch, struct rq *locked_rq)
{ return unlikely(scx_missing_caps(sch, cpu_of(locked_rq), SCX_CAP_BASE)); }
bool lupos_scx_core_kq_unlikely_slice_protected(bool rejected)
{ return unlikely(rejected); }
bool lupos_scx_core_kq_unlikely_vtime_unauthorized(struct scx_sched *sch, struct task_struct *p)
{ return unlikely(!sch || !scx_task_on_sched(sch, p)); }
bool lupos_scx_core_kq_likely_kick_sched(struct scx_sched *sch)
{ return likely(sch); }
bool lupos_scx_core_kq_unlikely_kick_cid_no_sched(struct scx_sched *sch)
{ return unlikely(!sch); }
bool lupos_scx_core_kq_unlikely_count_no_sched(struct scx_sched *sch)
{ return unlikely(!sch); }
bool lupos_scx_core_kq_unlikely_iter_no_sched(struct scx_sched *sch)
{ return unlikely(!sch); }
bool lupos_scx_core_kq_unlikely_peek_no_sched(struct scx_sched *sch)
{ return unlikely(!sch); }
bool lupos_scx_core_kq_unlikely_peek_builtin(u64 dsq_id)
{ return unlikely(dsq_id & SCX_DSQ_FLAG_BUILTIN); }
bool lupos_scx_core_kq_unlikely_peek_missing(struct scx_dispatch_q *dsq)
{ return unlikely(!dsq); }
bool lupos_scx_core_kq_unlikely_reenq_no_sched(struct scx_sched *sch)
{ return unlikely(!sch); }
bool lupos_scx_core_kq_unlikely_reenq_flags(u64 reenq_flags)
{ return unlikely(reenq_flags & ~__SCX_REENQ_USER_MASK); }
