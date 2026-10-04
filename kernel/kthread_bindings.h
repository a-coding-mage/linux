/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_KTHREAD_BINDINGS_H
#define LUPOS_KTHREAD_BINDINGS_H
/* Configured native definitions from the original translation unit. */
#include <uapi/linux/sched/types.h>
#include <linux/mm.h>
#include <linux/mmu_context.h>
#include <linux/sched.h>
#include <linux/sched/mm.h>
#include <linux/sched/task.h>
#include <linux/kthread.h>
#include <linux/completion.h>
#include <linux/err.h>
#include <linux/cgroup.h>
#include <linux/cpuset.h>
#include <linux/unistd.h>
#include <linux/file.h>
#include <linux/export.h>
#include <linux/mutex.h>
#include <linux/slab.h>
#include <linux/freezer.h>
#include <linux/ptrace.h>
#include <linux/uaccess.h>
#include <linux/numa.h>
#include <linux/sched/isolation.h>
#include <linux/cpuhotplug.h>
#include <linux/init.h>

/* These two private C types have no public-header definition. Copied verbatim
 * from kernel/kthread.c, and checked against it by verify-source.pl. They are
 * bindgen inputs only: there is no hand-maintained Rust kernel layout. */
struct kthread_create_info
{
	/* Information passed to kthread() from kthreadd. */
	char *full_name;
	int (*threadfn)(void *data);
	void *data;
	int node;

	/* Result passed back to kthread_create() from kthreadd. */
	struct task_struct *result;
	struct completion *done;

	struct list_head list;
};

struct kthread {
	unsigned long flags;
	unsigned int cpu;
	unsigned int node;
	int started;
	int result;
	int (*threadfn)(void *);
	void *data;
	struct completion parked;
	struct completion exited;
#ifdef CONFIG_BLK_CGROUP
	struct cgroup_subsys_state *blkcg_css;
#endif
	/* To store the full name if task comm is truncated. */
	char *full_name;
	struct task_struct *task;
	struct list_head affinity_node;
	struct cpumask *preferred_affinity;
};

enum KTHREAD_BITS {
	KTHREAD_IS_PER_CPU = 0,
	KTHREAD_SHOULD_STOP,
	KTHREAD_SHOULD_PARK,
};

struct lupos_kthread_cpumask { cpumask_var_t mask; };
struct lupos_kthread_flush_work {
	struct kthread_work work;
	struct completion done;
};

/* Primitive calls whose inline/macro definitions carry configuration, lockdep,
 * barrier, instrumentation or arch semantics which Rust must not guess. */
