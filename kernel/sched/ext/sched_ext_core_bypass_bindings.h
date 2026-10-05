/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_BYPASS_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_BYPASS_BINDINGS_H
/* F10 source-only native interface. Include in the single common native type
 * universe/envelope; no duplicate layout or independent native object. */
#include "internal.h"

u32 lupos_scx_core_bypass_cpu_cursor_frame(struct scx_sched *sch,
	struct rq *donor_rq, struct scx_dispatch_q *donor_dsq,
	struct cpumask *donee_mask, struct cpumask *resched_mask,
	u32 nr_donor_target, u32 nr_donee_target);
u32 lupos_scx_core_bypass_cpu_cursor_body(struct scx_sched *sch,
	struct rq *donor_rq, struct scx_dispatch_q *donor_dsq,
	struct scx_dsq_list_node *cursor, struct cpumask *donee_mask,
	struct cpumask *resched_mask, u32 nr_donor_target, u32 nr_donee_target);
void lupos_scx_core_bypass_lb_timer_body(struct timer_list *timer);

struct rq *lupos_scx_core_bypass_cpu_rq(int cpu);
u32 lupos_scx_core_bypass_nr_read_once(struct scx_dispatch_q *dsq);
u32 lupos_scx_core_bypass_delta_threshold(u32 min_delta_us);
u64 lupos_scx_core_bypass_ktime_get_ns(void);
void lupos_scx_core_bypass_rq_lock_irq(struct rq *rq);
void lupos_scx_core_bypass_rq_unlock_irq(struct rq *rq);
void lupos_scx_core_bypass_raw_lock(raw_spinlock_t *lock);
void lupos_scx_core_bypass_raw_unlock(raw_spinlock_t *lock);
unsigned long lupos_scx_core_bypass_lock_irqsave(raw_spinlock_t *lock);
void lupos_scx_core_bypass_unlock_irqrestore(raw_spinlock_t *lock,
	unsigned long flags);
void lupos_scx_core_bypass_list_add(struct list_head *node, struct list_head *head);
void lupos_scx_core_bypass_list_move_tail(struct list_head *node, struct list_head *head);
void lupos_scx_core_bypass_list_del_init(struct list_head *node);
struct rq *lupos_scx_core_bypass_task_rq(struct task_struct *p);
bool lupos_scx_core_bypass_mask_empty(const struct cpumask *mask);
int lupos_scx_core_bypass_any_allowed(const struct cpumask *mask,
	struct task_struct *p);
void lupos_scx_core_bypass_mask_set_cpu(int cpu, struct cpumask *mask);
void lupos_scx_core_bypass_mask_clear_cpu(int cpu, struct cpumask *mask);
void lupos_scx_core_bypass_mask_clear(struct cpumask *mask);
bool lupos_scx_core_bypass_mask_test_cpu(int cpu, const struct cpumask *mask);
const struct cpumask *lupos_scx_core_bypass_node_mask(int node);
struct cpumask *lupos_scx_core_bypass_donee_mask(struct scx_sched *sch);
struct cpumask *lupos_scx_core_bypass_resched_mask(struct scx_sched *sch);
int lupos_scx_core_bypass_next_online_node_cpu(int cpu,
	const struct cpumask *node_mask);
int lupos_scx_core_bypass_next_mask_cpu(int cpu, const struct cpumask *mask);
unsigned int lupos_scx_core_bypass_mask_cpu_limit(void);
int lupos_scx_core_bypass_next_possible_cpu(int cpu);
unsigned int lupos_scx_core_bypass_possible_cpu_limit(void);
int lupos_scx_core_bypass_next_cpu_node(int node);
unsigned int lupos_scx_core_bypass_cpu_node_limit(void);
void lupos_scx_core_bypass_trace_lb(int node, u32 nr_cpus, u32 nr_tasks,
	u32 nr_balanced, u32 before_min, u32 before_max, u32 after_min, u32 after_max);
struct scx_sched *lupos_scx_core_bypass_timer_sched(struct timer_list *timer);
bool lupos_scx_core_bypass_dsp_enabled(struct scx_sched *sch);
void lupos_scx_core_bypass_mod_timer(struct timer_list *timer, u32 intv_us);
bool lupos_scx_core_bypass_timer_pending(struct timer_list *timer);
void lupos_scx_core_bypass_assert_inc_lock(void);
void lupos_scx_core_bypass_assert_dec_lock(void);
void lupos_scx_core_bypass_warn_inc_depth(struct scx_sched *sch);
void lupos_scx_core_bypass_warn_dec_depth(struct scx_sched *sch);
void lupos_scx_core_bypass_depth_write_once(struct scx_sched *sch, s32 depth);
void lupos_scx_core_bypass_slice_write_once(struct scx_sched *sch, u64 slice);
void lupos_scx_core_bypass_event_activate(struct scx_sched *sch);
void lupos_scx_core_bypass_event_duration(struct scx_sched *sch);
bool lupos_scx_core_bypass_claim_enable(struct scx_sched *sch);
bool lupos_scx_core_bypass_claim_disable(struct scx_sched *sch);
s32 lupos_scx_core_bypass_dsp_inc(struct scx_sched *sch);
s32 lupos_scx_core_bypass_dsp_dec(struct scx_sched *sch);
void lupos_scx_core_bypass_warn_enable_self(s32 ret);
void lupos_scx_core_bypass_warn_enable_host(s32 ret);
void lupos_scx_core_bypass_warn_disable_self(s32 ret);
void lupos_scx_core_bypass_warn_disable_parent(s32 ret);
struct scx_sched *lupos_scx_core_bypass_next_descendant(struct scx_sched *pos,
	struct scx_sched *root);
struct scx_sched_pcpu *lupos_scx_core_bypass_pcpu(struct scx_sched *sch, int cpu);
void lupos_scx_core_bypass_replay_ecaps(struct rq *rq, struct scx_sched *sch);
bool lupos_scx_core_bypass_enabled(void);
struct task_struct *lupos_scx_core_bypass_runnable_task(struct list_head *node);
struct scx_sched *lupos_scx_core_bypass_task_sched(struct task_struct *p);
bool lupos_scx_core_bypass_task_current(struct rq *rq, struct task_struct *p);
void lupos_scx_core_bypass_cycle_task(struct task_struct *p);
bool lupos_scx_core_bypass_cpu_online(int cpu);
int lupos_scx_core_bypass_processor_id(void);

#endif /* LUPOS_SCHED_EXT_CORE_BYPASS_BINDINGS_H */
