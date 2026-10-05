/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_TOPOLOGY_BINDINGS_H
#define LUPOS_SCHED_TOPOLOGY_BINDINGS_H
/* Configured native headers own all layouts, enum values and constants. */
#include <linux/sched/clock.h>
#include <linux/sched/isolation.h>
#include <linux/bsearch.h>
#include <linux/cpufreq.h>
#include <linux/cpumask_api.h>
#include <linux/cpuset.h>
#include <linux/energy_model.h>
#include <linux/irq.h>
#include <linux/mempolicy.h>
#include <linux/slab.h>
#include <linux/spinlock_api.h>
#include "sched.h"
#include <linux/cacheinfo.h>
#include <linux/cpuhplock.h>

/* Original topology.c private native layouts, not Rust replicas. */
struct s_data {
    struct sched_domain_shared * __percpu *sds;
    struct sched_domain * __percpu *sd;
    struct root_domain *rd;
};
enum s_alloc { sa_rootdomain, sa_sd, sa_sd_shared, sa_sd_storage, sa_none };
extern int max_lid;
struct sched_domain *lupos_topology_data_sd(struct s_data *d, int cpu);

#define LUPOS_TOPOLOGY_SD_FLAG_COUNT __SD_FLAG_CNT
#define LUPOS_TOPOLOGY_SDF_SHARED_CHILD SDF_SHARED_CHILD
#define LUPOS_TOPOLOGY_SDF_SHARED_PARENT SDF_SHARED_PARENT
#define LUPOS_TOPOLOGY_CAPACITY_SCALE SCHED_CAPACITY_SCALE
#define LUPOS_TOPOLOGY_ENOMEM ENOMEM
#define LUPOS_TOPOLOGY_EPERM EPERM
#define LUPOS_TOPOLOGY_EOPNOTSUPP EOPNOTSUPP

void lupos_topology_domains_lock(void);
void lupos_topology_domains_unlock(void);
struct cpumask *lupos_topology_tmpmask(void);
struct cpumask *lupos_topology_tmpmask2(void);
struct cpumask *lupos_topology_domain_span(struct sched_domain *sd);
struct cpumask *lupos_topology_group_span(struct sched_group *sg);
struct cpumask *lupos_topology_balance_mask(struct sched_group *sg);
void lupos_topology_mask_clear(struct cpumask *mask);
bool lupos_topology_mask_test(int cpu, const struct cpumask *mask);
bool lupos_topology_mask_empty(const struct cpumask *mask);
bool lupos_topology_mask_equal(const struct cpumask *a, const struct cpumask *b);
bool lupos_topology_mask_subset(const struct cpumask *a, const struct cpumask *b);
bool lupos_topology_mask_intersects(const struct cpumask *a, const struct cpumask *b);
void lupos_topology_mask_or(struct cpumask *dst, const struct cpumask *a, const struct cpumask *b);
unsigned int lupos_topology_mask_weight(const struct cpumask *mask);
unsigned int lupos_topology_mask_first(const struct cpumask *mask);
unsigned int lupos_topology_mask_next(int cpu, const struct cpumask *mask);
unsigned int lupos_topology_nr_cpu_ids(void);
unsigned int lupos_topology_mask_bound(void);
unsigned int lupos_topology_flag_meta(unsigned int index);
unsigned int lupos_topology_degenerate_groups_mask(void);
void lupos_topology_debug_domain(struct sched_domain *sd, int level);
void lupos_topology_debug_missing_cpu(int cpu);
void lupos_topology_debug_missing_group_cpu(int cpu);
void lupos_topology_debug_flag_child(unsigned int idx);
void lupos_topology_debug_flag_parent(unsigned int idx);
void lupos_topology_debug_groups(int level);
void lupos_topology_debug_null_group(void);
void lupos_topology_debug_empty_group(void);
void lupos_topology_debug_repeated_cpu(void);
void lupos_topology_debug_group_child_mismatch(void);
void lupos_topology_debug_group_span_mismatch(void);
void lupos_topology_debug_parent_span_mismatch(void);
void lupos_topology_debug_group_end(void);
void lupos_topology_debug_group_separator(void);
void lupos_topology_debug_groups_end(void);
void lupos_topology_debug_group(struct sched_group *group);
void lupos_topology_debug_balance_mask(struct sched_group *group);
void lupos_topology_debug_capacity(unsigned long capacity);
void lupos_topology_debug_attach_null(int cpu);
void lupos_topology_debug_attach(int cpu);
int __init lupos_topology_debug_setup(char *str);
const struct cpumask *lupos_topology_active_mask(void);
struct rq *lupos_topology_cpu_rq(int cpu);
void lupos_topology_free(const void *ptr);
void lupos_topology_assert_domains_locked(void);
#include "sched_topology_energy_bindings.h"
#include "sched_topology_root_bindings.h"
#include "sched_topology_cache_bindings.h"
#include "sched_topology_init_bindings.h"
#include "sched_topology_numa_bindings.h"
#include "sched_topology_groups_bindings.h"
struct cpumask *lupos_topology_llc_id_allocmask(void);
bool __init lupos_topology_init_llc_id_allocmask(void);
bool __init lupos_topology_init_tmpmask(void);
bool __init lupos_topology_init_tmpmask2(void);
#include "sched_topology_build_bindings.h"
#endif
