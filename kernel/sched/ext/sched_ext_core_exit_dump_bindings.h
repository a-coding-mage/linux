/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_SCHED_EXT_CORE_EXIT_DUMP_BINDINGS_H
#define LUPOS_SCHED_EXT_CORE_EXIT_DUMP_BINDINGS_H

/* Included into the one canonical native type universe by F00. */
#include "internal.h"
struct scx_exit_dump_va_args;

#define LUPOS_SCX_EXIT_EINVAL EINVAL
#define LUPOS_SCX_EXIT_UINT_MAX UINT_MAX
#define LUPOS_SCX_EXIT_MAX_BPRINTF_VARARGS MAX_BPRINTF_VARARGS
#define LUPOS_SCX_EXIT_MSG_LEN SCX_EXIT_MSG_LEN
#define LUPOS_SCX_EXIT_BT_LEN SCX_EXIT_BT_LEN
#define LUPOS_SCX_EXIT_TASK_STATE_SHIFT SCX_TASK_STATE_SHIFT
#define LUPOS_SCX_EXIT_TASK_STATE_MASK SCX_TASK_STATE_MASK
#define LUPOS_SCX_EXIT_OPSS_STATE_MASK SCX_OPSS_STATE_MASK
#define LUPOS_SCX_EXIT_OPSS_QSEQ_SHIFT SCX_OPSS_QSEQ_SHIFT

/* Exact native callback identities for F09 initialization in the same TU. */
static void scx_disable_irq_workfn(struct irq_work *irq_work);
static void scx_propagate_exit_irq_workfn(struct irq_work *irq_work);
static void scx_disable_workfn(struct kthread_work *work);

void lupos_scx_exit_disable_irq_workfn(struct scx_sched *sch);
void lupos_scx_exit_propagate_irq_workfn(struct scx_sched *sch);
void lupos_scx_exit_disable_workfn(struct scx_sched *sch);
bool lupos_scx_exit_vexit(struct scx_sched *sch, enum scx_exit_kind kind,
                        s64 exit_code, s32 exit_cpu, const char *fmt,
                        struct scx_exit_dump_va_args *args);
bool lupos_scx_exit_handle_lockup(int exit_cpu, const char *fmt,
                                 struct scx_exit_dump_va_args *args);
void lupos_scx_exit_dump_line(struct seq_buf *s, const char *fmt,
                             struct scx_exit_dump_va_args *trace_args,
                             struct scx_exit_dump_va_args *seq_args);
__printf(5, 0)
s32 lupos_scx_exit_bstr_format_body(struct scx_sched *sch, u64 *data_buf,
                                   char *line_buf, size_t line_size, char *fmt,
                                   unsigned long long *data, u32 data__sz);
__printf(5, 0)
s32 lupos_scx_exit_bstr_format(struct scx_sched *sch, u64 *data_buf,
                              char *line_buf, size_t line_size, char *fmt,
                              unsigned long long *data, u32 data__sz);
void lupos_scx_exit_bpf_exit_bstr(s64 exit_code, char *fmt,
                                  unsigned long long *data, u32 data__sz,
                                  const struct bpf_prog_aux *aux);
void lupos_scx_exit_bpf_error_bstr(char *fmt, unsigned long long *data,
                                  u32 data__sz, const struct bpf_prog_aux *aux);
void lupos_scx_exit_bpf_dump_bstr(char *fmt, unsigned long long *data,
                                 u32 data__sz, const struct bpf_prog_aux *aux);

void lupos_scx_exit_rcu_lock(void) __acquires_shared(RCU);
void lupos_scx_exit_rcu_unlock(void) __releases_shared(RCU);
void lupos_scx_exit_preempt_disable(void);
void lupos_scx_exit_preempt_enable(void);
void lupos_scx_exit_assert_preempt_disabled(void);
void lupos_scx_exit_assert_irqs_disabled(void);
void lupos_scx_exit_disable_dump_locked(struct scx_sched *sch);
void lupos_scx_exit_with_dump_lock(struct scx_sched *sch, struct scx_exit_info *ei,
                                  size_t dump_len, bool dump_all_tasks,
                                  struct scx_dump_ctx *dctx);
