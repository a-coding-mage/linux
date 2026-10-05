// SPDX-License-Identifier: GPL-2.0
/* F17 native metadata, initialization/assertion sites and narrow leaves.
 * Explicit unqualified C runtime. Include only in the single original
 * build_policy envelope after the real F00/F02/F04/F05/F06/F07/F09/F13 pieces.
 * No private metadata extern workaround or separately linked native object.
 */
#error "SOURCE ONLY HOLD: sched_ext registration/init ABI, sections, metadata and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_registration_init_bindings.h"

/* F01's sole Rust update_curr_scx algorithm has no native shell. This exact
 * static class callback only forwards through the agreed F17 C-ABI seam. */
static void update_curr_scx(struct rq *rq)
{
	lupos_scx_ri_update_curr_body(rq);
}

/* BEGIN EXACT METADATA ext.c:4997-5038 */
/*
 * Omitted operations:
 *
 * - migrate_task_rq: Unnecessary as task to cpu mapping is transient.
 *
 * - task_fork/dead: We need fork/dead notifications for all tasks regardless of
 *   their current sched_class. Call them directly from sched core instead.
 */
DEFINE_SCHED_CLASS(ext) = {
	.enqueue_task		= enqueue_task_scx,
	.dequeue_task		= dequeue_task_scx,
	.yield_task		= yield_task_scx,
	.yield_to_task		= yield_to_task_scx,

	.wakeup_preempt		= wakeup_preempt_scx,

	.pick_task		= pick_task_scx,

	.put_prev_task		= put_prev_task_scx,
	.set_next_task		= set_next_task_scx,

	.select_task_rq		= select_task_rq_scx,
	.task_woken		= task_woken_scx,
	.set_cpus_allowed	= set_cpus_allowed_scx,

	.rq_online		= rq_online_scx,
	.rq_offline		= rq_offline_scx,

	.task_tick		= task_tick_scx,

	.switching_to		= switching_to_scx,
	.switched_from		= switched_from_scx,
	.switched_to		= switched_to_scx,
	.reweight_task		= reweight_task_scx,
	.prio_changed		= prio_changed_scx,

	.update_curr		= update_curr_scx,

#ifdef CONFIG_UCLAMP_TASK
	.uclamp_enabled		= 1,
#endif
};
/* END EXACT METADATA */

static void sysrq_handle_sched_ext_reset(u8 key)
{
	lupos_scx_ri_sysrq_reset_body(key);
}

/* BEGIN EXACT METADATA ext.c:8401-8406 */
static const struct sysrq_key_op sysrq_sched_ext_reset_op = {
	.handler	= sysrq_handle_sched_ext_reset,
	.help_msg	= "reset-sched-ext(S)",
	.action_msg	= "Disable sched_ext and revert all tasks to CFS",
	.enable_mask	= SYSRQ_ENABLE_RTNICE,
};
/* END EXACT METADATA */

static void sysrq_handle_sched_ext_dump(u8 key)
{
	struct scx_exit_info ei = {
		.kind		= SCX_EXIT_NONE,
		.exit_cpu	= -1,
		.reason		= "SysRq-D",
	};

	lupos_scx_ri_sysrq_dump_body(key, &ei);
}

/* BEGIN EXACT METADATA ext.c:8421-8426 */
static const struct sysrq_key_op sysrq_sched_ext_dump_op = {
	.handler	= sysrq_handle_sched_ext_dump,
	.help_msg	= "dump-sched-ext(D)",
	.action_msg	= "Trigger sched_ext debug dump",
	.enable_mask	= SYSRQ_ENABLE_RTNICE,
};
/* END EXACT METADATA */

static int scx_pm_handler(struct notifier_block *nb, unsigned long event, void *ptr)
{
	/* Exact original guard definition/context attributes, not manual lock
	 * approximations; Rust returns through this native scope. No cross-FFI
	 * unwinding is permitted; native scope exit releases the RCU guard. */
	guard(rcu)();
	return lupos_scx_ri_pm_body(nb, event, ptr);
}

/* BEGIN EXACT METADATA ext.c:8658-8660 */
static struct notifier_block scx_pm_notifier = {
	.notifier_call = scx_pm_handler,
};
/* END EXACT METADATA */

