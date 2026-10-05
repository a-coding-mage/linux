/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_CPUTIME_BINDINGS_H
#define LUPOS_CPUTIME_BINDINGS_H

/* Configured native headers are the sole authority for types and constants. */
#include <linux/sched/clock.h>
#include <linux/sched/cputime.h>
#include <linux/tsacct_kern.h>
#include "sched.h"
#ifdef CONFIG_VIRT_CPU_ACCOUNTING_NATIVE
#include <asm/cputime.h>
#endif

#define LUPOS_CPUTIME_PF_VCPU PF_VCPU
#define LUPOS_CPUTIME_HARDIRQ_MASK HARDIRQ_MASK
#define LUPOS_CPUTIME_HARDIRQ_OFFSET HARDIRQ_OFFSET
#define LUPOS_CPUTIME_SOFTIRQ_OFFSET SOFTIRQ_OFFSET
#define LUPOS_CPUTIME_NSEC_PER_USEC NSEC_PER_USEC
#define LUPOS_CPUTIME_EAGAIN EAGAIN

struct kernel_cpustat *lupos_cputime_this_cpustat(void);
struct rq *lupos_cputime_this_rq(void);
struct task_struct *lupos_cputime_current(void);
struct task_struct *lupos_cputime_this_cpu_ksoftirqd(void);
int lupos_cputime_smp_processor_id(void);
u64 lupos_cputime_sched_clock(void);
u64 lupos_cputime_sched_clock_cpu(int cpu);
u64 lupos_cputime_tick_nsec(void);
u64 lupos_cputime_ulong_max(void);
unsigned int lupos_cputime_irq_count(void);
unsigned int lupos_cputime_hardirq_count(void);
bool lupos_cputime_in_serving_softirq(void);
bool lupos_cputime_irqtime_enabled(void);
bool lupos_cputime_kcpustat_idle_dyntick(void);
int lupos_cputime_atomic_read(const atomic_t *value);
int lupos_cputime_task_nice(const struct task_struct *task);
bool lupos_cputime_same_thread_group(struct task_struct *a, struct task_struct *b);
bool lupos_cputime_is_idle_task(const struct task_struct *task);
void lupos_cputime_root_add(int index, u64 value);
void lupos_cputime_cgroup_account_cputime_field(struct task_struct *task,
					      int index, u64 value);
void lupos_cputime_account_group_user_time(struct task_struct *task, u64 value);
void lupos_cputime_account_group_system_time(struct task_struct *task, u64 value);
void lupos_cputime_acct_account_cputime(struct task_struct *task);
void lupos_cputime_lockdep_assert_irqs_disabled(void);
void lupos_cputime_rcu_read_lock(void);
void lupos_cputime_rcu_read_unlock(void);
unsigned long lupos_cputime_group_read_begin(struct signal_struct *sig, int *seq);
bool lupos_cputime_group_read_retry(struct signal_struct *sig, int seq);
void lupos_cputime_group_read_end(struct signal_struct *sig, int seq,
				  unsigned long flags);
struct task_struct *lupos_cputime_thread_first(struct signal_struct *sig);
struct task_struct *lupos_cputime_thread_next(struct signal_struct *sig,
					   struct task_struct *task);
bool lupos_cputime_vtime_accounting_enabled_this_cpu(void);
void lupos_cputime_write_seqcount_begin(seqcount_t *seq);
void lupos_cputime_write_seqcount_end(seqcount_t *seq);
unsigned int lupos_cputime_read_seqcount_begin(const seqcount_t *seq);
bool lupos_cputime_read_seqcount_retry(const seqcount_t *seq, unsigned int start);
struct kernel_cpustat *lupos_cputime_cpu_cpustat(int cpu);
unsigned int lupos_cputime_nr_iowait_cpu(int cpu);
ktime_t lupos_cputime_ktime_get(void);
s64 lupos_cputime_ktime_to_us(ktime_t time);
bool lupos_cputime_vtime_generic_enabled_cpu(int cpu);
bool lupos_cputime_vtime_generic_enabled_this_cpu(void);
u64 lupos_cputime_field_default(enum cpu_usage_stat usage, int cpu);

#if defined(CONFIG_NO_HZ_COMMON) && !defined(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE)
void lupos_cputime_warn_idle_dyntick(struct kernel_cpustat *kc);
void lupos_cputime_vtime_dyntick_start(void);
void lupos_cputime_vtime_dyntick_stop(void);
#endif

#ifdef CONFIG_IRQ_TIME_ACCOUNTING
void lupos_cputime_irqtime_enable(void);
void lupos_cputime_irqtime_disable(void);
struct irqtime *lupos_cputime_this_irqtime(void);
void lupos_cputime_u64_stats_update_begin(struct u64_stats_sync *sync);
void lupos_cputime_u64_stats_update_end(struct u64_stats_sync *sync);
#endif
#ifdef CONFIG_SCHED_CORE
void lupos_cputime_forceidle_add(struct task_struct *task, u64 delta);
#endif
#ifdef CONFIG_PARAVIRT
bool lupos_cputime_steal_enabled(void);
u64 lupos_cputime_steal_clock(int cpu);
#endif
#ifndef CONFIG_64BIT
struct rq *lupos_cputime_task_rq_lock(struct task_struct *task, struct rq_flags *rf);
void lupos_cputime_task_rq_unlock(struct rq *rq, struct task_struct *task,
				  struct rq_flags *rf);
#endif
#ifndef CONFIG_VIRT_CPU_ACCOUNTING_NATIVE
unsigned long lupos_cputime_prev_lock(struct prev_cputime *prev);
void lupos_cputime_prev_unlock(struct prev_cputime *prev, unsigned long flags);
u64 lupos_cputime_mul_u64_u64_div_u64(u64 a, u64 b, u64 divisor);
#else
void lupos_cputime_vtime_account_hardirq(struct task_struct *task);
void lupos_cputime_vtime_account_softirq(struct task_struct *task);
#ifndef CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE
void lupos_cputime_vtime_account_idle(struct task_struct *task);
#endif
void lupos_cputime_arch_account_kernel(struct task_struct *task);
void lupos_cputime_vtime_reset(void);
#endif
#ifdef CONFIG_VIRT_CPU_ACCOUNTING_GEN
bool lupos_cputime_vtime_accounting_enabled(void);
void lupos_cputime_warn_vtime_inactive(struct vtime *vtime);
unsigned long lupos_cputime_local_irq_save(void);
void lupos_cputime_local_irq_restore(unsigned long flags);
enum vtime_state lupos_cputime_vtime_state_read_once(struct vtime *vtime);
struct rq *lupos_cputime_cpu_rq(int cpu);
struct task_struct *lupos_cputime_rq_current(int cpu);
bool lupos_cputime_warn_null_task(struct task_struct *task);
bool lupos_cputime_warn_null_task_fetch(struct task_struct *task);
void lupos_cputime_warn_bad_vtime_state(void);
void lupos_cputime_cpu_relax(void);
void lupos_cputime_cpu_fetch_default(struct kernel_cpustat *dst, int cpu);
#endif

#endif /* LUPOS_CPUTIME_BINDINGS_H */
