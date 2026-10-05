// SPDX-License-Identifier: GPL-2.0
/* F13: native metadata, callback shells, layout and primitive leaves.
 * Explicit unqualified C runtime. Include in the original build_policy native
 * envelope with ext/CID/arena/idle/sub. NEVER compile as an independent object.
 * F12 must supply its real static scx_enable; F17 owns final registration.
 */
#error "SOURCE ONLY HOLD: struct_ops/BTF ABI, metadata and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_structops_btf_bindings.h"

static const struct btf_type *task_struct_type;
/* Existing same-TU F12 definition, not an external linkage workaround. */
static s32 scx_enable(struct scx_enable_cmd *cmd, struct bpf_link *link);

static bool bpf_scx_is_valid_access(int off, int size,
				    enum bpf_access_type type,
				    const struct bpf_prog *prog,
				    struct bpf_insn_access_aux *info)
{ return lupos_scx_st_valid_access_body(off, size, type, prog, info); }

static int bpf_scx_btf_struct_access(struct bpf_verifier_log *log,
				     const struct bpf_reg_state *reg, int off,
				     int size)
{ return lupos_scx_st_struct_access_body(log, reg, off, size); }

static int bpf_scx_cid_btf_struct_access(struct bpf_verifier_log *log,
					 const struct bpf_reg_state *reg, int off,
					 int size)
{ return lupos_scx_st_cid_struct_access_body(log, reg, off, size); }

/* BEGIN EXACT METADATA ext.c:7986-7996 */
static const struct bpf_verifier_ops bpf_scx_verifier_ops = {
	.get_func_proto = bpf_base_func_proto,
	.is_valid_access = bpf_scx_is_valid_access,
	.btf_struct_access = bpf_scx_btf_struct_access,
};

static const struct bpf_verifier_ops bpf_scx_cid_verifier_ops = {
	.get_func_proto = bpf_base_func_proto,
	.is_valid_access = bpf_scx_is_valid_access,
	.btf_struct_access = bpf_scx_cid_btf_struct_access,
};
/* END EXACT METADATA */

static int bpf_scx_init_member(const struct btf_type *t,
			       const struct btf_member *member,
			       void *kdata, const void *udata)
{ return lupos_scx_st_init_member_body(t, member, kdata, udata); }

static int bpf_scx_check_member(const struct btf_type *t,
				const struct btf_member *member,
				const struct bpf_prog *prog)
{ return lupos_scx_st_check_member_body(t, member, prog); }

static int bpf_scx_reg(void *kdata, struct bpf_link *link)
{
	struct scx_enable_cmd cmd = { .ops = kdata };

	return lupos_scx_st_reg_body(&cmd, link);
}

static int scx_arena_scan_prog(struct bpf_prog *prog, void *data)
{
	struct scx_arena_scan *s = data;

	return lupos_scx_st_scan_prog_body(prog, s);
}

static int bpf_scx_reg_cid(void *kdata, struct bpf_link *link)
{
	struct scx_enable_cmd cmd = { .ops_cid = kdata, .is_cid_type = true };
	struct scx_arena_scan scan = {};

	return lupos_scx_st_reg_cid_body(kdata, link, &cmd, &scan);
}

static void bpf_scx_unreg(void *kdata, struct bpf_link *link)
{ lupos_scx_st_unreg_body(kdata, link); }
static int bpf_scx_init(struct btf *btf)
{ return lupos_scx_st_init_body(btf); }
static int bpf_scx_update(void *kdata, void *old_kdata, struct bpf_link *link)
{ return lupos_scx_st_update_body(kdata, old_kdata, link); }
static int bpf_scx_validate(void *kdata)
{ return lupos_scx_st_validate_body(kdata); }

/* These ORIGINAL CFI stubs are metadata targets ONLY. They are not runtime
 * owner bodies, successful fallbacks, or evidence of algorithm coverage. */
