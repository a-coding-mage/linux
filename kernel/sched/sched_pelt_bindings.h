/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_PELT_BINDINGS_H
#define LUPOS_SCHED_PELT_BINDINGS_H

/* Configured native headers alone own scheduler layouts and constant values. */
#include <linux/math64.h>
#include "pelt.h"

static const unsigned int LUPOS_PELT_LOAD_AVG_PERIOD = LOAD_AVG_PERIOD;
static const u32 LUPOS_PELT_LOAD_AVG_MAX = LOAD_AVG_MAX;
static const u32 LUPOS_PELT_MIN_DIVIDER = PELT_MIN_DIVIDER;
static const unsigned int LUPOS_PELT_SCHED_CAPACITY_SHIFT = SCHED_CAPACITY_SHIFT;
static const unsigned int LUPOS_PELT_UTIL_AVG_UNCHANGED = UTIL_AVG_UNCHANGED;

bool lupos_pelt_unlikely_decay_zero(bool condition);
bool lupos_pelt_unlikely_decay_period(bool condition);
u32 lupos_pelt_decay_factor(unsigned int n);
u64 lupos_pelt_mul_u64_u32_shr(u64 value, u32 factor, unsigned int shift);
u64 lupos_pelt_div_u64(u64 dividend, u32 divisor);
long lupos_pelt_se_weight(struct sched_entity *se);
long lupos_pelt_se_runnable(struct sched_entity *se);
unsigned long lupos_pelt_scale_load_down(unsigned long weight);
bool lupos_pelt_util_est_enabled(void);
void lupos_pelt_write_util_avg(struct sched_avg *avg, unsigned long value);
void lupos_pelt_write_util_est(struct sched_avg *avg, unsigned int value);
void lupos_pelt_lockdep_assert_rq_held(struct rq *rq);
void lupos_pelt_assert_clock_updated(struct rq *rq);
u64 lupos_pelt_rq_clock_task(struct rq *rq);
unsigned long lupos_pelt_hw_pressure(int cpu);
const struct sched_class *lupos_pelt_donor_class(struct rq *rq);
void lupos_pelt_trace_se(struct sched_entity *se);
void lupos_pelt_trace_cfs(struct cfs_rq *cfs_rq);
void lupos_pelt_trace_rt(struct rq *rq);
void lupos_pelt_trace_dl(struct rq *rq);
#ifdef CONFIG_SCHED_HW_PRESSURE
void lupos_pelt_trace_hw(struct rq *rq);
#endif /* CONFIG_SCHED_HW_PRESSURE */
#ifdef CONFIG_HAVE_SCHED_AVG_IRQ
/* Match the u64 operand's usual arithmetic conversion in cap_scale(). */
u64 lupos_pelt_freq_capacity(int cpu);
u64 lupos_pelt_cpu_capacity(int cpu);
void lupos_pelt_trace_irq(struct rq *rq);
#endif /* CONFIG_HAVE_SCHED_AVG_IRQ */

#endif /* LUPOS_SCHED_PELT_BINDINGS_H */
