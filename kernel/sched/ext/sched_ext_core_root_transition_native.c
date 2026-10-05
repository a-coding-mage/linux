// SPDX-License-Identifier: GPL-2.0
/* F12 native ABI/guard/macro leaves. Unqualified C runtime, not Rust coverage.
 * One future build_policy envelope only; no original ext.c algorithm fallback.
 */
#error "SOURCE ONLY HOLD: root transition ABI, stack, race and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_root_transition_bindings.h"

static void scx_root_enable_workfn(struct kthread_work *work)
{ lupos_scx_root_enable_work_body(work); }

/* The original helper storage identities, scope and DEFINE_MUTEX name remain
 * in the original static function. The transaction itself is Rust-owned. */
static s32 scx_enable(struct scx_enable_cmd *cmd, struct bpf_link *link)
{
	static struct kthread_worker *helper;
	static DEFINE_MUTEX(helper_mutex);
	return lupos_scx_root_enable_body(cmd, link, &helper, &helper_mutex);
}

struct scx_enable_cmd *lupos_scx_root_work_cmd(struct kthread_work *work)
{ return container_of(work, struct scx_enable_cmd, work); }
struct sched_ext_ops *lupos_scx_root_cmd_ops(struct scx_enable_cmd *cmd) { return cmd->ops; }
struct sched_ext_ops *lupos_scx_root_sched_ops(struct scx_sched *sch) { return &sch->ops; }
bool lupos_scx_root_ops_priv_present(struct sched_ext_ops *ops) { return rcu_access_pointer(ops->priv); }
bool lupos_scx_root_has_init_cids(struct scx_sched *sch) { return sch->ops_cid.init_cids; }
int lupos_scx_root_call_init_cids(struct scx_sched *sch) { return SCX_CALL_OP_RET(sch, init_cids, NULL); }
int lupos_scx_root_call_init(struct scx_sched *sch) { return SCX_CALL_OP_RET(sch, init, NULL); }
void lupos_scx_root_call_exit(struct scx_sched *sch) { SCX_CALL_OP(sch, exit, NULL, sch->exit_info); }
int lupos_scx_root_sanitize_init_cids(struct scx_sched *sch, int ret) { return scx_ops_sanitize_err(sch, "init_cids", ret); }
int lupos_scx_root_sanitize_init(struct scx_sched *sch, int ret) { return scx_ops_sanitize_err(sch, "init", ret); }
void lupos_scx_root_error_init_cids(struct scx_sched *sch, int ret) { scx_error(sch, "ops.init_cids() failed (%d)", ret); }
void lupos_scx_root_error_init(struct scx_sched *sch, int ret) { scx_error(sch, "ops.init() failed (%d)", ret); }
void lupos_scx_root_error_init_task(struct scx_sched *sch, int ret, struct task_struct *p)
{ scx_error(sch, "ops.init_task() failed (%d) for %s[%d]", ret, p->comm, p->pid); }
void lupos_scx_root_error_enable(struct scx_sched *sch, int ret) { scx_error(sch, "scx_root_enable() failed (%d)", ret); }
void lupos_scx_root_exit_hotplug(struct scx_sched *sch, const struct sched_ext_ops *ops,
	unsigned long long global_hotplug_seq)
{
	scx_exit(sch, SCX_EXIT_UNREG_KERN,
		 SCX_ECODE_ACT_RESTART | SCX_ECODE_RSN_HOTPLUG,
		 "expected hotplug seq %llu did not match actual %llu",
		 ops->hotplug_seq, global_hotplug_seq);
}
void lupos_scx_root_error_enq_last(struct scx_sched *sch)
{ scx_error(sch, "SCX_OPS_ENQ_LAST requires ops.enqueue() to be implemented"); }
void lupos_scx_root_error_tid_dependency(struct scx_sched *sch)
{ scx_error(sch, "SCX_OPS_TID_TO_TASK requires root scheduler to enable it"); }
void lupos_scx_root_error_idle_per_node(struct scx_sched *sch)
{ scx_error(sch, "SCX_OPS_BUILTIN_IDLE_PER_NODE requires CPU idle selection enabled"); }
void lupos_scx_root_warn_deprecated_cpu_ops(void)
{ pr_warn_ratelimited("ops->cpu_acquire/release() are deprecated, use sched_switch TP instead\n"); }
void lupos_scx_root_error_sub_cpu_form(struct scx_sched *sch) { scx_error(sch, "sub-sched requires cid-form struct_ops"); }
void lupos_scx_root_error_attach_cpu_form(struct scx_sched *sch) { scx_error(sch, "sub_attach/sub_detach requires cid-form struct_ops"); }
void lupos_scx_root_warn_duplicate_disable(void) { WARN_ONCE(true, "sched_ext: duplicate disabling instance?"); }
void lupos_scx_root_warn_no_ops(struct scx_sched *sch)
{ pr_warn("sched_ext: ops error detected without ops (%s)\n", sch->exit_info->msg); }
void lupos_scx_root_warn_exit_none(struct scx_sched *sch) { WARN_ON_ONCE(atomic_read(&sch->exit_kind) == SCX_EXIT_NONE); }
void lupos_scx_root_warn_attach_bw(int cpu, int ret)
{ pr_warn("sched_ext: failed to attach ext_server on CPU %d (%d)\n", cpu, ret); }
void lupos_scx_root_warn_restore_bw(int cpu) { pr_warn("failed to re-attach fair_server on CPU %d\n", cpu); }
void lupos_scx_root_log_enabled(struct scx_sched *sch)
{ pr_info("sched_ext: BPF scheduler \"%s\" enabled%s\n", sch->ops.name, scx_switched_all() ? "" : " (partial)"); }
void lupos_scx_root_error_isolation(void)
{ pr_err("sched_ext: Not compatible with \"isolcpus=\" domain isolation\n"); }
bool lupos_scx_root_unlikely_init_error(int ret) { return unlikely(ret); }

