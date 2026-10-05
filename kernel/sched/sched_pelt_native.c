// SPDX-License-Identifier: GPL-2.0-only
/*
 * Configured native primitive leaves for the existing Rust PELT translation.
 * No pelt.c algorithm is included, implemented or called from these leaves.
 */
#error "SOURCE ONLY HOLD: PELT native ABI/accounting admission is pending"

#include <linux/array_size.h>
#include <linux/build_bug.h>
#include "sched_pelt_bindings.h"

/* The bound proved in Rust must remain identical to the native table extent. */
static_assert(ARRAY_SIZE(runnable_avg_yN_inv) == LOAD_AVG_PERIOD);

bool lupos_pelt_unlikely_decay_zero(bool condition)
{
	return unlikely(condition);
}

bool lupos_pelt_unlikely_decay_period(bool condition)
{
	return unlikely(condition);
}

u32 lupos_pelt_decay_factor(unsigned int n)
{
	return runnable_avg_yN_inv[n];
}

u64 lupos_pelt_mul_u64_u32_shr(u64 value, u32 factor, unsigned int shift)
{
	return mul_u64_u32_shr(value, factor, shift);
}

u64 lupos_pelt_div_u64(u64 dividend, u32 divisor)
{
	return div_u64(dividend, divisor);
}

long lupos_pelt_se_weight(struct sched_entity *se)
{
	return se_weight(se);
}

long lupos_pelt_se_runnable(struct sched_entity *se)
{
	return se_runnable(se);
}

unsigned long lupos_pelt_scale_load_down(unsigned long weight)
{
	return scale_load_down(weight);
}

bool lupos_pelt_util_est_enabled(void)
{
	return sched_feat(UTIL_EST);
}

void lupos_pelt_write_util_avg(struct sched_avg *avg, unsigned long value)
{
	WRITE_ONCE(avg->util_avg, value);
}

void lupos_pelt_write_util_est(struct sched_avg *avg, unsigned int value)
{
	WRITE_ONCE(avg->util_est, value);
}

void lupos_pelt_lockdep_assert_rq_held(struct rq *rq)
{
	lockdep_assert_rq_held(rq);
}

void lupos_pelt_assert_clock_updated(struct rq *rq)
{
	assert_clock_updated(rq);
}

u64 lupos_pelt_rq_clock_task(struct rq *rq)
{
	return rq_clock_task(rq);
}

unsigned long lupos_pelt_hw_pressure(int cpu)
{
	return arch_scale_hw_pressure(cpu);
}

/* Keep the plain native access across conditional anonymous storage. */
const struct sched_class *lupos_pelt_donor_class(struct rq *rq)
{
	return rq->donor->sched_class;
}

void lupos_pelt_trace_se(struct sched_entity *se)
{
	trace_pelt_se_tp(se);
}

void lupos_pelt_trace_cfs(struct cfs_rq *cfs_rq)
{
	trace_pelt_cfs_tp(cfs_rq);
}

void lupos_pelt_trace_rt(struct rq *rq)
{
	trace_pelt_rt_tp(rq);
}

void lupos_pelt_trace_dl(struct rq *rq)
{
	trace_pelt_dl_tp(rq);
}

#ifdef CONFIG_SCHED_HW_PRESSURE
void lupos_pelt_trace_hw(struct rq *rq)
{
	trace_pelt_hw_tp(rq);
}
#endif /* CONFIG_SCHED_HW_PRESSURE */

#ifdef CONFIG_HAVE_SCHED_AVG_IRQ
/*
 * Native arch helpers may return signed long. Convert directly to the u64
 * multiplier type, without an intermediate unsigned-long truncation on 32-bit.
 */
u64 lupos_pelt_freq_capacity(int cpu)
{
	return arch_scale_freq_capacity(cpu);
}

u64 lupos_pelt_cpu_capacity(int cpu)
{
	return arch_scale_cpu_capacity(cpu);
}

void lupos_pelt_trace_irq(struct rq *rq)
{
	trace_pelt_irq_tp(rq);
}
#endif /* CONFIG_HAVE_SCHED_AVG_IRQ */
