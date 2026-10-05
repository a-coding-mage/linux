/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_STRUCTOPS_BTF_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_STRUCTOPS_BTF_BINDINGS_H

/* F13 uses the single configured native type universe. No hand-written Rust
 * struct/union/bitfield layout and no independently linked native object. */
#include <linux/bpf_verifier.h>
#include <linux/bpf.h>
#include <linux/btf.h>
#include "internal.h"

/* Exact native local layout from ext.c:8127-8130. Stack-owned by the native
 * registration shell; the Rust scan callback borrows it synchronously. */
struct scx_arena_scan {
	struct bpf_map	*arena;
	int		err;
};

/* Values are native expressions, not copied layout numbers. */
enum lupos_scx_structops_native_consts {
	LUPOS_SCX_ST_CTX_BYTES = sizeof(__u64) * MAX_BPF_FUNC_ARGS,
	LUPOS_SCX_ST_DISALLOW = offsetof(struct task_struct, scx.disallow),
	LUPOS_SCX_ST_DISALLOW_END = offsetofend(struct task_struct, scx.disallow),
	LUPOS_SCX_ST_SLICE = offsetof(struct task_struct, scx.slice),
	LUPOS_SCX_ST_SLICE_END = offsetofend(struct task_struct, scx.slice),
	LUPOS_SCX_ST_VTIME = offsetof(struct task_struct, scx.dsq_vtime),
	LUPOS_SCX_ST_VTIME_END = offsetofend(struct task_struct, scx.dsq_vtime),
	LUPOS_SCX_ST_DISPATCH_MAX_BATCH = offsetof(struct sched_ext_ops, dispatch_max_batch),
	LUPOS_SCX_ST_FLAGS = offsetof(struct sched_ext_ops, flags),
	LUPOS_SCX_ST_NAME = offsetof(struct sched_ext_ops, name),
	LUPOS_SCX_ST_TIMEOUT_MS = offsetof(struct sched_ext_ops, timeout_ms),
	LUPOS_SCX_ST_EXIT_DUMP_LEN = offsetof(struct sched_ext_ops, exit_dump_len),
	LUPOS_SCX_ST_HOTPLUG_SEQ = offsetof(struct sched_ext_ops, hotplug_seq),
	LUPOS_SCX_ST_CID_SHARD_SIZE = offsetof(struct sched_ext_ops, cid_shard_size),
	LUPOS_SCX_ST_RESCUE_BANDWIDTH_PPT = offsetof(struct sched_ext_ops, rescue_bandwidth_ppt),
	LUPOS_SCX_ST_RESCUE_QUANTUM_US = offsetof(struct sched_ext_ops, rescue_quantum_us),
	LUPOS_SCX_ST_INIT_TASK = offsetof(struct sched_ext_ops, init_task),
	LUPOS_SCX_ST_CPU_ONLINE = offsetof(struct sched_ext_ops, cpu_online),
	LUPOS_SCX_ST_CPU_OFFLINE = offsetof(struct sched_ext_ops, cpu_offline),
	LUPOS_SCX_ST_INIT_CIDS = offsetof(struct sched_ext_ops, init_cids),
	LUPOS_SCX_ST_INIT = offsetof(struct sched_ext_ops, init),
	LUPOS_SCX_ST_EXIT = offsetof(struct sched_ext_ops, exit),
	LUPOS_SCX_ST_SUB_ATTACH = offsetof(struct sched_ext_ops, sub_attach),
	LUPOS_SCX_ST_SUB_DETACH = offsetof(struct sched_ext_ops, sub_detach),
#ifdef CONFIG_EXT_GROUP_SCHED
	LUPOS_SCX_ST_CGROUP_INIT = offsetof(struct sched_ext_ops, cgroup_init),
	LUPOS_SCX_ST_CGROUP_EXIT = offsetof(struct sched_ext_ops, cgroup_exit),
	LUPOS_SCX_ST_CGROUP_PREP_MOVE = offsetof(struct sched_ext_ops, cgroup_prep_move),
#endif
#ifdef CONFIG_EXT_SUB_SCHED
	LUPOS_SCX_ST_SUB_CGROUP_ID = offsetof(struct sched_ext_ops, sub_cgroup_id),
	LUPOS_SCX_ST_DISPATCH = offsetof(struct sched_ext_ops, dispatch),
	LUPOS_SCX_ST_SUB_CAPS_UPDATED = offsetof(struct sched_ext_ops, sub_caps_updated),
#endif
};

/* Exact ext.c:10761-10768; the native indexed allow table stays in the TU. */
enum scx_kf_allow_flags {
	SCX_KF_ALLOW_UNLOCKED		= 1 << 0,
	SCX_KF_ALLOW_INIT_CIDS		= 1 << 1,
	SCX_KF_ALLOW_CPU_RELEASE	= 1 << 2,
	SCX_KF_ALLOW_DISPATCH		= 1 << 3,
	SCX_KF_ALLOW_ENQUEUE		= 1 << 4,
	SCX_KF_ALLOW_SELECT_CPU		= 1 << 5,
};

bool lupos_scx_st_valid_access_body(int off, int size, enum bpf_access_type type,
	const struct bpf_prog *prog, struct bpf_insn_access_aux *info);
int lupos_scx_st_struct_access_body(struct bpf_verifier_log *log,
	const struct bpf_reg_state *reg, int off, int size);
int lupos_scx_st_cid_struct_access_body(struct bpf_verifier_log *log,
	const struct bpf_reg_state *reg, int off, int size);
