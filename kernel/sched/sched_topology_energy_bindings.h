/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_TOPOLOGY_ENERGY_BINDINGS_H
#define LUPOS_SCHED_TOPOLOGY_ENERGY_BINDINGS_H
#if defined(CONFIG_ENERGY_MODEL) && defined(CONFIG_CPU_FREQ_GOV_SCHEDUTIL)
struct sched_domain *lupos_topology_energy_asym_access(int cpu);
bool lupos_topology_energy_smt_active(void);
bool lupos_topology_energy_freq_invariant(void);
bool lupos_topology_energy_cpufreq_ready(const struct cpumask *mask);
void lupos_topology_energy_lock(void);
void lupos_topology_energy_unlock(void);
void lupos_topology_energy_set_update(bool update);
void lupos_topology_energy_rebuild_domains(void);
bool lupos_topology_energy_update(void);
unsigned int lupos_topology_energy_requested(void);
bool lupos_topology_energy_enabled(void);
void lupos_topology_energy_disable(void);
void lupos_topology_energy_enable(void);
void lupos_topology_energy_debug_asym(const struct cpumask *mask);
void lupos_topology_energy_debug_smt(const struct cpumask *mask);
void lupos_topology_energy_debug_freq(const struct cpumask *mask);
void lupos_topology_energy_debug_cpufreq(const struct cpumask *mask);
void lupos_topology_energy_debug_no_em(int cpu);
void lupos_topology_energy_debug_root(const struct cpumask *mask);
void lupos_topology_energy_debug_end(void);
void lupos_topology_energy_debug_pd(struct perf_domain *pd);
void lupos_topology_energy_debug_stop(void);
void lupos_topology_energy_debug_start(void);
struct perf_domain *lupos_topology_energy_pd_alloc(void);
struct em_perf_domain *lupos_topology_energy_em_cpu_get(int cpu);
const struct cpumask *lupos_topology_energy_pd_span(struct perf_domain *pd);
struct perf_domain *lupos_topology_energy_pd_from_rcu(struct rcu_head *rcu);
void lupos_topology_energy_pd_assign(struct root_domain *rd, struct perf_domain *pd);
void lupos_topology_energy_pd_clear(struct root_domain *rd);
void lupos_topology_energy_pd_call_rcu(struct perf_domain *pd);
void lupos_topology_destroy_perf_domain_rcu(struct rcu_head *rcu);
#ifdef CONFIG_SYSCTL
bool lupos_topology_energy_admin(void);
int lupos_topology_energy_aware_handler(const struct ctl_table *, int, void *, size_t *, loff_t *);
#endif
#endif
#endif
