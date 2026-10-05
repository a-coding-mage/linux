/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_OBJECT_LIFETIME_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_OBJECT_LIFETIME_BINDINGS_H
/* F09: configured native headers are the only type/layout/value authority.
 * Included in the single F00 native envelope, never a separate C object.
 */
#include "internal.h"
#define LUPOS_SCX_CORE_OBJECT_DSQ_INVALID SCX_DSQ_INVALID
#define LUPOS_SCX_CORE_OBJECT_DSQ_GLOBAL SCX_DSQ_GLOBAL
#define LUPOS_SCX_CORE_OBJECT_DSQ_BYPASS SCX_DSQ_BYPASS
#define LUPOS_SCX_CORE_OBJECT_SLICE_DFL SCX_SLICE_DFL
void lupos_scx_core_object_zero_dsq(struct scx_dispatch_q *dsq);
void lupos_scx_core_object_init_dsq_lock(struct scx_dispatch_q *dsq);
void lupos_scx_core_object_init_list(struct list_head *head);
int lupos_scx_core_object_next_cpu(int cpu);
unsigned int lupos_scx_core_object_possible_cpu_limit(void);
int lupos_scx_core_object_next_node(int node);
void lupos_scx_core_object_alloc_dsq_pcpu(struct scx_dispatch_q *dsq);
struct scx_dsq_pcpu *lupos_scx_core_object_dsq_pcpu(struct scx_dispatch_q *dsq, int cpu);
struct scx_sched_pcpu *lupos_scx_core_object_sched_pcpu(struct scx_sched *sch, int cpu);
struct rq *lupos_scx_core_object_cpu_rq(int cpu);
bool lupos_scx_core_object_warn_deferred_user(struct scx_deferred_reenq_user *dru);
unsigned long lupos_scx_core_object_lock_irqsave(raw_spinlock_t *lock) __acquires(lock);
void lupos_scx_core_object_unlock_irqrestore(raw_spinlock_t *lock, unsigned long flags) __releases(lock);
void lupos_scx_core_object_deferred_unlink(struct rq *rq, struct scx_deferred_reenq_user *dru);
void lupos_scx_core_object_free_dsq_pcpu(struct scx_dispatch_q *dsq);
void lupos_scx_core_object_free(const void *p);
struct scx_dispatch_q *lupos_scx_core_object_dsq_from_rcu(struct rcu_head *rcu);
struct scx_dispatch_q *lupos_scx_core_object_dsq_from_free_node(struct llist_node *node);
struct llist_node *lupos_scx_core_object_take_free_dsqs(void);
void lupos_scx_core_object_call_dsq_rcu(struct scx_dispatch_q *dsq);
void lupos_scx_core_object_rcu_read_lock(void) __acquires_shared(RCU);
void lupos_scx_core_object_rcu_read_unlock(void) __releases_shared(RCU);
void lupos_scx_core_object_error_dsq_busy(struct scx_sched *sch, struct scx_dispatch_q *dsq);
int lupos_scx_core_object_remove_dsq(struct scx_sched *sch, struct scx_dispatch_q *dsq);
bool lupos_scx_core_object_add_free_dsq(struct scx_dispatch_q *dsq);
void lupos_scx_core_object_queue_free_dsq(void);
size_t lupos_scx_core_object_scratch_size(void);
void lupos_scx_core_object_alloc_scratch_slots(struct scx_sched *sch);
struct scx_cmask **lupos_scx_core_object_scratch_slot(struct scx_sched *sch, int cpu);
void lupos_scx_core_object_free_scratch_slots(struct scx_sched *sch);
void lupos_scx_core_object_cmask_init(struct scx_cmask *cm, u32 nr);
struct scx_sched *lupos_scx_core_object_kobj_sched(struct kobject *kobj);
const struct scx_sched *lupos_scx_core_object_const_kobj_sched(const struct kobject *kobj);
void lupos_scx_core_object_init_rcu_work(struct scx_sched *sch);
void lupos_scx_core_object_queue_rcu_work(struct scx_sched *sch);
ssize_t lupos_scx_core_object_emit_state(char *buf, const char *state);
ssize_t lupos_scx_core_object_emit_switch(char *buf, bool switching);
ssize_t lupos_scx_core_object_emit_long(char *buf, long value);
ssize_t lupos_scx_core_object_emit_ops(char *buf, const struct scx_sched *sch);
int lupos_scx_core_object_emit_events(char *buf, const struct scx_event_stats *events);
bool lupos_scx_core_object_is_sched_kobj(const struct kobject *kobj);
int lupos_scx_core_object_add_uevent(struct kobj_uevent_env *env, const struct scx_sched *sch);
const struct attribute_group *lupos_scx_core_object_global_attr_group(void);
const struct kset_uevent_ops *lupos_scx_core_object_uevent_ops(void);
#ifdef CONFIG_EXT_SUB_SCHED
struct scx_cmask *lupos_scx_core_object_alloc_agg(u32 nr);
unsigned long *lupos_scx_core_object_alloc_bitmap(u32 nr);
void lupos_scx_core_object_free_bitmap(unsigned long *bitmap);
ssize_t lupos_scx_core_object_with_caps_snap(struct scx_sched *sch, char *buf,
        struct scx_cmask *agg, unsigned long *bitmap, int cap, ssize_t at, u32 nr);
