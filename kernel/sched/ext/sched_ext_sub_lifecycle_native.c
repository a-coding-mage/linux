// SPDX-License-Identifier: GPL-2.0
/*
 * Explicit unqualified native runtime boundary. These leaves retain native
 * macros, inline accessors, callback dispatch, diagnostics and metadata only.
 * Lifecycle traversal, transfer decisions and unwind policy live in Rust.
 * This fragment never includes, renames or falls back to the old sub.c owner.
 */
#error "SOURCE ONLY HOLD: sched_ext lifecycle native ABI/protection admission is pending"
#include "sched_ext_sub_bindings.h"
#ifdef CONFIG_EXT_SUB_SCHED
/* Keep sub.c:1255-1256 separate from the punt assertions at 1284-1285. */
void lupos_scx_sub_lifecycle_assert_rehome_task_locked(struct task_struct *p)
{
	lockdep_assert_held(&p->pi_lock);
	lockdep_assert_rq_held(task_rq(p));
}
void lupos_scx_sub_lifecycle_assert_punt_task_locked(struct task_struct *p)
{
	lockdep_assert_held(&p->pi_lock);
	lockdep_assert_rq_held(task_rq(p));
}
void lupos_scx_sub_lifecycle_warn_not_bypassed(struct scx_sched *sch)
{
	WARN_ON_ONCE(!READ_ONCE(sch->bypass_depth));
}
struct sched_change_ctx *lupos_scx_sub_lifecycle_change_begin(struct task_struct *p)
{
	return sched_change_begin(p, DEQUEUE_SAVE | DEQUEUE_MOVE);
}
void lupos_scx_sub_lifecycle_change_end(struct sched_change_ctx *ctx)
{
	sched_change_end(ctx);
}
struct scx_sched *lupos_scx_sub_lifecycle_task_sched(struct task_struct *p)
{
	return scx_task_sched(p);
}
bool lupos_scx_sub_lifecycle_task_on_sched(struct scx_sched *sch, struct task_struct *p)
{
	return scx_task_on_sched(sch, p);
}
bool lupos_scx_sub_lifecycle_task_is_ext(struct task_struct *p)
{
	return p->sched_class == &ext_sched_class;
}
void lupos_scx_sub_lifecycle_fail_parent(struct scx_sched *parent, struct task_struct *p, s32 ret)
{
	scx_error(parent, "ops.init_task() failed (%d) for %s[%d] while disabling a sub-scheduler",
		  ret, p->comm, p->pid);
}
/* This family owns one waitqueue; no second copy exists in any Rust owner. */
static DECLARE_WAIT_QUEUE_HEAD(lupos_scx_sub_lifecycle_unlink_waitq);

