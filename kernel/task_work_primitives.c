// SPDX-License-Identifier: GPL-2.0
/*
 * Task-work native layout/storage and primitive boundary, explicitly
 * UNQUALIFIED. This is not translated algorithm coverage. The existing Rust
 * owner retains add/cancel/run control flow; task_work.c is not a fallback.
 * The two callback-bearing public entry points only forward through native
 * storage/conversion, retaining their exact C outer KCFI signatures.
 * Original compilation/instrumentation origin: kernel/task_work.o.
 */
#include "task_work_bindings.h"
#include <linux/percpu.h>
#include <linux/resume_user_mode.h>
#include <linux/sched/signal.h>
#include <linux/spinlock.h>

#error "SOURCE ONLY HOLD: task work native primitives are not admitted"

/* Native lifetime, alignment and zero initialization, including next == NULL. */
static struct callback_head work_exited;

/* Synchronous stack context preserves a NULL matcher on no-call paths. */
struct callback_head *
task_work_cancel_match(struct task_struct *task,
		       bool (*match)(struct callback_head *, void *), void *data)
{
	const struct lupos_task_work_match_ctx context = { .match = match };

	return lupos_task_work_cancel_match(task, &context, data);
}

struct callback_head *
task_work_cancel_func(struct task_struct *task, task_work_func_t func)
{
	/* Exactly the original native function-pointer/void-pointer convention. */
	return lupos_task_work_cancel_func(task, func);
}

/* Callback algorithms stay in Rust; C owns their typed pointer storage. */
static const struct lupos_task_work_match_ctx func_match_context = {
	.match = lupos_task_work_func_match,
};
static const struct lupos_task_work_match_ctx match_context = {
	.match = lupos_task_work_match,
};

const struct lupos_task_work_match_ctx *lupos_task_work_func_match_context(void)
{
	return &func_match_context;
}

const struct lupos_task_work_match_ctx *lupos_task_work_match_context(void)
{
	return &match_context;
}

#ifdef CONFIG_IRQ_WORK
/* One hard-IRQ entry per CPU, never a shared or zero-sized placeholder. */
static DEFINE_PER_CPU(struct irq_work, irq_work_NMI_resume) =
	IRQ_WORK_INIT_HARD(lupos_task_work_set_notify_irq);

void lupos_task_work_notify_nmi_current(void)
{
	set_tsk_thread_flag(current, TIF_NOTIFY_RESUME);
	irq_work_queue(this_cpu_ptr(&irq_work_NMI_resume));
}
#endif

struct task_struct *lupos_task_work_current(void)
{
	return current;
}

struct callback_head **lupos_task_work_head_slot(struct task_struct *task)
{
	return &task->task_works;
}

struct callback_head **lupos_task_work_next_slot(struct callback_head *work)
{
	return &work->next;
}

struct callback_head *lupos_task_work_read_once(struct callback_head **slot)
{
	return READ_ONCE(*slot);
}

bool lupos_task_work_try_cmpxchg(struct callback_head **slot,
			       struct callback_head **expected,
			       struct callback_head *replacement)
{
	/* The native primitive writes the observed pointer to *expected on failure. */
	return try_cmpxchg(slot, expected, replacement);
}

struct callback_head *lupos_task_work_next(struct callback_head *work)
{
	/* Intentionally the original plain load, not a stronger atomic access. */
	return work->next;
}

void lupos_task_work_set_next(struct callback_head *work,
			    struct callback_head *next)
{
	/* Before publication; retain the original plain assignment. */
	work->next = next;
}

struct callback_head *lupos_task_work_exited(void)
{
	return &work_exited;
}

bool lupos_task_work_is_exited(struct callback_head *head)
{
	return unlikely(head == &work_exited);
}

bool lupos_task_work_task_exiting(struct task_struct *task)
{
	return task->flags & PF_EXITING;
}

bool lupos_task_work_likely_not_pending(struct task_struct *task)
{
	return likely(!task_work_pending(task));
}

unsigned long lupos_task_work_lock_irqsave(struct task_struct *task)
{
	unsigned long flags;

	raw_spin_lock_irqsave(&task->pi_lock, flags);
	return flags;
}

void lupos_task_work_unlock_irqrestore(struct task_struct *task,
				       unsigned long flags)
{
	raw_spin_unlock_irqrestore(&task->pi_lock, flags);
}

void lupos_task_work_lock_irq(struct task_struct *task)
{
	raw_spin_lock_irq(&task->pi_lock);
}

void lupos_task_work_unlock_irq(struct task_struct *task)
{
	raw_spin_unlock_irq(&task->pi_lock);
}

void lupos_task_work_cond_resched(void)
{
	cond_resched();
}

void lupos_task_work_record_aux_stack(struct callback_head *work)
{
	kasan_record_aux_stack(work);
}

bool lupos_task_work_warn_not_current(struct task_struct *task)
{
	/* Distinct native WARN_ON_ONCE site from the invalid-mode warning. */
	return WARN_ON_ONCE(task != current);
}

void lupos_task_work_warn_unknown_notify(void)
{
	WARN_ON_ONCE(1);
}

void lupos_task_work_notify_resume(struct task_struct *task)
{
	set_notify_resume(task);
}

void lupos_task_work_notify_signal(struct task_struct *task)
{
	set_notify_signal(task);
}

void lupos_task_work_notify_signal_no_ipi(struct task_struct *task)
{
	__set_notify_signal(task);
}

bool lupos_task_work_call_match(const struct lupos_task_work_match_ctx *match,
			       struct callback_head *work, void *data)
{
	return match->match(work, data);
}

bool lupos_task_work_func_matches(struct callback_head *cb, void *data)
{
	return cb->func == data;
}

void lupos_task_work_invoke(struct callback_head *work)
{
	/* Do not silently skip a NULL callback: the native caller must provide one. */
	work->func(work);
}
