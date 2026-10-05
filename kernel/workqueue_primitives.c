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

/* Generic native primitives; all workqueue decisions remain in Rust. */
void lupos_wq_local_irq_disable(void)
{
	local_irq_disable();
}
void lupos_wq_local_irq_enable(void)
{
	local_irq_enable();
}
void lupos_wq_local_irq_restore(unsigned long flags)
{
	local_irq_restore(flags);
}
void lupos_wq_preempt_disable(void)
{
	preempt_disable();
}
void lupos_wq_preempt_enable(void)
{
	preempt_enable();
}
void lupos_wq_cpu_relax(void)
{
	cpu_relax();
}
int lupos_wq_cond_resched(void)
{
	return cond_resched();
}
int lupos_wq_raw_smp_processor_id(void)
{
	return raw_smp_processor_id();
}
bool lupos_wq_in_atomic(void)
{
	return in_atomic();
}
bool lupos_wq_in_hardirq(void)
{
	return in_hardirq();
}
bool lupos_wq_in_interrupt(void)
{
	return in_interrupt();
}
void lupos_wq_rcu_barrier(void)
{
	rcu_barrier();
}
int lupos_wq_rcu_preempt_depth(void)
{
	return rcu_preempt_depth();
}
void lupos_wq_destroy_work_on_stack(struct work_struct *work)
{
	destroy_work_on_stack(work);
}
int lupos_wq_timer_delete_sync(struct timer_list *timer)
{
	return timer_delete_sync(timer);
}
void lupos_wq_wait_for_completion(struct completion *done)
{
	wait_for_completion(done);
}
bool lupos_wq_try_wait_for_completion(struct completion *done)
{
	return try_wait_for_completion(done);
}
void lupos_wq_list_splice_tail_init(struct list_head *list, struct list_head *head)
{
	list_splice_tail_init(list, head);
}
void lupos_wq_list_add_tail_rcu(struct list_head *entry, struct list_head *head)
{
	list_add_tail_rcu(entry, head);
}
void lupos_wq_list_del_rcu(struct list_head *entry)
{
	list_del_rcu(entry);
}
void lupos_wq_hlist_add_head(struct hlist_node *node, struct hlist_head *head)
{
	hlist_add_head(node, head);
}
void lupos_wq_hash_del(struct hlist_node *node)
{
	hash_del(node);
}
void lupos_wq_init_hlist_node(struct hlist_node *node)
{
	INIT_HLIST_NODE(node);
}
void lupos_wq_ida_init(struct ida *ida)
{
	ida_init(ida);
}
void * lupos_wq_err_ptr(long error)
{
	return ERR_PTR(error);
}
bool lupos_wq_test_bit(unsigned int bit, const unsigned long *word)
{
	return test_bit(bit, word);
}
bool lupos_wq_test_and_set_bit(long bit, unsigned long *word)
{
	return test_and_set_bit(bit, word);
}
void lupos_wq_kasan_record_aux_stack(struct work_struct *work)
{
	kasan_record_aux_stack(work);
}
unsigned long lupos_wq_bh_worker_jiffies(void)
{
	return BH_WORKER_JIFFIES;
}
bool lupos_wq_debug_force_rr_cpu(void)
{
	return wq_debug_force_rr_cpu;
}
unsigned long lupos_wq_cpu_intensive_thresh_us(void)
{
	return wq_cpu_intensive_thresh_us;
}
u64 lupos_wq_read_task_sum_exec_runtime(struct task_struct *task)
{
	return READ_ONCE(task->se.sum_exec_runtime);
}
int lupos_wq_pool_nr_idle_plain(struct worker_pool *pool)
{
	return pool->nr_idle;
}
int lupos_wq_pool_nr_running_plain(struct worker_pool *pool)
{
	return pool->nr_running;
}
int lupos_wq_this_cpu_rr_read(void)
{
	return __this_cpu_read(wq_rr_cpu_last);
}
void lupos_wq_this_cpu_rr_write(int cpu)
{
	__this_cpu_write(wq_rr_cpu_last, cpu);
}
struct delayed_work * lupos_wq_to_delayed_work(struct work_struct *work)
{
	return to_delayed_work(work);
}
struct delayed_work * lupos_wq_delayed_work_from_timer(struct timer_list *timer)
{
	return container_of(timer, struct delayed_work, timer);
}
struct rcu_work * lupos_wq_rcu_work_from_rcu(struct rcu_head *rcu)
{
	return container_of(rcu, struct rcu_work, rcu);
}
struct worker_pool * lupos_wq_pool_from_rcu(struct rcu_head *rcu)
{
	return container_of(rcu, struct worker_pool, rcu);
}
struct workqueue_struct * lupos_wq_wq_from_rcu(struct rcu_head *rcu)
{
	return container_of(rcu, struct workqueue_struct, rcu);
}
struct worker_pool * lupos_wq_pool_from_hash_node(struct hlist_node *node)
{
	return container_of(node, struct worker_pool, hash_node);
}
struct pool_workqueue * lupos_wq_pwq_from_release_work(struct kthread_work *work)
{
	return container_of(work, struct pool_workqueue, release_work);
}
struct wq_node_nr_active ** lupos_wq_node_nr_active_array(struct workqueue_struct *wq)
{
	return wq->node_nr_active;
}
struct wq_node_nr_active ** lupos_wq_node_nr_active_slot(struct workqueue_struct *wq, int node)
{
	return &wq->node_nr_active[node];
}
struct pool_workqueue ** lupos_wq_ctx_pwq_slot(struct apply_wqattrs_ctx *ctx, int cpu)
{
	return &ctx->pwq_tbl[cpu];
}
char * lupos_wq_wq_name(struct workqueue_struct *wq)
{
	return wq->name;
}
struct workqueue_attrs * lupos_wq_update_pwq_attrs_buf(void)
{
	return unbound_wq_update_pwq_attrs_buf;
}
struct workqueue_attrs * lupos_wq_std_unbound_attrs(int highpri)
{
	return unbound_std_wq_attrs[highpri];
}
struct workqueue_attrs * lupos_wq_ordered_attrs(int highpri)
{
	return ordered_wq_attrs[highpri];
}
const struct cpumask * lupos_wq_online_cpumask(void)
{
	return wq_online_cpumask;
}
struct wq_pod_type * lupos_wq_pod_type(int scope)
{
	return &wq_pod_types[scope];
}
enum wq_affn_scope lupos_wq_affn_dfl(void)
{
	return wq_affn_dfl;
}
const struct cpumask * lupos_wq_pod_cpus(const struct wq_pod_type *type, int pod)
{
	return type->pod_cpus[pod];
}
void lupos_wq_cpumask_copy(struct cpumask *to, const struct cpumask *from)
{
	cpumask_copy(to, from);
}
bool lupos_wq_cpumask_equal(const struct cpumask *a, const struct cpumask *b)
{
	return cpumask_equal(a, b);
}
bool lupos_wq_cpumask_and(struct cpumask *to, const struct cpumask *a, const struct cpumask *b)
{
	return cpumask_and(to, a, b);
}
bool lupos_wq_cpumask_empty(const struct cpumask *mask)
{
	return cpumask_empty(mask);
}
bool lupos_wq_cpumask_subset(const struct cpumask *a, const struct cpumask *b)
{
	return cpumask_subset(a, b);
}
const unsigned long * lupos_wq_cpumask_bits(const struct cpumask *mask)
{
	return cpumask_bits(mask);
}
u32 lupos_wq_cpumask_hash_bytes(void)
{
	return BITS_TO_LONGS(nr_cpumask_bits) * sizeof(long);
}
unsigned int lupos_wq_cpumask_any_and(const struct cpumask *a, const struct cpumask *b)
{
	return cpumask_any_and(a, b);
}
unsigned int lupos_wq_cpumask_next_and_wrap(int cpu, const struct cpumask *a, const struct cpumask *b)
{
	return cpumask_next_and_wrap(cpu, a, b);
}
const struct cpumask * lupos_wq_cpumask_of_node(int node)
{
	return cpumask_of_node(node);
}
int lupos_wq_cpu_to_node(int cpu)
{
	return cpu_to_node(cpu);
}
int lupos_wq_nr_node_ids(void)
{
	return nr_node_ids;
}
bool lupos_wq_node_online(int node)
{
	return node_online(node);
}
bool lupos_wq_cpu_online(int cpu)
{
	return cpu_online(cpu);
}
bool lupos_wq_housekeeping_enabled(enum hk_type type)
{
	return housekeeping_enabled(type);
}
bool lupos_wq_housekeeping_test_cpu(int cpu, enum hk_type type)
{
	return housekeeping_test_cpu(cpu, type);
}
int lupos_wq_housekeeping_any_cpu(enum hk_type type)
{
	return housekeeping_any_cpu(type);
}
void lupos_wq_init_pool_busy_hash(struct worker_pool *pool)
{
	hash_init(pool->busy_hash);
}
void lupos_wq_free_attrs_cpumask(struct workqueue_attrs *attrs)
{
	free_cpumask_var(attrs->cpumask);
}
void lupos_wq_free_attrs_pod_cpumask(struct workqueue_attrs *attrs)
{
	free_cpumask_var(attrs->__pod_cpumask);
}
void lupos_wq_cpus_read_lock(void)
{
	cpus_read_lock();
}
void lupos_wq_cpus_read_unlock(void)
{
	cpus_read_unlock();
}
void lupos_wq_prepare_to_rcuwait(struct rcuwait *wait)
{
	prepare_to_rcuwait(wait);
}
bool lupos_wq_signal_pending_state(unsigned int state, struct task_struct *task)
{
	return signal_pending_state(state, task);
}
struct wq_flusher * lupos_wq_read_first_flusher(struct workqueue_struct *wq)
{
	return READ_ONCE(wq->first_flusher);
}
void lupos_wq_write_first_flusher(struct workqueue_struct *wq, struct wq_flusher *flusher)
{
	WRITE_ONCE(wq->first_flusher, flusher);
}
struct pool_workqueue * lupos_wq_pwq_rcu_access(struct pool_workqueue __rcu *const *slot)
{
	return rcu_access_pointer(*slot);
}
void lupos_wq_pwq_rcu_assign(struct pool_workqueue __rcu **slot, struct pool_workqueue *pwq)
{
	rcu_assign_pointer(*slot, pwq);
}
struct pool_workqueue * lupos_wq_cpu_pwq_dereference(struct workqueue_struct *wq, int cpu)
{
	return rcu_dereference(*per_cpu_ptr(wq->cpu_pwq, cpu));
}
struct worker_pool * lupos_wq_cpu_pool(int cpu, int highpri)
{
	return &per_cpu(cpu_worker_pools, cpu)[highpri];
}
unsigned long lupos_wq_local_irq_save(void)
{
	unsigned long flags;
	local_irq_save(flags);
	return flags;
}
void lupos_wq_rcu_read_lock(void) { rcu_read_lock(); }
void lupos_wq_rcu_read_unlock(void) { rcu_read_unlock(); }
void lupos_wq_raw_spin_lock(raw_spinlock_t *lock) { raw_spin_lock(lock); }
void lupos_wq_raw_spin_unlock(raw_spinlock_t *lock) { raw_spin_unlock(lock); }
void lupos_wq_raw_spin_lock_irq(raw_spinlock_t *lock) { raw_spin_lock_irq(lock); }
void lupos_wq_raw_spin_unlock_irq(raw_spinlock_t *lock) { raw_spin_unlock_irq(lock); }
void lupos_wq_nna_lock_irq(struct wq_node_nr_active *nna) { raw_spin_lock_irq(&nna->lock); }
void lupos_wq_nna_unlock_irq(struct wq_node_nr_active *nna) { raw_spin_unlock_irq(&nna->lock); }
#ifdef CONFIG_PREEMPT_RT
void lupos_wq_spin_lock(spinlock_t *lock) { spin_lock(lock); }
void lupos_wq_spin_unlock(spinlock_t *lock) { spin_unlock(lock); }
#endif
/* State-site identities remain distinct; original emitted attribution is held. */
void lupos_wq_set_current_state(unsigned int state) { set_current_state(state); }
void lupos_wq_set_state_worker_thread_idle(void) { __set_current_state(TASK_IDLE); }
void lupos_wq_set_state_rescuer_thread_idle(void) { set_current_state(TASK_IDLE); }
void lupos_wq_set_state_rescuer_thread_running(void) { __set_current_state(TASK_RUNNING); }
void lupos_wq_set_state_rescuer_thread_stopping(void) { __set_current_state(TASK_RUNNING); }
void lupos_wq_pwq_cpu_time_add_data_race(struct pool_workqueue *pwq, u64 delta)
{
	/* Native advisory scalar update, intentionally not an atomic fetch-add. */
	data_race(pwq->stats[PWQ_STAT_CPU_TIME] += delta);
}
struct work_struct __percpu *lupos_wq_alloc_percpu_work(void)
{
	return alloc_percpu(struct work_struct);
}
void lupos_wq_free_percpu_work(struct work_struct __percpu *works) { free_percpu(works); }
struct work_struct *lupos_wq_percpu_work_at(struct work_struct __percpu *works, int cpu)
{
	return per_cpu_ptr(works, cpu);
}
void lupos_wq_free_cpu_pwq(struct workqueue_struct *wq) { free_percpu(wq->cpu_pwq); }
bool lupos_wq_has_cpu_pwq(struct workqueue_struct *wq) { return wq->cpu_pwq != NULL; }
void lupos_wq_clear_cpu_pwq(struct workqueue_struct *wq) { wq->cpu_pwq = NULL; }
int lupos_wq_first_online_cpu(void)
{
#if NR_CPUS == 1
	return 0;
#else
	unsigned int cpu = find_next_bit(cpumask_bits(cpu_online_mask),
					small_cpumask_bits, 0);
	return cpu < small_cpumask_bits ? (int)cpu : -1;
#endif
}
int lupos_wq_next_online_cpu(int cpu)
{
#if NR_CPUS == 1
	return -1;
#else
	unsigned int next = find_next_bit(cpumask_bits(cpu_online_mask),
					 small_cpumask_bits, cpu + 1);
	return next < small_cpumask_bits ? (int)next : -1;
#endif
}
unsigned int lupos_wq_first_possible_cpu(void)
{
#if NR_CPUS == 1
	return 0;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask),
			     small_cpumask_bits, 0);
