// SPDX-License-Identifier: GPL-2.0
/*
 * Native leaves for the Rust cpumask owner: the per-CPU distribution cursor
 * and its this-CPU accessors, the configured CPU-count and NUMA lookup, the
 * inline wrapping searches and the local-spread warning. Selection policy
 * lives in lib/cpumask.rs.
 */
#include <linux/bug.h>
#include <linux/cpumask.h>
#include <linux/percpu.h>
#include <linux/topology.h>

static DEFINE_PER_CPU(int, distribute_cpu_mask_prev);

unsigned int lupos_cpumask_nr_cpu_ids(void);
unsigned int lupos_cpumask_find_nth_online(unsigned int i, int node);
void lupos_cpumask_warn_local_spread(void);
int lupos_cpumask_distribute_prev_read(void);
void lupos_cpumask_distribute_prev_write(unsigned int next);
unsigned int lupos_cpumask_next_and_wrap(int n, const struct cpumask *src1p,
					 const struct cpumask *src2p);
unsigned int lupos_cpumask_next_wrap(int n, const struct cpumask *src);

unsigned int lupos_cpumask_nr_cpu_ids(void)
{
	return nr_cpu_ids;
}

unsigned int lupos_cpumask_find_nth_online(unsigned int i, int node)
{
	return sched_numa_find_nth_cpu(cpu_online_mask, i, node);
}

void lupos_cpumask_warn_local_spread(void)
{
	WARN_ON(1);
}

int lupos_cpumask_distribute_prev_read(void)
{
	return __this_cpu_read(distribute_cpu_mask_prev);
}

void lupos_cpumask_distribute_prev_write(unsigned int next)
{
	__this_cpu_write(distribute_cpu_mask_prev, next);
}

unsigned int lupos_cpumask_next_and_wrap(int n, const struct cpumask *src1p,
					 const struct cpumask *src2p)
{
	return cpumask_next_and_wrap(n, src1p, src2p);
}

unsigned int lupos_cpumask_next_wrap(int n, const struct cpumask *src)
{
	return cpumask_next_wrap(n, src);
}
