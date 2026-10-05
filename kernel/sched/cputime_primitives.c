// SPDX-License-Identifier: GPL-2.0-only
/*
 * Native leaves and metadata for the Rust cputime owner.
 * Original owner: build_policy.c. No cputime.c inclusion or algorithm call.
 * This file is source-only and must inherit the original owner's policy before
 * admission; native configured headers preserve all diagnostics/instrumentation.
 */
#include "cputime_bindings.h"

#ifdef CONFIG_RUST_SCHED_CPUTIME
#error "SOURCE ONLY HOLD: scheduler cputime is not admitted"
#endif

#ifdef CONFIG_IRQ_TIME_ACCOUNTING
DEFINE_STATIC_KEY_FALSE(sched_clock_irqtime);
DEFINE_PER_CPU(struct irqtime, cpu_irqtime);

void lupos_cputime_irqtime_enable(void)
{
	static_branch_enable(&sched_clock_irqtime);
}

void lupos_cputime_irqtime_disable(void)
{
	static_branch_disable(&sched_clock_irqtime);
}

struct irqtime *lupos_cputime_this_irqtime(void)
{
	return this_cpu_ptr(&cpu_irqtime);
}

void lupos_cputime_u64_stats_update_begin(struct u64_stats_sync *sync)
{
	u64_stats_update_begin(sync);
}

void lupos_cputime_u64_stats_update_end(struct u64_stats_sync *sync)
{
	u64_stats_update_end(sync);
}
#endif /* CONFIG_IRQ_TIME_ACCOUNTING */

#ifdef CONFIG_PARAVIRT
struct static_key paravirt_steal_enabled;
#ifdef CONFIG_HAVE_PV_STEAL_CLOCK_GEN
/* The native static-call default is zero until a platform replaces it. */
static u64 native_steal_clock(int cpu)
{
	return 0;
}
DEFINE_STATIC_CALL(pv_steal_clock, native_steal_clock);
#endif

bool lupos_cputime_steal_enabled(void)
{
	return static_key_false(&paravirt_steal_enabled);
}

u64 lupos_cputime_steal_clock(int cpu)
{
	return paravirt_steal_clock(cpu);
}
#endif /* CONFIG_PARAVIRT */

struct kernel_cpustat *lupos_cputime_this_cpustat(void)
{
	return kcpustat_this_cpu;
}

struct rq *lupos_cputime_this_rq(void)
{
	return this_rq();
}

struct task_struct *lupos_cputime_current(void)
{
	return current;
}

struct task_struct *lupos_cputime_this_cpu_ksoftirqd(void)
{
	return this_cpu_ksoftirqd();
}

int lupos_cputime_smp_processor_id(void)
{
	return smp_processor_id();
}

u64 lupos_cputime_sched_clock(void)
{
	return sched_clock();
}

u64 lupos_cputime_sched_clock_cpu(int cpu)
{
	return sched_clock_cpu(cpu);
}

u64 lupos_cputime_tick_nsec(void)
{
	return TICK_NSEC;
}

u64 lupos_cputime_ulong_max(void)
{
	return ULONG_MAX;
}

unsigned int lupos_cputime_irq_count(void)
{
	return irq_count();
}

unsigned int lupos_cputime_hardirq_count(void)
{
	return hardirq_count();
}

bool lupos_cputime_in_serving_softirq(void)
{
	return in_serving_softirq();
}

bool lupos_cputime_irqtime_enabled(void)
{
	return irqtime_enabled();
}

bool lupos_cputime_kcpustat_idle_dyntick(void)
{
	return kcpustat_idle_dyntick();
}

int lupos_cputime_atomic_read(const atomic_t *value)
{
	return atomic_read(value);
}

int lupos_cputime_task_nice(const struct task_struct *task)
{
	return task_nice(task);
}

bool lupos_cputime_same_thread_group(struct task_struct *a, struct task_struct *b)
{
	return same_thread_group(a, b);
}

bool lupos_cputime_is_idle_task(const struct task_struct *task)
{
	return is_idle_task(task);
}

void lupos_cputime_root_add(int index, u64 value)
{
	__this_cpu_add(kernel_cpustat.cpustat[index], value);
}

void lupos_cputime_cgroup_account_cputime_field(struct task_struct *task,
					      int index, u64 value)
{
	cgroup_account_cputime_field(task, index, value);
}

