// SPDX-License-Identifier: GPL-2.0
/*
 * Native field/header primitives and registration for the existing stats.rs.
 * No original stats.c body is included, called or selected as a fallback.
 * Native compiler/instrumentation policy origin: build_utility.c.
 */
#include "sched_stats_bindings.h"

#error "SOURCE ONLY HOLD: scheduler statistics is not admitted"

#ifndef CONFIG_SCHEDSTATS
#error "scheduler statistics native primitives require CONFIG_SCHEDSTATS"
#endif

/* Native seq_operations field order is start, stop, next, show. */
static const struct seq_operations schedstat_sops = {
	.start = lupos_stats_schedstat_start,
	.next = lupos_stats_schedstat_next,
	.stop = lupos_stats_schedstat_stop,
	.show = lupos_stats_show_schedstat,
};

void __init lupos_stats_proc_create_seq(void)
{
	proc_create_seq("schedstat", 0, NULL, &schedstat_sops);
}

/* Preserve native init section, initcall level, and indirect callback types. */
static int __init proc_schedstat_init(void)
{
	return lupos_stats_proc_schedstat_init();
}
subsys_initcall(proc_schedstat_init);

u64 lupos_stats_rq_clock(struct rq *rq)
{
	/* Native inline retains lockdep_assert_rq_held and assert_clock_updated. */
	return rq_clock(rq);
}

int lupos_stats_task_on_rq_migrating(struct task_struct *p)
{
	return task_on_rq_migrating(p);
}

unsigned int lupos_stats_task_in_iowait(const struct task_struct *p)
{
	return p->in_iowait;
}

void lupos_stats_account_latency(struct task_struct *p, int usecs, int inter)
{
	account_scheduler_latency(p, usecs, inter);
}

void lupos_stats_trace_wait(struct task_struct *p, u64 delta)
{
	trace_sched_stat_wait(p, delta);
}

void lupos_stats_trace_sleep(struct task_struct *p, u64 delta)
{
	trace_sched_stat_sleep(p, delta);
}

void lupos_stats_trace_iowait(struct task_struct *p, u64 delta)
{
	trace_sched_stat_iowait(p, delta);
}

void lupos_stats_trace_blocked(struct task_struct *p, u64 delta)
{
	trace_sched_stat_blocked(p, delta);
}

#define LUPOS_STATS_FIELD(type, field) \
	type lupos_stats_##field##_read(const struct sched_statistics *stats) \
	{ \
		return stats->field; \
	} \
	void lupos_stats_##field##_write(struct sched_statistics *stats, type value) \
	{ \
		stats->field = value; \
	}
LUPOS_STATS_FIELD(u64, wait_start)
LUPOS_STATS_FIELD(u64, wait_max)
LUPOS_STATS_FIELD(u64, wait_count)
LUPOS_STATS_FIELD(u64, wait_sum)
LUPOS_STATS_FIELD(u64, iowait_count)
LUPOS_STATS_FIELD(u64, iowait_sum)
LUPOS_STATS_FIELD(u64, sleep_start)
LUPOS_STATS_FIELD(u64, sleep_max)
LUPOS_STATS_FIELD(s64, sum_sleep_runtime)
LUPOS_STATS_FIELD(u64, block_start)
LUPOS_STATS_FIELD(u64, block_max)
LUPOS_STATS_FIELD(s64, sum_block_runtime)
#undef LUPOS_STATS_FIELD

struct rq *lupos_stats_cpu_rq(int cpu)
{
	return cpu_rq(cpu);
}

#define LUPOS_STATS_RQ_FIELD(type, name, field) \
	type lupos_stats_rq_##name(const struct rq *rq) \
	{ \
		return rq->field; \
	}
LUPOS_STATS_RQ_FIELD(unsigned int, yld_count, yld_count)
LUPOS_STATS_RQ_FIELD(unsigned int, sched_count, sched_count)
LUPOS_STATS_RQ_FIELD(unsigned int, sched_goidle, sched_goidle)
LUPOS_STATS_RQ_FIELD(unsigned int, ttwu_count, ttwu_count)
LUPOS_STATS_RQ_FIELD(unsigned int, ttwu_local, ttwu_local)
LUPOS_STATS_RQ_FIELD(unsigned long long, cpu_time, rq_cpu_time)
LUPOS_STATS_RQ_FIELD(unsigned long long, run_delay, rq_sched_info.run_delay)
LUPOS_STATS_RQ_FIELD(unsigned long, pcount, rq_sched_info.pcount)
#undef LUPOS_STATS_RQ_FIELD

