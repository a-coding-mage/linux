/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_PLACEMENT_TICK_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_PLACEMENT_TICK_BINDINGS_H
/* F05 declarations for F00's one canonical native binding universe. Native
 * configured headers remain the only layout/configuration authority. */
#include "internal.h"

int lupos_scx_core_pt_select_task_rq_body(struct task_struct *p, int prev_cpu,
                                       int wake_flags);
void lupos_scx_core_pt_task_woken_body(struct rq *rq, struct task_struct *p);
void lupos_scx_core_pt_set_cpus_allowed_body(struct task_struct *p,
                                          struct affinity_context *ac);
void lupos_scx_core_pt_rq_online_body(struct rq *rq);
void lupos_scx_core_pt_rq_offline_body(struct rq *rq);
bool lupos_scx_core_pt_check_timeouts_locked_body(struct rq *rq);
void lupos_scx_core_pt_watchdog_work_body(struct work_struct *work);
void lupos_scx_core_pt_task_tick_body(struct rq *rq, struct task_struct *curr,
                                   int queued);
unsigned long lupos_scx_core_pt_refresh_interval_rcu_body(void);

struct scx_sched *lupos_scx_core_pt_task_sched(struct task_struct *p);
s32 lupos_scx_core_pt_task_cpu(struct task_struct *p);
s32 lupos_scx_core_pt_cpu_of(struct rq *rq);
struct rq *lupos_scx_core_pt_this_rq(void);
bool lupos_scx_core_pt_unlikely_exec(int wake_flags);
bool lupos_scx_core_pt_likely_has_select(struct scx_sched *sch);
void lupos_scx_core_pt_warn_direct_dispatch(struct task_struct **ddsp_taskp);
s32 lupos_scx_core_pt_call_select(struct scx_sched *sch, struct task_struct *p,
                                int prev_cpu, int wake_flags);
s32 lupos_scx_core_pt_cpu_ret(struct scx_sched *sch, s32 cpu);
void lupos_scx_core_pt_event_bypass(struct scx_sched *sch);
void lupos_scx_core_pt_event_refill(struct scx_sched *sch);
s32 lupos_scx_core_pt_select_default(struct task_struct *p, int prev_cpu,
                                   int wake_flags);
u64 lupos_scx_core_pt_slice_dfl_read_once(struct scx_sched *sch);
void lupos_scx_core_pt_set_allowed_common(struct task_struct *p,
                                        struct affinity_context *ac);
bool lupos_scx_core_pt_has_set_cpumask(struct scx_sched *sch);
struct rq *lupos_scx_core_pt_task_rq(struct task_struct *p);
struct scx_sched *lupos_scx_core_pt_root_protected(void);
bool lupos_scx_core_pt_enabled(void);
void lupos_scx_core_pt_update_topology(struct scx_sched *sch);
void lupos_scx_core_pt_online_ecaps(struct rq *rq);
void lupos_scx_core_pt_offline_ecaps(struct rq *rq);
s16 *lupos_scx_core_pt_hotplug_cid_table(void);
bool lupos_scx_core_pt_has_cpu_online(struct scx_sched *sch);
bool lupos_scx_core_pt_has_cpu_offline(struct scx_sched *sch);
void lupos_scx_core_pt_call_cpu_online(struct scx_sched *sch, s32 cpu_or_cid);
void lupos_scx_core_pt_call_cpu_offline(struct scx_sched *sch, s32 cpu_or_cid);
void lupos_scx_core_pt_exit_hotplug(struct scx_sched *sch, s32 cpu, bool online);
void lupos_scx_core_pt_rescue_flush(struct rq *rq);
bool lupos_scx_core_pt_check_timeouts_locked(struct rq *rq);
struct scx_sched *lupos_scx_core_pt_timeout_root_bh(void);
struct task_struct *lupos_scx_core_pt_runnable_task(struct list_head *node);
bool lupos_scx_core_pt_unlikely_task_timeout(struct scx_sched *sch,
                                           unsigned long last_runnable);
struct scx_dispatch_q *lupos_scx_core_pt_task_dsq_read_once(struct task_struct *p);
u32 lupos_scx_core_pt_duration_ms(unsigned long last);
void lupos_scx_core_pt_exit_task_stall(struct scx_sched *sch, struct rq *rq,
                                     struct task_struct *p, u32 dur_ms);
unsigned long lupos_scx_core_pt_jiffies(void);
bool lupos_scx_core_pt_online_cpu_condition(int *cpu);
struct rq *lupos_scx_core_pt_cpu_rq(int cpu);
void lupos_scx_core_pt_cond_resched(void);
unsigned long lupos_scx_core_pt_ulong_max(void);
void lupos_scx_core_pt_queue_watchdog(struct work_struct *work, unsigned long intv);
struct scx_sched *lupos_scx_core_pt_tick_root_bh(void);
bool lupos_scx_core_pt_unlikely_watchdog_timeout(struct scx_sched *root,
                                               unsigned long last_check);
void lupos_scx_core_pt_exit_watchdog_stall(struct scx_sched *root, u32 dur_ms);
void lupos_scx_core_pt_update_other_load_avgs(struct rq *rq);
bool lupos_scx_core_pt_has_tick(struct scx_sched *sch);
void lupos_scx_core_pt_call_tick(struct scx_sched *sch, struct rq *rq,
                               struct task_struct *curr);
void lupos_scx_core_pt_resched_curr(struct rq *rq);
#ifdef CONFIG_NO_HZ_FULL
struct task_struct *lupos_scx_core_pt_rq_curr_plain(struct rq *rq);
bool lupos_scx_core_pt_is_ext_task(struct task_struct *p);
bool lupos_scx_core_pt_unlikely_rescuee(struct task_struct *p, struct rq *rq);
#endif
unsigned long lupos_scx_core_pt_refresh_interval_rcu(void);
struct scx_sched *lupos_scx_core_pt_sched_first_rcu(struct list_head *head);
bool lupos_scx_core_pt_sched_is_head(struct scx_sched *sch, struct list_head *head);
struct scx_sched *lupos_scx_core_pt_sched_next_rcu(struct scx_sched *sch);
void lupos_scx_core_pt_mod_watchdog(unsigned long intv);
void lupos_scx_core_pt_cancel_watchdog(void);

#endif /* LUPOS_SCHED_EXT_CORE_PLACEMENT_TICK_BINDINGS_H */