/* BEGIN EXACT METADATA ext.c:8215-8383 */
static s32 sched_ext_ops__select_cpu(struct task_struct *p, s32 prev_cpu, u64 wake_flags) { return -EINVAL; }
static void sched_ext_ops__enqueue(struct task_struct *p, u64 enq_flags) {}
static void sched_ext_ops__dequeue(struct task_struct *p, u64 enq_flags) {}
static void sched_ext_ops__dispatch(s32 prev_cpu, struct task_struct *prev__nullable) {}
static void sched_ext_ops__tick(struct task_struct *p) {}
static void sched_ext_ops__runnable(struct task_struct *p, u64 enq_flags) {}
static void sched_ext_ops__running(struct task_struct *p) {}
static void sched_ext_ops__stopping(struct task_struct *p, bool runnable) {}
static void sched_ext_ops__quiescent(struct task_struct *p, u64 deq_flags) {}
static bool sched_ext_ops__yield(struct task_struct *from, struct task_struct *to__nullable) { return false; }
static bool sched_ext_ops__core_sched_before(struct task_struct *a, struct task_struct *b) { return false; }
static void sched_ext_ops__set_weight(struct task_struct *p, u32 weight) {}
static void sched_ext_ops__set_cpumask(struct task_struct *p, const struct cpumask *mask) {}
static void sched_ext_ops__update_idle(s32 cpu, bool idle) {}
static void sched_ext_ops__cpu_acquire(s32 cpu, struct scx_cpu_acquire_args *args) {}
static void sched_ext_ops__cpu_release(s32 cpu, struct scx_cpu_release_args *args) {}
static s32 sched_ext_ops__init_task(struct task_struct *p, struct scx_init_task_args *args) { return -EINVAL; }
static void sched_ext_ops__exit_task(struct task_struct *p, struct scx_exit_task_args *args) {}
static void sched_ext_ops__enable(struct task_struct *p) {}
static void sched_ext_ops__disable(struct task_struct *p) {}
#ifdef CONFIG_EXT_GROUP_SCHED
static s32 sched_ext_ops__cgroup_init(struct cgroup *cgrp, struct scx_cgroup_init_args *args) { return -EINVAL; }
static void sched_ext_ops__cgroup_exit(struct cgroup *cgrp) {}
static s32 sched_ext_ops__cgroup_prep_move(struct task_struct *p, struct cgroup *from, struct cgroup *to) { return -EINVAL; }
static void sched_ext_ops__cgroup_move(struct task_struct *p, struct cgroup *from, struct cgroup *to) {}
static void sched_ext_ops__cgroup_cancel_move(struct task_struct *p, struct cgroup *from, struct cgroup *to) {}
static void sched_ext_ops__cgroup_set_weight(struct cgroup *cgrp, u32 weight) {}
static void sched_ext_ops__cgroup_set_bandwidth(struct cgroup *cgrp, u64 period_us, u64 quota_us, u64 burst_us) {}
static void sched_ext_ops__cgroup_set_idle(struct cgroup *cgrp, bool idle) {}
#endif	/* CONFIG_EXT_GROUP_SCHED */
static s32 sched_ext_ops__sub_attach(struct scx_sub_attach_args *args) { return -EINVAL; }
static void sched_ext_ops__sub_detach(struct scx_sub_detach_args *args) {}
static void sched_ext_ops__cpu_online(s32 cpu) {}
static void sched_ext_ops__cpu_offline(s32 cpu) {}
static s32 sched_ext_ops__init_cids(void) { return -EINVAL; }
static s32 sched_ext_ops__init(void) { return -EINVAL; }
static void sched_ext_ops__exit(struct scx_exit_info *info) {}
static void sched_ext_ops__dump(struct scx_dump_ctx *ctx) {}
static void sched_ext_ops__dump_cpu(struct scx_dump_ctx *ctx, s32 cpu, bool idle) {}
static void sched_ext_ops__dump_task(struct scx_dump_ctx *ctx, struct task_struct *p) {}

static struct sched_ext_ops __bpf_ops_sched_ext_ops = {
	.select_cpu		= sched_ext_ops__select_cpu,
	.enqueue		= sched_ext_ops__enqueue,
	.dequeue		= sched_ext_ops__dequeue,
	.dispatch		= sched_ext_ops__dispatch,
	.tick			= sched_ext_ops__tick,
	.runnable		= sched_ext_ops__runnable,
	.running		= sched_ext_ops__running,
	.stopping		= sched_ext_ops__stopping,
	.quiescent		= sched_ext_ops__quiescent,
	.yield			= sched_ext_ops__yield,
	.core_sched_before	= sched_ext_ops__core_sched_before,
	.set_weight		= sched_ext_ops__set_weight,
	.set_cpumask		= sched_ext_ops__set_cpumask,
	.update_idle		= sched_ext_ops__update_idle,
	.cpu_acquire		= sched_ext_ops__cpu_acquire,
	.cpu_release		= sched_ext_ops__cpu_release,
	.init_task		= sched_ext_ops__init_task,
	.exit_task		= sched_ext_ops__exit_task,
	.enable			= sched_ext_ops__enable,
	.disable		= sched_ext_ops__disable,
#ifdef CONFIG_EXT_GROUP_SCHED
	.cgroup_init		= sched_ext_ops__cgroup_init,
	.cgroup_exit		= sched_ext_ops__cgroup_exit,
	.cgroup_prep_move	= sched_ext_ops__cgroup_prep_move,
	.cgroup_move		= sched_ext_ops__cgroup_move,
	.cgroup_cancel_move	= sched_ext_ops__cgroup_cancel_move,
	.cgroup_set_weight	= sched_ext_ops__cgroup_set_weight,
	.cgroup_set_bandwidth	= sched_ext_ops__cgroup_set_bandwidth,
	.cgroup_set_idle	= sched_ext_ops__cgroup_set_idle,
#endif
	.sub_attach		= sched_ext_ops__sub_attach,
	.sub_detach		= sched_ext_ops__sub_detach,
	.cpu_online		= sched_ext_ops__cpu_online,
	.cpu_offline		= sched_ext_ops__cpu_offline,
	.init_cids		= sched_ext_ops__init_cids,
	.init			= sched_ext_ops__init,
	.exit			= sched_ext_ops__exit,
	.dump			= sched_ext_ops__dump,
	.dump_cpu		= sched_ext_ops__dump_cpu,
	.dump_task		= sched_ext_ops__dump_task,
};

