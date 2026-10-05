// SPDX-License-Identifier: GPL-2.0
/* F14 exact kfunc shells plus narrow native primitive/diagnostic leaves.
 * All native code is unqualified C runtime, never Rust algorithm coverage.
 * F00 composes the original single build_policy envelope; F13 alone owns BTF
 * registration and context flags. No old ext.c owner fallback is provided. */
#error "SOURCE ONLY HOLD: sched_ext dispatch kfunc native ABI and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_kfunc_dispatch_bindings.h"

/* Original first scope, ext.c:8791-8957, is wholly F14-owned. Keep diagnostic
 * suppression from the real kfunc macros confined to actual native wrappers;
 * no primitives or invented callbacks are covered by this scope. */
__bpf_kfunc_start_defs();

__bpf_kfunc bool scx_bpf_dsq_insert___v2(struct task_struct *p, u64 dsq_id,
					 u64 slice, u64 enq_flags,
					 const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	return lupos_scx_core_kd_insert_v2_body(p, dsq_id, slice, enq_flags, aux);
}

/* COMPAT: original v6.23 removal note also applies to the ___v2 suffix. */
__bpf_kfunc void scx_bpf_dsq_insert(struct task_struct *p, u64 dsq_id,
				    u64 slice, u64 enq_flags,
				    const struct bpf_prog_aux *aux)
{
	/* The compatibility owner calls the guarded v2 shell; no added guard. */
	lupos_scx_core_kd_insert_body(p, dsq_id, slice, enq_flags, aux);
}

__bpf_kfunc bool
__scx_bpf_dsq_insert_vtime(struct task_struct *p,
			   struct scx_bpf_dsq_insert_vtime_args *args,
			   const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	/* Pass args itself: its fields are read only after live scheduler lookup. */
	return lupos_scx_core_kd_insert_vtime_args_body(p, args, aux);
}

/* COMPAT: original v6.23 removal note. No implicit aux in this legacy ABI. */
__bpf_kfunc void scx_bpf_dsq_insert_vtime(struct task_struct *p, u64 dsq_id,
					  u64 slice, u64 vtime, u64 enq_flags)
{
	guard(rcu)();
	lupos_scx_core_kd_insert_vtime_compat_body(p, dsq_id, slice, vtime, enq_flags);
}

__bpf_kfunc_end_defs();

/* Original second scope, ext.c:9073-9288. F13's two interleaved registration
 * groups are not copied or redefined here. BTF retains these exact names and
 * parameter names, especially implicit aux and iterator it__iter. No original
 * function in F14 has nullable or arena-address-space parameter annotations. */
__bpf_kfunc_start_defs();

__bpf_kfunc u32 scx_bpf_dispatch_nr_slots(const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	return lupos_scx_core_kd_dispatch_nr_slots_body(aux);
}

__bpf_kfunc void scx_bpf_dispatch_cancel(const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	lupos_scx_core_kd_dispatch_cancel_body(aux);
}

__bpf_kfunc bool scx_bpf_dsq_move_to_local___v2(u64 dsq_id, u64 enq_flags,
						const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	return lupos_scx_core_kd_move_to_local_v2_body(dsq_id, enq_flags, aux);
}

/* COMPAT: original ___v2 introduced-in-v7.1 note. */
__bpf_kfunc bool scx_bpf_dsq_move_to_local(u64 dsq_id, const struct bpf_prog_aux *aux)
{
	return lupos_scx_core_kd_move_to_local_body(dsq_id, aux);
}

__bpf_kfunc void scx_bpf_dsq_move_set_slice(struct bpf_iter_scx_dsq *it__iter,
					    u64 slice)
{
	lupos_scx_core_kd_move_set_slice_body(it__iter, slice);
}

__bpf_kfunc void scx_bpf_dsq_move_set_vtime(struct bpf_iter_scx_dsq *it__iter,
					    u64 vtime)
{
	lupos_scx_core_kd_move_set_vtime_body(it__iter, vtime);
}

__bpf_kfunc bool scx_bpf_dsq_move(struct bpf_iter_scx_dsq *it__iter,
				  struct task_struct *p, u64 dsq_id,
				  u64 enq_flags)
{
	return lupos_scx_core_kd_move_body(it__iter, p, dsq_id, enq_flags);
}

