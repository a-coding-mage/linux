/* SPDX-License-Identifier: GPL-2.0+ */
#ifndef LUPOS_RSEQ_BINDINGS_H
#define LUPOS_RSEQ_BINDINGS_H
#include <linux/debugfs.h>
#include <linux/hrtimer.h>
#include <linux/percpu.h>
#include <linux/prctl.h>
#include <linux/rseq_entry.h>
#include <linux/sched.h>
#include <linux/syscalls.h>
#include <linux/uaccess.h>
#include <linux/types.h>
#include <linux/rseq.h>
#include <asm/ptrace.h>

/* Private C layouts, shared with bindgen, never reproduced in Rust. */
struct rust_rseq_user_write { size_t offset; u64 value; bool wide; };
#ifdef CONFIG_RSEQ_SLICE_EXTENSION
struct slice_timer { struct hrtimer timer; void *cookie; };
#endif
struct task_struct *rust_rseq_current(void);
unsigned int rust_rseq_cpu_limit(void);
struct rseq_data *rust_rseq_data(struct task_struct *t);
unsigned int rust_rseq_task_flags(struct task_struct *t);
unsigned long rust_rseq_task_size(void);
unsigned long rust_rseq_ip(struct pt_regs *regs);
void rust_rseq_set_ip(struct pt_regs *regs, unsigned long ip);
void rust_rseq_irq_disable(void);
void rust_rseq_irq_enable(void);
void rust_rseq_preempt_disable(void);
void rust_rseq_preempt_enable(void);
void rust_rseq_cond_resched(void);
u32 rust_rseq_task_cpu(struct task_struct *t);
u32 rust_rseq_task_mm_cid(struct task_struct *t);
u32 rust_rseq_cpu_to_node(u32 cpu);
bool rust_rseq_debug_enabled(void);
void rust_rseq_control_debug(bool on);
bool rust_rseq_slice_enabled(void);
void rust_rseq_force_update(void);
void rust_rseq_reset(struct task_struct *t);
u32 rust_rseq_alloc_align(void);
bool rust_rseq_access_ok(const void __user *p, u32 len);
/* Each scope stays wholly in C, including masking and exception-table fixup. */
bool rust_rseq_read_csaddr(struct rseq __user *p, u64 *value);
bool rust_rseq_read_cs_u64(struct rseq_cs __user *p, size_t offset, u64 *value);
bool rust_rseq_read_rseq_u32(struct rseq __user *p, size_t offset, u32 *value);
bool rust_rseq_write_fields(struct rseq __user *p, const struct rust_rseq_user_write *fields, size_t count, bool rw);
int rust_rseq_get_u32(const u32 __user *p, u32 *value);
int rust_rseq_get_u64(const u64 __user *p, u64 *value);
int rust_rseq_put_u32(u32 __user *p, u32 value);
void rust_rseq_stat_inc(unsigned int field);
#ifdef CONFIG_RSEQ_STATS
unsigned int rust_rseq_next_cpu(unsigned int cpu);
void rust_rseq_read_stats(unsigned int cpu, struct rseq_stats *stats);
#endif
struct dentry *rust_rseq_debugfs_dir(const char *name);
void rust_rseq_debugfs_file(const char *name, umode_t mode, struct dentry *dir, const struct file_operations *fops);
void *rust_rseq_inode_private(struct inode *inode);
#ifdef CONFIG_TRACEPOINTS
void rust_rseq_trace_update(struct task_struct *t);
void rust_rseq_trace_ip_fixup(unsigned long ip, unsigned long start, unsigned long offset, unsigned long abort);
void rust_rseq_trace_fixup_if_enabled(unsigned long ip, unsigned long start, unsigned long offset, unsigned long abort);
void rust_rseq_trace_update_if_enabled(struct task_struct *t);
#endif
#ifdef CONFIG_RSEQ_SLICE_EXTENSION
struct slice_timer *rust_rseq_this_timer(void);
struct slice_timer *rust_rseq_cpu_timer(unsigned int cpu);
unsigned int rust_rseq_next_timer_cpu(unsigned int cpu);
void rust_rseq_lockdep_irqs_disabled(void);
void rust_rseq_need_resched(void);
void rust_rseq_set_syscall_work(struct task_struct *t);
void rust_rseq_clear_syscall_work(struct task_struct *t);
void rust_rseq_timer_start(struct hrtimer *t, ktime_t expires);
void rust_rseq_timer_setup(struct hrtimer *t, clockid_t clock, enum hrtimer_mode mode);
void rust_rseq_slice_disable(void);
enum { RUST_NR_RSEQ_SLICE_YIELD = __NR_rseq_slice_yield };
#endif
#endif
