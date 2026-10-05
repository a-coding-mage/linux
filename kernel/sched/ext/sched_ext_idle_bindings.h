/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_IDLE_BINDINGS_H
#define LUPOS_SCHED_EXT_IDLE_BINDINGS_H

/* Configured native headers alone own kernel layouts, constants and cfg. */
#include <linux/btf.h>
#include <linux/btf_ids.h>
#include <linux/rhashtable.h>
#include <linux/seq_buf.h>
#include <linux/slab.h>
#include "internal.h"
#include "cid.h"
#include "idle.h"
#include "sub.h"

#define LUPOS_SCX_IDLE_EBUSY EBUSY
#define LUPOS_SCX_IDLE_EINVAL EINVAL
#define LUPOS_SCX_IDLE_ENOENT ENOENT
#define LUPOS_SCX_IDLE_ENODEV ENODEV
#define LUPOS_SCX_IDLE_EOPNOTSUPP EOPNOTSUPP
#define LUPOS_SCX_IDLE_NUMA_NO_NODE NUMA_NO_NODE
#define LUPOS_SCX_IDLE_MAX_NUMNODES MAX_NUMNODES
#define LUPOS_SCX_IDLE_PICK_CORE SCX_PICK_IDLE_CORE
#define LUPOS_SCX_IDLE_PICK_IN_NODE SCX_PICK_IDLE_IN_NODE
#define LUPOS_SCX_IDLE_WAKE_SYNC SCX_WAKE_SYNC
#define LUPOS_SCX_IDLE_OPS_PER_NODE SCX_OPS_BUILTIN_IDLE_PER_NODE
#define LUPOS_SCX_IDLE_OPS_KEEP SCX_OPS_KEEP_BUILTIN_IDLE
#define LUPOS_SCX_IDLE_RENOTIFY_SUB SCX_RQ_SUB_IDLE_RENOTIFY
#define LUPOS_SCX_IDLE_RENOTIFY_ROOT SCX_RQ_ROOT_IDLE_RENOTIFY
#define LUPOS_SCX_IDLE_PF_EXITING PF_EXITING

/* idle.c-local data; neither cpumask_var_t nor per-CPU slots cross by value. */
struct scx_idle_cpus {
	cpumask_var_t cpu;
	cpumask_var_t smt;
};

/* This kfunc argument structure is defined by the pinned idle.c. */
struct scx_bpf_select_cpu_and_args {
	s32 prev_cpu;
	u64 wake_flags;
	u64 flags;
};

struct cpumask *lupos_scx_idle_cpu_mask(int node);
struct cpumask *lupos_scx_idle_smt_mask(int node);
struct cpumask *lupos_scx_idle_local_mask(void);
struct cpumask *lupos_scx_idle_local_llc_mask(void);
struct cpumask *lupos_scx_idle_local_numa_mask(void);
void lupos_scx_idle_alloc_global(void);
void lupos_scx_idle_alloc_nodes(void);
void lupos_scx_idle_alloc_node(int node);
void lupos_scx_idle_alloc_cpu(int cpu);
int lupos_scx_idle_first_node(void);
int lupos_scx_idle_next_node(int node);
unsigned int lupos_scx_idle_nr_nodes(void);
bool lupos_scx_idle_node_possible(int node);
#ifdef CONFIG_NUMA
nodemask_t *lupos_scx_idle_unvisited(void);
void lupos_scx_idle_nodes_online(nodemask_t *mask);
void lupos_scx_idle_node_clear(int node, nodemask_t *mask);
int lupos_scx_idle_nearest_node(int start, nodemask_t *mask);
#endif
unsigned int lupos_scx_idle_nr_cpus(void);
unsigned int lupos_scx_idle_nr_possible(void);
unsigned int lupos_scx_idle_nr_online(void);
const struct cpumask *lupos_scx_idle_online_mask(void);
const struct cpumask *lupos_scx_idle_possible_mask(void);
const struct cpumask *lupos_scx_idle_none_mask(void);
const struct cpumask *lupos_scx_idle_node_mask(int node);
const struct cpumask *lupos_scx_idle_siblings(int cpu);
unsigned int lupos_scx_idle_mask_first(const struct cpumask *mask);
unsigned int lupos_scx_idle_online_scan(unsigned int offset);
unsigned int lupos_scx_idle_possible_scan(unsigned int offset);
unsigned int lupos_scx_idle_cpu_iter_limit(void);
unsigned int lupos_scx_idle_mask_scan_and(unsigned int offset, const struct cpumask *a,
				       const struct cpumask *b);
unsigned int lupos_scx_idle_mask_iter_limit(void);
unsigned int lupos_scx_idle_mask_any(const struct cpumask *mask);
unsigned int lupos_scx_idle_mask_any_and(const struct cpumask *a,
				      const struct cpumask *b);
