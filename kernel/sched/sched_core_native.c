// SPDX-License-Identifier: GPL-2.0-only
/* Configured state/metadata plus explicitly catalogued native runtime leaves.
 * No original core.c scheduling algorithm body is included or forwarded. */
#define LUPOS_CORE_DEFINE_TRACE_POINTS
#include "sched_core_bindings.h"
#undef LUPOS_CORE_DEFINE_TRACE_POINTS
#ifndef prepare_arch_switch
#define prepare_arch_switch(next) do { } while (0)
#endif
#ifndef finish_arch_post_lock_switch
#define finish_arch_post_lock_switch() do { } while (0)
#endif

DEFINE_PER_CPU_SHARED_ALIGNED(struct rq, runqueues);
DEFINE_PER_CPU(struct rnd_state, sched_rnd_state);
#ifdef CONFIG_SCHED_PROXY_EXEC
DEFINE_STATIC_KEY_TRUE(__sched_proxy_exec);
#endif
__setup("sched_proxy_exec", lupos_core_setup_proxy_exec);
#define SCHED_FEAT(name, enabled) (1UL << __SCHED_FEAT_##name) * enabled |
__read_mostly unsigned int sysctl_sched_features =
#include "features.h"
0;
#undef SCHED_FEAT
__read_mostly int sysctl_resched_latency_warn_ms = 100;
__read_mostly int sysctl_resched_latency_warn_once = 1;
__read_mostly unsigned int sysctl_sched_nr_migrate = SCHED_NR_MIGRATE_BREAK;
__read_mostly int scheduler_running;
#ifdef CONFIG_SCHED_CORE
DEFINE_STATIC_KEY_FALSE(__sched_core_enabled);
DEFINE_MUTEX(lupos_core_sched_core_mutex);
atomic_t lupos_core_sched_core_count;
struct cpumask lupos_core_sched_core_mask;
DECLARE_WORK(lupos_core_sched_core_put_work, lupos_core_core_put_workfn);
#endif
#ifdef CONFIG_PARAVIRT
struct static_key paravirt_steal_rq_enabled;
#endif
#ifdef CONFIG_PREEMPT_DYNAMIC
DEFINE_STATIC_KEY_FALSE(lupos_core_sk_dynamic_preempt_lazy);
#endif
#ifdef CONFIG_UCLAMP_TASK
DEFINE_MUTEX(lupos_core_uclamp_mutex);
unsigned int lupos_core_sysctl_sched_uclamp_util_min = SCHED_CAPACITY_SCALE;
unsigned int lupos_core_sysctl_sched_uclamp_util_max = SCHED_CAPACITY_SCALE;
unsigned int sysctl_sched_uclamp_util_min_rt_default = SCHED_CAPACITY_SCALE;
struct uclamp_se lupos_core_uclamp_default[UCLAMP_CNT];
DEFINE_STATIC_KEY_FALSE(sched_uclamp_used);
#endif
DEFINE_STATIC_KEY_FALSE(sched_numa_balancing);
#ifdef CONFIG_NUMA_BALANCING
int sysctl_numa_balancing_mode;
#endif
#ifdef CONFIG_SCHEDSTATS
DEFINE_STATIC_KEY_FALSE(sched_schedstats);
__setup("schedstats=", lupos_core_setup_schedstats);
#endif
#ifdef CONFIG_PREEMPT_NOTIFIERS
DEFINE_STATIC_KEY_FALSE(lupos_core_preempt_notifier_key);
#endif

#ifdef CONFIG_SYSCTL
static const struct ctl_table sched_core_sysctls[] = {
#ifdef CONFIG_SCHEDSTATS
	{
		.procname       = "sched_schedstats",
		.data           = NULL,
		.maxlen         = sizeof(unsigned int),
		.mode           = 0644,
		.proc_handler   = lupos_core_sysctl_schedstats,
		.extra1         = SYSCTL_ZERO,
		.extra2         = SYSCTL_ONE,
	},
#endif /* CONFIG_SCHEDSTATS */
#ifdef CONFIG_UCLAMP_TASK
	{
		.procname       = "sched_util_clamp_min",
		.data           = &lupos_core_sysctl_sched_uclamp_util_min,
		.maxlen         = sizeof(unsigned int),
		.mode           = 0644,
		.proc_handler   = lupos_core_sysctl_sched_uclamp_handler,
	},
	{
		.procname       = "sched_util_clamp_max",
		.data           = &lupos_core_sysctl_sched_uclamp_util_max,
		.maxlen         = sizeof(unsigned int),
		.mode           = 0644,
		.proc_handler   = lupos_core_sysctl_sched_uclamp_handler,
	},
	{
		.procname       = "sched_util_clamp_min_rt_default",
		.data           = &sysctl_sched_uclamp_util_min_rt_default,
		.maxlen         = sizeof(unsigned int),
		.mode           = 0644,
		.proc_handler   = lupos_core_sysctl_sched_uclamp_handler,
	},
#endif /* CONFIG_UCLAMP_TASK */
#ifdef CONFIG_NUMA_BALANCING
	{
		.procname	= "numa_balancing",
		.data		= NULL, /* filled in by handler */
		.maxlen		= sizeof(unsigned int),
		.mode		= 0644,
		.proc_handler	= lupos_core_sysctl_numa_balancing,
		.extra1		= SYSCTL_ZERO,
		.extra2		= SYSCTL_FOUR,
	},
#endif /* CONFIG_NUMA_BALANCING */
};
/* Array-form register_sysctl_init macro needs native array extent. */
void __init lupos_core_register_core_sysctl(void)
{
    register_sysctl_init("kernel", sched_core_sysctls);
}
late_initcall(lupos_core_sched_core_sysctl_init);
#endif