void lupos_scx_sub_lifecycle_wait_children(struct scx_sched *sch)
{
	wait_event(lupos_scx_sub_lifecycle_unlink_waitq, list_empty(&sch->children));
}
void lupos_scx_sub_lifecycle_wake_unlink(void)
{
	wake_up_all(&lupos_scx_sub_lifecycle_unlink_waitq);
}
void lupos_scx_sub_lifecycle_synchronize_expedited(void)
{
	synchronize_rcu_expedited();
}
void lupos_scx_sub_lifecycle_enable_lock(void)
{
	mutex_lock(&scx_enable_mutex);
}
void lupos_scx_sub_lifecycle_enable_unlock(void)
{
	mutex_unlock(&scx_enable_mutex);
}
void lupos_scx_sub_lifecycle_fork_down(void)
{
	percpu_down_write(&scx_fork_rwsem);
}
void lupos_scx_sub_lifecycle_fork_up(void)
{
	percpu_up_write(&scx_fork_rwsem);
}
void lupos_scx_sub_lifecycle_sched_lock(void)
{
	raw_spin_lock_irq(&scx_sched_lock);
}
void lupos_scx_sub_lifecycle_sched_unlock(void)
{
	raw_spin_unlock_irq(&scx_sched_lock);
}
void lupos_scx_sub_lifecycle_assert_sched_locked(void)
{
	lockdep_assert_held(&scx_sched_lock);
}
void lupos_scx_sub_lifecycle_warn_wrong_sched(struct scx_sched *sch, struct task_struct *p)
{
	WARN_ON_ONCE(!scx_task_on_sched(sch, p));
}
void lupos_scx_sub_lifecycle_get_task(struct task_struct *p)
{
	get_task_struct(p);
}
void lupos_scx_sub_lifecycle_put_task(struct task_struct *p)
{
	put_task_struct(p);
}
struct rq *lupos_scx_sub_lifecycle_task_lock(struct task_struct *p, struct rq_flags *rf)
{
	return task_rq_lock(p, rf);
}
void lupos_scx_sub_lifecycle_task_unlock(struct rq *rq, struct task_struct *p, struct rq_flags *rf)
{
	task_rq_unlock(rq, p, rf);
}
bool lupos_scx_sub_lifecycle_has_sub_detach(struct scx_sched *sch)
{
	return sch->ops.sub_detach;
}
void lupos_scx_sub_lifecycle_call_sub_detach(struct scx_sched *parent, struct scx_sub_detach_args *args)
{
	SCX_CALL_OP(parent, sub_detach, NULL, args);
}
bool lupos_scx_sub_lifecycle_has_exit(struct scx_sched *sch)
{
	return sch->ops.exit;
}
void lupos_scx_sub_lifecycle_call_exit(struct scx_sched *sch)
{
	SCX_CALL_OP(sch, exit, NULL, sch->exit_info);
}
void lupos_scx_sub_lifecycle_mark_dead(struct scx_sched *sch)
{
	WRITE_ONCE(sch->dead, true);
}
bool lupos_scx_sub_lifecycle_in_sysfs(struct scx_sched *sch)
{
	return sch->kobj.state_in_sysfs;
}
struct scx_sched *lupos_scx_sub_lifecycle_cgroup_sched(struct cgroup *cgrp)
{
	return scx_cgroup_sched(cgrp);
}
bool lupos_scx_sub_lifecycle_cgroup_descendant(struct cgroup *cgrp, struct cgroup *ancestor)
{
	return cgroup_is_descendant(cgrp, ancestor);
}
bool lupos_scx_sub_lifecycle_has_sub_attach(struct scx_sched *sch)
{
	return sch->ops.sub_attach;
}
struct scx_sched *lupos_scx_sub_lifecycle_next_child(struct scx_sched *parent, struct scx_sched *pos)
{
	if (!pos)
		return list_first_entry_or_null(&parent->children, struct scx_sched, sibling);
	if (list_is_last(&pos->sibling, &parent->children))
		return NULL;
	return list_next_entry(pos, sibling);
}
void *lupos_scx_sub_lifecycle_err_ptr(s32 err)
{
	return ERR_PTR(err);
}
bool lupos_scx_sub_lifecycle_is_err(const void *p)
{
	return IS_ERR(p);
}
s32 lupos_scx_sub_lifecycle_ptr_err(const void *p)
{
	return PTR_ERR(p);
}
void lupos_scx_sub_lifecycle_warn_task_state(struct task_struct *p, u32 state)
{
	WARN_ONCE(true, "sched_ext: Invalid task state %d for %s[%d] during enabling sub sched",
		  state, p->comm, p->pid);
}
struct scx_enable_cmd *lupos_scx_sub_lifecycle_enable_cmd(struct kthread_work *work)
{
	return container_of(work, struct scx_enable_cmd, work);
}
struct sched_ext_ops *lupos_scx_sub_lifecycle_cmd_ops(struct scx_enable_cmd *cmd)
{
	return cmd->ops;
}
bool lupos_scx_sub_lifecycle_enabled(void)
{
	return scx_enabled();
}
bool lupos_scx_sub_lifecycle_ops_published(struct sched_ext_ops *ops)
{
	return rcu_access_pointer(ops->priv);
}
void lupos_scx_sub_lifecycle_has_subs_inc(void)
{
	static_branch_inc(&__scx_has_subs);
}
void lupos_scx_sub_lifecycle_has_subs_dec(void)
{
	static_branch_dec(&__scx_has_subs);
}
void lupos_scx_sub_lifecycle_max_depth_error(struct scx_sched *sch)
{
	scx_error(sch, "max nesting depth %d violated", SCX_SUB_MAX_DEPTH);
}
bool lupos_scx_sub_lifecycle_has_init(struct scx_sched *sch)
{
	return sch->ops.init;
}
s32 lupos_scx_sub_lifecycle_call_init(struct scx_sched *sch)
{
	return SCX_CALL_OP_RET(sch, init, NULL);
}
s32 lupos_scx_sub_lifecycle_sanitize_init(struct scx_sched *sch, s32 ret)
{
	return scx_ops_sanitize_err(sch, "init", ret);
}
void lupos_scx_sub_lifecycle_init_error(struct scx_sched *sch, s32 ret)
{
	scx_error(sch, "ops.init() failed (%d)", ret);
}
s32 lupos_scx_sub_lifecycle_call_sub_attach(struct scx_sched *parent, struct scx_sub_attach_args *args)
{
	return SCX_CALL_OP_RET(parent, sub_attach, NULL, args);
}
s32 lupos_scx_sub_lifecycle_sanitize_attach(struct scx_sched *sch, s32 ret)
{
	return scx_ops_sanitize_err(sch, "sub_attach", ret);
}
void lupos_scx_sub_lifecycle_attach_error(struct scx_sched *sch, s32 ret)
{
	scx_error(sch, "parent rejected (%d)", ret);
}
bool lupos_scx_sub_lifecycle_op_present(struct sched_ext_ops *ops, s32 index)
{
	return ((void (**)(void))ops)[index];
}
void lupos_scx_sub_lifecycle_set_has_op(struct scx_sched *sch, s32 index)
{
	set_bit(index, sch->has_op);
}
void lupos_scx_sub_lifecycle_cgroup_offline_error(struct scx_sched *sch)
{
	scx_error(sch, "cgroup is not online");
}
void lupos_scx_sub_lifecycle_warn_enabling(void)
{
	WARN_ON_ONCE(scx_enabling_sub_sched);
}
void lupos_scx_sub_lifecycle_set_enabling(struct scx_sched *sch)
{
	scx_enabling_sub_sched = sch;
}
void lupos_scx_sub_lifecycle_log_enable(struct scx_sched *sch)
{
	pr_info("sched_ext: BPF sub-scheduler \"%s\" enabled\n", sch->ops.name);
}
void lupos_scx_sub_lifecycle_uevent_add(struct scx_sched *sch)
{
	kobject_uevent(&sch->kobj, KOBJ_ADD);
}
void lupos_scx_sub_lifecycle_enable_error(struct scx_sched *sch, s32 ret)
{
	scx_error(sch, "scx_sub_enable() failed (%d)", ret);
}
bool lupos_scx_sub_lifecycle_cgroup_online(struct cgroup *cgrp)
{
	return cgrp->self.flags & CSS_ONLINE;
}
void lupos_scx_sub_lifecycle_cgroup_put(struct cgroup *cgrp)
{
	cgroup_put(cgrp);
}
bool lupos_scx_sub_lifecycle_cgroup_enabled(void)
{
	return scx_cgroup_enabled;
}
s32 lupos_scx_sub_lifecycle_notifier_errno(s32 err)
{
	return notifier_from_errno(err);
}
struct cgroup *lupos_scx_sub_lifecycle_cgroup_parent(struct cgroup *cgrp)
{
	return cgroup_parent(cgrp);
}
bool lupos_scx_sub_lifecycle_cgroup_on_dfl(struct cgroup *cgrp)
{
	return cgroup_on_dfl(cgrp);
}
void lupos_scx_sub_lifecycle_publish_cgroup(struct cgroup *cgrp, struct scx_sched *sch)
{
	rcu_assign_pointer(cgrp->scx_sched, sch);
}
void lupos_scx_sub_lifecycle_exit_offline(struct scx_sched *sch, struct cgroup *cgrp)
{
	scx_exit(sch, SCX_EXIT_UNREG_KERN, SCX_ECODE_RSN_CGROUP_OFFLINE,
		 "cgroup %llu going offline", cgroup_id(cgrp));
}

