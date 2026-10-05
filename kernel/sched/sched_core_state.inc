/* SPDX-License-Identifier: GPL-2.0-only */
/* Sole native storage for tail state. Original Rust immutable weight arrays
 * and balance_push_callback keep their existing unique Rust definitions. */
DEFINE_PER_CPU(struct kernel_stat, kstat);
DEFINE_PER_CPU(struct kernel_cpustat, kernel_cpustat) = {
#ifdef CONFIG_NO_HZ_COMMON
	.idle_sleeptime_seq = SEQCNT_ZERO(kernel_cpustat.idle_sleeptime_seq)
#endif
};
EXPORT_PER_CPU_SYMBOL(kstat);
EXPORT_PER_CPU_SYMBOL(kernel_cpustat);
#ifdef CONFIG_NO_HZ_FULL
static struct tick_work __percpu *tick_work_cpu;
#endif
#ifdef CONFIG_SCHED_CORE
static DEFINE_PER_CPU(struct balance_callback, core_balance_head);
#endif
#ifdef CONFIG_HOTPLUG_CPU
static DEFINE_PER_CPU(struct cpu_stop_work, push_work);
#endif
static DEFINE_PER_CPU(struct sched_change_ctx, sched_change_ctx);
bool sched_smp_initialized __read_mostly;
static int num_cpus_frozen;
static bool lupos_core_resched_warned_once_state;
static DEFINE_WAIT_OVERRIDE_MAP(lupos_core_sched_map, LD_WAIT_CONFIG);
#ifdef CONFIG_DEBUG_ATOMIC_SLEEP
static unsigned long lupos_core_might_resched_jiffy;
static unsigned long lupos_core_cant_sleep_jiffy;
#ifdef CONFIG_SMP
static unsigned long lupos_core_cant_migrate_jiffy;
#endif
#endif
#ifdef CONFIG_PREEMPT_DYNAMIC
int preempt_dynamic_mode = preempt_dynamic_undefined;
static DEFINE_MUTEX(sched_dynamic_mutex);
#endif
const char *preempt_modes[] = { "none", "voluntary", "full", "lazy", NULL };
static char lupos_core_preempt_model_buf[128];
#ifdef CONFIG_CGROUP_SCHED
struct task_group root_task_group;
LIST_HEAD(task_groups);
static struct kmem_cache *task_group_cache __ro_after_init;
static DEFINE_SPINLOCK(task_group_lock);
#endif
#ifdef CONFIG_CFS_BANDWIDTH
static DEFINE_MUTEX(cfs_constraints_mutex);
#endif
#ifdef CONFIG_GROUP_SCHED_BANDWIDTH
const u64 max_bw_quota_period_us = 1 * USEC_PER_SEC;
#endif
#ifdef CONFIG_RT_GROUP_SCHED
#ifdef CONFIG_RT_GROUP_SCHED_DEFAULT_DISABLED
DEFINE_STATIC_KEY_FALSE(rt_group_sched);
#else
DEFINE_STATIC_KEY_TRUE(rt_group_sched);
#endif
#endif