bool lupos_scx_idle_mask_test(int cpu, const struct cpumask *mask);
bool lupos_scx_idle_mask_claim(int cpu, struct cpumask *mask);
void lupos_scx_idle_mask_clear_cpu(int cpu, struct cpumask *mask);
void lupos_scx_idle_mask_assign(int cpu, struct cpumask *mask, bool value);
void lupos_scx_idle_mask_clear(struct cpumask *mask);
bool lupos_scx_idle_mask_empty(const struct cpumask *mask);
bool lupos_scx_idle_mask_intersects(const struct cpumask *a, const struct cpumask *b);
bool lupos_scx_idle_mask_subset(const struct cpumask *a, const struct cpumask *b);
bool lupos_scx_idle_mask_and(struct cpumask *dst, const struct cpumask *a,
			   const struct cpumask *b);
void lupos_scx_idle_mask_andnot(struct cpumask *dst, const struct cpumask *a,
			      const struct cpumask *b);
void lupos_scx_idle_mask_or(struct cpumask *dst, const struct cpumask *a,
			  const struct cpumask *b);
bool lupos_scx_idle_smt_active(void);
int lupos_scx_idle_cpu_node(int cpu);
int lupos_scx_idle_this_cpu(void);
bool lupos_scx_idle_share_cache(int a, int b);
struct task_struct *lupos_scx_idle_current(void);
struct rq *lupos_scx_idle_cpu_rq(int cpu);
struct rq *lupos_scx_idle_this_rq(void);
struct rq *lupos_scx_idle_task_rq(struct task_struct *p);
int lupos_scx_idle_rq_cpu(struct rq *rq);
struct sched_domain *lupos_scx_idle_llc_domain(int cpu);
struct sched_domain *lupos_scx_idle_numa_domain(int cpu);
struct cpumask *lupos_scx_idle_domain_span(struct sched_domain *sd);
struct cpumask *lupos_scx_idle_group_span(struct sched_group *sg);
void lupos_scx_idle_rcu_lock(void);
void lupos_scx_idle_rcu_unlock(void);
void lupos_scx_idle_preempt_disable(void);
void lupos_scx_idle_preempt_enable(void);
void lupos_scx_idle_assert_rq(struct rq *rq);
void lupos_scx_idle_assert_pi(struct task_struct *p);
unsigned long lupos_scx_idle_lock_pi(struct task_struct *p);
void lupos_scx_idle_unlock_pi(struct task_struct *p, unsigned long flags);
bool lupos_scx_idle_preempt_rcu(void);
/* Separate key uses preserve the source's configured branch modes. */
bool lupos_scx_idle_builtin_likely(void);
bool lupos_scx_idle_per_node_maybe(void);
bool lupos_scx_idle_per_node_likely(void);
bool lupos_scx_idle_per_node_unlikely(void);
bool lupos_scx_idle_llc_maybe(void);
bool lupos_scx_idle_numa_maybe(void);
void lupos_scx_idle_set_builtin(bool enabled);
void lupos_scx_idle_set_per_node(bool enabled);
void lupos_scx_idle_set_llc(bool enabled);
void lupos_scx_idle_set_numa(bool enabled);
void lupos_scx_idle_disable_keys(void);
struct scx_sched *lupos_scx_idle_prog_sched(const struct bpf_prog_aux *aux);
struct scx_sched *lupos_scx_idle_root(void);
struct scx_sched *lupos_scx_idle_live_root(void);
struct scx_sched *lupos_scx_idle_task_sched(struct task_struct *p);
struct rq *lupos_scx_idle_locked_rq(void);
bool lupos_scx_idle_task_ok(struct scx_sched *sch, struct task_struct *p);
bool lupos_scx_idle_cpu_valid(struct scx_sched *sch, int cpu);
bool lupos_scx_idle_has_subs(void);
bool lupos_scx_idle_has_children(struct scx_sched *sch);
bool lupos_scx_idle_missing_base(struct scx_sched *sch, int cpu);
bool lupos_scx_idle_has_update(struct scx_sched *sch);
bool lupos_scx_idle_bypassing(struct scx_sched *sch, int cpu);
bool lupos_scx_idle_take_renotify(struct scx_sched *sch, int cpu);
struct scx_sched *lupos_scx_idle_next_descendant(struct scx_sched *pos, struct scx_sched *root);
struct scx_sched *lupos_scx_idle_skip_subtree(struct scx_sched *pos, struct scx_sched *root);
int lupos_scx_idle_cpu_arg(int cpu);
void lupos_scx_idle_call_update(struct scx_sched *sch, struct rq *rq, int cid, bool idle);
void lupos_scx_idle_debug_llc(int cpu);
void lupos_scx_idle_debug_numa(int cpu, unsigned int weight);
/* Preserve pr_debug's lazy argument evaluation in the native macro. */
unsigned int lupos_scx_idle_llc_weight_rust(int cpu);
const struct cpumask *lupos_scx_idle_llc_span_rust(int cpu);
const struct cpumask *lupos_scx_idle_numa_span_rust(int cpu);
void lupos_scx_idle_debug_keys(bool llc, bool numa);
void lupos_scx_idle_error_disabled(struct scx_sched *sch);
void lupos_scx_idle_error_per_node_disabled(struct scx_sched *sch);
void lupos_scx_idle_error_per_node_enabled(struct scx_sched *sch);
void lupos_scx_idle_error_per_node_mask(struct scx_sched *sch);
void lupos_scx_idle_error_invalid_node(struct scx_sched *sch, int node);
void lupos_scx_idle_error_unavailable_node(struct scx_sched *sch, int node);
void lupos_scx_idle_error_cross_task(struct scx_sched *sch, struct task_struct *p);
void lupos_scx_idle_error_compat(struct task_struct *p);
/* One native unlikely callsite for each original kfunc. */
bool lupos_scx_idle_unlikely_cpu_node(bool value);
bool lupos_scx_idle_unlikely_select_dfl(bool value);
bool lupos_scx_idle_unlikely_select_and(bool value);
bool lupos_scx_idle_unlikely_select_compat(bool value);
bool lupos_scx_idle_unlikely_children(bool value);
bool lupos_scx_idle_unlikely_get_node(bool value);
bool lupos_scx_idle_unlikely_get(bool value);
bool lupos_scx_idle_unlikely_smt_node(bool value);
bool lupos_scx_idle_unlikely_smt(bool value);
bool lupos_scx_idle_unlikely_claim(bool value);
bool lupos_scx_idle_unlikely_pick_node(bool value);
bool lupos_scx_idle_unlikely_pick(bool value);
bool lupos_scx_idle_unlikely_any_node(bool value);
bool lupos_scx_idle_unlikely_any(bool value);
int lupos_scx_idle_register_ops(void);
int lupos_scx_idle_register_tracing(void);
int lupos_scx_idle_register_syscall(void);
int lupos_scx_idle_register_select_ops(void);
int lupos_scx_idle_register_select_syscall(void);

