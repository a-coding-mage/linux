/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_CPUFREQ_BINDINGS_H
#define LUPOS_SCHED_CPUFREQ_BINDINGS_H
#include <uapi/linux/sched/types.h>
#include <linux/slab.h>
#include <linux/energy_model.h>
#include "sched_cpufreq_private.h"

#ifdef CONFIG_CPU_FREQ
bool lupos_sched_cpufreq_warn_invalid_hook(bool condition);
bool lupos_sched_cpufreq_warn_occupied_hook(bool condition);
bool lupos_sched_cpufreq_hook_present(int cpu);
void lupos_sched_cpufreq_add_update_util_hook(int cpu, struct update_util_data *data,
		const struct update_util_data *callback);
bool lupos_sched_cpufreq_callback_missing(const struct update_util_data *callback);
void lupos_sched_cpufreq_hook_set_func(struct update_util_data *data,
		const struct update_util_data *callback);
void lupos_sched_cpufreq_hook_publish(int cpu, struct update_util_data *data);
bool lupos_sched_cpufreq_policy_has_current_cpu(struct cpufreq_policy *policy);
bool lupos_sched_cpufreq_policy_remote_dvfs(struct cpufreq_policy *policy);
struct update_util_data *lupos_sched_cpufreq_this_hook_rcu(void);
#endif

#ifdef CONFIG_CPU_FREQ_GOV_SCHEDUTIL
static const unsigned int LUPOS_SUGOV_IOWAIT_BOOST_MIN = SCHED_CAPACITY_SCALE / 8;
static const unsigned int LUPOS_SUGOV_CAPACITY_SCALE = SCHED_CAPACITY_SCALE;
static const unsigned int LUPOS_SUGOV_CAPACITY_SHIFT = SCHED_CAPACITY_SHIFT;
static const unsigned int LUPOS_SUGOV_IOWAIT = SCHED_CPUFREQ_IOWAIT;
static const s64 LUPOS_SUGOV_NSEC_PER_USEC = NSEC_PER_USEC;
static const s64 LUPOS_SUGOV_TICK_NSEC = TICK_NSEC;
static const u16 LUPOS_SUGOV_NEED_UPDATE_LIMITS = CPUFREQ_NEED_UPDATE_LIMITS;
static const unsigned int LUPOS_SUGOV_RELATION_L = CPUFREQ_RELATION_L;
static const int LUPOS_SUGOV_EINVAL = EINVAL;
static const int LUPOS_SUGOV_EBUSY = EBUSY;
static const int LUPOS_SUGOV_ENOMEM = ENOMEM;

/* Callback declarations retain the native ABI for registration and CFI. */
int lupos_sugov_init(struct cpufreq_policy *policy);
void lupos_sugov_exit(struct cpufreq_policy *policy);
int lupos_sugov_start(struct cpufreq_policy *policy);
void lupos_sugov_stop(struct cpufreq_policy *policy);
void lupos_sugov_limits(struct cpufreq_policy *policy);
void lupos_sugov_work(struct kthread_work *work);
void lupos_sugov_irq_work(struct irq_work *work);
void lupos_sugov_update_shared(struct update_util_data *hook, u64 time, unsigned int flags);
void lupos_sugov_update_single_perf(struct update_util_data *hook, u64 time, unsigned int flags);
void lupos_sugov_update_single_freq(struct update_util_data *hook, u64 time, unsigned int flags);
ssize_t lupos_sugov_rate_limit_us_show(struct gov_attr_set *attr_set, char *buf);
ssize_t lupos_sugov_rate_limit_us_store(struct gov_attr_set *attr_set,
				     const char *buf, size_t count);
void lupos_sugov_tunables_release(struct kobject *kobj);

#define SUGOV_LEAF(ret, name, args, ...) ret lupos_sugov_##name args;
#include "sched_cpufreq_leaves.inc"
#undef SUGOV_LEAF
#endif /* CONFIG_CPU_FREQ_GOV_SCHEDUTIL */
#endif
