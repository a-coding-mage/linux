// SPDX-License-Identifier: GPL-2.0
/* Explicit unqualified runtime C primitive/storage boundary. No sub.c fallback. */
#error "SOURCE ONLY HOLD: sched_ext sub native ABI/protection admission is pending"
#include "sched_ext_sub_bindings.h"
#ifdef CONFIG_EXT_SUB_SCHED
/* Only this sub-owner static key is defined here. Shared ext state stays external. */
DEFINE_STATIC_KEY_FALSE(__scx_has_subs);
void lupos_scx_sub_cmask_init(struct scx_cmask *m, u32 base, u32 nr) { scx_cmask_init(m, base, nr); }
void lupos_scx_sub_cmask_init_capacity(struct scx_cmask *m, u32 base, u32 nr, u32 capacity) { __scx_cmask_init(m, base, nr, capacity); }

/* Keep the two original traversal assertion sites separate. */
void lupos_scx_sub_assert_skip_tree(void)
{
	lockdep_assert(lockdep_is_held(&scx_enable_mutex) ||
		       lockdep_is_held(&scx_sched_lock) || rcu_read_lock_any_held());
}
void lupos_scx_sub_assert_next_tree(void)
{
	lockdep_assert(lockdep_is_held(&scx_enable_mutex) ||
		       lockdep_is_held(&scx_sched_lock) || rcu_read_lock_any_held());
}
struct scx_sched *lupos_scx_sub_parent(struct scx_sched *sch)
{
	return scx_parent(sch);
}
struct scx_sched *lupos_scx_sub_next_sibling(struct scx_sched *pos, struct scx_sched *parent)
{
	return list_next_or_null_rcu(&parent->children, &pos->sibling, struct scx_sched, sibling);
}
struct scx_sched *lupos_scx_sub_first_child(struct scx_sched *pos)
{
	return list_first_or_null_rcu(&pos->children, struct scx_sched, sibling);
}
struct scx_sched *lupos_scx_sub_find(u64 id)
{
	return rhashtable_lookup(&scx_sched_hash, &id, scx_sched_hash_params);
}
void lupos_scx_sub_set_task_sched(struct task_struct *p, struct scx_sched *sch)
{
	rcu_assign_pointer(p->scx.sched, sch);
}
struct cgroup_subsys_state *lupos_scx_sub_cgroup_css(struct cgroup *cgrp)
{
	return cgroup_css(cgrp, NULL);
}
struct cgroup_subsys_state *lupos_scx_sub_css_next(struct cgroup_subsys_state *pos, struct cgroup_subsys_state *root)
{
	return css_next_descendant_pre(pos, root);
}
struct cgroup *lupos_scx_sub_live_cgroup(struct cgroup_subsys_state *css)
{
	lockdep_assert_held(&cgroup_mutex);
	return cgroup_is_dead(css->cgroup) ? NULL : css->cgroup;
}
void lupos_scx_sub_set_cgroup_sched(struct cgroup *cgrp, struct scx_sched *sch)
{
	rcu_assign_pointer(cgrp->scx_sched, sch);
}
void lupos_scx_sub_kfree(const void *p)
{
	kfree(p);
}
size_t lupos_scx_sub_cmask_size(u32 nr)
{
	return struct_size_t(struct scx_cmask, bits, SCX_CMASK_NR_WORDS(nr));
}
const struct scx_cid_shard *lupos_scx_sub_shard_range(s32 index)
{
	return &rcu_dereference_protected(scx_cid_shard_ranges,
					lockdep_is_held(&scx_enable_mutex))[index];
}
s32 *lupos_scx_sub_shard_nodes(void)
{
	return rcu_dereference_protected(scx_shard_node, lockdep_is_held(&scx_enable_mutex));
}
u32 lupos_scx_sub_nr_shards(void)
{
	return scx_nr_cid_shards;
}
struct scx_pshard *lupos_scx_sub_alloc_pshard(s32 node)
{
	return kzalloc_node(sizeof(struct scx_pshard), GFP_KERNEL, node);
}
struct scx_pshard **lupos_scx_sub_alloc_pshard_array(u32 count)
{
	return kzalloc_objs(struct scx_pshard *, count, GFP_KERNEL);
}
void lupos_scx_sub_init_pshard_lock(struct scx_pshard *ps)
{
	raw_spin_lock_init(&ps->lock);
}
void lupos_scx_sub_init_updated(struct scx_caps_updated *cu)
{
	raw_spin_lock_init(&cu->lock);
	INIT_LIST_HEAD(&cu->node_in_flight);
}
struct scx_cmask *lupos_scx_sub_cap_cmask(struct scx_pshard *ps, u32 bit)
{
	return &ps->caps[bit].cmask;
}
struct scx_cmask *lupos_scx_sub_updated_cmask(struct scx_caps_updated *cu)
{
	return &cu->cmask;
}
void lupos_scx_sub_publish_pshards(struct scx_sched *sch, struct scx_pshard **ps)
{
	smp_wmb();
	WRITE_ONCE(sch->pshard, ps);
}
#endif