/* BPF-facing native metadata thunks call these Rust continuations. */
s32 lupos_scx_idle_bpf_cpu_node(s32 cpu, const struct bpf_prog_aux *aux);
s32 lupos_scx_idle_bpf_select_cpu_dfl(struct task_struct *p, s32 prev_cpu,
				   u64 wake_flags, bool *is_idle, const struct bpf_prog_aux *aux);
s32 lupos_scx_idle_bpf_select_cpu_and(struct task_struct *p, const struct cpumask *allowed,
				   struct scx_bpf_select_cpu_and_args *args,
				   const struct bpf_prog_aux *aux);
s32 lupos_scx_idle_bpf_select_cpu_compat(struct task_struct *p, s32 prev_cpu,
				      u64 wake_flags, const struct cpumask *allowed, u64 flags);
const struct cpumask *lupos_scx_idle_bpf_get_idle_cpumask_node(s32 node, const struct bpf_prog_aux *aux);
const struct cpumask *lupos_scx_idle_bpf_get_idle_cpumask(const struct bpf_prog_aux *aux);
const struct cpumask *lupos_scx_idle_bpf_get_idle_smtmask_node(s32 node, const struct bpf_prog_aux *aux);
const struct cpumask *lupos_scx_idle_bpf_get_idle_smtmask(const struct bpf_prog_aux *aux);
void lupos_scx_idle_bpf_put_idle_cpumask(const struct cpumask *mask);
bool lupos_scx_idle_bpf_test_and_clear_cpu_idle(s32 cpu, const struct bpf_prog_aux *aux);
s32 lupos_scx_idle_bpf_pick_idle_cpu_node(const struct cpumask *allowed, s32 node,
				      u64 flags, const struct bpf_prog_aux *aux);
s32 lupos_scx_idle_bpf_pick_idle_cpu(const struct cpumask *allowed, u64 flags,
				 const struct bpf_prog_aux *aux);
s32 lupos_scx_idle_bpf_pick_any_cpu_node(const struct cpumask *allowed, s32 node,
				     u64 flags, const struct bpf_prog_aux *aux);
s32 lupos_scx_idle_bpf_pick_any_cpu(const struct cpumask *allowed, u64 flags,
				const struct bpf_prog_aux *aux);
#endif /* LUPOS_SCHED_EXT_IDLE_BINDINGS_H */
