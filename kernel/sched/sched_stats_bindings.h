/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_STATS_BINDINGS_H
#define LUPOS_SCHED_STATS_BINDINGS_H

/* Configured native headers are the only type/layout/enum authority. */
#include "sched.h"
#include "stats.h"

#ifdef CONFIG_SCHEDSTATS

enum { LUPOS_STATS_CPU_MAX_IDLE_TYPES = CPU_MAX_IDLE_TYPES };

/* Rust-owned native entry points are already declared by stats.h. */
void *lupos_stats_schedstat_start(struct seq_file *file, loff_t *offset);
void *lupos_stats_schedstat_next(struct seq_file *file, void *data, loff_t *offset);
void lupos_stats_schedstat_stop(struct seq_file *file, void *data);
int lupos_stats_show_schedstat(struct seq_file *seq, void *data);
int __init lupos_stats_proc_schedstat_init(void);

u64 lupos_stats_rq_clock(struct rq *rq);
int lupos_stats_task_on_rq_migrating(struct task_struct *p);
unsigned int lupos_stats_task_in_iowait(const struct task_struct *p);
void lupos_stats_account_latency(struct task_struct *p, int usecs, int inter);
void lupos_stats_trace_wait(struct task_struct *p, u64 delta);
void lupos_stats_trace_sleep(struct task_struct *p, u64 delta);
void lupos_stats_trace_iowait(struct task_struct *p, u64 delta);
void lupos_stats_trace_blocked(struct task_struct *p, u64 delta);

/* One native field access per leaf; no update algorithm is moved to C. */
#define LUPOS_STATS_FIELD(type, field) \
	type lupos_stats_##field##_read(const struct sched_statistics *stats); \
	void lupos_stats_##field##_write(struct sched_statistics *stats, type value)
LUPOS_STATS_FIELD(u64, wait_start);
LUPOS_STATS_FIELD(u64, wait_max);
LUPOS_STATS_FIELD(u64, wait_count);
LUPOS_STATS_FIELD(u64, wait_sum);
LUPOS_STATS_FIELD(u64, iowait_count);
LUPOS_STATS_FIELD(u64, iowait_sum);
LUPOS_STATS_FIELD(u64, sleep_start);
LUPOS_STATS_FIELD(u64, sleep_max);
LUPOS_STATS_FIELD(s64, sum_sleep_runtime);
LUPOS_STATS_FIELD(u64, block_start);
LUPOS_STATS_FIELD(u64, block_max);
LUPOS_STATS_FIELD(s64, sum_block_runtime);
#undef LUPOS_STATS_FIELD

struct rq *lupos_stats_cpu_rq(int cpu);
unsigned int lupos_stats_rq_yld_count(const struct rq *rq);
unsigned int lupos_stats_rq_sched_count(const struct rq *rq);
unsigned int lupos_stats_rq_sched_goidle(const struct rq *rq);
unsigned int lupos_stats_rq_ttwu_count(const struct rq *rq);
unsigned int lupos_stats_rq_ttwu_local(const struct rq *rq);
unsigned long long lupos_stats_rq_cpu_time(const struct rq *rq);
unsigned long long lupos_stats_rq_run_delay(const struct rq *rq);
unsigned long lupos_stats_rq_pcount(const struct rq *rq);

void lupos_stats_rcu_read_lock(void) __acquires_shared(RCU);
void lupos_stats_rcu_read_unlock(void) __releases_shared(RCU);
struct sched_domain *lupos_stats_domain_first(int cpu);
struct sched_domain *lupos_stats_domain_parent(struct sched_domain *sd);
const char *lupos_stats_domain_name(const struct sched_domain *sd);
const unsigned long *lupos_stats_domain_span_bits(struct sched_domain *sd);

#define LUPOS_STATS_DOMAIN_ARRAY(field) \
	unsigned int lupos_stats_domain_##field(const struct sched_domain *sd, \
					       unsigned int itype)
LUPOS_STATS_DOMAIN_ARRAY(lb_count);
LUPOS_STATS_DOMAIN_ARRAY(lb_balanced);
LUPOS_STATS_DOMAIN_ARRAY(lb_failed);
LUPOS_STATS_DOMAIN_ARRAY(lb_imbalance_load);
LUPOS_STATS_DOMAIN_ARRAY(lb_imbalance_util);
LUPOS_STATS_DOMAIN_ARRAY(lb_imbalance_task);
LUPOS_STATS_DOMAIN_ARRAY(lb_imbalance_misfit);
LUPOS_STATS_DOMAIN_ARRAY(lb_gained);
LUPOS_STATS_DOMAIN_ARRAY(lb_hot_gained);
LUPOS_STATS_DOMAIN_ARRAY(lb_nobusyq);
LUPOS_STATS_DOMAIN_ARRAY(lb_nobusyg);
#undef LUPOS_STATS_DOMAIN_ARRAY

#define LUPOS_STATS_DOMAIN_SCALAR(field) \
	unsigned int lupos_stats_domain_##field(const struct sched_domain *sd)
LUPOS_STATS_DOMAIN_SCALAR(alb_count);
LUPOS_STATS_DOMAIN_SCALAR(alb_failed);
LUPOS_STATS_DOMAIN_SCALAR(alb_pushed);
LUPOS_STATS_DOMAIN_SCALAR(sbe_count);
LUPOS_STATS_DOMAIN_SCALAR(sbe_balanced);
LUPOS_STATS_DOMAIN_SCALAR(sbe_pushed);
LUPOS_STATS_DOMAIN_SCALAR(sbf_count);
LUPOS_STATS_DOMAIN_SCALAR(sbf_balanced);
LUPOS_STATS_DOMAIN_SCALAR(sbf_pushed);
LUPOS_STATS_DOMAIN_SCALAR(ttwu_wake_remote);
LUPOS_STATS_DOMAIN_SCALAR(ttwu_move_affine);
LUPOS_STATS_DOMAIN_SCALAR(ttwu_move_balance);
#undef LUPOS_STATS_DOMAIN_SCALAR

unsigned long lupos_stats_jiffies(void);
unsigned int lupos_stats_nr_cpu_ids(void);
unsigned int lupos_stats_first_online_cpu(void);
unsigned int lupos_stats_next_online_cpu(int cpu);
void __init lupos_stats_proc_create_seq(void);

#endif /* CONFIG_SCHEDSTATS */
#endif /* LUPOS_SCHED_STATS_BINDINGS_H */
