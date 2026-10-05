// SPDX-License-Identifier: GPL-2.0
/* F04 native primitives, original macro envelopes and typed callback targets.
 * Include in F00's single native build_policy envelope, never as an independent
 * object. These are unqualified native runtime boundaries, not Rust coverage. */
#error "SOURCE ONLY HOLD: sched_ext pick native ABI, recursion, stack and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_pick_bindings.h"

/* Exact C callback signatures; each forwards to its one Rust owner. F17 must
 * reference these static identities in the same native translation unit. */
static void set_next_task_scx(struct rq *rq, struct task_struct *p, bool first)
{
	lupos_scx_core_pick_set_next_body(rq, p, first);
}
static void put_prev_task_scx(struct rq *rq, struct task_struct *p,
			      struct task_struct *next)
{
	lupos_scx_core_pick_put_prev_body(rq, p, next);
}
static void kick_sync_wait_bal_cb(struct rq *rq)
{
	lupos_scx_core_pick_kick_sync_wait_body(rq);
}
static struct task_struct *pick_task_scx(struct rq *rq, struct rq_flags *rf)
{
	return lupos_scx_core_pick_task_body(rq, rf);
}
static struct task_struct *
ext_server_pick_task(struct sched_dl_entity *dl_se, struct rq_flags *rf)
{
	return lupos_scx_core_pick_server_task_body(dl_se, rf);
}

s32 lupos_scx_core_pick_cpu_of(struct rq *rq) { return cpu_of(rq); }
struct scx_sched *lupos_scx_core_pick_root_protected_live(void)
{
	return scx_root_protected_live();
}
s32 lupos_scx_core_pick_processor_id(void) { return smp_processor_id(); }
void lupos_scx_core_pick_assert_dispatch_rq(struct rq *rq)
{
	lockdep_assert_rq_held(rq);
}
void lupos_scx_core_pick_process_sync_ecaps(struct rq *rq, struct task_struct *prev)
{
	/* Existing sub-owner operation; not an ext.c fallback. */
	scx_process_sync_ecaps(rq, prev);
}
bool lupos_scx_core_pick_has_cpu_acquire(struct scx_sched *sch)
{
	return sch->ops.cpu_acquire;
}
bool lupos_scx_core_pick_has_cpu_release(struct scx_sched *sch)
{
	return sch->ops.cpu_release;
}
void lupos_scx_core_pick_call_cpu_acquire(struct scx_sched *sch, struct rq *rq, s32 cpu)
{
	SCX_CALL_OP(sch, cpu_acquire, rq, cpu, NULL);
}
void lupos_scx_core_pick_call_cpu_release(struct scx_sched *sch, struct rq *rq,
        struct task_struct *next, enum scx_cpu_preempt_reason reason)
{
	/* Only the original native aggregate is materialized here. Rust owns
	 * preempt_reason_from_class, and cpu_of stays inside SCX_CALL_OP. */
	struct scx_cpu_release_args args = {
		.reason = reason,
		.task = next,
	};

	SCX_CALL_OP(sch, cpu_release, rq, cpu_of(rq), &args);
}
bool lupos_scx_core_pick_has_running(struct scx_sched *sch)
{
	return SCX_HAS_OP(sch, running);
}
bool lupos_scx_core_pick_has_stopping(struct scx_sched *sch)
{
	return SCX_HAS_OP(sch, stopping);
}
void lupos_scx_core_pick_call_running(struct scx_sched *sch, struct rq *rq,
                                     struct task_struct *p)
{
	SCX_CALL_OP_TASK(sch, running, rq, p);
}
void lupos_scx_core_pick_call_stopping(struct scx_sched *sch, struct rq *rq,
                                      struct task_struct *p)
{
	SCX_CALL_OP_TASK(sch, stopping, rq, p, true);
}
bool lupos_scx_core_pick_can_stay(struct rq *rq, struct task_struct *p)
{
	return scx_task_can_stay_on_cpu(rq, p);
}
void lupos_scx_core_pick_event_keep_last(struct scx_sched *sch)
{
	__scx_add_event(sch, SCX_EV_DISPATCH_KEEP_LAST, 1);
}
void lupos_scx_core_pick_schedule_reenq(struct rq *rq)
{
	scx_schedule_reenq_local(rq, 0);
}
u64 lupos_scx_core_pick_clock_task(struct rq *rq) { return rq_clock_task(rq); }
void lupos_scx_core_pick_update_tick_dependency(struct rq *rq)
{
	sched_update_tick_dependency(rq);
}
void lupos_scx_core_pick_update_other_load_avgs(struct rq *rq)
{
	update_other_load_avgs(rq);
}
bool lupos_scx_core_pick_nohz_full(struct rq *rq)
{
	return tick_nohz_full_cpu(cpu_of(rq));
}
void lupos_scx_core_pick_tick_dep_set(struct rq *rq)
{
	tick_nohz_dep_set_cpu(cpu_of(rq), TICK_DEP_BIT_SCHED);
}
const struct sched_class *lupos_scx_core_pick_dl_class(void) { return &dl_sched_class; }
const struct sched_class *lupos_scx_core_pick_rt_class(void) { return &rt_sched_class; }
bool lupos_scx_core_pick_rescue_keep(struct rq *rq, struct task_struct *p)
{
	return scx_rescue_keep(rq, p);
}
void lupos_scx_core_pick_warn_enq_last(struct rq *rq, struct scx_sched *sch)
{
	WARN_ON_ONCE(!sched_core_enabled(rq) &&
		     !(sch->ops.flags & SCX_OPS_ENQ_LAST));
}

