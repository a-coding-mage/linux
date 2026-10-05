// SPDX-License-Identifier: GPL-2.0
/*
 * Native storage, typed primitive leaves, and cgroup metadata for cpuacct.rs.
 * Source authority: 126a30fae3bba11420ec2fcbde51a0a01bab1b5b.
 * This does not include, rename, or call the cpuacct.c owner algorithms.
 * The original compiler/instrumentation owner is build_utility.c.
 */
#include "sched_cpuacct_bindings.h"

#error "SOURCE ONLY HOLD: scheduler cpuacct is not admitted"
#ifndef CONFIG_CGROUP_CPUACCT
#error "cpuacct native primitives require CONFIG_CGROUP_CPUACCT"
#endif

/* Original private native layout; opaque and never allocated from Rust. */
struct cpuacct {
	struct cgroup_subsys_state css;
	u64 __percpu *cpuusage;
	struct kernel_cpustat __percpu *cpustat;
};

static DEFINE_PER_CPU(u64, root_cpuacct_cpuusage);
static struct cpuacct root_cpuacct = {
	.cpustat = &kernel_cpustat,
	.cpuusage = &root_cpuacct_cpuusage,
};

static const char * const cpuacct_stat_desc[] = {
	[CPUACCT_STAT_USER] = "user",
	[CPUACCT_STAT_SYSTEM] = "system",
};

struct cpuacct *lupos_cpuacct_root(void)
{
	return &root_cpuacct;
}

struct cpuacct *lupos_cpuacct_from_css(struct cgroup_subsys_state *css)
{
	return container_of(css, struct cpuacct, css);
}

struct cgroup_subsys_state *lupos_cpuacct_css(struct cpuacct *ca)
{
	return &ca->css;
}

struct cgroup_subsys_state *lupos_cpuacct_task_css(struct task_struct *task)
{
	return task_css(task, cpuacct_cgrp_id);
}

struct cgroup_subsys_state *lupos_cpuacct_parent_css(struct cpuacct *ca)
{
	return ca->css.parent;
}

struct cgroup_subsys_state *lupos_cpuacct_seq_css(struct seq_file *sf)
{
	return seq_css(sf);
}

struct prev_cputime *lupos_cpuacct_seq_prev_cputime(struct seq_file *sf)
{
	return &seq_css(sf)->cgroup->prev_cputime;
}

struct cpuacct *lupos_cpuacct_alloc(void)
{
	struct cpuacct *ca = kzalloc_obj(*ca);

	return ca;
}

void lupos_cpuacct_free(struct cpuacct *ca)
{
	kfree(ca);
}

bool lupos_cpuacct_alloc_usage(struct cpuacct *ca)
{
	ca->cpuusage = alloc_percpu(u64);
	return ca->cpuusage != NULL;
}

bool lupos_cpuacct_alloc_stat(struct cpuacct *ca)
{
	ca->cpustat = alloc_percpu(struct kernel_cpustat);
	return ca->cpustat != NULL;
}

void lupos_cpuacct_free_usage(struct cpuacct *ca)
{
	free_percpu(ca->cpuusage);
}

void lupos_cpuacct_free_stat(struct cpuacct *ca)
{
	free_percpu(ca->cpustat);
}

struct cgroup_subsys_state *lupos_cpuacct_nomem(void)
{
	return ERR_PTR(-ENOMEM);
}

u64 *lupos_cpuacct_usage_ptr(struct cpuacct *ca, int cpu)
{
	return per_cpu_ptr(ca->cpuusage, cpu);
}

u64 *lupos_cpuacct_stat_ptr(struct cpuacct *ca, int cpu)
{
	return per_cpu_ptr(ca->cpustat, cpu)->cpustat;
}

/*
 * Keep plain shared-counter accesses in the native memory-model boundary.
 * These are the original unmarked accesses, not an atomic/READ_ONCE upgrade.
 * Rust still chooses fields, performs arithmetic, resets and walks ancestors.
 */
u64 lupos_cpuacct_read_counter(const u64 *counter)
{
	return *counter;
}

void lupos_cpuacct_write_counter(u64 *counter, u64 value)
{
	*counter = value;
}

