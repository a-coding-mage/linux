/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_SUB_BINDINGS_H
#define LUPOS_SCHED_EXT_SUB_BINDINGS_H
/* Deferred binding input only. Native configured headers are authoritative. */
#include <linux/btf.h>
#include <linux/btf_ids.h>
#include <linux/init.h>
#include <linux/rhashtable.h>
#include <linux/seq_buf.h>
#include <linux/slab.h>
#include "internal.h"
#include "cid.h"
#include "arena.h"
#include "sub.h"
#include "inlines.h"
#include "sched_ext_shared_access.h"
#define LUPOS_SCX_SUB_ENOMEM ENOMEM
#define LUPOS_SCX_SUB_SHARD_MAX_CPUS SCX_CID_SHARD_MAX_CPUS
#ifdef CONFIG_EXT_SUB_SCHED
#include "sched_ext_sub_rescue_bindings.h"
#include "sched_ext_sub_ecaps_bindings.h"
#include "sched_ext_sub_caps_bindings.h"
#include "sched_ext_sub_branch_bindings.h"
#include "sched_ext_sub_lifecycle_bindings.h"
void lupos_scx_sub_cmask_init(struct scx_cmask *m, u32 base, u32 nr);
void lupos_scx_sub_cmask_init_capacity(struct scx_cmask *m, u32 base, u32 nr, u32 capacity);
void lupos_scx_sub_assert_skip_tree(void);
void lupos_scx_sub_assert_next_tree(void);
struct scx_sched *lupos_scx_sub_parent(struct scx_sched *sch);
struct scx_sched *lupos_scx_sub_next_sibling(struct scx_sched *pos, struct scx_sched *parent);
struct scx_sched *lupos_scx_sub_first_child(struct scx_sched *pos);
struct scx_sched *lupos_scx_sub_find(u64 id);
void lupos_scx_sub_set_task_sched(struct task_struct *p, struct scx_sched *sch);
struct cgroup_subsys_state *lupos_scx_sub_cgroup_css(struct cgroup *cgrp);
struct cgroup_subsys_state *lupos_scx_sub_css_next(struct cgroup_subsys_state *pos, struct cgroup_subsys_state *root);
struct cgroup *lupos_scx_sub_live_cgroup(struct cgroup_subsys_state *css);
void lupos_scx_sub_set_cgroup_sched(struct cgroup *cgrp, struct scx_sched *sch);
void lupos_scx_sub_kfree(const void *p);
size_t lupos_scx_sub_cmask_size(u32 nr);
const struct scx_cid_shard *lupos_scx_sub_shard_range(s32 index);
s32 *lupos_scx_sub_shard_nodes(void);
u32 lupos_scx_sub_nr_shards(void);
struct scx_pshard *lupos_scx_sub_alloc_pshard(s32 node);
struct scx_pshard **lupos_scx_sub_alloc_pshard_array(u32 count);
void lupos_scx_sub_init_pshard_lock(struct scx_pshard *ps);
void lupos_scx_sub_init_updated(struct scx_caps_updated *cu);
struct scx_cmask *lupos_scx_sub_cap_cmask(struct scx_pshard *ps, u32 bit);
struct scx_cmask *lupos_scx_sub_updated_cmask(struct scx_caps_updated *cu);
void lupos_scx_sub_publish_pshards(struct scx_sched *sch, struct scx_pshard **ps);
#endif
#endif