void lupos_scx_exit_dump_state_locked(struct scx_sched *sch, struct scx_exit_info *ei,
                                     size_t dump_len, bool dump_all_tasks,
                                     struct scx_dump_ctx *dctx);
bool lupos_scx_exit_lockup_unlikely_null(struct scx_sched *sch);
bool lupos_scx_exit_stall_unlikely_null(struct scx_sched *sch);
bool lupos_scx_exit_dump_unlikely_null(struct scx_sched *sch);
bool lupos_scx_exit_exit_likely_nonnull(struct scx_sched *sch);
bool lupos_scx_exit_error_likely_nonnull(struct scx_sched *sch);
bool lupos_scx_exit_warn_bad_kind(enum scx_exit_kind kind);
void lupos_scx_exit_warn_none(int kind);
bool lupos_scx_exit_try_claim(struct scx_sched *sch, int *old, int kind);
int lupos_scx_exit_kind_read(struct scx_sched *sch);
void lupos_scx_exit_aborting_once(struct scx_sched *sch);
void lupos_scx_exit_aborting_mb(struct scx_sched *sch);
void lupos_scx_exit_trace_exit(struct scx_sched *sch, enum scx_exit_kind kind);
struct scx_sched *lupos_scx_exit_next_descendant(struct scx_sched *pos,
                                               struct scx_sched *root);
void lupos_scx_exit_sub_disable(struct scx_sched *sch);
int lupos_scx_exit_soft_cpu(void);
int lupos_scx_exit_ops_dump_cpu(void);
int lupos_scx_exit_raw_cpu(void);
bool lupos_scx_exit_in_nmi(void);
unsigned int lupos_scx_exit_save_finish_stack(unsigned long *bt);
unsigned int lupos_scx_exit_save_stall_stack(unsigned long *bt);
void lupos_scx_exit_print_stack(const unsigned long *bt, unsigned int len);
void lupos_scx_exit_format_va(char *buf, size_t size, const char *fmt,
                            struct scx_exit_dump_va_args *args);
void lupos_scx_exit_soft_notice(int cpu, u32 dur_s);
void lupos_scx_exit_hard_notice(int cpu);
bool lupos_scx_exit_soft_lockup(int cpu, u32 dur_s);
bool lupos_scx_exit_hard_lockup(int cpu);
bool lupos_scx_exit_mask_empty(const struct cpumask *mask);
int lupos_scx_exit_mask_first(const struct cpumask *mask);
int lupos_scx_exit_mask_iter_first(const struct cpumask *mask);
int lupos_scx_exit_mask_next(int cpu, const struct cpumask *mask);
unsigned int lupos_scx_exit_mask_iter_limit(void);
bool lupos_scx_exit_mask_test(int cpu, const struct cpumask *mask);
int lupos_scx_exit_possible_iter_first(void);
int lupos_scx_exit_possible_iter_next(int cpu);
unsigned int lupos_scx_exit_possible_iter_limit(void);
struct cpumask *lupos_scx_exit_stall_mask(struct scx_sched *sch);
void lupos_scx_exit_mask_copy(struct cpumask *dst, const struct cpumask *src);
void lupos_scx_exit_format_stall(char *msg, const struct cpumask *mask);
void lupos_scx_exit_log_error(struct scx_sched *sch, const char *type);
void lupos_scx_exit_log_message(struct scx_sched *sch);
void lupos_scx_exit_log_info(struct scx_sched *sch, const char *type);
void lupos_scx_exit_trace_newline(void);
bool lupos_scx_exit_trace_dump_enabled(void);
void lupos_scx_exit_trace_line(const char *fmt, struct scx_exit_dump_va_args *args);
void lupos_scx_exit_seq_line(struct seq_buf *s, const char *fmt,
                           struct scx_exit_dump_va_args *args);
