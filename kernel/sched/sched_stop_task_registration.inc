/* SPDX-License-Identifier: GPL-2.0 */
/* Metadata only: exact const stop_sched_class symbol, __stop_sched_class
 * section, native instance alignment and zero initialization of omitted fields.
 * Every algorithm callback remains implemented by the Rust owner. */
DEFINE_SCHED_CLASS(stop) = {
	.enqueue_task		= lupos_stop_task_enqueue_task,
	.dequeue_task		= lupos_stop_task_dequeue_task,
	.yield_task		= lupos_stop_task_yield_task,
	.wakeup_preempt		= lupos_stop_task_wakeup_preempt,
	.pick_task		= lupos_stop_task_pick_task,
	.put_prev_task		= lupos_stop_task_put_prev_task,
	.set_next_task		= lupos_stop_task_set_next_task,
	.balance		= lupos_stop_task_balance,
	.select_task_rq		= lupos_stop_task_select_task_rq,
	.set_cpus_allowed	= set_cpus_allowed_common,
	.task_tick		= lupos_stop_task_task_tick,
	.prio_changed		= lupos_stop_task_prio_changed,
	.switching_to		= lupos_stop_task_switching_to,
	.update_curr		= lupos_stop_task_update_curr,
};