void lupos_scx_core_pick_kick_sync_advance(struct rq *rq)
{
	smp_store_release(&rq->scx.kick_sync, rq->scx.kick_sync + 1);
}
bool lupos_scx_core_pick_kick_sync_acquire_advanced(s32 cpu,
        const unsigned long *ksyncs)
{
	/* Keep the original plain snapshot operand in native C too. This is an
	 * access-boundary repair, not synchronization or race qualification. */
	return smp_load_acquire(&cpu_rq(cpu)->scx.kick_sync) != ksyncs[cpu];
}
bool lupos_scx_core_pick_kick_sync_read_once_pending(s32 cpu,
        const unsigned long *ksyncs)
{
	return READ_ONCE(cpu_rq(cpu)->scx.kick_sync) == ksyncs[cpu];
}
bool lupos_scx_core_pick_next_sync_cpu(struct rq *rq, s32 *cpu)
{
	/* Exact condition of for_each_cpu -> for_each_set_bit at cpumask.h:380
	 * and find.h:583. Rust owns initial cpu=0, increment and loop body.
	 * Do not substitute cpumask_next/nr_cpu_ids or snapshot the mask/limit. */
	*cpu = find_next_bit(cpumask_bits(rq->scx.cpus_to_sync),
			     small_cpumask_bits, *cpu);
	return *cpu < small_cpumask_bits;
}
void lupos_scx_core_pick_clear_sync_cpu(struct rq *rq, s32 cpu)
{
	cpumask_clear_cpu(cpu, rq->scx.cpus_to_sync);
}
void lupos_scx_core_pick_unlock_irq(struct rq *rq)
{
	/* Explicit original API, not the different scoped IRQ guard protocol. */
	raw_spin_rq_unlock_irq(rq);
}
void lupos_scx_core_pick_lock_irq(struct rq *rq)
{
	raw_spin_rq_lock_irq(rq);
}
struct task_struct *lupos_scx_core_pick_first_local(struct rq *rq)
{
	return list_first_entry_or_null(&rq->scx.local_dsq.list,
					struct task_struct, scx.dsq_list.node);
}
void lupos_scx_core_pick_unpin(struct rq *rq, struct rq_flags *rf)
{
	rq_unpin_lock(rq, rf);
}
void lupos_scx_core_pick_repin(struct rq *rq, struct rq_flags *rf)
{
	rq_repin_lock(rq, rf);
}
void lupos_scx_core_pick_queue_kick_sync(struct rq *rq)
{
	queue_balance_callback(rq, &rq->scx.kick_sync_bal_cb,
				kick_sync_wait_bal_cb);
}
void lupos_scx_core_pick_modified_begin(struct rq *rq)
{
	rq_modified_begin(rq, &ext_sched_class);
}
bool lupos_scx_core_pick_modified_above(struct rq *rq)
{
	return rq_modified_above(rq, &ext_sched_class);
}
struct task_struct *lupos_scx_core_pick_retry_task(void) { return RETRY_TASK; }
bool lupos_scx_core_pick_warned_zero_slice(struct scx_sched *sch)
{
	return sch->warned_zero_slice;
}
void lupos_scx_core_pick_mark_warned_zero_slice(struct scx_sched *sch)
{
	sch->warned_zero_slice = true;
}

/* Preserve the original printk_deferred index metadata and __func__ value.
 * Configured include/prefix mapping controls the original __FILE__ spelling;
 * its absence is an additional hard error, never guessed or spoofed by #line. */
#ifdef CONFIG_PRINTK_INDEX
#ifndef LUPOS_SCX_PICK_ORIGINAL_FILE
#error "SOURCE ONLY HOLD: pick printk index requires original ext.c __FILE__ spelling"
#endif
#endif
void lupos_scx_core_pick_print_zero_slice(struct task_struct *p)
{
#ifdef CONFIG_PRINTK_INDEX
	static const struct pi_entry _entry __used = {
		.fmt = KERN_WARNING "sched_ext: %s[%d] has zero slice in %s()\n",
		.func = "do_pick_task_scx",
		.file = LUPOS_SCX_PICK_ORIGINAL_FILE,
		.line = 3409,
		.level = NULL,
		.subsys_fmt_prefix = NULL,
	};
	static const struct pi_entry *_entry_ptr
		__used __section(".printk_index") = &_entry;
#endif
	/* Original printk_deferred() expands to printk_index_wrap around this
	 * runtime function. No adapter-location index record is emitted. */
	_printk_deferred(KERN_WARNING "sched_ext: %s[%d] has zero slice in %s()\n",
			 p->comm, p->pid, "do_pick_task_scx");
}
void lupos_scx_core_pick_dl_server_init(struct sched_dl_entity *dl_se, struct rq *rq)
{
	dl_server_init(dl_se, rq, ext_server_pick_task);
}

