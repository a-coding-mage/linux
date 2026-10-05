/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_ROOT_TRANSITION_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_ROOT_TRANSITION_BINDINGS_H
/* F12: configured native types only, in the one future native envelope. */
#include "internal.h"

/* Exact typed Rust continuations; no callback function-pointer casts. */
void lupos_scx_root_enable_work_body(struct kthread_work *work);
s32 lupos_scx_root_enable_body(struct scx_enable_cmd *cmd, struct bpf_link *link,
	struct kthread_worker **helper, struct mutex *helper_mutex);
void lupos_scx_root_disable_change_body(struct task_struct *p, const struct sched_class *new_class);
void lupos_scx_root_enable_change_body(struct scx_sched *sch, struct task_struct *p,
	const struct sched_class *new_class);
int lupos_scx_root_attach_bw_body(struct rq *rq);
void lupos_scx_root_restore_bw_body(struct rq *rq, bool was_switched_all, int cpu);
void lupos_scx_root_detach_fair_body(struct rq *rq);

struct scx_enable_cmd *lupos_scx_root_work_cmd(struct kthread_work *work);
struct sched_ext_ops *lupos_scx_root_cmd_ops(struct scx_enable_cmd *cmd);
struct sched_ext_ops *lupos_scx_root_sched_ops(struct scx_sched *sch);
bool lupos_scx_root_ops_priv_present(struct sched_ext_ops *ops);
bool lupos_scx_root_has_init_cids(struct scx_sched *sch);
int lupos_scx_root_call_init_cids(struct scx_sched *sch);
int lupos_scx_root_call_init(struct scx_sched *sch);
void lupos_scx_root_call_exit(struct scx_sched *sch);
int lupos_scx_root_sanitize_init_cids(struct scx_sched *sch, int ret);
int lupos_scx_root_sanitize_init(struct scx_sched *sch, int ret);
void lupos_scx_root_error_init_cids(struct scx_sched *sch, int ret);
void lupos_scx_root_error_init(struct scx_sched *sch, int ret);
void lupos_scx_root_error_init_task(struct scx_sched *sch, int ret, struct task_struct *p);
void lupos_scx_root_error_enable(struct scx_sched *sch, int ret);
void lupos_scx_root_exit_hotplug(struct scx_sched *sch, const struct sched_ext_ops *ops,
	unsigned long long global_hotplug_seq);
void lupos_scx_root_error_enq_last(struct scx_sched *sch);
void lupos_scx_root_error_tid_dependency(struct scx_sched *sch);
void lupos_scx_root_error_idle_per_node(struct scx_sched *sch);
void lupos_scx_root_warn_deprecated_cpu_ops(void);
void lupos_scx_root_error_sub_cpu_form(struct scx_sched *sch);
void lupos_scx_root_error_attach_cpu_form(struct scx_sched *sch);
void lupos_scx_root_warn_duplicate_disable(void);
void lupos_scx_root_warn_no_ops(struct scx_sched *sch);
void lupos_scx_root_warn_exit_none(struct scx_sched *sch);
void lupos_scx_root_warn_attach_bw(int cpu, int ret);
void lupos_scx_root_warn_restore_bw(int cpu);
void lupos_scx_root_log_enabled(struct scx_sched *sch);
void lupos_scx_root_error_isolation(void);
bool lupos_scx_root_unlikely_init_error(int ret);

int lupos_scx_root_next_possible_cpu(int cpu);
unsigned int lupos_scx_root_possible_cpu_limit(void);
struct rq *lupos_scx_root_cpu_rq(int cpu);
void lupos_scx_root_update_rq_clock(struct rq *rq);
void lupos_scx_root_disable_change(struct task_struct *p, unsigned int queue_flags,
	const struct sched_class *new_class);
void lupos_scx_root_enable_change(struct scx_sched *sch, struct task_struct *p,
	unsigned int queue_flags, const struct sched_class *new_class);
