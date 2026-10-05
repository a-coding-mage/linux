// SPDX-License-Identifier: GPL-2.0
/* F08 native types, static rwsem and narrow primitive/macro leaves. Rust owns
 * the ext.c cgroup state machines, iteration and lifetime/error ordering.
 * This belongs in the future single native envelope, never a separate owner
 * or the old ext.c algorithm fallback. Native runtime remains unqualified.
 */
#error "SOURCE ONLY HOLD: sched_ext cgroup native ABI and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_cgroup_bindings.h"

#ifdef CONFIG_EXT_GROUP_SCHED

/* Exact ext.c:4678 native initializer and sole storage owner. */
DEFINE_STATIC_PERCPU_RWSEM(scx_cgroup_ops_rwsem);

u64 lupos_scx_core_cgroup_default_bw_period_us(void)
{
	return default_bw_period_us();
}

struct task_group *lupos_scx_core_cgroup_root_task_group(void)
{
	return &root_task_group;
}

struct task_group *lupos_scx_core_cgroup_css_tg(struct cgroup_subsys_state *css)
{
	return css_tg(css);
}

void lupos_scx_core_cgroup_assert_tg_sched(void)
{
	lockdep_assert(lockdep_is_held(&cgroup_mutex) ||
		       lockdep_is_held(&scx_cgroup_ops_rwsem));
}

void lupos_scx_core_cgroup_assert_tg_knob_sched(void)
{
	lockdep_assert(lockdep_is_held(&cgroup_mutex) ||
		       lockdep_is_held(&scx_cgroup_ops_rwsem));
}

void lupos_scx_core_cgroup_warn_online(struct task_group *tg)
{
	WARN_ON_ONCE(tg->scx.flags & (SCX_TG_ONLINE | SCX_TG_INITED));
}

void lupos_scx_core_cgroup_warn_offline(struct task_group *tg)
{
	WARN_ON_ONCE(!(tg->scx.flags & SCX_TG_ONLINE));
}

void lupos_scx_core_cgroup_warn_moving_from(struct task_struct *p)
{
	WARN_ON_ONCE(p->scx.cgrp_moving_from);
}

bool lupos_scx_core_cgroup_on_dfl(struct cgroup *cgrp)
{
	return cgroup_on_dfl(cgrp);
}

/* These header-defined scheduler reads retain their original RCU diagnostics.
 * They do not implement subtree assignment, claim/return or notifier owners.
 */
struct scx_sched *lupos_scx_core_cgroup_online_sched(struct cgroup *cgrp)
{
	return scx_cgroup_sched(cgrp);
}

struct scx_sched *lupos_scx_core_cgroup_task_sched_protected(struct task_struct *p)
{
	return rcu_dereference_protected(p->scx.sched, lockdep_is_held(&cgroup_mutex));
}

struct scx_sched *lupos_scx_core_cgroup_migration_dst_sched(struct task_struct *p)
{
	return scx_cgroup_sched(task_css_set(p)->mg_dst_cset->dfl_cgrp);
}

struct task_struct *lupos_scx_core_cgroup_taskset_first(
        struct cgroup_taskset *tset, struct cgroup_subsys_state **css)
{
	return cgroup_taskset_first(tset, css);
}

struct task_struct *lupos_scx_core_cgroup_taskset_next(
        struct cgroup_taskset *tset, struct cgroup_subsys_state **css)
{
	return cgroup_taskset_next(tset, css);
}

struct cgroup_subsys_state *lupos_scx_core_cgroup_next_pre(struct cgroup_subsys_state *pos)
{
	return css_next_descendant_pre(pos, &root_task_group.css);
}

struct cgroup_subsys_state *lupos_scx_core_cgroup_next_post(struct cgroup_subsys_state *pos)
{
	return css_next_descendant_post(pos, &root_task_group.css);
}

bool lupos_scx_core_cgroup_has_online_init(struct scx_sched *sch) { return SCX_HAS_OP(sch, cgroup_init); }
bool lupos_scx_core_cgroup_has_offline_exit(struct scx_sched *sch) { return SCX_HAS_OP(sch, cgroup_exit); }
bool lupos_scx_core_cgroup_has_prep_move(struct scx_sched *sch) { return SCX_HAS_OP(sch, cgroup_prep_move); }
bool lupos_scx_core_cgroup_has_prepare_cancel(struct scx_sched *sch) { return SCX_HAS_OP(sch, cgroup_cancel_move); }
bool lupos_scx_core_cgroup_has_move(struct scx_sched *sch) { return SCX_HAS_OP(sch, cgroup_move); }
bool lupos_scx_core_cgroup_has_attach_cancel(struct scx_sched *sch) { return SCX_HAS_OP(sch, cgroup_cancel_move); }
bool lupos_scx_core_cgroup_has_set_weight(struct scx_sched *sch) { return SCX_HAS_OP(sch, cgroup_set_weight); }
bool lupos_scx_core_cgroup_has_set_idle(struct scx_sched *sch) { return SCX_HAS_OP(sch, cgroup_set_idle); }
bool lupos_scx_core_cgroup_has_set_bandwidth(struct scx_sched *sch) { return SCX_HAS_OP(sch, cgroup_set_bandwidth); }
bool lupos_scx_core_cgroup_root_init_present(struct scx_sched *sch) { return sch->ops.cgroup_init; }
bool lupos_scx_core_cgroup_root_exit_present(struct scx_sched *sch) { return sch->ops.cgroup_exit; }

/* Keep original field/helper expressions at the native callback macro sites.
 * In particular, call_move must not cache task_rq(p) or evaluate arguments
 * before SCX_CALL_OP_TASK has installed its task and locked-rq context.
 */
