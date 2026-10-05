/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_TASK_WORK_BINDINGS_H
#define LUPOS_TASK_WORK_BINDINGS_H

/* Configured native headers alone own all types, layouts and constants. */
#include <linux/errno.h>
#include <linux/irq_work.h>
#include <linux/task_work.h>

enum {
	LUPOS_TASK_WORK_EINVAL = EINVAL,
	LUPOS_TASK_WORK_ESRCH = ESRCH,
};

typedef bool (*lupos_task_work_match_t)(struct callback_head *, void *);

/* Native nullable callback storage; Rust never receives an Option<fn> argument. */
struct lupos_task_work_match_ctx {
	lupos_task_work_match_t match;
};

/* Sole Rust algorithm bodies behind exact native public callback signatures. */
struct callback_head *
lupos_task_work_cancel_match(struct task_struct *task,
			   const struct lupos_task_work_match_ctx *match,
			   void *data);
struct callback_head *lupos_task_work_cancel_func(struct task_struct *task,
					       void *func);

/* The internal matcher algorithms remain in the existing Rust owner. */
bool lupos_task_work_func_match(struct callback_head *cb, void *data);
bool lupos_task_work_match(struct callback_head *cb, void *data);
const struct lupos_task_work_match_ctx *lupos_task_work_func_match_context(void);
const struct lupos_task_work_match_ctx *lupos_task_work_match_context(void);

struct task_struct *lupos_task_work_current(void);
struct callback_head **lupos_task_work_head_slot(struct task_struct *task);
struct callback_head **lupos_task_work_next_slot(struct callback_head *work);
struct callback_head *lupos_task_work_read_once(struct callback_head **slot);
bool lupos_task_work_try_cmpxchg(struct callback_head **slot,
			       struct callback_head **expected,
			       struct callback_head *replacement);
struct callback_head *lupos_task_work_next(struct callback_head *work);
void lupos_task_work_set_next(struct callback_head *work,
			    struct callback_head *next);
struct callback_head *lupos_task_work_exited(void);
bool lupos_task_work_is_exited(struct callback_head *head);
bool lupos_task_work_task_exiting(struct task_struct *task);
bool lupos_task_work_likely_not_pending(struct task_struct *task);

unsigned long lupos_task_work_lock_irqsave(struct task_struct *task);
void lupos_task_work_unlock_irqrestore(struct task_struct *task,
				       unsigned long flags);
void lupos_task_work_lock_irq(struct task_struct *task);
void lupos_task_work_unlock_irq(struct task_struct *task);
void lupos_task_work_cond_resched(void);

void lupos_task_work_record_aux_stack(struct callback_head *work);
bool lupos_task_work_warn_not_current(struct task_struct *task);
void lupos_task_work_warn_unknown_notify(void);
void lupos_task_work_notify_resume(struct task_struct *task);
void lupos_task_work_notify_signal(struct task_struct *task);
void lupos_task_work_notify_signal_no_ipi(struct task_struct *task);

bool lupos_task_work_call_match(const struct lupos_task_work_match_ctx *match,
			       struct callback_head *work, void *data);
bool lupos_task_work_func_matches(struct callback_head *cb, void *data);
void lupos_task_work_invoke(struct callback_head *work);

#ifdef CONFIG_IRQ_WORK
void lupos_task_work_notify_nmi_current(void);
/* Rust no-op callback; native storage owns its configured irq_work object. */
void lupos_task_work_set_notify_irq(struct irq_work *entry);
#endif

#endif /* LUPOS_TASK_WORK_BINDINGS_H */