void lupos_stats_rcu_read_lock(void)
	__acquires_shared(RCU)
{
	rcu_read_lock();
}

void lupos_stats_rcu_read_unlock(void)
	__releases_shared(RCU)
{
	rcu_read_unlock();
}

struct sched_domain *lupos_stats_domain_first(int cpu)
{
	/* Identical first expression to for_each_domain; caller holds RCU. */
	return rcu_dereference_sched_domain(cpu_rq(cpu)->sd);
}

struct sched_domain *lupos_stats_domain_parent(struct sched_domain *sd)
{
	/* Identical advancement expression to for_each_domain under caller RCU. */
	return sd->parent;
}

const char *lupos_stats_domain_name(const struct sched_domain *sd)
{
	return sd->name;
}

const unsigned long *lupos_stats_domain_span_bits(struct sched_domain *sd)
{
	return cpumask_bits(sched_domain_span(sd));
}

#define LUPOS_STATS_DOMAIN_ARRAY(field) \
	unsigned int lupos_stats_domain_##field(const struct sched_domain *sd, \
					       unsigned int itype) \
	{ \
		return sd->field[itype]; \
	}
/* Caller holds RCU and bounds itype by the header-derived enum terminator. */
LUPOS_STATS_DOMAIN_ARRAY(lb_count)
LUPOS_STATS_DOMAIN_ARRAY(lb_balanced)
LUPOS_STATS_DOMAIN_ARRAY(lb_failed)
LUPOS_STATS_DOMAIN_ARRAY(lb_imbalance_load)
LUPOS_STATS_DOMAIN_ARRAY(lb_imbalance_util)
LUPOS_STATS_DOMAIN_ARRAY(lb_imbalance_task)
LUPOS_STATS_DOMAIN_ARRAY(lb_imbalance_misfit)
LUPOS_STATS_DOMAIN_ARRAY(lb_gained)
LUPOS_STATS_DOMAIN_ARRAY(lb_hot_gained)
LUPOS_STATS_DOMAIN_ARRAY(lb_nobusyq)
LUPOS_STATS_DOMAIN_ARRAY(lb_nobusyg)
#undef LUPOS_STATS_DOMAIN_ARRAY

#define LUPOS_STATS_DOMAIN_SCALAR(field) \
	unsigned int lupos_stats_domain_##field(const struct sched_domain *sd) \
	{ \
		return sd->field; \
	}
LUPOS_STATS_DOMAIN_SCALAR(alb_count)
LUPOS_STATS_DOMAIN_SCALAR(alb_failed)
LUPOS_STATS_DOMAIN_SCALAR(alb_pushed)
LUPOS_STATS_DOMAIN_SCALAR(sbe_count)
LUPOS_STATS_DOMAIN_SCALAR(sbe_balanced)
LUPOS_STATS_DOMAIN_SCALAR(sbe_pushed)
LUPOS_STATS_DOMAIN_SCALAR(sbf_count)
LUPOS_STATS_DOMAIN_SCALAR(sbf_balanced)
LUPOS_STATS_DOMAIN_SCALAR(sbf_pushed)
LUPOS_STATS_DOMAIN_SCALAR(ttwu_wake_remote)
LUPOS_STATS_DOMAIN_SCALAR(ttwu_move_affine)
LUPOS_STATS_DOMAIN_SCALAR(ttwu_move_balance)
#undef LUPOS_STATS_DOMAIN_SCALAR

unsigned long lupos_stats_jiffies(void)
{
	return jiffies;
}

unsigned int lupos_stats_nr_cpu_ids(void)
{
	return nr_cpu_ids;
}

unsigned int lupos_stats_first_online_cpu(void)
{
	return cpumask_first(cpu_online_mask);
}

unsigned int lupos_stats_next_online_cpu(int cpu)
{
	return cpumask_next(cpu, cpu_online_mask);
}