void __init init_sched_ext_class(void)
{
	s32 v;

	/* Preserve native enum retention and WRITE_ONCE's configured semantics. */
	WRITE_ONCE(v, SCX_ENQ_WAKEUP | SCX_DEQ_SLEEP | SCX_KICK_PREEMPT |
		   SCX_TG_ONLINE);
	lupos_scx_ri_init_class_body();
}

static int __init scx_init(void)
{
/* BEGIN EXACT ASSERTIONS ext.c:10894-10965 */
	/*
	 * sched_ext_ops_cid mirrors sched_ext_ops up to and including @priv.
	 * Both bpf_scx_init_member() and bpf_scx_check_member() use offsets
	 * from struct sched_ext_ops; sched_ext_ops_cid relies on those offsets
	 * matching for the shared fields. Catch any drift at boot.
	 */
#define CID_OFFSET_MATCH(cpu_field, cid_field)					\
	BUILD_BUG_ON(offsetof(struct sched_ext_ops, cpu_field) !=		\
		     offsetof(struct sched_ext_ops_cid, cid_field))
	/* data fields used by bpf_scx_init_member() */
	CID_OFFSET_MATCH(dispatch_max_batch, dispatch_max_batch);
	CID_OFFSET_MATCH(flags, flags);
	CID_OFFSET_MATCH(name, name);
	CID_OFFSET_MATCH(timeout_ms, timeout_ms);
	CID_OFFSET_MATCH(exit_dump_len, exit_dump_len);
	CID_OFFSET_MATCH(hotplug_seq, hotplug_seq);
	CID_OFFSET_MATCH(cid_shard_size, cid_shard_size);
	CID_OFFSET_MATCH(rescue_bandwidth_ppt, rescue_bandwidth_ppt);
	CID_OFFSET_MATCH(rescue_quantum_us, rescue_quantum_us);
	CID_OFFSET_MATCH(sub_cgroup_id, sub_cgroup_id);
	/* shared callbacks: the union view requires byte-for-byte offset match */
	CID_OFFSET_MATCH(enqueue, enqueue);
	CID_OFFSET_MATCH(dequeue, dequeue);
	CID_OFFSET_MATCH(dispatch, dispatch);
	CID_OFFSET_MATCH(tick, tick);
	CID_OFFSET_MATCH(runnable, runnable);
	CID_OFFSET_MATCH(running, running);
	CID_OFFSET_MATCH(stopping, stopping);
	CID_OFFSET_MATCH(quiescent, quiescent);
	CID_OFFSET_MATCH(yield, yield);
	CID_OFFSET_MATCH(core_sched_before, core_sched_before);
	CID_OFFSET_MATCH(set_weight, set_weight);
	CID_OFFSET_MATCH(update_idle, update_idle);
	CID_OFFSET_MATCH(init_task, init_task);
	CID_OFFSET_MATCH(exit_task, exit_task);
	CID_OFFSET_MATCH(enable, enable);
	CID_OFFSET_MATCH(disable, disable);
	CID_OFFSET_MATCH(dump, dump);
	CID_OFFSET_MATCH(dump_task, dump_task);
	CID_OFFSET_MATCH(sub_attach, sub_attach);
	CID_OFFSET_MATCH(sub_detach, sub_detach);
	CID_OFFSET_MATCH(sub_caps_updated, sub_caps_updated);
	CID_OFFSET_MATCH(sub_ecaps_updated, sub_ecaps_updated);
	CID_OFFSET_MATCH(init_cids, init_cids);
	CID_OFFSET_MATCH(init, init);
	CID_OFFSET_MATCH(exit, exit);
	/* renamed callbacks must occupy the same slot as their cpu-form sibling */
	CID_OFFSET_MATCH(select_cpu, select_cid);
	CID_OFFSET_MATCH(set_cpumask, set_cmask);
	CID_OFFSET_MATCH(cpu_online, cid_online);
	CID_OFFSET_MATCH(cpu_offline, cid_offline);
	CID_OFFSET_MATCH(dump_cpu, dump_cid);
#ifdef CONFIG_EXT_GROUP_SCHED
	CID_OFFSET_MATCH(cgroup_init, cpuctl_init);
	CID_OFFSET_MATCH(cgroup_exit, cpuctl_exit);
	CID_OFFSET_MATCH(cgroup_prep_move, cpuctl_prep_move);
	CID_OFFSET_MATCH(cgroup_move, cpuctl_move);
	CID_OFFSET_MATCH(cgroup_cancel_move, cpuctl_cancel_move);
	CID_OFFSET_MATCH(cgroup_set_weight, cpuctl_set_weight);
	CID_OFFSET_MATCH(cgroup_set_bandwidth, cpuctl_set_bandwidth);
	CID_OFFSET_MATCH(cgroup_set_idle, cpuctl_set_idle);
#endif
	/* @priv tail must align since both share the same data block */
	CID_OFFSET_MATCH(priv, priv);
	/*
	 * cid-form must end exactly at @priv - scx_validate_ops() skips
	 * cpu_acquire/cpu_release for cid-form because reading those fields
	 * past the BPF allocation would be UB.
	 */
	BUILD_BUG_ON(offsetof(struct sched_ext_ops_cid, __end) !=
		     offsetofend(struct sched_ext_ops, priv));
#undef CID_OFFSET_MATCH
/* END EXACT ASSERTIONS */
	return lupos_scx_ri_init_body();
}
__initcall(scx_init);