void lupos_scx_exit_seq_put_newline(struct seq_buf *s);
void lupos_scx_exit_emit_stack(struct seq_buf *s, const char *prefix, unsigned long addr);
void lupos_scx_exit_emit_ops_line(struct seq_buf *s, const char *prefix, const char *line);
void lupos_scx_exit_bprintf_init(struct bpf_bprintf_data *data);
void lupos_scx_exit_error_data(struct scx_sched *sch, unsigned long long *data, u32 size);
void lupos_scx_exit_error_read(struct scx_sched *sch, s32 ret);
void lupos_scx_exit_error_prepare(struct scx_sched *sch, s32 ret);
void lupos_scx_exit_error_format(struct scx_sched *sch, char *fmt,
                               unsigned long long *data, u32 size);
void lupos_scx_exit_format_fallback(char *msg, s32 ret);
void lupos_scx_exit_error_dump_context(struct scx_sched *sch);
void lupos_scx_exit_emit_format_error(struct seq_buf *s, const char *prefix,
                                     char *fmt, unsigned long long *data,
                                     u32 size, s32 ret);
struct scx_sched *lupos_scx_exit_exit_prog_sched(const struct bpf_prog_aux *aux);
struct scx_sched *lupos_scx_exit_error_prog_sched(const struct bpf_prog_aux *aux);
struct scx_sched *lupos_scx_exit_dump_prog_sched(const struct bpf_prog_aux *aux);
struct scx_sched *lupos_scx_exit_lockup_root_rcu(void);
struct scx_sched *lupos_scx_exit_stall_root_rcu(void);


/* Native format/lock/list leaves for task, CPU and whole-state dumping. */
struct scx_sched *lupos_scx_exit_dump_task_sched(struct task_struct *p);
struct scx_sched *lupos_scx_exit_info_task_sched(struct task_struct *p);
unsigned long *lupos_scx_exit_task_bt(void);
unsigned long lupos_scx_exit_task_ops_state(struct task_struct *p);
void lupos_scx_exit_format_root_id(char *buf, size_t size);
void lupos_scx_exit_format_sub_id(char *buf, size_t size, struct scx_sched *sch);
void lupos_scx_exit_format_dsq_id(char *buf, size_t size, struct scx_dispatch_q *dsq);
void lupos_scx_exit_emit_task_identity(struct seq_buf *s, struct task_struct *p,
                                      char marker, const char *own_marker,
                                      const char *sch_id, long delta);
void lupos_scx_exit_emit_task_flags(struct seq_buf *s, struct task_struct *p,
                                   u32 state, u32 flags, unsigned long ops_state,
                                   unsigned long qseq);
void lupos_scx_exit_emit_task_dsq(struct seq_buf *s, struct task_struct *p,
                                 const char *dsq_id);
void lupos_scx_exit_emit_task_slice(struct seq_buf *s, struct task_struct *p);
void lupos_scx_exit_emit_task_cpus(struct seq_buf *s, struct task_struct *p);
bool lupos_scx_exit_has_dump_task(struct scx_sched *sch);
bool lupos_scx_exit_has_dump_cpu(struct scx_sched *sch);
bool lupos_scx_exit_has_dump(struct scx_sched *sch);
void lupos_scx_exit_call_dump_task(struct scx_sched *sch, struct rq *rq,
                                  struct scx_dump_ctx *dctx, struct task_struct *p);
void lupos_scx_exit_call_dump_cpu(struct scx_sched *sch, struct rq *rq,
                                 struct scx_dump_ctx *dctx, int cpu, bool idle);