int lupos_scx_core_cgroup_call_online_init(struct scx_sched *sch, struct task_group *tg,
                                           struct scx_cgroup_init_args *args)
{
	return SCX_CALL_OP_RET(sch, cgroup_init, NULL, tg->css.cgroup, args);
}

/* Header policy dependency, not an ext.c owner algorithm fallback. */
int lupos_scx_core_cgroup_sanitize_online_err(struct scx_sched *sch, s32 err)
{
	return scx_ops_sanitize_err(sch, "cgroup_init", err);
}

void lupos_scx_core_cgroup_call_offline_exit(struct scx_sched *sch, struct task_group *tg)
{
	SCX_CALL_OP(sch, cgroup_exit, NULL, tg->css.cgroup);
}

int lupos_scx_core_cgroup_call_prep_move(struct scx_sched *sch, struct task_struct *p,
                                         struct cgroup *from, struct cgroup_subsys_state *css)
{
	return SCX_CALL_OP_RET(sch, cgroup_prep_move, NULL, p, from, css->cgroup);
}

int lupos_scx_core_cgroup_sanitize_prep_err(struct scx_sched *sch, s32 err)
{
	return scx_ops_sanitize_err(sch, "cgroup_prep_move", err);
}

void lupos_scx_core_cgroup_call_prepare_cancel(struct scx_sched *sch, struct task_struct *p,
                                               struct cgroup_subsys_state *css)
{
	SCX_CALL_OP(sch, cgroup_cancel_move, NULL,
		    p, p->scx.cgrp_moving_from, css->cgroup);
}

void lupos_scx_core_cgroup_call_move(struct scx_sched *sch, struct task_struct *p)
{
	SCX_CALL_OP_TASK(sch, cgroup_move, task_rq(p),
			 p, p->scx.cgrp_moving_from,
			 lupos_scx_core_cgroup_tg_cgrp(task_group(p)));
}

void lupos_scx_core_cgroup_call_attach_cancel(struct scx_sched *sch, struct task_struct *p,
                                              struct cgroup_subsys_state *css)
{
	SCX_CALL_OP(sch, cgroup_cancel_move, NULL,
		    p, p->scx.cgrp_moving_from, css->cgroup);
}

void lupos_scx_core_cgroup_call_set_weight(struct scx_sched *sch, struct task_group *tg,
                                           unsigned long weight)
{
	/* Native callback slot is u32; retain the source's unsigned-long conversion. */
	SCX_CALL_OP(sch, cgroup_set_weight, NULL, lupos_scx_core_cgroup_tg_cgrp(tg), weight);
}

void lupos_scx_core_cgroup_call_set_idle(struct scx_sched *sch, struct task_group *tg, bool idle)
{
	SCX_CALL_OP(sch, cgroup_set_idle, NULL, lupos_scx_core_cgroup_tg_cgrp(tg), idle);
}

void lupos_scx_core_cgroup_call_set_bandwidth(struct scx_sched *sch, struct task_group *tg,
                                              u64 period_us, u64 quota_us, u64 burst_us)
{
	SCX_CALL_OP(sch, cgroup_set_bandwidth, NULL,
		    lupos_scx_core_cgroup_tg_cgrp(tg), period_us, quota_us, burst_us);
}

void lupos_scx_core_cgroup_call_root_exit(struct scx_sched *sch, struct cgroup_subsys_state *css)
{
	SCX_CALL_OP(sch, cgroup_exit, NULL, css->cgroup);
}

int lupos_scx_core_cgroup_call_root_init(struct scx_sched *sch, struct cgroup_subsys_state *css,
                                         struct scx_cgroup_init_args *args)
{
	return SCX_CALL_OP_RET(sch, cgroup_init, NULL, css->cgroup, args);
}

void lupos_scx_core_cgroup_error_root_init(struct scx_sched *sch, int ret)
{
	scx_error(sch, "ops.cgroup_init() failed (%d)", ret);
}

void lupos_scx_core_cgroup_weight_down_read(void) { percpu_down_read(&scx_cgroup_ops_rwsem); }
void lupos_scx_core_cgroup_weight_up_read(void) { percpu_up_read(&scx_cgroup_ops_rwsem); }
void lupos_scx_core_cgroup_idle_down_read(void) { percpu_down_read(&scx_cgroup_ops_rwsem); }
void lupos_scx_core_cgroup_idle_up_read(void) { percpu_up_read(&scx_cgroup_ops_rwsem); }
void lupos_scx_core_cgroup_bandwidth_down_read(void) { percpu_down_read(&scx_cgroup_ops_rwsem); }
void lupos_scx_core_cgroup_bandwidth_up_read(void) { percpu_up_read(&scx_cgroup_ops_rwsem); }
void lupos_scx_core_cgroup_ops_down_write(void) { percpu_down_write(&scx_cgroup_ops_rwsem); }
void lupos_scx_core_cgroup_ops_up_write(void) { percpu_up_write(&scx_cgroup_ops_rwsem); }

#endif /* CONFIG_EXT_GROUP_SCHED */

#if defined(CONFIG_EXT_GROUP_SCHED) || defined(CONFIG_EXT_SUB_SCHED)
struct cgroup *lupos_scx_core_cgroup_default_root(void)
{
	return &cgrp_dfl_root.cgrp;
}

void lupos_scx_core_cgroup_mutex_lock(void)
{
	cgroup_lock();
}

void lupos_scx_core_cgroup_mutex_unlock(void)
{
	cgroup_unlock();
}
#endif