struct task_struct *lupos_kthread_current(void);
bool lupos_kthread_warn(bool value);
#define LUPOS_KTHREAD_WARN_SITE(name) bool lupos_kthread_warn_##name(bool value);
#include "kthread_warn_sites.inc"
#undef LUPOS_KTHREAD_WARN_SITE
void *lupos_kthread_alloc(size_t bytes);
void *lupos_kthread_zalloc(size_t bytes);
char *lupos_kthread_vasprintf(const char *format, void *args);
ssize_t lupos_kthread_strscpy(char *dst, const char *src, size_t len);
void lupos_kthread_init_completion(struct completion *done);
struct completion *lupos_kthread_xchg_done(struct completion **done);
bool lupos_kthread_test_bit(unsigned int bit, const unsigned long *flags);
void lupos_kthread_set_bit(unsigned int bit, unsigned long *flags);
void lupos_kthread_clear_bit(unsigned int bit, unsigned long *flags);
void lupos_kthread_create_lock(void);
void lupos_kthread_create_unlock(void);
void lupos_kthread_affinity_lock(void);
void lupos_kthread_affinity_unlock(void);
bool lupos_kthread_list_empty(const struct list_head *head);
void lupos_kthread_list_add(struct list_head *node, struct list_head *head);
void lupos_kthread_list_add_tail(struct list_head *node, struct list_head *head);
void lupos_kthread_list_del(struct list_head *node);
void lupos_kthread_list_del_init(struct list_head *node);
void lupos_kthread_set_state(unsigned int state);
void lupos_kthread_set_state_relaxed(unsigned int state);
void lupos_kthread_set_special_state(unsigned int state);
void lupos_kthread_preempt_disable(void);
void lupos_kthread_preempt_enable(void);
void lupos_kthread_might_sleep(void);
bool lupos_kthread_refrigerator(bool check_stop);
#ifdef CONFIG_FREEZER
bool lupos_kthread_freezer_active(void);
void lupos_kthread_debug_no_locks_held(void);
#endif
void lupos_kthread_set_freezable(void);
void lupos_kthread_cond_resched(void);
void lupos_kthread_cgroup_ready(void);
void lupos_kthread_cgroup_init(void);
void lupos_kthread_rcu_lock(void);
void lupos_kthread_rcu_unlock(void);
const struct cpumask *lupos_kthread_housekeeping_mask(void);
const struct cpumask *lupos_kthread_node_mask(int node);
const struct cpumask *lupos_kthread_cpu_mask(unsigned int cpu);
int lupos_kthread_cpu_to_node(int cpu);
bool lupos_kthread_alloc_cpumask(struct lupos_kthread_cpumask *storage);
void lupos_kthread_free_cpumask(struct lupos_kthread_cpumask *storage);
struct cpumask *lupos_kthread_cpumask_ptr(struct lupos_kthread_cpumask *storage);
void lupos_kthread_cpumask_and(struct cpumask *dst, const struct cpumask *a, const struct cpumask *b);
bool lupos_kthread_cpumask_empty(const struct cpumask *mask);
void lupos_kthread_cpumask_copy(struct cpumask *dst, const struct cpumask *src);
void lupos_kthread_raw_lock_init(raw_spinlock_t *lock);
void lupos_kthread_lockdep_class(raw_spinlock_t *lock, struct lock_class_key *key, const char *name);
void lupos_kthread_assert_locked(raw_spinlock_t *lock);
unsigned long lupos_kthread_raw_lock_irqsave(raw_spinlock_t *lock);
void lupos_kthread_raw_unlock_irqrestore(raw_spinlock_t *lock, unsigned long flags);
void lupos_kthread_raw_lock_irq(raw_spinlock_t *lock);
void lupos_kthread_raw_unlock_irq(raw_spinlock_t *lock);
void lupos_kthread_notify_signal(struct task_struct *task);
void lupos_kthread_get_task(struct task_struct *task);
bool lupos_kthread_task_usage_dec_and_test(struct task_struct *task);
void lupos_kthread_task_release_rcu(struct task_struct *task);
void lupos_kthread_set_comm(struct task_struct *task, const char *comm);
void lupos_kthread_ignore_signals(struct task_struct *task);
#ifdef CONFIG_CPUSETS
nodemask_t lupos_kthread_memory_nodes(void);
unsigned long lupos_kthread_irq_save(void);
void lupos_kthread_irq_restore(unsigned long flags);
void lupos_kthread_mems_seq_begin(struct task_struct *task);
void lupos_kthread_mems_seq_end(struct task_struct *task);
#endif
int lupos_kthread_cpuhp_setup(const char *name);
void lupos_kthread_init_worker_key(struct kthread_worker *worker);
unsigned long lupos_kthread_jiffies(void);
bool lupos_kthread_timer_callback_matches(struct timer_list *timer);
void lupos_kthread_trace_stop(struct task_struct *task);
void lupos_kthread_trace_stop_ret(int result);
void lupos_kthread_trace_execute_start(struct kthread_work *work);
void lupos_kthread_trace_execute_end(struct kthread_work *work, kthread_work_func_t func);
void lupos_kthread_trace_queue_work(struct kthread_worker *worker, struct kthread_work *work);
void lupos_kthread_mmgrab(struct mm_struct *mm);
bool lupos_kthread_mm_count_dec_and_test(struct mm_struct *mm);
void lupos_kthread_mb(void);
void lupos_kthread_task_lock(struct task_struct *task);
void lupos_kthread_task_unlock(struct task_struct *task);
void lupos_kthread_irq_disable(void);
void lupos_kthread_irq_enable(void);
void lupos_kthread_membarrier_update(struct mm_struct *mm);
void lupos_kthread_switch_mm(struct mm_struct *prev, struct mm_struct *next, struct task_struct *task);
void lupos_kthread_finish_arch_post_lock_switch(void);
void lupos_kthread_mb_after_spinlock(void);
void lupos_kthread_enter_lazy_tlb(struct mm_struct *mm, struct task_struct *task);
#ifdef CONFIG_BLK_CGROUP
enum {
	LUPOS_KTHREAD_CSS_NO_REF = CSS_NO_REF,
	LUPOS_KTHREAD_PERCPU_REF_ATOMIC_DEAD = __PERCPU_REF_ATOMIC_DEAD,
};
unsigned int lupos_kthread_css_flags(struct cgroup_subsys_state *css);
struct percpu_ref *lupos_kthread_css_refcount(struct cgroup_subsys_state *css);
unsigned long lupos_kthread_percpu_ref_read_mode(struct percpu_ref *ref);
void lupos_kthread_percpu_ref_cpu_add(unsigned long pointer, unsigned long nr);
void lupos_kthread_percpu_ref_cpu_sub(unsigned long pointer, unsigned long nr);
void lupos_kthread_percpu_ref_atomic_add(struct percpu_ref *ref, unsigned long nr);
bool lupos_kthread_percpu_ref_atomic_sub_and_test(struct percpu_ref *ref, unsigned long nr);
void lupos_kthread_percpu_ref_release(struct percpu_ref *ref);
#endif

/* C variadic and callback envelopes: no task/worker policy here. */
int lupos_kthread_thread_callback(void *data);
void lupos_kthread_flush_callback(struct kthread_work *work);
int lupos_kthread_thread(void *data);
void lupos_kthread_delayed_work_timer(struct timer_list *timer);
void lupos_kthread_flush_work_fn(struct kthread_work *work);
int lupos_kthread_online_cpu(unsigned int cpu);
int lupos_kthread_init(void);
struct task_struct *lupos_kthread_create_on_cpu(int (*threadfn)(void *), void *data,
	unsigned int cpu, const char *format);
struct task_struct *lupos_kthread_create_on_node_v(int (*threadfn)(void *), void *data,
	int node, const char *format, void *args);
struct kthread_worker *lupos_kthread_create_worker_on_node_v(unsigned int flags,
	int node, const char *format, void *args);

#include "kthread_layout.h"
#endif