__bpf_kfunc bool scx_bpf_dsq_move_vtime(struct bpf_iter_scx_dsq *it__iter,
					struct task_struct *p, u64 dsq_id,
					u64 enq_flags)
{
	return lupos_scx_core_kd_move_vtime_body(it__iter, p, dsq_id, enq_flags);
}

__bpf_kfunc_end_defs();

struct scx_sched *lupos_scx_core_kd_prog_sched(const struct bpf_prog_aux *aux)
{ return scx_prog_sched(aux); }
struct scx_sched *lupos_scx_core_kd_root_vtime_rcu(void)
{
	/* Distinct original C8939 RCU diagnostic site, F00's single root storage. */
	return rcu_dereference(scx_root);
}
struct scx_dsp_ctx *lupos_scx_core_kd_this_dsp_ctx(struct scx_sched *sch)
{ return &this_cpu_ptr(sch->pcpu)->dsp_ctx; }
u32 lupos_scx_core_kd_this_cursor_read(struct scx_sched *sch)
{ return __this_cpu_read(sch->pcpu->dsp_ctx.cursor); }
struct scx_dsp_buf_ent *lupos_scx_core_kd_buf_entry(struct scx_dsp_ctx *dspc, u32 index)
{ return &dspc->buf[index]; }
unsigned long lupos_scx_core_kd_ops_state_read(struct task_struct *p)
{ return atomic_long_read(&p->scx.ops_state); }
struct bpf_iter_scx_dsq_kern *lupos_scx_core_kd_iter_kern(struct bpf_iter_scx_dsq *it__iter)
{ return (struct bpf_iter_scx_dsq_kern *)it__iter; }
struct rq *lupos_scx_core_kd_task_rq(struct task_struct *p) { return task_rq(p); }
struct rq *lupos_scx_core_kd_this_rq(void) { return this_rq(); }
s32 lupos_scx_core_kd_task_cpu(struct task_struct *p) { return task_cpu(p); }
unsigned long lupos_scx_core_kd_irq_save(void)
{
	unsigned long flags;

	local_irq_save(flags);
	return flags;
}
void lupos_scx_core_kd_dsq_lock(struct scx_dispatch_q *dsq)
{ raw_spin_lock(&dsq->lock); }
void lupos_scx_core_kd_dsq_unlock(struct scx_dispatch_q *dsq)
{ raw_spin_unlock(&dsq->lock); }
void lupos_scx_core_kd_rq_unlock_irqrestore(struct rq *rq, unsigned long flags)
{ raw_spin_rq_unlock_irqrestore(rq, flags); }

bool lupos_scx_core_kd_call_insert_v2(struct task_struct *p, u64 dsq_id,
        u64 slice, u64 enq_flags, const struct bpf_prog_aux *aux)
{
	/* This is the new shell above, whose body is Rust; not an old-C fallback. */
	return scx_bpf_dsq_insert___v2(p, dsq_id, slice, enq_flags, aux);
}
bool lupos_scx_core_kd_call_move_to_local_v2(u64 dsq_id, u64 enq_flags,
        const struct bpf_prog_aux *aux)
{
	return scx_bpf_dsq_move_to_local___v2(dsq_id, enq_flags, aux);
}

