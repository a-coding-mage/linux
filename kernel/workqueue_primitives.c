// SPDX-License-Identifier: GPL-2.0-only
/*
 * Native configured storage and primitive leaves for the Rust owner.
 * No workqueue scheduling, allocation, flush or cancellation algorithm lives
 * here. Source origin, instrumentation, callback ABI and generated layout are
 * NOT qualified merely by retaining the original declarations and macros.
 */
#include "workqueue_bindings.h"

#error "SOURCE ONLY HOLD: workqueue native boundary is not admitted"

/* Original C 418-549: one storage/metadata authority, no algorithm bodies. */
static const char * const wq_affn_names[WQ_AFFN_NR_TYPES] = {
	[WQ_AFFN_DFL]		= "default",
	[WQ_AFFN_CPU]		= "cpu",
	[WQ_AFFN_SMT]		= "smt",
	[WQ_AFFN_CACHE]		= "cache",
	[WQ_AFFN_CACHE_SHARD]	= "cache_shard",
	[WQ_AFFN_NUMA]		= "numa",
	[WQ_AFFN_SYSTEM]	= "system",
};

/*
 * Per-cpu work items which run for longer than the following threshold are
 * automatically considered CPU intensive and excluded from concurrency
 * management to prevent them from noticeably delaying other per-cpu work items.
 * ULONG_MAX indicates that the user hasn't overridden it with a boot parameter.
 * The actual value is initialized in wq_cpu_intensive_thresh_init().
 */
static unsigned long wq_cpu_intensive_thresh_us = ULONG_MAX;
module_param_named(cpu_intensive_thresh_us, wq_cpu_intensive_thresh_us, ulong, 0644);
#ifdef CONFIG_WQ_CPU_INTENSIVE_REPORT
static unsigned int wq_cpu_intensive_warning_thresh = 4;
module_param_named(cpu_intensive_warning_thresh, wq_cpu_intensive_warning_thresh, uint, 0644);
#endif

/* see the comment above the definition of WQ_POWER_EFFICIENT */
static bool wq_power_efficient = IS_ENABLED(CONFIG_WQ_POWER_EFFICIENT_DEFAULT);
module_param_named(power_efficient, wq_power_efficient, bool, 0444);

static unsigned int wq_cache_shard_size = 8;
module_param_named(cache_shard_size, wq_cache_shard_size, uint, 0444);

static bool wq_online;			/* can kworkers be created yet? */
static bool wq_topo_initialized __read_mostly = false;

static struct kmem_cache *pwq_cache;

static struct wq_pod_type wq_pod_types[WQ_AFFN_NR_TYPES];
static enum wq_affn_scope wq_affn_dfl = WQ_AFFN_CACHE_SHARD;

/* buf for wq_update_unbound_pod_attrs(), protected by CPU hotplug exclusion */
static struct workqueue_attrs *unbound_wq_update_pwq_attrs_buf;

static DEFINE_MUTEX(wq_pool_mutex);	/* protects pools and workqueues list */
static DEFINE_MUTEX(wq_pool_attach_mutex); /* protects worker attach/detach */
static DEFINE_RAW_SPINLOCK(wq_mayday_lock);	/* protects wq->maydays list */
/* wait for manager to go away */
static struct rcuwait manager_wait = __RCUWAIT_INITIALIZER(manager_wait);

static LIST_HEAD(workqueues);		/* PR: list of all workqueues */
static bool workqueue_freezing;		/* PL: have wqs started freezing? */

/* PL: mirror the cpu_online_mask excluding the CPU in the midst of hotplugging */
static cpumask_var_t wq_online_cpumask;

/* PL&A: allowable cpus for unbound wqs and work items */
static cpumask_var_t wq_unbound_cpumask;

/* PL: user requested unbound cpumask via sysfs */
static cpumask_var_t wq_requested_unbound_cpumask;

/* PL: isolated cpumask to be excluded from unbound cpumask */
static cpumask_var_t wq_isolated_cpumask;

/* for further constrain wq_unbound_cpumask by cmdline parameter*/
static struct cpumask wq_cmdline_cpumask __initdata;

/* CPU where unbound work was last round robin scheduled from this CPU */
static DEFINE_PER_CPU(int, wq_rr_cpu_last);

/*
 * Local execution of unbound work items is no longer guaranteed.  The
 * following always forces round-robin CPU selection on unbound work items
 * to uncover usages which depend on it.
 */
#ifdef CONFIG_DEBUG_WQ_FORCE_RR_CPU
static bool wq_debug_force_rr_cpu = true;
#else
static bool wq_debug_force_rr_cpu = false;
#endif
module_param_named(debug_force_rr_cpu, wq_debug_force_rr_cpu, bool, 0644);

/* to raise softirq for the BH worker pools on other CPUs */
static DEFINE_PER_CPU_SHARED_ALIGNED(struct irq_work [NR_STD_WORKER_POOLS], bh_pool_irq_works);

/* the BH worker pools */
static DEFINE_PER_CPU_SHARED_ALIGNED(struct worker_pool [NR_STD_WORKER_POOLS], bh_worker_pools);

/* the per-cpu worker pools */
static DEFINE_PER_CPU_SHARED_ALIGNED(struct worker_pool [NR_STD_WORKER_POOLS], cpu_worker_pools);

