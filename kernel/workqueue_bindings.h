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

#endif /* LUPOS_WORKQUEUE_BINDINGS_H */
