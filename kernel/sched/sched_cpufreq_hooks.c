// SPDX-License-Identifier: GPL-2.0
/* Native per-CPU, RCU, diagnostics and export ownership only. */
#error "SOURCE ONLY HOLD: scheduler CPUFreq hook native boundary is unqualified"
#include "sched_cpufreq_bindings.h"

#ifdef CONFIG_CPU_FREQ
DEFINE_PER_CPU(struct update_util_data __rcu *, cpufreq_update_util_data);

bool lupos_sched_cpufreq_warn_invalid_hook(bool condition)
{
	return WARN_ON(condition);
}

bool lupos_sched_cpufreq_warn_occupied_hook(bool condition)
{
	return WARN_ON(condition);
}

bool lupos_sched_cpufreq_hook_present(int cpu)
{
	/* Match the native installation-time test of the stored hook. */
	return !!per_cpu(cpufreq_update_util_data, cpu);
}

/* The public C entry retains the native nullable function-pointer ABI.
 * Pass a temporary native callback holder to Rust so no Option<fn> encoding
 * is exported as the C entry ABI. The Rust body owns validation/publication.
 * The holder is read only during this synchronous call.
 */
void cpufreq_add_update_util_hook(int cpu, struct update_util_data *data,
		void (*func)(struct update_util_data *, u64, unsigned int))
{
	const struct update_util_data callback = { .func = func };

	lupos_sched_cpufreq_add_update_util_hook(cpu, data, &callback);
}

bool lupos_sched_cpufreq_callback_missing(const struct update_util_data *callback)
{
	return !callback->func;
}

void lupos_sched_cpufreq_hook_set_func(struct update_util_data *data,
		const struct update_util_data *callback)
{
	data->func = callback->func;
}

void lupos_sched_cpufreq_hook_publish(int cpu, struct update_util_data *data)
{
	rcu_assign_pointer(per_cpu(cpufreq_update_util_data, cpu), data);
}

bool lupos_sched_cpufreq_policy_has_current_cpu(struct cpufreq_policy *policy)
{
	return cpumask_test_cpu(smp_processor_id(), policy->cpus);
}

bool lupos_sched_cpufreq_policy_remote_dvfs(struct cpufreq_policy *policy)
{
	return policy->dvfs_possible_from_any_cpu;
}

struct update_util_data *lupos_sched_cpufreq_this_hook_rcu(void)
{
	return rcu_dereference_sched(*this_cpu_ptr(&cpufreq_update_util_data));
}

EXPORT_SYMBOL_GPL(cpufreq_add_update_util_hook);
EXPORT_SYMBOL_GPL(cpufreq_remove_update_util_hook);
#endif /* CONFIG_CPU_FREQ */
