// SPDX-License-Identifier: GPL-2.0
/* Idle-local storage and native primitive leaves. Unqualified C runtime
 * boundary: no original idle.c/ext.c algorithm fallback or admission. */
#error "SOURCE ONLY HOLD: sched_ext idle native ABI/protection admission is pending"
#include "sched_ext_idle_bindings.h"

static DEFINE_STATIC_KEY_FALSE(scx_builtin_idle_enabled);
static DEFINE_STATIC_KEY_FALSE(scx_builtin_idle_per_node);
static DEFINE_STATIC_KEY_FALSE(scx_selcpu_topo_llc);
static DEFINE_STATIC_KEY_FALSE(scx_selcpu_topo_numa);
static struct scx_idle_cpus scx_idle_global_masks;
static struct scx_idle_cpus **scx_idle_node_masks;
static DEFINE_PER_CPU(cpumask_var_t, local_idle_cpumask);
static DEFINE_PER_CPU(cpumask_var_t, local_llc_idle_cpumask);
static DEFINE_PER_CPU(cpumask_var_t, local_numa_idle_cpumask);
#ifdef CONFIG_NUMA
static DEFINE_PER_CPU(nodemask_t, per_cpu_unvisited);
#endif

struct cpumask *lupos_scx_idle_cpu_mask(int node)
{
	return node == NUMA_NO_NODE ? scx_idle_global_masks.cpu : scx_idle_node_masks[node]->cpu;
}
struct cpumask *lupos_scx_idle_smt_mask(int node)
{
	return node == NUMA_NO_NODE ? scx_idle_global_masks.smt : scx_idle_node_masks[node]->smt;
}
struct cpumask *lupos_scx_idle_local_mask(void) { return this_cpu_cpumask_var_ptr(local_idle_cpumask); }
struct cpumask *lupos_scx_idle_local_llc_mask(void) { return this_cpu_cpumask_var_ptr(local_llc_idle_cpumask); }
struct cpumask *lupos_scx_idle_local_numa_mask(void) { return this_cpu_cpumask_var_ptr(local_numa_idle_cpumask); }
void lupos_scx_idle_alloc_global(void)
{
	BUG_ON(!alloc_cpumask_var(&scx_idle_global_masks.cpu, GFP_KERNEL));
	BUG_ON(!alloc_cpumask_var(&scx_idle_global_masks.smt, GFP_KERNEL));
}
void lupos_scx_idle_alloc_nodes(void)
{
	scx_idle_node_masks = kzalloc_objs(*scx_idle_node_masks, nr_node_ids);
	BUG_ON(!scx_idle_node_masks);
}
void lupos_scx_idle_alloc_node(int node)
{
	scx_idle_node_masks[node] = kzalloc_node(sizeof(**scx_idle_node_masks), GFP_KERNEL, node);
	BUG_ON(!scx_idle_node_masks[node]);
	BUG_ON(!alloc_cpumask_var_node(&scx_idle_node_masks[node]->cpu, GFP_KERNEL, node));
	BUG_ON(!alloc_cpumask_var_node(&scx_idle_node_masks[node]->smt, GFP_KERNEL, node));
}
void lupos_scx_idle_alloc_cpu(int cpu)
{
	BUG_ON(!alloc_cpumask_var_node(&per_cpu(local_idle_cpumask, cpu), GFP_KERNEL, cpu_to_node(cpu)));
	BUG_ON(!alloc_cpumask_var_node(&per_cpu(local_llc_idle_cpumask, cpu), GFP_KERNEL, cpu_to_node(cpu)));
	BUG_ON(!alloc_cpumask_var_node(&per_cpu(local_numa_idle_cpumask, cpu), GFP_KERNEL, cpu_to_node(cpu)));
}
int lupos_scx_idle_first_node(void)
{
#if MAX_NUMNODES > 1
	return first_node(node_states[N_POSSIBLE]);
#else
	return 0;
#endif
}
int lupos_scx_idle_next_node(int node)
{
#if MAX_NUMNODES > 1
	return next_node(node, node_states[N_POSSIBLE]);
#else
	return 1;
#endif
}
unsigned int lupos_scx_idle_nr_nodes(void) { return nr_node_ids; }
bool lupos_scx_idle_node_possible(int node) { return node_possible(node); }
#ifdef CONFIG_NUMA
nodemask_t *lupos_scx_idle_unvisited(void) { return this_cpu_ptr(&per_cpu_unvisited); }
void lupos_scx_idle_nodes_online(nodemask_t *mask) { nodes_copy(*mask, node_states[N_ONLINE]); }
void lupos_scx_idle_node_clear(int node, nodemask_t *mask) { node_clear(node, *mask); }
int lupos_scx_idle_nearest_node(int start, nodemask_t *mask) { return nearest_node_nodemask(start, mask); }
#endif
unsigned int lupos_scx_idle_nr_cpus(void) { return nr_cpu_ids; }
unsigned int lupos_scx_idle_nr_possible(void) { return num_possible_cpus(); }
unsigned int lupos_scx_idle_nr_online(void) { return num_online_cpus(); }
const struct cpumask *lupos_scx_idle_online_mask(void) { return cpu_online_mask; }
const struct cpumask *lupos_scx_idle_possible_mask(void) { return cpu_possible_mask; }
const struct cpumask *lupos_scx_idle_none_mask(void) { return cpu_none_mask; }
const struct cpumask *lupos_scx_idle_node_mask(int node) { return cpumask_of_node(node); }
const struct cpumask *lupos_scx_idle_siblings(int cpu) { return cpu_smt_mask(cpu); }
unsigned int lupos_scx_idle_mask_first(const struct cpumask *mask) { return cpumask_first(mask); }
/* Exact for_each_{online,possible}_cpu specializations and scan bound. */
unsigned int lupos_scx_idle_online_scan(unsigned int offset)
{
#if NR_CPUS == 1
	return offset;
#else
	return find_next_bit(cpumask_bits(cpu_online_mask), small_cpumask_bits, offset);
#endif
}
unsigned int lupos_scx_idle_possible_scan(unsigned int offset)
{
#if NR_CPUS == 1
	return offset;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits, offset);
