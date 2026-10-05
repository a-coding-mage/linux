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

/* Native primitive leaves only: scheduling and lifetime decisions stay Rust. */
void lupos_wq_pool_lock(struct worker_pool *pool) { raw_spin_lock(&pool->lock); }
void lupos_wq_pool_unlock(struct worker_pool *pool) { raw_spin_unlock(&pool->lock); }
void lupos_wq_pool_lock_irq(struct worker_pool *pool) { raw_spin_lock_irq(&pool->lock); }
void lupos_wq_pool_unlock_irq(struct worker_pool *pool) { raw_spin_unlock_irq(&pool->lock); }
unsigned long lupos_wq_pool_lock_irqsave(struct worker_pool *pool)
{
	unsigned long flags;
	raw_spin_lock_irqsave(&pool->lock, flags);
	return flags;
}
void lupos_wq_pool_unlock_irqrestore(struct worker_pool *pool, unsigned long flags)
{
	raw_spin_unlock_irqrestore(&pool->lock, flags);
}
bool lupos_wq_pool_trylock(struct worker_pool *pool) { return raw_spin_trylock(&pool->lock); }
void lupos_wq_nna_lock(struct wq_node_nr_active *nna) { raw_spin_lock(&nna->lock); }
void lupos_wq_nna_unlock(struct wq_node_nr_active *nna) { raw_spin_unlock(&nna->lock); }
void lupos_wq_assert_pool_locked(struct worker_pool *pool) { lockdep_assert_held(&pool->lock); }
void lupos_wq_assert_wq_mutex_held(struct workqueue_struct *wq) { lockdep_assert_held(&wq->mutex); }
void lupos_wq_mutex_lock(struct mutex *lock) { mutex_lock(lock); }
void lupos_wq_mutex_unlock(struct mutex *lock) { mutex_unlock(lock); }
bool lupos_wq_list_empty(const struct list_head *head) { return list_empty(head); }
void lupos_wq_init_list_head(struct list_head *head) { INIT_LIST_HEAD(head); }
void lupos_wq_list_add(struct list_head *entry, struct list_head *head) { list_add(entry, head); }
void lupos_wq_list_add_tail(struct list_head *entry, struct list_head *head) { list_add_tail(entry, head); }
void lupos_wq_list_del(struct list_head *entry) { list_del(entry); }
void lupos_wq_list_del_init(struct list_head *entry) { list_del_init(entry); }
void lupos_wq_list_move(struct list_head *entry, struct list_head *head) { list_move(entry, head); }
void lupos_wq_list_move_tail(struct list_head *entry, struct list_head *head) { list_move_tail(entry, head); }
void lupos_wq_list_splice_init(struct list_head *list, struct list_head *head) { list_splice_init(list, head); }
struct worker *lupos_wq_worker_from_entry(struct list_head *entry)
{
	return container_of(entry, struct worker, entry);
}
struct worker *lupos_wq_worker_from_hentry(struct hlist_node *entry)
{
	return container_of(entry, struct worker, hentry);
}
struct work_struct *lupos_wq_work_from_entry(struct list_head *entry)
{
	/* Native list-next sentinel conversion is intentionally address-only. */
	return container_of(entry, struct work_struct, entry);
}
struct pool_workqueue *lupos_wq_pwq_from_pwqs_node(struct list_head *entry)
{
	return container_of(entry, struct pool_workqueue, pwqs_node);
}
struct pool_workqueue *lupos_wq_pwq_from_pending_node(struct list_head *entry)
{
	return container_of(entry, struct pool_workqueue, pending_node);
}
struct hlist_head *lupos_wq_busy_worker_hash_head(struct worker_pool *pool,
					       struct work_struct *work)
{
	return &pool->busy_hash[hash_min((unsigned long)work, HASH_BITS(pool->busy_hash))];
}
struct task_struct *lupos_wq_current(void) { return current; }
struct worker *lupos_wq_current_wq_worker(void) { return current_wq_worker(); }
bool lupos_wq_work_func_eq(work_func_t left, work_func_t right) { return left == right; }
unsigned long lupos_wq_read_jiffies(void) { return jiffies; }
bool lupos_wq_timer_pending(const struct timer_list *timer) { return timer_pending(timer); }
bool lupos_wq_work_pending(const struct work_struct *work) { return work_pending(work); }
bool lupos_wq_time_before(unsigned long left, unsigned long right) { return time_before(left, right); }
int lupos_wq_read_once_int(const int *value) { return READ_ONCE(*value); }
void lupos_wq_write_once_int(int *value, int new_value) { WRITE_ONCE(*value, new_value); }
int lupos_wq_atomic_read(const atomic_t *value) { return atomic_read(value); }
void lupos_wq_atomic_set(atomic_t *value, int new_value) { atomic_set(value, new_value); }
void lupos_wq_atomic_inc(atomic_t *value) { atomic_inc(value); }
bool lupos_wq_atomic_dec_and_test(atomic_t *value) { return atomic_dec_and_test(value); }
int lupos_wq_atomic_dec_return(atomic_t *value) { return atomic_dec_return(value); }
bool lupos_wq_atomic_try_cmpxchg_relaxed(atomic_t *value, int *old, int new_value)
{
	return atomic_try_cmpxchg_relaxed(value, old, new_value);
}
void lupos_wq___set_bit(unsigned int bit, unsigned long *data) { __set_bit(bit, data); }
void lupos_wq___clear_bit(unsigned int bit, unsigned long *data) { __clear_bit(bit, data); }
void lupos_wq_complete(struct completion *done) { complete(done); }
void lupos_wq_local_bh_disable(void) { local_bh_disable(); }
void lupos_wq_local_bh_enable(void) { local_bh_enable(); }

