/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_SUB_RESCUE_BINDINGS_H
#define LUPOS_SCHED_EXT_SUB_RESCUE_BINDINGS_H
#define LUPOS_SCX_SUB_TICK_NSEC TICK_NSEC
#define LUPOS_SCX_SUB_NSEC_PER_USEC NSEC_PER_USEC
#define LUPOS_SCX_SUB_CAPACITY_SHIFT SCHED_CAPACITY_SHIFT
#define LUPOS_SCX_SUB_CAPACITY_SCALE SCHED_CAPACITY_SCALE
void lupos_scx_sub_assert_rescue_charge_rq(struct rq *rq);
void lupos_scx_sub_assert_rescue_end_rq(struct rq *rq);
void lupos_scx_sub_assert_rescue_keep_rq(struct rq *rq);
void lupos_scx_sub_assert_rescue_accrue_rq(struct rq *rq);
void lupos_scx_sub_assert_rescue_admit_rq(struct rq *rq);
void lupos_scx_sub_assert_rescue_overload_rq(struct rq *rq);
void lupos_scx_sub_assert_rescue_flush_rq(struct rq *rq);
void lupos_scx_sub_assert_reenq_reject_rq(struct rq *rq);
s32 lupos_scx_sub_cpu(struct rq *rq);
struct scx_sched *lupos_scx_sub_task_sched(struct task_struct *p);
struct scx_sched_pcpu *lupos_scx_sub_pcpu(struct scx_sched *sch, s32 cpu);
bool lupos_scx_sub_bypassing(struct scx_sched *sch, s32 cpu);
bool lupos_scx_sub_list_empty(const struct list_head *list);
u64 lupos_scx_sub_jiffies64(void);
unsigned long lupos_scx_sub_jiffies(void);
unsigned long lupos_scx_sub_nsecs_jiffies(u64 ns);
unsigned long lupos_scx_sub_msecs_jiffies(u32 ms);
bool lupos_scx_sub_time_before(unsigned long a, unsigned long b);
bool lupos_scx_sub_time_before64(u64 a, u64 b);
bool lupos_scx_sub_timer_pending(struct timer_list *timer);
void lupos_scx_sub_add_timer(struct timer_list *timer, s32 cpu);
void lupos_scx_sub_timer_delete(struct timer_list *timer);
void lupos_scx_sub_timer_setup(struct rq *rq);
void lupos_scx_sub_rescue_timer_locked(struct rq *rq);
void lupos_scx_sub_set_slice(struct task_struct *p, u64 slice);
void lupos_scx_sub_warn_rescue_current(struct rq *rq);
struct task_struct *lupos_scx_sub_dsq_first(const struct scx_dispatch_q *dsq);
struct task_struct *lupos_scx_sub_dsq_next(const struct scx_dispatch_q *dsq, struct task_struct *p);
struct scx_sched *lupos_scx_sub_all_next(struct scx_sched *pos);
s32 lupos_scx_sub_exit_kind(struct scx_sched *sch);
void lupos_scx_sub_exit_rescue(struct scx_sched *victim, s32 cpu, u64 avg, struct task_struct *p, unsigned long duration);
bool lupos_scx_sub_ext_above_current(struct rq *rq);
void lupos_scx_sub_resched(struct rq *rq);
bool lupos_scx_sub_cpu_active(s32 cpu);
void lupos_scx_sub_dump_rescue(struct seq_buf *s, struct rq *rq);
void lupos_scx_sub_warn_timeout(struct scx_sched *sch, unsigned long threshold);
void lupos_scx_sub_warn_funding(struct scx_sched *sch, s64 period, unsigned long threshold);
struct sched_ext_ops *lupos_scx_sub_ops(struct scx_sched *sch);
void lupos_scx_sub_bug_init_dsq(s32 ret);
bool lupos_scx_sub_has_subs(void);
s32 lupos_scx_sub_cpu_cid(s32 cpu);
u64 lupos_scx_sub_caps_for_enq(u64 flags);
u64 lupos_scx_sub_caps_for_preempt(struct scx_sched *sch, struct rq *rq, u64 flags);
u64 lupos_scx_sub_missing_caps(struct scx_sched *sch, s32 cpu, u64 caps);
bool lupos_scx_sub_rq_online(struct rq *rq);
bool lupos_scx_sub_migration_disabled(struct task_struct *p);
void lupos_scx_sub_event_forced(struct scx_sched *sch);
void lupos_scx_sub_event_rescue(struct scx_sched *sch);
struct task_struct *lupos_scx_sub_rescuee(struct rq *rq);
u64 lupos_scx_sub_caps_for_task(struct task_struct *p);
void lupos_scx_sub_with_reject_list(struct rq *rq);
void lupos_scx_sub_reject_with_list(struct rq *rq, struct list_head *tasks);
bool lupos_scx_sub_warn_migration_pending(struct task_struct *p);
bool lupos_scx_sub_warn_reenq_flags(struct task_struct *p);
void lupos_scx_sub_task_list_add(struct task_struct *p, struct list_head *head);
void lupos_scx_sub_task_list_del(struct task_struct *p);
struct task_struct *lupos_scx_sub_task_list_first(struct list_head *head);
struct task_struct *lupos_scx_sub_task_list_next(struct list_head *head, struct task_struct *p);
#endif
