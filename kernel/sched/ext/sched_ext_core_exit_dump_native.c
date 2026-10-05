// SPDX-License-Identifier: GPL-2.0
/* F11 configured-native primitives and ABI entry shims. Include only in the
 * common native envelope. No owner algorithm falls back to original ext.c.
 */
#error "SOURCE ONLY HOLD: exit/dump native ABI, NMI, stack and protection qualification incomplete"
#include "sched_ext_core_bindings.h"
#include "sched_ext_core_exit_dump_bindings.h"

/*
 * Native printk.h's index contract, with the original owner's provenance.
 * The original build's __FILE__ spelling depends on inclusion/prefix mapping;
 * require that exact string rather than guessing a path or adding #line.
 * Runtime calls below use _printk[_deferred] so there is no adapter-site entry.
 */
#ifdef CONFIG_PRINTK_INDEX
#ifndef LUPOS_SCX_EXIT_ORIGINAL_FILE
#error "SOURCE ONLY HOLD: exit/dump printk index requires original ext.c __FILE__ spelling"
#endif
#define LUPOS_SCX_EXIT_INDEX(_fmt, _func, _line) do {                  \
	if (__builtin_constant_p(_fmt) && __builtin_constant_p(NULL)) { \
		static const struct pi_entry _entry __used = {          \
			.fmt = __builtin_constant_p(_fmt) ? (_fmt) : NULL, \
			.func = (_func),                                 \
			.file = LUPOS_SCX_EXIT_ORIGINAL_FILE,             \
			.line = (_line),                                 \
			.level = __builtin_constant_p(NULL) ? NULL : NULL, \
			.subsys_fmt_prefix = NULL,                       \
		};                                                       \
		static const struct pi_entry *_entry_ptr                 \
		__used __section(".printk_index") = &_entry;              \
	}                                                               \
} while (0)
#else
#define LUPOS_SCX_EXIT_INDEX(...) do {} while (0)
#endif
#define LUPOS_SCX_EXIT_PRINTK(_func, _line, _p_func, _fmt, ...) ({     \
	LUPOS_SCX_EXIT_INDEX(_fmt, _func, _line);                     \
	_p_func(_fmt, ##__VA_ARGS__);                               \
})

/* Deliberately not defined in the binding header: native va_list only. */
struct scx_exit_dump_va_args {
	va_list args;
};

__printf(5, 0) bool scx_vexit(struct scx_sched *sch, enum scx_exit_kind kind,
                            s64 exit_code, s32 exit_cpu, const char *fmt,
                            va_list args)
{
	struct scx_exit_dump_va_args holder;
	bool ret;

	va_copy(holder.args, args);
	ret = lupos_scx_exit_vexit(sch, kind, exit_code, exit_cpu, fmt, &holder);
	va_end(holder.args);
	return ret;
}

static __printf(2, 3) bool handle_lockup(int exit_cpu, const char *fmt, ...)
{
	struct scx_exit_dump_va_args holder;
	bool ret;

	va_start(holder.args, fmt);
	ret = lupos_scx_exit_handle_lockup(exit_cpu, fmt, &holder);
	va_end(holder.args);
	return ret;
}

__printf(2, 3) void scx_dump_line(struct seq_buf *s, const char *fmt, ...)
{
	struct scx_exit_dump_va_args trace_args, seq_args;

	/* Independent native argument lists; an inactive destination consumes none. */
	va_start(trace_args.args, fmt);
	va_start(seq_args.args, fmt);
	lupos_scx_exit_dump_line(s, fmt, &trace_args, &seq_args);
	va_end(seq_args.args);
	va_end(trace_args.args);
}

