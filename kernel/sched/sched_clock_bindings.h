/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_CLOCK_BINDINGS_H
#define LUPOS_SCHED_CLOCK_BINDINGS_H
#include <linux/sched/clock.h>
#include <linux/sched_clock.h>
#include "sched.h"

static const unsigned long LUPOS_CLOCK_INITIAL_JIFFIES = INITIAL_JIFFIES;
static const u64 LUPOS_CLOCK_NSEC_PER_SEC = NSEC_PER_SEC;
static const unsigned long LUPOS_CLOCK_HZ = HZ;
static const u64 LUPOS_CLOCK_TICK_NSEC = TICK_NSEC;

/* Exact private record from clock.c; C is responsible for per-CPU placement,
 * Rust is responsible for every initialization/update/copy decision. */
#ifdef CONFIG_HAVE_UNSTABLE_SCHED_CLOCK
struct sched_clock_data {
	u64 tick_raw;
	u64 tick_gtod;
	u64 clock;
};
static const size_t LUPOS_CLOCK_SCD_SIZE = sizeof(struct sched_clock_data);
static const size_t LUPOS_CLOCK_SCD_ALIGN = __alignof__(struct sched_clock_data);
static const size_t LUPOS_CLOCK_SCD_RAW = offsetof(struct sched_clock_data, tick_raw);
static const size_t LUPOS_CLOCK_SCD_GTOD = offsetof(struct sched_clock_data, tick_gtod);
static const size_t LUPOS_CLOCK_SCD_CLOCK = offsetof(struct sched_clock_data, clock);
struct sched_clock_data *lupos_clock_this_scd(void);
struct sched_clock_data *lupos_clock_cpu_scd(int cpu);
bool lupos_clock_stable_branch(void);
void lupos_clock_stable_enable(void);
void lupos_clock_stable_disable(void);
void lupos_clock_tick_dep_set(void);
void lupos_clock_tick_dep_clear(void);
void lupos_clock_schedule_work(void);
int lupos_clock_running_count(void);
void lupos_clock_disable_irqtime(void);
int lupos_clock_next_possible(int cpu);
unsigned int lupos_clock_nr_cpu_ids(void);
bool lupos_clock_raw_try_cmpxchg64(u64 *ptr, u64 *old, u64 new);
bool lupos_clock_try_cmpxchg64(u64 *ptr, u64 *old, u64 new);
u64 lupos_clock_cmpxchg64(u64 *ptr, u64 old, u64 new);
void lupos_clock_print_stable(u64 gtod, u64 gtod_offset, u64 raw, u64 raw_offset);
void lupos_clock_print_unstable(u64 gtod, u64 gtod_offset, u64 raw, u64 raw_offset);
void lupos_clock_work(struct work_struct *work);
u64 lupos_clock_ktime_get_ns(void);
int lupos_clock_init_late(void);
#else
void lupos_clock_generic_init(void);
#endif
unsigned long lupos_clock_jiffies(void);
u64 lupos_clock_default_sched_clock(void);
u64 lupos_clock_default_running_clock(void);
u64 lupos_clock_sched_clock_noinstr(void);
bool lupos_clock_running_branch(void);
void lupos_clock_running_inc(void);
void lupos_clock_irq_disable(void);
void lupos_clock_irq_enable(void);
unsigned long lupos_clock_irq_save(void);
void lupos_clock_irq_restore(unsigned long flags);
void lupos_clock_preempt_disable(void);
void lupos_clock_preempt_enable(void);
void lupos_clock_preempt_disable_notrace(void);
void lupos_clock_preempt_enable_notrace(void);
void lupos_clock_smp_mb(void);
void lupos_clock_assert_irqs_disabled(void);
int lupos_clock_cpu(void);
#endif
