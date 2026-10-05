/* SPDX-License-Identifier: GPL-2.0 */
/* Native class layout/section initializer only. Every callback is Rust owned. */
void rust_dl_enqueue_task_dl(struct rq *, struct task_struct *, int);
bool rust_dl_dequeue_task_dl(struct rq *, struct task_struct *, int);
void rust_dl_yield_task_dl(struct rq *);
void rust_dl_wakeup_preempt_dl(struct rq *, struct task_struct *, int);
struct task_struct *rust_dl_pick_task_dl(struct rq *, struct rq_flags *);
void rust_dl_put_prev_task_dl(struct rq *, struct task_struct *, struct task_struct *);
void rust_dl_set_next_task_dl(struct rq *, struct task_struct *, bool);
int rust_dl_balance_dl(struct rq *, struct rq_flags *);
int rust_dl_select_task_rq_dl(struct task_struct *, int, int);
void rust_dl_migrate_task_rq_dl(struct task_struct *, int);
void rust_dl_set_cpus_allowed_dl(struct task_struct *, struct affinity_context *);
void rust_dl_rq_online_dl(struct rq *);
void rust_dl_rq_offline_dl(struct rq *);
void rust_dl_task_woken_dl(struct rq *, struct task_struct *);
struct rq *rust_dl_find_lock_later_rq(struct task_struct *, struct rq *);
void rust_dl_task_tick_dl(struct rq *, struct task_struct *, int);
void rust_dl_task_fork_dl(struct task_struct *);
u64 rust_dl_get_prio_dl(struct rq *, struct task_struct *);
void rust_dl_prio_changed_dl(struct rq *, struct task_struct *, u64);
void rust_dl_switched_from_dl(struct rq *, struct task_struct *);
void rust_dl_switched_to_dl(struct rq *, struct task_struct *);
void rust_dl_update_curr_dl(struct rq *);
#ifdef CONFIG_SCHED_CORE
int rust_dl_task_is_throttled_dl(struct task_struct *, int);
#endif

DEFINE_SCHED_CLASS(dl) = {
	.enqueue_task = rust_dl_enqueue_task_dl,
	.dequeue_task = rust_dl_dequeue_task_dl,
	.yield_task = rust_dl_yield_task_dl,
	.wakeup_preempt = rust_dl_wakeup_preempt_dl,
	.pick_task = rust_dl_pick_task_dl,
	.put_prev_task = rust_dl_put_prev_task_dl,
	.set_next_task = rust_dl_set_next_task_dl,
	.balance = rust_dl_balance_dl,
	.select_task_rq = rust_dl_select_task_rq_dl,
	.migrate_task_rq = rust_dl_migrate_task_rq_dl,
	.set_cpus_allowed = rust_dl_set_cpus_allowed_dl,
	.rq_online = rust_dl_rq_online_dl,
	.rq_offline = rust_dl_rq_offline_dl,
	.task_woken = rust_dl_task_woken_dl,
	.find_lock_rq = rust_dl_find_lock_later_rq,
	.task_tick = rust_dl_task_tick_dl,
	.task_fork = rust_dl_task_fork_dl,
	.get_prio = rust_dl_get_prio_dl,
	.prio_changed = rust_dl_prio_changed_dl,
	.switched_from = rust_dl_switched_from_dl,
	.switched_to = rust_dl_switched_to_dl,
	.update_curr = rust_dl_update_curr_dl,
#ifdef CONFIG_SCHED_CORE
	.task_is_throttled = rust_dl_task_is_throttled_dl,
#endif
};