static void scx_disable_irq_workfn(struct irq_work *irq_work)
{
	lupos_scx_exit_disable_irq_workfn(
		container_of(irq_work, struct scx_sched, disable_irq_work));
}
static void scx_propagate_exit_irq_workfn(struct irq_work *irq_work)
{
	struct scx_sched *sch = container_of(irq_work, struct scx_sched,
					     propagate_exit_irq_work);

	/* Preserve the pinned tree's guard, not conventional irqsave primitives. */
	scoped_guard (raw_spinlock_irqsave, lupos_scx_core_sched_lock()) {
		lupos_scx_exit_propagate_irq_workfn(sch);
	}
}
static void scx_disable_workfn(struct kthread_work *work)
{
	lupos_scx_exit_disable_workfn(
		container_of(work, struct scx_sched, disable_work));
}

void lupos_scx_exit_rcu_lock(void) __acquires_shared(RCU) { rcu_read_lock(); }
void lupos_scx_exit_rcu_unlock(void) __releases_shared(RCU) { rcu_read_unlock(); }
void lupos_scx_exit_preempt_disable(void) { preempt_disable(); }
void lupos_scx_exit_preempt_enable(void) { preempt_enable(); }
void lupos_scx_exit_assert_preempt_disabled(void) { lockdep_assert_preemption_disabled(); }
void lupos_scx_exit_assert_irqs_disabled(void) { lockdep_assert_irqs_disabled(); }
void scx_disable_dump(struct scx_sched *sch)
{
	guard(raw_spinlock_irqsave)(lupos_scx_core_dump_lock());
	lupos_scx_exit_disable_dump_locked(sch);
}
void lupos_scx_exit_with_dump_lock(struct scx_sched *sch, struct scx_exit_info *ei,
                                  size_t dump_len, bool dump_all_tasks,
                                  struct scx_dump_ctx *dctx)
{
	guard(raw_spinlock_irqsave)(lupos_scx_core_dump_lock());
	lupos_scx_exit_dump_state_locked(sch, ei, dump_len, dump_all_tasks, dctx);
}
bool lupos_scx_exit_warn_bad_kind(enum scx_exit_kind kind)
{
	return WARN_ON_ONCE(kind == SCX_EXIT_NONE || kind == SCX_EXIT_DONE);
}
void lupos_scx_exit_warn_none(int kind) { WARN_ON_ONCE(kind == SCX_EXIT_NONE); }
bool lupos_scx_exit_try_claim(struct scx_sched *sch, int *old, int kind)
{
	return atomic_try_cmpxchg(&sch->exit_kind, old, kind);
}
int lupos_scx_exit_kind_read(struct scx_sched *sch) { return atomic_read(&sch->exit_kind); }
void lupos_scx_exit_aborting_once(struct scx_sched *sch) { WRITE_ONCE(sch->aborting, true); }
void lupos_scx_exit_aborting_mb(struct scx_sched *sch) { smp_store_mb(sch->aborting, true); }
void lupos_scx_exit_trace_exit(struct scx_sched *sch, enum scx_exit_kind kind)
{
	trace_sched_ext_exit(sch, kind);
}
struct scx_sched *lupos_scx_exit_next_descendant(struct scx_sched *pos,
                                               struct scx_sched *root)
{
	return scx_next_descendant_pre(pos, root);
}
/* sub owner supplies the enabled body; header supplies only its native !SUB alternative. */
void lupos_scx_exit_sub_disable(struct scx_sched *sch) { scx_sub_disable(sch); }
/* Keep the two checked-CPU source sites distinct under DEBUG_PREEMPT. */
int lupos_scx_exit_soft_cpu(void) { return smp_processor_id(); }
int lupos_scx_exit_ops_dump_cpu(void) { return smp_processor_id(); }
int lupos_scx_exit_raw_cpu(void) { return raw_smp_processor_id(); }
bool lupos_scx_exit_in_nmi(void) { return in_nmi(); }
#ifdef CONFIG_STACKTRACE
unsigned int lupos_scx_exit_save_finish_stack(unsigned long *bt)
{
	return stack_trace_save(bt, SCX_EXIT_BT_LEN, 1);
}
unsigned int lupos_scx_exit_save_stall_stack(unsigned long *bt)
{
	return stack_trace_save(bt, SCX_EXIT_BT_LEN, 1);
}
void lupos_scx_exit_print_stack(const unsigned long *bt, unsigned int len)
{
	stack_trace_print(bt, len, 2);
}
#endif
void lupos_scx_exit_format_va(char *buf, size_t size, const char *fmt,
                            struct scx_exit_dump_va_args *args)
{
	vscnprintf(buf, size, fmt, args->args);
}
bool lupos_scx_exit_soft_lockup(int cpu, u32 dur_s)
{
	return handle_lockup(cpu, "soft lockup - CPU %d stuck for %us", cpu, dur_s);
}
bool lupos_scx_exit_hard_lockup(int cpu)
{
	return handle_lockup(cpu, "hard lockup - CPU %d", cpu);
}
void lupos_scx_exit_soft_notice(int cpu, u32 dur_s)
{
	LUPOS_SCX_EXIT_PRINTK("scx_softlockup", 5677, _printk_deferred,
			KERN_ERR "sched_ext: Soft lockup - CPU %d stuck for %us, disabling BPF scheduler\n",
			cpu, dur_s);
}
void lupos_scx_exit_hard_notice(int cpu)
{
	LUPOS_SCX_EXIT_PRINTK("scx_hardlockup", 5703, _printk_deferred,
			KERN_ERR "sched_ext: Hard lockup - CPU %d, disabling BPF scheduler\n", cpu);
}
struct scx_sched *lupos_scx_exit_lockup_root_rcu(void)
{
	return rcu_dereference(scx_root);
}
struct scx_sched *lupos_scx_exit_stall_root_rcu(void)
{
	return rcu_dereference(scx_root);
}
bool lupos_scx_exit_mask_empty(const struct cpumask *mask) { return cpumask_empty(mask); }
int lupos_scx_exit_mask_first(const struct cpumask *mask) { return (int)cpumask_first(mask); }
int lupos_scx_exit_mask_iter_first(const struct cpumask *mask)
{
	return find_next_bit(cpumask_bits(mask), small_cpumask_bits, 0);
}
/* for_each_cpu expands for_each_set_bit, not cpumask_next's extra check. */
int lupos_scx_exit_mask_next(int cpu, const struct cpumask *mask)
{
	return find_next_bit(cpumask_bits(mask), small_cpumask_bits, cpu + 1);
}
unsigned int lupos_scx_exit_mask_iter_limit(void) { return small_cpumask_bits; }
bool lupos_scx_exit_mask_test(int cpu, const struct cpumask *mask) { return cpumask_test_cpu(cpu, mask); }
/* Preserve for_each_possible_cpu's separate NR_CPUS == 1 expansion. */
int lupos_scx_exit_possible_iter_first(void)
{
#if NR_CPUS == 1
	return 0;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits, 0);
