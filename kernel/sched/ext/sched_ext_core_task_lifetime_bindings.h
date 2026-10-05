/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_TASK_LIFETIME_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_TASK_LIFETIME_BINDINGS_H

/* F06 source-only native interface. All structures, locks, enums and values
 * come from the configured original headers and F00's one native envelope.
 * No storage or Rust-owned native layout is introduced here.
 */
#include "internal.h"

void lupos_scx_core_task_warn_state(const struct task_struct *p, u32 prev, u32 state);
void lupos_scx_core_task_warn_transition(const struct task_struct *p, u32 prev,
                                         u32 state, bool warn);
void lupos_scx_core_task_zero_iter(struct scx_task_iter *iter);
void lupos_scx_core_task_list_lock_irq(void);
void lupos_scx_core_task_list_unlock_irq(void);
void lupos_scx_core_task_list_add(struct list_head *node, struct list_head *head);
void lupos_scx_core_task_list_move(struct list_head *node, struct list_head *head);
void lupos_scx_core_task_list_del_init(struct list_head *node);
struct sched_ext_entity *lupos_scx_core_task_node_entity(struct list_head *node);
struct task_struct *lupos_scx_core_task_entity_task(struct sched_ext_entity *scx);
void lupos_scx_core_task_balance_callbacks(struct rq *rq, struct rq_flags *rf);
struct rq *lupos_scx_core_task_rq_lock(struct task_struct *p, struct rq_flags *rf);
void lupos_scx_core_task_rq_unlock(struct rq *rq, struct task_struct *p,
                                   struct rq_flags *rf);
void lupos_scx_core_task_cond_resched(void);
const struct sched_class *lupos_scx_core_task_idle_class(void);
__noreturn void lupos_scx_core_task_iter_bug(void);
#ifdef CONFIG_EXT_SUB_SCHED
void lupos_scx_core_task_assert_cgroup_mutex(void);
struct cgroup_subsys_state *lupos_scx_core_task_css_next(
        struct cgroup_subsys_state *pos, struct cgroup *root);
void lupos_scx_core_task_css_start(struct cgroup_subsys_state *pos,
                                   struct css_task_iter *iter);
struct task_struct *lupos_scx_core_task_css_next_task(struct css_task_iter *iter);
void lupos_scx_core_task_css_end(struct css_task_iter *iter);
#endif

/* Native-width macro values used by Rust without copied numeric authority. */
#define LUPOS_SCX_CORE_TASK_DSQ_INVALID SCX_DSQ_INVALID
#define LUPOS_SCX_CORE_TASK_SLICE_DFL SCX_SLICE_DFL
#define LUPOS_SCX_CORE_TASK_ALLOW_QUEUED_WAKEUP SCX_OPS_ALLOW_QUEUED_WAKEUP
#ifdef CONFIG_EXT_GROUP_SCHED
struct cgroup *lupos_scx_core_task_default_cgroup(void);
struct task_group *lupos_scx_core_task_group(struct task_struct *p);
#endif
bool lupos_scx_core_task_has_init(struct scx_sched *sch);
bool lupos_scx_core_task_has_enable(struct scx_sched *sch);
bool lupos_scx_core_task_has_enable_weight(struct scx_sched *sch);
bool lupos_scx_core_task_has_disable(struct scx_sched *sch);
bool lupos_scx_core_task_has_exit(struct scx_sched *sch);
bool lupos_scx_core_task_has_cancel_exit(struct scx_sched *sch);
bool lupos_scx_core_task_has_reweight(struct scx_sched *sch);
bool lupos_scx_core_task_has_cpumask(struct scx_sched *sch);
int lupos_scx_core_task_call_init(struct scx_sched *sch, struct task_struct *p,
                                  struct scx_init_task_args *args);
int lupos_scx_core_task_sanitize_init_err(struct scx_sched *sch, s32 err);
void lupos_scx_core_task_error_disallow_parent(struct scx_sched *sch, struct task_struct *p);
void lupos_scx_core_task_error_disallow_fork(struct scx_sched *sch, struct task_struct *p);
void lupos_scx_core_task_error_disallow_enable(struct scx_sched *sch, struct task_struct *p);
struct rq *lupos_scx_core_task_rq(struct task_struct *p);
void lupos_scx_core_task_assert_enable_rq(struct rq *rq);
void lupos_scx_core_task_warn_enable_custody(struct task_struct *p);
bool lupos_scx_core_task_idle_policy(struct task_struct *p);
u32 lupos_scx_core_task_prio_weight(int index);
unsigned long lupos_scx_core_task_weight_to_cgroup(unsigned long weight);
void lupos_scx_core_task_call_enable(struct scx_sched *sch, struct rq *rq, struct task_struct *p);
void lupos_scx_core_task_call_enable_weight(struct scx_sched *sch, struct rq *rq, struct task_struct *p);
void lupos_scx_core_task_assert_disable_rq(struct rq *rq);
void lupos_scx_core_task_warn_disable_state(struct task_struct *p);
void lupos_scx_core_task_call_disable(struct scx_sched *sch, struct rq *rq, struct task_struct *p);
void lupos_scx_core_task_warn_disable_custody(struct task_struct *p);
void lupos_scx_core_task_assert_exit_pi(struct task_struct *p);
void lupos_scx_core_task_assert_exit_rq(struct task_struct *p);
void lupos_scx_core_task_warn_exit_state(void);
void lupos_scx_core_task_call_exit(struct scx_sched *sch, struct task_struct *p,
                                  struct scx_exit_task_args *args);
