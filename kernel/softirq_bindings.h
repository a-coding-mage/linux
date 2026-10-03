/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SOFTIRQ_BINDINGS_H
#define LUPOS_SOFTIRQ_BINDINGS_H
#define INSTANTIATE_EXPORTED_INTERRUPT_DISABLE
#include <linux/export.h>
#include <linux/kernel_stat.h>
#include <linux/interrupt.h>
#include <linux/init.h>
#include <linux/local_lock.h>
#include <linux/mm.h>
#include <linux/notifier.h>
#include <linux/percpu.h>
#include <linux/cpu.h>
#include <linux/freezer.h>
#include <linux/kthread.h>
#include <linux/rcupdate.h>
#include <linux/ftrace.h>
#include <linux/smp.h>
#include <linux/smpboot.h>
#include <linux/tick.h>
#include <linux/irq.h>
#include <linux/wait_bit.h>
#include <linux/workqueue.h>
#include <asm/softirq_stack.h>

/* These three private types are copied, without layout changes, from softirq.c. */
struct tasklet_head {
	struct tasklet_struct *head;
	struct tasklet_struct **tail;
};
#ifdef CONFIG_PREEMPT_RT
struct softirq_ctrl { local_lock_t lock; int cnt; };
struct tasklet_sync_callback { spinlock_t cb_lock; atomic_t cb_waiters; };
DECLARE_PER_CPU(struct softirq_ctrl, rust_softirq_ctrl);
DECLARE_PER_CPU(struct tasklet_sync_callback, rust_tasklet_sync_callback);
#ifdef CONFIG_DEBUG_LOCK_ALLOC
extern struct lockdep_map bh_lock_map;
#endif
#endif
DECLARE_PER_CPU(struct tasklet_head, rust_tasklet_vec);
DECLARE_PER_CPU(struct tasklet_head, rust_tasklet_hi_vec);

/* Native declaration-derived alignment, no parallel Rust kernel layouts. */
extern struct softirq_action rust_softirq_vec[NR_SOFTIRQS] __cacheline_aligned_in_smp;
struct rust_softirq_vec_storage { struct softirq_action value[NR_SOFTIRQS]; }
	__aligned(__alignof__(rust_softirq_vec));
struct rust_softirq_names_storage { const char *value[NR_SOFTIRQS]; };
enum {
	RUST_SIRQ_VEC_ALIGN = __alignof__(rust_softirq_vec),
	RUST_SIRQ_VEC_OFFSET = offsetof(struct rust_softirq_vec_storage, value),
	RUST_SIRQ_NAMES_OFFSET = offsetof(struct rust_softirq_names_storage, value),
	RUST_SIRQ_SOFTIRQ_OFFSET = SOFTIRQ_OFFSET,
	RUST_SIRQ_SOFTIRQ_DISABLE_OFFSET = SOFTIRQ_DISABLE_OFFSET,
	RUST_SIRQ_SOFTIRQ_MASK = SOFTIRQ_MASK,
	RUST_SIRQ_HARDIRQ_OFFSET = HARDIRQ_OFFSET,
	RUST_SIRQ_HARDIRQ_MASK = HARDIRQ_MASK,
	RUST_SIRQ_NMI_MASK = NMI_MASK,
	RUST_SIRQ_HARDIRQ_DISABLE_MASK = HARDIRQ_DISABLE_MASK,
	RUST_SIRQ_NR_IRQS_LEGACY = NR_IRQS_LEGACY,
	RUST_SIRQ_LD_WAIT_FREE = LD_WAIT_FREE,
	RUST_SIRQ_LD_WAIT_CONFIG = LD_WAIT_CONFIG,
	RUST_SIRQ_LD_WAIT_SPIN = LD_WAIT_SPIN,
	RUST_SIRQ_LD_LOCK_PERCPU = LD_LOCK_PERCPU,
	RUST_SIRQ_HZ = HZ,
};
#ifdef __ARCH_IRQ_STAT
enum { RUST_SIRQ_ARCH_IRQ_STAT = 1 };
#else
enum { RUST_SIRQ_ARCH_IRQ_STAT = 0 };
struct rust_softirq_irq_stat_storage { irq_cpustat_t value; } ____cacheline_aligned;
#endif
#ifdef __ARCH_IRQ_EXIT_IRQS_DISABLED
enum { RUST_SIRQ_ARCH_EXIT_IRQS_DISABLED = 1 };
#else
enum { RUST_SIRQ_ARCH_EXIT_IRQS_DISABLED = 0 };
#endif
#ifdef CONFIG_PREEMPT_RT
enum {
	RUST_SIRQ_RAW_LOCK_SIZE = sizeof(raw_spinlock_t),
	RUST_SIRQ_RAW_LOCK_ALIGN = __alignof__(raw_spinlock_t),
	RUST_SIRQ_RT_LOCK_OFFSET = offsetof(spinlock_t, lock),
	RUST_SIRQ_LOCAL_LOCK_SIZE = sizeof(local_lock_t),
	RUST_SIRQ_LOCAL_LOCK_ALIGN = __alignof__(local_lock_t),
};
#endif

/* Compiler/architecture/atomic operations and explicit header dependencies.
 * See NATIVE-BOUNDARY.md: not every helper is merely an ABI primitive. */