struct scx_cmask *lupos_scx_core_object_shard_cap(struct scx_sched *sch, int si, int cap);
void lupos_scx_core_object_cmask_reframe(struct scx_cmask *cm, u32 base, u32 nr);
void lupos_scx_core_object_cmask_copy(struct scx_cmask *dst, const struct scx_cmask *src);
void lupos_scx_core_object_cmask_or(struct scx_cmask *dst, const struct scx_cmask *src);
void lupos_scx_core_object_bitmap_from_cmask(unsigned long *bitmap, struct scx_cmask *cm, u32 nr);
ssize_t lupos_scx_core_object_emit_cap(char *buf, ssize_t at, int cap, u32 nr, unsigned long *bitmap);
ssize_t lupos_scx_core_object_caps_one_body(struct scx_sched *sch, char *buf,
        struct scx_cmask *agg, unsigned long *bitmap, int cap, ssize_t at, u32 nr,
        struct scx_cmask *snap);
ssize_t lupos_scx_core_object_caps_body(struct kobject *kobj, struct kobj_attribute *ka, char *buf);
#endif
/* Native callback adapters point at these Rust bodies; rcu_head is the native
 * macro spelling of callback_head and MUST NOT become a fabricated Rust type. */
void lupos_scx_core_object_free_dsq_rcu_body(struct rcu_head *rcu);
void lupos_scx_core_object_free_dsq_irq_body(struct irq_work *work);
void lupos_scx_core_object_sched_free_body(struct work_struct *work);
void lupos_scx_core_object_release_body(struct kobject *kobj);
ssize_t lupos_scx_core_object_state_body(struct kobject *kobj, struct kobj_attribute *ka, char *buf);
ssize_t lupos_scx_core_object_switch_all_body(struct kobject *kobj, struct kobj_attribute *ka, char *buf);
ssize_t lupos_scx_core_object_nr_rejected_body(struct kobject *kobj, struct kobj_attribute *ka, char *buf);
ssize_t lupos_scx_core_object_hotplug_seq_body(struct kobject *kobj, struct kobj_attribute *ka, char *buf);
ssize_t lupos_scx_core_object_enable_seq_body(struct kobject *kobj, struct kobj_attribute *ka, char *buf);
ssize_t lupos_scx_core_object_ops_body(struct kobject *kobj, struct kobj_attribute *ka, char *buf);
ssize_t lupos_scx_core_object_events_body(struct kobject *kobj, struct kobj_attribute *ka, char *buf);
int lupos_scx_core_object_uevent_body(const struct kobject *kobj, struct kobj_uevent_env *env);
/* Checkpoint 2 object allocation/destruction and hierarchy primitives. */
int lupos_scx_core_object_node_limit(void);
struct scx_sched *lupos_scx_core_object_work_sched(struct work_struct *work);
void lupos_scx_core_object_irq_sync(struct irq_work *work);
void lupos_scx_core_object_destroy_helper(struct scx_sched *sch);
void lupos_scx_core_object_shutdown_timer(struct scx_sched *sch);
void lupos_scx_core_object_free_donee(struct scx_sched *sch);
void lupos_scx_core_object_free_resched(struct scx_sched *sch);
void lupos_scx_core_object_free_stall(struct scx_sched *sch);
void lupos_scx_core_object_warn_deferred_local(struct scx_sched_pcpu *pcpu);
void lupos_scx_core_object_warn_kick_node(struct scx_sched_pcpu *pcpu);
void lupos_scx_core_object_discard_ecaps(int cpu, struct scx_sched_pcpu *pcpu);
void lupos_scx_core_object_free_kick(struct scx_sched_pcpu *pcpu);
void lupos_scx_core_object_free_kick_idle(struct scx_sched_pcpu *pcpu);
void lupos_scx_core_object_free_preempt(struct scx_sched_pcpu *pcpu);
void lupos_scx_core_object_free_wait(struct scx_sched_pcpu *pcpu);
void lupos_scx_core_object_free_sched_pcpu(struct scx_sched *sch);
struct scx_sched_pnode *lupos_scx_core_object_pnode(struct scx_sched *sch, int node);
void lupos_scx_core_object_set_pnode(struct scx_sched *sch, int node, struct scx_sched_pnode *pnode);
void lupos_scx_core_object_free_pshards(struct scx_sched *sch);
void lupos_scx_core_object_walk_enter(struct scx_sched *sch, struct rhashtable_iter *iter);
void lupos_scx_core_object_walk_start(struct rhashtable_iter *iter) __acquires_shared(RCU);
struct scx_dispatch_q *lupos_scx_core_object_walk_next(struct rhashtable_iter *iter);
void lupos_scx_core_object_walk_stop(struct rhashtable_iter *iter) __releases_shared(RCU);
void lupos_scx_core_object_walk_exit(struct rhashtable_iter *iter);
bool lupos_scx_core_object_is_err_or_null(const void *p);
bool lupos_scx_core_object_is_err(const void *p);
long lupos_scx_core_object_ptr_err(const void *p);
struct scx_sched *lupos_scx_core_object_err_sched(long err);
void lupos_scx_core_object_free_hash(struct scx_sched *sch);
void lupos_scx_core_object_put_map(struct bpf_map *map);
void lupos_scx_core_object_dec_has_subs(struct scx_sched *sch);
void lupos_scx_core_object_kvfree(const void *p);
struct scx_exit_info *lupos_scx_core_object_alloc_ei(void);
unsigned long *lupos_scx_core_object_alloc_bt(void);
char *lupos_scx_core_object_alloc_msg(void);
char *lupos_scx_core_object_alloc_dump(size_t size);
int lupos_scx_core_object_with_link_locks(struct scx_sched *sch);
int lupos_scx_core_object_link_locked_body(struct scx_sched *sch);
void lupos_scx_core_object_with_unlink_lock(struct scx_sched *sch);
void lupos_scx_core_object_unlink_locked_body(struct scx_sched *sch);
void lupos_scx_core_object_list_add_tail_rcu(struct list_head *node, struct list_head *head);
void lupos_scx_core_object_list_del_rcu(struct list_head *node);
void lupos_scx_core_object_mb(void);
struct scx_sched_pnode *lupos_scx_core_object_alloc_pnode(int node);
struct sched_ext_ops *lupos_scx_core_object_cmd_ops(struct scx_enable_cmd *cmd);
struct scx_sched *lupos_scx_core_object_alloc_sched(int level);
int lupos_scx_core_object_init_hash(struct scx_sched *sch);
void lupos_scx_core_object_alloc_pnodes(struct scx_sched *sch);
void lupos_scx_core_object_alloc_sched_pcpu(struct scx_sched *sch);
bool lupos_scx_core_object_alloc_kick(struct scx_sched_pcpu *pcpu, int node);
bool lupos_scx_core_object_alloc_kick_idle(struct scx_sched_pcpu *pcpu, int node);
bool lupos_scx_core_object_alloc_preempt(struct scx_sched_pcpu *pcpu, int node);
bool lupos_scx_core_object_alloc_wait(struct scx_sched_pcpu *pcpu, int node);
void lupos_scx_core_object_run_helper(struct scx_sched *sch);
void lupos_scx_core_object_helper_fifo(struct scx_sched *sch);
void lupos_scx_core_object_copy_ancestors(struct scx_sched *sch, struct scx_sched *parent, int level);
void lupos_scx_core_object_set_ancestor(struct scx_sched *sch, int level);
unsigned long lupos_scx_core_object_msecs_to_jiffies(unsigned int ms);
void lupos_scx_core_object_init_exit_kind(struct scx_sched *sch);
void lupos_scx_core_object_init_disable_irq(struct scx_sched *sch);
void lupos_scx_core_object_init_propagate_irq(struct scx_sched *sch);
void lupos_scx_core_object_init_disable_work(struct scx_sched *sch);
void lupos_scx_core_object_init_bypass_timer(struct scx_sched *sch);
bool lupos_scx_core_object_alloc_donee(struct scx_sched *sch);
bool lupos_scx_core_object_alloc_resched(struct scx_sched *sch);
bool lupos_scx_core_object_alloc_stall(struct scx_sched *sch);
void lupos_scx_core_object_copy_cpu_ops(struct scx_sched *sch, struct scx_enable_cmd *cmd);
void lupos_scx_core_object_copy_cid_ops(struct scx_sched *sch, struct scx_enable_cmd *cmd);
void lupos_scx_core_object_publish_priv(struct sched_ext_ops *ops, struct scx_sched *sch);
void lupos_scx_core_object_init_kobj(struct scx_sched *sch);
#if defined(CONFIG_MMU) && defined(CONFIG_64BIT)
void lupos_scx_core_object_arena_kern_base(struct scx_sched *sch);
#endif
int lupos_scx_core_object_add_root_kobj(struct scx_sched *sch);
#ifdef CONFIG_EXT_SUB_SCHED
void lupos_scx_core_object_cgroup_put(struct cgroup *cgrp);
struct cgroup *lupos_scx_core_object_sch_cgroup(struct scx_sched *sch);
void lupos_scx_core_object_kobject_put(struct kobject *kobj);
void lupos_scx_core_object_kobject_get(struct kobject *kobj);
s32 lupos_scx_core_object_bypass_depth(struct scx_sched *sch);
void lupos_scx_core_object_error_parent_bypass(struct scx_sched *sch);
int lupos_scx_core_object_insert_sched(struct scx_sched *sch);
void lupos_scx_core_object_error_insert_sched(struct scx_sched *sch, int ret);
void lupos_scx_core_object_remove_sched(struct scx_sched *sch);
bool lupos_scx_core_object_parent_aborting(struct scx_sched *parent);
void lupos_scx_core_object_error_parent_disabled(struct scx_sched *sch);
void lupos_scx_core_object_init_ecaps_node(struct scx_sched_pcpu *pcpu);
char *lupos_scx_core_object_alloc_path(void);
void lupos_scx_core_object_cgroup_path(struct cgroup *cgrp, char *buf);
char *lupos_scx_core_object_dup_path(const char *buf);
int lupos_scx_core_object_add_sub_kobj(struct scx_sched *sch, struct scx_sched *parent);
bool lupos_scx_core_object_has_sub_attach(struct scx_sched *sch);
struct kset *lupos_scx_core_object_create_sub_kset(struct scx_sched *sch);
#endif
#endif /* LUPOS_SCHED_EXT_CORE_OBJECT_LIFETIME_BINDINGS_H */