void lupos_scx_core_task_assert_cancel_pi(struct task_struct *p);
void lupos_scx_core_task_assert_cancel_rq(struct task_struct *p);
void lupos_scx_core_task_call_cancel_exit(struct scx_sched *sch, struct task_struct *p,
                                         struct scx_exit_task_args *args);
bool lupos_scx_core_task_warn_no_sub(void);
void lupos_scx_core_task_set_sched(struct task_struct *p, struct scx_sched *sch);
struct scx_sched *lupos_scx_core_task_sched(const struct task_struct *p);
void lupos_scx_core_task_zero_entity(struct sched_ext_entity *scx);
void lupos_scx_core_task_init_list(struct list_head *head);
void lupos_scx_core_task_clear_rb(struct rb_node *node);
unsigned long lupos_scx_core_task_jiffies(void);
void lupos_scx_core_task_preempt_disable(void);
void lupos_scx_core_task_preempt_enable(void);
void lupos_scx_core_task_assert_tid_lock(void);
int lupos_scx_core_task_tid_insert(struct task_struct *p);
void lupos_scx_core_task_warn_tid_insert(int ret);
void lupos_scx_core_task_fork_down_read(void);
void lupos_scx_core_task_assert_fork_sem(void);
void lupos_scx_core_task_fork_up_read(void);
#ifdef CONFIG_EXT_SUB_SCHED
struct cgroup *lupos_scx_core_task_fork_cgroup(struct kernel_clone_args *kargs);
struct scx_sched *lupos_scx_core_task_cgroup_sched(struct cgroup *cgrp);
#else
struct scx_sched *lupos_scx_core_task_root_protected_live(void);
#endif
void lupos_scx_core_task_list_add_tail(struct list_head *node, struct list_head *head);
void lupos_scx_core_task_warn_cancel_state(struct task_struct *p);
void lupos_scx_core_task_assert_dead_rq(struct rq *rq);
unsigned int lupos_scx_core_task_state_read_once(struct task_struct *p);
bool lupos_scx_core_task_on_cpu(struct rq *rq, struct task_struct *p);
/* Exact native scoped guards enclose synchronous Rust list/hash bodies. Their
 * refcounted interrupt semantics must not be replaced by saved-flags leaves. */
void lupos_scx_core_task_post_fork_publish(struct task_struct *p);
void lupos_scx_core_task_post_fork_publish_body(struct task_struct *p);
void lupos_scx_core_task_dead_unpublish(struct task_struct *p);
void lupos_scx_core_task_dead_unpublish_body(struct task_struct *p);
void lupos_scx_core_task_tid_remove(struct task_struct *p);
void lupos_scx_core_task_assert_reweight_rq(struct task_struct *p);
unsigned long lupos_scx_core_task_scale_load_down(unsigned long weight);
void lupos_scx_core_task_call_reweight(struct scx_sched *sch, struct rq *rq, struct task_struct *p);
void lupos_scx_core_task_assert_setscheduler_rq(struct task_struct *p);
bool lupos_scx_core_task_enabled(void);
bool lupos_scx_core_task_disallow_read_once(struct task_struct *p);
/* Each original unlikely site keeps a separate native expansion. */
bool lupos_scx_core_task_unlikely_init_error(bool condition);
bool lupos_scx_core_task_unlikely_disallow_parent(bool condition);
bool lupos_scx_core_task_unlikely_disallow_fork(bool condition);
bool lupos_scx_core_task_unlikely_disallow_enable(bool condition);
bool lupos_scx_core_task_unlikely_tid_refill(bool condition);
bool lupos_scx_core_task_unlikely_fork_error(bool condition);
bool lupos_scx_core_task_unlikely_dead(bool condition);
bool lupos_scx_core_task_unlikely_disabling(bool condition);
bool lupos_scx_core_task_unlikely_no_sched(bool condition);
bool lupos_scx_core_task_unlikely_non_ext(bool condition);

/* Native sched_class callback adapters keep original C signatures; their CFI
 * qualification remains open. These symbols call the Rust bodies declared below; F17 alone owns
 * the sched_class record that will reference them in the common envelope. */
void lupos_scx_core_task_reweight_body(struct rq *rq, struct task_struct *p,
                                       const struct load_weight *lw);
void lupos_scx_core_task_prio_changed_body(struct rq *rq, struct task_struct *p, u64 oldprio);
void lupos_scx_core_task_switching_to_body(struct rq *rq, struct task_struct *p);
void lupos_scx_core_task_switched_from_body(struct rq *rq, struct task_struct *p);
void lupos_scx_core_task_switched_to_body(struct rq *rq, struct task_struct *p);

u64 lupos_scx_core_task_ops_flags(const struct scx_sched *sch);
#endif /* LUPOS_SCHED_EXT_CORE_TASK_LIFETIME_BINDINGS_H */
