/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_IRQ_WORK_BINDINGS_H
#define LUPOS_IRQ_WORK_BINDINGS_H

/* Configured native headers are the only type/layout/constant authority. */
#include <linux/init.h>
#include <linux/irq_work.h>
#include <linux/llist.h>
#include <linux/smpboot.h>

struct task_struct *lupos_irq_work_this_task(void);
struct llist_head *lupos_irq_work_this_lazy(void);
struct llist_head *lupos_irq_work_this_raised(void);
void lupos_irq_work_wake_process(struct task_struct *task);

int lupos_irq_work_atomic_fetch_or(int flags, struct irq_work *work);
int lupos_irq_work_atomic_read(struct irq_work *work);
void lupos_irq_work_atomic_set(struct irq_work *work, int flags);
int lupos_irq_work_atomic_cmpxchg(struct irq_work *work, int old, int new);
void lupos_irq_work_mb(void);
bool lupos_irq_work_is_busy(struct irq_work *work);
bool lupos_irq_work_is_hard(struct irq_work *work);

bool lupos_irq_work_trace_enabled(void);
bool lupos_irq_work_has_interrupt(void);
void lupos_irq_work_trace_send(struct irq_work *work);
bool lupos_irq_work_tick_stopped(void);
void lupos_irq_work_preempt_disable(void);
void lupos_irq_work_preempt_enable(void);

bool lupos_irq_work_list_empty(struct llist_head *head);
bool lupos_irq_work_list_add(struct irq_work *work, struct llist_head *head);
struct llist_node *lupos_irq_work_list_del_all(struct llist_head *head);
struct irq_work *lupos_irq_work_node_work(struct llist_node *node);
struct llist_node *lupos_irq_work_node_next(struct llist_node *node);

#ifdef CONFIG_SMP
struct llist_head *lupos_irq_work_cpu_lazy(int cpu);
struct irq_work *lupos_irq_work_cpu_wakeup(int cpu);
void lupos_irq_work_queue_remote(int cpu, struct irq_work *work);
unsigned int lupos_irq_work_processor_id(void);
void lupos_irq_work_warn_queue_cpu_offline(int cpu);
void lupos_irq_work_warn_remote_nmi(void);
void lupos_irq_work_record_aux_stack(struct irq_work *work);
/* Rust callback stored by native IRQ_WORK_INIT_HARD. */
void lupos_irq_work_wake(struct irq_work *entry);
#endif

void lupos_irq_work_warn_needs_cpu_offline(void);
void lupos_irq_work_lockdep_enter(int flags);
void lupos_irq_work_invoke(struct irq_work *work);
void lupos_irq_work_lockdep_exit(int flags);
void lupos_irq_work_bug_run_context(void);
void lupos_irq_work_assert_irqs_enabled(void);
void lupos_irq_work_might_sleep(void);
void lupos_irq_work_wait_prepare(struct irq_work *work);
void lupos_irq_work_wait_set_state(void);
bool lupos_irq_work_wait_signal_pending(void);
void lupos_irq_work_schedule(void);
void lupos_irq_work_wait_finish(struct irq_work *work);
void lupos_irq_work_wait_wake(struct irq_work *work);
void lupos_irq_work_synchronize_rcu(void);
void lupos_irq_work_relax(void);
void lupos_irq_work_rcu_read_lock(void);
void lupos_irq_work_rcu_read_unlock(void);
void lupos_irq_work_setup_current(void);
void __init lupos_irq_work_bug_register_threads(void);

/* Native smp_hotplug_thread/early_initcall entries implemented by Rust. */
int lupos_irq_workd_should_run(unsigned int cpu);
void lupos_irq_work_run_workd(unsigned int cpu);
void lupos_irq_workd_setup(unsigned int cpu);
int __init lupos_irq_work_init_threads(void);

#endif /* LUPOS_IRQ_WORK_BINDINGS_H */
