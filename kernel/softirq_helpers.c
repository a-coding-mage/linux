// SPDX-License-Identifier: GPL-2.0-only
/* Native compiler/architecture primitives and explicitly unmigrated header
 * dependencies. No function body from kernel/softirq.c is included or called.
 * State, queue policy, loops and descriptor initialization are all Rust-owned.
 * See NATIVE-BOUNDARY.md before describing this companion's migration status.
 */
#define pr_fmt(fmt) "softirq: " fmt
#include "softirq_bindings.h"
#define CREATE_TRACE_POINTS
#include <trace/events/irq.h>

struct task_struct *lupos_sirq_current(void) { return current; }
unsigned int lupos_sirq_preempt_count(void) { return preempt_count(); }
void lupos_sirq_preempt_set(int value) { preempt_count_set(value); }
void lupos_sirq_preempt_add_raw(unsigned int value) { __preempt_count_add(value); }
void lupos_sirq_preempt_sub_raw(unsigned int value) { __preempt_count_sub(value); }
void lupos_sirq_preempt_check_resched(void) { preempt_check_resched(); }
bool lupos_sirq_preemptible(void) { return preemptible(); }
bool lupos_sirq_need_resched(void) { return need_resched(); }
unsigned long lupos_sirq_raw_irq_save(void) { return arch_local_irq_save(); }
void lupos_sirq_arch_irq_restore(unsigned long flags) { arch_local_irq_restore(flags); }
void lupos_sirq_arch_irq_disable(void) { arch_local_irq_disable(); }
void lupos_sirq_arch_irq_enable(void) { arch_local_irq_enable(); }
bool lupos_sirq_arch_irqs_disabled(void) { return arch_irqs_disabled(); }
bool lupos_sirq_arch_irqs_disabled_flags(unsigned long flags) { return arch_irqs_disabled_flags(flags); }
void lupos_sirq_assert_irqs_enabled(void) { lockdep_assert_irqs_enabled(); }
void lupos_sirq_assert_irqs_disabled(void) { lockdep_assert_irqs_disabled(); }
unsigned int lupos_sirq_pending(void) { return local_softirq_pending(); }
void lupos_sirq_set_pending(unsigned int value) { set_softirq_pending(value); }
void lupos_sirq_or_pending(unsigned int value) { or_softirq_pending(value); }
void lupos_sirq_interrupt_state_write(unsigned long flags) { raw_cpu_write(local_interrupt_disable_state, flags); }
unsigned long lupos_sirq_interrupt_state_read(void) { return raw_cpu_read(local_interrupt_disable_state); }
struct task_struct *lupos_sirq_ksoftirqd_read(void) { return __this_cpu_read(ksoftirqd); }
unsigned long lupos_sirq_jiffies(void) { return jiffies; }
void lupos_sirq_kstat_inc(unsigned int nr) { __this_cpu_inc(kstat.softirqs[nr]); }
unsigned int lupos_sirq_cpu(void) { return smp_processor_id(); }
int lupos_sirq_next_possible(int cpu) { return cpumask_next(cpu, cpu_possible_mask); }
unsigned int lupos_sirq_nr_cpu_ids(void) { return nr_cpu_ids; }
struct tasklet_head *lupos_sirq_this_tasklet(bool high)
{ return high ? this_cpu_ptr(&rust_tasklet_hi_vec) : this_cpu_ptr(&rust_tasklet_vec); }
struct tasklet_head *lupos_sirq_cpu_tasklet(bool high, unsigned int cpu)
{ return high ? per_cpu_ptr(&rust_tasklet_hi_vec, cpu) : per_cpu_ptr(&rust_tasklet_vec, cpu); }
bool lupos_sirq_force_irqthreads(void) { return force_irqthreads(); }
#ifndef CONFIG_PREEMPT_RT
void lupos_sirq_own_stack(void) { do_softirq_own_stack(); }
#endif
void lupos_sirq_ct_enter(void) { ct_irq_enter(); }
void lupos_sirq_ct_exit(void) { ct_irq_exit(); }
bool lupos_sirq_tick_full(unsigned int cpu) { return tick_nohz_full_cpu(cpu); }
bool lupos_sirq_core_idle(int cpu) { return sched_core_idle_cpu(cpu); }
void lupos_sirq_tick_enter(void) { tick_irq_enter(); }
#ifdef CONFIG_NO_HZ_COMMON
void lupos_sirq_tick_exit(void) { tick_nohz_irq_exit(); }
#endif
void lupos_sirq_hrtimer_rearm(void) { hrtimer_rearm_deferred(); }
void lupos_sirq_rcu_qs(void) { rcu_softirq_qs(); }
void lupos_sirq_rcu_lock(void) { rcu_read_lock(); }
void lupos_sirq_rcu_unlock(void) { rcu_read_unlock(); }
void lupos_sirq_migrate_disable(void) { migrate_disable(); }
void lupos_sirq_migrate_enable(void) { migrate_enable(); }
void lupos_sirq_cond_resched(void) { cond_resched(); }
void lupos_sirq_softirqs_off(unsigned long ip) { lockdep_softirqs_off(ip); }
void lupos_sirq_softirqs_on(unsigned long ip) { lockdep_softirqs_on(ip); }
void lupos_sirq_softirq_enter(void) { lockdep_softirq_enter(); }
void lupos_sirq_softirq_exit(void) { lockdep_softirq_exit(); }
bool lupos_sirq_hardirq_context(void) { return lockdep_hardirq_context(); }
void lupos_sirq_hardirq_enter(void) { lockdep_hardirq_enter(); }
void lupos_sirq_hardirq_exit(void) { lockdep_hardirq_exit(); }
void lupos_sirq_preempt_off(unsigned long caller, unsigned long parent) { trace_preempt_off(caller, parent); }
void lupos_sirq_preempt_on(unsigned long caller, unsigned long parent) { trace_preempt_on(caller, parent); }
void lupos_sirq_trace_entry(unsigned int nr) { trace_softirq_entry(nr); }
void lupos_sirq_trace_exit(unsigned int nr) { trace_softirq_exit(nr); }
void lupos_sirq_trace_raise(unsigned int nr) { trace_softirq_raise(nr); }
void lupos_sirq_tasklet_entry(struct tasklet_struct *t, void *callback) { trace_tasklet_entry(t, callback); }
void lupos_sirq_tasklet_exit(struct tasklet_struct *t, void *callback) { trace_tasklet_exit(t, callback); }
bool lupos_sirq_test_set_bit(unsigned int bit, unsigned long *word) { return test_and_set_bit(bit, word); }
bool lupos_sirq_test_bit(unsigned int bit, const unsigned long *word) { return test_bit(bit, word); }
bool lupos_sirq_clear_wake_bit(unsigned int bit, unsigned long *word) { return test_and_clear_wake_up_bit(bit, word); }
void lupos_sirq_clear_and_wake_bit(unsigned int bit, unsigned long *word) { clear_and_wake_up_bit(bit, word); }
void lupos_sirq_wait_bit(unsigned long *word, unsigned int bit) { wait_on_bit(word, bit, TASK_UNINTERRUPTIBLE); }
void lupos_sirq_wait_bit_lock(unsigned long *word, unsigned int bit) { wait_on_bit_lock(word, bit, TASK_UNINTERRUPTIBLE); }
int lupos_sirq_atomic_read(const atomic_t *value) { return atomic_read(value); }
void lupos_sirq_atomic_set(atomic_t *value, int count) { atomic_set(value, count); }
void lupos_sirq_relax(void) { cpu_relax(); }
void lupos_sirq_warn_hardirq(bool condition, unsigned int site)
{
	/* Retain independent WARN_ON_ONCE state at each original call site. */
	switch (site) {
	case 0: WARN_ON_ONCE(condition); break; /* disable_ip */
	case 1: WARN_ON_ONCE(condition); break; /* enable_ip */
	case 2: WARN_ON_ONCE(condition); break; /* _local_bh_enable */
	}
}
void lupos_sirq_warn_count_negative(bool condition, unsigned int site)
{
	if (site == 0) { WARN_ON_ONCE(condition); }
	else { WARN_ON_ONCE(condition); }
}
void lupos_sirq_warn_interrupt(bool condition) { WARN_ON_ONCE(condition); }
void lupos_sirq_warn_flush(bool condition) { WARN_ON_ONCE(condition); }
void lupos_sirq_debug_warn(bool condition) { DEBUG_LOCKS_WARN_ON(condition); }
void lupos_sirq_tasklet_warn(struct tasklet_struct *t)
{
	WARN_ONCE(1, "tasklet SCHED state not set: %s %pS\n",
		t->use_callback ? "callback" : "func",
		t->use_callback ? (void *)t->callback : (void *)t->func);
}
void lupos_sirq_kill_notice(void) { pr_notice("Attempt to kill tasklet from interrupt\n"); }
void lupos_sirq_count_error(unsigned int nr, const char *name,
	void (*action)(void), unsigned int before, unsigned int after)
{
	pr_err("huh, entered softirq %u %s %p with preempt_count %08x, exited with %08x?\n",
		nr, name, action, before, after);
}
void lupos_sirq_bug(bool condition) { BUG_ON(condition); }
int lupos_sirq_cpuhp(int (*dead)(unsigned int))
{ return cpuhp_setup_state_nocalls(CPUHP_SOFTIRQ_DEAD, "softirq:dead", NULL, dead); }
#ifdef CONFIG_PREEMPT_RT
int lupos_sirq_ctrl_read(void) { return this_cpu_read(rust_softirq_ctrl.cnt); }
int lupos_sirq_ctrl_read_raw(void) { return __this_cpu_read(rust_softirq_ctrl.cnt); }
int lupos_sirq_ctrl_add_return(unsigned int count) { return this_cpu_add_return(rust_softirq_ctrl.cnt, count); }
int lupos_sirq_ctrl_sub_return(unsigned int count) { return this_cpu_sub_return(rust_softirq_ctrl.cnt, count); }
void lupos_sirq_ctrl_add(unsigned int count) { this_cpu_add(rust_softirq_ctrl.cnt, count); }
void lupos_sirq_ctrl_sub(unsigned int count) { this_cpu_sub(rust_softirq_ctrl.cnt, count); }
spinlock_t *lupos_sirq_ctrl_lock(void) { return this_cpu_ptr(&rust_softirq_ctrl.lock); }
struct tasklet_sync_callback *lupos_sirq_sync_callback(void)
{ return this_cpu_ptr(&rust_tasklet_sync_callback); }
void lupos_sirq_spin_lock(spinlock_t *lock) { spin_lock(lock); }
void lupos_sirq_spin_unlock(spinlock_t *lock) { spin_unlock(lock); }
void lupos_sirq_atomic_inc(atomic_t *value) { atomic_inc(value); }
void lupos_sirq_atomic_dec(atomic_t *value) { atomic_dec(value); }
void lupos_sirq_bh_acquire(unsigned long ip)
{
#ifdef CONFIG_DEBUG_LOCK_ALLOC
	lock_acquire(&bh_lock_map, 0, 0, 2, 1, NULL, ip);
#endif
}
void lupos_sirq_bh_release(unsigned long ip)
{
#ifdef CONFIG_DEBUG_LOCK_ALLOC
	lock_release(&bh_lock_map, ip);
#endif
}
#endif
#ifdef CONFIG_IRQ_FORCED_THREADING
struct task_struct *lupos_sirq_ktimerd_read(void) { return __this_cpu_read(ktimerd); }
unsigned int lupos_sirq_timer_pending(void) { return __this_cpu_read(pending_timer_softirq); }
void lupos_sirq_timer_clear(void) { __this_cpu_write(pending_timer_softirq, 0); }
void lupos_sirq_timer_or(unsigned long value) { __this_cpu_or(pending_timer_softirq, value); }
#endif