#endif
}
unsigned int lupos_wq_next_possible_cpu(unsigned int cpu)
{
#if NR_CPUS == 1
	return cpu + 1;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask),
			     small_cpumask_bits, cpu + 1);
#endif
}
int lupos_wq_first_possible_node(void)
{
#if MAX_NUMNODES > 1
	return first_node(node_states[N_POSSIBLE]);
#else
	return 0;
#endif
}
int lupos_wq_next_possible_node(int node)
{
#if MAX_NUMNODES > 1
	return next_node(node, node_states[N_POSSIBLE]);
#else
	return 1;
#endif
}
void lupos_wq_init_completion_c3871(struct completion *done)
{
	init_completion(done);
}
void lupos_wq_init_completion_wq_map(struct completion *done, struct workqueue_struct *wq)
{
	/* Pinned init_completion_map discards this macro operand on all configs. */
	init_completion_map(done, &(*wq->lockdep_map));
}
bool lupos_wq_timer_func_eq(lupos_wq_timer_fn_t left, lupos_wq_timer_fn_t right)
{
	return left == right;
}
void lupos_wq_trace_workqueue_queue_work(int cpu, struct pool_workqueue *pwq,
				        struct work_struct *work)
{
	trace_workqueue_queue_work(cpu, pwq, work);
}
void lupos_wq_trace_process_one_work_execute_start(struct work_struct *work)
{
	trace_workqueue_execute_start(work);
}
void lupos_wq_trace_process_one_work_execute_end(struct work_struct *work, work_func_t func)
{
	/* Exact recovered TP_fast_assign records only the address and func value. */
	trace_workqueue_execute_end(work, func);
}