static struct bpf_struct_ops bpf_sched_ext_ops = {
	.verifier_ops = &bpf_scx_verifier_ops,
	.reg = bpf_scx_reg,
	.unreg = bpf_scx_unreg,
	.check_member = bpf_scx_check_member,
	.init_member = bpf_scx_init_member,
	.init = bpf_scx_init,
	.update = bpf_scx_update,
	.validate = bpf_scx_validate,
	.name = "sched_ext_ops",
	.owner = THIS_MODULE,
	.cfi_stubs = &__bpf_ops_sched_ext_ops
};

/*
 * cid-form cfi stubs. Stubs whose signatures match the cpu-form (param types
 * identical, only param names differ across structs) are reused. Some need
 * fresh stubs, set_cmask due to an argument type difference and the sub-sched
 * notifiers because no cpu-form stub exists to reuse.
 */
static void sched_ext_ops_cid__set_cmask(struct task_struct *p, const struct scx_cmask *cmask__arena) {}
static void sched_ext_ops__sub_caps_updated(const struct scx_cmask *cmask__arena, u64 caps) {}
static void sched_ext_ops__sub_ecaps_updated(s32 cid, u64 before, u64 after) {}

static struct sched_ext_ops_cid __bpf_ops_sched_ext_ops_cid = {
	.select_cid		= sched_ext_ops__select_cpu,
	.enqueue		= sched_ext_ops__enqueue,
	.dequeue		= sched_ext_ops__dequeue,
	.dispatch		= sched_ext_ops__dispatch,
	.tick			= sched_ext_ops__tick,
	.runnable		= sched_ext_ops__runnable,
	.running		= sched_ext_ops__running,
	.stopping		= sched_ext_ops__stopping,
	.quiescent		= sched_ext_ops__quiescent,
	.yield			= sched_ext_ops__yield,
	.core_sched_before	= sched_ext_ops__core_sched_before,
	.set_weight		= sched_ext_ops__set_weight,
	.set_cmask		= sched_ext_ops_cid__set_cmask,
	.update_idle		= sched_ext_ops__update_idle,
	.init_task		= sched_ext_ops__init_task,
	.exit_task		= sched_ext_ops__exit_task,
	.enable			= sched_ext_ops__enable,
	.disable		= sched_ext_ops__disable,
#ifdef CONFIG_EXT_GROUP_SCHED
	.cpuctl_init		= sched_ext_ops__cgroup_init,
	.cpuctl_exit		= sched_ext_ops__cgroup_exit,
	.cpuctl_prep_move	= sched_ext_ops__cgroup_prep_move,
	.cpuctl_move		= sched_ext_ops__cgroup_move,
	.cpuctl_cancel_move	= sched_ext_ops__cgroup_cancel_move,
	.cpuctl_set_weight	= sched_ext_ops__cgroup_set_weight,
	.cpuctl_set_bandwidth	= sched_ext_ops__cgroup_set_bandwidth,
	.cpuctl_set_idle	= sched_ext_ops__cgroup_set_idle,
#endif
	.sub_attach		= sched_ext_ops__sub_attach,
	.sub_detach		= sched_ext_ops__sub_detach,
	.sub_caps_updated	= sched_ext_ops__sub_caps_updated,
	.sub_ecaps_updated	= sched_ext_ops__sub_ecaps_updated,
	.cid_online		= sched_ext_ops__cpu_online,
	.cid_offline		= sched_ext_ops__cpu_offline,
	.init_cids		= sched_ext_ops__init_cids,
	.init			= sched_ext_ops__init,
	.exit			= sched_ext_ops__exit,
	.dump			= sched_ext_ops__dump,
	.dump_cid		= sched_ext_ops__dump_cpu,
	.dump_task		= sched_ext_ops__dump_task,
};

/*
 * The cid-form struct_ops shares all bpf_struct_ops hooks with the cpu form.
 * init_member, check_member, reg, unreg, etc. process kdata as the byte block
 * verified to match by the BUILD_BUG_ON checks in scx_init().
 */
