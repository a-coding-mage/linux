/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_KFUNC_DSQ_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_KFUNC_DSQ_BINDINGS_H

/* F15 declarations join F00's single generated native type universe. F01
 * alone supplies the exact ext.c-private iterator types and flag enum. */
#include "internal.h"
#include "sched_ext_core_slice_cursor_bindings.h"
#include "sched_ext_shared_access.h"

/* Typed Rust owner bodies, reached only through exact native kfunc shells. */
u32 lupos_scx_core_kq_reenqueue_local_body(const struct bpf_prog_aux *aux);
s32 lupos_scx_core_kq_create_dsq_body(u64 dsq_id, s32 node,
                                    const struct bpf_prog_aux *aux);
s32 lupos_scx_core_kq_create_rcu_body(struct scx_dispatch_q *dsq,
                                    const struct bpf_prog_aux *aux);
bool lupos_scx_core_kq_task_set_slice_body(struct task_struct *p, u64 slice,
                                         const struct bpf_prog_aux *aux);
bool lupos_scx_core_kq_task_set_dsq_vtime_body(struct task_struct *p, u64 vtime,
                                             const struct bpf_prog_aux *aux);
void lupos_scx_core_kq_kick_cpu_body(s32 cpu, u64 flags, const struct bpf_prog_aux *aux);
void lupos_scx_core_kq_kick_cid_body(s32 cid, u64 flags, const struct bpf_prog_aux *aux);
s32 lupos_scx_core_kq_dsq_nr_queued_body(u64 dsq_id, const struct bpf_prog_aux *aux);
void lupos_scx_core_kq_destroy_dsq_body(u64 dsq_id, const struct bpf_prog_aux *aux);
int lupos_scx_core_kq_iter_new_body(struct bpf_iter_scx_dsq_kern *kit, u64 dsq_id,
                                  u64 flags, const struct bpf_prog_aux *aux);
struct task_struct *lupos_scx_core_kq_iter_next_body(struct bpf_iter_scx_dsq_kern *kit);
struct task_struct *lupos_scx_core_kq_iter_next_locked_body(struct bpf_iter_scx_dsq_kern *kit);
void lupos_scx_core_kq_iter_destroy_body(struct bpf_iter_scx_dsq_kern *kit);
struct task_struct *lupos_scx_core_kq_dsq_peek_body(u64 dsq_id,
                                                 const struct bpf_prog_aux *aux);
void lupos_scx_core_kq_dsq_reenq_body(u64 dsq_id, u64 reenq_flags,
                                    const struct bpf_prog_aux *aux,
                                    struct rq *locked_rq);
void lupos_scx_core_kq_reenqueue_local_v2_body(const struct bpf_prog_aux *aux);

/* Borrowed native operations: none implicitly owns a task or DSQ reference. */
struct scx_sched *lupos_scx_core_kq_prog_sched(const struct bpf_prog_aux *aux);
int lupos_scx_core_kq_processor_id(void);
struct rq *lupos_scx_core_kq_cpu_rq(s32 cpu);
struct rq *lupos_scx_core_kq_this_rq(void);
int lupos_scx_core_kq_cpu_of(struct rq *rq);
void lupos_scx_core_kq_assert_reenqueue_rq(struct rq *rq);
struct scx_dispatch_q *lupos_scx_core_kq_alloc_dsq(s32 node);
void lupos_scx_core_kq_free_dsq(struct scx_dispatch_q *dsq);
s32 lupos_scx_core_kq_create_rcu(struct scx_dispatch_q *dsq,
                               const struct bpf_prog_aux *aux);
s32 lupos_scx_core_kq_hash_insert(struct scx_sched *sch, struct scx_dispatch_q *dsq);
s32 lupos_scx_core_kq_runnable_cpu_read_once(struct task_struct *p);
bool lupos_scx_core_kq_task_current(struct rq *rq, struct task_struct *p);
void lupos_scx_core_kq_event_slice_denied(struct scx_sched *sch);
s32 lupos_scx_core_kq_cid_to_cpu(struct scx_sched *sch, s32 cid);
s32 lupos_scx_core_kq_cpu_ret(struct scx_sched *sch, s32 cpu_or_cid);
u32 lupos_scx_core_kq_local_nr_read_once(struct rq *rq);
u32 lupos_scx_core_kq_local_on_nr_read_once(s32 cpu);
u32 lupos_scx_core_kq_user_nr_read_once(struct scx_dispatch_q *dsq);
bool lupos_scx_core_kq_iter_bad_flags(u64 flags);
void lupos_scx_core_kq_init_cursor(struct bpf_iter_scx_dsq_kern *kit, u64 flags);
struct task_struct *lupos_scx_core_kq_iter_next_guard(struct bpf_iter_scx_dsq_kern *kit);
bool lupos_scx_core_kq_cursor_empty(struct bpf_iter_scx_dsq_kern *kit);
void lupos_scx_core_kq_cursor_unlink_locked(struct bpf_iter_scx_dsq_kern *kit);
struct task_struct *lupos_scx_core_kq_first_task_rcu(struct scx_dispatch_q *dsq);
void lupos_scx_core_kq_error_peek_builtin(struct scx_sched *sch, u64 dsq_id);
void lupos_scx_core_kq_error_peek_missing(struct scx_sched *sch, u64 dsq_id);
void lupos_scx_core_kq_error_reenq_flags(struct scx_sched *sch, u64 reenq_flags);
void lupos_scx_core_kq_call_dsq_reenq(u64 dsq_id, u64 reenq_flags,
                                    const struct bpf_prog_aux *aux);

/* Distinct original branch sites; build_policy's original branch-profiling
 * policy remains authoritative, without any new warning/profiling waiver. */
bool lupos_scx_core_kq_unlikely_reenqueue_no_sched(struct scx_sched *sch);
bool lupos_scx_core_kq_unlikely_bad_node(s32 node);
bool lupos_scx_core_kq_unlikely_create_builtin(u64 dsq_id);
bool lupos_scx_core_kq_unlikely_slice_unauthorized(struct scx_sched *sch, struct task_struct *p);
bool lupos_scx_core_kq_unlikely_slice_missing_base(struct scx_sched *sch, struct rq *locked_rq);
bool lupos_scx_core_kq_unlikely_slice_protected(bool rejected);
bool lupos_scx_core_kq_unlikely_vtime_unauthorized(struct scx_sched *sch, struct task_struct *p);
bool lupos_scx_core_kq_likely_kick_sched(struct scx_sched *sch);
bool lupos_scx_core_kq_unlikely_kick_cid_no_sched(struct scx_sched *sch);
bool lupos_scx_core_kq_unlikely_count_no_sched(struct scx_sched *sch);
bool lupos_scx_core_kq_unlikely_iter_no_sched(struct scx_sched *sch);
bool lupos_scx_core_kq_unlikely_peek_no_sched(struct scx_sched *sch);
bool lupos_scx_core_kq_unlikely_peek_builtin(u64 dsq_id);
bool lupos_scx_core_kq_unlikely_peek_missing(struct scx_dispatch_q *dsq);
bool lupos_scx_core_kq_unlikely_reenq_no_sched(struct scx_sched *sch);
bool lupos_scx_core_kq_unlikely_reenq_flags(u64 reenq_flags);

#endif /* LUPOS_SCHED_EXT_CORE_KFUNC_DSQ_BINDINGS_H */