/*
 * Native callback signatures, notifier object lifetime and core_initcall level
 * are retained. The trampoline/extern Rust CFI boundary is not yet qualified.
 * These two notifier blocks are this family's sole storage instances.
 */
static int lupos_scx_sub_lifecycle_lifetime_notify(struct notifier_block *nb,
						unsigned long action, void *data)
{
	return scx_cgroup_lifetime_notify(nb, action, data);
}
static struct notifier_block lupos_scx_sub_lifecycle_lifetime_nb = {
	.notifier_call = lupos_scx_sub_lifecycle_lifetime_notify,
};
static int lupos_scx_sub_lifecycle_task_notify(struct notifier_block *nb,
					    unsigned long action, void *data)
{
	return scx_cgroup_task_notify(nb, action, data);
}
static struct notifier_block lupos_scx_sub_lifecycle_task_nb = {
	.notifier_call = lupos_scx_sub_lifecycle_task_notify,
};
s32 lupos_scx_sub_lifecycle_register_lifetime(void)
{
	return blocking_notifier_chain_register(&cgroup_lifetime_notifier,
					       &lupos_scx_sub_lifecycle_lifetime_nb);
}
s32 lupos_scx_sub_lifecycle_register_task(void)
{
	return blocking_notifier_chain_register(&cgroup_task_notifier,
					       &lupos_scx_sub_lifecycle_task_nb);
}
static s32 __init lupos_scx_sub_lifecycle_notifier_init(void)
{
	return scx_cgroup_notifier_init();
}
core_initcall(lupos_scx_sub_lifecycle_notifier_init);
#ifdef CONFIG_EXT_GROUP_SCHED
struct cgroup_subsys_state *lupos_scx_sub_lifecycle_cpu_ecss(struct cgroup *cgrp)
{
	return cgroup_e_css(cgrp, &cpu_cgrp_subsys);
}
struct cgroup_subsys_state *lupos_scx_sub_lifecycle_css_pre(struct cgroup_subsys_state *pos, struct cgroup_subsys_state *root)
{
	return css_next_descendant_pre(pos, root);
}
struct cgroup_subsys_state *lupos_scx_sub_lifecycle_css_post(struct cgroup_subsys_state *pos, struct cgroup_subsys_state *root)
{
	return css_next_descendant_post(pos, root);
}
struct task_group *lupos_scx_sub_lifecycle_css_tg(struct cgroup_subsys_state *css)
{
	return css_tg(css);
}
bool lupos_scx_sub_lifecycle_has_cgroup_init(struct scx_sched *sch)
{
	return SCX_HAS_OP(sch, cgroup_init);
}
bool lupos_scx_sub_lifecycle_has_cgroup_exit(struct scx_sched *sch)
{
	return SCX_HAS_OP(sch, cgroup_exit);
}
/* These five leaves preserve the original, separate SCX_CALL_OP* sites. */
s32 lupos_scx_sub_lifecycle_call_cgroup_claim_init(struct scx_sched *sch, struct cgroup *cgrp, struct scx_cgroup_init_args *args)
{
	return SCX_CALL_OP_RET(sch, cgroup_init, NULL, cgrp, args);
}
void lupos_scx_sub_lifecycle_call_cgroup_claim_parent_exit(struct scx_sched *parent, struct cgroup *cgrp)
{
	SCX_CALL_OP(parent, cgroup_exit, NULL, cgrp);
}
void lupos_scx_sub_lifecycle_call_cgroup_claim_cancel_exit(struct scx_sched *sch, struct cgroup *cgrp)
{
	SCX_CALL_OP(sch, cgroup_exit, NULL, cgrp);
}
void lupos_scx_sub_lifecycle_call_cgroup_return_exit(struct scx_sched *sch, struct cgroup *cgrp)
{
	SCX_CALL_OP(sch, cgroup_exit, NULL, cgrp);
}
s32 lupos_scx_sub_lifecycle_call_cgroup_return_init(struct scx_sched *parent, struct cgroup *cgrp, struct scx_cgroup_init_args *args)
{
	return SCX_CALL_OP_RET(parent, cgroup_init, NULL, cgrp, args);
}
void lupos_scx_sub_lifecycle_cgroup_init_failed(struct scx_sched *sch, s32 ret)
{
	scx_error(sch, "ops.cgroup_init() failed (%d)", ret);
}
void lupos_scx_sub_lifecycle_cgroup_return_failed(struct scx_sched *parent, s32 ret)
{
	scx_error(parent, "ops.cgroup_init() failed (%d) while disabling a sub-scheduler", ret);
}
void lupos_scx_sub_lifecycle_warn_unreturned(struct task_group *tg, struct scx_sched *sch)
{
	WARN_ON_ONCE(tg->scx.sched == sch);
}
#endif
#endif
