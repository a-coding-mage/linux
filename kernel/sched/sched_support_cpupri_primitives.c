// SPDX-License-Identifier: GPL-2.0-only
/*
 * Configured primitive leaves for cpupri.rs. Source proposal only.
 * Policy origin: build_utility.c. No cpupri.c algorithm is included here.
 */
#include "sched_support_bindings.h"

int lupos_support_pri_atomic_read(const atomic_t *count)
{
	return atomic_read(count);
}

void lupos_support_pri_atomic_set(atomic_t *count, int value)
{
	atomic_set(count, value);
}

void lupos_support_pri_atomic_inc(atomic_t *count)
{
	atomic_inc(count);
}

void lupos_support_pri_atomic_dec(atomic_t *count)
{
	atomic_dec(count);
}

void lupos_support_pri_read_barrier(void)
{
	smp_rmb();
}

void lupos_support_pri_before_publish(void)
{
	smp_mb__before_atomic();
}

void lupos_support_pri_after_publish(void)
{
	smp_mb__after_atomic();
}

void lupos_support_pri_after_remove(void)
{
	smp_mb__after_atomic();
}

struct cpumask *lupos_support_pri_mask(struct cpupri_vec *vec)
{
	return vec->mask;
}

bool lupos_support_pri_alloc_mask(struct cpupri_vec *vec)
{
	return zalloc_cpumask_var(&vec->mask, GFP_KERNEL);
}

void lupos_support_pri_free_mask(struct cpupri_vec *vec)
{
	free_cpumask_var(vec->mask);
}

int *lupos_support_pri_alloc_priorities(void)
{
	return kzalloc_objs(int, nr_cpu_ids);
}

unsigned int lupos_support_pri_mask_any_and(const struct cpumask *a,
					  const struct cpumask *b)
{
	return cpumask_any_and(a, b);
}

bool lupos_support_pri_mask_and(struct cpumask *dst,
			      const struct cpumask *a, const struct cpumask *b)
{
	return cpumask_and(dst, a, b);
}

bool lupos_support_pri_mask_empty(const struct cpumask *mask)
{
	return cpumask_empty(mask);
}

void lupos_support_pri_mask_set(int cpu, struct cpumask *mask)
{
	cpumask_set_cpu(cpu, mask);
}

void lupos_support_pri_mask_clear(int cpu, struct cpumask *mask)
{
	cpumask_clear_cpu(cpu, mask);
}

const struct cpumask *lupos_support_pri_active_mask(void)
{
	return cpu_active_mask;
}

unsigned int lupos_support_pri_nr_cpu_ids(void)
{
	return nr_cpu_ids;
}

int lupos_support_pri_mask_first(const struct cpumask *mask)
{
	return find_next_bit(cpumask_bits(mask), small_cpumask_bits, 0);
}

int lupos_support_pri_mask_next(int cpu, const struct cpumask *mask)
{
	return find_next_bit(cpumask_bits(mask), small_cpumask_bits, cpu + 1);
}

int lupos_support_pri_mask_limit(void)
{
	return small_cpumask_bits;
}

/* Match the NR_CPUS == 1 branch of for_each_possible_cpu exactly. */
int lupos_support_pri_possible_first(void)
{
#if NR_CPUS == 1
	return 0;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits, 0);
#endif
}

int lupos_support_pri_possible_next(int cpu)
{
#if NR_CPUS == 1
	return cpu + 1;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits,
			     cpu + 1);
#endif
}

int lupos_support_pri_possible_limit(void)
{
#if NR_CPUS == 1
	return 1;
#else
	return small_cpumask_bits;
#endif
}

void lupos_support_pri_warn_find_prio(int task_pri)
{
	WARN_ON_ONCE(task_pri >= CPUPRI_NR_PRIORITIES);
}

void lupos_support_pri_bug_set_prio(int newpri)
{
	BUG_ON(newpri >= CPUPRI_NR_PRIORITIES);
}

bool lupos_support_pri_likely_new(int newpri)
{
	return likely(newpri != CPUPRI_INVALID);
}

bool lupos_support_pri_likely_old(int oldpri)
{
	return likely(oldpri != CPUPRI_INVALID);
}
