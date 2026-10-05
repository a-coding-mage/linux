// SPDX-License-Identifier: GPL-2.0
/* F09 native leaves and exact callback/metadata ABI. No whole-owner C fallback.
 * Inactive source proposal for the common native envelope; unqualified C runtime.
 */
#error "SOURCE ONLY HOLD: sched_ext object lifetime native qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_object_lifetime_bindings.h"

static void free_dsq_rcufn(struct rcu_head *rcu)
{ lupos_scx_core_object_free_dsq_rcu_body(rcu); }
static void free_dsq_irq_workfn(struct irq_work *work)
{ lupos_scx_core_object_free_dsq_irq_body(work); }
static DEFINE_IRQ_WORK(free_dsq_irq_work, free_dsq_irq_workfn);
static void scx_sched_free_rcu_work(struct work_struct *work)
{ lupos_scx_core_object_sched_free_body(work); }
static void scx_kobj_release(struct kobject *kobj)
{ lupos_scx_core_object_release_body(kobj); }

void lupos_scx_core_object_zero_dsq(struct scx_dispatch_q *dsq) { memset(dsq, 0, sizeof(*dsq)); }
void lupos_scx_core_object_init_dsq_lock(struct scx_dispatch_q *dsq) { raw_spin_lock_init(&dsq->lock); }
void lupos_scx_core_object_init_list(struct list_head *head) { INIT_LIST_HEAD(head); }
/* Match for_each_possible_cpu's find_next_bit/size expressions without the
 * additional cpumask_check in cpumask_next. Keep the NR_CPUS == 1 alternative.
 * The Rust loop uses this exact native loop bound, not nr_cpu_ids: the pinned
 * small_cpumask_bits may be the larger compile-time NR_CPUS.
 */
int lupos_scx_core_object_next_cpu(int cpu)
{
#if NR_CPUS == 1
	return cpu < 0 ? 0 : 1;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits, cpu + 1);
