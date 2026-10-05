/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_RT_CLASS_CALLBACKS_H
#define LUPOS_RT_CLASS_CALLBACKS_H
/* Exact native callback types from rt.c and struct sched_class. */
void enqueue_task_rt(struct rq *rq, struct task_struct *p, int flags);
bool dequeue_task_rt(struct rq *rq, struct task_struct *p, int flags);
void yield_task_rt(struct rq *rq);
void wakeup_preempt_rt(struct rq *rq, struct task_struct *p, int flags);
struct task_struct *pick_task_rt(struct rq *rq, struct rq_flags *rf);
void put_prev_task_rt(struct rq *rq, struct task_struct *p,
		      struct task_struct *next);
void set_next_task_rt(struct rq *rq, struct task_struct *p, bool first);
int balance_rt(struct rq *rq, struct rq_flags *rf);
int select_task_rq_rt(struct task_struct *p, int cpu, int flags);
void rq_online_rt(struct rq *rq);
void rq_offline_rt(struct rq *rq);
void task_woken_rt(struct rq *rq, struct task_struct *p);
void switched_from_rt(struct rq *rq, struct task_struct *p);
struct rq *find_lock_lowest_rq(struct task_struct *task, struct rq *rq);
void task_tick_rt(struct rq *rq, struct task_struct *p, int queued);
unsigned int get_rr_interval_rt(struct rq *rq, struct task_struct *task);
void switched_to_rt(struct rq *rq, struct task_struct *p);
void prio_changed_rt(struct rq *rq, struct task_struct *p, u64 oldprio);
void rust_rt_update_curr_rt(struct rq *rq);
#ifdef CONFIG_SCHED_CORE
int task_is_throttled_rt(struct task_struct *p, int cpu);
#endif
#ifdef CONFIG_SYSCTL
int sched_rt_handler(const struct ctl_table *table, int write, void *buffer,
		     size_t *lenp, loff_t *ppos);
int sched_rr_handler(const struct ctl_table *table, int write, void *buffer,
		     size_t *lenp, loff_t *ppos);
int __init sched_rt_sysctl_init(void);
#endif
#endif