static struct bpf_struct_ops bpf_sched_ext_ops_cid = {
	.verifier_ops = &bpf_scx_cid_verifier_ops,
	.reg = bpf_scx_reg_cid,
	.unreg = bpf_scx_unreg,
	.check_member = bpf_scx_check_member,
	.init_member = bpf_scx_init_member,
	.init = bpf_scx_init,
	.update = bpf_scx_update,
	.validate = bpf_scx_validate,
	.name = "sched_ext_ops_cid",
	.owner = THIS_MODULE,
	.cfi_stubs = &__bpf_ops_sched_ext_ops_cid
};
/* END EXACT METADATA */

/* Native primitives retain exact configured type, member and macro authority.
 * CPU/CID common-byte processing matches the original kdata interpretation and
 * is conditional on F17's real offset/tail checks; no Rust lookalike cast. */
bool lupos_scx_st_ctx_access(int off, int size, enum bpf_access_type type,
	const struct bpf_prog *prog, struct bpf_insn_access_aux *info)
{ return btf_ctx_access(off, size, type, prog, info); }
const struct btf_type *lupos_scx_st_reg_type(const struct bpf_reg_state *reg)
{ return btf_type_by_id(reg->btf, reg->btf_id); }
const struct btf_type *lupos_scx_st_task_type(void) { return task_struct_type; }
const struct btf_type *lupos_scx_st_tracing_task_type(struct btf *btf)
{ return btf_type_by_id(btf, btf_tracing_ids[BTF_TRACING_TYPE_TASK]); }
void lupos_scx_st_set_task_type(const struct btf_type *t) { task_struct_type = t; }
u32 lupos_scx_st_member_bit_offset(const struct btf_type *t, const struct btf_member *member)
{ return __btf_member_bit_offset(t, member); }
u32 lupos_scx_st_udata_u32(const void *udata, u32 moff) { return *(u32 *)(udata + moff); }
u64 lupos_scx_st_udata_u64(const void *udata, u32 moff) { return *(u64 *)(udata + moff); }
void lupos_scx_st_set_dispatch_max_batch(void *kdata, u32 value)
{ struct sched_ext_ops *ops = kdata; ops->dispatch_max_batch = value; }
void lupos_scx_st_set_flags(void *kdata, u64 value)
{ struct sched_ext_ops *ops = kdata; ops->flags = value; }
int lupos_scx_st_copy_name(void *kdata, const void *udata)
{
	const struct sched_ext_ops *uops = udata;
	struct sched_ext_ops *ops = kdata;

	return bpf_obj_name_cpy(ops->name, uops->name, sizeof(ops->name));
}
unsigned long lupos_scx_st_msecs_to_jiffies(u32 msecs) { return msecs_to_jiffies(msecs); }
void lupos_scx_st_set_timeout_ms(void *kdata, u32 value)
{ struct sched_ext_ops *ops = kdata; ops->timeout_ms = value; }
void lupos_scx_st_set_exit_dump_len(void *kdata, u32 value)
{ struct sched_ext_ops *ops = kdata; ops->exit_dump_len = value; }
void lupos_scx_st_set_hotplug_seq(void *kdata, u64 value)
{ struct sched_ext_ops *ops = kdata; ops->hotplug_seq = value; }
void lupos_scx_st_set_cid_shard_size(void *kdata, u32 value)
{ struct sched_ext_ops *ops = kdata; ops->cid_shard_size = value; }
void lupos_scx_st_set_rescue_bandwidth_ppt(void *kdata, u32 value)
{ struct sched_ext_ops *ops = kdata; ops->rescue_bandwidth_ppt = value; }
void lupos_scx_st_set_rescue_quantum_us(void *kdata, u32 value)
{ struct sched_ext_ops *ops = kdata; ops->rescue_quantum_us = value; }
bool lupos_scx_st_prog_sleepable(const struct bpf_prog *prog) { return prog->sleepable; }
#ifdef CONFIG_EXT_SUB_SCHED
void lupos_scx_st_set_sub_cgroup_id(void *kdata, u64 value)
{ struct sched_ext_ops *ops = kdata; ops->sub_cgroup_id = value; }
void lupos_scx_st_request_dispatch_stack(const struct bpf_prog *prog)
{
	prog->aux->priv_stack_requested = true;
	prog->aux->recursion_detected = scx_pstack_recursion_on_dispatch;
}
void lupos_scx_st_request_caps_stack(const struct bpf_prog *prog)
{
	prog->aux->priv_stack_requested = true;
	prog->aux->recursion_detected = scx_pstack_recursion_on_caps_updated;
}
#endif
#if defined(CONFIG_MMU) && defined(CONFIG_64BIT)
struct bpf_map *lupos_scx_st_prog_arena(struct bpf_prog *prog) { return bpf_prog_arena(prog); }
#endif
void lupos_scx_st_for_each_prog(void *kdata, struct scx_arena_scan *scan)
{ bpf_struct_ops_for_each_prog(kdata, scx_arena_scan_prog, scan); }
void lupos_scx_st_error_multiple_arenas(void)
{ pr_err("sched_ext: cid-form scheduler uses multiple arena maps\n"); }
void lupos_scx_st_error_missing_arena(void)
{ pr_err("sched_ext: cid-form scheduler must use a BPF arena map\n"); }
void lupos_scx_st_map_inc(struct bpf_map *arena) { bpf_map_inc(arena); }
void lupos_scx_st_map_put(struct bpf_map *arena) { bpf_map_put(arena); }
void lupos_scx_st_cmd_set_arena(struct scx_enable_cmd *cmd, struct bpf_map *arena)
{ cmd->arena_map = arena; }
struct bpf_map *lupos_scx_st_cmd_arena(struct scx_enable_cmd *cmd) { return cmd->arena_map; }
int lupos_scx_st_enable(struct scx_enable_cmd *cmd, struct bpf_link *link)
{ return scx_enable(cmd, link); }
struct scx_sched *lupos_scx_st_ops_priv_protected(void *kdata)
{
	struct sched_ext_ops *ops = kdata;

