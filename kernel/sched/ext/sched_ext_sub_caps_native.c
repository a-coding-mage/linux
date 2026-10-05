// SPDX-License-Identifier: GPL-2.0
/* Native scratch/IRQ/RCU/formatting leaves, an unqualified C runtime boundary. */
#error "SOURCE ONLY HOLD: sched_ext sub caps native admission is pending"
#include "sched_ext_sub_bindings.h"
#ifdef CONFIG_EXT_SUB_SCHED
void lupos_scx_sub_rcu_lock(void) { rcu_read_lock(); }
void lupos_scx_sub_rcu_unlock(void) { rcu_read_unlock(); }
struct scx_sched *lupos_scx_sub_prog_sched(const struct bpf_prog_aux *aux) { return scx_prog_sched(aux); }
void lupos_scx_sub_error_recursion(struct scx_sched *sch, const char *op) { scx_error(sch, "%s recursion detected", op); }
const char *lupos_scx_sub_dispatch_name(void) { return "dispatch"; }
const char *lupos_scx_sub_caps_name(void) { return "sub_caps_updated"; }
struct rq *lupos_scx_sub_locked_rq(void) { return scx_locked_rq(); }
enum scx_dsp_verdict lupos_scx_sub_dispatch_sched(struct scx_sched *sch, struct rq *rq, struct task_struct *prev, bool nested) { return scx_dispatch_sched(sch, rq, prev, nested); }
void lupos_scx_sub_error_dispatch(struct scx_sched *sch, u64 id)
{
	scx_error(sch, "trying to dispatch a distant sub-sched on cgroup %llu", id);
}
bool lupos_scx_sub_is_cid_type(void) { return scx_is_cid_type(); }
/* Each original diagnostic expansion retains a distinct native call site. */
void lupos_scx_sub_error_preamble_cid_form(struct scx_sched *sch)
{
	scx_error(sch, "sub-cap kfuncs require a cid-form scheduler");
}
void lupos_scx_sub_error_read_cid_form(struct scx_sched *sch)
{
	scx_error(sch, "sub-cap kfuncs require a cid-form scheduler");
}
void lupos_scx_sub_error_kill_cid_form(struct scx_sched *sch)
{
	scx_error(sch, "sub-cap kfuncs require a cid-form scheduler");
}
void lupos_scx_sub_error_preamble_direct_child(struct scx_sched *sch, u64 id)
{
	scx_error(sch, "%s: sub-%llu is not a direct child", sch->cgrp_path, id);
}
void lupos_scx_sub_error_read_direct_child(struct scx_sched *sch, u64 id)
{
	scx_error(sch, "%s: sub-%llu is not a direct child", sch->cgrp_path, id);
}
void lupos_scx_sub_error_kill_direct_child(struct scx_sched *sch, u64 id)
{
	scx_error(sch, "%s: sub-%llu is not a direct child", sch->cgrp_path, id);
}
void lupos_scx_sub_error_preamble_caps(struct scx_sched *sch, u64 caps) { scx_error(sch, "invalid caps 0x%llx", caps); }
void lupos_scx_sub_error_read_caps(struct scx_sched *sch, u64 caps) { scx_error(sch, "invalid caps 0x%llx", caps); }
void lupos_scx_sub_error_grant_cmask(struct scx_sched *sch, s32 ret) { scx_error(sch, "invalid cmask (%d)", ret); }
void lupos_scx_sub_error_revoke_cmask(struct scx_sched *sch, s32 ret) { scx_error(sch, "invalid cmask (%d)", ret); }
void lupos_scx_sub_error_denied(struct scx_sched *sch, s32 ret) { scx_error(sch, "invalid denied_out (%d)", ret); }
void lupos_scx_sub_error_out(struct scx_sched *sch, s32 ret) { scx_error(sch, "invalid out (%d)", ret); }
void lupos_scx_sub_error_uninitialized_caps(struct scx_sched *sch)
{
	scx_error(sch, "scx_bpf_sub_caps() called before caps storage is initialized");
}
u32 lupos_scx_sub_cmask_used_words(const struct scx_cmask *m) { return scx_cmask_nr_used_words(m); }
u64 lupos_scx_sub_cmask_read_word(const struct scx_cmask *m, u32 index) { return READ_ONCE(m->bits[index]); }
s32 lupos_scx_sub_with_grant_masks(u64 id, u64 caps, const struct scx_cmask *cmask, struct scx_cmask *denied_out, const struct bpf_prog_aux *aux)
{
	SCX_CMASK_DEFINE_SHARD(slice, 0, SCX_CID_SHARD_MAX_CPUS);
	SCX_CMASK_DEFINE_SHARD(granted, 0, SCX_CID_SHARD_MAX_CPUS);
	SCX_CMASK_DEFINE_SHARD(changed, 0, SCX_CID_SHARD_MAX_CPUS);
	SCX_CMASK_DEFINE_SHARD(delta, 0, SCX_CID_SHARD_MAX_CPUS);
	SCX_CMASK_DEFINE_SHARD(denied, 0, SCX_CID_SHARD_MAX_CPUS);
	LIST_HEAD(to_deliver);
	const struct lupos_scx_sub_grant_scratch scratch = {
		.slice = slice, .granted = granted, .changed = changed,
		.delta = delta, .denied = denied, .to_deliver = &to_deliver,
	};
	guard(irqsave)();
	return lupos_scx_sub_grant_locked(id, caps, cmask, denied_out, aux, &scratch);
}
void lupos_scx_sub_with_revoke_masks(u64 id, u64 caps, const struct scx_cmask *cmask, const struct bpf_prog_aux *aux)
{
	SCX_CMASK_DEFINE_SHARD(slice, 0, SCX_CID_SHARD_MAX_CPUS);
	SCX_CMASK_DEFINE_SHARD(changed, 0, SCX_CID_SHARD_MAX_CPUS);
	SCX_CMASK_DEFINE_SHARD(delta, 0, SCX_CID_SHARD_MAX_CPUS);
	LIST_HEAD(to_deliver);
	guard(irqsave)();
	lupos_scx_sub_revoke_locked(id, caps, cmask, aux, slice, changed, delta, &to_deliver);
}
s32 lupos_scx_sub_with_caps_mask(u64 id, u64 caps, struct scx_cmask *out, const struct bpf_prog_aux *aux)
{
	SCX_CMASK_DEFINE_SHARD(local, 0, SCX_CID_SHARD_MAX_CPUS);
	guard(irqsave)();
	return lupos_scx_sub_caps_locked(id, caps, out, aux, local);
}
struct scx_pshard **lupos_scx_sub_acquire_pshards(struct scx_sched *sch) { return smp_load_acquire(&sch->pshard); }
const struct scx_cid_shard *lupos_scx_sub_shard_range_all(s32 index) { return &rcu_dereference_all(scx_cid_shard_ranges)[index]; }
void lupos_scx_sub_exit_bstr(struct scx_sched *child, struct scx_sched *parent, char *fmt, unsigned long long *data, u32 size)
{
	scx_exit_bstr(child, SCX_EXIT_PARENT_KILL, 0, parent, fmt, data, size);
}
#endif
