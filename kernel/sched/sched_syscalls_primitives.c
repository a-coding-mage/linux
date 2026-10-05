// SPDX-License-Identifier: GPL-2.0-only
/* Native macro/inline, locking, usercopy and diagnostic leaves only.
 * Scheduler syscall decisions and algorithms are owned by syscalls.rs.
 * SOURCE ONLY: this file does not admit the owner to a build.
 */
#include "sched_syscalls_bindings.h"

struct task_struct *lupos_syscalls_current(void) { return current; }
struct rq *lupos_syscalls_cpu_rq(int cpu) { return cpu_rq(cpu); }
struct rq *lupos_syscalls_this_rq(void) { return this_rq(); }
struct rq *lupos_syscalls_task_rq(struct task_struct *p) { return task_rq(p); }
/* !CONFIG_SCHED_PROXY_EXEC stores donor/curr in a native anonymous union. */
struct task_struct *lupos_syscalls_rq_donor(struct rq *rq) { return rq->donor; }
bool lupos_syscalls_idle_rq(struct rq *rq) { return idle_rq(rq); }
bool lupos_syscalls_dl_policy(int policy) { return dl_policy(policy); }
bool lupos_syscalls_rt_policy(int policy) { return rt_policy(policy); }
bool lupos_syscalls_fair_policy(int policy) { return fair_policy(policy); }
bool lupos_syscalls_idle_policy(int policy) { return idle_policy(policy); }
bool lupos_syscalls_valid_policy(int policy) { return valid_policy(policy); }
bool lupos_syscalls_rt_or_dl_prio(int prio) { return rt_or_dl_prio(prio); }
bool lupos_syscalls_dl_prio(int prio) { return dl_prio(prio); }
bool lupos_syscalls_dl_task(struct task_struct *p) { return dl_task(p); }
bool lupos_syscalls_rt_task(struct task_struct *p) { return rt_task(p); }
bool lupos_syscalls_task_has_dl_policy(struct task_struct *p)
{
	return task_has_dl_policy(p);
}
bool lupos_syscalls_task_has_rt_policy(struct task_struct *p)
{
	return task_has_rt_policy(p);
}
bool lupos_syscalls_task_has_idle_policy(struct task_struct *p)
{
	return task_has_idle_policy(p);
}
bool lupos_syscalls_rt_or_dl_task_policy(struct task_struct *p)
{
	return rt_or_dl_task_policy(p);
}
int lupos_syscalls_task_nice(const struct task_struct *p) { return task_nice(p); }
int lupos_syscalls_nice_to_prio(int nice) { return NICE_TO_PRIO(nice); }
int lupos_syscalls_prio_to_nice(int prio) { return PRIO_TO_NICE(prio); }
int lupos_syscalls_nice_to_rlimit(int nice) { return nice_to_rlimit(nice); }
unsigned long lupos_syscalls_task_rlimit(const struct task_struct *p,
					unsigned int limit)
{
	return task_rlimit(p, limit);
}
bool lupos_syscalls_reset_on_fork(struct task_struct *p)
{
	return p->sched_reset_on_fork;
}
void lupos_syscalls_set_reset_on_fork(struct task_struct *p, bool reset)
{
	p->sched_reset_on_fork = reset;
}
void lupos_syscalls_get_task(struct task_struct *p) { get_task_struct(p); }
void lupos_syscalls_put_task(struct task_struct *p) { put_task_struct(p); }
void lupos_syscalls_rcu_lock(void) { rcu_read_lock(); }
void lupos_syscalls_rcu_unlock(void) { rcu_read_unlock(); }
const struct cred *lupos_syscalls_current_cred(void) { return current_cred(); }
const struct cred *lupos_syscalls_task_cred(struct task_struct *p)
{
	return __task_cred(p);
}
bool lupos_syscalls_cred_euid_eq(const struct cred *a, const struct cred *b)
{
	return uid_eq(a->euid, b->euid);
}
bool lupos_syscalls_cred_euid_uid_eq(const struct cred *a, const struct cred *b)
{
	return uid_eq(a->euid, b->uid);
}
struct rq *lupos_syscalls_task_rq_lock(struct task_struct *p, struct rq_flags *rf)
{
	return task_rq_lock(p, rf);
}
void lupos_syscalls_task_rq_unlock(struct rq *rq, struct task_struct *p,
				    struct rq_flags *rf)
{
	task_rq_unlock(rq, p, rf);
}
struct rq *lupos_syscalls_this_rq_lock_irq(struct rq_flags *rf)
{
	return this_rq_lock_irq(rf);
}
void lupos_syscalls_rq_unlock_irq(struct rq *rq, struct rq_flags *rf)
{
	rq_unlock_irq(rq, rf);
}
void lupos_syscalls_pi_lock(struct task_struct *p)
{
	/* Exact raw_spinlock_irqsave cleanup-guard constructor in this baseline. */
	raw_spin_lock_irq_disable(&p->pi_lock);
}
void lupos_syscalls_pi_unlock(struct task_struct *p)
{
	raw_spin_unlock_irq_enable(&p->pi_lock);
}
void lupos_syscalls_double_rq_lock(struct rq *a, struct rq *b)
{
	double_rq_lock(a, b);
}
void lupos_syscalls_double_rq_unlock(struct rq *a, struct rq *b)
{
	double_rq_unlock(a, b);
}
void lupos_syscalls_preempt_disable(void) { preempt_disable(); }
void lupos_syscalls_preempt_enable(void) { preempt_enable(); }
void lupos_syscalls_preempt_enable_no_resched(void)
{
	sched_preempt_enable_no_resched();
}
void lupos_syscalls_bug_pi_interrupt(bool pi) { BUG_ON(pi && in_interrupt()); }
int lupos_syscalls_rt_effective_prio(struct task_struct *p, int prio)
{
	return rt_effective_prio(p, prio);
}
int lupos_syscalls_scx_check_setscheduler(struct task_struct *p, int policy)
{
	return scx_check_setscheduler(p, policy);
}
bool lupos_syscalls_dl_bandwidth_enabled(void) { return dl_bandwidth_enabled(); }
bool lupos_syscalls_dl_entity_is_special(struct sched_dl_entity *dl)
{
	return dl_entity_is_special(dl);
}
/* Each original WARN_ON_ONCE site retains its own native once state. */
void lupos_syscalls_warn_fifo(int result) { WARN_ON_ONCE(result != 0); }
void lupos_syscalls_warn_fifo_low(int result) { WARN_ON_ONCE(result != 0); }
void lupos_syscalls_warn_fifo_secondary(int result) { WARN_ON_ONCE(result != 0); }
void lupos_syscalls_warn_normal(int result) { WARN_ON_ONCE(result != 0); }
bool lupos_syscalls_alloc_mask(cpumask_var_t *mask, bool zero)
{
	return zero ? zalloc_cpumask_var(mask, GFP_KERNEL) :
		      alloc_cpumask_var(mask, GFP_KERNEL);
}
struct cpumask *lupos_syscalls_mask_ptr(cpumask_var_t *mask) { return *mask; }
void lupos_syscalls_free_mask(cpumask_var_t *mask) { free_cpumask_var(*mask); }
struct cpumask *lupos_syscalls_alloc_user_mask(void)
{
	return alloc_user_cpus_ptr(NUMA_NO_NODE);
}
bool lupos_syscalls_mask_subset(const struct cpumask *a, const struct cpumask *b)
{
	return cpumask_subset(a, b);
}
bool lupos_syscalls_mask_and(struct cpumask *dst, const struct cpumask *a,
			      const struct cpumask *b)
{
	return cpumask_and(dst, a, b);
}
void lupos_syscalls_mask_copy(struct cpumask *dst, const struct cpumask *src)
{
	cpumask_copy(dst, src);
}
void lupos_syscalls_mask_clear(struct cpumask *mask) { cpumask_clear(mask); }
unsigned int lupos_syscalls_mask_size(void) { return cpumask_size(); }
const struct cpumask *lupos_syscalls_active_mask(void) { return cpu_active_mask; }
unsigned int lupos_syscalls_nr_cpu_ids(void) { return nr_cpu_ids; }
unsigned long lupos_syscalls_page_size(void) { return PAGE_SIZE; }
int lupos_syscalls_get_attr_size(const struct sched_attr __user *attr, u32 *size)
{
	return get_user(*size, &attr->size);
}
int lupos_syscalls_put_attr_size(struct sched_attr __user *attr, u32 size)
{
	return put_user(size, &attr->size);
}
int lupos_syscalls_copy_attr_from_user(struct sched_attr *dst,
				       const struct sched_attr __user *src, u32 size)
{
	return copy_struct_from_user(dst, sizeof(*dst), src, size);
}
int lupos_syscalls_copy_attr_to_user(struct sched_attr __user *dst, u32 size,
				     const struct sched_attr *src)
{
	return copy_struct_to_user(dst, size, src, sizeof(*src), NULL);
}
void lupos_syscalls_yield_stat(struct rq *rq) { schedstat_inc(rq->yld_count); }
void lupos_syscalls_set_running(void) { set_current_state(TASK_RUNNING); }
bool lupos_syscalls_task_on_cpu(struct rq *rq, struct task_struct *p)
{
	return task_on_cpu(rq, p);
}
bool lupos_syscalls_task_is_running(struct task_struct *p)
{
	return task_is_running(p);
}
void lupos_syscalls_jiffies_to_timespec64(unsigned int jiffies, struct timespec64 *t)
{
	jiffies_to_timespec64(jiffies, t);
}
#ifdef CONFIG_SCHED_CORE
struct task_struct *lupos_syscalls_rq_curr(struct rq *rq) { return rq->curr; }
bool lupos_syscalls_sched_core_enabled(struct rq *rq)
{
	return sched_core_enabled(rq);
}
#endif
#ifdef CONFIG_RT_GROUP_SCHED
bool lupos_syscalls_rt_group_sched_enabled(void) { return rt_group_sched_enabled(); }
bool lupos_syscalls_rt_bandwidth_enabled(void) { return rt_bandwidth_enabled(); }
struct task_group *lupos_syscalls_task_group(struct task_struct *p)
{
	return task_group(p);
}
bool lupos_syscalls_task_group_is_autogroup(struct task_group *tg)
{
	return task_group_is_autogroup(tg);
}
#endif
#ifdef CONFIG_RT_MUTEXES
struct task_struct *lupos_syscalls_rt_mutex_get_top_task(struct task_struct *p)
{
	return rt_mutex_get_top_task(p);
}
#endif
#ifdef CONFIG_UCLAMP_TASK
unsigned int lupos_syscalls_uclamp_value(struct uclamp_se *uc) { return uc->value; }
bool lupos_syscalls_uclamp_user_defined(struct uclamp_se *uc)
{
	return uc->user_defined;
}
void lupos_syscalls_uclamp_enable(void) { sched_uclamp_enable(); }
unsigned int lupos_syscalls_uclamp_none(unsigned int id) { return uclamp_none((enum uclamp_id)id); }
void lupos_syscalls_uclamp_set(struct uclamp_se *uc, unsigned int value, bool user)
{
	uclamp_se_set(uc, value, user);
}
#endif

