/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_CPUACCT_BINDINGS_H
#define LUPOS_SCHED_CPUACCT_BINDINGS_H

/* Configured native headers are the sole layout and constant authority. */
#include <linux/sched/cputime.h>
#include <linux/err.h>
#include <linux/percpu.h>
#include <linux/slab.h>
#include "sched.h"

/* Private cpuacct.c enum, retained natively for the continued Rust owner. */
enum cpuacct_stat_index {
	CPUACCT_STAT_USER,
	CPUACCT_STAT_SYSTEM,
	CPUACCT_STAT_NSTATS,
};

struct cpuacct;
#define LUPOS_CPUACCT_EINVAL EINVAL

struct cpuacct *lupos_cpuacct_root(void);
struct cpuacct *lupos_cpuacct_from_css(struct cgroup_subsys_state *css);
struct cgroup_subsys_state *lupos_cpuacct_css(struct cpuacct *ca);
struct cgroup_subsys_state *lupos_cpuacct_task_css(struct task_struct *task);
struct cgroup_subsys_state *lupos_cpuacct_parent_css(struct cpuacct *ca);
struct cgroup_subsys_state *lupos_cpuacct_seq_css(struct seq_file *sf);
struct prev_cputime *lupos_cpuacct_seq_prev_cputime(struct seq_file *sf);
struct cpuacct *lupos_cpuacct_alloc(void);
void lupos_cpuacct_free(struct cpuacct *ca);
bool lupos_cpuacct_alloc_usage(struct cpuacct *ca);
bool lupos_cpuacct_alloc_stat(struct cpuacct *ca);
void lupos_cpuacct_free_usage(struct cpuacct *ca);
void lupos_cpuacct_free_stat(struct cpuacct *ca);
struct cgroup_subsys_state *lupos_cpuacct_nomem(void);
u64 *lupos_cpuacct_usage_ptr(struct cpuacct *ca, int cpu);
u64 *lupos_cpuacct_stat_ptr(struct cpuacct *ca, int cpu);
u64 lupos_cpuacct_read_counter(const u64 *counter);
void lupos_cpuacct_write_counter(u64 *counter, u64 value);
bool lupos_cpuacct_warn_index(enum cpuacct_stat_index index);
/* Pass -1 to begin; -1 is also the terminal cursor. */
int lupos_cpuacct_next_possible_cpu(int previous);
unsigned int lupos_cpuacct_task_cpu(struct task_struct *task);
void lupos_cpuacct_assert_rq_held(unsigned int cpu)
	__assumes_ctx_lock(__rq_lockp(cpu_rq(cpu)));
void lupos_cpuacct_this_cpu_add(struct cpuacct *ca, int index, u64 value);
#ifndef CONFIG_64BIT
void lupos_cpuacct_rq_lock_irq(int cpu)
	__acquires(__rq_lockp(cpu_rq(cpu)));
void lupos_cpuacct_rq_unlock_irq(int cpu)
	__releases(__rq_lockp(cpu_rq(cpu)));
#endif

/* Typed leaves preserve native varargs formats and descriptor strings. */
void lupos_cpuacct_seq_percpu(struct seq_file *sf, u64 value);
void lupos_cpuacct_seq_percpu_end(struct seq_file *sf);
void lupos_cpuacct_seq_cpu_header(struct seq_file *sf);
void lupos_cpuacct_seq_stat_header(struct seq_file *sf,
				 enum cpuacct_stat_index index);
void lupos_cpuacct_seq_newline(struct seq_file *sf);
void lupos_cpuacct_seq_cpu(struct seq_file *sf, int cpu);
void lupos_cpuacct_seq_usage(struct seq_file *sf, u64 value);
void lupos_cpuacct_seq_stat(struct seq_file *sf, enum cpuacct_stat_index index,
			    u64 nsecs);

/* Rust-owned callbacks referenced by the native cgroup metadata. */
struct cgroup_subsys_state *
lupos_cpuacct_css_alloc(struct cgroup_subsys_state *parent_css);
void lupos_cpuacct_css_free(struct cgroup_subsys_state *css);
u64 lupos_cpuacct_usage_user_read(struct cgroup_subsys_state *css,
				 struct cftype *cft);
u64 lupos_cpuacct_usage_sys_read(struct cgroup_subsys_state *css,
				struct cftype *cft);
u64 lupos_cpuacct_usage_read(struct cgroup_subsys_state *css,
			    struct cftype *cft);
int lupos_cpuacct_usage_write(struct cgroup_subsys_state *css,
			     struct cftype *cft, u64 value);
int lupos_cpuacct_percpu_user_seq_show(struct seq_file *sf, void *v);
int lupos_cpuacct_percpu_sys_seq_show(struct seq_file *sf, void *v);
int lupos_cpuacct_percpu_seq_show(struct seq_file *sf, void *v);
int lupos_cpuacct_all_seq_show(struct seq_file *sf, void *v);
int lupos_cpuacct_stats_show(struct seq_file *sf, void *v);

#endif /* LUPOS_SCHED_CPUACCT_BINDINGS_H */
