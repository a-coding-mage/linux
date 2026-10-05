// SPDX-License-Identifier: GPL-2.0
/* F16 native kfunc ABI, scoped protection and narrow primitive leaves.
 * Unqualified C runtime; no old ext.c algorithm fallback or independent object.
 * Same future build_policy translation-unit envelope and F00 storage required. */
#error "SOURCE ONLY HOLD: sched_ext query/event native ABI and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_query_events_bindings.h"

/* Pinned C's group starts at ext.c:9958 and ends at 10658, crossing F11/F16.
 * This balanced local scope applies only to exact native kfunc wrappers. F00
 * must reconcile the original same-TU scope before admission. No new warning
 * exemption is applied to native primitives or typed Rust body declarations. */
__bpf_kfunc_start_defs();

__bpf_kfunc u32 scx_bpf_cpuperf_cap(s32 cpu, const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	return lupos_scx_core_qe_cpuperf_cap_body(cpu, aux);
}

__bpf_kfunc u32 scx_bpf_cidperf_cap(s32 cid, const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	return lupos_scx_core_qe_cidperf_cap_body(cid, aux);
}

__bpf_kfunc u32 scx_bpf_cpuperf_cur(s32 cpu, const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	return lupos_scx_core_qe_cpuperf_cur_body(cpu, aux);
}

__bpf_kfunc u32 scx_bpf_cidperf_cur(s32 cid, const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	return lupos_scx_core_qe_cidperf_cur_body(cid, aux);
}

__bpf_kfunc void scx_bpf_cpuperf_set(s32 cpu, u32 perf, const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	lupos_scx_core_qe_cpuperf_set_body(cpu, perf, aux);
}

__bpf_kfunc s32 scx_bpf_cidperf_set(s32 cid, u32 perf,
                                    const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	return lupos_scx_core_qe_cidperf_set_body(cid, perf, aux);
}

__bpf_kfunc u32 scx_bpf_nr_node_ids(void)
{ return lupos_scx_core_qe_nr_node_ids_body(); }

__bpf_kfunc u32 scx_bpf_nr_cpu_ids(void)
{ return lupos_scx_core_qe_nr_cpu_ids_body(); }

__bpf_kfunc u32 scx_bpf_nr_cids(void)
{ return lupos_scx_core_qe_nr_cids_body(); }

__bpf_kfunc u32 scx_bpf_nr_online_cids(void)
{ return lupos_scx_core_qe_nr_online_cids_body(); }

__bpf_kfunc s32 scx_bpf_this_cid(void)
{
	guard(rcu)();
	return lupos_scx_core_qe_this_cid_body();
}

__bpf_kfunc const struct cpumask *scx_bpf_get_possible_cpumask(void)
{ return lupos_scx_core_qe_get_possible_cpumask_body(); }

__bpf_kfunc const struct cpumask *scx_bpf_get_online_cpumask(void)
{ return lupos_scx_core_qe_get_online_cpumask_body(); }

__bpf_kfunc void scx_bpf_put_cpumask(const struct cpumask *cpumask)
{ lupos_scx_core_qe_put_cpumask_body(cpumask); }

__bpf_kfunc bool scx_bpf_task_running(const struct task_struct *p)
{ return lupos_scx_core_qe_task_running_body(p); }

__bpf_kfunc s32 scx_bpf_task_cpu(const struct task_struct *p)
{ return lupos_scx_core_qe_task_cpu_body(p); }

__bpf_kfunc s32 scx_bpf_task_cid(const struct task_struct *p)
{
	/* KF_RCU covers only p; even a sleepable program needs this table guard. */
	guard(rcu)();
	return lupos_scx_core_qe_task_cid_body(p);
}

__bpf_kfunc struct rq *scx_bpf_locked_rq(const struct bpf_prog_aux *aux)
{
	guard(preempt)();
	return lupos_scx_core_qe_locked_rq_body(aux);
}

__bpf_kfunc struct task_struct *scx_bpf_cpu_curr(s32 cpu, const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	return lupos_scx_core_qe_cpu_curr_body(cpu, aux);
}

__bpf_kfunc struct task_struct *scx_bpf_cid_curr(s32 cid, const struct bpf_prog_aux *aux)
{
	guard(rcu)();
	return lupos_scx_core_qe_cid_curr_body(cid, aux);
}

__bpf_kfunc struct task_struct *scx_bpf_tid_to_task(u64 tid)
{
	/* The original relies on caller KF_RCU_PROTECTED, without a new guard. */
	return lupos_scx_core_qe_tid_to_task_body(tid);
}

__bpf_kfunc u64 scx_bpf_now(void)
{
	guard(preempt)();
	return lupos_scx_core_qe_now_body();
}

