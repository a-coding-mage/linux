// SPDX-License-Identifier: GPL-2.0
/* F10 native primitives and native macro/resource envelopes. Unqualified C
 * runtime, never Rust algorithm coverage. Compose only in the single native
 * envelope, with F00 storage and F09's static callback declaration. */
#error "SOURCE ONLY HOLD: sched_ext bypass native ABI and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_bypass_bindings.h"

/* Preserve timer_setup's actual static native callback identity and CFI type. */
static void scx_bypass_lb_timerfn(struct timer_list *timer)
{ lupos_scx_core_bypass_lb_timer_body(timer); }

/* The self-referential native initializer and its READ_ONCE(donor_dsq->seq)
 * precede the body's nr/parameter reads, just as ext.c:5715. The cursor stays
 * at this address through every lock-drop/reacquire and is never copied. */
u32 lupos_scx_core_bypass_cpu_cursor_frame(struct scx_sched *sch,
	struct rq *donor_rq, struct scx_dispatch_q *donor_dsq,
	struct cpumask *donee_mask, struct cpumask *resched_mask,
	u32 nr_donor_target, u32 nr_donee_target)
{
	struct scx_dsq_list_node cursor = INIT_DSQ_LIST_CURSOR(cursor, donor_dsq, 0);
	return lupos_scx_core_bypass_cpu_cursor_body(sch, donor_rq, donor_dsq,
		&cursor, donee_mask, resched_mask, nr_donor_target, nr_donee_target);
}

struct rq *lupos_scx_core_bypass_cpu_rq(int cpu) { return cpu_rq(cpu); }
u32 lupos_scx_core_bypass_nr_read_once(struct scx_dispatch_q *dsq) { return READ_ONCE(dsq->nr); }
/* DIV_ROUND_UP evaluates d twice. F00 permits this exact macro envelope to
 * reference its one private slot after common-native composition. */
u32 lupos_scx_core_bypass_delta_threshold(u32 min_delta_us)
{ return DIV_ROUND_UP(min_delta_us, READ_ONCE(scx_slice_bypass_us)); }
u64 lupos_scx_core_bypass_ktime_get_ns(void) { return ktime_get_ns(); }
void lupos_scx_core_bypass_rq_lock_irq(struct rq *rq) { raw_spin_rq_lock_irq(rq); }
void lupos_scx_core_bypass_rq_unlock_irq(struct rq *rq) { raw_spin_rq_unlock_irq(rq); }
void lupos_scx_core_bypass_raw_lock(raw_spinlock_t *lock) __acquires(lock)
{ raw_spin_lock(lock); }
void lupos_scx_core_bypass_raw_unlock(raw_spinlock_t *lock) __releases(lock)
{ raw_spin_unlock(lock); }
/* These are the explicit original APIs at 6071/6179, not guard-derived IRQ
 * operations. Their flags remain native unsigned long across the Rust body. */
