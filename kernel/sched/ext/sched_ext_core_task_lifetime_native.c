// SPDX-License-Identifier: GPL-2.0
/* F06 native macro/primitive boundary. Algorithms and lifetime decisions are
 * in Rust. Shared objects are provided only by F00. This file is a member of
 * the future single native envelope, not a separately linked runtime object.
 */
#error "SOURCE ONLY HOLD: task lifetime native ABI and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_task_lifetime_bindings.h"

void lupos_scx_core_task_warn_state(const struct task_struct *p, u32 prev, u32 state)
{
	WARN_ONCE(1, "sched_ext: Invalid task state %d -> %d for %s[%d]",
		  prev, state, p->comm, p->pid);
}

void lupos_scx_core_task_warn_transition(const struct task_struct *p, u32 prev,
                                         u32 state, bool warn)
{
	WARN_ONCE(warn, "sched_ext: Invalid task state transition 0x%x -> 0x%x for %s[%d]",
		  prev, state, p->comm, p->pid);
}

void lupos_scx_core_task_zero_iter(struct scx_task_iter *iter)
{
	memset(iter, 0, sizeof(*iter));
}

void lupos_scx_core_task_list_lock_irq(void)
{
	raw_spin_lock_irq(lupos_scx_core_tasks_lock());
}

void lupos_scx_core_task_list_unlock_irq(void)
{
	raw_spin_unlock_irq(lupos_scx_core_tasks_lock());
}

void lupos_scx_core_task_list_add(struct list_head *node, struct list_head *head)
{
	list_add(node, head);
}

void lupos_scx_core_task_list_move(struct list_head *node, struct list_head *head)
{
	list_move(node, head);
}

void lupos_scx_core_task_list_del_init(struct list_head *node)
{
	list_del_init(node);
}

struct sched_ext_entity *lupos_scx_core_task_node_entity(struct list_head *node)
{
	return container_of(node, struct sched_ext_entity, tasks_node);
}

struct task_struct *lupos_scx_core_task_entity_task(struct sched_ext_entity *scx)
{
	return container_of(scx, struct task_struct, scx);
}

void lupos_scx_core_task_balance_callbacks(struct rq *rq, struct rq_flags *rf)
{
	__balance_callbacks(rq, rf);
}

struct rq *lupos_scx_core_task_rq_lock(struct task_struct *p, struct rq_flags *rf)
{
	return task_rq_lock(p, rf);
}

void lupos_scx_core_task_rq_unlock(struct rq *rq, struct task_struct *p,
                                   struct rq_flags *rf)
{
	task_rq_unlock(rq, p, rf);
}

void lupos_scx_core_task_cond_resched(void)
{
	cond_resched();
}

const struct sched_class *lupos_scx_core_task_idle_class(void)
{
	return &idle_sched_class;
}

__noreturn void lupos_scx_core_task_iter_bug(void)
{
	BUG();
}

#ifdef CONFIG_EXT_SUB_SCHED
void lupos_scx_core_task_assert_cgroup_mutex(void)
{
	lockdep_assert_held(&cgroup_mutex);
}

struct cgroup_subsys_state *lupos_scx_core_task_css_next(
        struct cgroup_subsys_state *pos, struct cgroup *root)
{
	return css_next_descendant_pre(pos, &root->self);
}

void lupos_scx_core_task_css_start(struct cgroup_subsys_state *pos,
                                   struct css_task_iter *iter)
{
	css_task_iter_start(pos, CSS_TASK_ITER_WITH_DEAD, iter);
}

struct task_struct *lupos_scx_core_task_css_next_task(struct css_task_iter *iter)
{
	return css_task_iter_next(iter);
}

void lupos_scx_core_task_css_end(struct css_task_iter *iter)
{
	css_task_iter_end(iter);
}
#endif

#ifdef CONFIG_EXT_GROUP_SCHED
struct cgroup *lupos_scx_core_task_default_cgroup(void)
{
	return &cgrp_dfl_root.cgrp;
}

struct task_group *lupos_scx_core_task_group(struct task_struct *p)
{
	return task_group(p);
}
#endif

