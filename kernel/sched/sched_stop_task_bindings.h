/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_STOP_TASK_BINDINGS_H
#define LUPOS_SCHED_STOP_TASK_BINDINGS_H

/* The configured scheduler header owns the layout and callback types. */
#include "sched.h"

int lupos_stop_task_cpu(struct task_struct *p);
bool lupos_stop_task_runnable(struct rq *rq);
u64 lupos_stop_task_rq_clock_task(struct rq *rq);
void lupos_stop_task_add_nr_running(struct rq *rq, unsigned int count);
void lupos_stop_task_sub_nr_running(struct rq *rq, unsigned int count);
void lupos_stop_task_bug_yield(void);
void lupos_stop_task_bug_switching(void);
void lupos_stop_task_bug_prio(void);

/* Rust callback symbols, preserving the exact native sched_class signatures. */
int lupos_stop_task_select_task_rq(struct task_struct *p, int cpu, int flags);
int lupos_stop_task_balance(struct rq *rq, struct rq_flags *rf);
void lupos_stop_task_wakeup_preempt(struct rq *rq, struct task_struct *p,
				  int flags);
void lupos_stop_task_set_next_task(struct rq *rq, struct task_struct *stop,
				 bool first);
struct task_struct *lupos_stop_task_pick_task(struct rq *rq, struct rq_flags *rf);
void lupos_stop_task_enqueue_task(struct rq *rq, struct task_struct *p, int flags);
bool lupos_stop_task_dequeue_task(struct rq *rq, struct task_struct *p, int flags);
void lupos_stop_task_yield_task(struct rq *rq);
void lupos_stop_task_put_prev_task(struct rq *rq, struct task_struct *prev,
				 struct task_struct *next);
void lupos_stop_task_task_tick(struct rq *rq, struct task_struct *curr, int queued);
void lupos_stop_task_switching_to(struct rq *rq, struct task_struct *p);
void lupos_stop_task_prio_changed(struct rq *rq, struct task_struct *p, u64 oldprio);
void lupos_stop_task_update_curr(struct rq *rq);

#endif /* LUPOS_SCHED_STOP_TASK_BINDINGS_H */
