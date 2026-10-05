// SPDX-License-Identifier: GPL-2.0
/* Unqualified native runtime leaves and static storage for cache scheduling. */
#include "sched_topology_bindings.h"
#error "SOURCE ONLY HOLD: scheduler topology cache is not admitted"
#ifdef CONFIG_SCHED_CACHE
DEFINE_STATIC_KEY_FALSE(sched_cache_present);
DEFINE_STATIC_KEY_FALSE(sched_cache_active);
int sysctl_sched_cache_user = 1;
struct cacheinfo *lupos_topology_cache_info(int cpu) { return get_cpu_cacheinfo_llc(cpu); }
u64 lupos_topology_cache_div_u64(u64 dividend, u32 divisor) { return div_u64(dividend, divisor); }
unsigned int *lupos_topology_cache_counts_alloc(int cpu) { return kcalloc_node(max_lid + 1, sizeof(unsigned int), GFP_KERNEL, cpu_to_node(cpu)); }
struct sched_domain *lupos_topology_cache_parent(struct sched_domain *sd) { return rcu_dereference_protected(sd->parent, true); }
void lupos_topology_cache_assert_cpus_held(void) { lockdep_assert_cpus_held(); }
void lupos_topology_cache_cpus_read_lock(void) { cpus_read_lock(); }
void lupos_topology_cache_cpus_read_unlock(void) { cpus_read_unlock(); }
bool lupos_topology_cache_present(void) { return static_branch_likely(&sched_cache_present); }
void lupos_topology_cache_active_disable(void) { static_branch_disable_cpuslocked(&sched_cache_active); }
void lupos_topology_cache_active_enable(void) { static_branch_enable_cpuslocked(&sched_cache_active); }
void lupos_topology_cache_present_disable(void) { static_branch_disable_cpuslocked(&sched_cache_present); }
void lupos_topology_cache_present_enable(void) { static_branch_enable_cpuslocked(&sched_cache_present); }
void lupos_topology_cache_debug_unsupported(void) { pr_info("%s: cache aware scheduling not supported on this platform\n", "_sched_cache_active_set"); }
void lupos_topology_cache_debug_enable(void) { pr_info("%s: enabling cache aware scheduling\n", "_sched_cache_active_set"); }
void lupos_topology_cache_debug_disable(void) { pr_info("%s: disabling cache aware scheduling\n", "_sched_cache_active_set"); }
struct sched_domain *lupos_topology_cache_sd_llc_dereference(unsigned int cpu) { return rcu_dereference_sched_domain(per_cpu(sd_llc, cpu)); }
struct sched_domain *lupos_topology_cache_rq_sd_dereference(unsigned int cpu) { return rcu_dereference_sched_domain(cpu_rq(cpu)->sd); }
#endif
