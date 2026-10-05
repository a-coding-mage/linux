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

#endif /* LUPOS_WORKQUEUE_BINDINGS_H */
