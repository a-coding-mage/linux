/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_KFUNC_DISPATCH_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_KFUNC_DISPATCH_BINDINGS_H

/* F14 source-only ABI boundary. Include in F00's single canonical native
 * binding/envelope; do not compile an independent C object or generate a
 * second native type universe. F01 owns both iterator layouts and flag enum.
 * The shared-access owner supplies its exact header; F14 defines no accessors. */
#include "internal.h"
#include "sched_ext_core_slice_cursor_bindings.h"
#include "sched_ext_shared_access.h"

/* Exact ext.c:8873-8879 native argument layout. p stays separate because
 * KF_RCU is not transitive. There is no nullable or arena annotation here. */
struct scx_bpf_dsq_insert_vtime_args {
	/* @p can't be packed together as KF_RCU is not transitive */
	u64			dsq_id;
	u64			slice;
	u64			vtime;
	u64			enq_flags;
};

/* These typed body entry points are implemented only by F14 Rust. Native
 * __bpf_kfunc wrappers preserve original public names, parameter spellings,
 * implicit aux roles and scoped guards. No original C owner is a fallback. */
bool lupos_scx_core_kd_insert_v2_body(struct task_struct *p, u64 dsq_id,
        u64 slice, u64 enq_flags, const struct bpf_prog_aux *aux);
void lupos_scx_core_kd_insert_body(struct task_struct *p, u64 dsq_id,
        u64 slice, u64 enq_flags, const struct bpf_prog_aux *aux);
bool lupos_scx_core_kd_insert_vtime_args_body(struct task_struct *p,
        struct scx_bpf_dsq_insert_vtime_args *args, const struct bpf_prog_aux *aux);
void lupos_scx_core_kd_insert_vtime_compat_body(struct task_struct *p, u64 dsq_id,
        u64 slice, u64 vtime, u64 enq_flags);
u32 lupos_scx_core_kd_dispatch_nr_slots_body(const struct bpf_prog_aux *aux);
void lupos_scx_core_kd_dispatch_cancel_body(const struct bpf_prog_aux *aux);
bool lupos_scx_core_kd_move_to_local_v2_body(u64 dsq_id, u64 enq_flags,
        const struct bpf_prog_aux *aux);
bool lupos_scx_core_kd_move_to_local_body(u64 dsq_id, const struct bpf_prog_aux *aux);
void lupos_scx_core_kd_move_set_slice_body(struct bpf_iter_scx_dsq *it__iter, u64 slice);
void lupos_scx_core_kd_move_set_vtime_body(struct bpf_iter_scx_dsq *it__iter, u64 vtime);
bool lupos_scx_core_kd_move_body(struct bpf_iter_scx_dsq *it__iter,
        struct task_struct *p, u64 dsq_id, u64 enq_flags);
bool lupos_scx_core_kd_move_vtime_body(struct bpf_iter_scx_dsq *it__iter,
        struct task_struct *p, u64 dsq_id, u64 enq_flags);

/* Header-owned native helper bodies retain their original configuration and
 * protection rules. These leaves neither acquire ownership nor extend life. */
struct scx_sched *lupos_scx_core_kd_prog_sched(const struct bpf_prog_aux *aux);
struct scx_sched *lupos_scx_core_kd_root_vtime_rcu(void);
struct scx_dsp_ctx *lupos_scx_core_kd_this_dsp_ctx(struct scx_sched *sch);
u32 lupos_scx_core_kd_this_cursor_read(struct scx_sched *sch);
struct scx_dsp_buf_ent *lupos_scx_core_kd_buf_entry(struct scx_dsp_ctx *dspc, u32 index);
unsigned long lupos_scx_core_kd_ops_state_read(struct task_struct *p);
struct bpf_iter_scx_dsq_kern *lupos_scx_core_kd_iter_kern(struct bpf_iter_scx_dsq *it__iter);
struct rq *lupos_scx_core_kd_task_rq(struct task_struct *p);
struct rq *lupos_scx_core_kd_this_rq(void);
s32 lupos_scx_core_kd_task_cpu(struct task_struct *p);
unsigned long lupos_scx_core_kd_irq_save(void);
void lupos_scx_core_kd_dsq_lock(struct scx_dispatch_q *dsq);
void lupos_scx_core_kd_dsq_unlock(struct scx_dispatch_q *dsq);
void lupos_scx_core_kd_rq_unlock_irqrestore(struct rq *rq, unsigned long flags);