const struct cpumask *lupos_syscalls_root_span(struct rq *rq) { return rq->rd->span; }
bool lupos_syscalls_capable(int cap) { return capable(cap); }
bool lupos_syscalls_ns_capable(struct user_namespace *ns, int cap)
{
	return ns_capable(ns, cap);
}
int lupos_syscalls_security_setnice(struct task_struct *p, int nice)
{
	return security_task_setnice(p, nice);
}
int lupos_syscalls_security_setscheduler(struct task_struct *p)
{
	return security_task_setscheduler(p);
}
int lupos_syscalls_security_getscheduler(struct task_struct *p)
{
	return security_task_getscheduler(p);
}
void lupos_syscalls_cpuset_lock(void) { cpuset_lock(); }
void lupos_syscalls_cpuset_unlock(void) { cpuset_unlock(); }
void lupos_syscalls_cpuset_cpus_allowed(struct task_struct *p, struct cpumask *mask)
{
	cpuset_cpus_allowed(p, mask);
}
void lupos_syscalls_rt_mutex_adjust_pi(struct task_struct *p) { rt_mutex_adjust_pi(p); }

/* Use typed native usercopy entry points. Cross-language object provenance
 * and compiler object-size coverage remain blocked admission requirements.
 */
unsigned long lupos_syscalls_copy_param_from_user(struct sched_param *dst,
						 const struct sched_param __user *src)
{
	return copy_from_user(dst, src, sizeof(*dst));
}
unsigned long lupos_syscalls_copy_param_to_user(struct sched_param __user *dst,
					       const struct sched_param *src)
{
	return copy_to_user(dst, src, sizeof(*src));
}
unsigned long lupos_syscalls_copy_mask_from_user(struct cpumask *dst,
						const unsigned long __user *src,
						unsigned int len)
{
	return copy_from_user(dst, src, len);
}
unsigned long lupos_syscalls_copy_mask_to_user(unsigned long __user *dst,
					      const struct cpumask *src,
					      unsigned int len)
{
	return copy_to_user(dst, cpumask_bits(src), len);
}

