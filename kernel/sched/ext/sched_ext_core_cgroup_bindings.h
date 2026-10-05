/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_CGROUP_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_CGROUP_BINDINGS_H

/* F08 declarations for the one canonical native type projection. Native
 * configured headers provide every structure, enum, lock and scalar ABI.
 * Include in the future single envelope; not a separately linked C owner.
 */
#include "internal.h"

#ifdef CONFIG_EXT_GROUP_SCHED
#define LUPOS_SCX_CORE_CGROUP_WEIGHT_DFL CGROUP_WEIGHT_DFL
#define LUPOS_SCX_CORE_CGROUP_RUNTIME_INF RUNTIME_INF

u64 lupos_scx_core_cgroup_default_bw_period_us(void);
struct task_group *lupos_scx_core_cgroup_root_task_group(void);
struct task_group *lupos_scx_core_cgroup_css_tg(struct cgroup_subsys_state *css);
void lupos_scx_core_cgroup_assert_tg_sched(void);
void lupos_scx_core_cgroup_assert_tg_knob_sched(void);
void lupos_scx_core_cgroup_warn_online(struct task_group *tg);
void lupos_scx_core_cgroup_warn_offline(struct task_group *tg);
void lupos_scx_core_cgroup_warn_moving_from(struct task_struct *p);
bool lupos_scx_core_cgroup_on_dfl(struct cgroup *cgrp);
struct scx_sched *lupos_scx_core_cgroup_online_sched(struct cgroup *cgrp);
struct scx_sched *lupos_scx_core_cgroup_task_sched_protected(struct task_struct *p);
struct scx_sched *lupos_scx_core_cgroup_migration_dst_sched(struct task_struct *p);
struct task_struct *lupos_scx_core_cgroup_taskset_first(
        struct cgroup_taskset *tset, struct cgroup_subsys_state **css);
struct task_struct *lupos_scx_core_cgroup_taskset_next(
        struct cgroup_taskset *tset, struct cgroup_subsys_state **css);
struct cgroup_subsys_state *lupos_scx_core_cgroup_next_pre(struct cgroup_subsys_state *pos);
struct cgroup_subsys_state *lupos_scx_core_cgroup_next_post(struct cgroup_subsys_state *pos);

/* Separate source sites retain their native warning/lockdep/callback domains. */
bool lupos_scx_core_cgroup_has_online_init(struct scx_sched *sch);
bool lupos_scx_core_cgroup_has_offline_exit(struct scx_sched *sch);
bool lupos_scx_core_cgroup_has_prep_move(struct scx_sched *sch);
bool lupos_scx_core_cgroup_has_prepare_cancel(struct scx_sched *sch);
bool lupos_scx_core_cgroup_has_move(struct scx_sched *sch);
bool lupos_scx_core_cgroup_has_attach_cancel(struct scx_sched *sch);
bool lupos_scx_core_cgroup_has_set_weight(struct scx_sched *sch);
bool lupos_scx_core_cgroup_has_set_idle(struct scx_sched *sch);
bool lupos_scx_core_cgroup_has_set_bandwidth(struct scx_sched *sch);
/* Root init/exit use actual ops pointer presence, not SCX_HAS_OP. */
bool lupos_scx_core_cgroup_root_init_present(struct scx_sched *sch);
bool lupos_scx_core_cgroup_root_exit_present(struct scx_sched *sch);

/* Rust adapter, not a native definition or second tg_cgrp algorithm owner.
 * Called synchronously from original callback argument evaluation sites.
 */
struct cgroup *lupos_scx_core_cgroup_tg_cgrp(struct task_group *tg);

int lupos_scx_core_cgroup_call_online_init(struct scx_sched *sch, struct task_group *tg,
                                           struct scx_cgroup_init_args *args);
int lupos_scx_core_cgroup_sanitize_online_err(struct scx_sched *sch, s32 err);
void lupos_scx_core_cgroup_call_offline_exit(struct scx_sched *sch, struct task_group *tg);
int lupos_scx_core_cgroup_call_prep_move(struct scx_sched *sch, struct task_struct *p,
                                         struct cgroup *from, struct cgroup_subsys_state *css);
int lupos_scx_core_cgroup_sanitize_prep_err(struct scx_sched *sch, s32 err);
void lupos_scx_core_cgroup_call_prepare_cancel(struct scx_sched *sch, struct task_struct *p,
                                               struct cgroup_subsys_state *css);
void lupos_scx_core_cgroup_call_move(struct scx_sched *sch, struct task_struct *p);
void lupos_scx_core_cgroup_call_attach_cancel(struct scx_sched *sch, struct task_struct *p,
                                              struct cgroup_subsys_state *css);
void lupos_scx_core_cgroup_call_set_weight(struct scx_sched *sch, struct task_group *tg,
                                           unsigned long weight);
void lupos_scx_core_cgroup_call_set_idle(struct scx_sched *sch, struct task_group *tg, bool idle);
void lupos_scx_core_cgroup_call_set_bandwidth(struct scx_sched *sch, struct task_group *tg,
                                              u64 period_us, u64 quota_us, u64 burst_us);
void lupos_scx_core_cgroup_call_root_exit(struct scx_sched *sch, struct cgroup_subsys_state *css);
int lupos_scx_core_cgroup_call_root_init(struct scx_sched *sch, struct cgroup_subsys_state *css,
                                         struct scx_cgroup_init_args *args);
void lupos_scx_core_cgroup_error_root_init(struct scx_sched *sch, int ret);

void lupos_scx_core_cgroup_weight_down_read(void);
void lupos_scx_core_cgroup_weight_up_read(void);
void lupos_scx_core_cgroup_idle_down_read(void);
void lupos_scx_core_cgroup_idle_up_read(void);
void lupos_scx_core_cgroup_bandwidth_down_read(void);
void lupos_scx_core_cgroup_bandwidth_up_read(void);
void lupos_scx_core_cgroup_ops_down_write(void);
void lupos_scx_core_cgroup_ops_up_write(void);
#endif /* CONFIG_EXT_GROUP_SCHED */

#if defined(CONFIG_EXT_GROUP_SCHED) || defined(CONFIG_EXT_SUB_SCHED)
struct cgroup *lupos_scx_core_cgroup_default_root(void);
void lupos_scx_core_cgroup_mutex_lock(void);
void lupos_scx_core_cgroup_mutex_unlock(void);
#endif

#endif /* LUPOS_SCHED_EXT_CORE_CGROUP_BINDINGS_H */
