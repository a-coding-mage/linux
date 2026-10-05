// SPDX-License-Identifier: GPL-2.0
/* Native header primitives, diagnostics and const class metadata only. */
#include "sched_stop_task_bindings.h"

#ifdef CONFIG_RUST_SCHED_STOP_TASK
#error "SOURCE ONLY HOLD: scheduler stop-task owner is not admitted"
#endif

#include "sched_stop_task_registration.inc"

int lupos_stop_task_cpu(struct task_struct *p)
{
	return task_cpu(p);
}

bool lupos_stop_task_runnable(struct rq *rq)
{
	return sched_stop_runnable(rq);
}

u64 lupos_stop_task_rq_clock_task(struct rq *rq)
{
	return rq_clock_task(rq);
}

void lupos_stop_task_add_nr_running(struct rq *rq, unsigned int count)
{
	add_nr_running(rq, count);
}

void lupos_stop_task_sub_nr_running(struct rq *rq, unsigned int count)
{
	sub_nr_running(rq, count);
}

/* Preserve three distinct original diagnostic sites, including their native
 * instrumentation. They are real leaves, not qualified Rust substitutes. */
void lupos_stop_task_bug_yield(void)
{
	BUG();
}

void lupos_stop_task_bug_switching(void)
{
	BUG();
}

void lupos_stop_task_bug_prio(void)
{
	BUG();
}
