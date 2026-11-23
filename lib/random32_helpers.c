// SPDX-License-Identifier: GPL-2.0
/*
 * Native leaves for the Rust prandom owner: possible-CPU iteration and
 * per-CPU state addressing. The generator algorithm lives in random32.rs.
 */
#include <linux/build_bug.h>
#include <linux/cpumask.h>
#include <linux/percpu.h>
#include <linux/prandom.h>
#include <linux/stddef.h>

/* random32.rs mirrors struct rnd_state; keep its layout tied to the header. */
static_assert(sizeof(struct rnd_state) == 16);
static_assert(offsetof(struct rnd_state, s1) == 0);
static_assert(offsetof(struct rnd_state, s2) == 4);
static_assert(offsetof(struct rnd_state, s3) == 8);
static_assert(offsetof(struct rnd_state, s4) == 12);

int lupos_random32_next_possible_cpu(int cpu);
unsigned int lupos_random32_nr_cpu_ids(void);
struct rnd_state *lupos_random32_cpu_state(struct rnd_state __percpu *pcpu_state, int cpu);

int lupos_random32_next_possible_cpu(int cpu)
{
	return cpumask_next(cpu, cpu_possible_mask);
}

unsigned int lupos_random32_nr_cpu_ids(void)
{
	return nr_cpu_ids;
}

struct rnd_state *lupos_random32_cpu_state(struct rnd_state __percpu *pcpu_state, int cpu)
{
	return per_cpu_ptr(pcpu_state, cpu);
}
