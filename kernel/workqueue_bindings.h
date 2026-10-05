/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_WORKQUEUE_BINDINGS_H
#define LUPOS_WORKQUEUE_BINDINGS_H

#include "workqueue_private.h"

/* Native declarations, not a hand-projected Rust layout or generated output. */
unsigned long lupos_wq_work_data_read(struct work_struct *work);
void lupos_wq_work_data_set(struct work_struct *work, unsigned long data);
unsigned int lupos_wq_work_static(struct work_struct *work);
void lupos_wq_warn_set_work_data_not_pending(struct work_struct *work);
void lupos_wq_warn_work_offqd_unpack_pwq(unsigned long data);
void lupos_wq_assert_rcu_or_pool_mutex(void);
void lupos_wq_assert_pool_mutex_held(void);
struct idr *lupos_wq_worker_pool_idr(void);
struct mutex *lupos_wq_pool_mutex(void);
struct mutex *lupos_wq_pool_attach_mutex(void);
struct raw_spinlock *lupos_wq_mayday_lock(void);
struct rcuwait *lupos_wq_manager_wait(void);
struct list_head *lupos_wq_workqueues(void);
struct pool_workqueue __rcu **lupos_wq_percpu_pwq_slot(struct workqueue_struct *wq,
                                              int cpu);
struct pool_workqueue *lupos_wq_unbound_pwq_dereference(
        struct pool_workqueue __rcu **slot, struct workqueue_struct *wq);
struct cpumask *lupos_wq_attrs_cpumask(struct workqueue_attrs *attrs);
struct cpumask *lupos_wq_attrs_pod_cpumask(struct workqueue_attrs *attrs);
struct list_head *lupos_wq_worker_entry(struct worker *worker);
struct hlist_node *lupos_wq_worker_hentry(struct worker *worker);

/* Common primitive requests, first source-reviewed A/B/C foundations. */
void lupos_wq_pool_lock(struct worker_pool *pool) __acquires(&pool->lock);
void lupos_wq_pool_unlock(struct worker_pool *pool) __releases(&pool->lock);
void lupos_wq_pool_lock_irq(struct worker_pool *pool) __acquires(&pool->lock);
void lupos_wq_pool_unlock_irq(struct worker_pool *pool) __releases(&pool->lock);
unsigned long lupos_wq_pool_lock_irqsave(struct worker_pool *pool)
	__acquires(&pool->lock);
void lupos_wq_pool_unlock_irqrestore(struct worker_pool *pool, unsigned long flags)
	__releases(&pool->lock);
bool lupos_wq_pool_trylock(struct worker_pool *pool)
	__cond_acquires(true, &pool->lock);
void lupos_wq_nna_lock(struct wq_node_nr_active *nna) __acquires(&nna->lock);
void lupos_wq_nna_unlock(struct wq_node_nr_active *nna) __releases(&nna->lock);
void lupos_wq_assert_pool_locked(struct worker_pool *pool);
void lupos_wq_assert_wq_mutex_held(struct workqueue_struct *wq);
void lupos_wq_mutex_lock(struct mutex *lock) __acquires(lock);
void lupos_wq_mutex_unlock(struct mutex *lock) __releases(lock);
bool lupos_wq_list_empty(const struct list_head *head);
void lupos_wq_init_list_head(struct list_head *head);
void lupos_wq_list_add(struct list_head *entry, struct list_head *head);
void lupos_wq_list_add_tail(struct list_head *entry, struct list_head *head);
void lupos_wq_list_del(struct list_head *entry);
void lupos_wq_list_del_init(struct list_head *entry);
void lupos_wq_list_move(struct list_head *entry, struct list_head *head);
void lupos_wq_list_move_tail(struct list_head *entry, struct list_head *head);
void lupos_wq_list_splice_init(struct list_head *list, struct list_head *head);
struct worker *lupos_wq_worker_from_entry(struct list_head *entry);
struct worker *lupos_wq_worker_from_hentry(struct hlist_node *entry);
struct work_struct *lupos_wq_work_from_entry(struct list_head *entry);
struct pool_workqueue *lupos_wq_pwq_from_pwqs_node(struct list_head *entry);
struct pool_workqueue *lupos_wq_pwq_from_pending_node(struct list_head *entry);
struct hlist_head *lupos_wq_busy_worker_hash_head(struct worker_pool *pool,
					       struct work_struct *work);