#endif
}
unsigned int lupos_scx_core_object_possible_cpu_limit(void)
{
#if NR_CPUS == 1
	return 1;
#else
	return small_cpumask_bits;
#endif
}
int lupos_scx_core_object_next_node(int node)
{
#if MAX_NUMNODES > 1
	return node < 0 ? first_node(node_states[N_POSSIBLE]) : next_node(node, node_states[N_POSSIBLE]);
#else
	return node < 0 ? 0 : 1;
#endif
}
void lupos_scx_core_object_alloc_dsq_pcpu(struct scx_dispatch_q *dsq) { dsq->pcpu = alloc_percpu(struct scx_dsq_pcpu); }
struct scx_dsq_pcpu *lupos_scx_core_object_dsq_pcpu(struct scx_dispatch_q *dsq, int cpu) { return per_cpu_ptr(dsq->pcpu, cpu); }
struct scx_sched_pcpu *lupos_scx_core_object_sched_pcpu(struct scx_sched *sch, int cpu) { return per_cpu_ptr(sch->pcpu, cpu); }
struct rq *lupos_scx_core_object_cpu_rq(int cpu) { return cpu_rq(cpu); }
bool lupos_scx_core_object_warn_deferred_user(struct scx_deferred_reenq_user *dru) { return WARN_ON_ONCE(!list_empty(&dru->node)); }
unsigned long lupos_scx_core_object_lock_irqsave(raw_spinlock_t *lock)
__acquires(lock)
{ unsigned long flags; raw_spin_lock_irqsave(lock, flags); return flags; }
void lupos_scx_core_object_unlock_irqrestore(raw_spinlock_t *lock, unsigned long flags)
__releases(lock)
{ raw_spin_unlock_irqrestore(lock, flags); }
void lupos_scx_core_object_deferred_unlink(struct rq *rq, struct scx_deferred_reenq_user *dru)
{
	guard(raw_spinlock_irqsave)(&rq->scx.deferred_reenq_lock);
	list_del_init(&dru->node);
}
void lupos_scx_core_object_free_dsq_pcpu(struct scx_dispatch_q *dsq) { free_percpu(dsq->pcpu); }
void lupos_scx_core_object_free(const void *p) { kfree(p); }
struct scx_dispatch_q *lupos_scx_core_object_dsq_from_rcu(struct rcu_head *rcu) { return container_of(rcu, struct scx_dispatch_q, rcu); }
struct scx_dispatch_q *lupos_scx_core_object_dsq_from_free_node(struct llist_node *node) { return llist_entry(node, struct scx_dispatch_q, free_node); }
struct llist_node *lupos_scx_core_object_take_free_dsqs(void) { return llist_del_all(lupos_scx_core_dsqs_to_free()); }
void lupos_scx_core_object_call_dsq_rcu(struct scx_dispatch_q *dsq) { call_rcu(&dsq->rcu, free_dsq_rcufn); }
void lupos_scx_core_object_rcu_read_lock(void) __acquires_shared(RCU) { rcu_read_lock(); }
void lupos_scx_core_object_rcu_read_unlock(void) __releases_shared(RCU) { rcu_read_unlock(); }
void lupos_scx_core_object_error_dsq_busy(struct scx_sched *sch, struct scx_dispatch_q *dsq)
{ scx_error(sch, "attempting to destroy in-use dsq 0x%016llx (nr=%u)", dsq->id, dsq->nr); }
int lupos_scx_core_object_remove_dsq(struct scx_sched *sch, struct scx_dispatch_q *dsq)
{ return rhashtable_remove_fast(&sch->dsq_hash, &dsq->hash_node, *lupos_scx_core_dsq_hash_params()); }
bool lupos_scx_core_object_add_free_dsq(struct scx_dispatch_q *dsq) { return llist_add(&dsq->free_node, lupos_scx_core_dsqs_to_free()); }
void lupos_scx_core_object_queue_free_dsq(void) { irq_work_queue(&free_dsq_irq_work); }
size_t lupos_scx_core_object_scratch_size(void) { return struct_size_t(struct scx_cmask, bits, SCX_CMASK_NR_WORDS(num_possible_cpus())); }
void lupos_scx_core_object_alloc_scratch_slots(struct scx_sched *sch) { sch->set_cmask_scratch = alloc_percpu(struct scx_cmask *); }
struct scx_cmask **lupos_scx_core_object_scratch_slot(struct scx_sched *sch, int cpu) { return per_cpu_ptr(sch->set_cmask_scratch, cpu); }
void lupos_scx_core_object_free_scratch_slots(struct scx_sched *sch) { free_percpu(sch->set_cmask_scratch); }
void lupos_scx_core_object_cmask_init(struct scx_cmask *cm, u32 nr) { scx_cmask_init(cm, 0, nr); }
struct scx_sched *lupos_scx_core_object_kobj_sched(struct kobject *kobj) { return container_of(kobj, struct scx_sched, kobj); }
const struct scx_sched *lupos_scx_core_object_const_kobj_sched(const struct kobject *kobj) { return container_of(kobj, struct scx_sched, kobj); }
void lupos_scx_core_object_init_rcu_work(struct scx_sched *sch) { INIT_RCU_WORK(&sch->rcu_work, scx_sched_free_rcu_work); }
void lupos_scx_core_object_queue_rcu_work(struct scx_sched *sch) { queue_rcu_work(system_dfl_wq, &sch->rcu_work); }
ssize_t lupos_scx_core_object_emit_state(char *buf, const char *state) { return sysfs_emit(buf, "%s\n", state); }
ssize_t lupos_scx_core_object_emit_switch(char *buf, bool switching) { return sysfs_emit(buf, "%d\n", switching); }
ssize_t lupos_scx_core_object_emit_long(char *buf, long value) { return sysfs_emit(buf, "%ld\n", value); }
ssize_t lupos_scx_core_object_emit_ops(char *buf, const struct scx_sched *sch) { return sysfs_emit(buf, "%s\n", sch->ops.name); }
/* Formatting-only native runtime: preserve the source macro's field names,
 * order, format, and int accumulator without an invented indexed BUG path.
 * Rust owns the sysfs callback, scheduler lookup, event storage and F16 read.
 * This expansion is not Rust algorithm coverage.
 */
