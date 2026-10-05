/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_SYSCALLS_BINDINGS_H
#define LUPOS_SCHED_SYSCALLS_BINDINGS_H

/* Configured native headers alone own all C layouts and constants. */
#include <linux/compat.h>
#include <linux/cred.h>
#include <linux/sched/debug.h>
#include <linux/sched/rt.h>
#include <linux/security.h>
#include <linux/slab.h>
#include <linux/time.h>
#include <linux/time32.h>
#include <linux/uaccess.h>
#include <uapi/linux/sched/types.h>
#include "sched.h"
#include "autogroup.h"
#include "stats.h"

enum {
	LUPOS_SYS_MAX_DL_PRIO = MAX_DL_PRIO,
	LUPOS_SYS_MAX_RT_PRIO = MAX_RT_PRIO,
	LUPOS_SYS_MIN_NICE = MIN_NICE,
	LUPOS_SYS_MAX_NICE = MAX_NICE,
	LUPOS_SYS_NICE_WIDTH = NICE_WIDTH,
	LUPOS_SYS_CAP_SYS_NICE = CAP_SYS_NICE,
	LUPOS_SYS_RLIMIT_NICE = RLIMIT_NICE,
	LUPOS_SYS_RLIMIT_RTPRIO = RLIMIT_RTPRIO,
	LUPOS_SYS_EPERM = EPERM,
	LUPOS_SYS_ESRCH = ESRCH,
	LUPOS_SYS_EINVAL = EINVAL,
	LUPOS_SYS_ENOMEM = ENOMEM,
	LUPOS_SYS_EFAULT = EFAULT,
	LUPOS_SYS_E2BIG = E2BIG,
	LUPOS_SYS_EBUSY = EBUSY,
	LUPOS_SYS_EOPNOTSUPP = EOPNOTSUPP,
	LUPOS_SYS_DEQUEUE_SAVE = DEQUEUE_SAVE,
	LUPOS_SYS_DEQUEUE_MOVE = DEQUEUE_MOVE,
	LUPOS_SYS_DEQUEUE_NOCLOCK = DEQUEUE_NOCLOCK,
	LUPOS_SYS_DEQUEUE_CLASS = DEQUEUE_CLASS,
	LUPOS_SYS_ENQUEUE_REPLENISH = ENQUEUE_REPLENISH,
	LUPOS_SYS_ENQUEUE_HEAD = ENQUEUE_HEAD,
	LUPOS_SYS_SCA_USER = SCA_USER,
	LUPOS_SYS_SCA_CHECK = SCA_CHECK,
	LUPOS_SYS_PF_NO_SETAFFINITY = PF_NO_SETAFFINITY,
	LUPOS_SYS_BITS_PER_BYTE = BITS_PER_BYTE,
};

struct task_struct *lupos_syscalls_current(void);
struct rq *lupos_syscalls_cpu_rq(int cpu);
struct rq *lupos_syscalls_this_rq(void);
struct rq *lupos_syscalls_task_rq(struct task_struct *p);
struct task_struct *lupos_syscalls_rq_donor(struct rq *rq);
bool lupos_syscalls_idle_rq(struct rq *rq);
bool lupos_syscalls_dl_policy(int policy);
bool lupos_syscalls_rt_policy(int policy);
bool lupos_syscalls_fair_policy(int policy);
bool lupos_syscalls_idle_policy(int policy);
bool lupos_syscalls_valid_policy(int policy);
bool lupos_syscalls_rt_or_dl_prio(int prio);
bool lupos_syscalls_dl_prio(int prio);
bool lupos_syscalls_dl_task(struct task_struct *p);
bool lupos_syscalls_rt_task(struct task_struct *p);
bool lupos_syscalls_task_has_dl_policy(struct task_struct *p);
bool lupos_syscalls_task_has_rt_policy(struct task_struct *p);
bool lupos_syscalls_task_has_idle_policy(struct task_struct *p);
bool lupos_syscalls_rt_or_dl_task_policy(struct task_struct *p);
int lupos_syscalls_task_nice(const struct task_struct *p);
int lupos_syscalls_nice_to_prio(int nice);
int lupos_syscalls_prio_to_nice(int prio);
int lupos_syscalls_nice_to_rlimit(int nice);
unsigned long lupos_syscalls_task_rlimit(const struct task_struct *p,
					unsigned int limit);