__bpf_kfunc void scx_bpf_events(struct scx_event_stats *events, size_t events__sz,
                                const struct bpf_prog_aux *aux)
{
	/* events__sz is exact verifier-recognized spelling; no nullable annotation
	 * or arena address-space annotation exists on this original function. */
	lupos_scx_core_qe_events_body(events, events__sz, aux);
}

#ifdef CONFIG_CGROUP_SCHED
__bpf_kfunc struct cgroup *scx_bpf_task_cgroup(struct task_struct *p,
                                               const struct bpf_prog_aux *aux)
{
	/* These original local snapshots precede guard(rcu), including task-group
	 * evaluation. Never recompute p->sched_task_group inside the Rust body. */
	struct task_group *tg = p->sched_task_group;
	struct cgroup *cgrp = &cgrp_dfl_root.cgrp;

	guard(rcu)();
	return lupos_scx_core_qe_task_cgroup_body(p, aux, tg, cgrp);
}
#endif /* CONFIG_CGROUP_SCHED */

__bpf_kfunc_end_defs();

/* Native header-owned helpers retain their own configured bodies. Calling
 * them here does not claim their algorithms as F16 Rust coverage. */
struct scx_sched *lupos_scx_core_qe_prog_sched(const struct bpf_prog_aux *aux)
{ return scx_prog_sched(aux); }
bool lupos_scx_core_qe_likely_sched(struct scx_sched *sch)
{ return likely(sch); }
bool lupos_scx_core_qe_unlikely_no_sched(struct scx_sched *sch)
{ return unlikely(!sch); }
u32 lupos_scx_core_qe_arch_capacity(s32 cpu)
{ return arch_scale_cpu_capacity(cpu); }
u32 lupos_scx_core_qe_arch_frequency(s32 cpu)
{ return arch_scale_freq_capacity(cpu); }
s32 lupos_scx_core_qe_cid_to_cpu(struct scx_sched *sch, s32 cid)
{ return scx_cid_to_cpu(sch, cid); }
bool lupos_scx_core_qe_unlikely_bad_perf(u32 perf)
{ return unlikely(perf > SCX_CPUPERF_ONE); }
void lupos_scx_core_qe_error_perf(struct scx_sched *sch, u32 perf, s32 cpu)
{ scx_error(sch, "Invalid cpuperf target %u for CPU %d", perf, cpu); }
void lupos_scx_core_qe_error_target(struct scx_sched *sch, s32 cpu)
{ scx_error(sch, "Invalid target CPU %d", cpu); }
struct rq *lupos_scx_core_qe_cpu_rq(s32 cpu) { return cpu_rq(cpu); }

s32 lupos_scx_core_qe_cpuperf_unlocked(struct scx_sched *sch, s32 cpu,
                                     u32 perf, struct rq *rq)
{
	struct rq_flags rf;
	s32 ret;

	/* ext.c:10197-10200/10216-10217 are an explicit ordinary lock pair,
	 * not any scoped counted-IRQ guard. Keep its exact native rq_flags and
	 * rq pin/unpin machinery; no invented Rust flags or lock object. */
	rq_lock_irqsave(rq, &rf);
	update_rq_clock(rq);
	ret = lupos_scx_core_qe_cpuperf_locked_body(sch, cpu, perf, rq);
	rq_unlock_irqrestore(rq, &rf);
	return ret;
}

bool lupos_scx_core_qe_likely_perf_allowed(struct scx_sched *sch, s32 cpu)
{ return likely(!scx_missing_caps(sch, cpu, SCX_CAP_PERF)); }
/* ext.c:10208, paired with ext.h:28's plain native reader. The target rq
 * lock does not exclude schedutil's sibling-CPU policy-lock reader. Preserve
 * the exact store before cpufreq_update_util; native race qualification stays
 * held, without stronger ordering, a new lock or an old-owner algorithm. */