	return rcu_dereference_protected(ops->priv, true);
}
void lupos_scx_st_ops_priv_clear(void *kdata)
{
	struct sched_ext_ops *ops = kdata;

	RCU_INIT_POINTER(ops->priv, NULL);
}
void lupos_scx_st_sched_kobject_put(struct scx_sched *sch) { kobject_put(&sch->kobj); }

/* BEGIN EXACT METADATA ext.c:8959-8970 */
BTF_KFUNCS_START(scx_kfunc_ids_enqueue_dispatch)
BTF_ID_FLAGS(func, scx_bpf_dsq_insert, KF_IMPLICIT_ARGS | KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_dsq_insert___v2, KF_IMPLICIT_ARGS | KF_RCU)
BTF_ID_FLAGS(func, __scx_bpf_dsq_insert_vtime, KF_IMPLICIT_ARGS | KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_dsq_insert_vtime, KF_RCU)
BTF_KFUNCS_END(scx_kfunc_ids_enqueue_dispatch)

static const struct btf_kfunc_id_set scx_kfunc_set_enqueue_dispatch = {
	.owner			= THIS_MODULE,
	.set			= &scx_kfunc_ids_enqueue_dispatch,
	.filter			= scx_kfunc_context_filter,
};
/* END EXACT METADATA */
/* BEGIN EXACT METADATA ext.c:9290-9309 */
BTF_KFUNCS_START(scx_kfunc_ids_dispatch)
BTF_ID_FLAGS(func, scx_bpf_dispatch_nr_slots, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_dispatch_cancel, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_dsq_move_to_local, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_dsq_move_to_local___v2, KF_IMPLICIT_ARGS)
/* scx_bpf_dsq_move*() also in scx_kfunc_ids_unlocked: callable from unlocked contexts */
BTF_ID_FLAGS(func, scx_bpf_dsq_move_set_slice, KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_dsq_move_set_vtime, KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_dsq_move, KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_dsq_move_vtime, KF_RCU)
#ifdef CONFIG_EXT_SUB_SCHED
BTF_ID_FLAGS(func, scx_bpf_sub_dispatch, KF_IMPLICIT_ARGS)
#endif
BTF_KFUNCS_END(scx_kfunc_ids_dispatch)

static const struct btf_kfunc_id_set scx_kfunc_set_dispatch = {
	.owner			= THIS_MODULE,
	.set			= &scx_kfunc_ids_dispatch,
	.filter			= scx_kfunc_context_filter,
};
/* END EXACT METADATA */
/* BEGIN EXACT METADATA ext.c:9339-9347 */
BTF_KFUNCS_START(scx_kfunc_ids_cpu_release)
BTF_ID_FLAGS(func, scx_bpf_reenqueue_local, KF_IMPLICIT_ARGS)
BTF_KFUNCS_END(scx_kfunc_ids_cpu_release)