/* Preserve each annotated-branch site's native profiling hook separately. */
bool lupos_syscalls_likely_find_task(bool present) { return likely(present); }
bool lupos_syscalls_unlikely_same_policy(bool same) { return unlikely(same); }
bool lupos_syscalls_unlikely_policy_changed(bool changed) { return unlikely(changed); }
bool lupos_syscalls_unlikely_setparam_args(bool invalid) { return unlikely(invalid); }
bool lupos_syscalls_unlikely_setattr_args(bool invalid) { return unlikely(invalid); }
bool lupos_syscalls_unlikely_getparam_args(bool invalid) { return unlikely(invalid); }
bool lupos_syscalls_unlikely_getattr_args(bool invalid) { return unlikely(invalid); }
bool lupos_syscalls_unlikely_old_user_mask(bool present) { return unlikely(present); }
#ifdef CONFIG_UCLAMP_TASK
bool lupos_syscalls_likely_uclamp_reset(bool no_flags) { return likely(no_flags); }
bool lupos_syscalls_unlikely_uclamp_rt_min(bool rt_min) { return unlikely(rt_min); }
bool lupos_syscalls_likely_uclamp_no_flags(bool no_flags) { return likely(no_flags); }
#endif

/* Native asmlinkage is architecture-specific; do not guess it in Rust. */
void lupos_syscalls_schedule(void) { schedule(); }