#endif
}
unsigned int lupos_scx_idle_cpu_iter_limit(void)
{
#if NR_CPUS == 1
	return 1;
#else
	return small_cpumask_bits;
#endif
}
/* for_each_cpu_and uses find_next_and_bit, without cpumask_check. */
unsigned int lupos_scx_idle_mask_scan_and(unsigned int offset,
				       const struct cpumask *a, const struct cpumask *b)
{
	return find_next_and_bit(cpumask_bits(a), cpumask_bits(b), small_cpumask_bits, offset);
}
unsigned int lupos_scx_idle_mask_iter_limit(void) { return small_cpumask_bits; }
unsigned int lupos_scx_idle_mask_any(const struct cpumask *mask) { return cpumask_any_distribute(mask); }
unsigned int lupos_scx_idle_mask_any_and(const struct cpumask *a, const struct cpumask *b) { return cpumask_any_and_distribute(a, b); }
bool lupos_scx_idle_mask_test(int cpu, const struct cpumask *mask) { return cpumask_test_cpu(cpu, mask); }
bool lupos_scx_idle_mask_claim(int cpu, struct cpumask *mask) { return cpumask_test_and_clear_cpu(cpu, mask); }
void lupos_scx_idle_mask_clear_cpu(int cpu, struct cpumask *mask) { __cpumask_clear_cpu(cpu, mask); }
void lupos_scx_idle_mask_assign(int cpu, struct cpumask *mask, bool value) { assign_cpu(cpu, mask, value); }
void lupos_scx_idle_mask_clear(struct cpumask *mask) { cpumask_clear(mask); }
bool lupos_scx_idle_mask_empty(const struct cpumask *mask) { return cpumask_empty(mask); }
bool lupos_scx_idle_mask_intersects(const struct cpumask *a, const struct cpumask *b) { return cpumask_intersects(a, b); }
bool lupos_scx_idle_mask_subset(const struct cpumask *a, const struct cpumask *b) { return cpumask_subset(a, b); }
bool lupos_scx_idle_mask_and(struct cpumask *dst, const struct cpumask *a, const struct cpumask *b) { return cpumask_and(dst, a, b); }
void lupos_scx_idle_mask_andnot(struct cpumask *dst, const struct cpumask *a, const struct cpumask *b) { cpumask_andnot(dst, a, b); }
void lupos_scx_idle_mask_or(struct cpumask *dst, const struct cpumask *a, const struct cpumask *b) { cpumask_or(dst, a, b); }
bool lupos_scx_idle_smt_active(void) { return sched_smt_active(); }
int lupos_scx_idle_cpu_node(int cpu) { return cpu_to_node(cpu); }
int lupos_scx_idle_this_cpu(void) { return smp_processor_id(); }
bool lupos_scx_idle_share_cache(int a, int b) { return cpus_share_cache(a, b); }
struct task_struct *lupos_scx_idle_current(void) { return current; }
struct rq *lupos_scx_idle_cpu_rq(int cpu) { return cpu_rq(cpu); }
struct rq *lupos_scx_idle_this_rq(void) { return this_rq(); }
struct rq *lupos_scx_idle_task_rq(struct task_struct *p) { return task_rq(p); }
int lupos_scx_idle_rq_cpu(struct rq *rq) { return cpu_of(rq); }
struct sched_domain *lupos_scx_idle_llc_domain(int cpu) { return rcu_dereference(per_cpu(sd_llc, cpu)); }
struct sched_domain *lupos_scx_idle_numa_domain(int cpu) { return rcu_dereference(per_cpu(sd_numa, cpu)); }
struct cpumask *lupos_scx_idle_domain_span(struct sched_domain *sd) { return sched_domain_span(sd); }
struct cpumask *lupos_scx_idle_group_span(struct sched_group *sg) { return sched_group_span(sg); }
void lupos_scx_idle_rcu_lock(void) { rcu_read_lock(); }
void lupos_scx_idle_rcu_unlock(void) { rcu_read_unlock(); }
void lupos_scx_idle_preempt_disable(void) { preempt_disable(); }
void lupos_scx_idle_preempt_enable(void) { preempt_enable(); }
void lupos_scx_idle_assert_rq(struct rq *rq) { lockdep_assert_rq_held(rq); }
void lupos_scx_idle_assert_pi(struct task_struct *p) { lockdep_assert_held(&p->pi_lock); }
unsigned long lupos_scx_idle_lock_pi(struct task_struct *p)
{
	unsigned long flags;
	raw_spin_lock_irqsave(&p->pi_lock, flags);
	return flags;
}
void lupos_scx_idle_unlock_pi(struct task_struct *p, unsigned long flags) { raw_spin_unlock_irqrestore(&p->pi_lock, flags); }
bool lupos_scx_idle_preempt_rcu(void) { return IS_ENABLED(CONFIG_PREEMPT_RCU); }
bool lupos_scx_idle_builtin_likely(void) { return static_branch_likely(&scx_builtin_idle_enabled); }
bool lupos_scx_idle_per_node_maybe(void) { return static_branch_maybe(CONFIG_NUMA, &scx_builtin_idle_per_node); }
bool lupos_scx_idle_per_node_likely(void) { return static_branch_likely(&scx_builtin_idle_per_node); }
bool lupos_scx_idle_per_node_unlikely(void) { return static_branch_unlikely(&scx_builtin_idle_per_node); }
bool lupos_scx_idle_llc_maybe(void) { return static_branch_maybe(CONFIG_SCHED_MC, &scx_selcpu_topo_llc); }
bool lupos_scx_idle_numa_maybe(void) { return static_branch_maybe(CONFIG_NUMA, &scx_selcpu_topo_numa); }
#define LUPOS_IDLE_SET_KEY(name, key) \
void name(bool enabled) \
{ \
	if (enabled) \
		static_branch_enable_cpuslocked(&key); \
	else \
		static_branch_disable_cpuslocked(&key); \
}
LUPOS_IDLE_SET_KEY(lupos_scx_idle_set_builtin, scx_builtin_idle_enabled)
LUPOS_IDLE_SET_KEY(lupos_scx_idle_set_per_node, scx_builtin_idle_per_node)
LUPOS_IDLE_SET_KEY(lupos_scx_idle_set_llc, scx_selcpu_topo_llc)
LUPOS_IDLE_SET_KEY(lupos_scx_idle_set_numa, scx_selcpu_topo_numa)
#undef LUPOS_IDLE_SET_KEY
void lupos_scx_idle_disable_keys(void)
{
	static_branch_disable(&scx_builtin_idle_enabled);
	static_branch_disable(&scx_builtin_idle_per_node);
}
struct scx_sched *lupos_scx_idle_prog_sched(const struct bpf_prog_aux *aux) { return scx_prog_sched(aux); }
struct scx_sched *lupos_scx_idle_root(void) { return rcu_dereference(scx_root); }
struct scx_sched *lupos_scx_idle_live_root(void) { return scx_root_protected_live(); }
struct scx_sched *lupos_scx_idle_task_sched(struct task_struct *p) { return scx_task_sched(p); }
struct rq *lupos_scx_idle_locked_rq(void) { return scx_locked_rq(); }
bool lupos_scx_idle_task_ok(struct scx_sched *sch, struct task_struct *p) { return scx_kf_arg_task_ok(sch, p); }
bool lupos_scx_idle_cpu_valid(struct scx_sched *sch, int cpu) { return scx_cpu_valid(sch, cpu, NULL); }
bool lupos_scx_idle_has_subs(void) { return scx_has_subs(); }
bool lupos_scx_idle_has_children(struct scx_sched *sch)
{
#ifdef CONFIG_EXT_SUB_SCHED
	return !list_empty(&sch->children);
#else
	return false;
#endif
}
bool lupos_scx_idle_missing_base(struct scx_sched *sch, int cpu) { return unlikely(scx_missing_caps(sch, cpu, SCX_CAP_BASE)); }
bool lupos_scx_idle_has_update(struct scx_sched *sch) { return SCX_HAS_OP(sch, update_idle); }
bool lupos_scx_idle_bypassing(struct scx_sched *sch, int cpu) { return scx_bypassing(sch, cpu); }
bool lupos_scx_idle_take_renotify(struct scx_sched *sch, int cpu)
{
#ifdef CONFIG_EXT_SUB_SCHED
	bool owed = per_cpu_ptr(sch->pcpu, cpu)->idle_renotify;
	if (owed)
		per_cpu_ptr(sch->pcpu, cpu)->idle_renotify = false;
	return owed;
#else
	return false;
#endif
}
struct scx_sched *lupos_scx_idle_next_descendant(struct scx_sched *pos, struct scx_sched *root) { return scx_next_descendant_pre(pos, root); }
struct scx_sched *lupos_scx_idle_skip_subtree(struct scx_sched *pos, struct scx_sched *root) { return scx_skip_subtree_pre(pos, root); }
int lupos_scx_idle_cpu_arg(int cpu) { return scx_cpu_arg(cpu); }
void lupos_scx_idle_call_update(struct scx_sched *sch, struct rq *rq, int cid, bool idle)
{
	SCX_CALL_OP(sch, update_idle, rq, cid, idle);
}
void lupos_scx_idle_debug_llc(int cpu)
{
	pr_debug("sched_ext: LLC=%*pb weight=%u\n",
		 cpumask_pr_args(lupos_scx_idle_llc_span_rust(cpu)),
		 lupos_scx_idle_llc_weight_rust(cpu));
}
void lupos_scx_idle_debug_numa(int cpu, unsigned int weight)
{
	pr_debug("sched_ext: NUMA=%*pb weight=%u\n",
		 cpumask_pr_args(lupos_scx_idle_numa_span_rust(cpu)), weight);
}
void lupos_scx_idle_debug_keys(bool llc, bool numa)
{
	pr_debug("sched_ext: LLC idle selection %s\n", str_enabled_disabled(llc));
	pr_debug("sched_ext: NUMA idle selection %s\n", str_enabled_disabled(numa));
}
void lupos_scx_idle_error_disabled(struct scx_sched *sch) { scx_error(sch, "built-in idle tracking is disabled"); }
void lupos_scx_idle_error_per_node_disabled(struct scx_sched *sch) { scx_error(sch, "per-node idle tracking is disabled"); }
void lupos_scx_idle_error_per_node_enabled(struct scx_sched *sch) { scx_error(sch, "per-node idle tracking is enabled"); }
void lupos_scx_idle_error_per_node_mask(struct scx_sched *sch) { scx_error(sch, "SCX_OPS_BUILTIN_IDLE_PER_NODE enabled"); }
void lupos_scx_idle_error_invalid_node(struct scx_sched *sch, int node) { scx_error(sch, "invalid node %d", node); }
void lupos_scx_idle_error_unavailable_node(struct scx_sched *sch, int node) { scx_error(sch, "unavailable node %d", node); }
void lupos_scx_idle_error_cross_task(struct scx_sched *sch, struct task_struct *p) { scx_error(sch, "select_cpu kfunc called cross-task on %s[%d]", p->comm, p->pid); }
void lupos_scx_idle_error_compat(struct task_struct *p) { scx_error(scx_task_sched(p), "__scx_bpf_select_cpu_and() must be used"); }
#define LUPOS_IDLE_UNLIKELY(name) bool name(bool value) { return unlikely(value); }
LUPOS_IDLE_UNLIKELY(lupos_scx_idle_unlikely_cpu_node)
LUPOS_IDLE_UNLIKELY(lupos_scx_idle_unlikely_select_dfl)
LUPOS_IDLE_UNLIKELY(lupos_scx_idle_unlikely_select_and)
LUPOS_IDLE_UNLIKELY(lupos_scx_idle_unlikely_select_compat)
LUPOS_IDLE_UNLIKELY(lupos_scx_idle_unlikely_children)
LUPOS_IDLE_UNLIKELY(lupos_scx_idle_unlikely_get_node)
LUPOS_IDLE_UNLIKELY(lupos_scx_idle_unlikely_get)
LUPOS_IDLE_UNLIKELY(lupos_scx_idle_unlikely_smt_node)
LUPOS_IDLE_UNLIKELY(lupos_scx_idle_unlikely_smt)
LUPOS_IDLE_UNLIKELY(lupos_scx_idle_unlikely_claim)
LUPOS_IDLE_UNLIKELY(lupos_scx_idle_unlikely_pick_node)
LUPOS_IDLE_UNLIKELY(lupos_scx_idle_unlikely_pick)
LUPOS_IDLE_UNLIKELY(lupos_scx_idle_unlikely_any_node)
LUPOS_IDLE_UNLIKELY(lupos_scx_idle_unlikely_any)
#undef LUPOS_IDLE_UNLIKELY