#endif
}
int lupos_scx_exit_possible_iter_next(int cpu)
{
#if NR_CPUS == 1
	return cpu + 1;
#else
	return find_next_bit(cpumask_bits(cpu_possible_mask), small_cpumask_bits, cpu + 1);
#endif
}
unsigned int lupos_scx_exit_possible_iter_limit(void)
{
#if NR_CPUS == 1
	return 1;
#else
	return small_cpumask_bits;
#endif
}
struct cpumask *lupos_scx_exit_stall_mask(struct scx_sched *sch) { return sch->stall_cpus; }
void lupos_scx_exit_mask_copy(struct cpumask *dst, const struct cpumask *src) { cpumask_copy(dst, src); }
void lupos_scx_exit_format_stall(char *msg, const struct cpumask *mask)
{
	scnprintf(msg, SCX_EXIT_MSG_LEN, "RCU CPU stall on CPUs (%*pbl)", cpumask_pr_args(mask));
}
void lupos_scx_exit_log_error(struct scx_sched *sch, const char *type)
{
	LUPOS_SCX_EXIT_PRINTK("scx_log_sched_disable", 6366, _printk,
			KERN_ERR pr_fmt("sched_ext: BPF %s \"%s\" disabled (%s)\n"),
			type, sch->ops.name, sch->exit_info->reason);
}
void lupos_scx_exit_log_message(struct scx_sched *sch)
{
	LUPOS_SCX_EXIT_PRINTK("scx_log_sched_disable", 6370, _printk,
			KERN_ERR pr_fmt("sched_ext: %s: %s\n"),
			sch->ops.name, sch->exit_info->msg);
}
void lupos_scx_exit_log_info(struct scx_sched *sch, const char *type)
{
	LUPOS_SCX_EXIT_PRINTK("scx_log_sched_disable", 6375, _printk,
			KERN_INFO pr_fmt("sched_ext: BPF %s \"%s\" disabled (%s)\n"),
			type, sch->ops.name, sch->exit_info->reason);
}
void lupos_scx_exit_trace_newline(void) { trace_sched_ext_dump(""); }
#ifdef CONFIG_TRACEPOINTS
bool lupos_scx_exit_trace_dump_enabled(void) { return trace_sched_ext_dump_enabled(); }
void lupos_scx_exit_trace_line(const char *fmt, struct scx_exit_dump_va_args *args)
{
	/* Exact original private storage, protected by F00's scx_dump_lock. */
	static char line_buf[SCX_EXIT_MSG_LEN];

	vscnprintf(line_buf, sizeof(line_buf), fmt, args->args);
	trace_call__sched_ext_dump(line_buf);
}
#endif
void lupos_scx_exit_seq_line(struct seq_buf *s, const char *fmt,
                           struct scx_exit_dump_va_args *args)
{
	seq_buf_vprintf(s, fmt, args->args);
}
void lupos_scx_exit_seq_put_newline(struct seq_buf *s) { seq_buf_putc(s, '\n'); }
void lupos_scx_exit_emit_stack(struct seq_buf *s, const char *prefix, unsigned long addr)
{
	scx_dump_line(s, "%s%pS", prefix, (void *)addr);
}
void lupos_scx_exit_emit_ops_line(struct seq_buf *s, const char *prefix, const char *line)
{
	scx_dump_line(s, "%s%s", prefix, line);
}
/* Retain the private native format-annotated identity; Rust owns its body. */
__printf(5, 0)
static s32 __bstr_format(struct scx_sched *sch, u64 *data_buf, char *line_buf,
			 size_t line_size, char *fmt, unsigned long long *data,
			 u32 data__sz)
{
	return lupos_scx_exit_bstr_format_body(sch, data_buf, line_buf, line_size,
					     fmt, data, data__sz);
}
/* External typed bridge only: __bstr_format remains static in the one TU. */
__printf(5, 0)
s32 lupos_scx_exit_bstr_format(struct scx_sched *sch, u64 *data_buf,
			     char *line_buf, size_t line_size, char *fmt,
			     unsigned long long *data, u32 data__sz)
{
	return __bstr_format(sch, data_buf, line_buf, line_size, fmt, data, data__sz);
}
void lupos_scx_exit_bprintf_init(struct bpf_bprintf_data *data)
{
	*data = (struct bpf_bprintf_data){ .get_bin_args = true };
}
void lupos_scx_exit_error_data(struct scx_sched *sch, unsigned long long *data, u32 size)
{
	scx_error(sch, "invalid data=%p and data__sz=%u", (void *)data, size);
}
void lupos_scx_exit_error_read(struct scx_sched *sch, s32 ret)
{
	scx_error(sch, "failed to read data fields (%d)", ret);
}
void lupos_scx_exit_error_prepare(struct scx_sched *sch, s32 ret)
{
	scx_error(sch, "format preparation failed (%d)", ret);
}
void lupos_scx_exit_error_format(struct scx_sched *sch, char *fmt,
                               unsigned long long *data, u32 size)
{
	scx_error(sch, "(\"%s\", %p, %u) failed to format", fmt, data, size);
}
void lupos_scx_exit_format_fallback(char *msg, s32 ret)
{
	scnprintf(msg, SCX_EXIT_MSG_LEN, "exit message formatting failed (%d)", ret);
}
void lupos_scx_exit_error_dump_context(struct scx_sched *sch)
{
	scx_error(sch, "scx_bpf_dump() must only be called from ops.dump() and friends");
}
void lupos_scx_exit_emit_format_error(struct seq_buf *s, const char *prefix,
                                     char *fmt, unsigned long long *data,
                                     u32 size, s32 ret)
{
	scx_dump_line(s, "%s[!] (\"%s\", %p, %u) failed to format (%d)",
		      prefix, fmt, data, size, ret);
}
struct scx_sched *lupos_scx_exit_exit_prog_sched(const struct bpf_prog_aux *aux)
{
	return scx_prog_sched(aux);
}
struct scx_sched *lupos_scx_exit_error_prog_sched(const struct bpf_prog_aux *aux)
{
	return scx_prog_sched(aux);
}
struct scx_sched *lupos_scx_exit_dump_prog_sched(const struct bpf_prog_aux *aux)
{
	return scx_prog_sched(aux);
}