bool lupos_scx_core_task_has_init(struct scx_sched *sch) { return SCX_HAS_OP(sch, init_task); }
bool lupos_scx_core_task_has_enable(struct scx_sched *sch) { return SCX_HAS_OP(sch, enable); }
bool lupos_scx_core_task_has_enable_weight(struct scx_sched *sch) { return SCX_HAS_OP(sch, set_weight); }
bool lupos_scx_core_task_has_disable(struct scx_sched *sch) { return SCX_HAS_OP(sch, disable); }
bool lupos_scx_core_task_has_exit(struct scx_sched *sch) { return SCX_HAS_OP(sch, exit_task); }
bool lupos_scx_core_task_has_cancel_exit(struct scx_sched *sch) { return SCX_HAS_OP(sch, exit_task); }
bool lupos_scx_core_task_has_reweight(struct scx_sched *sch) { return SCX_HAS_OP(sch, set_weight); }
bool lupos_scx_core_task_has_cpumask(struct scx_sched *sch) { return SCX_HAS_OP(sch, set_cpumask); }

int lupos_scx_core_task_call_init(struct scx_sched *sch, struct task_struct *p,
                                  struct scx_init_task_args *args)
{
	return SCX_CALL_OP_RET(sch, init_task, NULL, p, args);
}

/* Header-defined policy dependency, not an ext.c algorithm fallback. */
int lupos_scx_core_task_sanitize_init_err(struct scx_sched *sch, s32 err)
{
	return scx_ops_sanitize_err(sch, "init_task", err);
}

void lupos_scx_core_task_error_disallow_parent(struct scx_sched *sch, struct task_struct *p)
{
	scx_error(sch, "non-root ops.init_task() set task->scx.disallow for %s[%d]",
		  p->comm, p->pid);
}

void lupos_scx_core_task_error_disallow_fork(struct scx_sched *sch, struct task_struct *p)
{
	scx_error(sch, "ops.init_task() set task->scx.disallow for %s[%d] during fork",
		  p->comm, p->pid);
}

void lupos_scx_core_task_error_disallow_enable(struct scx_sched *sch, struct task_struct *p)
{
	scx_error(sch, "ops.init_task() set task->scx.disallow for %s[%d] outside the enable path",
		  p->comm, p->pid);
}

struct rq *lupos_scx_core_task_rq(struct task_struct *p) { return task_rq(p); }
void lupos_scx_core_task_assert_enable_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_core_task_warn_enable_custody(struct task_struct *p) { WARN_ON_ONCE(p->scx.flags & SCX_TASK_IN_CUSTODY); }
bool lupos_scx_core_task_idle_policy(struct task_struct *p) { return task_has_idle_policy(p); }
u32 lupos_scx_core_task_prio_weight(int index) { return sched_prio_to_weight[index]; }
unsigned long lupos_scx_core_task_weight_to_cgroup(unsigned long weight) { return sched_weight_to_cgroup(weight); }

void lupos_scx_core_task_call_enable(struct scx_sched *sch, struct rq *rq, struct task_struct *p)
{
	SCX_CALL_OP_TASK(sch, enable, rq, p);
}

void lupos_scx_core_task_call_enable_weight(struct scx_sched *sch, struct rq *rq, struct task_struct *p)
{
	SCX_CALL_OP_TASK(sch, set_weight, rq, p, p->scx.weight);
}

void lupos_scx_core_task_assert_disable_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_core_task_warn_disable_state(struct task_struct *p) { WARN_ON_ONCE(scx_get_task_state(p) != SCX_TASK_ENABLED); }

void lupos_scx_core_task_call_disable(struct scx_sched *sch, struct rq *rq, struct task_struct *p)
{
	SCX_CALL_OP_TASK(sch, disable, rq, p);
}

void lupos_scx_core_task_warn_disable_custody(struct task_struct *p) { WARN_ON_ONCE(p->scx.flags & SCX_TASK_IN_CUSTODY); }
void lupos_scx_core_task_assert_exit_pi(struct task_struct *p) { lockdep_assert_held(&p->pi_lock); }
void lupos_scx_core_task_assert_exit_rq(struct task_struct *p) { lockdep_assert_rq_held(task_rq(p)); }
void lupos_scx_core_task_warn_exit_state(void) { WARN_ON_ONCE(true); }