struct task_struct *lupos_wq_current(void);
struct worker *lupos_wq_current_wq_worker(void);
bool lupos_wq_work_func_eq(work_func_t left, work_func_t right);
unsigned long lupos_wq_read_jiffies(void);
bool lupos_wq_timer_pending(const struct timer_list *timer);
bool lupos_wq_work_pending(const struct work_struct *work);
bool lupos_wq_time_before(unsigned long left, unsigned long right);
int lupos_wq_read_once_int(const int *value);
void lupos_wq_write_once_int(int *value, int new_value);
int lupos_wq_atomic_read(const atomic_t *value);
void lupos_wq_atomic_set(atomic_t *value, int new_value);
void lupos_wq_atomic_inc(atomic_t *value);
bool lupos_wq_atomic_dec_and_test(atomic_t *value);
int lupos_wq_atomic_dec_return(atomic_t *value);
bool lupos_wq_atomic_try_cmpxchg_relaxed(atomic_t *value, int *old, int new_value);
void lupos_wq___set_bit(unsigned int bit, unsigned long *data);
void lupos_wq___clear_bit(unsigned int bit, unsigned long *data);
void lupos_wq_complete(struct completion *done);
void lupos_wq_local_bh_disable(void);
void lupos_wq_local_bh_enable(void);
struct lock_class_key *lupos_wq_barrier_bh_key(void);
struct lock_class_key *lupos_wq_barrier_thr_key(void);
void lupos_wq_init_completion_work_map(struct completion *done,
				     struct work_struct *target);
struct pool_workqueue *lupos_wq_pwq_first(struct workqueue_struct *wq);
struct pool_workqueue *lupos_wq_pwq_next(struct workqueue_struct *wq,
				      struct pool_workqueue *pwq);
void *lupos_wq_kzalloc_node(size_t size, gfp_t flags, int node);
int lupos_wq_ida_alloc(struct ida *ida, gfp_t flags);
bool lupos_wq_is_err(const void *value);
long lupos_wq_ptr_err(const void *value);
void lupos_wq_get_task_struct(struct task_struct *task);
struct kthread_worker *lupos_wq_pwq_release_worker(void);
bool lupos_wq_topo_initialized(void);
const struct cpumask *lupos_wq_unbound_cpumask(void);
const struct cpumask *lupos_wq_cpu_online_mask(void);
const struct cpumask *lupos_wq_cpu_active_mask(void);
const struct cpumask *lupos_wq_cpu_possible_mask(void);
unsigned int lupos_wq_nr_cpu_ids(void);
unsigned int lupos_wq_smp_processor_id(void);
bool lupos_wq_cpumask_test_cpu(int cpu, const struct cpumask *mask);
bool lupos_wq_cpumask_intersects(const struct cpumask *a, const struct cpumask *b);
unsigned int lupos_wq_cpumask_weight_and(const struct cpumask *a,
				      const struct cpumask *b);
unsigned int lupos_wq_cpumask_any_and_distribute(const struct cpumask *a,
					      const struct cpumask *b);
struct irq_work *lupos_wq_bh_pool_irq_work_at(int cpu, int index);
void lupos_wq_raise_softirq_irqoff(unsigned int nr);
#ifdef CONFIG_SMP
int lupos_wq_read_task_wake_cpu(struct task_struct *task);
void lupos_wq_write_task_wake_cpu(struct task_struct *task, int cpu);
#endif
void lupos_wq_trace_workqueue_activate_work(struct work_struct *work);

bool lupos_wq_online(void);
struct list_head *lupos_wq_work_entry(struct work_struct *work);
struct worker_pool *lupos_wq_bh_pool_at(int cpu, int index);
struct lock_class_key *lupos_wq_schedule_each_cpu_key(void);
struct lock_class_key *lupos_wq_execute_process_context_key(void);
struct lock_class_key *lupos_wq_dead_softirq_work_key(void);

