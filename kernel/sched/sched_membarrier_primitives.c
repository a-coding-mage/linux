// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Native storage, macro/inline primitives and syscall/initcall ABI only.
 * The existing Rust continuation owns selection, registration and dispatch.
 * Native policy origin: build_utility.c. No original C owner is included.
 */
#include "sched_membarrier_bindings.h"

#error "SOURCE ONLY HOLD: scheduler membarrier is not admitted"

static DEFINE_MUTEX(membarrier_ipi_mutex);
static DEFINE_PER_CPU(struct mutex, membarrier_cpu_mutexes);

static int __init sched_membarrier_init_native(void)
{
	return lupos_membarrier_init();
}
core_initcall(sched_membarrier_init_native);

/* Preserve native syscall wrappers, metadata, tracing and error injection. */
SYSCALL_DEFINE3(membarrier, int, cmd, unsigned int, flags, int, cpu_id)
{
	return lupos_membarrier_syscall(cmd, flags, cpu_id);
}

void __init lupos_membarrier_cpu_mutex_init(int cpu)
{
	mutex_init(&per_cpu(membarrier_cpu_mutexes, cpu));
}

struct mutex *lupos_membarrier_ipi_mutex(void)
{
	return &membarrier_ipi_mutex;
}

struct mutex *lupos_membarrier_cpu_mutex(int cpu)
{
	return &per_cpu(membarrier_cpu_mutexes, cpu);
}

void lupos_membarrier_mutex_lock(struct mutex *lock)
{
	mutex_lock(lock);
}

void lupos_membarrier_mutex_unlock(struct mutex *lock)
{
	mutex_unlock(lock);
}

void lupos_membarrier_mb(void)
{
	smp_mb();
}

void lupos_membarrier_cpus_read_lock(void)
{
	cpus_read_lock();
}

void lupos_membarrier_cpus_read_unlock(void)
{
	cpus_read_unlock();
}

void lupos_membarrier_rcu_read_lock(void)
{
	rcu_read_lock();
}

void lupos_membarrier_rcu_read_unlock(void)
{
	rcu_read_unlock();
}

void lupos_membarrier_synchronize_rcu(void)
{
	synchronize_rcu();
}

void lupos_membarrier_preempt_disable(void)
{
	preempt_disable();
}

void lupos_membarrier_preempt_enable(void)
{
	preempt_enable();
}

unsigned int lupos_membarrier_nr_cpu_ids(void)
{
	return nr_cpu_ids;
}

unsigned int lupos_membarrier_num_online_cpus(void)
{
	return num_online_cpus();
}

int lupos_membarrier_next_possible_cpu(int cpu)
{
#if NR_CPUS == 1
	/* Matches the header's special for_each_possible_cpu form. */
	return cpu < 0 ? 0 : 1;
#else
	return cpumask_next(cpu, cpu_possible_mask);
#endif
}

int lupos_membarrier_next_online_cpu(int cpu)
{
#if NR_CPUS == 1
	/* Matches the header's special for_each_online_cpu form. */
	return cpu < 0 ? 0 : 1;
#else
	return cpumask_next(cpu, cpu_online_mask);
#endif
}

int lupos_membarrier_raw_cpu(void)
{
	return raw_smp_processor_id();
}

bool lupos_membarrier_cpu_possible(unsigned int cpu)
{
	return cpu_possible(cpu);
}

bool lupos_membarrier_cpu_online(unsigned int cpu)
{
	return cpu_online(cpu);
}

bool lupos_membarrier_nohz_full_enabled(void)
{
	return tick_nohz_full_enabled();
}

struct mm_struct *lupos_membarrier_current_mm(void)
{
	return current->mm;
}

struct mm_struct *lupos_membarrier_task_mm(const struct task_struct *task)
{
	return task->mm;
}

struct rq *lupos_membarrier_this_rq(void)
{
	return this_rq();
}

struct rq *lupos_membarrier_cpu_rq(int cpu)
{
	return cpu_rq(cpu);
}

struct task_struct *lupos_membarrier_rq_curr_rcu(struct rq *rq)
{
	return rcu_dereference(rq->curr);
}

int lupos_membarrier_rq_state_read(const struct rq *rq)
{
	return READ_ONCE(rq->membarrier_state);
}

void lupos_membarrier_rq_state_write(struct rq *rq, int state)
{
	WRITE_ONCE(rq->membarrier_state, state);
}

void lupos_membarrier_this_rq_state_write(int state)
{
	this_cpu_write(runqueues.membarrier_state, state);
}

int lupos_membarrier_mm_state_read(const struct mm_struct *mm)
{
	return atomic_read(&mm->membarrier_state);
}

void lupos_membarrier_mm_state_set(struct mm_struct *mm, int state)
{
	atomic_set(&mm->membarrier_state, state);
}

void lupos_membarrier_mm_state_or(struct mm_struct *mm, int state)
{
	atomic_or(state, &mm->membarrier_state);
}

int lupos_membarrier_mm_users_read(const struct mm_struct *mm)
{
	return atomic_read(&mm->mm_users);
}

bool lupos_membarrier_mask_zalloc(struct lupos_membarrier_cpumask *mask)
{
	return zalloc_cpumask_var(&mask->mask, GFP_KERNEL);
}

void lupos_membarrier_mask_free(struct lupos_membarrier_cpumask *mask)
{
	free_cpumask_var(mask->mask);
}

void lupos_membarrier_mask_set_cpu(unsigned int cpu,
				 struct lupos_membarrier_cpumask *mask)
{
	__cpumask_set_cpu(cpu, mask->mask);
}

void lupos_membarrier_call_many(const struct lupos_membarrier_cpumask *mask,
			       smp_call_func_t func, void *info, bool wait)
{
	smp_call_function_many(mask->mask, func, info, wait);
}

int lupos_membarrier_call_single(int cpu, smp_call_func_t func,
				void *info, bool wait)
{
	return smp_call_function_single(cpu, func, info, wait);
}

void lupos_membarrier_call_each(const struct lupos_membarrier_cpumask *mask,
			       smp_call_func_t func, void *info, bool wait)
{
	on_each_cpu_mask(mask->mask, func, info, wait);
}

#ifdef CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE
void lupos_membarrier_sync_core_before_usermode(void)
{
	sync_core_before_usermode();
}

void lupos_membarrier_prepare_sync_core_cmd(struct mm_struct *mm)
{
	prepare_sync_core_cmd(mm);
}
#endif /* CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE */

#ifdef CONFIG_RSEQ
bool lupos_membarrier_rseq_v2_current(void)
{
	return rseq_v2(current);
}

void lupos_membarrier_rseq_switch_current(void)
{
	rseq_sched_switch_event(current);
}

void lupos_membarrier_rseq_force_update(void)
{
	rseq_force_update();
}
#endif /* CONFIG_RSEQ */

void lupos_membarrier_warn_private_flags(int flags)
{
	WARN_ON_ONCE(flags);
}

void lupos_membarrier_warn_register_flags(int flags)
{
	WARN_ON_ONCE(flags);
}

void lupos_membarrier_warn_registration_state(int state)
{
	WARN_ON_ONCE(state != 0);
}