/* Original callback lock class has one stable identity. */
static struct lock_class_key lupos_core_stop_pi_lock;
void lupos_core_lockdep_stop_pi_class(struct task_struct *stop)
{
    lockdep_set_class(&stop->pi_lock, &lupos_core_stop_pi_lock);
}
/* Sparse-only context transfer. Runtime lockdep IP operations remain OPEN. */
void lupos_core_context_transfer_rq(struct rq *rq)
{
    __release(__rq_lockp(rq));
    __acquire(__rq_lockp(this_rq()));
}

#include "sched_core_state.inc"
#include "sched_core_dynamic.inc"
#include "sched_core_registration.inc"

#define LUPOS_CORE_VALUE(type, name, args, ...) type name args { return (__VA_ARGS__); }
#define LUPOS_CORE_VOID(name, args, ...) void name args { __VA_ARGS__; }
#define LUPOS_CORE_BODY(type, name, args, ...) type name args { __VA_ARGS__ }
#include "sched_core_native_leaves.def"
#include "sched_core_header_leaves.def"
#include "sched_core_state_leaves.def"
#include "sched_core_dynamic_leaves.def"
#include "sched_core_diagnostics.def"
#undef LUPOS_CORE_VALUE
#undef LUPOS_CORE_VOID
#undef LUPOS_CORE_BODY

/* The native tracing infrastructure owns tracepoints; callback/body ownership
 * is Rust. This export/initcall table is metadata, unlike the leaves above. */
EXPORT_TRACEPOINT_SYMBOL_GPL(ipi_send_cpu);
EXPORT_TRACEPOINT_SYMBOL_GPL(ipi_send_cpumask);

/*
 * Export tracepoints that act as a bare tracehook (ie: have no trace event
 * associated with them) to allow external modules to probe them.
 */
EXPORT_TRACEPOINT_SYMBOL_GPL(pelt_cfs_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(pelt_rt_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(pelt_dl_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(pelt_irq_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(pelt_se_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(pelt_hw_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(sched_cpu_capacity_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(sched_overutilized_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(sched_util_est_cfs_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(sched_util_est_se_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(sched_update_nr_running_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(sched_compute_energy_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(sched_entry_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(sched_exit_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(sched_set_need_resched_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(sched_dl_throttle_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(sched_dl_replenish_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(sched_dl_update_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(sched_dl_server_start_tp);
EXPORT_TRACEPOINT_SYMBOL_GPL(sched_dl_server_stop_tp);

EXPORT_TRACEPOINT_SYMBOL(sched_set_state_tp);
EXPORT_SYMBOL(__trace_set_current_state);
EXPORT_SYMBOL_GPL(__trace_set_need_resched);
EXPORT_SYMBOL_GPL(___migrate_enable);
EXPORT_SYMBOL_GPL(migrate_disable);
EXPORT_SYMBOL_GPL(migrate_enable);
EXPORT_SYMBOL_GPL(set_cpus_allowed_ptr);
EXPORT_SYMBOL_GPL(kick_process);
EXPORT_SYMBOL(wake_up_process);
#ifdef CONFIG_PREEMPT_NOTIFIERS
EXPORT_SYMBOL_GPL(preempt_notifier_inc);
EXPORT_SYMBOL_GPL(preempt_notifier_dec);
EXPORT_SYMBOL_GPL(preempt_notifier_register);
EXPORT_SYMBOL_GPL(preempt_notifier_unregister);
#endif

#include "sched_core_metadata.inc"