/* No kfunc diagnostic scope encloses any F17 helper or record. */
/* Distinct original rcu_dereference sites retain distinct lockdep state. */
struct scx_sched *lupos_scx_ri_reset_root_rcu(void)
{ return rcu_dereference(scx_root); }
struct scx_sched *lupos_scx_ri_pm_root_rcu(void)
{ return rcu_dereference(scx_root); }
bool lupos_scx_ri_likely_sched(struct scx_sched *sch)
{ return likely(sch); }
void lupos_scx_ri_info_not_loaded(void)
{ pr_info("sched_ext: BPF schedulers not loaded\n"); }
void lupos_scx_ri_for_each_sched_rcu(struct scx_exit_info *ei)
{
	struct scx_sched *sch;

	/* Exact native iterator/protection diagnostics; no extra RCU guard. */
	list_for_each_entry_rcu(sch, &scx_sched_all, all)
		lupos_scx_ri_dump_one_body(sch, ei);
}

void __init lupos_scx_ri_idle_init_masks(void)
{ scx_idle_init_masks(); }
void __init lupos_scx_ri_for_each_possible_cpu(void)
{
	s32 cpu;

	/* CONFIG-specific CPU iteration remains native, including UP behavior. */
	for_each_possible_cpu(cpu)
		lupos_scx_ri_init_cpu_body(cpu);
}
struct rq *__init lupos_scx_ri_cpu_rq(s32 cpu)
{ return cpu_rq(cpu); }
void __init lupos_scx_ri_init_local_dsq(struct rq *rq)
{
	/* F09's existing exported Rust body owns DSQ initialization. */
	BUG_ON(scx_init_dsq(&rq->scx.local_dsq, SCX_DSQ_LOCAL, NULL));
}
#ifdef CONFIG_EXT_SUB_SCHED
void __init lupos_scx_ri_init_reject_dsq(struct rq *rq)
{ BUG_ON(scx_init_dsq(&rq->scx.reject_dsq, SCX_DSQ_REJECT, NULL)); }
void __init lupos_scx_ri_rescue_init(struct rq *rq)
{ scx_rescue_init(rq); }
void __init lupos_scx_ri_init_sched_hash(void)
{ BUG_ON(rhashtable_init(&scx_sched_hash, &scx_sched_hash_params)); }
#endif /* CONFIG_EXT_SUB_SCHED */
void __init lupos_scx_ri_init_list(struct list_head *head)
{ INIT_LIST_HEAD(head); }
void __init lupos_scx_ri_alloc_cpus_to_sync(struct rq *rq, int n)
{ BUG_ON(!zalloc_cpumask_var_node(&rq->scx.cpus_to_sync, GFP_KERNEL, n)); }
void __init lupos_scx_ri_init_reenq_lock(struct rq *rq)
{
	/* Keep the exact operand spelling and this single native lock-class key. */
	raw_spin_lock_init(&rq->scx.deferred_reenq_lock);
}
void __init lupos_scx_ri_init_deferred_irq(struct rq *rq)
{ rq->scx.deferred_irq_work = IRQ_WORK_INIT_HARD(deferred_irq_workfn); }
void __init lupos_scx_ri_init_kick_irq(struct rq *rq)
{ rq->scx.kick_cpus_irq_work = IRQ_WORK_INIT_HARD(kick_cpus_irq_workfn); }
bool __init lupos_scx_ri_cpu_online(s32 cpu)
{ return cpu_online(cpu); }
void __init lupos_scx_ri_mark_online(s32 cpu)
{ cpu_rq(cpu)->scx.flags |= SCX_RQ_ONLINE; }
void __init lupos_scx_ri_register_reset_key(void)
{ register_sysrq_key('S', &sysrq_sched_ext_reset_op); }
void __init lupos_scx_ri_register_dump_key(void)
{ register_sysrq_key('D', &sysrq_sched_ext_dump_op); }
void __init lupos_scx_ri_init_watchdog(void)
{ INIT_DELAYED_WORK(&scx_watchdog_work, scx_watchdog_workfn); }