#define scx_attr_event_show(buf, at, events, kind) ({ \
	sysfs_emit_at(buf, at, "%s %llu\n", #kind, (events)->kind); \
})
int lupos_scx_core_object_emit_events(char *buf, const struct scx_event_stats *events)
{
	int at = 0;
#define SCX_EVENT(name) (at += scx_attr_event_show(buf, at, events, name))
	SCX_EVENTS_LIST(SCX_EVENT);
#undef SCX_EVENT
	return at;
}
#undef scx_attr_event_show
#define SCX_ATTR(_name) \
static ssize_t scx_attr_##_name##_show(struct kobject *kobj, struct kobj_attribute *ka, char *buf) \
{ return lupos_scx_core_object_##_name##_body(kobj, ka, buf); } \
static struct kobj_attribute scx_attr_##_name = { \
	.attr = { .name = __stringify(_name), .mode = 0444 }, \
	.show = scx_attr_##_name##_show, \
}
SCX_ATTR(state);
SCX_ATTR(switch_all);
SCX_ATTR(nr_rejected);
SCX_ATTR(hotplug_seq);
SCX_ATTR(enable_seq);
SCX_ATTR(ops);
SCX_ATTR(events);
static struct attribute *scx_global_attrs[] = {
	&scx_attr_state.attr, &scx_attr_switch_all.attr, &scx_attr_nr_rejected.attr,
	&scx_attr_hotplug_seq.attr, &scx_attr_enable_seq.attr, NULL,
};
static const struct attribute_group scx_global_attr_group = { .attrs = scx_global_attrs };
#ifdef CONFIG_EXT_SUB_SCHED
static const char *scx_cap_names[__SCX_NR_CAPS] = {
	[__SCX_CAP_ENQ_IMMED] = "enq_immed", [__SCX_CAP_ENQ] = "enq",
	[__SCX_CAP_PREEMPT] = "preempt", [__SCX_CAP_PERF] = "perf",
};
SCX_ATTR(caps);
struct scx_cmask *lupos_scx_core_object_alloc_agg(u32 nr) { return kzalloc(struct_size_t(struct scx_cmask, bits, SCX_CMASK_NR_WORDS(nr)), GFP_KERNEL); }
unsigned long *lupos_scx_core_object_alloc_bitmap(u32 nr) { return bitmap_zalloc(nr, GFP_KERNEL); }
void lupos_scx_core_object_free_bitmap(unsigned long *bitmap) { bitmap_free(bitmap); }
ssize_t lupos_scx_core_object_with_caps_snap(struct scx_sched *sch, char *buf,
        struct scx_cmask *agg, unsigned long *bitmap, int cap, ssize_t at, u32 nr)
{
	SCX_CMASK_DEFINE(snap, 0, SCX_CID_SHARD_MAX_CPUS);
	return lupos_scx_core_object_caps_one_body(sch, buf, agg, bitmap, cap, at, nr, snap);
}
struct scx_cmask *lupos_scx_core_object_shard_cap(struct scx_sched *sch, int si, int cap) { return &sch->pshard[si]->caps[cap].cmask; }
void lupos_scx_core_object_cmask_reframe(struct scx_cmask *cm, u32 base, u32 nr) { scx_cmask_reframe(cm, base, nr); }
void lupos_scx_core_object_cmask_copy(struct scx_cmask *dst, const struct scx_cmask *src) { scx_cmask_copy(dst, src); }
void lupos_scx_core_object_cmask_or(struct scx_cmask *dst, const struct scx_cmask *src) { scx_cmask_or(dst, src); }
void lupos_scx_core_object_bitmap_from_cmask(unsigned long *bitmap, struct scx_cmask *cm, u32 nr) { bitmap_from_arr64(bitmap, cm->bits, nr); }
ssize_t lupos_scx_core_object_emit_cap(char *buf, ssize_t at, int cap, u32 nr, unsigned long *bitmap)
{ return sysfs_emit_at(buf, at, "%s: %*pbl\n", scx_cap_names[cap], nr, bitmap); }
#endif
#undef SCX_ATTR
static struct attribute *scx_sched_attrs[] = {
	&scx_attr_ops.attr, &scx_attr_events.attr,
#ifdef CONFIG_EXT_SUB_SCHED
	&scx_attr_caps.attr,
#endif
	NULL,
};
ATTRIBUTE_GROUPS(scx_sched);
static const struct kobj_type scx_ktype = {
	.release = scx_kobj_release, .sysfs_ops = &kobj_sysfs_ops, .default_groups = scx_sched_groups,
};
static int scx_uevent(const struct kobject *kobj, struct kobj_uevent_env *env)
{ return lupos_scx_core_object_uevent_body(kobj, env); }
static const struct kset_uevent_ops scx_uevent_ops = { .uevent = scx_uevent };
bool lupos_scx_core_object_is_sched_kobj(const struct kobject *kobj) { return kobj->ktype == &scx_ktype; }
int lupos_scx_core_object_add_uevent(struct kobj_uevent_env *env, const struct scx_sched *sch) { return add_uevent_var(env, "SCXOPS=%s", sch->ops.name); }
const struct attribute_group *lupos_scx_core_object_global_attr_group(void) { return &scx_global_attr_group; }
const struct kset_uevent_ops *lupos_scx_core_object_uevent_ops(void) { return &scx_uevent_ops; }

