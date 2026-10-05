/* SPDX-License-Identifier: GPL-2.0-or-later */
#ifndef LUPOS_SCHED_MEMBARRIER_BINDINGS_H
#define LUPOS_SCHED_MEMBARRIER_BINDINGS_H

/* Configured native headers are the sole type, layout and constant authority. */
#include <linux/cpu.h>
#include <linux/cpumask_api.h>
#include <linux/errno.h>
#include <linux/init.h>
#include <linux/mutex.h>
#include <linux/sched/mm.h>
#include <linux/sched/rseq_api.h>
#include <linux/smp.h>
#include <linux/sync_core.h>
#include <linux/syscalls.h>
#include <uapi/linux/membarrier.h>
#include "sched.h"

/* Keeps cpumask_var_t's configured pointer/array choice in native headers. */
struct lupos_membarrier_cpumask {
	cpumask_var_t mask;
};

int __init lupos_membarrier_init(void);
long lupos_membarrier_syscall(int cmd, unsigned int flags, int cpu_id);

void __init lupos_membarrier_cpu_mutex_init(int cpu);
struct mutex *lupos_membarrier_ipi_mutex(void);
struct mutex *lupos_membarrier_cpu_mutex(int cpu);
void lupos_membarrier_mutex_lock(struct mutex *lock);
void lupos_membarrier_mutex_unlock(struct mutex *lock);
void lupos_membarrier_mb(void);
void lupos_membarrier_cpus_read_lock(void);
void lupos_membarrier_cpus_read_unlock(void);
void lupos_membarrier_rcu_read_lock(void);
void lupos_membarrier_rcu_read_unlock(void);
void lupos_membarrier_synchronize_rcu(void);
void lupos_membarrier_preempt_disable(void);
void lupos_membarrier_preempt_enable(void);

unsigned int lupos_membarrier_nr_cpu_ids(void);
unsigned int lupos_membarrier_num_online_cpus(void);
int lupos_membarrier_next_possible_cpu(int cpu);
int lupos_membarrier_next_online_cpu(int cpu);
int lupos_membarrier_raw_cpu(void);
bool lupos_membarrier_cpu_possible(unsigned int cpu);
bool lupos_membarrier_cpu_online(unsigned int cpu);
bool lupos_membarrier_nohz_full_enabled(void);

struct mm_struct *lupos_membarrier_current_mm(void);
struct mm_struct *lupos_membarrier_task_mm(const struct task_struct *task);
struct rq *lupos_membarrier_this_rq(void);
struct rq *lupos_membarrier_cpu_rq(int cpu);
struct task_struct *lupos_membarrier_rq_curr_rcu(struct rq *rq);
int lupos_membarrier_rq_state_read(const struct rq *rq);
void lupos_membarrier_rq_state_write(struct rq *rq, int state);
void lupos_membarrier_this_rq_state_write(int state);
int lupos_membarrier_mm_state_read(const struct mm_struct *mm);
void lupos_membarrier_mm_state_set(struct mm_struct *mm, int state);
void lupos_membarrier_mm_state_or(struct mm_struct *mm, int state);
int lupos_membarrier_mm_users_read(const struct mm_struct *mm);

bool lupos_membarrier_mask_zalloc(struct lupos_membarrier_cpumask *mask);
void lupos_membarrier_mask_free(struct lupos_membarrier_cpumask *mask);
void lupos_membarrier_mask_set_cpu(unsigned int cpu,
				 struct lupos_membarrier_cpumask *mask);
void lupos_membarrier_call_many(const struct lupos_membarrier_cpumask *mask,
			       smp_call_func_t func, void *info, bool wait);
int lupos_membarrier_call_single(int cpu, smp_call_func_t func,
				void *info, bool wait);
void lupos_membarrier_call_each(const struct lupos_membarrier_cpumask *mask,
			       smp_call_func_t func, void *info, bool wait);

#ifdef CONFIG_ARCH_HAS_MEMBARRIER_SYNC_CORE
void lupos_membarrier_sync_core_before_usermode(void);
void lupos_membarrier_prepare_sync_core_cmd(struct mm_struct *mm);
#endif
#ifdef CONFIG_RSEQ
bool lupos_membarrier_rseq_v2_current(void);
void lupos_membarrier_rseq_switch_current(void);
void lupos_membarrier_rseq_force_update(void);
#endif

/* Distinct WARN_ON_ONCE sites must not collapse into one shared once flag. */
void lupos_membarrier_warn_private_flags(int flags);
void lupos_membarrier_warn_register_flags(int flags);
void lupos_membarrier_warn_registration_state(int state);

#endif /* LUPOS_SCHED_MEMBARRIER_BINDINGS_H */
