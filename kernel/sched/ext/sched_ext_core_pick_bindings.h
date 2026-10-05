/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_PICK_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_PICK_BINDINGS_H
/* F04 and its sole retained inlines_header.rs owner. All types, enum values,
 * per-CPU forms and anonymous unions come from the configured native headers.
 * Include only in the one F00 type universe and future native envelope. */
#include "internal.h"
#include "inlines.h"

/* Native callback adapters call these uniquely named Rust owner entries. */
#ifdef CONFIG_EXT_SUB_SCHED
/* Native enum and pointer identities; recursive FFI stack/CFI remains held. */
enum scx_dsp_verdict
lupos_scx_core_pick_dispatch_sched_body(struct scx_sched *sch, struct rq *rq,
				      struct task_struct *prev, bool nested);
#endif
void lupos_scx_core_pick_set_next_body(struct rq *rq, struct task_struct *p, bool first);
void lupos_scx_core_pick_put_prev_body(struct rq *rq, struct task_struct *p,
                                      struct task_struct *next);
void lupos_scx_core_pick_kick_sync_wait_body(struct rq *rq);
struct task_struct *lupos_scx_core_pick_task_body(struct rq *rq, struct rq_flags *rf);
struct task_struct *lupos_scx_core_pick_server_task_body(struct sched_dl_entity *dl_se,
                                                       struct rq_flags *rf);

s32 lupos_scx_core_pick_cpu_of(struct rq *rq);
struct scx_sched *lupos_scx_core_pick_root_protected_live(void);
s32 lupos_scx_core_pick_processor_id(void);
void lupos_scx_core_pick_assert_dispatch_rq(struct rq *rq);
void lupos_scx_core_pick_process_sync_ecaps(struct rq *rq, struct task_struct *prev);
bool lupos_scx_core_pick_has_cpu_acquire(struct scx_sched *sch);
bool lupos_scx_core_pick_has_cpu_release(struct scx_sched *sch);
void lupos_scx_core_pick_call_cpu_acquire(struct scx_sched *sch, struct rq *rq, s32 cpu);
void lupos_scx_core_pick_call_cpu_release(struct scx_sched *sch, struct rq *rq,
        struct task_struct *next, enum scx_cpu_preempt_reason reason);
bool lupos_scx_core_pick_has_running(struct scx_sched *sch);
bool lupos_scx_core_pick_has_stopping(struct scx_sched *sch);
void lupos_scx_core_pick_call_running(struct scx_sched *sch, struct rq *rq,
                                     struct task_struct *p);
void lupos_scx_core_pick_call_stopping(struct scx_sched *sch, struct rq *rq,
                                      struct task_struct *p);
bool lupos_scx_core_pick_can_stay(struct rq *rq, struct task_struct *p);
void lupos_scx_core_pick_event_keep_last(struct scx_sched *sch);
void lupos_scx_core_pick_schedule_reenq(struct rq *rq);
u64 lupos_scx_core_pick_clock_task(struct rq *rq);
void lupos_scx_core_pick_update_tick_dependency(struct rq *rq);
void lupos_scx_core_pick_update_other_load_avgs(struct rq *rq);
bool lupos_scx_core_pick_nohz_full(struct rq *rq);
void lupos_scx_core_pick_tick_dep_set(struct rq *rq);
const struct sched_class *lupos_scx_core_pick_dl_class(void);
const struct sched_class *lupos_scx_core_pick_rt_class(void);
bool lupos_scx_core_pick_rescue_keep(struct rq *rq, struct task_struct *p);
void lupos_scx_core_pick_warn_enq_last(struct rq *rq, struct scx_sched *sch);

/* Exact release increment; foreign polling keeps acquire and READ_ONCE apart. */
void lupos_scx_core_pick_kick_sync_advance(struct rq *rq);
bool lupos_scx_core_pick_kick_sync_acquire_advanced(s32 cpu,
        const unsigned long *ksyncs);
bool lupos_scx_core_pick_kick_sync_read_once_pending(s32 cpu,
        const unsigned long *ksyncs);
