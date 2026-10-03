/* SPDX-License-Identifier: GPL-2.0-only */
/* Field-address, user-copy, diagnostics, constants, and local IRQ primitives
 * for header_algorithms.rs. Include after the original fork headers when
 * generating b bindings and exactly once in the helper translation unit.
 * None of the four translated header algorithms is called by this header.
 */
#ifndef RUST_FORK_HEADER_PRIMITIVES_H
#define RUST_FORK_HEADER_PRIMITIVES_H

#include <linux/irqflags.h>
#include <linux/limits.h>
#include <linux/list.h>
#include <linux/posix-timers.h>
#include <linux/ptrace.h>
#include <linux/rseq.h>
#include <linux/sched/jobctl.h>
#include <linux/signal.h>
#include <linux/uaccess.h>

/* The Rust caller supplies the destination's known object bound. Computing
 * __builtin_object_size() here would lose the original inline caller's type
 * information and silently turn a known bound into an unknown bound. */
bool rust_fork_header_warn_copy_bound(bool too_large);
bool rust_fork_header_warn_copy_bound(bool too_large)
{
	return WARN_ON_ONCE(too_large);
}

unsigned long rust_fork_header_copy_from_user(void *dst,
		const void __user *src, unsigned long size);
unsigned long rust_fork_header_copy_from_user(void *dst,
		const void __user *src, unsigned long size)
{
	return copy_from_user(dst, src, size);
}

#define RFH_TASK_FIELD(member) \
	__typeof__(((struct task_struct *)0)->member) * \
	rust_fork_header_task_##member(struct task_struct *task); \
	__typeof__(((struct task_struct *)0)->member) * \
	rust_fork_header_task_##member(struct task_struct *task) \
	{ return &task->member; }
RFH_TASK_FIELD(ptrace_entry)
RFH_TASK_FIELD(ptraced)
RFH_TASK_FIELD(jobctl)
RFH_TASK_FIELD(ptrace)
RFH_TASK_FIELD(parent)
RFH_TASK_FIELD(real_parent)
RFH_TASK_FIELD(ptracer_cred)

sigset_t *rust_fork_header_pending_signal(struct task_struct *task);
sigset_t *rust_fork_header_pending_signal(struct task_struct *task)
{
	return &task->pending.signal;
}
unsigned long *rust_fork_header_signal_words(sigset_t *set);
unsigned long *rust_fork_header_signal_words(sigset_t *set)
{
	return set->sig;
}
unsigned int rust_fork_header_signal_word_bits(void);
unsigned int rust_fork_header_signal_word_bits(void)
{ return _NSIG_BPW; }
unsigned int rust_fork_header_signal_word_count(void);
unsigned int rust_fork_header_signal_word_count(void)
{ return _NSIG_WORDS; }

#ifdef CONFIG_RSEQ
RFH_TASK_FIELD(rseq)
u32 *rust_fork_header_rseq_cpu_id(struct task_struct *task);
u32 *rust_fork_header_rseq_cpu_id(struct task_struct *task)
{
	return &task->rseq.ids.cpu_id;
}
u32 rust_fork_header_rseq_cpu_uninitialized(void);
u32 rust_fork_header_rseq_cpu_uninitialized(void)
{
	return RSEQ_CPU_ID_UNINITIALIZED;
}
unsigned long rust_fork_header_irq_save(void);
unsigned long rust_fork_header_irq_save(void)
{
	unsigned long flags;
	local_irq_save(flags);
	return flags;
}
void rust_fork_header_irq_restore(unsigned long flags);
void rust_fork_header_irq_restore(unsigned long flags)
{
	local_irq_restore(flags);
}
#endif
#undef RFH_TASK_FIELD

#ifdef CONFIG_POSIX_TIMERS
struct posix_cputimer_base *rust_fork_header_timer_bases(struct posix_cputimers *pct);
struct posix_cputimer_base *rust_fork_header_timer_bases(struct posix_cputimers *pct)
{
	return pct->bases;
}
u64 rust_fork_header_u64_max(void);
u64 rust_fork_header_u64_max(void)
{ return U64_MAX; }
#endif
#endif
