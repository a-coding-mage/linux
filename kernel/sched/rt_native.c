// SPDX-License-Identifier: GPL-2.0
/* Native metadata/storage and executable header leaves only. rt.c algorithms
 * remain in the recovered Rust owner. This proposal is not build-admitted. */
#include "rt_native_bindings.h"

static DEFINE_PER_CPU(struct balance_callback, rust_rt_push_head_storage);
static DEFINE_PER_CPU(struct balance_callback, rust_rt_pull_head_storage);
static DEFINE_PER_CPU(cpumask_var_t, rust_rt_local_cpu_mask_storage);
#ifdef CONFIG_RT_GROUP_SCHED
static DEFINE_MUTEX(rt_constraints_mutex);
#endif

#define RT_LEAF(ret, name, args, body) ret rust_rt_##name args body
#include "rt_native_primitives.inc"
#undef RT_LEAF

/* Distinct expansions preserve separate once-state and native BUG/WARN sites. */
#define RT_WARN(name, operation) \
	noinline bool rust_rt_##name(bool condition) { return operation(condition); }
#define RT_BUG(name) \
	noinline void rust_rt_##name(bool condition) { BUG_ON(condition); }
#include "rt_native_diagnostics.inc"
#undef RT_WARN
#undef RT_BUG

#ifdef CONFIG_SYSCTL
static const struct ctl_table sched_rt_sysctls[] = {
	{
		.procname	= "sched_rt_period_us",
		.data		= &sysctl_sched_rt_period,
		.maxlen		= sizeof(int),
		.mode		= 0644,
		.proc_handler	= sched_rt_handler,
		.extra1		= SYSCTL_ONE,
		.extra2		= SYSCTL_INT_MAX,
	},
	{
		.procname	= "sched_rt_runtime_us",
		.data		= &sysctl_sched_rt_runtime,
		.maxlen		= sizeof(int),
		.mode		= 0644,
		.proc_handler	= sched_rt_handler,
		.extra1		= SYSCTL_NEG_ONE,
		.extra2		= (void *)&sysctl_sched_rt_period,
	},
	{
		.procname	= "sched_rr_timeslice_ms",
		.data		= &rust_rt_sysctl_sched_rr_timeslice,
		.maxlen		= sizeof(int),
		.mode		= 0644,
		.proc_handler	= sched_rr_handler,
	},
};

void rust_rt_register_sysctl_init(void)
{
	register_sysctl_init("kernel", sched_rt_sysctls);
}
late_initcall(sched_rt_sysctl_init);
#endif /* CONFIG_SYSCTL */

/* Native placement/alignment is controlled by DEFINE_SCHED_CLASS. */
DEFINE_SCHED_CLASS(rt) = {
	.enqueue_task		= enqueue_task_rt,
	.dequeue_task		= dequeue_task_rt,
	.yield_task		= yield_task_rt,
	.wakeup_preempt		= wakeup_preempt_rt,
	.pick_task		= pick_task_rt,
	.put_prev_task		= put_prev_task_rt,
	.set_next_task		= set_next_task_rt,
	.balance		= balance_rt,
	.select_task_rq		= select_task_rq_rt,
	.set_cpus_allowed	= set_cpus_allowed_common,
	.rq_online		= rq_online_rt,
	.rq_offline		= rq_offline_rt,
	.task_woken		= task_woken_rt,
	.switched_from		= switched_from_rt,
	.find_lock_rq		= find_lock_lowest_rq,
	.task_tick		= task_tick_rt,
	.get_rr_interval	= get_rr_interval_rt,
	.switched_to		= switched_to_rt,
	.prio_changed		= prio_changed_rt,
	.update_curr		= rust_rt_update_curr_rt,
#ifdef CONFIG_SCHED_CORE
	.task_is_throttled	= task_is_throttled_rt,
#endif
#ifdef CONFIG_UCLAMP_TASK
	.uclamp_enabled		= 1,
#endif
};