void lupos_scx_core_qe_cpuperf_target_write(struct rq *rq, u32 perf)
{ rq->scx.cpuperf_target = perf; }
void lupos_scx_core_qe_cpufreq_update(struct rq *rq)
{ cpufreq_update_util(rq, 0); }
void lupos_scx_core_qe_event_perf_denied(struct scx_sched *sch)
{ __scx_add_event(sch, SCX_EV_SUB_CIDPERF_DENIED, 1); }
u32 lupos_scx_core_qe_nr_node_ids(void) { return nr_node_ids; }
unsigned int lupos_scx_core_qe_num_online_cpus(void) { return num_online_cpus(); }
/* Separate original RCU macro sites retain separate lockdep warning state. */
s16 *lupos_scx_core_qe_this_cid_table_rcu(void)
{ return rcu_dereference(scx_cpu_to_cid_tbl); }
s16 *lupos_scx_core_qe_task_cid_table_rcu(void)
{ return rcu_dereference(scx_cpu_to_cid_tbl); }
int lupos_scx_core_qe_raw_cpu(void) { return raw_smp_processor_id(); }
const struct cpumask *lupos_scx_core_qe_possible_cpumask(void)
{ return cpu_possible_mask; }
const struct cpumask *lupos_scx_core_qe_online_cpumask(void)
{ return cpu_online_mask; }
struct task_struct *lupos_scx_core_qe_task_rq_curr_plain(const struct task_struct *p)
{
	/* Native sched.h owns CONFIG_SCHED_PROXY_EXEC union/field alternatives.
	 * The original task-running query reads curr plainly, not READ_ONCE. */
	return task_rq(p)->curr;
}
unsigned int lupos_scx_core_qe_task_cpu(const struct task_struct *p)
{ return task_cpu(p); }
void lupos_scx_core_qe_error_unlocked_rq(struct scx_sched *sch)
{ scx_error(sch, "accessing rq without holding rq lock"); }
struct task_struct *lupos_scx_core_qe_cpu_curr_rcu(s32 cpu)
{ return rcu_dereference(cpu_rq(cpu)->curr); }
/* Do not merge this original CID-query RCU warning site with the CPU query. */
struct task_struct *lupos_scx_core_qe_cid_curr_rcu(s32 cpu)
{ return rcu_dereference(cpu_rq(cpu)->curr); }
struct scx_sched *lupos_scx_core_qe_root_rcu(void)
{ return rcu_dereference(scx_root); }
void lupos_scx_core_qe_error_tid_disabled(struct scx_sched *sch)
{ scx_error(sch, "scx_bpf_tid_to_task() called without SCX_OPS_TID_TO_TASK"); }
struct task_struct *lupos_scx_core_qe_entity_task(struct sched_ext_entity *scx)
{ return container_of(scx, struct task_struct, scx); }
void lupos_scx_core_qe_assert_now(struct rq *rq)
{
	/* Keep exact native diagnostic expression, short-circuit evaluations and
	 * this site's WARN identity. A bool argument would alter the diagnostic. */
	lockdep_assert((rq == this_rq() && !preemptible()) ||
		       lockdep_is_held(__rq_lockp(rq)));
}
u32 lupos_scx_core_qe_flags_acquire(struct rq *rq)
{ return smp_load_acquire(&rq->scx.flags); }
u64 lupos_scx_core_qe_clock_read_once(struct rq *rq)
{ return READ_ONCE(rq->scx.clock); }
u64 lupos_scx_core_qe_sched_clock_cpu(struct rq *rq)
{ return sched_clock_cpu(cpu_of(rq)); }
struct rq *lupos_scx_core_qe_this_rq(void) { return this_rq(); }
void lupos_scx_core_qe_zero_events(struct scx_event_stats *events)
{ memset(events, 0, sizeof(*events)); }

bool lupos_scx_core_qe_possible_cpu_condition(int *cpu)
{
	/* cpumask.h:1131/1140 and find.h:583-584 exact condition. Rust initializes
	 * cpu=0 and increments int after each body. cpumask_next is not equivalent
	 * to this tree's find_next_bit(..., small_cpumask_bits, ...) expansion. */
#if NR_CPUS == 1
	return *cpu < 1;
#else
	*cpu = find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits, *cpu);
	return *cpu < small_cpumask_bits;
#endif
}

struct scx_event_stats *lupos_scx_core_qe_cpu_events(struct scx_sched *sch, int cpu)
{ return &per_cpu_ptr(sch->pcpu, cpu)->event_stats; }
s64 lupos_scx_core_qe_read_event(const s64 *counter)
{ return READ_ONCE(*counter); }

void lupos_scx_core_qe_events_rcu(struct scx_event_stats *e_sys,
                                const struct bpf_prog_aux *aux)
{
	/* Original explicit calls, not guard(rcu). Copy happens after unlock. */
	rcu_read_lock();
	lupos_scx_core_qe_events_rcu_body(e_sys, aux);
	rcu_read_unlock();
}
size_t lupos_scx_core_qe_events_size(void) { return sizeof(struct scx_event_stats); }
void lupos_scx_core_qe_copy_events(struct scx_event_stats *events,
                                  const struct scx_event_stats *e_sys,
                                  size_t events__sz)
{ memcpy(events, e_sys, events__sz); }
#ifdef CONFIG_CGROUP_SCHED
bool lupos_scx_core_qe_task_arg_ok(struct scx_sched *sch, struct task_struct *p)
{ return scx_kf_arg_task_ok(sch, p); }
void lupos_scx_core_qe_cgroup_get(struct cgroup *cgrp) { cgroup_get(cgrp); }
#endif