/* F13 owns every referenced native set/table, its filter and CFI stubs. These
 * leaves each perform one original registration, in Rust-selected order. */
int __init lupos_scx_ri_register_enqueue_dispatch(void)
{ return register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &scx_kfunc_set_enqueue_dispatch); }
int __init lupos_scx_ri_register_dispatch(void)
{ return register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &scx_kfunc_set_dispatch); }
int __init lupos_scx_ri_register_cpu_release(void)
{ return register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &scx_kfunc_set_cpu_release); }
int __init lupos_scx_ri_register_unlocked_struct_ops(void)
{ return register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &scx_kfunc_set_unlocked); }
int __init lupos_scx_ri_register_unlocked_syscall(void)
{ return register_btf_kfunc_id_set(BPF_PROG_TYPE_SYSCALL, &scx_kfunc_set_unlocked); }
int __init lupos_scx_ri_register_any_struct_ops(void)
{ return register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &scx_kfunc_set_any); }
int __init lupos_scx_ri_register_any_tracing(void)
{ return register_btf_kfunc_id_set(BPF_PROG_TYPE_TRACING, &scx_kfunc_set_any); }
int __init lupos_scx_ri_register_any_syscall(void)
{ return register_btf_kfunc_id_set(BPF_PROG_TYPE_SYSCALL, &scx_kfunc_set_any); }
int __init lupos_scx_ri_idle_init(void)
{ return scx_idle_init(); }
int __init lupos_scx_ri_cid_kfunc_init(void)
{ return scx_cid_kfunc_init(); }
int __init lupos_scx_ri_register_cpu_ops(void)
{ return register_bpf_struct_ops(&bpf_sched_ext_ops, sched_ext_ops); }
int __init lupos_scx_ri_register_cid_ops(void)
{ return register_bpf_struct_ops(&bpf_sched_ext_ops_cid, sched_ext_ops_cid); }
int __init lupos_scx_ri_register_pm(void)
{ return register_pm_notifier(&scx_pm_notifier); }
struct kset *__init lupos_scx_ri_create_kset(void)
{ return kset_create_and_add("sched_ext", &scx_uevent_ops, kernel_kobj); }
int __init lupos_scx_ri_create_global_group(void)
{ return sysfs_create_group(&scx_kset->kobj, &scx_global_attr_group); }

/* One native printk site per original diagnostic; no format rewrite, generic
 * logging wrapper or warning waiver. Source/line metadata remains held. */
void __init lupos_scx_ri_error_kfunc_sets(int ret)
{ pr_err("sched_ext: Failed to register kfunc sets (%d)\n", ret); }
void __init lupos_scx_ri_error_idle_init(int ret)
{ pr_err("sched_ext: Failed to initialize idle tracking (%d)\n", ret); }
void __init lupos_scx_ri_error_cid_kfuncs(int ret)
{ pr_err("sched_ext: Failed to register cid kfuncs (%d)\n", ret); }
void __init lupos_scx_ri_error_cpu_ops(int ret)
{ pr_err("sched_ext: Failed to register struct_ops (%d)\n", ret); }
void __init lupos_scx_ri_error_cid_ops(int ret)
{ pr_err("sched_ext: Failed to register cid struct_ops (%d)\n", ret); }
void __init lupos_scx_ri_error_pm(int ret)
{ pr_err("sched_ext: Failed to register PM notifier (%d)\n", ret); }
void __init lupos_scx_ri_error_kset(void)
{ pr_err("sched_ext: Failed to create /sys/kernel/sched_ext\n"); }
void __init lupos_scx_ri_error_global_group(void)
{ pr_err("sched_ext: Failed to add global attributes\n"); }