static const struct btf_kfunc_id_set scx_kfunc_set_cpu_release = {
	.owner			= THIS_MODULE,
	.set			= &scx_kfunc_ids_cpu_release,
	.filter			= scx_kfunc_context_filter,
};
/* END EXACT METADATA */
/* BEGIN EXACT METADATA ext.c:9408-9425 */
BTF_KFUNCS_START(scx_kfunc_ids_unlocked)
BTF_ID_FLAGS(func, scx_bpf_create_dsq, KF_IMPLICIT_ARGS | KF_SLEEPABLE)
/* also in scx_kfunc_ids_dispatch: also callable from ops.dispatch() */
BTF_ID_FLAGS(func, scx_bpf_dsq_move_set_slice, KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_dsq_move_set_vtime, KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_dsq_move, KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_dsq_move_vtime, KF_RCU)
/* also in scx_kfunc_ids_select_cpu: also callable from ops.select_cpu()/ops.enqueue() */
BTF_ID_FLAGS(func, __scx_bpf_select_cpu_and, KF_IMPLICIT_ARGS | KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_select_cpu_and, KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_select_cpu_dfl, KF_IMPLICIT_ARGS | KF_RCU)
BTF_KFUNCS_END(scx_kfunc_ids_unlocked)

static const struct btf_kfunc_id_set scx_kfunc_set_unlocked = {
	.owner			= THIS_MODULE,
	.set			= &scx_kfunc_ids_unlocked,
	.filter			= scx_kfunc_context_filter,
};
/* END EXACT METADATA */
/* BEGIN EXACT METADATA ext.c:10660-10760 */
BTF_KFUNCS_START(scx_kfunc_ids_any)
BTF_ID_FLAGS(func, scx_bpf_task_set_slice, KF_IMPLICIT_ARGS | KF_RCU);
BTF_ID_FLAGS(func, scx_bpf_task_set_dsq_vtime, KF_IMPLICIT_ARGS | KF_RCU);
BTF_ID_FLAGS(func, scx_bpf_kick_cpu, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_kick_cid, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_dsq_nr_queued, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_destroy_dsq, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_dsq_peek, KF_IMPLICIT_ARGS | KF_RCU_PROTECTED | KF_RET_NULL)
BTF_ID_FLAGS(func, scx_bpf_dsq_reenq, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_reenqueue_local___v2, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, bpf_iter_scx_dsq_new, KF_IMPLICIT_ARGS | KF_ITER_NEW | KF_RCU_PROTECTED)
BTF_ID_FLAGS(func, bpf_iter_scx_dsq_next, KF_ITER_NEXT | KF_RET_NULL)
BTF_ID_FLAGS(func, bpf_iter_scx_dsq_destroy, KF_ITER_DESTROY)
BTF_ID_FLAGS(func, scx_bpf_exit_bstr, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_error_bstr, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_dump_bstr, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_cpuperf_cap, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_cpuperf_cur, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_cpuperf_set, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_cidperf_cap, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_cidperf_cur, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_cidperf_set, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_nr_node_ids)
BTF_ID_FLAGS(func, scx_bpf_nr_cpu_ids)
BTF_ID_FLAGS(func, scx_bpf_nr_cids)
BTF_ID_FLAGS(func, scx_bpf_nr_online_cids)
BTF_ID_FLAGS(func, scx_bpf_this_cid)
BTF_ID_FLAGS(func, scx_bpf_get_possible_cpumask, KF_ACQUIRE)
BTF_ID_FLAGS(func, scx_bpf_get_online_cpumask, KF_ACQUIRE)
BTF_ID_FLAGS(func, scx_bpf_put_cpumask, KF_RELEASE)
BTF_ID_FLAGS(func, scx_bpf_task_running, KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_task_cpu, KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_task_cid, KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_locked_rq, KF_IMPLICIT_ARGS | KF_RET_NULL)
BTF_ID_FLAGS(func, scx_bpf_cpu_curr, KF_IMPLICIT_ARGS | KF_RET_NULL | KF_RCU_PROTECTED)
BTF_ID_FLAGS(func, scx_bpf_cid_curr, KF_IMPLICIT_ARGS | KF_RET_NULL | KF_RCU_PROTECTED)
BTF_ID_FLAGS(func, scx_bpf_tid_to_task, KF_RET_NULL | KF_RCU_PROTECTED)
BTF_ID_FLAGS(func, scx_bpf_now)
BTF_ID_FLAGS(func, scx_bpf_events, KF_IMPLICIT_ARGS)
#ifdef CONFIG_CGROUP_SCHED
BTF_ID_FLAGS(func, scx_bpf_task_cgroup, KF_IMPLICIT_ARGS | KF_RCU | KF_ACQUIRE)
#endif
BTF_ID_FLAGS(func, scx_bpf_sub_grant, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_sub_revoke, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_sub_caps, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_sub_kill_bstr, KF_IMPLICIT_ARGS)
BTF_KFUNCS_END(scx_kfunc_ids_any)

static const struct btf_kfunc_id_set scx_kfunc_set_any = {
	.owner			= THIS_MODULE,
	.set			= &scx_kfunc_ids_any,
	.filter			= scx_kfunc_context_filter,
};

