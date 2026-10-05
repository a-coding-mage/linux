/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_QUERY_EVENTS_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_QUERY_EVENTS_BINDINGS_H
/* F16 declarations for F00's single canonical native binding universe.
 * Original configured native headers alone own layouts, widths and constants.
 * Kfunc definitions retain their original native names and verifier arguments;
 * F13 owns all BTF ID sets/flags/filter metadata. */
#include "internal.h"

u32 lupos_scx_core_qe_cpuperf_cap_body(s32 cpu, const struct bpf_prog_aux *aux);
u32 lupos_scx_core_qe_cidperf_cap_body(s32 cid, const struct bpf_prog_aux *aux);
u32 lupos_scx_core_qe_cpuperf_cur_body(s32 cpu, const struct bpf_prog_aux *aux);
u32 lupos_scx_core_qe_cidperf_cur_body(s32 cid, const struct bpf_prog_aux *aux);
void lupos_scx_core_qe_cpuperf_set_body(s32 cpu, u32 perf,
                                      const struct bpf_prog_aux *aux);
s32 lupos_scx_core_qe_cidperf_set_body(s32 cid, u32 perf,
                                     const struct bpf_prog_aux *aux);
s32 lupos_scx_core_qe_cpuperf_locked_body(struct scx_sched *sch, s32 cpu,
                                        u32 perf, struct rq *rq);
u32 lupos_scx_core_qe_nr_node_ids_body(void);
u32 lupos_scx_core_qe_nr_cpu_ids_body(void);
u32 lupos_scx_core_qe_nr_cids_body(void);
u32 lupos_scx_core_qe_nr_online_cids_body(void);
s32 lupos_scx_core_qe_this_cid_body(void);
const struct cpumask *lupos_scx_core_qe_get_possible_cpumask_body(void);
const struct cpumask *lupos_scx_core_qe_get_online_cpumask_body(void);
void lupos_scx_core_qe_put_cpumask_body(const struct cpumask *cpumask);
bool lupos_scx_core_qe_task_running_body(const struct task_struct *p);
s32 lupos_scx_core_qe_task_cpu_body(const struct task_struct *p);
s32 lupos_scx_core_qe_task_cid_body(const struct task_struct *p);
struct rq *lupos_scx_core_qe_locked_rq_body(const struct bpf_prog_aux *aux);
struct task_struct *lupos_scx_core_qe_cpu_curr_body(s32 cpu,
                                                  const struct bpf_prog_aux *aux);
struct task_struct *lupos_scx_core_qe_cid_curr_body(s32 cid,
                                                  const struct bpf_prog_aux *aux);
struct task_struct *lupos_scx_core_qe_tid_to_task_body(u64 tid);
u64 lupos_scx_core_qe_now_body(void);
void lupos_scx_core_qe_events_body(struct scx_event_stats *events, size_t events__sz,
                                 const struct bpf_prog_aux *aux);
void lupos_scx_core_qe_events_rcu_body(struct scx_event_stats *e_sys,
                                     const struct bpf_prog_aux *aux);
#ifdef CONFIG_CGROUP_SCHED
struct cgroup *lupos_scx_core_qe_task_cgroup_body(struct task_struct *p,
                                               const struct bpf_prog_aux *aux,
                                               struct task_group *tg,
                                               struct cgroup *cgrp);
#endif

struct scx_sched *lupos_scx_core_qe_prog_sched(const struct bpf_prog_aux *aux);
bool lupos_scx_core_qe_likely_sched(struct scx_sched *sch);
bool lupos_scx_core_qe_unlikely_no_sched(struct scx_sched *sch);
u32 lupos_scx_core_qe_arch_capacity(s32 cpu);
u32 lupos_scx_core_qe_arch_frequency(s32 cpu);
s32 lupos_scx_core_qe_cid_to_cpu(struct scx_sched *sch, s32 cid);
bool lupos_scx_core_qe_unlikely_bad_perf(u32 perf);
void lupos_scx_core_qe_error_perf(struct scx_sched *sch, u32 perf, s32 cpu);
void lupos_scx_core_qe_error_target(struct scx_sched *sch, s32 cpu);
struct rq *lupos_scx_core_qe_cpu_rq(s32 cpu);
s32 lupos_scx_core_qe_cpuperf_unlocked(struct scx_sched *sch, s32 cpu,
                                     u32 perf, struct rq *rq);
bool lupos_scx_core_qe_likely_perf_allowed(struct scx_sched *sch, s32 cpu);
/* One original plain u32 field store; no synchronization or race proof. */
void lupos_scx_core_qe_cpuperf_target_write(struct rq *rq, u32 perf);
void lupos_scx_core_qe_cpufreq_update(struct rq *rq);
void lupos_scx_core_qe_event_perf_denied(struct scx_sched *sch);
u32 lupos_scx_core_qe_nr_node_ids(void);
unsigned int lupos_scx_core_qe_num_online_cpus(void);
s16 *lupos_scx_core_qe_this_cid_table_rcu(void);
s16 *lupos_scx_core_qe_task_cid_table_rcu(void);
int lupos_scx_core_qe_raw_cpu(void);
const struct cpumask *lupos_scx_core_qe_possible_cpumask(void);
const struct cpumask *lupos_scx_core_qe_online_cpumask(void);
struct task_struct *lupos_scx_core_qe_task_rq_curr_plain(const struct task_struct *p);
unsigned int lupos_scx_core_qe_task_cpu(const struct task_struct *p);
void lupos_scx_core_qe_error_unlocked_rq(struct scx_sched *sch);
struct task_struct *lupos_scx_core_qe_cpu_curr_rcu(s32 cpu);
struct task_struct *lupos_scx_core_qe_cid_curr_rcu(s32 cpu);
struct scx_sched *lupos_scx_core_qe_root_rcu(void);
void lupos_scx_core_qe_error_tid_disabled(struct scx_sched *sch);
struct task_struct *lupos_scx_core_qe_entity_task(struct sched_ext_entity *scx);
void lupos_scx_core_qe_assert_now(struct rq *rq);
u32 lupos_scx_core_qe_flags_acquire(struct rq *rq);
u64 lupos_scx_core_qe_clock_read_once(struct rq *rq);
u64 lupos_scx_core_qe_sched_clock_cpu(struct rq *rq);
struct rq *lupos_scx_core_qe_this_rq(void);
void lupos_scx_core_qe_zero_events(struct scx_event_stats *events);
bool lupos_scx_core_qe_possible_cpu_condition(int *cpu);
struct scx_event_stats *lupos_scx_core_qe_cpu_events(struct scx_sched *sch, int cpu);
s64 lupos_scx_core_qe_read_event(const s64 *counter);
void lupos_scx_core_qe_events_rcu(struct scx_event_stats *e_sys,
                                const struct bpf_prog_aux *aux);
size_t lupos_scx_core_qe_events_size(void);
void lupos_scx_core_qe_copy_events(struct scx_event_stats *events,
                                  const struct scx_event_stats *e_sys,
                                  size_t events__sz);
#ifdef CONFIG_CGROUP_SCHED
bool lupos_scx_core_qe_task_arg_ok(struct scx_sched *sch, struct task_struct *p);
void lupos_scx_core_qe_cgroup_get(struct cgroup *cgrp);
#endif

#endif /* LUPOS_SCHED_EXT_CORE_QUERY_EVENTS_BINDINGS_H */
