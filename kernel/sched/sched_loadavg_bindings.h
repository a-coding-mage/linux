/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_LOADAVG_BINDINGS_H
#define LUPOS_SCHED_LOADAVG_BINDINGS_H

/* Native configured headers are the sole layout and constant authority. */
#include "sched.h"
#include <linux/sched/nohz.h>

#define LUPOS_LOADAVG_FSHIFT FSHIFT
#define LUPOS_LOADAVG_FIXED_1 FIXED_1
#define LUPOS_LOADAVG_LOAD_FREQ LOAD_FREQ
#define LUPOS_LOADAVG_EXP_1 EXP_1
#define LUPOS_LOADAVG_EXP_5 EXP_5
#define LUPOS_LOADAVG_EXP_15 EXP_15

struct rq *lupos_loadavg_this_rq(void);
unsigned int lupos_loadavg_rq_nr_running(const struct rq *rq);
unsigned long lupos_loadavg_rq_nr_uninterruptible(const struct rq *rq);
long lupos_loadavg_rq_active_read(const struct rq *rq);
void lupos_loadavg_rq_active_write(struct rq *rq, long value);
unsigned long lupos_loadavg_rq_update_read(const struct rq *rq);
void lupos_loadavg_rq_update_write(struct rq *rq, unsigned long value);
unsigned long lupos_loadavg_jiffies(void);
bool lupos_loadavg_time_before(unsigned long a, unsigned long b);
unsigned long lupos_loadavg_update_read_once(void);
void lupos_loadavg_update_write_once(unsigned long value);
long lupos_loadavg_tasks_read(void);
void lupos_loadavg_tasks_add(long delta);
unsigned long lupos_loadavg_avenrun_read(unsigned int index);
void lupos_loadavg_avenrun_write(unsigned int index, unsigned long value);
unsigned long lupos_loadavg_calc_load(unsigned long load, unsigned long exp,
				     unsigned long active);

#ifdef CONFIG_NO_HZ_COMMON
void lupos_loadavg_read_barrier(void);
void lupos_loadavg_write_barrier(void);
int lupos_loadavg_nohz_idx_read(void);
void lupos_loadavg_nohz_idx_write(int value);
long lupos_loadavg_nohz_read(unsigned int index);
void lupos_loadavg_nohz_add(unsigned int index, long delta);
long lupos_loadavg_nohz_xchg(unsigned int index, long value);
#endif /* CONFIG_NO_HZ_COMMON */

#endif /* LUPOS_SCHED_LOADAVG_BINDINGS_H */
