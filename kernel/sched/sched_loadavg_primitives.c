// SPDX-License-Identifier: GPL-2.0
/*
 * Native storage and header primitives for the continued loadavg.rs owner.
 * This file replaces storage from loadavg.c only if that owner is selected.
 * It does not include or call any algorithm from the original loadavg.c.
 * Native compiler/instrumentation policy origin: build_utility.c.
 */
#include "sched_loadavg_bindings.h"

#error "SOURCE ONLY HOLD: scheduler loadavg is not admitted"

/* Same native definitions and export as loadavg.c; no Rust atomic layout. */
atomic_long_t calc_load_tasks;
unsigned long calc_load_update;
unsigned long avenrun[3];
EXPORT_SYMBOL(avenrun);

struct rq *lupos_loadavg_this_rq(void)
{
	return this_rq();
}

unsigned int lupos_loadavg_rq_nr_running(const struct rq *rq)
{
	return rq->nr_running;
}

unsigned long lupos_loadavg_rq_nr_uninterruptible(const struct rq *rq)
{
	return rq->nr_uninterruptible;
}

long lupos_loadavg_rq_active_read(const struct rq *rq)
{
	return rq->calc_load_active;
}

void lupos_loadavg_rq_active_write(struct rq *rq, long value)
{
	rq->calc_load_active = value;
}

unsigned long lupos_loadavg_rq_update_read(const struct rq *rq)
{
	return rq->calc_load_update;
}

void lupos_loadavg_rq_update_write(struct rq *rq, unsigned long value)
{
	rq->calc_load_update = value;
}

unsigned long lupos_loadavg_jiffies(void)
{
	return jiffies;
}

bool lupos_loadavg_time_before(unsigned long a, unsigned long b)
{
	return time_before(a, b);
}

unsigned long lupos_loadavg_update_read_once(void)
{
	return READ_ONCE(calc_load_update);
}

void lupos_loadavg_update_write_once(unsigned long value)
{
	WRITE_ONCE(calc_load_update, value);
}

long lupos_loadavg_tasks_read(void)
{
	return atomic_long_read(&calc_load_tasks);
}

void lupos_loadavg_tasks_add(long delta)
{
	atomic_long_add(delta, &calc_load_tasks);
}

unsigned long lupos_loadavg_avenrun_read(unsigned int index)
{
	return avenrun[index];
}

void lupos_loadavg_avenrun_write(unsigned int index, unsigned long value)
{
	avenrun[index] = value;
}

unsigned long lupos_loadavg_calc_load(unsigned long load, unsigned long exp,
				     unsigned long active)
{
	/* Header-inline fixed-point arithmetic, not an original owner fallback. */
	return calc_load(load, exp, active);
}

#ifdef CONFIG_NO_HZ_COMMON
static atomic_long_t calc_load_nohz[2];
static int calc_load_idx;

void lupos_loadavg_read_barrier(void)
{
	smp_rmb();
}

void lupos_loadavg_write_barrier(void)
{
	smp_wmb();
}

int lupos_loadavg_nohz_idx_read(void)
{
	return calc_load_idx;
}

void lupos_loadavg_nohz_idx_write(int value)
{
	calc_load_idx = value;
}

long lupos_loadavg_nohz_read(unsigned int index)
{
	return atomic_long_read(&calc_load_nohz[index]);
}

void lupos_loadavg_nohz_add(unsigned int index, long delta)
{
	atomic_long_add(delta, &calc_load_nohz[index]);
}

long lupos_loadavg_nohz_xchg(unsigned int index, long value)
{
	return atomic_long_xchg(&calc_load_nohz[index], value);
}
#endif /* CONFIG_NO_HZ_COMMON */
