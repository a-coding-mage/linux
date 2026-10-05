/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_SUB_CAPS_BINDINGS_H
#define LUPOS_SCHED_EXT_SUB_CAPS_BINDINGS_H
#define LUPOS_SCX_SUB_ENODEV ENODEV
#define LUPOS_SCX_SUB_EOPNOTSUPP EOPNOTSUPP
#define LUPOS_SCX_SUB_EINVAL EINVAL
#define LUPOS_SCX_SUB_EPERM EPERM
/* New private bridge descriptor, generated from this C authority. Every mask
 * and the list borrow automatic storage for one synchronous call only. */
struct lupos_scx_sub_grant_scratch {
	struct scx_cmask *slice;
	struct scx_cmask *granted;
	struct scx_cmask *changed;
	struct scx_cmask *delta;
	struct scx_cmask *denied;
	struct list_head *to_deliver;
};
void lupos_scx_sub_rcu_lock(void);
void lupos_scx_sub_rcu_unlock(void);
struct scx_sched *lupos_scx_sub_prog_sched(const struct bpf_prog_aux *aux);
void lupos_scx_sub_error_recursion(struct scx_sched *sch, const char *op);
const char *lupos_scx_sub_dispatch_name(void);
const char *lupos_scx_sub_caps_name(void);
struct rq *lupos_scx_sub_locked_rq(void);
enum scx_dsp_verdict lupos_scx_sub_dispatch_sched(struct scx_sched *sch, struct rq *rq, struct task_struct *prev, bool nested);
void lupos_scx_sub_error_dispatch(struct scx_sched *sch, u64 id);
bool lupos_scx_sub_is_cid_type(void);
void lupos_scx_sub_error_preamble_cid_form(struct scx_sched *sch);
void lupos_scx_sub_error_read_cid_form(struct scx_sched *sch);
void lupos_scx_sub_error_kill_cid_form(struct scx_sched *sch);
void lupos_scx_sub_error_preamble_direct_child(struct scx_sched *sch, u64 id);
void lupos_scx_sub_error_read_direct_child(struct scx_sched *sch, u64 id);
void lupos_scx_sub_error_kill_direct_child(struct scx_sched *sch, u64 id);
void lupos_scx_sub_error_preamble_caps(struct scx_sched *sch, u64 caps);
void lupos_scx_sub_error_read_caps(struct scx_sched *sch, u64 caps);
void lupos_scx_sub_error_grant_cmask(struct scx_sched *sch, s32 ret);
void lupos_scx_sub_error_revoke_cmask(struct scx_sched *sch, s32 ret);
void lupos_scx_sub_error_denied(struct scx_sched *sch, s32 ret);
void lupos_scx_sub_error_out(struct scx_sched *sch, s32 ret);
void lupos_scx_sub_error_uninitialized_caps(struct scx_sched *sch);
u32 lupos_scx_sub_cmask_used_words(const struct scx_cmask *m);
u64 lupos_scx_sub_cmask_read_word(const struct scx_cmask *m, u32 index);
s32 lupos_scx_sub_with_grant_masks(u64 id, u64 caps, const struct scx_cmask *cmask, struct scx_cmask *denied, const struct bpf_prog_aux *aux);
s32 lupos_scx_sub_grant_locked(u64 id, u64 caps, const struct scx_cmask *cmask, struct scx_cmask *denied, const struct bpf_prog_aux *aux, const struct lupos_scx_sub_grant_scratch *scratch);
void lupos_scx_sub_with_revoke_masks(u64 id, u64 caps, const struct scx_cmask *cmask, const struct bpf_prog_aux *aux);
void lupos_scx_sub_revoke_locked(u64 id, u64 caps, const struct scx_cmask *cmask, const struct bpf_prog_aux *aux, struct scx_cmask *slice, struct scx_cmask *changed, struct scx_cmask *delta, struct list_head *list);
s32 lupos_scx_sub_with_caps_mask(u64 id, u64 caps, struct scx_cmask *out, const struct bpf_prog_aux *aux);
s32 lupos_scx_sub_caps_locked(u64 id, u64 caps, struct scx_cmask *out, const struct bpf_prog_aux *aux, struct scx_cmask *local);
struct scx_pshard **lupos_scx_sub_acquire_pshards(struct scx_sched *sch);
const struct scx_cid_shard *lupos_scx_sub_shard_range_all(s32 index);
void lupos_scx_sub_exit_bstr(struct scx_sched *child, struct scx_sched *parent, char *fmt, unsigned long long *data, u32 size);
#endif