__bpf_kfunc_start_defs();

__printf(2, 0)
__bpf_kfunc void scx_bpf_exit_bstr(s64 exit_code, char *fmt,
                                 unsigned long long *data, u32 data__sz,
                                 const struct bpf_prog_aux *aux)
{
	lupos_scx_exit_bpf_exit_bstr(exit_code, fmt, data, data__sz, aux);
}
__printf(1, 0)
__bpf_kfunc void scx_bpf_error_bstr(char *fmt, unsigned long long *data,
                                  u32 data__sz, const struct bpf_prog_aux *aux)
{
	lupos_scx_exit_bpf_error_bstr(fmt, data, data__sz, aux);
}
__printf(1, 0)
__bpf_kfunc void scx_bpf_dump_bstr(char *fmt, unsigned long long *data,
                                 u32 data__sz, const struct bpf_prog_aux *aux)
{
	lupos_scx_exit_bpf_dump_bstr(fmt, data, data__sz, aux);
}

__bpf_kfunc_end_defs();

struct scx_sched *lupos_scx_exit_dump_task_sched(struct task_struct *p)
{
	return scx_task_sched(p);
}
struct scx_sched *lupos_scx_exit_info_task_sched(struct task_struct *p)
{
	return scx_task_sched_rcu(p);
}
unsigned long *lupos_scx_exit_task_bt(void)
{
	/* Exact ext.c:6779 storage, serialized by the global dump lock. */
	static unsigned long bt[SCX_EXIT_BT_LEN];
	return bt;
}
unsigned long lupos_scx_exit_task_ops_state(struct task_struct *p)
{
	return atomic_long_read(&p->scx.ops_state);
}
void lupos_scx_exit_format_root_id(char *buf, size_t size)
{
	scnprintf(buf, size, "root");
}
void lupos_scx_exit_format_sub_id(char *buf, size_t size, struct scx_sched *sch)
{
	scnprintf(buf, size, "sub%d-%llu", sch->level, sch->ops.sub_cgroup_id);
}
void lupos_scx_exit_format_dsq_id(char *buf, size_t size, struct scx_dispatch_q *dsq)
{
	scnprintf(buf, size, "0x%llx", (unsigned long long)dsq->id);
}
void lupos_scx_exit_emit_task_identity(struct seq_buf *s, struct task_struct *p,
                                      char marker, const char *own_marker,
                                      const char *sch_id, long delta)
{
	scx_dump_line(s, " %c%c %s[%d] %s%s %+ldms", marker, task_state_to_char(p),
		      p->comm, p->pid, own_marker, sch_id, delta);
}
void lupos_scx_exit_emit_task_flags(struct seq_buf *s, struct task_struct *p,
                                   u32 state, u32 flags, unsigned long ops_state,
                                   unsigned long qseq)
{
	scx_dump_line(s, "      scx_state/flags=%u/0x%x dsq_flags=0x%x ops_state/qseq=%lu/%lu",
		      state, flags, p->scx.dsq_flags, ops_state, qseq);
}
void lupos_scx_exit_emit_task_dsq(struct seq_buf *s, struct task_struct *p,
                                 const char *dsq_id)
{
	scx_dump_line(s, "      sticky/holding_cpu=%d/%d dsq_id=%s",
		      p->scx.sticky_cpu, p->scx.holding_cpu, dsq_id);
}
void lupos_scx_exit_emit_task_slice(struct seq_buf *s, struct task_struct *p)
{
	scx_dump_line(s, "      dsq_vtime=%llu slice=%llu weight=%u",
		      p->scx.dsq_vtime, p->scx.slice, p->scx.weight);
}
void lupos_scx_exit_emit_task_cpus(struct seq_buf *s, struct task_struct *p)
{
	scx_dump_line(s, "      cpus=%*pb no_mig=%u", cpumask_pr_args(p->cpus_ptr), p->migration_disabled);
}
bool lupos_scx_exit_has_dump_task(struct scx_sched *sch) { return SCX_HAS_OP(sch, dump_task); }
bool lupos_scx_exit_has_dump_cpu(struct scx_sched *sch) { return SCX_HAS_OP(sch, dump_cpu); }
bool lupos_scx_exit_has_dump(struct scx_sched *sch) { return SCX_HAS_OP(sch, dump); }
void lupos_scx_exit_call_dump_task(struct scx_sched *sch, struct rq *rq,
                                  struct scx_dump_ctx *dctx, struct task_struct *p)
{
	/* The original is SCX_CALL_OP, not the kf_tasks-setting TASK variant. */
	SCX_CALL_OP(sch, dump_task, rq, dctx, p);
}
void lupos_scx_exit_call_dump_cpu(struct scx_sched *sch, struct rq *rq,
                                 struct scx_dump_ctx *dctx, int cpu, bool idle)
{
	SCX_CALL_OP(sch, dump_cpu, rq, dctx, scx_cpu_arg(cpu), idle);
}
void lupos_scx_exit_call_dump(struct scx_sched *sch, struct scx_dump_ctx *dctx)
{
	SCX_CALL_OP(sch, dump, NULL, dctx);
}
#ifdef CONFIG_STACKTRACE
unsigned int lupos_scx_exit_save_task_stack(struct task_struct *p, unsigned long *bt)
{
	return stack_trace_save_tsk(p, bt, SCX_EXIT_BT_LEN, 1);
}
#endif
struct rq *lupos_scx_exit_cpu_rq(int cpu) { return cpu_rq(cpu); }
struct scx_sched_pcpu *lupos_scx_exit_pcpu(struct scx_sched *sch, int cpu)
{
	return per_cpu_ptr(sch->pcpu, cpu);
}
void lupos_scx_exit_rq_lock(struct rq *rq, struct rq_flags *rf)
	__acquires(__rq_lockp(rq))
{
	rq_lock_irqsave(rq, rf);
}
void lupos_scx_exit_rq_unlock(struct rq *rq, struct rq_flags *rf)
	__releases(__rq_lockp(rq))
{
	rq_unlock_irqrestore(rq, rf);
}
bool lupos_scx_exit_list_empty(struct list_head *head) { return list_empty(head); }
const struct sched_class *lupos_scx_exit_idle_class(void) { return &idle_sched_class; }
size_t lupos_scx_exit_seq_get_buf(struct seq_buf *s, char **buf) { return seq_buf_get_buf(s, buf); }
void lupos_scx_exit_seq_init(struct seq_buf *s, char *buf, size_t size) { seq_buf_init(s, buf, size); }
size_t lupos_scx_exit_seq_used(struct seq_buf *s) { return seq_buf_used(s); }
bool lupos_scx_exit_seq_overflowed(struct seq_buf *s) { return seq_buf_has_overflowed(s); }
void lupos_scx_exit_seq_commit(struct seq_buf *s, size_t used) { seq_buf_commit(s, used); }
void lupos_scx_exit_seq_set_overflow(struct seq_buf *s) { seq_buf_set_overflow(s); }
void lupos_scx_exit_emit_cpu_state(struct seq_buf *s, struct rq *rq, int cpu)
{
	scx_dump_line(s, "CPU %-4d: nr_run=%u flags=0x%x cpu_rel=%d ops_qseq=%lu ksync=%lu",
		      cpu, rq->scx.nr_running, rq->scx.flags, rq->scx.cpu_released,
		      rq->scx.ops_qseq, rq->scx.kick_sync);
}
void lupos_scx_exit_rescue_dump(struct seq_buf *s, struct rq *rq) { scx_rescue_dump(s, rq); }
void lupos_scx_exit_emit_cpu_current(struct seq_buf *s, struct rq *rq)
{
	scx_dump_line(s, "          curr=%s[%d] class=%ps", rq->curr->comm,
		      rq->curr->pid, rq->curr->sched_class);
}
const struct cpumask *lupos_scx_exit_to_kick(struct scx_sched_pcpu *pcpu) { return pcpu->cpus_to_kick; }
const struct cpumask *lupos_scx_exit_to_idle(struct scx_sched_pcpu *pcpu) { return pcpu->cpus_to_kick_if_idle; }
const struct cpumask *lupos_scx_exit_to_preempt(struct scx_sched_pcpu *pcpu) { return pcpu->cpus_to_preempt; }
const struct cpumask *lupos_scx_exit_to_wait(struct scx_sched_pcpu *pcpu) { return pcpu->cpus_to_wait; }
const struct cpumask *lupos_scx_exit_to_sync(struct rq *rq) { return rq->scx.cpus_to_sync; }
void lupos_scx_exit_emit_cpu_kick(struct seq_buf *s, const struct cpumask *mask)
{
	scx_dump_line(s, "  cpus_to_kick   : %*pb", cpumask_pr_args(mask));
}
void lupos_scx_exit_emit_cpu_idle(struct seq_buf *s, const struct cpumask *mask)
{
	scx_dump_line(s, "  idle_to_kick   : %*pb", cpumask_pr_args(mask));
}
void lupos_scx_exit_emit_cpu_preempt(struct seq_buf *s, const struct cpumask *mask)
{
	scx_dump_line(s, "  cpus_to_preempt: %*pb", cpumask_pr_args(mask));
}
void lupos_scx_exit_emit_cpu_wait(struct seq_buf *s, const struct cpumask *mask)
{
	scx_dump_line(s, "  cpus_to_wait   : %*pb", cpumask_pr_args(mask));
}
void lupos_scx_exit_emit_cpu_sync(struct seq_buf *s, const struct cpumask *mask)
{
	scx_dump_line(s, "  cpus_to_sync   : %*pb", cpumask_pr_args(mask));
}
bool lupos_scx_exit_curr_on_sched(struct scx_sched *sch, struct task_struct *p)
{
	return scx_task_on_sched(sch, p);
}
bool lupos_scx_exit_runnable_on_sched(struct scx_sched *sch, struct task_struct *p)
{
	return scx_task_on_sched(sch, p);
}
struct task_struct *lupos_scx_exit_runnable_task(struct list_head *node)
{
	return list_entry(node, struct task_struct, scx.runnable_node);
}
u64 lupos_scx_exit_now_ns(void) { return ktime_get_ns(); }
unsigned long lupos_scx_exit_jiffies(void) { return jiffies; }
size_t lupos_scx_exit_dump_len(struct scx_sched *sch) { return sch->ops.exit_dump_len; }
#ifdef CONFIG_EXT_SUB_SCHED
void lupos_scx_exit_emit_root(struct seq_buf *s, struct scx_sched *sch)
{
	scx_dump_line(s, "%s: root", sch->ops.name);
}
void lupos_scx_exit_emit_sub(struct seq_buf *s, struct scx_sched *sch)
{
	scx_dump_line(s, "%s: sub%d-%llu %s", sch->ops.name, sch->level,
		      sch->ops.sub_cgroup_id, sch->cgrp_path);
}
#endif
void lupos_scx_exit_emit_debug(struct seq_buf *s, const char *reason)
{
	scx_dump_line(s, "Debug dump triggered by %s", reason);
}
void lupos_scx_exit_emit_cause_cpu(struct seq_buf *s, struct scx_exit_info *ei)
{
	scx_dump_line(s, "%s[%d] triggered exit kind %d on CPU %d:",
		      current->comm, current->pid, ei->kind, ei->exit_cpu);
}
void lupos_scx_exit_emit_cause(struct seq_buf *s, struct scx_exit_info *ei)
{
	scx_dump_line(s, "%s[%d] triggered exit kind %d:", current->comm, current->pid, ei->kind);
}
void lupos_scx_exit_emit_reason(struct seq_buf *s, struct scx_exit_info *ei)
{
	scx_dump_line(s, "  %s (%s)", ei->reason, ei->msg);
}
void lupos_scx_exit_emit_backtrace(struct seq_buf *s) { scx_dump_line(s, "Backtrace:"); }
void lupos_scx_exit_emit_cpu_heading(struct seq_buf *s) { scx_dump_line(s, "CPU states"); }
void lupos_scx_exit_emit_cpu_rule(struct seq_buf *s) { scx_dump_line(s, "----------"); }
void lupos_scx_exit_emit_event_heading(struct seq_buf *s) { scx_dump_line(s, "Event counters"); }
void lupos_scx_exit_emit_event_rule(struct seq_buf *s) { scx_dump_line(s, "--------------"); }
/* Original fixed event-expansion macro: formatting-only native residue. */
#define scx_dump_event(s, events, kind) do {					\
	scx_dump_line(&(s), "%40s: %16lld", #kind, (events)->kind);		\
} while (0)
void lupos_scx_exit_emit_events(struct seq_buf *s, struct scx_event_stats *events)
{
#define SCX_EVENT(name) scx_dump_event(*s, events, name)
	SCX_EVENTS_LIST(SCX_EVENT);
#undef SCX_EVENT
}
#undef scx_dump_event
void lupos_scx_exit_format_runnable(char *buf, size_t size, long delta)
{
	scnprintf(buf, size, "%+ldms", delta);
}
void lupos_scx_exit_print_info(const char *level, struct scx_sched *sch,
                              enum scx_enable_state state, const char *all)
{
	LUPOS_SCX_EXIT_PRINTK("print_scx_info", 8610, _printk,
			"%sSched_ext: %s (%s%s)", level, sch->ops.name,
			lupos_scx_core_enable_state_name(state), all);
}
void lupos_scx_exit_print_task_info(const char *level, struct scx_sched *sch,
                                   enum scx_enable_state state, const char *all,
                                   const char *runnable)
{
	LUPOS_SCX_EXIT_PRINTK("print_scx_info", 8621, _printk,
			"%sSched_ext: %s (%s%s), task: runnable_at=%s",
			level, sch->ops.name, lupos_scx_core_enable_state_name(state),
			all, runnable);
}

/* Keep the five original likely/unlikely instrumentation sites distinct. */
bool lupos_scx_exit_lockup_unlikely_null(struct scx_sched *sch) { return unlikely(!sch); }
bool lupos_scx_exit_stall_unlikely_null(struct scx_sched *sch) { return unlikely(!sch); }
bool lupos_scx_exit_dump_unlikely_null(struct scx_sched *sch) { return unlikely(!sch); }
bool lupos_scx_exit_exit_likely_nonnull(struct scx_sched *sch) { return likely(sch); }
bool lupos_scx_exit_error_likely_nonnull(struct scx_sched *sch) { return likely(sch); }

#undef LUPOS_SCX_EXIT_PRINTK
#undef LUPOS_SCX_EXIT_INDEX