/* Native compiler metadata is emitted adjacent to the original softirq.o
 * archive slot. This TU defines no daemon, tasklet, BH, IRQ or dispatch body. */
extern int __init lupos_spawn_ksoftirqd(void);
early_initcall(lupos_spawn_ksoftirqd);
EXPORT_SYMBOL(_local_interrupt_disable);
EXPORT_SYMBOL(_local_interrupt_enable);
#ifdef CONFIG_TRACE_IRQFLAGS
EXPORT_PER_CPU_SYMBOL_GPL(hardirqs_enabled);
EXPORT_PER_CPU_SYMBOL_GPL(hardirq_context);
#endif
#if defined(CONFIG_PREEMPT_RT) || defined(CONFIG_TRACE_IRQFLAGS)
EXPORT_SYMBOL(__local_bh_disable_ip);
#endif
EXPORT_SYMBOL(__local_bh_enable_ip);
#ifndef CONFIG_PREEMPT_RT
EXPORT_SYMBOL(_local_bh_enable);
#elif defined(CONFIG_DEBUG_LOCK_ALLOC)
EXPORT_SYMBOL_GPL(bh_lock_map);
#endif
EXPORT_SYMBOL(__tasklet_schedule);
EXPORT_SYMBOL(__tasklet_hi_schedule);
EXPORT_SYMBOL(tasklet_setup);
EXPORT_SYMBOL(tasklet_init);
EXPORT_SYMBOL(tasklet_kill);
#if defined(CONFIG_SMP) || defined(CONFIG_PREEMPT_RT)
EXPORT_SYMBOL(tasklet_unlock_spin_wait);
EXPORT_SYMBOL_GPL(tasklet_unlock);
EXPORT_SYMBOL_GPL(tasklet_unlock_wait);
#endif