bool lupos_syscalls_reset_on_fork(struct task_struct *p);
void lupos_syscalls_set_reset_on_fork(struct task_struct *p, bool reset);
void lupos_syscalls_get_task(struct task_struct *p);
void lupos_syscalls_put_task(struct task_struct *p);
void lupos_syscalls_rcu_lock(void);
void lupos_syscalls_rcu_unlock(void);
const struct cred *lupos_syscalls_current_cred(void);
const struct cred *lupos_syscalls_task_cred(struct task_struct *p);
bool lupos_syscalls_cred_euid_eq(const struct cred *a, const struct cred *b);
bool lupos_syscalls_cred_euid_uid_eq(const struct cred *a, const struct cred *b);
struct rq *lupos_syscalls_task_rq_lock(struct task_struct *p, struct rq_flags *rf);
void lupos_syscalls_task_rq_unlock(struct rq *rq, struct task_struct *p,
				    struct rq_flags *rf);
struct rq *lupos_syscalls_this_rq_lock_irq(struct rq_flags *rf);
void lupos_syscalls_rq_unlock_irq(struct rq *rq, struct rq_flags *rf);
void lupos_syscalls_pi_lock(struct task_struct *p);
void lupos_syscalls_pi_unlock(struct task_struct *p);
void lupos_syscalls_double_rq_lock(struct rq *a, struct rq *b);
void lupos_syscalls_double_rq_unlock(struct rq *a, struct rq *b);
void lupos_syscalls_preempt_disable(void);
void lupos_syscalls_preempt_enable(void);
void lupos_syscalls_preempt_enable_no_resched(void);
void lupos_syscalls_bug_pi_interrupt(bool pi);
int lupos_syscalls_rt_effective_prio(struct task_struct *p, int prio);
int lupos_syscalls_scx_check_setscheduler(struct task_struct *p, int policy);
bool lupos_syscalls_dl_bandwidth_enabled(void);
bool lupos_syscalls_dl_entity_is_special(struct sched_dl_entity *dl);
void lupos_syscalls_warn_fifo(int result);
void lupos_syscalls_warn_fifo_low(int result);
void lupos_syscalls_warn_fifo_secondary(int result);
void lupos_syscalls_warn_normal(int result);
bool lupos_syscalls_alloc_mask(cpumask_var_t *mask, bool zero);
struct cpumask *lupos_syscalls_mask_ptr(cpumask_var_t *mask);
void lupos_syscalls_free_mask(cpumask_var_t *mask);
struct cpumask *lupos_syscalls_alloc_user_mask(void);
bool lupos_syscalls_mask_subset(const struct cpumask *a, const struct cpumask *b);
bool lupos_syscalls_mask_and(struct cpumask *dst, const struct cpumask *a,
			      const struct cpumask *b);
void lupos_syscalls_mask_copy(struct cpumask *dst, const struct cpumask *src);
void lupos_syscalls_mask_clear(struct cpumask *mask);
unsigned int lupos_syscalls_mask_size(void);
const struct cpumask *lupos_syscalls_active_mask(void);
unsigned int lupos_syscalls_nr_cpu_ids(void);
unsigned long lupos_syscalls_page_size(void);
int lupos_syscalls_get_attr_size(const struct sched_attr __user *attr, u32 *size);
int lupos_syscalls_put_attr_size(struct sched_attr __user *attr, u32 size);
int lupos_syscalls_copy_attr_from_user(struct sched_attr *dst,
				       const struct sched_attr __user *src, u32 size);
int lupos_syscalls_copy_attr_to_user(struct sched_attr __user *dst, u32 size,
				     const struct sched_attr *src);
