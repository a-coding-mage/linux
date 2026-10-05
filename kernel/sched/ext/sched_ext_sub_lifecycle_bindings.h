/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_SUB_LIFECYCLE_BINDINGS_H
#define LUPOS_SCHED_EXT_SUB_LIFECYCLE_BINDINGS_H
/*
 * Deferred configured-header binding input. The native types, enum values,
 * callback machinery and lock classes are authoritative, never Rust replicas.
 * Leaves below require the same pointer lifetimes and locks as their original
 * sub.c callsites. They do not acquire scheduler/cgroup ownership implicitly.
 *
 * All passed objects must be live and native-layout. task_lock initializes
 * the caller-owned rf and returns with p's pi/rq locks and IRQ state held;
 * task_unlock consumes that exact rq/p/rf combination. change_begin/end must
 * be paired under those task locks without dropping them. get_task requires
 * an already pinned task and each extra reference is consumed by put_task.
 * sched_lock/unlock use raw_spin_lock_irq/unlock_irq, not irqsave: entry must
 * have IRQs enabled. enable_lock and fork_down require sleepable context.
 * Cgroup iteration/transfer requires the enable mutex, fork writer, and
 * scx_cgroup_lock interval; notifier ownership accesses use cgroup_mutex.
 * Callback args are synchronous stack borrows; all referenced schedulers and
 * cgroups remain pinned until return, and the selected op must be present.
 */
#include "internal.h"
#include "sub.h"
#include "inlines.h"

#ifdef CONFIG_EXT_SUB_SCHED
#define LUPOS_SCX_SUB_LIFECYCLE_EBUSY EBUSY
#define LUPOS_SCX_SUB_LIFECYCLE_EINVAL EINVAL
#define LUPOS_SCX_SUB_LIFECYCLE_ENODEV ENODEV
#define LUPOS_SCX_SUB_LIFECYCLE_EOPNOTSUPP EOPNOTSUPP
#define LUPOS_SCX_SUB_LIFECYCLE_TASK_INIT_BEGIN SCX_TASK_INIT_BEGIN
#define LUPOS_SCX_SUB_LIFECYCLE_TASK_INIT SCX_TASK_INIT
#define LUPOS_SCX_SUB_LIFECYCLE_TASK_READY SCX_TASK_READY
#define LUPOS_SCX_SUB_LIFECYCLE_TASK_ENABLED SCX_TASK_ENABLED
#define LUPOS_SCX_SUB_LIFECYCLE_TASK_DEAD SCX_TASK_DEAD
#define LUPOS_SCX_SUB_LIFECYCLE_TASK_SUB_INIT SCX_TASK_SUB_INIT
#define LUPOS_SCX_SUB_LIFECYCLE_MAX_DEPTH SCX_SUB_MAX_DEPTH
#define LUPOS_SCX_SUB_LIFECYCLE_EFLAG_INITIALIZED SCX_EFLAG_INITIALIZED
#define LUPOS_SCX_SUB_LIFECYCLE_OPI_BEGIN SCX_OPI_BEGIN
#define LUPOS_SCX_SUB_LIFECYCLE_OPI_END SCX_OPI_END
#define LUPOS_SCX_SUB_LIFECYCLE_NOTIFY_OK NOTIFY_OK
#define LUPOS_SCX_SUB_LIFECYCLE_CGROUP_ONLINE CGROUP_LIFETIME_ONLINE
#define LUPOS_SCX_SUB_LIFECYCLE_CGROUP_OFFLINE CGROUP_LIFETIME_OFFLINE
#define LUPOS_SCX_SUB_LIFECYCLE_TASK_MIGRATING CGROUP_TASK_MIGRATING
#define LUPOS_SCX_SUB_LIFECYCLE_TASK_MIGRATED CGROUP_TASK_MIGRATED
#define LUPOS_SCX_SUB_LIFECYCLE_TASK_MIGRATE_CANCELED CGROUP_TASK_MIGRATE_CANCELED