/*
 * cpu-form kfuncs that are forbidden from cid-form schedulers
 * (bpf_sched_ext_ops_cid). Programs targeting the cid struct_ops type must
 * use the cid-form alternative (cid/cmask kfuncs).
 *
 * Membership overlaps with scx_kfunc_ids_{any,idle,select_cpu}; the filter
 * tests this set independently and rejects matches before the per-op
 * allow-list check runs.
 *
 * pahole/resolve_btfids scans every BTF_ID_FLAGS() at build time and
 * intersects flags across duplicate entries, so each entry must carry the
 * same flags as the kfunc's primary declaration; otherwise the flags get
 * dropped globally.
 */
BTF_KFUNCS_START(scx_kfunc_ids_cpu_only)
BTF_ID_FLAGS(func, scx_bpf_kick_cpu, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_task_cpu, KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_cpu_curr, KF_IMPLICIT_ARGS | KF_RET_NULL | KF_RCU_PROTECTED)
BTF_ID_FLAGS(func, scx_bpf_cpu_node, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_cpuperf_cap, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_cpuperf_cur, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_cpuperf_set, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_get_possible_cpumask, KF_ACQUIRE)
BTF_ID_FLAGS(func, scx_bpf_get_online_cpumask, KF_ACQUIRE)
BTF_ID_FLAGS(func, scx_bpf_put_cpumask, KF_RELEASE)
BTF_ID_FLAGS(func, scx_bpf_select_cpu_dfl, KF_IMPLICIT_ARGS | KF_RCU)
BTF_ID_FLAGS(func, __scx_bpf_select_cpu_and, KF_IMPLICIT_ARGS | KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_select_cpu_and, KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_get_idle_cpumask, KF_IMPLICIT_ARGS | KF_ACQUIRE)
BTF_ID_FLAGS(func, scx_bpf_get_idle_cpumask_node, KF_IMPLICIT_ARGS | KF_ACQUIRE)
BTF_ID_FLAGS(func, scx_bpf_get_idle_smtmask, KF_IMPLICIT_ARGS | KF_ACQUIRE)
BTF_ID_FLAGS(func, scx_bpf_get_idle_smtmask_node, KF_IMPLICIT_ARGS | KF_ACQUIRE)
BTF_ID_FLAGS(func, scx_bpf_put_idle_cpumask, KF_RELEASE)
BTF_ID_FLAGS(func, scx_bpf_test_and_clear_cpu_idle, KF_IMPLICIT_ARGS)
BTF_ID_FLAGS(func, scx_bpf_pick_idle_cpu, KF_IMPLICIT_ARGS | KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_pick_idle_cpu_node, KF_IMPLICIT_ARGS | KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_pick_any_cpu, KF_IMPLICIT_ARGS | KF_RCU)
BTF_ID_FLAGS(func, scx_bpf_pick_any_cpu_node, KF_IMPLICIT_ARGS | KF_RCU)
BTF_KFUNCS_END(scx_kfunc_ids_cpu_only)

/*
 * Per-op kfunc allow flags. Each bit corresponds to a context-sensitive kfunc
 * group; an op may permit zero or more groups, with the union expressed in
 * scx_kf_allow_flags[]. The verifier-time filter (scx_kfunc_context_filter())
 * consults this table to decide whether a context-sensitive kfunc is callable
 * from a given SCX op.
 */
/* END EXACT METADATA; scx_kf_allow_flags enum retained in native binding header. */
/* BEGIN EXACT METADATA ext.c:10770-10800 */
/*
 * Map each SCX op to the union of kfunc groups it permits, indexed by
 * SCX_OP_IDX(op). Ops not listed only permit kfuncs that are not
 * context-sensitive.
 */
