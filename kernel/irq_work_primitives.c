// SPDX-License-Identifier: GPL-2.0-only
/*
 * IRQ-work native layout/storage/primitive boundary, explicitly UNQUALIFIED.
 * Queue routing, claim decisions, draining and waiting remain in irq_work.rs.
 * This is not translated algorithm coverage or an irq_work.c fallback.
 * Original compilation/instrumentation origin: kernel/irq_work.o.
 */
#include "irq_work_bindings.h"
#include <linux/bug.h>
#include <linux/cpu.h>
#include <linux/export.h>
#include <linux/hardirq.h>
#include <linux/irqflags.h>
#include <linux/kasan.h>
#include <linux/kernel.h>
#include <linux/percpu.h>
#include <linux/sched.h>
#include <linux/smp.h>
#include <linux/tick.h>
#include <asm/processor.h>
#include <trace/events/ipi.h>

#error "SOURCE ONLY HOLD: IRQ-work native primitives are not admitted"

/* Real native per-CPU storage, alignment and static zero initialization. */
static DEFINE_PER_CPU(struct llist_head, raised_list);
static DEFINE_PER_CPU(struct llist_head, lazy_list);
static DEFINE_PER_CPU(struct task_struct *, irq_workd);

#ifdef CONFIG_SMP
static DEFINE_PER_CPU(struct irq_work, irq_work_wakeup) =
	IRQ_WORK_INIT_HARD(lupos_irq_work_wake);
#endif

/* Optional descriptor fields retain native static zero initialization. */
static struct smp_hotplug_thread irqwork_threads = {
	.store			= &irq_workd,
	.setup			= lupos_irq_workd_setup,
	.thread_should_run	= lupos_irq_workd_should_run,
	.thread_fn		= lupos_irq_work_run_workd,
	.thread_comm		= "irq_work/%u",
};

/* Keep the original replaceable fallback, not a Rust-local permanent no-op. */
void __weak arch_irq_work_raise(void)
{
	/* Lame architectures will get the timer tick callback. */
}

struct task_struct *lupos_irq_work_this_task(void)
{
	return __this_cpu_read(irq_workd);
}

struct llist_head *lupos_irq_work_this_lazy(void)
{
	return this_cpu_ptr(&lazy_list);
}

struct llist_head *lupos_irq_work_this_raised(void)
{
	return this_cpu_ptr(&raised_list);
}

void lupos_irq_work_wake_process(struct task_struct *task)
{
	wake_up_process(task);
}

int lupos_irq_work_atomic_fetch_or(int flags, struct irq_work *work)
{
	return atomic_fetch_or(flags, &work->node.a_flags);
}

int lupos_irq_work_atomic_read(struct irq_work *work)
{
	return atomic_read(&work->node.a_flags);
}

void lupos_irq_work_atomic_set(struct irq_work *work, int flags)
{
	atomic_set(&work->node.a_flags, flags);
}

int lupos_irq_work_atomic_cmpxchg(struct irq_work *work, int old, int new)
{
	return atomic_cmpxchg(&work->node.a_flags, old, new);
}

void lupos_irq_work_mb(void)
{
	smp_mb();
}

bool lupos_irq_work_is_busy(struct irq_work *work)
{
	return irq_work_is_busy(work);
}

bool lupos_irq_work_is_hard(struct irq_work *work)
{
	return irq_work_is_hard(work);
}

bool lupos_irq_work_trace_enabled(void)
{
	return trace_ipi_send_cpu_enabled();
}

bool lupos_irq_work_has_interrupt(void)
{
	return arch_irq_work_has_interrupt();
}

void lupos_irq_work_trace_send(struct irq_work *work)
{
	trace_call__ipi_send_cpu(smp_processor_id(), _RET_IP_, work->func);
}

bool lupos_irq_work_tick_stopped(void)
{
	return tick_nohz_tick_stopped();
}

void lupos_irq_work_preempt_disable(void)
{
	preempt_disable();
}

void lupos_irq_work_preempt_enable(void)
{
	preempt_enable();
}

bool lupos_irq_work_list_empty(struct llist_head *head)
{
	return llist_empty(head);
}

bool lupos_irq_work_list_add(struct irq_work *work, struct llist_head *head)
{
	return llist_add(&work->node.llist, head);
}

