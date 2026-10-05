// SPDX-License-Identifier: GPL-2.0
/* Configured native primitives only; explicit unqualified runtime C boundary. */
#error "SOURCE ONLY HOLD: sched_ext sub ecaps native admission is pending"
#include "sched_ext_sub_bindings.h"
#ifdef CONFIG_EXT_SUB_SCHED
void lupos_scx_sub_assert_discard_syncs_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_sub_assert_process_sync_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_sub_assert_unbypass_replay_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_sub_updated_lock(struct scx_caps_updated *cu) { raw_spin_lock(&cu->lock); }
void lupos_scx_sub_updated_unlock(struct scx_caps_updated *cu) { raw_spin_unlock(&cu->lock); }
void lupos_scx_sub_list_add_tail(struct list_head *node, struct list_head *head) { list_add_tail(node, head); }
void lupos_scx_sub_list_del_init(struct list_head *node) { list_del_init(node); }
struct scx_caps_updated *lupos_scx_sub_updated_first(struct list_head *head)
{
	return list_first_entry_or_null(head, struct scx_caps_updated, node_in_flight);
}
struct scx_caps_updated *lupos_scx_sub_updated_next(struct list_head *head, struct scx_caps_updated *cu)
{
	return list_is_last(&cu->node_in_flight, head) ? NULL : list_next_entry(cu, node_in_flight);
}
struct scx_pshard *lupos_scx_sub_updated_owner(struct scx_caps_updated *cu)
{
	return container_of(cu, struct scx_pshard, caps_updated);
}
bool lupos_scx_sub_has_caps_op(struct scx_sched *sch) { return SCX_HAS_OP(sch, sub_caps_updated); }
bool lupos_scx_sub_aborting(struct scx_sched *sch) { return READ_ONCE(sch->aborting); }
void lupos_scx_sub_call_caps_op(struct scx_sched *sch, const struct scx_cmask *cmask, u64 caps)
{
	SCX_CALL_OP(sch, sub_caps_updated, NULL, cmask, caps);
}
void lupos_scx_sub_with_seed_list(struct scx_sched *sch)
{
	LIST_HEAD(to_deliver);
	guard(irqsave)();
	lupos_scx_sub_seed_with_list(sch, &to_deliver);
}
bool lupos_scx_sub_cmask_test(u32 cid, const struct scx_cmask *m) { return scx_cmask_test(cid, m); }
u64 lupos_scx_sub_caps_implied(u64 cap) { return scx_caps_implied(cap); }
s32 lupos_scx_sub_cid_cpu(s32 cid) { return __scx_cid_to_cpu(cid); }
void lupos_scx_sub_mb(void) { smp_mb(); }
void lupos_scx_sub_schedule_reenq_local(struct rq *rq, u64 flags) { scx_schedule_reenq_local(rq, flags); }
bool lupos_scx_sub_llist_on(struct llist_node *node) { return llist_on_list(node); }
bool lupos_scx_sub_llist_add(struct llist_node *node, struct llist_head *head) { return llist_add(node, head); }
bool lupos_scx_sub_llist_empty(struct llist_head *head) { return llist_empty(head); }
struct llist_node *lupos_scx_sub_llist_del_all(struct llist_head *head) { return llist_del_all(head); }
void lupos_scx_sub_llist_init(struct llist_node *node) { init_llist_node(node); }
void lupos_scx_sub_llist_add_batch(struct llist_node *head, struct llist_node *tail, struct llist_head *list)
{
	llist_add_batch(head, tail, list);
}
struct rq *lupos_scx_sub_cpu_rq(s32 cpu) { return cpu_rq(cpu); }
struct scx_sched *lupos_scx_sub_ancestor(struct scx_sched *sch, u32 index) { return sch->ancestors[index]; }
s32 lupos_scx_sub_cid_shard(s32 cid) { return rcu_dereference_all(scx_cid_to_shard)[cid]; }
struct scx_sched_pcpu *lupos_scx_sub_pcpu_from_sync(struct llist_node *node)
{
	return container_of(node, struct scx_sched_pcpu, ecaps_to_sync_node);
}
u64 lupos_scx_sub_read_ecaps(struct scx_sched_pcpu *pcpu) { return READ_ONCE(pcpu->ecaps); }
void lupos_scx_sub_write_ecaps(struct scx_sched_pcpu *pcpu, u64 caps) { WRITE_ONCE(pcpu->ecaps, caps); }
bool lupos_scx_sub_has_ecaps_op(struct scx_sched *sch) { return SCX_HAS_OP(sch, sub_ecaps_updated); }
struct scx_dsp_ctx *lupos_scx_sub_this_dsp_ctx(struct scx_sched *sch) { return &this_cpu_ptr(sch->pcpu)->dsp_ctx; }
void lupos_scx_sub_call_ecaps_op(struct scx_sched *sch, struct rq *rq, s32 cpu, u64 old, u64 caps)
{
	SCX_CALL_OP(sch, sub_ecaps_updated, rq, scx_cpu_arg(cpu), old, caps);
}
void lupos_scx_sub_ps_lock(struct scx_pshard *ps) { raw_spin_lock(&ps->lock); }
void lupos_scx_sub_ps_lock_nested(struct scx_pshard *ps) { raw_spin_lock_nested(&ps->lock, SINGLE_DEPTH_NESTING); }
void lupos_scx_sub_ps_unlock(struct scx_pshard *ps) { raw_spin_unlock(&ps->lock); }
bool lupos_scx_sub_enabled(void) { return scx_enabled(); }
void lupos_scx_sub_rq_lock(struct rq *rq, struct rq_flags *rf) { rq_lock_irqsave(rq, rf); }
void lupos_scx_sub_rq_unlock(struct rq *rq, struct rq_flags *rf) { rq_unlock_irqrestore(rq, rf); }
struct scx_sched *lupos_scx_sub_root_protected(void) { return scx_root_protected(); }
void lupos_scx_sub_cpu_relax(void) { cpu_relax(); }
u32 lupos_scx_sub_possible_limit(void)
{
#if NR_CPUS == 1
	return 1;
#else
	return small_cpumask_bits;
#endif
}
u32 lupos_scx_sub_possible_scan(u32 offset)
{
#if NR_CPUS == 1
	return offset;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits, offset);
#endif
}
#endif