void lupos_scx_core_task_call_exit(struct scx_sched *sch, struct task_struct *p,
                                  struct scx_exit_task_args *args)
{
	SCX_CALL_OP_TASK(sch, exit_task, task_rq(p), p, args);
}

void lupos_scx_core_task_assert_cancel_pi(struct task_struct *p) { lockdep_assert_held(&p->pi_lock); }
void lupos_scx_core_task_assert_cancel_rq(struct task_struct *p) { lockdep_assert_rq_held(task_rq(p)); }

void lupos_scx_core_task_call_cancel_exit(struct scx_sched *sch, struct task_struct *p,
                                         struct scx_exit_task_args *args)
{
	__SCX_CALL_OP_TASK(sch, ops, exit_task, task_rq(p), p, args);
}

/* F00 must precede this file: its !SUB NULL macro is authoritative. */
bool lupos_scx_core_task_warn_no_sub(void) { return WARN_ON_ONCE(!scx_enabling_sub_sched); }
void lupos_scx_core_task_set_sched(struct task_struct *p, struct scx_sched *sch) { scx_set_task_sched(p, sch); }
struct scx_sched *lupos_scx_core_task_sched(const struct task_struct *p) { return scx_task_sched(p); }
void lupos_scx_core_task_zero_entity(struct sched_ext_entity *scx) { memset(scx, 0, sizeof(*scx)); }
void lupos_scx_core_task_init_list(struct list_head *head) { INIT_LIST_HEAD(head); }
void lupos_scx_core_task_clear_rb(struct rb_node *node) { RB_CLEAR_NODE(node); }
unsigned long lupos_scx_core_task_jiffies(void) { return jiffies; }
void lupos_scx_core_task_preempt_disable(void) { preempt_disable(); }
void lupos_scx_core_task_preempt_enable(void) { preempt_enable(); }
void lupos_scx_core_task_assert_tid_lock(void) { lockdep_assert_held(lupos_scx_core_tasks_lock()); }

int lupos_scx_core_task_tid_insert(struct task_struct *p)
{
	return rhashtable_lookup_insert_fast(lupos_scx_core_tid_hash(),
		&p->scx.tid_hash_node, *lupos_scx_core_tid_hash_params());
}

void lupos_scx_core_task_warn_tid_insert(int ret) { WARN_ON_ONCE(ret); }
void lupos_scx_core_task_fork_down_read(void) { percpu_down_read(lupos_scx_core_fork_rwsem()); }
void lupos_scx_core_task_assert_fork_sem(void) { percpu_rwsem_assert_held(lupos_scx_core_fork_rwsem()); }
void lupos_scx_core_task_fork_up_read(void) { percpu_up_read(lupos_scx_core_fork_rwsem()); }
#ifdef CONFIG_EXT_SUB_SCHED
struct cgroup *lupos_scx_core_task_fork_cgroup(struct kernel_clone_args *kargs) { return kargs->cset->dfl_cgrp; }
struct scx_sched *lupos_scx_core_task_cgroup_sched(struct cgroup *cgrp) { return scx_cgroup_sched(cgrp); }
#else
struct scx_sched *lupos_scx_core_task_root_protected_live(void) { return scx_root_protected_live(); }
#endif
void lupos_scx_core_task_list_add_tail(struct list_head *node, struct list_head *head) { list_add_tail(node, head); }
void lupos_scx_core_task_warn_cancel_state(struct task_struct *p) { WARN_ON_ONCE(scx_get_task_state(p) >= SCX_TASK_READY); }
void lupos_scx_core_task_assert_dead_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
unsigned int lupos_scx_core_task_state_read_once(struct task_struct *p) { return READ_ONCE(p->__state); }
bool lupos_scx_core_task_on_cpu(struct rq *rq, struct task_struct *p) { return task_on_cpu(rq, p); }