int lupos_scx_root_next_possible_cpu(int cpu)
{
#if NR_CPUS == 1
	return cpu < 0 ? 0 : 1;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits, cpu + 1);
#endif
}
unsigned int lupos_scx_root_possible_cpu_limit(void)
{
#if NR_CPUS == 1
	return 1;
#else
	return small_cpumask_bits;
#endif
}
struct rq *lupos_scx_root_cpu_rq(int cpu) { return cpu_rq(cpu); }
void lupos_scx_root_update_rq_clock(struct rq *rq) { update_rq_clock(rq); }
/* Exact native resource envelopes retain their constructor/destructor and
 * DECLARE_LOCK_GUARD attributes. No guard layout crosses the FFI boundary. */
void lupos_scx_root_disable_change(struct task_struct *p, unsigned int queue_flags,
	const struct sched_class *new_class)
{
	scoped_guard (sched_change, p, queue_flags) {
		lupos_scx_root_disable_change_body(p, new_class);
	}
}
void lupos_scx_root_enable_change(struct scx_sched *sch, struct task_struct *p,
	unsigned int queue_flags, const struct sched_class *new_class)
{
	scoped_guard (sched_change, p, queue_flags) {
		lupos_scx_root_enable_change_body(sch, p, new_class);
	}
}
u64 lupos_scx_root_slice_read_once(struct scx_sched *sch) { return READ_ONCE(sch->slice_dfl); }
int lupos_scx_root_attach_bw(struct rq *rq)
{
	int ret;
	scoped_guard(rq_lock_irqsave, rq) {
		ret = lupos_scx_root_attach_bw_body(rq);
	}
	return ret;
}
void lupos_scx_root_restore_bw(struct rq *rq, bool was_switched_all, int cpu)
{
	scoped_guard(rq_lock_irqsave, rq) {
		lupos_scx_root_restore_bw_body(rq, was_switched_all, cpu);
	}
}
void lupos_scx_root_detach_fair(struct rq *rq)
{
	guard(rq_lock_irqsave)(rq);
	lupos_scx_root_detach_fair_body(rq);
}
int lupos_scx_root_dl_attach_ext(struct rq *rq) { return dl_server_attach_bw(&rq->ext_server); }
bool lupos_scx_root_warn_dl_swap(struct rq *rq)
{ return WARN_ON_ONCE(dl_server_swap_bw(&rq->ext_server, &rq->fair_server)); }
void lupos_scx_root_dl_detach_ext(struct rq *rq) { dl_server_detach_bw(&rq->ext_server); }
void lupos_scx_root_dl_detach_fair(struct rq *rq) { dl_server_detach_bw(&rq->fair_server); }
void lupos_scx_root_get_task(struct task_struct *p) { get_task_struct(p); }
void lupos_scx_root_put_task(struct task_struct *p) { put_task_struct(p); }
bool lupos_scx_root_tasks_node_empty(struct task_struct *p) { return list_empty(&p->scx.tasks_node); }
void lupos_scx_root_fork_down_write(void) { percpu_down_write(lupos_scx_core_fork_rwsem()); }
void lupos_scx_root_fork_up_write(void) { percpu_up_write(lupos_scx_core_fork_rwsem()); }
void lupos_scx_root_enable_mutex_lock(void) { mutex_lock(lupos_scx_core_enable_mutex()); }
void lupos_scx_root_enable_mutex_unlock(void) { mutex_unlock(lupos_scx_core_enable_mutex()); }
void lupos_scx_root_cpus_read_lock(void) { cpus_read_lock(); }
void lupos_scx_root_cpus_read_unlock(void) { cpus_read_unlock(); }
void lupos_scx_root_synchronize_rcu(void) { synchronize_rcu(); }
void lupos_scx_root_mark_dead(struct scx_sched *sch) { WRITE_ONCE(sch->dead, true); }
void lupos_scx_root_zero_has_op(struct scx_sched *sch) { bitmap_zero(sch->has_op, SCX_OPI_END); }
bool lupos_scx_root_ops_slot_present(struct sched_ext_ops *ops, int i) { return ((void (**)(void))ops)[i]; }
void lupos_scx_root_set_has_op(struct scx_sched *sch, int i) { set_bit(i, sch->has_op); }
bool lupos_scx_root_switched_all(void) { return scx_switched_all(); }
int lupos_scx_root_tid_hash_init(void) { return rhashtable_init(lupos_scx_core_tid_hash(), lupos_scx_core_tid_hash_params()); }
void lupos_scx_root_tid_hash_free(void) { rhashtable_free_and_destroy(lupos_scx_core_tid_hash(), NULL, NULL); }
bool lupos_scx_root_is_err_sched(struct scx_sched *sch) { return IS_ERR(sch); }
long lupos_scx_root_sched_ptr_err(struct scx_sched *sch) { return PTR_ERR(sch); }
void lupos_scx_root_del_kobj(struct kobject *kobj) { kobject_del(kobj); }
bool lupos_scx_root_in_sysfs(struct scx_sched *sch) { return sch->kobj.state_in_sysfs; }
void lupos_scx_root_uevent_add(struct scx_sched *sch) { kobject_uevent(&sch->kobj, KOBJ_ADD); }
#ifdef CONFIG_EXT_SUB_SCHED
void lupos_scx_root_cgroup_get(struct cgroup *cgrp) { cgroup_get(cgrp); }
struct kobject *lupos_scx_root_sub_kobj(struct scx_sched *sch) { return &sch->sub_kset->kobj; }
void lupos_scx_root_init_sub_work(struct kthread_work *work) { kthread_init_work(work, scx_sub_enable_workfn); }
#endif
void lupos_scx_root_drain_descendants(struct scx_sched *sch) { drain_descendants(sch); }
void lupos_scx_root_set_cgroup_sched(struct scx_sched *sch, struct scx_sched *target)
{ set_cgroup_sched(sch_cgroup(sch), target); }
void lupos_scx_root_discard_stale_ecaps(void) { scx_discard_stale_ecaps_syncs(); }
void lupos_scx_root_rescue_set_knobs(struct scx_sched *sch) { scx_rescue_set_knobs(sch); }
int lupos_scx_root_alloc_pshards(struct scx_sched *sch) { return scx_alloc_pshards(sch); }
void lupos_scx_root_init_caps(struct scx_sched *sch) { scx_init_root_caps(sch); }

