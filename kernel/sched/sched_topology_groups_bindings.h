/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_TOPOLOGY_GROUPS_BINDINGS_H
#define LUPOS_SCHED_TOPOLOGY_GROUPS_BINDINGS_H

/* Native sched.h owns these types, including all flexible-array layouts. */
#include "sched.h"

struct sd_data *lupos_topology_groups_sd_data(struct sched_domain *sd);
struct sched_domain *lupos_topology_groups_sd(struct sd_data *sdd, int cpu);
struct sched_group *lupos_topology_groups_sg(struct sd_data *sdd, int cpu);
struct sched_group_capacity *lupos_topology_groups_sgc(struct sd_data *sdd, int cpu);
struct sched_group *lupos_topology_groups_alloc(int cpu);
void lupos_topology_groups_ref_inc(struct sched_group *sg);
int lupos_topology_groups_ref_inc_return(struct sched_group *sg);
int lupos_topology_groups_capacity_ref_inc_return(struct sched_group_capacity *sgc);

unsigned int lupos_topology_groups_mask_bits(void);
unsigned int lupos_topology_groups_wrap_first(const struct cpumask *mask, int start);
unsigned int lupos_topology_groups_wrap_next(const struct cpumask *mask, int start, int cpu);
void lupos_topology_groups_mask_set(int cpu, struct cpumask *mask);
void lupos_topology_groups_mask_set_nonatomic(int cpu, struct cpumask *mask);
void lupos_topology_groups_mask_copy(struct cpumask *dst, const struct cpumask *src);
void lupos_topology_groups_mask_andnot(struct cpumask *dst,
				     const struct cpumask *a,
				     const struct cpumask *b);
const struct cpumask *lupos_topology_groups_smt_mask(int cpu);
unsigned int lupos_topology_groups_next_capacity_cpu(int cpu);

bool lupos_topology_groups_asym_prefer(int cpu, int other);
unsigned long lupos_topology_groups_cpu_capacity(int cpu);
void lupos_topology_groups_write_prefer_cpu(struct sched_group *sg, int cpu);
void lupos_topology_groups_rcu_read_lock(void);
void lupos_topology_groups_rcu_read_unlock(void);
struct sched_domain *lupos_topology_groups_cpu_domain(int cpu);

/* Asymmetry-list storage and single native list/container/RCU operations. */
struct list_head *lupos_topology_groups_asym_head(void);
struct asym_cap_data *lupos_topology_groups_asym_entry(struct list_head *link);
struct asym_cap_data *lupos_topology_groups_asym_from_rcu(struct rcu_head *head);
struct cpumask *lupos_topology_groups_capacity_span(struct asym_cap_data *entry);
struct asym_cap_data *lupos_topology_groups_asym_alloc(void);
void lupos_topology_groups_asym_free(struct asym_cap_data *entry);
bool lupos_topology_groups_asym_empty(void);
bool lupos_topology_groups_asym_singular(void);
void lupos_topology_groups_asym_add_tail(struct asym_cap_data *entry);
void lupos_topology_groups_asym_add_after(struct asym_cap_data *entry,
					struct list_head *previous);
void lupos_topology_groups_asym_del(struct asym_cap_data *entry);
void lupos_topology_groups_call_rcu(struct rcu_head *head,
				   void (*func)(struct rcu_head *));

/* One wrapper per original diagnostic site preserves WARN_ONCE scope. */
void lupos_topology_groups_warn_empty_balance(bool empty);
void lupos_topology_groups_warn_balance_mismatch(bool mismatch);
void lupos_topology_groups_warn_refcount_mismatch(bool mismatch);
void lupos_topology_groups_warn_missing_group(bool missing);
void lupos_topology_groups_warn_numa_preference(bool numa);
void lupos_topology_groups_warn_missing_capacity(bool missing);
bool lupos_topology_groups_warn_asym_alloc(bool failed);

#endif
