// SPDX-License-Identifier: GPL-2.0
// Runtime macro/header/architecture leaves plus native linker/registration ABI.
// Scheduling owner algorithms are implemented in fair_foundation.rs/fair_tail.rs.
#include "fair_native_bindings.h"
#ifdef CONFIG_FAIR_GROUP_SCHED
DEFINE_STATIC_CALL(rust_fair_calc_group_shares, calc_concur_shares);
#endif
#ifdef CONFIG_CFS_BANDWIDTH
#ifdef CONFIG_JUMP_LABEL
static struct static_key rust_fair_cfs_bandwidth_used;
#endif
#endif
static DEFINE_PER_CPU(cpumask_var_t, rust_fair_load_balance_mask);
static DEFINE_PER_CPU(cpumask_var_t, rust_fair_select_rq_mask);
static DEFINE_PER_CPU(cpumask_var_t, rust_fair_should_we_balance_tmpmask);
static atomic_t rust_fair_sched_balance_running = ATOMIC_INIT(0);
#ifdef CONFIG_NO_HZ_COMMON
static struct rust_fair_nohz_state rust_fair_nohz_storage ____cacheline_aligned;
#endif
DEFINE_STATIC_KEY_FALSE(sched_smt_present);
EXPORT_SYMBOL_GPL(sched_smt_present);
#ifdef CONFIG_FAIR_GROUP_SCHED
static DEFINE_MUTEX(rust_fair_shares_mutex);
#endif
extern void rust_fair_sched_balance_softirq(void);
static __latent_entropy void rust_fair_sched_balance_softirq_entry(void)
{ rust_fair_sched_balance_softirq(); }
#define FAIR_LEAF(ret, name, args, body) ret rust_fair_##name args body
#include "fair_native_primitives.inc"
#undef FAIR_LEAF
/* noinline preserves a distinct native warning instruction/once identity. */
#define FAIR_WARNING(name) noinline bool name(bool condition) { return WARN_ON_ONCE(condition); }
#include "fair_foundation_warnings.inc"
#include "fair_tail_warnings.inc"
#undef FAIR_WARNING
extern int __init setup_sched_thermal_decay_shift(char *str);
__setup("sched_thermal_decay_shift=", setup_sched_thermal_decay_shift);
extern int rust_fair_arch_asym_cpu_priority(int cpu);
int __weak arch_asym_cpu_priority(int cpu) { return rust_fair_arch_asym_cpu_priority(cpu); }
#ifdef CONFIG_SYSCTL
static const struct ctl_table sched_fair_sysctls[] = {
#ifdef CONFIG_CFS_BANDWIDTH
	{
		.procname       = "sched_cfs_bandwidth_slice_us",
		.data           = &sysctl_sched_cfs_bandwidth_slice,
		.maxlen         = sizeof(unsigned int),
		.mode           = 0644,
		.proc_handler   = proc_dointvec_minmax,
		.extra1         = SYSCTL_ONE,
	},
#endif
#ifdef CONFIG_NUMA_BALANCING
	{
		.procname	= "numa_balancing_promote_rate_limit_MBps",
		.data		= &sysctl_numa_balancing_promote_rate_limit,
		.maxlen		= sizeof(unsigned int),
		.mode		= 0644,
		.proc_handler	= proc_dointvec_minmax,
		.extra1		= SYSCTL_ZERO,
	},
#endif /* CONFIG_NUMA_BALANCING */
};
/* register_sysctl_init expands the ARRAY_SIZE-sensitive native macro only. */
void rust_fair_register_sysctl_init(void) { register_sysctl_init("kernel", sched_fair_sysctls); }
extern int __init sched_fair_sysctl_init(void);
late_initcall(sched_fair_sysctl_init);
#endif
#include "fair_class_callbacks.h"
#include "fair_class.inc"