/* Original insert_wq_barrier keys remain distinct, static and address-stable. */
static __maybe_unused struct lock_class_key wq_barrier_bh_key, wq_barrier_thr_key;
struct lock_class_key *lupos_wq_barrier_bh_key(void) { return &wq_barrier_bh_key; }
struct lock_class_key *lupos_wq_barrier_thr_key(void) { return &wq_barrier_thr_key; }
void lupos_wq_init_completion_work_map(struct completion *done, struct work_struct *target)
{
	/* The pinned native macro erases its map operand when that field is absent. */
	init_completion_map(done, &target->lockdep_map);
}
struct pool_workqueue *lupos_wq_pwq_first(struct workqueue_struct *wq)
{
	struct pool_workqueue *pwq;
	__list_check_rcu(dummy, lockdep_is_held(&wq->mutex), 0);
	pwq = list_entry_rcu(wq->pwqs.next, struct pool_workqueue, pwqs_node);
	return &pwq->pwqs_node == &wq->pwqs ? NULL : pwq;
}
struct pool_workqueue *lupos_wq_pwq_next(struct workqueue_struct *wq,
				      struct pool_workqueue *pwq)
{
	pwq = list_entry_rcu(pwq->pwqs_node.next, struct pool_workqueue, pwqs_node);
	return &pwq->pwqs_node == &wq->pwqs ? NULL : pwq;
}
void *lupos_wq_kzalloc_node(size_t size, gfp_t flags, int node) { return kzalloc_node(size, flags, node); }
int lupos_wq_ida_alloc(struct ida *ida, gfp_t flags) { return ida_alloc(ida, flags); }
bool lupos_wq_is_err(const void *value) { return IS_ERR(value); }
long lupos_wq_ptr_err(const void *value) { return PTR_ERR(value); }
void lupos_wq_get_task_struct(struct task_struct *task) { get_task_struct(task); }
struct kthread_worker *lupos_wq_pwq_release_worker(void) { return pwq_release_worker; }
bool lupos_wq_topo_initialized(void) { return wq_topo_initialized; }
const struct cpumask *lupos_wq_unbound_cpumask(void) { return wq_unbound_cpumask; }
const struct cpumask *lupos_wq_cpu_online_mask(void) { return cpu_online_mask; }
const struct cpumask *lupos_wq_cpu_active_mask(void) { return cpu_active_mask; }
const struct cpumask *lupos_wq_cpu_possible_mask(void) { return cpu_possible_mask; }
unsigned int lupos_wq_nr_cpu_ids(void) { return nr_cpu_ids; }
unsigned int lupos_wq_smp_processor_id(void) { return smp_processor_id(); }
bool lupos_wq_cpumask_test_cpu(int cpu, const struct cpumask *mask) { return cpumask_test_cpu(cpu, mask); }
bool lupos_wq_cpumask_intersects(const struct cpumask *a, const struct cpumask *b)
{
	return cpumask_intersects(a, b);
}
unsigned int lupos_wq_cpumask_weight_and(const struct cpumask *a, const struct cpumask *b)
{
	return cpumask_weight_and(a, b);
}
unsigned int lupos_wq_cpumask_any_and_distribute(const struct cpumask *a, const struct cpumask *b)
{
	return cpumask_any_and_distribute(a, b);
}
struct irq_work *lupos_wq_bh_pool_irq_work_at(int cpu, int index)
{
	return &per_cpu(bh_pool_irq_works, cpu)[index];
}
void lupos_wq_raise_softirq_irqoff(unsigned int nr) { raise_softirq_irqoff(nr); }
#ifdef CONFIG_SMP
int lupos_wq_read_task_wake_cpu(struct task_struct *task) { return READ_ONCE(task->wake_cpu); }
void lupos_wq_write_task_wake_cpu(struct task_struct *task, int cpu) { WRITE_ONCE(task->wake_cpu, cpu); }
#endif
void lupos_wq_trace_workqueue_activate_work(struct work_struct *work)
{
	trace_workqueue_activate_work(work);
}

bool lupos_wq_online(void) { return wq_online; }
struct list_head *lupos_wq_work_entry(struct work_struct *work)
{
	/* Address-only inverse of work_from_entry, including native list sentinels. */
	return &work->entry;
}
struct worker_pool *lupos_wq_bh_pool_at(int cpu, int index)
{
	return &per_cpu(bh_worker_pools, cpu)[index];
}
static __maybe_unused struct lock_class_key schedule_each_cpu_key;
static __maybe_unused struct lock_class_key execute_process_context_key;
static __maybe_unused struct lock_class_key dead_softirq_work_key;
struct lock_class_key *lupos_wq_schedule_each_cpu_key(void)
{
	return &schedule_each_cpu_key;
}
struct lock_class_key *lupos_wq_execute_process_context_key(void)
{
	return &execute_process_context_key;
}
struct lock_class_key *lupos_wq_dead_softirq_work_key(void)
{
	return &dead_softirq_work_key;
}