static const u32 scx_kf_allow_flags[] = {
	[SCX_OP_IDX(select_cpu)]	= SCX_KF_ALLOW_SELECT_CPU | SCX_KF_ALLOW_ENQUEUE,
	[SCX_OP_IDX(enqueue)]		= SCX_KF_ALLOW_SELECT_CPU | SCX_KF_ALLOW_ENQUEUE,
	[SCX_OP_IDX(dispatch)]		= SCX_KF_ALLOW_ENQUEUE | SCX_KF_ALLOW_DISPATCH,
	[SCX_OP_IDX(cpu_release)]	= SCX_KF_ALLOW_CPU_RELEASE,
	[SCX_OP_IDX(init_task)]		= SCX_KF_ALLOW_UNLOCKED,
	[SCX_OP_IDX(dump)]		= SCX_KF_ALLOW_UNLOCKED,
#ifdef CONFIG_EXT_GROUP_SCHED
	[SCX_OP_IDX(cgroup_init)]	= SCX_KF_ALLOW_UNLOCKED,
	[SCX_OP_IDX(cgroup_exit)]	= SCX_KF_ALLOW_UNLOCKED,
	[SCX_OP_IDX(cgroup_prep_move)]	= SCX_KF_ALLOW_UNLOCKED,
	[SCX_OP_IDX(cgroup_cancel_move)] = SCX_KF_ALLOW_UNLOCKED,
	[SCX_OP_IDX(cgroup_set_weight)]	= SCX_KF_ALLOW_UNLOCKED,
	[SCX_OP_IDX(cgroup_set_bandwidth)] = SCX_KF_ALLOW_UNLOCKED,
	[SCX_OP_IDX(cgroup_set_idle)]	= SCX_KF_ALLOW_UNLOCKED,
#endif	/* CONFIG_EXT_GROUP_SCHED */
	[SCX_OP_IDX(sub_attach)]	= SCX_KF_ALLOW_UNLOCKED,
	[SCX_OP_IDX(sub_detach)]	= SCX_KF_ALLOW_UNLOCKED,
	[SCX_OP_IDX(sub_ecaps_updated)]	= SCX_KF_ALLOW_ENQUEUE | SCX_KF_ALLOW_DISPATCH,
	[SCX_OP_IDX(cpu_online)]	= SCX_KF_ALLOW_UNLOCKED,
	[SCX_OP_IDX(cpu_offline)]	= SCX_KF_ALLOW_UNLOCKED,
	[SCX_OP_IDX(init_cids)]		= SCX_KF_ALLOW_UNLOCKED | SCX_KF_ALLOW_INIT_CIDS,
	[SCX_OP_IDX(init)]		= SCX_KF_ALLOW_UNLOCKED,
	[SCX_OP_IDX(exit)]		= SCX_KF_ALLOW_UNLOCKED,
};

/* END EXACT METADATA */

/* All leaves below access the ORIGINAL native sets. cid.h and idle.h already
 * declare their owners' set identities; this file defines no duplicate set and
 * adds no extern workaround for the local BTF assembler symbols. */
bool lupos_scx_st_in_unlocked(u32 kfunc_id)
{ return btf_id_set8_contains(&scx_kfunc_ids_unlocked, kfunc_id); }
bool lupos_scx_st_in_init_cids(u32 kfunc_id)
{ return btf_id_set8_contains(&scx_kfunc_ids_init_cids, kfunc_id); }
bool lupos_scx_st_in_select_cpu(u32 kfunc_id)
{ return btf_id_set8_contains(&scx_kfunc_ids_select_cpu, kfunc_id); }
bool lupos_scx_st_in_enqueue(u32 kfunc_id)
{ return btf_id_set8_contains(&scx_kfunc_ids_enqueue_dispatch, kfunc_id); }
bool lupos_scx_st_in_dispatch(u32 kfunc_id)
{ return btf_id_set8_contains(&scx_kfunc_ids_dispatch, kfunc_id); }
bool lupos_scx_st_in_cpu_release(u32 kfunc_id)
{ return btf_id_set8_contains(&scx_kfunc_ids_cpu_release, kfunc_id); }
bool lupos_scx_st_in_idle(u32 kfunc_id)
{ return btf_id_set8_contains(&scx_kfunc_ids_idle, kfunc_id); }
bool lupos_scx_st_in_any(u32 kfunc_id)
{ return btf_id_set8_contains(&scx_kfunc_ids_any, kfunc_id); }
bool lupos_scx_st_in_cpu_only(u32 kfunc_id)
{ return btf_id_set8_contains(&scx_kfunc_ids_cpu_only, kfunc_id); }
bool lupos_scx_st_in_cid(u32 kfunc_id)
{ return btf_id_set8_contains(&scx_kfunc_ids_cid, kfunc_id); }
enum bpf_prog_type lupos_scx_st_prog_type(const struct bpf_prog *prog) { return prog->type; }
const struct bpf_struct_ops *lupos_scx_st_prog_ops(const struct bpf_prog *prog) { return prog->aux->st_ops; }
const struct bpf_struct_ops *lupos_scx_st_cpu_ops(void) { return &bpf_sched_ext_ops; }
const struct bpf_struct_ops *lupos_scx_st_cid_ops(void) { return &bpf_sched_ext_ops_cid; }
u32 lupos_scx_st_prog_member_off(const struct bpf_prog *prog) { return prog->aux->attach_st_ops_member_off; }
u32 lupos_scx_st_allow_flags(u32 moff) { return scx_kf_allow_flags[SCX_MOFF_IDX(moff)]; }

/* The exact native filter identity is shared by every owner set; BPF core
 * deduplicates by this callback address. Decisions reside in the Rust body. */
int scx_kfunc_context_filter(const struct bpf_prog *prog, u32 kfunc_id)
{ return lupos_scx_st_context_filter_body(prog, kfunc_id); }