/* Native primitive/address declarations, not generated Rust bindings. */
void lupos_wq_local_irq_disable(void);
void lupos_wq_local_irq_enable(void);
void lupos_wq_local_irq_restore(unsigned long flags);
void lupos_wq_preempt_disable(void);
void lupos_wq_preempt_enable(void);
void lupos_wq_cpu_relax(void);
int lupos_wq_cond_resched(void);
int lupos_wq_raw_smp_processor_id(void);
bool lupos_wq_in_atomic(void);
bool lupos_wq_in_hardirq(void);
bool lupos_wq_in_interrupt(void);
void lupos_wq_rcu_barrier(void);
int lupos_wq_rcu_preempt_depth(void);
void lupos_wq_destroy_work_on_stack(struct work_struct *work);
int lupos_wq_timer_delete_sync(struct timer_list *timer);
void lupos_wq_wait_for_completion(struct completion *done);
bool lupos_wq_try_wait_for_completion(struct completion *done);
void lupos_wq_list_splice_tail_init(struct list_head *list, struct list_head *head);
void lupos_wq_list_add_tail_rcu(struct list_head *entry, struct list_head *head);
void lupos_wq_list_del_rcu(struct list_head *entry);
void lupos_wq_hlist_add_head(struct hlist_node *node, struct hlist_head *head);
void lupos_wq_hash_del(struct hlist_node *node);
void lupos_wq_init_hlist_node(struct hlist_node *node);
void lupos_wq_ida_init(struct ida *ida);
void * lupos_wq_err_ptr(long error);
bool lupos_wq_test_bit(unsigned int bit, const unsigned long *word);
bool lupos_wq_test_and_set_bit(long bit, unsigned long *word);
void lupos_wq_kasan_record_aux_stack(struct work_struct *work);
unsigned long lupos_wq_bh_worker_jiffies(void);
bool lupos_wq_debug_force_rr_cpu(void);
unsigned long lupos_wq_cpu_intensive_thresh_us(void);
u64 lupos_wq_read_task_sum_exec_runtime(struct task_struct *task);
int lupos_wq_pool_nr_idle_plain(struct worker_pool *pool);
int lupos_wq_pool_nr_running_plain(struct worker_pool *pool);
int lupos_wq_this_cpu_rr_read(void);
void lupos_wq_this_cpu_rr_write(int cpu);
struct delayed_work * lupos_wq_to_delayed_work(struct work_struct *work);
struct delayed_work * lupos_wq_delayed_work_from_timer(struct timer_list *timer);
struct rcu_work * lupos_wq_rcu_work_from_rcu(struct rcu_head *rcu);
struct worker_pool * lupos_wq_pool_from_rcu(struct rcu_head *rcu);
struct workqueue_struct * lupos_wq_wq_from_rcu(struct rcu_head *rcu);
struct worker_pool * lupos_wq_pool_from_hash_node(struct hlist_node *node);
struct pool_workqueue * lupos_wq_pwq_from_release_work(struct kthread_work *work);
struct wq_node_nr_active ** lupos_wq_node_nr_active_array(struct workqueue_struct *wq);
struct wq_node_nr_active ** lupos_wq_node_nr_active_slot(struct workqueue_struct *wq, int node);
struct pool_workqueue ** lupos_wq_ctx_pwq_slot(struct apply_wqattrs_ctx *ctx, int cpu);
char * lupos_wq_wq_name(struct workqueue_struct *wq);
struct workqueue_attrs * lupos_wq_update_pwq_attrs_buf(void);
struct workqueue_attrs * lupos_wq_std_unbound_attrs(int highpri);
struct workqueue_attrs * lupos_wq_ordered_attrs(int highpri);
const struct cpumask * lupos_wq_online_cpumask(void);
struct wq_pod_type * lupos_wq_pod_type(int scope);
enum wq_affn_scope lupos_wq_affn_dfl(void);
const struct cpumask * lupos_wq_pod_cpus(const struct wq_pod_type *type, int pod);
void lupos_wq_cpumask_copy(struct cpumask *to, const struct cpumask *from);
bool lupos_wq_cpumask_equal(const struct cpumask *a, const struct cpumask *b);
bool lupos_wq_cpumask_and(struct cpumask *to, const struct cpumask *a, const struct cpumask *b);
bool lupos_wq_cpumask_empty(const struct cpumask *mask);
bool lupos_wq_cpumask_subset(const struct cpumask *a, const struct cpumask *b);
const unsigned long * lupos_wq_cpumask_bits(const struct cpumask *mask);
u32 lupos_wq_cpumask_hash_bytes(void);
unsigned int lupos_wq_cpumask_any_and(const struct cpumask *a, const struct cpumask *b);
unsigned int lupos_wq_cpumask_next_and_wrap(int cpu, const struct cpumask *a, const struct cpumask *b);
const struct cpumask * lupos_wq_cpumask_of_node(int node);
int lupos_wq_cpu_to_node(int cpu);
int lupos_wq_nr_node_ids(void);
bool lupos_wq_node_online(int node);
bool lupos_wq_cpu_online(int cpu);
bool lupos_wq_housekeeping_enabled(enum hk_type type);
bool lupos_wq_housekeeping_test_cpu(int cpu, enum hk_type type);
int lupos_wq_housekeeping_any_cpu(enum hk_type type);
void lupos_wq_init_pool_busy_hash(struct worker_pool *pool);
void lupos_wq_free_attrs_cpumask(struct workqueue_attrs *attrs);
void lupos_wq_free_attrs_pod_cpumask(struct workqueue_attrs *attrs);
void lupos_wq_cpus_read_lock(void);
void lupos_wq_cpus_read_unlock(void);
void lupos_wq_prepare_to_rcuwait(struct rcuwait *wait);
bool lupos_wq_signal_pending_state(unsigned int state, struct task_struct *task);
struct wq_flusher * lupos_wq_read_first_flusher(struct workqueue_struct *wq);
void lupos_wq_write_first_flusher(struct workqueue_struct *wq, struct wq_flusher *flusher);
struct pool_workqueue * lupos_wq_pwq_rcu_access(struct pool_workqueue __rcu *const *slot);
void lupos_wq_pwq_rcu_assign(struct pool_workqueue __rcu **slot, struct pool_workqueue *pwq);
struct pool_workqueue * lupos_wq_cpu_pwq_dereference(struct workqueue_struct *wq, int cpu);
struct worker_pool * lupos_wq_cpu_pool(int cpu, int highpri);
unsigned long lupos_wq_local_irq_save(void);
void lupos_wq_rcu_read_lock(void) __acquires_shared(RCU);
void lupos_wq_rcu_read_unlock(void) __releases_shared(RCU);
void lupos_wq_raw_spin_lock(raw_spinlock_t *lock) __acquires(lock);
void lupos_wq_raw_spin_unlock(raw_spinlock_t *lock) __releases(lock);
void lupos_wq_raw_spin_lock_irq(raw_spinlock_t *lock) __acquires(lock);
void lupos_wq_raw_spin_unlock_irq(raw_spinlock_t *lock) __releases(lock);
void lupos_wq_nna_lock_irq(struct wq_node_nr_active *nna) __acquires(&nna->lock);
void lupos_wq_nna_unlock_irq(struct wq_node_nr_active *nna) __releases(&nna->lock);
#ifdef CONFIG_PREEMPT_RT
void lupos_wq_spin_lock(spinlock_t *lock) __acquires(lock);
void lupos_wq_spin_unlock(spinlock_t *lock) __releases(lock);
#endif
void lupos_wq_set_current_state(unsigned int state);
void lupos_wq_set_state_worker_thread_idle(void);
void lupos_wq_set_state_rescuer_thread_idle(void);
void lupos_wq_set_state_rescuer_thread_running(void);
void lupos_wq_set_state_rescuer_thread_stopping(void);
void lupos_wq_pwq_cpu_time_add_data_race(struct pool_workqueue *pwq, u64 delta);
struct work_struct __percpu *lupos_wq_alloc_percpu_work(void);
void lupos_wq_free_percpu_work(struct work_struct __percpu *works);
struct work_struct *lupos_wq_percpu_work_at(struct work_struct __percpu *works, int cpu);
void lupos_wq_free_cpu_pwq(struct workqueue_struct *wq);
bool lupos_wq_has_cpu_pwq(struct workqueue_struct *wq);
void lupos_wq_clear_cpu_pwq(struct workqueue_struct *wq);
int lupos_wq_first_online_cpu(void);
int lupos_wq_next_online_cpu(int cpu);
unsigned int lupos_wq_first_possible_cpu(void);
unsigned int lupos_wq_next_possible_cpu(unsigned int cpu);
int lupos_wq_first_possible_node(void);
int lupos_wq_next_possible_node(int node);
void lupos_wq_init_completion_c3871(struct completion *done);
void lupos_wq_init_completion_wq_map(struct completion *done, struct workqueue_struct *wq);
typedef void (*lupos_wq_timer_fn_t)(struct timer_list *);
bool lupos_wq_timer_func_eq(lupos_wq_timer_fn_t left, lupos_wq_timer_fn_t right);
void lupos_wq_trace_workqueue_queue_work(int cpu, struct pool_workqueue *pwq,
				         struct work_struct *work);
