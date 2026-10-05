/* ABI declaration requests for foundation's native DEFINE_SCHED_CLASS owner.
 * Original signatures from fair.c and struct sched_class; not implementation. */
extern void rust_fair_enqueue_task_fair(struct rq *, struct task_struct *, int);
extern bool rust_fair_dequeue_task_fair(struct rq *, struct task_struct *, int);
extern void rust_fair_yield_task_fair(struct rq *);
extern bool rust_fair_yield_to_task_fair(struct rq *, struct task_struct *);
extern void rust_fair_wakeup_preempt_fair(struct rq *, struct task_struct *, int);
extern struct task_struct *pick_task_fair(struct rq *, struct rq_flags *);
extern void rust_fair_put_prev_task_fair(struct rq *, struct task_struct *, struct task_struct *);
extern void rust_fair_set_next_task_fair(struct rq *, struct task_struct *, bool);
extern int rust_fair_select_task_rq_fair(struct task_struct *, int, int);
extern void rust_fair_migrate_task_rq_fair(struct task_struct *, int);
extern void rust_fair_rq_online_fair(struct rq *);
extern void rust_fair_rq_offline_fair(struct rq *);
extern void rust_fair_task_dead_fair(struct task_struct *);
extern void rust_fair_set_cpus_allowed_fair(struct task_struct *, struct affinity_context *);
extern void rust_fair_task_tick_fair(struct rq *, struct task_struct *, int);
extern void rust_fair_task_fork_fair(struct task_struct *);
extern void rust_fair_reweight_task_fair(struct rq *, struct task_struct *, const struct load_weight *);
extern void rust_fair_prio_changed_fair(struct rq *, struct task_struct *, u64);
extern void rust_fair_switching_from_fair(struct rq *, struct task_struct *);
extern void rust_fair_switched_from_fair(struct rq *, struct task_struct *);
extern void rust_fair_switched_to_fair(struct rq *, struct task_struct *);
extern unsigned int rust_fair_get_rr_interval_fair(struct rq *, struct task_struct *);
extern void rust_fair_update_curr_fair(struct rq *);
#ifdef CONFIG_FAIR_GROUP_SCHED
extern void rust_fair_task_change_group_fair(struct task_struct *);
#endif
#ifdef CONFIG_SCHED_CORE
extern int rust_fair_task_is_throttled_fair(struct task_struct *, int);
#endif
extern void rust_fair_sched_balance_softirq(void);
