/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_POWER_PROCESS_RUST_H
#define LUPOS_POWER_PROCESS_RUST_H

/* process.o is always built-in. Canonical bindgen adds -DMODULE; keep all
 * included declarations in the original built-in context, then restore it. */
#ifdef MODULE
#define LUPOS_PROCESS_RESTORE_MODULE
#undef MODULE
#endif

#include <linux/interrupt.h>
#include <linux/oom.h>
#include <linux/suspend.h>
#include <linux/module.h>
#include <linux/sched/debug.h>
#include <linux/sched/task.h>
#include <linux/sched/signal.h>
#include <linux/syscalls.h>
#include <linux/freezer.h>
#include <linux/delay.h>
#include <linux/workqueue.h>
#include <linux/kmod.h>
#include <linux/cpuset.h>
#include <linux/ktime.h>
#include <linux/timekeeping.h>
#include <linux/jiffies.h>

/* Types, constants and member positions come from configured C headers. */
enum {
	LUPOS_PROCESS_TIMEOUT_SIZE = sizeof(unsigned int),
	LUPOS_PROCESS_TIMEOUT_ALIGN = __alignof__(unsigned int),
	LUPOS_PROCESS_MSEC_PER_SEC = MSEC_PER_SEC,
	LUPOS_PROCESS_TASK_SIZE = sizeof(struct task_struct),
	LUPOS_PROCESS_TASK_ALIGN = __alignof__(struct task_struct),
	LUPOS_PROCESS_FLAGS_OFFSET = offsetof(struct task_struct, flags),
	LUPOS_PROCESS_SIGNAL_OFFSET = offsetof(struct task_struct, signal),
	LUPOS_PROCESS_THREAD_NODE_OFFSET = offsetof(struct task_struct, thread_node),
	LUPOS_PROCESS_SIGNAL_SIZE = sizeof(struct signal_struct),
	LUPOS_PROCESS_SIGNAL_ALIGN = __alignof__(struct signal_struct),
	LUPOS_PROCESS_THREAD_HEAD_OFFSET = offsetof(struct signal_struct, thread_head),
	LUPOS_PROCESS_PF_SUSPEND_TASK = PF_SUSPEND_TASK,
	LUPOS_PROCESS_PF_KTHREAD = PF_KTHREAD,
	LUPOS_PROCESS_UMH_FREEZING = UMH_FREEZING,
	LUPOS_PROCESS_UMH_DISABLED = UMH_DISABLED,
	LUPOS_PROCESS_USEC_PER_MSEC = USEC_PER_MSEC,
	LUPOS_PROCESS_EBUSY = EBUSY,
};

struct task_struct *lupos_process_current(void);
void lupos_process_tasklist_read_lock(void) __acquires(&tasklist_lock);
void lupos_process_tasklist_read_unlock(void) __releases(&tasklist_lock);
struct task_struct *lupos_process_next_task(struct task_struct *task);
void lupos_process_freeze_tasks_check_rcu(void);
void lupos_process_show_tasks_check_rcu(void);
void lupos_process_thaw_processes_check_rcu(void);
void lupos_process_thaw_kernel_threads_check_rcu(void);
struct list_head *lupos_process_list_next_rcu(const struct list_head *node);
struct task_struct *lupos_process_thread_entry(struct list_head *node);
unsigned long lupos_process_jiffies(void);
unsigned long lupos_process_msecs_to_jiffies(unsigned int msecs);
bool lupos_process_time_after(unsigned long now, unsigned long end);
ktime_t lupos_process_ktime_get_boottime(void);
s64 lupos_process_ktime_to_ms(ktime_t value);
void lupos_process_usleep_range(unsigned long min, unsigned long max);
bool lupos_process_pm_wakeup_pending(void);
void lupos_process_pm_wakeup_clear(void);
bool lupos_process_pm_debug_messages_on(void);
bool lupos_process_freezing(struct task_struct *task);
void lupos_process_freezer_active_inc(void);
void lupos_process_freezer_active_dec(void);
void lupos_process_usermodehelper_enable(void);
void lupos_process_freeze_processes_bug_on_atomic(void);
void lupos_process_freeze_kernel_threads_bug_on_atomic(void);
void lupos_process_warn_other_suspend_task(struct task_struct *p,
					 struct task_struct *curr);
void lupos_process_warn_current_not_suspend_task(struct task_struct *curr);
void lupos_process_trace_thaw_begin(void);
void lupos_process_trace_thaw_end(void);
void lupos_process_log_freezing(const char *what);
void lupos_process_log_failure(const char *what, const char *result,
	unsigned int seconds, unsigned int millis, unsigned int refusing, bool wq_busy);
void lupos_process_log_complete(const char *what, unsigned int seconds,
				unsigned int millis);
void lupos_process_log_restart_begin(void);
void lupos_process_log_restart_done(void);
void lupos_process_log_restart_kernel_begin(void);
void lupos_process_log_restart_kernel_done(void);

#ifdef LUPOS_PROCESS_RESTORE_MODULE
#define MODULE
#undef LUPOS_PROCESS_RESTORE_MODULE
#endif
#endif /* LUPOS_POWER_PROCESS_RUST_H */
