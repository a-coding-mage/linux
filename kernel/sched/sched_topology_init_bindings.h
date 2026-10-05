/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_TOPOLOGY_INIT_BINDINGS_H
#define LUPOS_SCHED_TOPOLOGY_INIT_BINDINGS_H
#define LUPOS_TOPOLOGY_ALLOWED_FLAGS (SD_SHARE_CPUCAPACITY | SD_CLUSTER | SD_SHARE_LLC | SD_NUMA | SD_ASYM_PACKING)
extern int sched_domain_level_max;
int __init lupos_topology_setup_relax_domain_level(char *str);
int __init lupos_topology_relax_parse(char *str);
int lupos_topology_relax_level(void);
void __init lupos_topology_relax_warn(void);
struct sched_domain *lupos_topology_init_sd(struct sd_data *sdd, int cpu);
u64 lupos_topology_init_sched_clock(void);
unsigned long lupos_topology_init_jiffies(void);
void lupos_topology_mask_and(struct cpumask *dst, const struct cpumask *a, const struct cpumask *b);
const struct cpumask *lupos_topology_level_mask(struct sched_domain_topology_level *tl, int cpu);
bool lupos_topology_level_has_flags(struct sched_domain_topology_level *tl);
int lupos_topology_level_flags(struct sched_domain_topology_level *tl);
bool lupos_topology_init_warn_flags(bool invalid);
void lupos_topology_init_warn_smt_asym(bool invalid);
void lupos_topology_init_domain_zero(struct sched_domain *sd);
void lupos_topology_domain_private_set(struct sched_domain *sd, struct sd_data *sdd);
struct sd_data *lupos_topology_domain_private(struct sched_domain *sd);
#ifdef CONFIG_NUMA
int lupos_topology_init_reclaim_distance(void);
#endif
#ifdef CONFIG_SCHED_SMT
const struct cpumask *lupos_topology_init_smt_mask(int cpu);
#endif
#ifdef CONFIG_SCHED_CLUSTER
const struct cpumask *lupos_topology_init_cluster_mask(int cpu);
#endif
#ifdef CONFIG_SCHED_MC
const struct cpumask *lupos_topology_init_core_mask(int cpu);
#endif
const struct cpumask *lupos_topology_init_node_mask(int cpu);
const struct cpumask *lupos_topology_llc_mask(int cpu);
struct sched_domain_topology_level *lupos_topology_numa_get_topology(void);
struct sched_domain_topology_level *lupos_topology_numa_get_saved_topology(void);
void lupos_topology_numa_set_topology(struct sched_domain_topology_level *tl);
void lupos_topology_numa_set_saved_topology(struct sched_domain_topology_level *tl);
bool __init lupos_topology_init_warn_smp(void);
void __init lupos_topology_set_sched_topology(struct sched_domain_topology_level *tl);
#endif
