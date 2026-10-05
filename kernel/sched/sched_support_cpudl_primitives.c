// SPDX-License-Identifier: GPL-2.0-only
/*
 * Configured primitive leaves for cpudeadline.rs. Source proposal only.
 * Policy origin: build_policy.c. No cpudeadline.c algorithm is included here.
 */
#include "sched_support_bindings.h"

bool lupos_support_dl_time_before(u64 a, u64 b)
{
	return dl_time_before(a, b);
}

struct cpumask *lupos_support_dl_free_cpus(struct cpudl *cp)
{
	return cp->free_cpus;
}

bool lupos_support_dl_alloc_mask(struct cpudl *cp)
{
	return zalloc_cpumask_var(&cp->free_cpus, GFP_KERNEL);
}

void lupos_support_dl_free_mask(struct cpudl *cp)
{
	free_cpumask_var(cp->free_cpus);
}

struct cpudl_item *lupos_support_dl_alloc_elements(void)
{
	return kzalloc_objs(struct cpudl_item, nr_cpu_ids);
}

bool lupos_support_dl_mask_and(struct cpumask *dst,
			     const struct cpumask *a, const struct cpumask *b)
{
	return cpumask_and(dst, a, b);
}

bool lupos_support_dl_mask_empty(const struct cpumask *mask)
{
	return cpumask_empty(mask);
}

bool lupos_support_dl_mask_test(int cpu, const struct cpumask *mask)
{
	return cpumask_test_cpu(cpu, mask);
}

void lupos_support_dl_mask_set(int cpu, struct cpumask *mask)
{
	cpumask_set_cpu(cpu, mask);
}

void lupos_support_dl_mask_clear(int cpu, struct cpumask *mask)
{
	cpumask_clear_cpu(cpu, mask);
}

void lupos_support_dl_mask_set_private(int cpu, struct cpumask *mask)
{
	__cpumask_set_cpu(cpu, mask);
}

void lupos_support_dl_mask_clear_private(int cpu, struct cpumask *mask)
{
	__cpumask_clear_cpu(cpu, mask);
}

/* Distinct native diagnostic sites preserve each original occurrence. */
void lupos_support_dl_warn_find_cpu(int best_cpu)
{
	WARN_ON(best_cpu != -1 && !cpu_present(best_cpu));
}

void lupos_support_dl_warn_clear_cpu(int cpu)
{
	WARN_ON(!cpu_present(cpu));
}

void lupos_support_dl_warn_set_cpu(int cpu)
{
	WARN_ON(!cpu_present(cpu));
}

void lupos_support_dl_lock_init(struct cpudl *cp)
{
	raw_spin_lock_init(&cp->lock);
}

unsigned long lupos_support_dl_clear_lock(struct cpudl *cp)
{
	unsigned long flags;

	raw_spin_lock_irqsave(&cp->lock, flags);
	return flags;
}

void lupos_support_dl_clear_unlock(struct cpudl *cp, unsigned long flags)
{
	raw_spin_unlock_irqrestore(&cp->lock, flags);
}

unsigned long lupos_support_dl_set_lock(struct cpudl *cp)
{
	unsigned long flags;

	raw_spin_lock_irqsave(&cp->lock, flags);
	return flags;
}

void lupos_support_dl_set_unlock(struct cpudl *cp, unsigned long flags)
{
	raw_spin_unlock_irqrestore(&cp->lock, flags);
}

bool lupos_support_dl_asym_active(void)
{
	return sched_asym_cpucap_active();
}

bool lupos_support_dl_fits_capacity(struct task_struct *p, int cpu)
{
	return dl_task_fits_capacity(p, cpu);
}

unsigned long lupos_support_dl_cpu_capacity(int cpu)
{
	return arch_scale_cpu_capacity(cpu);
}

unsigned int lupos_support_dl_task_cpu(const struct task_struct *p)
{
	return task_cpu(p);
}

int lupos_support_dl_mask_first(const struct cpumask *mask)
{
	return find_next_bit(cpumask_bits(mask), small_cpumask_bits, 0);
}

int lupos_support_dl_mask_next(int cpu, const struct cpumask *mask)
{
	return find_next_bit(cpumask_bits(mask), small_cpumask_bits, cpu + 1);
}

int lupos_support_dl_mask_limit(void)
{
	return small_cpumask_bits;
}

/* Match the NR_CPUS == 1 branch of for_each_possible_cpu exactly. */
int lupos_support_dl_possible_first(void)
{
#if NR_CPUS == 1
	return 0;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits, 0);
#endif
}

int lupos_support_dl_possible_next(int cpu)
{
#if NR_CPUS == 1
	return cpu + 1;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits,
			     cpu + 1);
#endif
}

int lupos_support_dl_possible_limit(void)
{
#if NR_CPUS == 1
	return 1;
#else
	return small_cpumask_bits;
#endif
}

bool lupos_support_dl_likely_online(bool online)
{
	return likely(online);
}