int lupos_scx_st_init_member_body(const struct btf_type *t,
	const struct btf_member *member, void *kdata, const void *udata);
int lupos_scx_st_check_member_body(const struct btf_type *t,
	const struct btf_member *member, const struct bpf_prog *prog);
int lupos_scx_st_reg_body(struct scx_enable_cmd *cmd, struct bpf_link *link);
int lupos_scx_st_scan_prog_body(struct bpf_prog *prog, struct scx_arena_scan *s);
int lupos_scx_st_reg_cid_body(void *kdata, struct bpf_link *link,
	struct scx_enable_cmd *cmd, struct scx_arena_scan *scan);
void lupos_scx_st_unreg_body(void *kdata, struct bpf_link *link);
int lupos_scx_st_init_body(struct btf *btf);
int lupos_scx_st_update_body(void *kdata, void *old_kdata, struct bpf_link *link);
int lupos_scx_st_validate_body(void *kdata);
int lupos_scx_st_context_filter_body(const struct bpf_prog *prog, u32 kfunc_id);

bool lupos_scx_st_ctx_access(int off, int size, enum bpf_access_type type,
	const struct bpf_prog *prog, struct bpf_insn_access_aux *info);
const struct btf_type *lupos_scx_st_reg_type(const struct bpf_reg_state *reg);
const struct btf_type *lupos_scx_st_task_type(void);
const struct btf_type *lupos_scx_st_tracing_task_type(struct btf *btf);
void lupos_scx_st_set_task_type(const struct btf_type *t);
u32 lupos_scx_st_member_bit_offset(const struct btf_type *t, const struct btf_member *member);
u32 lupos_scx_st_udata_u32(const void *udata, u32 moff);
u64 lupos_scx_st_udata_u64(const void *udata, u32 moff);
void lupos_scx_st_set_dispatch_max_batch(void *kdata, u32 value);
void lupos_scx_st_set_flags(void *kdata, u64 value);
int lupos_scx_st_copy_name(void *kdata, const void *udata);
unsigned long lupos_scx_st_msecs_to_jiffies(u32 msecs);
void lupos_scx_st_set_timeout_ms(void *kdata, u32 value);
void lupos_scx_st_set_exit_dump_len(void *kdata, u32 value);
void lupos_scx_st_set_hotplug_seq(void *kdata, u64 value);
void lupos_scx_st_set_cid_shard_size(void *kdata, u32 value);
void lupos_scx_st_set_rescue_bandwidth_ppt(void *kdata, u32 value);
void lupos_scx_st_set_rescue_quantum_us(void *kdata, u32 value);
bool lupos_scx_st_prog_sleepable(const struct bpf_prog *prog);
#ifdef CONFIG_EXT_SUB_SCHED
void lupos_scx_st_set_sub_cgroup_id(void *kdata, u64 value);
void lupos_scx_st_request_dispatch_stack(const struct bpf_prog *prog);
void lupos_scx_st_request_caps_stack(const struct bpf_prog *prog);
#endif
#if defined(CONFIG_MMU) && defined(CONFIG_64BIT)
struct bpf_map *lupos_scx_st_prog_arena(struct bpf_prog *prog);
#endif
void lupos_scx_st_for_each_prog(void *kdata, struct scx_arena_scan *scan);
void lupos_scx_st_error_multiple_arenas(void);
void lupos_scx_st_error_missing_arena(void);
void lupos_scx_st_map_inc(struct bpf_map *arena);
void lupos_scx_st_map_put(struct bpf_map *arena);
void lupos_scx_st_cmd_set_arena(struct scx_enable_cmd *cmd, struct bpf_map *arena);
struct bpf_map *lupos_scx_st_cmd_arena(struct scx_enable_cmd *cmd);
int lupos_scx_st_enable(struct scx_enable_cmd *cmd, struct bpf_link *link);
struct scx_sched *lupos_scx_st_ops_priv_protected(void *kdata);
void lupos_scx_st_ops_priv_clear(void *kdata);
void lupos_scx_st_sched_kobject_put(struct scx_sched *sch);
bool lupos_scx_st_in_unlocked(u32 kfunc_id);
bool lupos_scx_st_in_init_cids(u32 kfunc_id);
bool lupos_scx_st_in_select_cpu(u32 kfunc_id);
bool lupos_scx_st_in_enqueue(u32 kfunc_id);
bool lupos_scx_st_in_dispatch(u32 kfunc_id);
bool lupos_scx_st_in_cpu_release(u32 kfunc_id);
bool lupos_scx_st_in_idle(u32 kfunc_id);
bool lupos_scx_st_in_any(u32 kfunc_id);
bool lupos_scx_st_in_cpu_only(u32 kfunc_id);
bool lupos_scx_st_in_cid(u32 kfunc_id);
enum bpf_prog_type lupos_scx_st_prog_type(const struct bpf_prog *prog);
const struct bpf_struct_ops *lupos_scx_st_prog_ops(const struct bpf_prog *prog);
const struct bpf_struct_ops *lupos_scx_st_cpu_ops(void);
const struct bpf_struct_ops *lupos_scx_st_cid_ops(void);
u32 lupos_scx_st_prog_member_off(const struct bpf_prog *prog);
u32 lupos_scx_st_allow_flags(u32 moff);

#endif /* LUPOS_SCHED_EXT_CORE_STRUCTOPS_BTF_BINDINGS_H */