void lupos_wq_trace_process_one_work_execute_start(struct work_struct *work);
void lupos_wq_trace_process_one_work_execute_end(struct work_struct *work, work_func_t func);

/* Native macro traversal, guard and diagnostic-observation contracts. */
unsigned int lupos_wq_small_cpumask_bits(void);
int lupos_wq_max_numnodes(void);
unsigned int lupos_wq_first_cpu_in_mask(const struct cpumask *mask);
unsigned int lupos_wq_next_cpu_in_mask(unsigned int cpu, const struct cpumask *mask);
unsigned int lupos_wq_cpumask_first(const struct cpumask *mask);
#ifdef CONFIG_WQ_WATCHDOG
void lupos_wq_pool_lock_irq_disable_c7979(struct worker_pool *pool)
	__acquires(&pool->lock);
void lupos_wq_pool_unlock_irq_enable_c7979(struct worker_pool *pool)
	__releases(&pool->lock);
#endif
unsigned long lupos_wq_work_data_bits_plain_read(struct work_struct *work);
unsigned int lupos_wq_pool_flags_read(const struct worker_pool *pool);
unsigned int lupos_wq_wq_flags_read(const struct workqueue_struct *wq);
unsigned int lupos_wq_task_flags_read(struct task_struct *task);
char lupos_wq_worker_desc_first_read(struct worker *worker);
void *lupos_wq_work_func_hint(work_func_t func);
struct hlist_head *lupos_wq_busy_hash_bucket(struct worker_pool *pool, size_t bucket);
struct workqueue_struct *lupos_wq_workqueues_first_rcu(void);
struct workqueue_struct *lupos_wq_workqueues_next_rcu(struct workqueue_struct *wq);
const struct cpumask *lupos_wq_requested_unbound_cpumask(void);
struct cpumask *lupos_wq_isolated_cpumask(void);
unsigned long lupos_wq_read_once_ulong(const unsigned long *value);
void lupos_wq_write_once_ulong(unsigned long *value, unsigned long new_value);
unsigned long lupos_wq_read_ulong(const unsigned long *value);
void lupos_wq_write_ulong(unsigned long *value, unsigned long new_value);
bool lupos_wq_time_after(unsigned long left, unsigned long right);
unsigned int lupos_wq_jiffies_to_msecs(unsigned long delta);
void lupos_wq_printk_deferred_enter(void);
void lupos_wq_printk_deferred_exit(void);
void lupos_wq_touch_nmi_watchdog(void);
int lupos_wq_task_pid_nr(struct task_struct *task);
#ifdef CONFIG_DEBUG_OBJECTS_WORK
bool lupos_wq_work_static_test_bit(struct work_struct *work);
#endif
#ifdef CONFIG_WQ_CPU_INTENSIVE_REPORT
struct wci_ent *lupos_wq_wci_ents(void);
raw_spinlock_t *lupos_wq_wci_lock(void);
int lupos_wq_wci_nr_ents_read(void);
void lupos_wq_wci_nr_ents_write(int nr);
struct hlist_node *lupos_wq_wci_hash_first_rcu(work_func_t func);
struct hlist_node *lupos_wq_hlist_next_rcu(struct hlist_node *node);
struct wci_ent *lupos_wq_wci_from_hash_node(struct hlist_node *node);
void lupos_wq_wci_hash_add_rcu(struct hlist_node *node, work_func_t func);
s64 lupos_wq_atomic64_inc_return_relaxed(atomic64_t *counter);
s64 lupos_wq_atomic64_read(const atomic64_t *counter);
void lupos_wq_atomic64_set(atomic64_t *counter, s64 value);
unsigned int lupos_wq_cpu_intensive_warning_thresh(void);
#endif
#ifdef CONFIG_SYSFS
struct device *lupos_wq_kobj_to_dev(struct kobject *object);
int lupos_wq_saved_max_active_read(const struct workqueue_struct *wq);
bool lupos_wq_affn_strict_read(const struct workqueue_struct *wq);
const char *lupos_wq_affn_name(int scope);
int lupos_wq_cpumask_parse(const char *text, struct cpumask *mask);
struct cpumask *lupos_wq_cpumask_var_ptr(cpumask_var_t *storage);
void lupos_wq_free_cpumask_var_storage(cpumask_var_t *storage);
void lupos_wq_dev_set_uevent_suppress(struct device *dev, bool suppress);
#endif
#ifdef CONFIG_WQ_WATCHDOG
unsigned long *lupos_wq_watchdog_thresh(void);
unsigned long *lupos_wq_watchdog_touched(void);
unsigned long *lupos_wq_watchdog_touched_cpu(int cpu);
struct timer_list *lupos_wq_watchdog_timer(void);
unsigned int lupos_wq_panic_on_stall(void);
unsigned int lupos_wq_panic_on_stall_time(void);
unsigned int *lupos_wq_stall_count(void);
bool lupos_wq_task_is_running(struct task_struct *task);
void lupos_wq_trigger_single_cpu_backtrace(int cpu);
bool lupos_wq_kvm_check_and_clear_guest_paused(void);
int lupos_wq_kstrtoul(const char *text, unsigned int base, unsigned long *result);
#endif

#endif /* LUPOS_WORKQUEUE_BINDINGS_H */