void lupos_scx_exit_call_dump(struct scx_sched *sch, struct scx_dump_ctx *dctx);
unsigned int lupos_scx_exit_save_task_stack(struct task_struct *p, unsigned long *bt);
struct rq *lupos_scx_exit_cpu_rq(int cpu);
struct scx_sched_pcpu *lupos_scx_exit_pcpu(struct scx_sched *sch, int cpu);
void lupos_scx_exit_rq_lock(struct rq *rq, struct rq_flags *rf) __acquires(__rq_lockp(rq));
void lupos_scx_exit_rq_unlock(struct rq *rq, struct rq_flags *rf) __releases(__rq_lockp(rq));
bool lupos_scx_exit_list_empty(struct list_head *head);
const struct sched_class *lupos_scx_exit_idle_class(void);
size_t lupos_scx_exit_seq_get_buf(struct seq_buf *s, char **buf);
void lupos_scx_exit_seq_init(struct seq_buf *s, char *buf, size_t size);
size_t lupos_scx_exit_seq_used(struct seq_buf *s);
bool lupos_scx_exit_seq_overflowed(struct seq_buf *s);
void lupos_scx_exit_seq_commit(struct seq_buf *s, size_t used);
void lupos_scx_exit_seq_set_overflow(struct seq_buf *s);
void lupos_scx_exit_emit_cpu_state(struct seq_buf *s, struct rq *rq, int cpu);
void lupos_scx_exit_rescue_dump(struct seq_buf *s, struct rq *rq);
void lupos_scx_exit_emit_cpu_current(struct seq_buf *s, struct rq *rq);
const struct cpumask *lupos_scx_exit_to_kick(struct scx_sched_pcpu *pcpu);
const struct cpumask *lupos_scx_exit_to_idle(struct scx_sched_pcpu *pcpu);
const struct cpumask *lupos_scx_exit_to_preempt(struct scx_sched_pcpu *pcpu);
const struct cpumask *lupos_scx_exit_to_wait(struct scx_sched_pcpu *pcpu);
const struct cpumask *lupos_scx_exit_to_sync(struct rq *rq);
void lupos_scx_exit_emit_cpu_kick(struct seq_buf *s, const struct cpumask *mask);
void lupos_scx_exit_emit_cpu_idle(struct seq_buf *s, const struct cpumask *mask);
void lupos_scx_exit_emit_cpu_preempt(struct seq_buf *s, const struct cpumask *mask);
void lupos_scx_exit_emit_cpu_wait(struct seq_buf *s, const struct cpumask *mask);
void lupos_scx_exit_emit_cpu_sync(struct seq_buf *s, const struct cpumask *mask);
bool lupos_scx_exit_curr_on_sched(struct scx_sched *sch, struct task_struct *p);
bool lupos_scx_exit_runnable_on_sched(struct scx_sched *sch, struct task_struct *p);
struct task_struct *lupos_scx_exit_runnable_task(struct list_head *node);
u64 lupos_scx_exit_now_ns(void);
unsigned long lupos_scx_exit_jiffies(void);
size_t lupos_scx_exit_dump_len(struct scx_sched *sch);
void lupos_scx_exit_emit_root(struct seq_buf *s, struct scx_sched *sch);
void lupos_scx_exit_emit_sub(struct seq_buf *s, struct scx_sched *sch);
void lupos_scx_exit_emit_debug(struct seq_buf *s, const char *reason);
void lupos_scx_exit_emit_cause_cpu(struct seq_buf *s, struct scx_exit_info *ei);
void lupos_scx_exit_emit_cause(struct seq_buf *s, struct scx_exit_info *ei);
void lupos_scx_exit_emit_reason(struct seq_buf *s, struct scx_exit_info *ei);
void lupos_scx_exit_emit_backtrace(struct seq_buf *s);
void lupos_scx_exit_emit_cpu_heading(struct seq_buf *s);
void lupos_scx_exit_emit_cpu_rule(struct seq_buf *s);
void lupos_scx_exit_emit_event_heading(struct seq_buf *s);
void lupos_scx_exit_emit_event_rule(struct seq_buf *s);
void lupos_scx_exit_emit_events(struct seq_buf *s, struct scx_event_stats *events);
void lupos_scx_exit_format_runnable(char *buf, size_t size, long delta);
void lupos_scx_exit_print_info(const char *level, struct scx_sched *sch,
                              enum scx_enable_state state, const char *all);
void lupos_scx_exit_print_task_info(const char *level, struct scx_sched *sch,
                                   enum scx_enable_state state, const char *all,
                                   const char *runnable);
#endif /* LUPOS_SCHED_EXT_CORE_EXIT_DUMP_BINDINGS_H */
