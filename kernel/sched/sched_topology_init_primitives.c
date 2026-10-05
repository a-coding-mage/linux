// SPDX-License-Identifier: GPL-2.0
/* Native initialized metadata, ABI storage and header leaves. Unqualified C. */
#include "sched_topology_bindings.h"
#error "SOURCE ONLY HOLD: scheduler topology initialization is not admitted"
static int default_relax_domain_level = -1;
int sched_domain_level_max;
static int __init setup_relax_domain_level(char *str) { return lupos_topology_setup_relax_domain_level(str); }
__setup("relax_domain_level=", setup_relax_domain_level);
int __init lupos_topology_relax_parse(char *str) { return kstrtoint(str, 0, &default_relax_domain_level); }
int lupos_topology_relax_level(void) { return default_relax_domain_level; }
void __init lupos_topology_relax_warn(void) { pr_warn("Unable to set relax_domain_level\n"); }
struct sched_domain *lupos_topology_init_sd(struct sd_data *sdd, int cpu) { return *per_cpu_ptr(sdd->sd, cpu); }
u64 lupos_topology_init_sched_clock(void) { return sched_clock(); }
unsigned long lupos_topology_init_jiffies(void) { return jiffies; }
void lupos_topology_mask_and(struct cpumask *dst, const struct cpumask *a, const struct cpumask *b) { cpumask_and(dst, a, b); }
const struct cpumask *lupos_topology_level_mask(struct sched_domain_topology_level *tl, int cpu) { return tl->mask(tl, cpu); }
bool lupos_topology_level_has_flags(struct sched_domain_topology_level *tl) { return !!tl->sd_flags; }
int lupos_topology_level_flags(struct sched_domain_topology_level *tl) { return tl->sd_flags(); }
bool lupos_topology_init_warn_flags(bool invalid) { return WARN_ONCE(invalid, "wrong sd_flags in topology description\n"); }
void lupos_topology_init_warn_smt_asym(bool invalid) { WARN_ONCE(invalid, "CPU capacity asymmetry not supported on SMT\n"); }
void lupos_topology_init_domain_zero(struct sched_domain *sd) { *sd = (struct sched_domain){}; }
void lupos_topology_domain_private_set(struct sched_domain *sd, struct sd_data *sdd) { sd->private = sdd; }
struct sd_data *lupos_topology_domain_private(struct sched_domain *sd) { return sd->private; }
#ifdef CONFIG_NUMA
int lupos_topology_init_reclaim_distance(void) { return node_reclaim_distance; }
#endif
#ifdef CONFIG_SCHED_SMT
const struct cpumask *lupos_topology_init_smt_mask(int cpu) { return cpu_smt_mask(cpu); }
#endif
#ifdef CONFIG_SCHED_CLUSTER
const struct cpumask *lupos_topology_init_cluster_mask(int cpu) { return cpu_clustergroup_mask(cpu); }
#endif
#ifdef CONFIG_SCHED_MC
const struct cpumask *lupos_topology_init_core_mask(int cpu) { return cpu_coregroup_mask(cpu); }
#ifndef arch_llc_mask
#define arch_llc_mask(cpu) cpu_coregroup_mask(cpu)
#endif
#else
#define arch_llc_mask(cpu) cpumask_of(cpu)
#endif
const struct cpumask *lupos_topology_llc_mask(int cpu) { return arch_llc_mask(cpu); }
const struct cpumask *lupos_topology_init_node_mask(int cpu) { return cpu_node_mask(cpu); }
static struct sched_domain_topology_level default_topology[] = {
#ifdef CONFIG_SCHED_SMT
    SDTL_INIT(tl_smt_mask, cpu_smt_flags, SMT),
#endif
#ifdef CONFIG_SCHED_CLUSTER
    SDTL_INIT(tl_cls_mask, cpu_cluster_flags, CLS),
#endif
#ifdef CONFIG_SCHED_MC
    SDTL_INIT(tl_mc_mask, cpu_core_flags, MC),
#endif
    SDTL_INIT(tl_pkg_mask, NULL, PKG),
    { NULL, },
};
static struct sched_domain_topology_level *sched_domain_topology = default_topology;
static struct sched_domain_topology_level *sched_domain_topology_saved;
struct sched_domain_topology_level *lupos_topology_numa_get_topology(void) { return sched_domain_topology; }
struct sched_domain_topology_level *lupos_topology_numa_get_saved_topology(void) { return sched_domain_topology_saved; }
void lupos_topology_numa_set_topology(struct sched_domain_topology_level *tl) { sched_domain_topology = tl; }
void lupos_topology_numa_set_saved_topology(struct sched_domain_topology_level *tl) { sched_domain_topology_saved = tl; }
bool __init lupos_topology_init_warn_smp(void) { return WARN_ON_ONCE(sched_smp_initialized); }
void __init set_sched_topology(struct sched_domain_topology_level *tl) { lupos_topology_set_sched_topology(tl); }