void lupos_cputime_account_group_user_time(struct task_struct *task, u64 value)
{
	account_group_user_time(task, value);
}

void lupos_cputime_account_group_system_time(struct task_struct *task, u64 value)
{
	account_group_system_time(task, value);
}

void lupos_cputime_acct_account_cputime(struct task_struct *task)
{
	acct_account_cputime(task);
}

#ifdef CONFIG_SCHED_CORE
void lupos_cputime_forceidle_add(struct task_struct *task, u64 delta)
{
	__schedstat_add(task->stats.core_forceidle_sum, delta);
}
#endif

void lupos_cputime_lockdep_assert_irqs_disabled(void)
{
	lockdep_assert_irqs_disabled();
}

void lupos_cputime_rcu_read_lock(void)
{
	rcu_read_lock();
}

void lupos_cputime_rcu_read_unlock(void)
{
	rcu_read_unlock();
}

unsigned long lupos_cputime_group_read_begin(struct signal_struct *sig, int *seq)
{
	return read_seqbegin_or_lock_irqsave(&sig->stats_lock, seq);
}

bool lupos_cputime_group_read_retry(struct signal_struct *sig, int seq)
{
	return need_seqretry(&sig->stats_lock, seq);
}

void lupos_cputime_group_read_end(struct signal_struct *sig, int seq,
				  unsigned long flags)
{
	done_seqretry_irqrestore(&sig->stats_lock, seq, flags);
}

struct task_struct *lupos_cputime_thread_first(struct signal_struct *sig)
{
	/* Preserve __for_each_thread()'s configured traversal diagnostic. */
	__list_check_rcu(dummy, lockdep_is_held(&tasklist_lock), 0);
	return list_first_or_null_rcu(&sig->thread_head, struct task_struct,
				     thread_node);
}

struct task_struct *lupos_cputime_thread_next(struct signal_struct *sig,
					   struct task_struct *task)
{
	return list_next_or_null_rcu(&sig->thread_head, &task->thread_node,
				    struct task_struct, thread_node);
}

#ifndef CONFIG_64BIT
struct rq *lupos_cputime_task_rq_lock(struct task_struct *task, struct rq_flags *rf)
{
	return task_rq_lock(task, rf);
}

void lupos_cputime_task_rq_unlock(struct rq *rq, struct task_struct *task,
				  struct rq_flags *rf)
{
	task_rq_unlock(rq, task, rf);
}
#endif

#ifndef CONFIG_VIRT_CPU_ACCOUNTING_NATIVE
unsigned long lupos_cputime_prev_lock(struct prev_cputime *prev)
{
	unsigned long flags;

	raw_spin_lock_irqsave(&prev->lock, flags);
	return flags;
}

void lupos_cputime_prev_unlock(struct prev_cputime *prev, unsigned long flags)
{
	raw_spin_unlock_irqrestore(&prev->lock, flags);
}

u64 lupos_cputime_mul_u64_u64_div_u64(u64 a, u64 b, u64 divisor)
{
	return mul_u64_u64_div_u64(a, b, divisor);
}
#else
void lupos_cputime_vtime_account_hardirq(struct task_struct *task)
{
	vtime_account_hardirq(task);
}

void lupos_cputime_vtime_account_softirq(struct task_struct *task)
{
	vtime_account_softirq(task);
}

#ifndef CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE
void lupos_cputime_vtime_account_idle(struct task_struct *task)
{
	vtime_account_idle(task);
}
#endif

void lupos_cputime_arch_account_kernel(struct task_struct *task)
{
	vtime_account_kernel(task);
}

void lupos_cputime_vtime_reset(void)
{
	vtime_reset();
}
#endif /* CONFIG_VIRT_CPU_ACCOUNTING_NATIVE */

bool lupos_cputime_vtime_accounting_enabled_this_cpu(void)
{
	return vtime_accounting_enabled_this_cpu();
}

void lupos_cputime_write_seqcount_begin(seqcount_t *seq)
{
	write_seqcount_begin(seq);
}

void lupos_cputime_write_seqcount_end(seqcount_t *seq)
{
	write_seqcount_end(seq);
}

