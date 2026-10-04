// SPDX-License-Identifier: GPL-2.0-only
/* Compiler/architecture/static-key/per-CPU metadata boundary. No clock.c
 * algorithm body is compiled here. All clock decisions execute in clock.rs. */
#include "sched_clock_bindings.h"
static DEFINE_STATIC_KEY_FALSE(sched_clock_running);
notrace unsigned long long __weak sched_clock(void)
{ return lupos_clock_default_sched_clock(); }
EXPORT_SYMBOL_GPL(sched_clock);
notrace u64 __weak running_clock(void)
{ return lupos_clock_default_running_clock(); }
unsigned long notrace lupos_clock_jiffies(void) { return jiffies; }
noinstr u64 lupos_clock_sched_clock_noinstr(void) { return sched_clock_noinstr(); }
noinstr bool lupos_clock_running_branch(void)
{ return static_branch_likely(&sched_clock_running); }
void lupos_clock_running_inc(void) { static_branch_inc(&sched_clock_running); }
void notrace lupos_clock_irq_disable(void) { local_irq_disable(); }
void notrace lupos_clock_irq_enable(void) { local_irq_enable(); }
unsigned long notrace lupos_clock_irq_save(void)
{ unsigned long flags; local_irq_save(flags); return flags; }
void notrace lupos_clock_irq_restore(unsigned long flags) { local_irq_restore(flags); }
void notrace lupos_clock_preempt_disable(void) { preempt_disable(); }
void notrace lupos_clock_preempt_enable(void) { preempt_enable(); }
void notrace lupos_clock_preempt_disable_notrace(void) { preempt_disable_notrace(); }
void notrace lupos_clock_preempt_enable_notrace(void) { preempt_enable_notrace(); }
void notrace lupos_clock_smp_mb(void) { smp_mb(); }
void notrace lupos_clock_assert_irqs_disabled(void) { lockdep_assert_irqs_disabled(); }
int notrace lupos_clock_cpu(void) { return smp_processor_id(); }
#ifdef CONFIG_HAVE_UNSTABLE_SCHED_CLOCK
u64 notrace lupos_clock_ktime_get_ns(void) { return ktime_get_ns(); }
static DEFINE_STATIC_KEY_FALSE(__sched_clock_stable);
static DEFINE_PER_CPU_SHARED_ALIGNED(struct sched_clock_data, sched_clock_data);
static DECLARE_WORK(sched_clock_work, lupos_clock_work);
noinstr struct sched_clock_data *lupos_clock_this_scd(void)
{ return this_cpu_ptr(&sched_clock_data); }
notrace struct sched_clock_data *lupos_clock_cpu_scd(int cpu)
{ return &per_cpu(sched_clock_data, cpu); }
noinstr bool lupos_clock_stable_branch(void)
{ return static_branch_likely(&__sched_clock_stable); }
void notrace lupos_clock_stable_enable(void) { static_branch_enable(&__sched_clock_stable); }
void notrace lupos_clock_stable_disable(void) { static_branch_disable(&__sched_clock_stable); }
void notrace lupos_clock_tick_dep_set(void) { tick_dep_set(TICK_DEP_BIT_CLOCK_UNSTABLE); }
void notrace lupos_clock_tick_dep_clear(void) { tick_dep_clear(TICK_DEP_BIT_CLOCK_UNSTABLE); }
void notrace lupos_clock_schedule_work(void) { schedule_work(&sched_clock_work); }
int notrace lupos_clock_running_count(void) { return static_key_count(&sched_clock_running.key); }
void notrace lupos_clock_disable_irqtime(void) { disable_sched_clock_irqtime(); }
int notrace lupos_clock_next_possible(int cpu) { return cpumask_next(cpu, cpu_possible_mask); }
unsigned int notrace lupos_clock_nr_cpu_ids(void) { return nr_cpu_ids; }
noinstr bool lupos_clock_raw_try_cmpxchg64(u64 *ptr, u64 *old, u64 new)
{ return raw_try_cmpxchg64(ptr, old, new); }
bool notrace lupos_clock_try_cmpxchg64(u64 *ptr, u64 *old, u64 new)
{ return try_cmpxchg64(ptr, old, new); }
u64 notrace lupos_clock_cmpxchg64(u64 *ptr, u64 old, u64 new)
{ return cmpxchg64(ptr, old, new); }
void notrace lupos_clock_print_stable(u64 gtod, u64 gtod_offset, u64 raw, u64 raw_offset)
{
	printk(KERN_INFO "sched_clock: Marking stable (%lld, %lld)->(%lld, %lld)\n",
		gtod, gtod_offset, raw, raw_offset);
}
void notrace lupos_clock_print_unstable(u64 gtod, u64 gtod_offset, u64 raw, u64 raw_offset)
{
	printk(KERN_WARNING "TSC found unstable after boot, most likely due to broken BIOS. Use 'tsc=unstable'.\n");
	printk(KERN_INFO "sched_clock: Marking unstable (%lld, %lld)<-(%lld, %lld)\n",
		gtod, gtod_offset, raw, raw_offset);
}
late_initcall(lupos_clock_init_late);
EXPORT_SYMBOL_GPL(local_clock);
EXPORT_SYMBOL_GPL(sched_clock_cpu);
EXPORT_SYMBOL_GPL(sched_clock_idle_sleep_event);
EXPORT_SYMBOL_GPL(sched_clock_idle_wakeup_event);
#else
void __init lupos_clock_generic_init(void) { generic_sched_clock_init(); }
#endif