bool lupos_cpuacct_warn_index(enum cpuacct_stat_index index)
{
	return WARN_ON_ONCE(index > CPUACCT_STAT_NSTATS);
}

int lupos_cpuacct_next_possible_cpu(int previous)
{
#if NR_CPUS == 1
	/* Preserve for_each_possible_cpu's constant-UP special case. */
	return previous < 0 ? 0 : -1;
#else
	unsigned int cpu;

	cpu = find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits,
			    previous + 1);
	return cpu < small_cpumask_bits ? (int)cpu : -1;
#endif
}

unsigned int lupos_cpuacct_task_cpu(struct task_struct *task)
{
	return task_cpu(task);
}

void lupos_cpuacct_assert_rq_held(unsigned int cpu)
{
	lockdep_assert_rq_held(cpu_rq(cpu));
}

void lupos_cpuacct_this_cpu_add(struct cpuacct *ca, int index, u64 value)
{
	__this_cpu_add(ca->cpustat->cpustat[index], value);
}

#ifndef CONFIG_64BIT
void lupos_cpuacct_rq_lock_irq(int cpu)
{
	raw_spin_rq_lock_irq(cpu_rq(cpu));
}

void lupos_cpuacct_rq_unlock_irq(int cpu)
{
	raw_spin_rq_unlock_irq(cpu_rq(cpu));
}
#endif /* !CONFIG_64BIT */

void lupos_cpuacct_seq_percpu(struct seq_file *sf, u64 value)
{
	seq_printf(sf, "%llu ", (unsigned long long)value);
}

void lupos_cpuacct_seq_percpu_end(struct seq_file *sf)
{
	seq_printf(sf, "\n");
}

void lupos_cpuacct_seq_cpu_header(struct seq_file *sf)
{
	seq_puts(sf, "cpu");
}

void lupos_cpuacct_seq_stat_header(struct seq_file *sf,
				 enum cpuacct_stat_index index)
{
	seq_printf(sf, " %s", cpuacct_stat_desc[index]);
}

void lupos_cpuacct_seq_newline(struct seq_file *sf)
{
	seq_puts(sf, "\n");
}

void lupos_cpuacct_seq_cpu(struct seq_file *sf, int cpu)
{
	seq_printf(sf, "%d", cpu);
}

void lupos_cpuacct_seq_usage(struct seq_file *sf, u64 value)
{
	seq_printf(sf, " %llu", value);
}

void lupos_cpuacct_seq_stat(struct seq_file *sf, enum cpuacct_stat_index index,
			    u64 nsecs)
{
	seq_printf(sf, "%s %llu\n", cpuacct_stat_desc[index],
		   nsec_to_clock_t(nsecs));
}

/*
 * Native headers determine name-array layout, callback signatures, bitfields,
 * and the terminating zero entry. Every callback body remains in Rust.
 */
static struct cftype files[] = {
	{
		.name = "usage",
		.read_u64 = lupos_cpuacct_usage_read,
		.write_u64 = lupos_cpuacct_usage_write,
	},
	{
		.name = "usage_user",
		.read_u64 = lupos_cpuacct_usage_user_read,
	},
	{
		.name = "usage_sys",
		.read_u64 = lupos_cpuacct_usage_sys_read,
	},
	{
		.name = "usage_percpu",
		.seq_show = lupos_cpuacct_percpu_seq_show,
	},
	{
		.name = "usage_percpu_user",
		.seq_show = lupos_cpuacct_percpu_user_seq_show,
	},
	{
		.name = "usage_percpu_sys",
		.seq_show = lupos_cpuacct_percpu_sys_seq_show,
	},
	{
		.name = "usage_all",
		.seq_show = lupos_cpuacct_all_seq_show,
	},
	{
		.name = "stat",
		.seq_show = lupos_cpuacct_stats_show,
	},
	{ }
};

struct cgroup_subsys cpuacct_cgrp_subsys = {
	.css_alloc = lupos_cpuacct_css_alloc,
	.css_free = lupos_cpuacct_css_free,
	.legacy_cftypes = files,
	.early_init = true,
};
