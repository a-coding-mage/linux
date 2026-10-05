/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_SUB_ECAPS_BINDINGS_H
#define LUPOS_SCHED_EXT_SUB_ECAPS_BINDINGS_H
void lupos_scx_sub_assert_discard_syncs_rq(struct rq *rq);
void lupos_scx_sub_assert_process_sync_rq(struct rq *rq);
void lupos_scx_sub_assert_unbypass_replay_rq(struct rq *rq);
void lupos_scx_sub_updated_lock(struct scx_caps_updated *cu);
void lupos_scx_sub_updated_unlock(struct scx_caps_updated *cu);
void lupos_scx_sub_list_add_tail(struct list_head *node, struct list_head *head);
void lupos_scx_sub_list_del_init(struct list_head *node);
struct scx_caps_updated *lupos_scx_sub_updated_first(struct list_head *head);
struct scx_caps_updated *lupos_scx_sub_updated_next(struct list_head *head, struct scx_caps_updated *cu);
struct scx_pshard *lupos_scx_sub_updated_owner(struct scx_caps_updated *cu);
bool lupos_scx_sub_has_caps_op(struct scx_sched *sch);
bool lupos_scx_sub_aborting(struct scx_sched *sch);
void lupos_scx_sub_call_caps_op(struct scx_sched *sch, const struct scx_cmask *cmask, u64 caps);
void lupos_scx_sub_with_seed_list(struct scx_sched *sch);
void lupos_scx_sub_seed_with_list(struct scx_sched *sch, struct list_head *list);
bool lupos_scx_sub_cmask_test(u32 cid, const struct scx_cmask *m);
u64 lupos_scx_sub_caps_implied(u64 cap);
s32 lupos_scx_sub_cid_cpu(s32 cid);
void lupos_scx_sub_mb(void);
void lupos_scx_sub_schedule_reenq_local(struct rq *rq, u64 flags);
bool lupos_scx_sub_llist_on(struct llist_node *node);
bool lupos_scx_sub_llist_add(struct llist_node *node, struct llist_head *head);
bool lupos_scx_sub_llist_empty(struct llist_head *head);
struct llist_node *lupos_scx_sub_llist_del_all(struct llist_head *head);
void lupos_scx_sub_llist_init(struct llist_node *node);
void lupos_scx_sub_llist_add_batch(struct llist_node *head, struct llist_node *tail, struct llist_head *list);
struct rq *lupos_scx_sub_cpu_rq(s32 cpu);
struct scx_sched *lupos_scx_sub_ancestor(struct scx_sched *sch, u32 index);
s32 lupos_scx_sub_cid_shard(s32 cid);
struct scx_sched_pcpu *lupos_scx_sub_pcpu_from_sync(struct llist_node *node);
u64 lupos_scx_sub_read_ecaps(struct scx_sched_pcpu *pcpu);
void lupos_scx_sub_write_ecaps(struct scx_sched_pcpu *pcpu, u64 caps);
bool lupos_scx_sub_has_ecaps_op(struct scx_sched *sch);
struct scx_dsp_ctx *lupos_scx_sub_this_dsp_ctx(struct scx_sched *sch);
void lupos_scx_sub_call_ecaps_op(struct scx_sched *sch, struct rq *rq, s32 cpu, u64 old, u64 caps);
void lupos_scx_sub_ps_lock(struct scx_pshard *ps);
void lupos_scx_sub_ps_lock_nested(struct scx_pshard *ps);
void lupos_scx_sub_ps_unlock(struct scx_pshard *ps);
bool lupos_scx_sub_enabled(void);
void lupos_scx_sub_rq_lock(struct rq *rq, struct rq_flags *rf);
void lupos_scx_sub_rq_unlock(struct rq *rq, struct rq_flags *rf);
struct scx_sched *lupos_scx_sub_root_protected(void);
void lupos_scx_sub_cpu_relax(void);
u32 lupos_scx_sub_possible_limit(void);
u32 lupos_scx_sub_possible_scan(u32 offset);
#endif
