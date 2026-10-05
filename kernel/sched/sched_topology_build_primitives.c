// SPDX-License-Identifier: GPL-2.0
/* Native allocation/storage/header leaves for retained topology build policy.
 * Baseline 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. All leaves are explicit,
 * unqualified runtime C. No original topology.c owner fallback is linked. */
#include "sched_topology_bindings.h"
#include "sched_topology_build_bindings.h"
#error "SOURCE ONLY HOLD: scheduler topology build is not admitted"

void lupos_topology_build_data_zero(struct s_data *d) { memset(d, 0, sizeof(*d)); }
bool lupos_topology_build_data_sd_alloc(struct s_data *d) { d->sd = alloc_percpu(struct sched_domain *); return d->sd != NULL; }
void lupos_topology_build_data_sd_free(struct s_data *d) { free_percpu(d->sd); }
void lupos_topology_build_data_sd_set(struct s_data *d, int cpu, struct sched_domain *sd) { *per_cpu_ptr(d->sd, cpu) = sd; }
bool lupos_topology_build_data_sds_alloc(struct s_data *d) { d->sds = alloc_percpu(struct sched_domain_shared *); return d->sds != NULL; }
void lupos_topology_build_data_sds_free(struct s_data *d) { free_percpu(d->sds); d->sds = NULL; }
struct sched_domain_shared *lupos_topology_build_data_sds(struct s_data *d, int cpu) { return *per_cpu_ptr(d->sds, cpu); }
void lupos_topology_build_data_sds_set(struct s_data *d, int cpu, struct sched_domain_shared *sds) { *per_cpu_ptr(d->sds, cpu) = sds; }
bool lupos_topology_build_sdd_sd_alloc(struct sd_data *sdd) { sdd->sd = alloc_percpu(struct sched_domain *); return sdd->sd != NULL; }
bool lupos_topology_build_sdd_sg_alloc(struct sd_data *sdd) { sdd->sg = alloc_percpu(struct sched_group *); return sdd->sg != NULL; }
bool lupos_topology_build_sdd_sgc_alloc(struct sd_data *sdd) { sdd->sgc = alloc_percpu(struct sched_group_capacity *); return sdd->sgc != NULL; }
void lupos_topology_build_sdd_sd_free(struct sd_data *sdd) { free_percpu(sdd->sd); sdd->sd = NULL; }
void lupos_topology_build_sdd_sg_free(struct sd_data *sdd) { free_percpu(sdd->sg); sdd->sg = NULL; }
void lupos_topology_build_sdd_sgc_free(struct sd_data *sdd) { free_percpu(sdd->sgc); sdd->sgc = NULL; }
void lupos_topology_build_sdd_sd_set(struct sd_data *sdd, int cpu, struct sched_domain *sd) { *per_cpu_ptr(sdd->sd, cpu) = sd; }
void lupos_topology_build_sdd_sg_set(struct sd_data *sdd, int cpu, struct sched_group *sg) { *per_cpu_ptr(sdd->sg, cpu) = sg; }
void lupos_topology_build_sdd_sgc_set(struct sd_data *sdd, int cpu, struct sched_group_capacity *sgc) { *per_cpu_ptr(sdd->sgc, cpu) = sgc; }
struct sched_domain *lupos_topology_build_sd_alloc(int cpu) { return kzalloc_node(sizeof(struct sched_domain) + cpumask_size(), GFP_KERNEL, cpu_to_node(cpu)); }
struct sched_group *lupos_topology_build_sg_alloc(int cpu) { return kzalloc_node(sizeof(struct sched_group) + cpumask_size(), GFP_KERNEL, cpu_to_node(cpu)); }
struct sched_group_capacity *lupos_topology_build_sgc_alloc(int cpu) { return kzalloc_node(sizeof(struct sched_group_capacity) + cpumask_size(), GFP_KERNEL, cpu_to_node(cpu)); }
struct sched_domain_shared *lupos_topology_build_sds_alloc(int cpu) { return kzalloc_node(sizeof(struct sched_domain_shared), GFP_KERNEL, cpu_to_node(cpu)); }
atomic_t *lupos_topology_build_sds_ref(struct sched_domain_shared *sds) { return &sds->ref; }
atomic_t *lupos_topology_build_sg_ref(struct sched_group *sg) { return &sg->ref; }
atomic_t *lupos_topology_build_sgc_ref(struct sched_group_capacity *sgc) { return &sgc->ref; }
int lupos_topology_build_alloc_flags(struct sched_domain_shared *sds) { return sds->alloc_flags; }
void lupos_topology_build_alloc_flags_set(struct sched_domain_shared *sds, int flags) { sds->alloc_flags = flags; }
int lupos_topology_build_llc_id(int cpu) { return per_cpu(sd_llc_id, cpu); }
void lupos_topology_build_llc_id_set(int cpu, int lid) { per_cpu(sd_llc_id, cpu) = lid; }
unsigned int lupos_topology_build_mask_first_zero(const struct cpumask *mask) { return cpumask_first_zero(mask); }
unsigned int lupos_topology_build_mask_last(const struct cpumask *mask) { return cpumask_last(mask); }
unsigned int lupos_topology_build_mask_any(const struct cpumask *mask) { return cpumask_any(mask); }
unsigned int lupos_topology_build_nr_cpumask_bits(void) { return nr_cpumask_bits; }
void lupos_topology_build_mask_set_nonatomic(int cpu, struct cpumask *mask) { __cpumask_set_cpu(cpu, mask); }
void lupos_topology_build_mask_clear_nonatomic(int cpu, struct cpumask *mask) { __cpumask_clear_cpu(cpu, mask); }
void lupos_topology_build_asym_inc(void) { static_branch_inc_cpuslocked(&sched_asym_cpucapacity); }
void lupos_topology_build_asym_dec(void) { static_branch_dec_cpuslocked(&sched_asym_cpucapacity); }
void lupos_topology_build_cluster_inc(void) { static_branch_inc_cpuslocked(&sched_cluster_active); }
void lupos_topology_build_cluster_dec(void) { static_branch_dec_cpuslocked(&sched_cluster_active); }
bool lupos_topology_build_cluster_active(void) { return static_branch_unlikely(&sched_cluster_active); }
bool lupos_topology_build_asym_present(int cpu) { return rcu_access_pointer(per_cpu(sd_asym_cpucapacity, cpu)) != NULL; }
struct sched_domain *lupos_topology_build_cpu_domain(int cpu) { return rcu_dereference(cpu_rq(cpu)->sd); }
const struct cpumask *lupos_topology_build_housekeeping_mask(void) { return housekeeping_cpumask(HK_TYPE_DOMAIN); }