bool lupos_scx_root_housekeeping_domain_boot(void) { return housekeeping_enabled(HK_TYPE_DOMAIN_BOOT); }
struct kthread_worker *lupos_scx_root_helper_read_once(struct kthread_worker **helper) { return READ_ONCE(*helper); }
void lupos_scx_root_helper_write_once(struct kthread_worker **helper, struct kthread_worker *w) { WRITE_ONCE(*helper, w); }
void lupos_scx_root_helper_mutex_lock(struct mutex *mutex) { mutex_lock(mutex); }
void lupos_scx_root_helper_mutex_unlock(struct mutex *mutex) { mutex_unlock(mutex); }
struct kthread_worker *lupos_scx_root_run_worker(void) { return kthread_run_worker(0, "scx_enable_helper"); }
bool lupos_scx_root_is_err_or_null_worker(struct kthread_worker *w) { return IS_ERR_OR_NULL(w); }
void lupos_scx_root_helper_fifo(struct kthread_worker *w) { sched_set_fifo(w->task); }
void lupos_scx_root_init_root_work(struct kthread_work *work) { kthread_init_work(work, scx_root_enable_workfn); }
void lupos_scx_root_queue_work(struct kthread_worker *worker, struct kthread_work *work) { kthread_queue_work(worker, work); }
void lupos_scx_root_flush_work(struct kthread_work *work) { kthread_flush_work(work); }

/* This shim delegates to the F00 Rust owner. Exact active WARN text and each
 * distinct native warning site survive; there is no second atomic algorithm. */
static enum scx_enable_state scx_set_enable_state(enum scx_enable_state to)
{ return lupos_scx_root_set_enable_state(to); }
void lupos_scx_root_warn_disabled_without_ops(void)
{ WARN_ON_ONCE(scx_set_enable_state(SCX_DISABLED) != SCX_DISABLING); }
void lupos_scx_root_warn_disabled_at_tail(void)
{ WARN_ON_ONCE(scx_set_enable_state(SCX_DISABLED) != SCX_DISABLING); }
void lupos_scx_root_warn_set_enabling(void)
{ WARN_ON_ONCE(scx_set_enable_state(SCX_ENABLING) != SCX_DISABLED); }
void lupos_scx_root_warn_root_present(void) { WARN_ON_ONCE(scx_root); }
void lupos_scx_root_warn_init_task_enabled(void) { WARN_ON_ONCE(scx_init_task_enabled); }
void lupos_scx_root_warn_cgroup_enabled(void) { WARN_ON_ONCE(scx_cgroup_enabled); }