bool lupos_scx_core_pick_unlikely_cpu_released(struct rq *rq)
{
	return unlikely(rq->scx.cpu_released);
}
bool lupos_scx_core_pick_unlikely_extra_immed(struct rq *rq)
{
	return unlikely(rq->scx.local_dsq.nr > 1 && rq->scx.nr_immed);
}
bool lupos_scx_core_pick_unlikely_rescue_no_slice(struct task_struct *p, struct rq *rq)
{
	return unlikely(p == scx_rescuee(rq));
}
bool lupos_scx_core_pick_unlikely_rescue_keep_local(struct task_struct *p, struct rq *rq)
{
	return unlikely(p == scx_rescuee(rq));
}
bool lupos_scx_core_pick_unlikely_rescue_enq_flags(struct task_struct *p, struct rq *rq)
{
	return unlikely(p == scx_rescuee(rq));
}
bool lupos_scx_core_pick_unlikely_foreign_wait(struct rq *rq)
{
	return unlikely(cpu_of(rq) != smp_processor_id());
}
bool lupos_scx_core_pick_unlikely_sync_pending(struct rq *rq)
{
	return unlikely(rq->scx.kick_sync_pending);
}
bool lupos_scx_core_pick_unlikely_zero_slice(struct task_struct *p)
{
	return unlikely(!p->scx.slice);
}
#ifdef CONFIG_SCHED_CORE
bool lupos_scx_core_pick_unlikely_core_sync_pending(struct rq *rq)
{
	return unlikely(rq->scx.kick_sync_pending);
}
bool lupos_scx_core_pick_unlikely_foreign_balance(struct rq *rq)
{
	return unlikely(rq->scx.flags & SCX_RQ_BAL_CB_PENDING);
}
bool lupos_scx_core_pick_has_core_sched_before(struct scx_sched *sch)
{
	return SCX_HAS_OP(sch, core_sched_before);
}
bool lupos_scx_core_pick_call_core_sched_before(struct scx_sched *sch,
        const struct task_struct *a, const struct task_struct *b)
{
	/* Repeated task_rq(a) evaluations remain within the real macro, after
	 * its task guard mutations; do not pass an early Rust rq snapshot. */
	return SCX_CALL_OP_2TASKS_RET(sch, core_sched_before, task_rq(a),
				      (struct task_struct *)b,
				      (struct task_struct *)a);
}
s32 lupos_scx_core_pick_task_cpu(const struct task_struct *p) { return task_cpu(p); }
#endif

struct scx_dsp_ctx *lupos_scx_core_pick_inline_dsp_ctx(struct scx_sched *sch)
{
	return &this_cpu_ptr(sch->pcpu)->dsp_ctx;
}
bool lupos_scx_core_pick_inline_task_on_sched(struct scx_sched *sch,
                                            const struct task_struct *prev)
{
	return scx_task_on_sched(sch, prev);
}
bool lupos_scx_core_pick_inline_bypass_enabled(struct scx_sched *sch)
{
	return scx_bypass_dsp_enabled(sch);
}
#ifdef CONFIG_EXT_SUB_SCHED
struct scx_sched_pcpu *lupos_scx_core_pick_inline_pcpu(struct scx_sched *sch, s32 cpu)
{
	return per_cpu_ptr(sch->pcpu, cpu);
}
void lupos_scx_core_pick_inline_event_sub_bypass(struct scx_sched *sch)
{
	__scx_add_event(sch, SCX_EV_SUB_BYPASS_DISPATCH, 1);
}
#endif
bool lupos_scx_core_pick_inline_unlikely_no_dispatch(struct scx_sched *sch)
{
	return unlikely(!SCX_HAS_OP(sch, dispatch));
}
void lupos_scx_core_pick_inline_call_dispatch(struct scx_sched *sch, struct rq *rq,
        s32 cpu, struct task_struct *prev, bool prev_on_sch)
{
	/* scx_cpu_arg and the conditional prev expression stay inside the macro;
	 * nested calls retain its original scx_locked_rq save/restore behavior. */
	SCX_CALL_OP(sch, dispatch, rq, scx_cpu_arg(cpu),
		    prev_on_sch ? prev : NULL);
}
bool lupos_scx_core_pick_inline_unlikely_last_loop(int *nr_loops)
{
	return unlikely(!--*nr_loops);
}

/* No native scx_dispatch_sched bridge is installed here. Existing native/sub
 * references still reach the unchanged header and remain unmigrated ext-owned
 * dependencies. A future integrator must resolve callsite/inline/stack policy
 * explicitly; this file neither clones that algorithm nor calls it as fallback.
 * Preserve original CFLAGS_build_policy.o += -DDISABLE_BRANCH_PROFILING. */
// Assisted-by: LLM