/* F11 owns these exact static native callback shims. F10's timer shim is an
 * explicit pending owner dependency, not a fallback body supplied by F09. */
#include "sched_ext_core_exit_dump_bindings.h"
static void scx_bypass_lb_timerfn(struct timer_list *timer);
int lupos_scx_core_object_node_limit(void) { return MAX_NUMNODES; }
struct scx_sched *lupos_scx_core_object_work_sched(struct work_struct *work)
{ return container_of(to_rcu_work(work), struct scx_sched, rcu_work); }
void lupos_scx_core_object_irq_sync(struct irq_work *work) { irq_work_sync(work); }
void lupos_scx_core_object_destroy_helper(struct scx_sched *sch) { kthread_destroy_worker(sch->helper); }
void lupos_scx_core_object_shutdown_timer(struct scx_sched *sch) { timer_shutdown_sync(&sch->bypass_lb_timer); }
void lupos_scx_core_object_free_donee(struct scx_sched *sch) { free_cpumask_var(sch->bypass_lb_donee_cpumask); }
void lupos_scx_core_object_free_resched(struct scx_sched *sch) { free_cpumask_var(sch->bypass_lb_resched_cpumask); }
void lupos_scx_core_object_free_stall(struct scx_sched *sch) { free_cpumask_var(sch->stall_cpus); }
void lupos_scx_core_object_warn_deferred_local(struct scx_sched_pcpu *pcpu) { WARN_ON_ONCE(!list_empty(&pcpu->deferred_reenq_local.node)); }
void lupos_scx_core_object_warn_kick_node(struct scx_sched_pcpu *pcpu) { WARN_ON_ONCE(!list_empty(&pcpu->to_kick_node)); }
void lupos_scx_core_object_discard_ecaps(int cpu, struct scx_sched_pcpu *pcpu) { scx_discard_ecaps_to_sync(cpu, pcpu); }
void lupos_scx_core_object_free_kick(struct scx_sched_pcpu *pcpu) { free_cpumask_var(pcpu->cpus_to_kick); }
void lupos_scx_core_object_free_kick_idle(struct scx_sched_pcpu *pcpu) { free_cpumask_var(pcpu->cpus_to_kick_if_idle); }
void lupos_scx_core_object_free_preempt(struct scx_sched_pcpu *pcpu) { free_cpumask_var(pcpu->cpus_to_preempt); }
void lupos_scx_core_object_free_wait(struct scx_sched_pcpu *pcpu) { free_cpumask_var(pcpu->cpus_to_wait); }
void lupos_scx_core_object_free_sched_pcpu(struct scx_sched *sch) { free_percpu(sch->pcpu); }
struct scx_sched_pnode *lupos_scx_core_object_pnode(struct scx_sched *sch, int node) { return sch->pnode[node]; }
void lupos_scx_core_object_set_pnode(struct scx_sched *sch, int node, struct scx_sched_pnode *pnode) { sch->pnode[node] = pnode; }
void lupos_scx_core_object_free_pshards(struct scx_sched *sch) { scx_free_pshards(sch); }
void lupos_scx_core_object_walk_enter(struct scx_sched *sch, struct rhashtable_iter *iter) { rhashtable_walk_enter(&sch->dsq_hash, iter); }
void lupos_scx_core_object_walk_start(struct rhashtable_iter *iter)
__acquires_shared(RCU)
{ rhashtable_walk_start(iter); }
struct scx_dispatch_q *lupos_scx_core_object_walk_next(struct rhashtable_iter *iter) { return rhashtable_walk_next(iter); }
void lupos_scx_core_object_walk_stop(struct rhashtable_iter *iter)
__releases_shared(RCU)
{ rhashtable_walk_stop(iter); }
void lupos_scx_core_object_walk_exit(struct rhashtable_iter *iter) { rhashtable_walk_exit(iter); }
bool lupos_scx_core_object_is_err_or_null(const void *p) { return IS_ERR_OR_NULL(p); }
bool lupos_scx_core_object_is_err(const void *p) { return IS_ERR(p); }
long lupos_scx_core_object_ptr_err(const void *p) { return PTR_ERR(p); }
struct scx_sched *lupos_scx_core_object_err_sched(long err) { return ERR_PTR(err); }
void lupos_scx_core_object_free_hash(struct scx_sched *sch) { rhashtable_free_and_destroy(&sch->dsq_hash, NULL, NULL); }
void lupos_scx_core_object_put_map(struct bpf_map *map) { bpf_map_put(map); }
void lupos_scx_core_object_dec_has_subs(struct scx_sched *sch) { scx_dec_has_subs(sch); }
void lupos_scx_core_object_kvfree(const void *p) { kvfree(p); }
struct scx_exit_info *lupos_scx_core_object_alloc_ei(void) { return kzalloc_obj(struct scx_exit_info); }
unsigned long *lupos_scx_core_object_alloc_bt(void) { return kzalloc_objs(unsigned long, SCX_EXIT_BT_LEN); }
char *lupos_scx_core_object_alloc_msg(void) { return kzalloc(SCX_EXIT_MSG_LEN, GFP_KERNEL); }
char *lupos_scx_core_object_alloc_dump(size_t size) { return kvzalloc(size, GFP_KERNEL); }
int lupos_scx_core_object_with_link_locks(struct scx_sched *sch)
{
	int ret;
	scoped_guard(raw_spinlock_irqsave, lupos_scx_core_bypass_lock())
	scoped_guard(raw_spinlock, lupos_scx_core_sched_lock())
		ret = lupos_scx_core_object_link_locked_body(sch);
	return ret;
}
void lupos_scx_core_object_with_unlink_lock(struct scx_sched *sch)
{
	scoped_guard(raw_spinlock_irq, lupos_scx_core_sched_lock())
		lupos_scx_core_object_unlink_locked_body(sch);
}
void lupos_scx_core_object_list_add_tail_rcu(struct list_head *node, struct list_head *head) { list_add_tail_rcu(node, head); }
void lupos_scx_core_object_list_del_rcu(struct list_head *node) { list_del_rcu(node); }
void lupos_scx_core_object_mb(void) { smp_mb(); }
struct scx_sched_pnode *lupos_scx_core_object_alloc_pnode(int node) { return kzalloc_node(sizeof(struct scx_sched_pnode), GFP_KERNEL, node); }
struct sched_ext_ops *lupos_scx_core_object_cmd_ops(struct scx_enable_cmd *cmd) { return cmd->ops; }
struct scx_sched *lupos_scx_core_object_alloc_sched(int level) { return kzalloc_flex(struct scx_sched, ancestors, level + 1); }
int lupos_scx_core_object_init_hash(struct scx_sched *sch) { return rhashtable_init(&sch->dsq_hash, lupos_scx_core_dsq_hash_params()); }
void lupos_scx_core_object_alloc_pnodes(struct scx_sched *sch) { sch->pnode = kzalloc_objs(sch->pnode[0], nr_node_ids); }
void lupos_scx_core_object_alloc_sched_pcpu(struct scx_sched *sch)
{ sch->pcpu = __alloc_percpu(struct_size_t(struct scx_sched_pcpu, dsp_ctx.buf, sch->dsp_max_batch), __alignof__(struct scx_sched_pcpu)); }
bool lupos_scx_core_object_alloc_kick(struct scx_sched_pcpu *pcpu, int node) { return zalloc_cpumask_var_node(&pcpu->cpus_to_kick, GFP_KERNEL, node); }
bool lupos_scx_core_object_alloc_kick_idle(struct scx_sched_pcpu *pcpu, int node) { return zalloc_cpumask_var_node(&pcpu->cpus_to_kick_if_idle, GFP_KERNEL, node); }
bool lupos_scx_core_object_alloc_preempt(struct scx_sched_pcpu *pcpu, int node) { return zalloc_cpumask_var_node(&pcpu->cpus_to_preempt, GFP_KERNEL, node); }
bool lupos_scx_core_object_alloc_wait(struct scx_sched_pcpu *pcpu, int node) { return zalloc_cpumask_var_node(&pcpu->cpus_to_wait, GFP_KERNEL, node); }
void lupos_scx_core_object_run_helper(struct scx_sched *sch) { sch->helper = kthread_run_worker(0, "sched_ext_helper"); }
void lupos_scx_core_object_helper_fifo(struct scx_sched *sch) { sched_set_fifo(sch->helper->task); }
void lupos_scx_core_object_copy_ancestors(struct scx_sched *sch, struct scx_sched *parent, int level) { memcpy(sch->ancestors, parent->ancestors, level * sizeof(parent->ancestors[0])); }
void lupos_scx_core_object_set_ancestor(struct scx_sched *sch, int level) { sch->ancestors[level] = sch; }
unsigned long lupos_scx_core_object_msecs_to_jiffies(unsigned int ms) { return msecs_to_jiffies(ms); }
void lupos_scx_core_object_init_exit_kind(struct scx_sched *sch) { atomic_set(&sch->exit_kind, SCX_EXIT_NONE); }
void lupos_scx_core_object_init_disable_irq(struct scx_sched *sch) { sch->disable_irq_work = IRQ_WORK_INIT_HARD(scx_disable_irq_workfn); }
void lupos_scx_core_object_init_propagate_irq(struct scx_sched *sch) { sch->propagate_exit_irq_work = IRQ_WORK_INIT_HARD(scx_propagate_exit_irq_workfn); }
void lupos_scx_core_object_init_disable_work(struct scx_sched *sch) { kthread_init_work(&sch->disable_work, scx_disable_workfn); }
void lupos_scx_core_object_init_bypass_timer(struct scx_sched *sch) { timer_setup(&sch->bypass_lb_timer, scx_bypass_lb_timerfn, 0); }
bool lupos_scx_core_object_alloc_donee(struct scx_sched *sch) { return alloc_cpumask_var(&sch->bypass_lb_donee_cpumask, GFP_KERNEL); }
bool lupos_scx_core_object_alloc_resched(struct scx_sched *sch) { return alloc_cpumask_var(&sch->bypass_lb_resched_cpumask, GFP_KERNEL); }
bool lupos_scx_core_object_alloc_stall(struct scx_sched *sch) { return zalloc_cpumask_var(&sch->stall_cpus, GFP_KERNEL); }
void lupos_scx_core_object_copy_cpu_ops(struct scx_sched *sch, struct scx_enable_cmd *cmd) { sch->ops = *cmd->ops; }
void lupos_scx_core_object_copy_cid_ops(struct scx_sched *sch, struct scx_enable_cmd *cmd) { sch->ops_cid = *cmd->ops_cid; }
void lupos_scx_core_object_publish_priv(struct sched_ext_ops *ops, struct scx_sched *sch) { rcu_assign_pointer(ops->priv, sch); }
void lupos_scx_core_object_init_kobj(struct scx_sched *sch) { kobject_init(&sch->kobj, &scx_ktype); }
#if defined(CONFIG_MMU) && defined(CONFIG_64BIT)
void lupos_scx_core_object_arena_kern_base(struct scx_sched *sch) { sch->arena_kern_base = bpf_arena_map_kern_vm_start(sch->arena_map); }
#endif
int lupos_scx_core_object_add_root_kobj(struct scx_sched *sch) { return kobject_add(&sch->kobj, NULL, "root"); }
#ifdef CONFIG_EXT_SUB_SCHED
void lupos_scx_core_object_cgroup_put(struct cgroup *cgrp) { cgroup_put(cgrp); }
struct cgroup *lupos_scx_core_object_sch_cgroup(struct scx_sched *sch) { return sch_cgroup(sch); }
void lupos_scx_core_object_kobject_put(struct kobject *kobj) { kobject_put(kobj); }
void lupos_scx_core_object_kobject_get(struct kobject *kobj) { kobject_get(kobj); }
s32 lupos_scx_core_object_bypass_depth(struct scx_sched *sch) { return READ_ONCE(sch->bypass_depth); }
void lupos_scx_core_object_error_parent_bypass(struct scx_sched *sch) { scx_error(sch, "parent bypassing (%d)", -EBUSY); }
int lupos_scx_core_object_insert_sched(struct scx_sched *sch)
{ return rhashtable_lookup_insert_fast(lupos_scx_core_sched_hash(), &sch->hash_node, *lupos_scx_core_sched_hash_params()); }
void lupos_scx_core_object_error_insert_sched(struct scx_sched *sch, int ret) { scx_error(sch, "failed to insert into scx_sched_hash (%d)", ret); }
void lupos_scx_core_object_remove_sched(struct scx_sched *sch)
{ rhashtable_remove_fast(lupos_scx_core_sched_hash(), &sch->hash_node, *lupos_scx_core_sched_hash_params()); }
bool lupos_scx_core_object_parent_aborting(struct scx_sched *parent) { return unlikely(READ_ONCE(parent->aborting)); }
void lupos_scx_core_object_error_parent_disabled(struct scx_sched *sch) { scx_error(sch, "parent disabled (%d)", -ENOENT); }
void lupos_scx_core_object_init_ecaps_node(struct scx_sched_pcpu *pcpu) { init_llist_node(&pcpu->ecaps_to_sync_node); }
char *lupos_scx_core_object_alloc_path(void) { return kzalloc(PATH_MAX, GFP_KERNEL); }
void lupos_scx_core_object_cgroup_path(struct cgroup *cgrp, char *buf) { cgroup_path(cgrp, buf, PATH_MAX); }
char *lupos_scx_core_object_dup_path(const char *buf) { return kstrdup(buf, GFP_KERNEL); }
int lupos_scx_core_object_add_sub_kobj(struct scx_sched *sch, struct scx_sched *parent)
{ return kobject_add(&sch->kobj, &parent->sub_kset->kobj, "sub-%llu", cgroup_id(sch_cgroup(sch))); }
bool lupos_scx_core_object_has_sub_attach(struct scx_sched *sch) { return sch->ops.sub_attach; }
struct kset *lupos_scx_core_object_create_sub_kset(struct scx_sched *sch) { return kset_create_and_add("sub", NULL, &sch->kobj); }
#endif