/*
 * Keep the actual scoped_guard macros, including their context-analysis
 * annotations and configured cleanup. In the pinned native headers both guard
 * kinds below use raw_spin_lock_irq_disable/raw_spin_unlock_irq_enable and the
 * refcounted local_interrupt API; ordinary irq/irqsave pairs are not equivalent.
 * SAFETY: p is pinned by the owning Rust lifecycle entry. Each synchronous Rust
 * body returns normally while retaining this lock and cannot unwind across C.
 */
void lupos_scx_core_task_post_fork_publish(struct task_struct *p)
{
	scoped_guard(raw_spinlock_irq, lupos_scx_core_tasks_lock()) {
		lupos_scx_core_task_post_fork_publish_body(p);
	}
}

void lupos_scx_core_task_dead_unpublish(struct task_struct *p)
{
	scoped_guard(raw_spinlock_irqsave, lupos_scx_core_tasks_lock()) {
		lupos_scx_core_task_dead_unpublish_body(p);
	}
}

void lupos_scx_core_task_tid_remove(struct task_struct *p)
{
	rhashtable_remove_fast(lupos_scx_core_tid_hash(),
		&p->scx.tid_hash_node, *lupos_scx_core_tid_hash_params());
}

void lupos_scx_core_task_assert_reweight_rq(struct task_struct *p) { lockdep_assert_rq_held(task_rq(p)); }
unsigned long lupos_scx_core_task_scale_load_down(unsigned long weight) { return scale_load_down(weight); }

void lupos_scx_core_task_call_reweight(struct scx_sched *sch, struct rq *rq, struct task_struct *p)
{
	SCX_CALL_OP_TASK(sch, set_weight, rq, p, p->scx.weight);
}

void lupos_scx_core_task_assert_setscheduler_rq(struct task_struct *p) { lockdep_assert_rq_held(task_rq(p)); }
bool lupos_scx_core_task_enabled(void) { return scx_enabled(); }
bool lupos_scx_core_task_disallow_read_once(struct task_struct *p) { return READ_ONCE(p->scx.disallow); }
bool lupos_scx_core_task_unlikely_init_error(bool condition) { return unlikely(condition); }
bool lupos_scx_core_task_unlikely_disallow_parent(bool condition) { return unlikely(condition); }
bool lupos_scx_core_task_unlikely_disallow_fork(bool condition) { return unlikely(condition); }
bool lupos_scx_core_task_unlikely_disallow_enable(bool condition) { return unlikely(condition); }
bool lupos_scx_core_task_unlikely_tid_refill(bool condition) { return unlikely(condition); }
bool lupos_scx_core_task_unlikely_fork_error(bool condition) { return unlikely(condition); }
bool lupos_scx_core_task_unlikely_dead(bool condition) { return unlikely(condition); }
bool lupos_scx_core_task_unlikely_disabling(bool condition) { return unlikely(condition); }
bool lupos_scx_core_task_unlikely_no_sched(bool condition) { return unlikely(condition); }
bool lupos_scx_core_task_unlikely_non_ext(bool condition) { return unlikely(condition); }

/* Native class callback identities remain in the original shared envelope;
 * these adapters do not execute the old C scheduling algorithms. */
static void reweight_task_scx(struct rq *rq, struct task_struct *p,
                              const struct load_weight *lw)
{
	lupos_scx_core_task_reweight_body(rq, p, lw);
}

static void prio_changed_scx(struct rq *rq, struct task_struct *p, u64 oldprio)
{
	lupos_scx_core_task_prio_changed_body(rq, p, oldprio);
}

static void switching_to_scx(struct rq *rq, struct task_struct *p)
{
	lupos_scx_core_task_switching_to_body(rq, p);
}

static void switched_from_scx(struct rq *rq, struct task_struct *p)
{
	lupos_scx_core_task_switched_from_body(rq, p);
}

static void switched_to_scx(struct rq *rq, struct task_struct *p)
{
	lupos_scx_core_task_switched_to_body(rq, p);
}

u64 lupos_scx_core_task_ops_flags(const struct scx_sched *sch)
{
	return sch->ops.flags;
}