bool lupos_scx_core_pick_next_sync_cpu(struct rq *rq, s32 *cpu);
void lupos_scx_core_pick_clear_sync_cpu(struct rq *rq, s32 cpu);
void lupos_scx_core_pick_unlock_irq(struct rq *rq);
void lupos_scx_core_pick_lock_irq(struct rq *rq);
struct task_struct *lupos_scx_core_pick_first_local(struct rq *rq);
void lupos_scx_core_pick_unpin(struct rq *rq, struct rq_flags *rf);
void lupos_scx_core_pick_repin(struct rq *rq, struct rq_flags *rf);
void lupos_scx_core_pick_queue_kick_sync(struct rq *rq);
void lupos_scx_core_pick_modified_begin(struct rq *rq);
bool lupos_scx_core_pick_modified_above(struct rq *rq);
struct task_struct *lupos_scx_core_pick_retry_task(void);
bool lupos_scx_core_pick_warned_zero_slice(struct scx_sched *sch);
void lupos_scx_core_pick_mark_warned_zero_slice(struct scx_sched *sch);
void lupos_scx_core_pick_print_zero_slice(struct task_struct *p);
void lupos_scx_core_pick_dl_server_init(struct sched_dl_entity *dl_se, struct rq *rq);

/* Independent native hint sites; original DISABLE_BRANCH_PROFILING stays.
 * No branch-only native-to-Rust bridge is introduced. */
bool lupos_scx_core_pick_unlikely_cpu_released(struct rq *rq);
bool lupos_scx_core_pick_unlikely_extra_immed(struct rq *rq);
bool lupos_scx_core_pick_unlikely_rescue_no_slice(struct task_struct *p, struct rq *rq);
bool lupos_scx_core_pick_unlikely_rescue_keep_local(struct task_struct *p, struct rq *rq);
bool lupos_scx_core_pick_unlikely_rescue_enq_flags(struct task_struct *p, struct rq *rq);
bool lupos_scx_core_pick_unlikely_foreign_wait(struct rq *rq);
bool lupos_scx_core_pick_unlikely_sync_pending(struct rq *rq);
bool lupos_scx_core_pick_unlikely_zero_slice(struct task_struct *p);
#ifdef CONFIG_SCHED_CORE
bool lupos_scx_core_pick_unlikely_core_sync_pending(struct rq *rq);
bool lupos_scx_core_pick_unlikely_foreign_balance(struct rq *rq);
bool lupos_scx_core_pick_has_core_sched_before(struct scx_sched *sch);
bool lupos_scx_core_pick_call_core_sched_before(struct scx_sched *sch,
        const struct task_struct *a, const struct task_struct *b);
s32 lupos_scx_core_pick_task_cpu(const struct task_struct *p);
#endif

/* Leaves for the existing scx_dispatch_sched Rust body, never a second body. */
struct scx_dsp_ctx *lupos_scx_core_pick_inline_dsp_ctx(struct scx_sched *sch);
bool lupos_scx_core_pick_inline_task_on_sched(struct scx_sched *sch,
                                            const struct task_struct *prev);
bool lupos_scx_core_pick_inline_bypass_enabled(struct scx_sched *sch);
#ifdef CONFIG_EXT_SUB_SCHED
struct scx_sched_pcpu *lupos_scx_core_pick_inline_pcpu(struct scx_sched *sch, s32 cpu);
void lupos_scx_core_pick_inline_event_sub_bypass(struct scx_sched *sch);
#endif
bool lupos_scx_core_pick_inline_unlikely_no_dispatch(struct scx_sched *sch);
void lupos_scx_core_pick_inline_call_dispatch(struct scx_sched *sch, struct rq *rq,
        s32 cpu, struct task_struct *prev, bool prev_on_sch);
bool lupos_scx_core_pick_inline_unlikely_last_loop(int *nr_loops);

#endif /* LUPOS_SCHED_EXT_CORE_PICK_BINDINGS_H */