unsigned int lupos_cputime_read_seqcount_begin(const seqcount_t *seq)
{
	return read_seqcount_begin(seq);
}

bool lupos_cputime_read_seqcount_retry(const seqcount_t *seq, unsigned int start)
{
	return read_seqcount_retry(seq, start);
}

#ifdef CONFIG_VIRT_CPU_ACCOUNTING_GEN
bool lupos_cputime_vtime_accounting_enabled(void)
{
	return vtime_accounting_enabled();
}

void lupos_cputime_warn_vtime_inactive(struct vtime *vtime)
{
	WARN_ON_ONCE(vtime->state == VTIME_INACTIVE);
}

unsigned long lupos_cputime_local_irq_save(void)
{
	unsigned long flags;

	local_irq_save(flags);
	return flags;
}

void lupos_cputime_local_irq_restore(unsigned long flags)
{
	local_irq_restore(flags);
}

EXPORT_SYMBOL_GPL(vtime_guest_enter);
EXPORT_SYMBOL_GPL(vtime_guest_exit);
#endif /* CONFIG_VIRT_CPU_ACCOUNTING_GEN */

EXPORT_SYMBOL_GPL(task_cputime_adjusted);

struct kernel_cpustat *lupos_cputime_cpu_cpustat(int cpu)
{
	return &kcpustat_cpu(cpu);
}

unsigned int lupos_cputime_nr_iowait_cpu(int cpu)
{
	return nr_iowait_cpu(cpu);
}

ktime_t lupos_cputime_ktime_get(void)
{
	return ktime_get();
}

s64 lupos_cputime_ktime_to_us(ktime_t time)
{
	return ktime_to_us(time);
}

bool lupos_cputime_vtime_generic_enabled_cpu(int cpu)
{
	return vtime_generic_enabled_cpu(cpu);
}

bool lupos_cputime_vtime_generic_enabled_this_cpu(void)
{
	return vtime_generic_enabled_this_cpu();
}

/* These are configured header accessors, not calls into cputime.c. */
u64 lupos_cputime_field_default(enum cpu_usage_stat usage, int cpu)
{
	return kcpustat_field_default(usage, cpu);
}

#if defined(CONFIG_NO_HZ_COMMON) && !defined(CONFIG_HAVE_VIRT_CPU_ACCOUNTING_IDLE)
void lupos_cputime_warn_idle_dyntick(struct kernel_cpustat *kc)
{
	WARN_ON_ONCE(!kc->idle_dyntick);
}

void lupos_cputime_vtime_dyntick_start(void)
{
	vtime_dyntick_start();
}

void lupos_cputime_vtime_dyntick_stop(void)
{
	vtime_dyntick_stop();
}

EXPORT_SYMBOL_GPL(kcpustat_field_idle);
EXPORT_SYMBOL_GPL(kcpustat_field_iowait);
#endif

#ifdef CONFIG_VIRT_CPU_ACCOUNTING_GEN
enum vtime_state lupos_cputime_vtime_state_read_once(struct vtime *vtime)
{
	return READ_ONCE(vtime->state);
}

struct rq *lupos_cputime_cpu_rq(int cpu)
{
	return cpu_rq(cpu);
}

struct task_struct *lupos_cputime_rq_current(int cpu)
{
	return rcu_dereference(cpu_rq(cpu)->curr);
}

bool lupos_cputime_warn_null_task(struct task_struct *task)
{
	return WARN_ON_ONCE(!task);
}

bool lupos_cputime_warn_null_task_fetch(struct task_struct *task)
{
	return WARN_ON_ONCE(!task);
}

void lupos_cputime_warn_bad_vtime_state(void)
{
	WARN_ON_ONCE(1);
}

void lupos_cputime_cpu_relax(void)
{
	cpu_relax();
}

void lupos_cputime_cpu_fetch_default(struct kernel_cpustat *dst, int cpu)
{
	kcpustat_cpu_fetch_default(dst, cpu);
}

EXPORT_SYMBOL_GPL(kcpustat_field);
EXPORT_SYMBOL_GPL(kcpustat_cpu_fetch);
#endif /* CONFIG_VIRT_CPU_ACCOUNTING_GEN */

EXPORT_SYMBOL_GPL(get_cpu_idle_time_us);
EXPORT_SYMBOL_GPL(get_cpu_iowait_time_us);
