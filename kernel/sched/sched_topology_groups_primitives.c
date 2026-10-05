// SPDX-License-Identifier: GPL-2.0
/*
 * Native storage and single-operation leaves for the existing topology.rs
 * group-construction family, pinned to 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
 * Every leaf is an unqualified runtime C boundary, not Rust algorithm coverage.
 * Original compiler/instrumentation context: build_utility.c.
 * No topology.c algorithm body is included or used as a fallback.
 */
#include "sched_topology_bindings.h"
#include "sched_topology_groups_bindings.h"

#error "SOURCE ONLY HOLD: scheduler topology groups are not admitted"

/* Original public storage, also read by fair.c under RCU. */
LIST_HEAD(asym_cap_list);

struct sd_data *lupos_topology_groups_sd_data(struct sched_domain *sd)
{
	return sd->private;
}

struct sched_domain *lupos_topology_groups_sd(struct sd_data *sdd, int cpu)
{
	return *per_cpu_ptr(sdd->sd, cpu);
}

struct sched_group *lupos_topology_groups_sg(struct sd_data *sdd, int cpu)
{
	return *per_cpu_ptr(sdd->sg, cpu);
}

struct sched_group_capacity *lupos_topology_groups_sgc(struct sd_data *sdd, int cpu)
{
	return *per_cpu_ptr(sdd->sgc, cpu);
}

struct sched_group *lupos_topology_groups_alloc(int cpu)
{
	return kzalloc_node(sizeof(struct sched_group) + cpumask_size(),
			    GFP_KERNEL, cpu_to_node(cpu));
}

void lupos_topology_groups_ref_inc(struct sched_group *sg)
{
	atomic_inc(&sg->ref);
}

int lupos_topology_groups_ref_inc_return(struct sched_group *sg)
{
	return atomic_inc_return(&sg->ref);
}

int lupos_topology_groups_capacity_ref_inc_return(struct sched_group_capacity *sgc)
{
	return atomic_inc_return(&sgc->ref);
}

unsigned int lupos_topology_groups_mask_bits(void)
{
	return small_cpumask_bits;
}

unsigned int lupos_topology_groups_wrap_first(const struct cpumask *mask, int start)
{
	return find_next_bit_wrap(cpumask_bits(mask), small_cpumask_bits, start);
}

unsigned int lupos_topology_groups_wrap_next(const struct cpumask *mask,
					    int start, int cpu)
{
	return __for_each_wrap(cpumask_bits(mask), small_cpumask_bits,
			       start, cpu + 1);
}

void lupos_topology_groups_mask_set(int cpu, struct cpumask *mask)
{
	cpumask_set_cpu(cpu, mask);
}

void lupos_topology_groups_mask_set_nonatomic(int cpu, struct cpumask *mask)
{
	__cpumask_set_cpu(cpu, mask);
}

void lupos_topology_groups_mask_copy(struct cpumask *dst, const struct cpumask *src)
{
	cpumask_copy(dst, src);
}

void lupos_topology_groups_mask_andnot(struct cpumask *dst,
				     const struct cpumask *a,
				     const struct cpumask *b)
{
	cpumask_andnot(dst, a, b);
}

const struct cpumask *lupos_topology_groups_smt_mask(int cpu)
{
	return cpu_smt_mask(cpu);
}

unsigned int lupos_topology_groups_next_capacity_cpu(int cpu)
{
	return cpumask_next_and(cpu, cpu_possible_mask,
				housekeeping_cpumask(HK_TYPE_DOMAIN));
}

bool lupos_topology_groups_asym_prefer(int cpu, int other)
{
	return sched_asym_prefer(cpu, other);
}

unsigned long lupos_topology_groups_cpu_capacity(int cpu)
{
	return arch_scale_cpu_capacity(cpu);
}

void lupos_topology_groups_write_prefer_cpu(struct sched_group *sg, int cpu)
{
	WRITE_ONCE(sg->asym_prefer_cpu, cpu);
}

void lupos_topology_groups_rcu_read_lock(void)
{
	rcu_read_lock();
}

void lupos_topology_groups_rcu_read_unlock(void)
{
	rcu_read_unlock();
}

struct sched_domain *lupos_topology_groups_cpu_domain(int cpu)
{
	return rcu_dereference_sched_domain(cpu_rq(cpu)->sd);
}

struct list_head *lupos_topology_groups_asym_head(void)
{
	return &asym_cap_list;
}

struct asym_cap_data *lupos_topology_groups_asym_entry(struct list_head *link)
{
	return list_entry(link, struct asym_cap_data, link);
}

struct asym_cap_data *lupos_topology_groups_asym_from_rcu(struct rcu_head *head)
{
	return container_of(head, struct asym_cap_data, rcu);
}

struct cpumask *lupos_topology_groups_capacity_span(struct asym_cap_data *entry)
{
	return cpu_capacity_span(entry);
}

struct asym_cap_data *lupos_topology_groups_asym_alloc(void)
{
	return kzalloc(sizeof(struct asym_cap_data) + cpumask_size(), GFP_KERNEL);
}

void lupos_topology_groups_asym_free(struct asym_cap_data *entry)
{
	kfree(entry);
}

bool lupos_topology_groups_asym_empty(void)
{
	return list_empty(&asym_cap_list);
}

bool lupos_topology_groups_asym_singular(void)
{
	return list_is_singular(&asym_cap_list);
}

void lupos_topology_groups_asym_add_tail(struct asym_cap_data *entry)
{
	list_add_tail_rcu(&entry->link, &asym_cap_list);
}

void lupos_topology_groups_asym_add_after(struct asym_cap_data *entry,
					struct list_head *previous)
{
	list_add_rcu(&entry->link, previous);
}

void lupos_topology_groups_asym_del(struct asym_cap_data *entry)
{
	list_del_rcu(&entry->link);
}

void lupos_topology_groups_call_rcu(struct rcu_head *head,
				   void (*func)(struct rcu_head *))
{
	call_rcu(head, func);
}

void lupos_topology_groups_warn_empty_balance(bool empty)
{
	WARN_ON_ONCE(empty);
}

void lupos_topology_groups_warn_balance_mismatch(bool mismatch)
{
	WARN_ON_ONCE(mismatch);
}

void lupos_topology_groups_warn_refcount_mismatch(bool mismatch)
{
	WARN_ON(mismatch);
}

void lupos_topology_groups_warn_missing_group(bool missing)
{
	WARN_ON(missing);
}

void lupos_topology_groups_warn_numa_preference(bool numa)
{
	WARN_ON_ONCE(numa);
}

void lupos_topology_groups_warn_missing_capacity(bool missing)
{
	WARN_ON_ONCE(missing);
}

bool lupos_topology_groups_warn_asym_alloc(bool failed)
{
	return WARN_ONCE(failed, "Failed to allocate memory for asymmetry data\n");
}