struct task_struct *lupos_sirq_current(void);
unsigned int lupos_sirq_preempt_count(void);
void lupos_sirq_preempt_set(int value);
void lupos_sirq_preempt_add_raw(unsigned int value);
void lupos_sirq_preempt_sub_raw(unsigned int value);
void lupos_sirq_preempt_check_resched(void);
bool lupos_sirq_preemptible(void);
bool lupos_sirq_need_resched(void);
unsigned long lupos_sirq_raw_irq_save(void);
void lupos_sirq_arch_irq_restore(unsigned long flags);
void lupos_sirq_arch_irq_disable(void);
void lupos_sirq_arch_irq_enable(void);
bool lupos_sirq_arch_irqs_disabled(void);
bool lupos_sirq_arch_irqs_disabled_flags(unsigned long flags);
void lupos_sirq_assert_irqs_enabled(void);
void lupos_sirq_assert_irqs_disabled(void);
unsigned int lupos_sirq_pending(void);
void lupos_sirq_set_pending(unsigned int value);
void lupos_sirq_or_pending(unsigned int value);
void lupos_sirq_interrupt_state_write(unsigned long flags);
unsigned long lupos_sirq_interrupt_state_read(void);
struct task_struct *lupos_sirq_ksoftirqd_read(void);
unsigned long lupos_sirq_jiffies(void);
void lupos_sirq_kstat_inc(unsigned int nr);
unsigned int lupos_sirq_cpu(void);
int lupos_sirq_next_possible(int cpu);
unsigned int lupos_sirq_nr_cpu_ids(void);
struct tasklet_head *lupos_sirq_this_tasklet(bool high);
struct tasklet_head *lupos_sirq_cpu_tasklet(bool high, unsigned int cpu);
bool lupos_sirq_force_irqthreads(void);
void lupos_sirq_own_stack(void);
void lupos_sirq_ct_enter(void);
void lupos_sirq_ct_exit(void);
bool lupos_sirq_tick_full(unsigned int cpu);
bool lupos_sirq_core_idle(int cpu);
void lupos_sirq_tick_enter(void);
void lupos_sirq_tick_exit(void);
void lupos_sirq_hrtimer_rearm(void);
void lupos_sirq_rcu_qs(void);
void lupos_sirq_rcu_lock(void);
void lupos_sirq_rcu_unlock(void);
void lupos_sirq_migrate_disable(void);
void lupos_sirq_migrate_enable(void);
void lupos_sirq_cond_resched(void);
void lupos_sirq_softirqs_off(unsigned long ip);
void lupos_sirq_softirqs_on(unsigned long ip);
void lupos_sirq_softirq_enter(void);
void lupos_sirq_softirq_exit(void);
bool lupos_sirq_hardirq_context(void);
void lupos_sirq_hardirq_enter(void);
void lupos_sirq_hardirq_exit(void);
void lupos_sirq_preempt_off(unsigned long caller, unsigned long parent);
void lupos_sirq_preempt_on(unsigned long caller, unsigned long parent);
void lupos_sirq_trace_entry(unsigned int nr);
void lupos_sirq_trace_exit(unsigned int nr);
void lupos_sirq_trace_raise(unsigned int nr);
void lupos_sirq_tasklet_entry(struct tasklet_struct *t, void *callback);
void lupos_sirq_tasklet_exit(struct tasklet_struct *t, void *callback);
bool lupos_sirq_test_set_bit(unsigned int bit, unsigned long *word);
bool lupos_sirq_test_bit(unsigned int bit, const unsigned long *word);
bool lupos_sirq_clear_wake_bit(unsigned int bit, unsigned long *word);
void lupos_sirq_clear_and_wake_bit(unsigned int bit, unsigned long *word);
void lupos_sirq_wait_bit(unsigned long *word, unsigned int bit);
void lupos_sirq_wait_bit_lock(unsigned long *word, unsigned int bit);
int lupos_sirq_atomic_read(const atomic_t *value);
void lupos_sirq_atomic_set(atomic_t *value, int count);
void lupos_sirq_relax(void);
void lupos_sirq_warn_hardirq(bool condition, unsigned int site);
void lupos_sirq_warn_count_negative(bool condition, unsigned int site);
void lupos_sirq_warn_interrupt(bool condition);
void lupos_sirq_warn_flush(bool condition);
void lupos_sirq_debug_warn(bool condition);
void lupos_sirq_tasklet_warn(struct tasklet_struct *t);
void lupos_sirq_kill_notice(void);
void lupos_sirq_count_error(unsigned int nr, const char *name,
	void (*action)(void), unsigned int before, unsigned int after);
void lupos_sirq_bug(bool condition);
int lupos_sirq_cpuhp(int (*dead)(unsigned int));
#ifdef CONFIG_PREEMPT_RT
int lupos_sirq_ctrl_read(void);
int lupos_sirq_ctrl_read_raw(void);
int lupos_sirq_ctrl_add_return(unsigned int count);
int lupos_sirq_ctrl_sub_return(unsigned int count);
void lupos_sirq_ctrl_add(unsigned int count);
void lupos_sirq_ctrl_sub(unsigned int count);
spinlock_t *lupos_sirq_ctrl_lock(void);
struct tasklet_sync_callback *lupos_sirq_sync_callback(void);
void lupos_sirq_spin_lock(spinlock_t *lock);
void lupos_sirq_spin_unlock(spinlock_t *lock);
void lupos_sirq_atomic_inc(atomic_t *value);
void lupos_sirq_atomic_dec(atomic_t *value);
void lupos_sirq_bh_acquire(unsigned long ip);
void lupos_sirq_bh_release(unsigned long ip);
#endif
#ifdef CONFIG_IRQ_FORCED_THREADING
struct task_struct *lupos_sirq_ktimerd_read(void);
unsigned int lupos_sirq_timer_pending(void);
void lupos_sirq_timer_clear(void);
void lupos_sirq_timer_or(unsigned long value);
#endif
#include "softirq_layout.h"
#endif
