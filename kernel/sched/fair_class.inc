DEFINE_SCHED_CLASS(fair) = {
	.enqueue_task		= rust_fair_enqueue_task_fair,
	.dequeue_task		= rust_fair_dequeue_task_fair,
	.yield_task		= rust_fair_yield_task_fair,
	.yield_to_task		= rust_fair_yield_to_task_fair,

	.wakeup_preempt		= rust_fair_wakeup_preempt_fair,

	.pick_task		= pick_task_fair,
	.put_prev_task		= rust_fair_put_prev_task_fair,
	.set_next_task          = rust_fair_set_next_task_fair,

	.select_task_rq		= rust_fair_select_task_rq_fair,
	.migrate_task_rq	= rust_fair_migrate_task_rq_fair,

	.rq_online		= rust_fair_rq_online_fair,
	.rq_offline		= rust_fair_rq_offline_fair,

	.task_dead		= rust_fair_task_dead_fair,
	.set_cpus_allowed	= rust_fair_set_cpus_allowed_fair,

	.task_tick		= rust_fair_task_tick_fair,
	.task_fork		= rust_fair_task_fork_fair,

	.reweight_task		= rust_fair_reweight_task_fair,
	.prio_changed		= rust_fair_prio_changed_fair,
	.switching_from		= rust_fair_switching_from_fair,
	.switched_from		= rust_fair_switched_from_fair,
	.switched_to		= rust_fair_switched_to_fair,

	.get_rr_interval	= rust_fair_get_rr_interval_fair,

	.update_curr		= rust_fair_update_curr_fair,

#ifdef CONFIG_FAIR_GROUP_SCHED
	.task_change_group	= rust_fair_task_change_group_fair,
#endif

#ifdef CONFIG_SCHED_CORE
	.task_is_throttled	= rust_fair_task_is_throttled_fair,
#endif

#ifdef CONFIG_UCLAMP_TASK
	.uclamp_enabled		= 1,
#endif
};