/* Distinct native diagnostic sites for the two original assertion pairs. */
void lupos_scx_sub_lifecycle_assert_rehome_task_locked(struct task_struct *p);
void lupos_scx_sub_lifecycle_assert_punt_task_locked(struct task_struct *p);
void lupos_scx_sub_lifecycle_warn_not_bypassed(struct scx_sched *sch);
struct sched_change_ctx *lupos_scx_sub_lifecycle_change_begin(struct task_struct *p);
void lupos_scx_sub_lifecycle_change_end(struct sched_change_ctx *ctx);
struct scx_sched *lupos_scx_sub_lifecycle_task_sched(struct task_struct *p);
bool lupos_scx_sub_lifecycle_task_on_sched(struct scx_sched *sch, struct task_struct *p);
bool lupos_scx_sub_lifecycle_task_is_ext(struct task_struct *p);
void lupos_scx_sub_lifecycle_fail_parent(struct scx_sched *parent, struct task_struct *p, s32 ret);
void lupos_scx_sub_lifecycle_wait_children(struct scx_sched *sch);
void lupos_scx_sub_lifecycle_wake_unlink(void);
void lupos_scx_sub_lifecycle_synchronize_expedited(void);
void lupos_scx_sub_lifecycle_enable_lock(void);
void lupos_scx_sub_lifecycle_enable_unlock(void);
void lupos_scx_sub_lifecycle_fork_down(void);
void lupos_scx_sub_lifecycle_fork_up(void);
void lupos_scx_sub_lifecycle_sched_lock(void);
void lupos_scx_sub_lifecycle_sched_unlock(void);
void lupos_scx_sub_lifecycle_assert_sched_locked(void);
void lupos_scx_sub_lifecycle_warn_wrong_sched(struct scx_sched *sch, struct task_struct *p);
void lupos_scx_sub_lifecycle_get_task(struct task_struct *p);
void lupos_scx_sub_lifecycle_put_task(struct task_struct *p);
struct rq *lupos_scx_sub_lifecycle_task_lock(struct task_struct *p, struct rq_flags *rf);
void lupos_scx_sub_lifecycle_task_unlock(struct rq *rq, struct task_struct *p, struct rq_flags *rf);
bool lupos_scx_sub_lifecycle_has_sub_detach(struct scx_sched *sch);
void lupos_scx_sub_lifecycle_call_sub_detach(struct scx_sched *parent, struct scx_sub_detach_args *args);
bool lupos_scx_sub_lifecycle_has_exit(struct scx_sched *sch);
void lupos_scx_sub_lifecycle_call_exit(struct scx_sched *sch);
void lupos_scx_sub_lifecycle_mark_dead(struct scx_sched *sch);
bool lupos_scx_sub_lifecycle_in_sysfs(struct scx_sched *sch);
struct scx_sched *lupos_scx_sub_lifecycle_cgroup_sched(struct cgroup *cgrp);
bool lupos_scx_sub_lifecycle_cgroup_descendant(struct cgroup *cgrp, struct cgroup *ancestor);
bool lupos_scx_sub_lifecycle_has_sub_attach(struct scx_sched *sch);
struct scx_sched *lupos_scx_sub_lifecycle_next_child(struct scx_sched *parent, struct scx_sched *pos);
void *lupos_scx_sub_lifecycle_err_ptr(s32 err);
bool lupos_scx_sub_lifecycle_is_err(const void *p);
s32 lupos_scx_sub_lifecycle_ptr_err(const void *p);
void lupos_scx_sub_lifecycle_warn_task_state(struct task_struct *p, u32 state);
struct scx_enable_cmd *lupos_scx_sub_lifecycle_enable_cmd(struct kthread_work *work);
struct sched_ext_ops *lupos_scx_sub_lifecycle_cmd_ops(struct scx_enable_cmd *cmd);
bool lupos_scx_sub_lifecycle_enabled(void);
bool lupos_scx_sub_lifecycle_ops_published(struct sched_ext_ops *ops);
void lupos_scx_sub_lifecycle_has_subs_inc(void);
void lupos_scx_sub_lifecycle_has_subs_dec(void);
void lupos_scx_sub_lifecycle_max_depth_error(struct scx_sched *sch);
bool lupos_scx_sub_lifecycle_has_init(struct scx_sched *sch);
s32 lupos_scx_sub_lifecycle_call_init(struct scx_sched *sch);
s32 lupos_scx_sub_lifecycle_sanitize_init(struct scx_sched *sch, s32 ret);
void lupos_scx_sub_lifecycle_init_error(struct scx_sched *sch, s32 ret);
s32 lupos_scx_sub_lifecycle_call_sub_attach(struct scx_sched *parent, struct scx_sub_attach_args *args);
s32 lupos_scx_sub_lifecycle_sanitize_attach(struct scx_sched *sch, s32 ret);
void lupos_scx_sub_lifecycle_attach_error(struct scx_sched *sch, s32 ret);
bool lupos_scx_sub_lifecycle_op_present(struct sched_ext_ops *ops, s32 index);
void lupos_scx_sub_lifecycle_set_has_op(struct scx_sched *sch, s32 index);
void lupos_scx_sub_lifecycle_cgroup_offline_error(struct scx_sched *sch);
void lupos_scx_sub_lifecycle_warn_enabling(void);
void lupos_scx_sub_lifecycle_set_enabling(struct scx_sched *sch);
void lupos_scx_sub_lifecycle_log_enable(struct scx_sched *sch);
void lupos_scx_sub_lifecycle_uevent_add(struct scx_sched *sch);
void lupos_scx_sub_lifecycle_enable_error(struct scx_sched *sch, s32 ret);
bool lupos_scx_sub_lifecycle_cgroup_online(struct cgroup *cgrp);
void lupos_scx_sub_lifecycle_cgroup_put(struct cgroup *cgrp);
bool lupos_scx_sub_lifecycle_cgroup_enabled(void);
s32 lupos_scx_sub_lifecycle_notifier_errno(s32 err);
struct cgroup *lupos_scx_sub_lifecycle_cgroup_parent(struct cgroup *cgrp);
bool lupos_scx_sub_lifecycle_cgroup_on_dfl(struct cgroup *cgrp);
void lupos_scx_sub_lifecycle_publish_cgroup(struct cgroup *cgrp, struct scx_sched *sch);
void lupos_scx_sub_lifecycle_exit_offline(struct scx_sched *sch, struct cgroup *cgrp);
s32 lupos_scx_sub_lifecycle_register_lifetime(void);
s32 lupos_scx_sub_lifecycle_register_task(void);

