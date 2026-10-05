/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_CPUFREQ_PRIVATE_H
#define LUPOS_SCHED_CPUFREQ_PRIVATE_H
/* File-local storage layouts retained from cpufreq_schedutil.c at
 * 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Only configured native headers
 * determine embedded type layout. Rust imports generated types, never replicas.
 */
#include "sched.h"
#ifdef CONFIG_CPU_FREQ_GOV_SCHEDUTIL
struct sugov_tunables {
	struct gov_attr_set attr_set;
	unsigned int rate_limit_us;
};
struct sugov_policy {
	struct cpufreq_policy *policy;
	struct sugov_tunables *tunables;
	struct list_head tunables_hook;
	raw_spinlock_t update_lock;
	u64 last_freq_update_time;
	s64 freq_update_delay_ns;
	unsigned int next_freq;
	unsigned int cached_raw_freq;
	struct irq_work irq_work;
	struct kthread_work work;
	struct mutex work_lock;
	struct kthread_worker worker;
	struct task_struct *thread;
	bool work_in_progress;
	bool limits_changed;
	bool need_freq_update;
};
struct sugov_cpu {
	struct update_util_data update_util;
	struct sugov_policy *sg_policy;
	unsigned int cpu;
	bool iowait_boost_pending;
	unsigned int iowait_boost;
	u64 last_update;
	unsigned long util;
	unsigned long bw_min;
	unsigned long bw_max;
#ifdef CONFIG_NO_HZ_COMMON
	unsigned long saved_idle_calls;
#endif
};
#endif /* CONFIG_CPU_FREQ_GOV_SCHEDUTIL */
#endif