/* These are the original partition storage objects, each owned exactly once. */
static cpumask_var_t *doms_cur;
static int ndoms_cur;
static struct sched_domain_attr *dattr_cur;
static cpumask_var_t fallback_doms;
cpumask_var_t *lupos_topology_build_masks_alloc(unsigned int ndoms) { return kmalloc_objs(cpumask_var_t, ndoms); }
bool lupos_topology_build_mask_alloc(cpumask_var_t *doms, unsigned int i) { return alloc_cpumask_var(&doms[i], GFP_KERNEL); }
void lupos_topology_build_mask_free(cpumask_var_t *doms, unsigned int i) { free_cpumask_var(doms[i]); }
struct cpumask *lupos_topology_build_mask(cpumask_var_t *doms, unsigned int i) { return doms[i]; }
cpumask_var_t *lupos_topology_build_doms_cur(void) { return doms_cur; }
void lupos_topology_build_doms_cur_set(cpumask_var_t *doms) { doms_cur = doms; }
int lupos_topology_build_ndoms_cur(void) { return ndoms_cur; }
void lupos_topology_build_ndoms_cur_set(int ndoms) { ndoms_cur = ndoms; }
struct sched_domain_attr *lupos_topology_build_dattr_cur(void) { return dattr_cur; }
void lupos_topology_build_dattr_cur_set(struct sched_domain_attr *attr) { dattr_cur = attr; }
cpumask_var_t *lupos_topology_build_fallback(void) { return &fallback_doms; }
bool __init lupos_topology_build_fallback_alloc(void) { return zalloc_cpumask_var(&fallback_doms, GFP_KERNEL); }
void lupos_topology_build_attr_init(struct sched_domain_attr *attr) { *attr = SD_ATTR_INIT; }
int lupos_topology_build_attr_compare(const struct sched_domain_attr *a, const struct sched_domain_attr *b) { return memcmp(a, b, sizeof(struct sched_domain_attr)); }
/* Exact original weak architecture hook, not a substitute for an arch override. */
int __weak arch_update_cpu_topology(void) { return 0; }
int lupos_topology_build_arch_update(void) { return arch_update_cpu_topology(); }
void lupos_topology_build_update_debugfs(void) { update_sched_domain_debugfs(); }
void lupos_topology_build_rebuild_dl(void) { dl_rebuild_rd_accounting(); }
int __init sched_init_domains(const struct cpumask *cpu_map) { return lupos_topology_build_init_domains(cpu_map); }

void lupos_topology_build_warn_claim(bool mismatch) { WARN_ON_ONCE(mismatch); }
void lupos_topology_build_warn_llc(bool invalid) { WARN_ON(invalid); }
void lupos_topology_build_warn_parent(bool missing) { WARN_ON(missing); }
bool lupos_topology_build_warn_shared(bool missing) { return WARN_ON_ONCE(missing); }
bool lupos_topology_build_warn_empty(bool empty) { return WARN_ON(empty); }
bool lupos_topology_build_warn_span(bool invalid) { return WARN_ON(invalid); }
void lupos_topology_build_warn_attrs(bool present) { WARN_ON_ONCE(present); }
void lupos_topology_build_debug_broken(struct sched_domain *child, struct sched_domain *sd)
{
    pr_err("BUG: arch topology borken\n");
    pr_err("     the %s domain not a subset of the %s domain\n", child->name, sd->name);
}
void lupos_topology_build_debug_root(const struct cpumask *cpu_map) { pr_info("root domain span: %*pbl\n", cpumask_pr_args(cpu_map)); }