unsigned long lupos_scx_core_bypass_lock_irqsave(raw_spinlock_t *lock) __acquires(lock)
{ unsigned long flags; raw_spin_lock_irqsave(lock, flags); return flags; }
void lupos_scx_core_bypass_unlock_irqrestore(raw_spinlock_t *lock,
	unsigned long flags) __releases(lock)
{ raw_spin_unlock_irqrestore(lock, flags); }
void lupos_scx_core_bypass_list_add(struct list_head *node, struct list_head *head) { list_add(node, head); }
void lupos_scx_core_bypass_list_move_tail(struct list_head *node, struct list_head *head) { list_move_tail(node, head); }
void lupos_scx_core_bypass_list_del_init(struct list_head *node) { list_del_init(node); }
struct rq *lupos_scx_core_bypass_task_rq(struct task_struct *p) { return task_rq(p); }
bool lupos_scx_core_bypass_mask_empty(const struct cpumask *mask) { return cpumask_empty(mask); }
int lupos_scx_core_bypass_any_allowed(const struct cpumask *mask, struct task_struct *p) { return cpumask_any_and_distribute(mask, p->cpus_ptr); }
void lupos_scx_core_bypass_mask_set_cpu(int cpu, struct cpumask *mask) { cpumask_set_cpu(cpu, mask); }
void lupos_scx_core_bypass_mask_clear_cpu(int cpu, struct cpumask *mask) { cpumask_clear_cpu(cpu, mask); }
void lupos_scx_core_bypass_mask_clear(struct cpumask *mask) { cpumask_clear(mask); }
bool lupos_scx_core_bypass_mask_test_cpu(int cpu, const struct cpumask *mask) { return cpumask_test_cpu(cpu, mask); }
const struct cpumask *lupos_scx_core_bypass_node_mask(int node) { return cpumask_of_node(node); }
/* cpumask_var_t's configured pointer/array form stays entirely native. */
struct cpumask *lupos_scx_core_bypass_donee_mask(struct scx_sched *sch) { return sch->bypass_lb_donee_cpumask; }
struct cpumask *lupos_scx_core_bypass_resched_mask(struct scx_sched *sch) { return sch->bypass_lb_resched_cpumask; }

/* Match for_each_cpu[_and]'s find_next[_and]_bit and small_cpumask_bits,
 * without cpumask_next's additional cpumask_check or nr_cpu_ids substitution. */
int lupos_scx_core_bypass_next_online_node_cpu(int cpu, const struct cpumask *node_mask)
{ return find_next_and_bit(cpumask_bits(cpu_online_mask), cpumask_bits(node_mask), small_cpumask_bits, cpu + 1); }
int lupos_scx_core_bypass_next_mask_cpu(int cpu, const struct cpumask *mask)
{ return find_next_bit(cpumask_bits(mask), small_cpumask_bits, cpu + 1); }
unsigned int lupos_scx_core_bypass_mask_cpu_limit(void) { return small_cpumask_bits; }
int lupos_scx_core_bypass_next_possible_cpu(int cpu)
{
#if NR_CPUS == 1
	return cpu < 0 ? 0 : 1;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits, cpu + 1);
#endif
}
unsigned int lupos_scx_core_bypass_possible_cpu_limit(void)
{
#if NR_CPUS == 1
	return 1;
#else
	return small_cpumask_bits;
#endif
}
int lupos_scx_core_bypass_next_cpu_node(int node)
{
#if MAX_NUMNODES > 1
	return node < 0 ? first_node(node_states[N_CPU]) : next_node(node, node_states[N_CPU]);
#else
	return node < 0 ? 0 : 1;
#endif
}
unsigned int lupos_scx_core_bypass_cpu_node_limit(void) { return MAX_NUMNODES; }
void lupos_scx_core_bypass_trace_lb(int node, u32 nr_cpus, u32 nr_tasks,
	u32 nr_balanced, u32 before_min, u32 before_max, u32 after_min, u32 after_max)
{ trace_sched_ext_bypass_lb(node, nr_cpus, nr_tasks, nr_balanced, before_min, before_max, after_min, after_max); }
struct scx_sched *lupos_scx_core_bypass_timer_sched(struct timer_list *timer)
{ return container_of(timer, struct scx_sched, bypass_lb_timer); }
bool lupos_scx_core_bypass_dsp_enabled(struct scx_sched *sch) { return scx_bypass_dsp_enabled(sch); }
void lupos_scx_core_bypass_mod_timer(struct timer_list *timer, u32 intv_us)
{ mod_timer(timer, jiffies + usecs_to_jiffies(intv_us)); }
bool lupos_scx_core_bypass_timer_pending(struct timer_list *timer) { return timer_pending(timer); }
/* Keep both lockdep sites and all seven warning-state objects separate. */
/* F00 owns this one private lock; exact operands retain diagnostic text. */
void lupos_scx_core_bypass_assert_inc_lock(void) { lockdep_assert_held(&scx_bypass_lock); }
void lupos_scx_core_bypass_assert_dec_lock(void) { lockdep_assert_held(&scx_bypass_lock); }
void lupos_scx_core_bypass_warn_inc_depth(struct scx_sched *sch) { WARN_ON_ONCE(sch->bypass_depth < 0); }
void lupos_scx_core_bypass_warn_dec_depth(struct scx_sched *sch) { WARN_ON_ONCE(sch->bypass_depth < 1); }
void lupos_scx_core_bypass_depth_write_once(struct scx_sched *sch, s32 depth) { WRITE_ONCE(sch->bypass_depth, depth); }
void lupos_scx_core_bypass_slice_write_once(struct scx_sched *sch, u64 slice) { WRITE_ONCE(sch->slice_dfl, slice); }
void lupos_scx_core_bypass_event_activate(struct scx_sched *sch) { scx_add_event(sch, SCX_EV_BYPASS_ACTIVATE, 1); }
/* scx_add_event evaluates cnt separately for this_cpu_add and tracing. Keep
 * both time reads inside the macro, in the original per-CPU native envelope. */
