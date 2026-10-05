// SPDX-License-Identifier: GPL-2.0
/* EAS native storage/metadata/header primitives. Unqualified runtime C. */
#include "sched_topology_bindings.h"
#error "SOURCE ONLY HOLD: scheduler topology energy is not admitted"
#if defined(CONFIG_ENERGY_MODEL) && defined(CONFIG_CPU_FREQ_GOV_SCHEDUTIL)
DEFINE_STATIC_KEY_FALSE(sched_energy_present);
static unsigned int sysctl_sched_energy_aware = 1;
static DEFINE_MUTEX(sched_energy_mutex);
static bool sched_energy_update;
#ifdef CONFIG_SYSCTL
static const struct ctl_table sched_energy_aware_sysctls[] = {
    {
        .procname = "sched_energy_aware",
        .data = &sysctl_sched_energy_aware,
        .maxlen = sizeof(unsigned int),
        .mode = 0644,
        .proc_handler = lupos_topology_energy_aware_handler,
        .extra1 = SYSCTL_ZERO,
        .extra2 = SYSCTL_ONE,
    },
};
static int __init sched_energy_aware_sysctl_init(void)
{
    register_sysctl_init("kernel", sched_energy_aware_sysctls);
    return 0;
}
late_initcall(sched_energy_aware_sysctl_init);
bool lupos_topology_energy_admin(void) { return capable(CAP_SYS_ADMIN); }
#endif
struct sched_domain *lupos_topology_energy_asym_access(int cpu) { return rcu_access_pointer(per_cpu(sd_asym_cpucapacity, cpu)); }
bool lupos_topology_energy_smt_active(void) { return sched_smt_active(); }
bool lupos_topology_energy_freq_invariant(void) { return arch_scale_freq_invariant(); }
bool lupos_topology_energy_cpufreq_ready(const struct cpumask *mask) { return cpufreq_ready_for_eas(mask); }
void lupos_topology_energy_lock(void) { mutex_lock(&sched_energy_mutex); }
void lupos_topology_energy_unlock(void) { mutex_unlock(&sched_energy_mutex); }
void lupos_topology_energy_set_update(bool update) { sched_energy_update = update; }
void lupos_topology_energy_rebuild_domains(void) { rebuild_sched_domains(); }
bool lupos_topology_energy_update(void) { return sched_energy_update; }
unsigned int lupos_topology_energy_requested(void) { return sysctl_sched_energy_aware; }
bool lupos_topology_energy_enabled(void) { return sched_energy_enabled(); }
void lupos_topology_energy_disable(void) { static_branch_disable_cpuslocked(&sched_energy_present); }
void lupos_topology_energy_enable(void) { static_branch_enable_cpuslocked(&sched_energy_present); }
void lupos_topology_energy_debug_asym(const struct cpumask *mask) { pr_info("rd %*pbl: Checking EAS, CPUs do not have asymmetric capacities\n", cpumask_pr_args(mask)); }
void lupos_topology_energy_debug_smt(const struct cpumask *mask) { pr_info("rd %*pbl: Checking EAS, SMT is not supported\n", cpumask_pr_args(mask)); }
void lupos_topology_energy_debug_freq(const struct cpumask *mask) { pr_info("rd %*pbl: Checking EAS: frequency-invariant load tracking not yet supported", cpumask_pr_args(mask)); }
void lupos_topology_energy_debug_cpufreq(const struct cpumask *mask) { pr_info("rd %*pbl: Checking EAS: cpufreq is not ready\n", cpumask_pr_args(mask)); }
void lupos_topology_energy_debug_no_em(int cpu) { pr_info("%s: no EM found for CPU%d\n", "pd_init", cpu); }
void lupos_topology_energy_debug_root(const struct cpumask *mask) { printk(KERN_DEBUG "root_domain %*pbl:", cpumask_pr_args(mask)); }
void lupos_topology_energy_debug_pd(struct perf_domain *pd) { printk(KERN_CONT " pd%d:{ cpus=%*pbl nr_pstate=%d }", cpumask_first(perf_domain_span(pd)), cpumask_pr_args(perf_domain_span(pd)), em_pd_nr_perf_states(pd->em_pd)); }
void lupos_topology_energy_debug_end(void) { printk(KERN_CONT "\n"); }
void lupos_topology_energy_debug_stop(void) { pr_info("%s: stopping EAS\n", "sched_energy_set"); }
void lupos_topology_energy_debug_start(void) { pr_info("%s: starting EAS\n", "sched_energy_set"); }
struct perf_domain *lupos_topology_energy_pd_alloc(void) { return kzalloc_obj(struct perf_domain); }
struct em_perf_domain *lupos_topology_energy_em_cpu_get(int cpu) { return em_cpu_get(cpu); }
const struct cpumask *lupos_topology_energy_pd_span(struct perf_domain *pd) { return perf_domain_span(pd); }
struct perf_domain *lupos_topology_energy_pd_from_rcu(struct rcu_head *rcu) { return container_of(rcu, struct perf_domain, rcu); }
void lupos_topology_energy_pd_assign(struct root_domain *rd, struct perf_domain *pd) { rcu_assign_pointer(rd->pd, pd); }
void lupos_topology_energy_pd_clear(struct root_domain *rd) { rcu_assign_pointer(rd->pd, NULL); }
void lupos_topology_energy_pd_call_rcu(struct perf_domain *pd) { call_rcu(&pd->rcu, lupos_topology_destroy_perf_domain_rcu); }
#endif