struct llist_node *lupos_irq_work_list_del_all(struct llist_head *head)
{
	return llist_del_all(head);
}

struct irq_work *lupos_irq_work_node_work(struct llist_node *node)
{
	return llist_entry(node, struct irq_work, node.llist);
}

struct llist_node *lupos_irq_work_node_next(struct llist_node *node)
{
	/* Original detached traversal has a plain next load, not llist_next(). */
	return node->next;
}

#ifdef CONFIG_SMP
struct llist_head *lupos_irq_work_cpu_lazy(int cpu)
{
	return &per_cpu(lazy_list, cpu);
}

struct irq_work *lupos_irq_work_cpu_wakeup(int cpu)
{
	return &per_cpu(irq_work_wakeup, cpu);
}

void lupos_irq_work_queue_remote(int cpu, struct irq_work *work)
{
	__smp_call_single_queue(cpu, &work->node.llist);
}

unsigned int lupos_irq_work_processor_id(void)
{
	return smp_processor_id();
}

void lupos_irq_work_warn_queue_cpu_offline(int cpu)
{
	WARN_ON_ONCE(cpu_is_offline(cpu));
}

void lupos_irq_work_warn_remote_nmi(void)
{
	WARN_ON_ONCE(in_nmi());
}

void lupos_irq_work_record_aux_stack(struct irq_work *work)
{
	kasan_record_aux_stack(work);
}
#endif /* CONFIG_SMP */

void lupos_irq_work_warn_needs_cpu_offline(void)
{
	/* A separate warning site; do not share its once-state with queue_on. */
	WARN_ON_ONCE(cpu_is_offline(smp_processor_id()));
}

void lupos_irq_work_lockdep_enter(int flags)
{
	lockdep_irq_work_enter(flags);
}

void lupos_irq_work_invoke(struct irq_work *work)
{
	/* Preserve the native callback type and unconditional call contract. */
	work->func(work);
}

void lupos_irq_work_lockdep_exit(int flags)
{
	lockdep_irq_work_exit(flags);
}

void lupos_irq_work_bug_run_context(void)
{
	BUG_ON(!irqs_disabled() && !IS_ENABLED(CONFIG_PREEMPT_RT));
}

void lupos_irq_work_assert_irqs_enabled(void)
{
	lockdep_assert_irqs_enabled();
}

void lupos_irq_work_might_sleep(void)
{
	might_sleep();
}

void lupos_irq_work_wait_prepare(struct irq_work *work)
{
	prepare_to_rcuwait(&work->irqwait);
}

void lupos_irq_work_wait_set_state(void)
{
	/* Includes the macro's barrier (A); never __set_current_state(). */
	set_current_state(TASK_UNINTERRUPTIBLE);
}

bool lupos_irq_work_wait_signal_pending(void)
{
	return signal_pending_state(TASK_UNINTERRUPTIBLE, current);
}

void lupos_irq_work_schedule(void)
{
	schedule();
}

void lupos_irq_work_wait_finish(struct irq_work *work)
{
	finish_rcuwait(&work->irqwait);
}

void lupos_irq_work_wait_wake(struct irq_work *work)
{
	rcuwait_wake_up(&work->irqwait);
}

void lupos_irq_work_synchronize_rcu(void)
{
	synchronize_rcu();
}

void lupos_irq_work_relax(void)
{
	cpu_relax();
}

void lupos_irq_work_rcu_read_lock(void)
{
	rcu_read_lock();
}

void lupos_irq_work_rcu_read_unlock(void)
{
	rcu_read_unlock();
}

void lupos_irq_work_setup_current(void)
{
	sched_set_fifo_low(current);
}

void __init lupos_irq_work_bug_register_threads(void)
{
	/* Keep registration inside BUG_ON's native operand evaluation. */
	BUG_ON(smpboot_register_percpu_thread(&irqwork_threads));
}

/* Native __init entry attributes and initcall type; all policy stays in Rust. */
static int __init irq_work_init_threads(void)
{
	return lupos_irq_work_init_threads();
}

/* Native export/initcall metadata; these bodies remain owned by Rust. */
EXPORT_SYMBOL_GPL(irq_work_queue);
EXPORT_SYMBOL_GPL(irq_work_run);
EXPORT_SYMBOL_GPL(irq_work_sync);
early_initcall(irq_work_init_threads);