u64 lupos_scx_root_slice_read_once(struct scx_sched *sch);
int lupos_scx_root_attach_bw(struct rq *rq);
void lupos_scx_root_restore_bw(struct rq *rq, bool was_switched_all, int cpu);
void lupos_scx_root_detach_fair(struct rq *rq);
int lupos_scx_root_dl_attach_ext(struct rq *rq);
bool lupos_scx_root_warn_dl_swap(struct rq *rq);
void lupos_scx_root_dl_detach_ext(struct rq *rq);
void lupos_scx_root_dl_detach_fair(struct rq *rq);
void lupos_scx_root_get_task(struct task_struct *p);
void lupos_scx_root_put_task(struct task_struct *p);
bool lupos_scx_root_tasks_node_empty(struct task_struct *p);
void lupos_scx_root_fork_down_write(void);
void lupos_scx_root_fork_up_write(void);
void lupos_scx_root_enable_mutex_lock(void);
void lupos_scx_root_enable_mutex_unlock(void);
void lupos_scx_root_cpus_read_lock(void);
void lupos_scx_root_cpus_read_unlock(void);
void lupos_scx_root_synchronize_rcu(void);
void lupos_scx_root_mark_dead(struct scx_sched *sch);
void lupos_scx_root_zero_has_op(struct scx_sched *sch);
bool lupos_scx_root_ops_slot_present(struct sched_ext_ops *ops, int i);
void lupos_scx_root_set_has_op(struct scx_sched *sch, int i);
bool lupos_scx_root_switched_all(void);
int lupos_scx_root_tid_hash_init(void);
void lupos_scx_root_tid_hash_free(void);
bool lupos_scx_root_is_err_sched(struct scx_sched *sch);
long lupos_scx_root_sched_ptr_err(struct scx_sched *sch);
void lupos_scx_root_del_kobj(struct kobject *kobj);
bool lupos_scx_root_in_sysfs(struct scx_sched *sch);
void lupos_scx_root_uevent_add(struct scx_sched *sch);
#ifdef CONFIG_EXT_SUB_SCHED
void lupos_scx_root_cgroup_get(struct cgroup *cgrp);
struct kobject *lupos_scx_root_sub_kobj(struct scx_sched *sch);
void lupos_scx_root_init_sub_work(struct kthread_work *work);
#endif
/* Config-alternative calls remain with their existing CID/idle/arena/sub owners. */
void lupos_scx_root_drain_descendants(struct scx_sched *sch);
void lupos_scx_root_set_cgroup_sched(struct scx_sched *sch, struct scx_sched *target);
void lupos_scx_root_discard_stale_ecaps(void);
void lupos_scx_root_rescue_set_knobs(struct scx_sched *sch);
int lupos_scx_root_alloc_pshards(struct scx_sched *sch);
void lupos_scx_root_init_caps(struct scx_sched *sch);

bool lupos_scx_root_housekeeping_domain_boot(void);
struct kthread_worker *lupos_scx_root_helper_read_once(struct kthread_worker **helper);
void lupos_scx_root_helper_write_once(struct kthread_worker **helper, struct kthread_worker *w);
void lupos_scx_root_helper_mutex_lock(struct mutex *mutex);
void lupos_scx_root_helper_mutex_unlock(struct mutex *mutex);
struct kthread_worker *lupos_scx_root_run_worker(void);
bool lupos_scx_root_is_err_or_null_worker(struct kthread_worker *w);
void lupos_scx_root_helper_fifo(struct kthread_worker *w);
void lupos_scx_root_init_root_work(struct kthread_work *work);
void lupos_scx_root_queue_work(struct kthread_worker *worker, struct kthread_work *work);
void lupos_scx_root_flush_work(struct kthread_work *work);

enum scx_enable_state lupos_scx_root_set_enable_state(enum scx_enable_state to);
void lupos_scx_root_warn_disabled_without_ops(void);
void lupos_scx_root_warn_disabled_at_tail(void);
void lupos_scx_root_warn_set_enabling(void);
void lupos_scx_root_warn_root_present(void);
void lupos_scx_root_warn_init_task_enabled(void);
void lupos_scx_root_warn_cgroup_enabled(void);

#endif /* LUPOS_SCHED_EXT_CORE_ROOT_TRANSITION_BINDINGS_H */