void lupos_scx_core_bypass_event_duration(struct scx_sched *sch)
{ scx_add_event(sch, SCX_EV_BYPASS_DURATION, ktime_get_ns() - sch->bypass_timestamp); }
bool lupos_scx_core_bypass_claim_enable(struct scx_sched *sch) { return WARN_ON_ONCE(test_and_set_bit(0, &sch->bypass_dsp_claim)); }
bool lupos_scx_core_bypass_claim_disable(struct scx_sched *sch) { return test_and_clear_bit(0, &sch->bypass_dsp_claim); }
s32 lupos_scx_core_bypass_dsp_inc(struct scx_sched *sch) { return atomic_inc_return(&sch->bypass_dsp_enable_depth); }
s32 lupos_scx_core_bypass_dsp_dec(struct scx_sched *sch) { return atomic_dec_return(&sch->bypass_dsp_enable_depth); }
void lupos_scx_core_bypass_warn_enable_self(s32 ret) { WARN_ON_ONCE(ret <= 0); }
void lupos_scx_core_bypass_warn_enable_host(s32 ret) { WARN_ON_ONCE(ret <= 0); }
void lupos_scx_core_bypass_warn_disable_self(s32 ret) { WARN_ON_ONCE(ret < 0); }
void lupos_scx_core_bypass_warn_disable_parent(s32 ret) { WARN_ON_ONCE(ret < 0); }
struct scx_sched *lupos_scx_core_bypass_next_descendant(struct scx_sched *pos, struct scx_sched *root)
{ return scx_next_descendant_pre(pos, root); }
struct scx_sched_pcpu *lupos_scx_core_bypass_pcpu(struct scx_sched *sch, int cpu) { return per_cpu_ptr(sch->pcpu, cpu); }
void lupos_scx_core_bypass_replay_ecaps(struct rq *rq, struct scx_sched *sch) { scx_unbypass_replay_ecaps(rq, sch); }
bool lupos_scx_core_bypass_enabled(void) { return scx_enabled(); }
struct task_struct *lupos_scx_core_bypass_runnable_task(struct list_head *node) { return list_entry(node, struct task_struct, scx.runnable_node); }
struct scx_sched *lupos_scx_core_bypass_task_sched(struct task_struct *p) { return scx_task_sched(p); }
bool lupos_scx_core_bypass_task_current(struct rq *rq, struct task_struct *p) { return task_current(rq, p); }
void lupos_scx_core_bypass_cycle_task(struct task_struct *p)
{
	scoped_guard (sched_change, p, DEQUEUE_SAVE | DEQUEUE_MOVE) {
		/* nothing */ ;
	}
}
bool lupos_scx_core_bypass_cpu_online(int cpu) { return cpu_online(cpu); }
int lupos_scx_core_bypass_processor_id(void) { return smp_processor_id(); }