static DEFINE_IDR(worker_pool_idr);	/* PR: idr of all pools */

/* PL: hash of all unbound pools keyed by pool->attrs */
static DEFINE_HASHTABLE(unbound_pool_hash, UNBOUND_POOL_HASH_ORDER);

/* I: attributes used when instantiating standard unbound pools on demand */
static struct workqueue_attrs *unbound_std_wq_attrs[NR_STD_WORKER_POOLS];

/* I: attributes used when instantiating ordered pools on demand */
static struct workqueue_attrs *ordered_wq_attrs[NR_STD_WORKER_POOLS];

/*
 * I: kthread_worker to release pwq's. pwq release needs to be bounced to a
 * process context while holding a pool lock. Bounce to a dedicated kthread
 * worker to avoid A-A deadlocks.
 */
static struct kthread_worker *pwq_release_worker __ro_after_init;

struct workqueue_struct *system_wq __ro_after_init;
EXPORT_SYMBOL(system_wq);
struct workqueue_struct *system_percpu_wq __ro_after_init;
EXPORT_SYMBOL(system_percpu_wq);
struct workqueue_struct *system_highpri_wq __ro_after_init;
EXPORT_SYMBOL_GPL(system_highpri_wq);
struct workqueue_struct *system_long_wq __ro_after_init;
EXPORT_SYMBOL_GPL(system_long_wq);
struct workqueue_struct *system_unbound_wq __ro_after_init;
EXPORT_SYMBOL_GPL(system_unbound_wq);
struct workqueue_struct *system_dfl_wq __ro_after_init;
EXPORT_SYMBOL_GPL(system_dfl_wq);
struct workqueue_struct *system_freezable_wq __ro_after_init;
EXPORT_SYMBOL_GPL(system_freezable_wq);
struct workqueue_struct *system_power_efficient_wq __ro_after_init;
EXPORT_SYMBOL_GPL(system_power_efficient_wq);
struct workqueue_struct *system_freezable_power_efficient_wq __ro_after_init;
EXPORT_SYMBOL_GPL(system_freezable_power_efficient_wq);
struct workqueue_struct *system_bh_wq;
EXPORT_SYMBOL_GPL(system_bh_wq);
struct workqueue_struct *system_bh_highpri_wq;
EXPORT_SYMBOL_GPL(system_bh_highpri_wq);
struct workqueue_struct *system_dfl_long_wq __ro_after_init;
EXPORT_SYMBOL_GPL(system_dfl_long_wq);
/* Exactly one tracepoint-definition owner; configured inputs remain required. */
#define CREATE_TRACE_POINTS
#include <trace/events/workqueue.h>

unsigned long lupos_wq_work_data_read(struct work_struct *work)
{
	return atomic_long_read(&work->data);
}

void lupos_wq_work_data_set(struct work_struct *work, unsigned long data)
{
	atomic_long_set(&work->data, data);
}

unsigned int lupos_wq_work_static(struct work_struct *work)
{
	return work_static(work);
}

void lupos_wq_warn_set_work_data_not_pending(struct work_struct *work)
{
	WARN_ON_ONCE(!work_pending(work));
}

void lupos_wq_warn_work_offqd_unpack_pwq(unsigned long data)
{
	WARN_ON_ONCE(data & WORK_STRUCT_PWQ);
}

void lupos_wq_assert_rcu_or_pool_mutex(void)
{
	RCU_LOCKDEP_WARN(!rcu_read_lock_any_held() &&
			 !lockdep_is_held(&wq_pool_mutex),
			 "RCU or wq_pool_mutex should be held");
}

void lupos_wq_assert_pool_mutex_held(void)
{
	lockdep_assert_held(&wq_pool_mutex);
}

struct idr *lupos_wq_worker_pool_idr(void)
{
	return &worker_pool_idr;
}

struct mutex *lupos_wq_pool_mutex(void)
{
	return &wq_pool_mutex;
}

struct mutex *lupos_wq_pool_attach_mutex(void)
{
	return &wq_pool_attach_mutex;
}

struct raw_spinlock *lupos_wq_mayday_lock(void)
{
	return &wq_mayday_lock;
}

struct rcuwait *lupos_wq_manager_wait(void)
{
	return &manager_wait;
}

struct list_head *lupos_wq_workqueues(void)
{
	return &workqueues;
}

struct pool_workqueue __rcu **lupos_wq_percpu_pwq_slot(struct workqueue_struct *wq,
					     int cpu)
{
	return per_cpu_ptr(wq->cpu_pwq, cpu);
}

struct pool_workqueue *lupos_wq_unbound_pwq_dereference(
		struct pool_workqueue __rcu **slot, struct workqueue_struct *wq)
{
	return rcu_dereference_check(*slot,
		lockdep_is_held(&wq_pool_mutex) || lockdep_is_held(&wq->mutex));
}

struct cpumask *lupos_wq_attrs_cpumask(struct workqueue_attrs *attrs)
{
	return attrs->cpumask;
}

struct cpumask *lupos_wq_attrs_pod_cpumask(struct workqueue_attrs *attrs)
{
	return attrs->__pod_cpumask;
}

struct list_head *lupos_wq_worker_entry(struct worker *worker)
{
	return &worker->entry;
}

struct hlist_node *lupos_wq_worker_hentry(struct worker *worker)
{
	return &worker->hentry;
}