/* Rust implementations reached only through the typed native metadata below. */
s32 scx_cgroup_lifetime_notify(struct notifier_block *nb, unsigned long action, void *data);
s32 scx_cgroup_task_notify(struct notifier_block *nb, unsigned long action, void *data);
s32 scx_cgroup_notifier_init(void);

#ifdef CONFIG_EXT_GROUP_SCHED
#define LUPOS_SCX_SUB_LIFECYCLE_TG_INITED SCX_TG_INITED
#define LUPOS_SCX_SUB_LIFECYCLE_TG_SUB_INIT SCX_TG_SUB_INIT
struct cgroup_subsys_state *lupos_scx_sub_lifecycle_cpu_ecss(struct cgroup *cgrp);
struct cgroup_subsys_state *lupos_scx_sub_lifecycle_css_pre(struct cgroup_subsys_state *pos, struct cgroup_subsys_state *root);
struct cgroup_subsys_state *lupos_scx_sub_lifecycle_css_post(struct cgroup_subsys_state *pos, struct cgroup_subsys_state *root);
struct task_group *lupos_scx_sub_lifecycle_css_tg(struct cgroup_subsys_state *css);
bool lupos_scx_sub_lifecycle_has_cgroup_init(struct scx_sched *sch);
bool lupos_scx_sub_lifecycle_has_cgroup_exit(struct scx_sched *sch);
/* One SCX_CALL_OP* site per original claim/return callback invocation. */
s32 lupos_scx_sub_lifecycle_call_cgroup_claim_init(struct scx_sched *sch, struct cgroup *cgrp, struct scx_cgroup_init_args *args);
void lupos_scx_sub_lifecycle_call_cgroup_claim_parent_exit(struct scx_sched *parent, struct cgroup *cgrp);
void lupos_scx_sub_lifecycle_call_cgroup_claim_cancel_exit(struct scx_sched *sch, struct cgroup *cgrp);
void lupos_scx_sub_lifecycle_call_cgroup_return_exit(struct scx_sched *sch, struct cgroup *cgrp);
s32 lupos_scx_sub_lifecycle_call_cgroup_return_init(struct scx_sched *parent, struct cgroup *cgrp, struct scx_cgroup_init_args *args);
void lupos_scx_sub_lifecycle_cgroup_init_failed(struct scx_sched *sch, s32 ret);
void lupos_scx_sub_lifecycle_cgroup_return_failed(struct scx_sched *parent, s32 ret);
void lupos_scx_sub_lifecycle_warn_unreturned(struct task_group *tg, struct scx_sched *sch);
#endif
#endif
#endif