void lupos_syscalls_yield_stat(struct rq *rq);
void lupos_syscalls_set_running(void);
bool lupos_syscalls_task_on_cpu(struct rq *rq, struct task_struct *p);
bool lupos_syscalls_task_is_running(struct task_struct *p);
void lupos_syscalls_jiffies_to_timespec64(unsigned int jiffies, struct timespec64 *t);
#ifdef CONFIG_SCHED_CORE
struct task_struct *lupos_syscalls_rq_curr(struct rq *rq);
bool lupos_syscalls_sched_core_enabled(struct rq *rq);
#endif
#ifdef CONFIG_RT_GROUP_SCHED
bool lupos_syscalls_rt_group_sched_enabled(void);
bool lupos_syscalls_rt_bandwidth_enabled(void);
struct task_group *lupos_syscalls_task_group(struct task_struct *p);
bool lupos_syscalls_task_group_is_autogroup(struct task_group *tg);
#endif
#ifdef CONFIG_RT_MUTEXES
struct task_struct *lupos_syscalls_rt_mutex_get_top_task(struct task_struct *p);
#endif
#ifdef CONFIG_UCLAMP_TASK
unsigned int lupos_syscalls_uclamp_value(struct uclamp_se *uc);
bool lupos_syscalls_uclamp_user_defined(struct uclamp_se *uc);
void lupos_syscalls_uclamp_enable(void);
unsigned int lupos_syscalls_uclamp_none(unsigned int id);
void lupos_syscalls_uclamp_set(struct uclamp_se *uc, unsigned int value, bool user);
#endif
const struct cpumask *lupos_syscalls_root_span(struct rq *rq);
bool lupos_syscalls_capable(int cap);
bool lupos_syscalls_ns_capable(struct user_namespace *ns, int cap);
int lupos_syscalls_security_setnice(struct task_struct *p, int nice);
int lupos_syscalls_security_setscheduler(struct task_struct *p);
int lupos_syscalls_security_getscheduler(struct task_struct *p);
void lupos_syscalls_cpuset_lock(void);
void lupos_syscalls_cpuset_unlock(void);
void lupos_syscalls_cpuset_cpus_allowed(struct task_struct *p, struct cpumask *mask);
void lupos_syscalls_rt_mutex_adjust_pi(struct task_struct *p);
unsigned long lupos_syscalls_copy_param_from_user(struct sched_param *dst,
                                                 const struct sched_param __user *src);
unsigned long lupos_syscalls_copy_param_to_user(struct sched_param __user *dst,
                                               const struct sched_param *src);
unsigned long lupos_syscalls_copy_mask_from_user(struct cpumask *dst,
                                                const unsigned long __user *src,
                                                unsigned int len);
unsigned long lupos_syscalls_copy_mask_to_user(unsigned long __user *dst,
                                              const struct cpumask *src,
                                              unsigned int len);
bool lupos_syscalls_likely_find_task(bool present);
bool lupos_syscalls_unlikely_same_policy(bool same);
bool lupos_syscalls_unlikely_policy_changed(bool changed);
bool lupos_syscalls_unlikely_setparam_args(bool invalid);
bool lupos_syscalls_unlikely_setattr_args(bool invalid);
bool lupos_syscalls_unlikely_getparam_args(bool invalid);
bool lupos_syscalls_unlikely_getattr_args(bool invalid);
bool lupos_syscalls_unlikely_old_user_mask(bool present);
#ifdef CONFIG_UCLAMP_TASK
bool lupos_syscalls_likely_uclamp_reset(bool no_flags);
bool lupos_syscalls_unlikely_uclamp_rt_min(bool rt_min);
bool lupos_syscalls_likely_uclamp_no_flags(bool no_flags);
#endif
void lupos_syscalls_schedule(void);
/* Plain native field store, not synchronization; paired with the original
 * pre-rq SCX iterator read. Caller retains all task lifetime/rq obligations. */
void lupos_syscalls_task_class_write(struct task_struct *p,
                                    const struct sched_class *class);
#endif /* LUPOS_SCHED_SYSCALLS_BINDINGS_H */