void lupos_scx_core_kd_assert_insert_irqs_disabled(void)
{ lockdep_assert_irqs_disabled(); }
void lupos_scx_core_kd_error_enq_flags(struct scx_sched *sch, u64 enq_flags)
{ scx_error(sch, "invalid enq_flags 0x%llx", enq_flags); }
void lupos_scx_core_kd_error_immed_nonlocal(struct scx_sched *sch, u64 dsq_id)
{ scx_error(sch, "SCX_ENQ_IMMED on a non-local DSQ 0x%llx", dsq_id); }
void lupos_scx_core_kd_error_rescue_nonlocal(struct scx_sched *sch, u64 dsq_id)
{ scx_error(sch, "SCX_ENQ_RESCUE on a non-local DSQ 0x%llx", dsq_id); }
void lupos_scx_core_kd_error_null_task(struct scx_sched *sch)
{ scx_error(sch, "called with NULL task"); }
void lupos_scx_core_kd_event_insert_not_owned(struct scx_sched *sch)
{ __scx_add_event(sch, SCX_EV_INSERT_NOT_OWNED, 1); }
void lupos_scx_core_kd_error_buffer_overflow(struct scx_sched *sch)
{ scx_error(sch, "dispatch buffer overflow"); }
#ifdef CONFIG_EXT_SUB_SCHED
void lupos_scx_core_kd_error_compat_vtime(struct task_struct *p)
{
	/* Keep scx_task_sched(p), including its protected-RCU lockdep predicate;
	 * replacing it with scx_task_sched_rcu() would change the native contract. */
	scx_error(scx_task_sched(p), "__scx_bpf_dsq_insert_vtime() must be used");
}
#endif
void lupos_scx_core_kd_error_move_not_owned(struct scx_sched *sch, struct task_struct *p)
{
	scx_error(sch, "scx_bpf_dsq_move[_vtime]() on %s[%d] but the task belongs to a different scheduler",
		  p->comm, p->pid);
}
void lupos_scx_core_kd_error_buffer_underflow(struct scx_sched *sch)
{ scx_error(sch, "dispatch buffer underflow"); }
void lupos_scx_core_kd_error_invalid_dsq(struct scx_sched *sch, u64 dsq_id)
{ scx_error(sch, "invalid DSQ ID 0x%016llx", dsq_id); }

/* Retain all 17 distinct original hint sites and exact native predicates.
 * Original build_policy DISABLE_BRANCH_PROFILING remains mandatory. Native
 * helper inlining/attribution and configuration-specific ABI remain held. */
bool lupos_scx_core_kd_unlikely_internal_flags(u64 enq_flags)
{ return unlikely(enq_flags & __SCX_ENQ_INTERNAL_MASK); }
bool lupos_scx_core_kd_unlikely_immed_nonlocal(bool is_local)
{ return unlikely(!is_local); }
bool lupos_scx_core_kd_unlikely_rescue_nonlocal(u64 enq_flags, bool is_local)
{ return unlikely((enq_flags & SCX_ENQ_RESCUE) && !is_local); }
bool lupos_scx_core_kd_unlikely_null_insert_task(struct task_struct *p)
{ return unlikely(!p); }
bool lupos_scx_core_kd_unlikely_insert_not_owned(struct scx_sched *sch, struct task_struct *p)
{ return unlikely(!scx_task_on_sched(sch, p)); }
bool lupos_scx_core_kd_unlikely_buffer_overflow(struct scx_dsp_ctx *dspc, struct scx_sched *sch)
{ return unlikely(dspc->cursor >= sch->dsp_max_batch); }
bool lupos_scx_core_kd_unlikely_no_insert_sched(struct scx_sched *sch)
{ return unlikely(!sch); }
bool lupos_scx_core_kd_unlikely_no_vtime_sched(struct scx_sched *sch)
{ return unlikely(!sch); }
bool lupos_scx_core_kd_unlikely_no_compat_root(struct scx_sched *sch)
{ return unlikely(!sch); }
#ifdef CONFIG_EXT_SUB_SCHED
bool lupos_scx_core_kd_unlikely_compat_children(struct scx_sched *sch)
{ return unlikely(!list_empty(&sch->children)); }
#endif
bool lupos_scx_core_kd_unlikely_no_move_dsq(struct scx_dispatch_q *src_dsq)
{ return unlikely(!src_dsq); }
bool lupos_scx_core_kd_unlikely_move_aborting(struct scx_sched *sch)
{ return unlikely(READ_ONCE(sch->aborting)); }
bool lupos_scx_core_kd_unlikely_move_not_owned(struct scx_sched *sch, struct task_struct *p)
{ return unlikely(!scx_task_on_sched(sch, p)); }
bool lupos_scx_core_kd_unlikely_no_slots_sched(struct scx_sched *sch)
{ return unlikely(!sch); }
bool lupos_scx_core_kd_unlikely_no_cancel_sched(struct scx_sched *sch)
{ return unlikely(!sch); }
bool lupos_scx_core_kd_unlikely_no_local_sched(struct scx_sched *sch)
{ return unlikely(!sch); }
bool lupos_scx_core_kd_unlikely_missing_local_dsq(struct scx_dispatch_q *dsq)
{ return unlikely(!dsq); }