/* Compatibility owners call these exact new native v2 wrappers so that the
 * original callee, rather than the compatibility entry, acquires guard(rcu). */
bool lupos_scx_core_kd_call_insert_v2(struct task_struct *p, u64 dsq_id,
        u64 slice, u64 enq_flags, const struct bpf_prog_aux *aux);
bool lupos_scx_core_kd_call_move_to_local_v2(u64 dsq_id, u64 enq_flags,
        const struct bpf_prog_aux *aux);

/* Original lockdep, scheduler-error and event sites stay distinct native
 * expansions. Diagnostic formatting/line attribution are not qualified. */
void lupos_scx_core_kd_assert_insert_irqs_disabled(void);
void lupos_scx_core_kd_error_enq_flags(struct scx_sched *sch, u64 enq_flags);
void lupos_scx_core_kd_error_immed_nonlocal(struct scx_sched *sch, u64 dsq_id);
void lupos_scx_core_kd_error_rescue_nonlocal(struct scx_sched *sch, u64 dsq_id);
void lupos_scx_core_kd_error_null_task(struct scx_sched *sch);
void lupos_scx_core_kd_event_insert_not_owned(struct scx_sched *sch);
void lupos_scx_core_kd_error_buffer_overflow(struct scx_sched *sch);
#ifdef CONFIG_EXT_SUB_SCHED
void lupos_scx_core_kd_error_compat_vtime(struct task_struct *p);
#endif
void lupos_scx_core_kd_error_move_not_owned(struct scx_sched *sch, struct task_struct *p);
void lupos_scx_core_kd_error_buffer_underflow(struct scx_sched *sch);
void lupos_scx_core_kd_error_invalid_dsq(struct scx_sched *sch, u64 dsq_id);

/* One native unlikely site per original source hint. The original
 * CFLAGS_build_policy.o += -DDISABLE_BRANCH_PROFILING policy stays mandatory;
 * these declarations do not authorize independent instrumentation or builds. */
bool lupos_scx_core_kd_unlikely_internal_flags(u64 enq_flags);
bool lupos_scx_core_kd_unlikely_immed_nonlocal(bool is_local);
bool lupos_scx_core_kd_unlikely_rescue_nonlocal(u64 enq_flags, bool is_local);
bool lupos_scx_core_kd_unlikely_null_insert_task(struct task_struct *p);
bool lupos_scx_core_kd_unlikely_insert_not_owned(struct scx_sched *sch, struct task_struct *p);
bool lupos_scx_core_kd_unlikely_buffer_overflow(struct scx_dsp_ctx *dspc, struct scx_sched *sch);
bool lupos_scx_core_kd_unlikely_no_insert_sched(struct scx_sched *sch);
bool lupos_scx_core_kd_unlikely_no_vtime_sched(struct scx_sched *sch);
bool lupos_scx_core_kd_unlikely_no_compat_root(struct scx_sched *sch);
#ifdef CONFIG_EXT_SUB_SCHED
bool lupos_scx_core_kd_unlikely_compat_children(struct scx_sched *sch);
#endif
bool lupos_scx_core_kd_unlikely_no_move_dsq(struct scx_dispatch_q *src_dsq);
bool lupos_scx_core_kd_unlikely_move_aborting(struct scx_sched *sch);
bool lupos_scx_core_kd_unlikely_move_not_owned(struct scx_sched *sch, struct task_struct *p);
bool lupos_scx_core_kd_unlikely_no_slots_sched(struct scx_sched *sch);
bool lupos_scx_core_kd_unlikely_no_cancel_sched(struct scx_sched *sch);
bool lupos_scx_core_kd_unlikely_no_local_sched(struct scx_sched *sch);
bool lupos_scx_core_kd_unlikely_missing_local_dsq(struct scx_dispatch_q *dsq);

#endif /* LUPOS_SCHED_EXT_CORE_KFUNC_DISPATCH_BINDINGS_H */
