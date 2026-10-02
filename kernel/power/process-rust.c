// SPDX-License-Identifier: GPL-2.0
/* Configured macros, header inlines and diagnostic metadata only.
 * Task traversal and all freezer algorithms are implemented in process.rs.
 */
#include "process-rust.h"
#include <trace/events/power.h>

struct task_struct *lupos_process_current(void)
{
	return current;
}

void lupos_process_tasklist_read_lock(void)
{
	read_lock(&tasklist_lock);
}

void lupos_process_tasklist_read_unlock(void)
{
	read_unlock(&tasklist_lock);
}

struct task_struct *lupos_process_next_task(struct task_struct *task)
{
	return next_task(task);
}

/* __list_check_rcu expands to RCU_LOCKDEP_WARN with per-site __warned state.
 * Keep four expansions, matching process.c's four distinct thread walks. */
void lupos_process_freeze_tasks_check_rcu(void)
{
	__list_check_rcu(dummy, lockdep_is_held(&tasklist_lock), 0);
}

void lupos_process_show_tasks_check_rcu(void)
{
	__list_check_rcu(dummy, lockdep_is_held(&tasklist_lock), 0);
}

void lupos_process_thaw_processes_check_rcu(void)
{
	__list_check_rcu(dummy, lockdep_is_held(&tasklist_lock), 0);
}

void lupos_process_thaw_kernel_threads_check_rcu(void)
{
	__list_check_rcu(dummy, lockdep_is_held(&tasklist_lock), 0);
}

struct list_head *lupos_process_list_next_rcu(const struct list_head *node)
{
	return READ_ONCE(node->next);
}

struct task_struct *lupos_process_thread_entry(struct list_head *node)
{
	return list_entry(node, struct task_struct, thread_node);
}

unsigned long lupos_process_jiffies(void)
{
	return jiffies;
}

unsigned long lupos_process_msecs_to_jiffies(unsigned int msecs)
{
	return msecs_to_jiffies(msecs);
}

bool lupos_process_time_after(unsigned long now, unsigned long end)
{
	return time_after(now, end);
}

ktime_t lupos_process_ktime_get_boottime(void)
{
	return ktime_get_boottime();
}

s64 lupos_process_ktime_to_ms(ktime_t value)
{
	return ktime_to_ms(value);
}

void lupos_process_usleep_range(unsigned long min, unsigned long max)
{
	usleep_range(min, max);
}

/* These PM interfaces become static inlines/macros without PM_SLEEP/DEBUG. */
bool lupos_process_pm_wakeup_pending(void)
{
	return pm_wakeup_pending();
}

void lupos_process_pm_wakeup_clear(void)
{
	pm_wakeup_clear(0);
}

bool lupos_process_pm_debug_messages_on(void)
{
	return pm_debug_messages_on;
}

bool lupos_process_freezing(struct task_struct *task)
{
	return freezing(task);
}

void lupos_process_freezer_active_inc(void)
{
	static_branch_inc(&freezer_active);
}

void lupos_process_freezer_active_dec(void)
{
	static_branch_dec(&freezer_active);
}

void lupos_process_usermodehelper_enable(void)
{
	usermodehelper_enable();
}

/* Distinct source sites preserve both original BUG checks and WARN sites. */
void lupos_process_freeze_processes_bug_on_atomic(void)
{
	BUG_ON(in_atomic());
}

void lupos_process_freeze_kernel_threads_bug_on_atomic(void)
{
	BUG_ON(in_atomic());
}

void lupos_process_warn_other_suspend_task(struct task_struct *p,
					 struct task_struct *curr)
{
	WARN_ON((p != curr) && (p->flags & PF_SUSPEND_TASK));
}

void lupos_process_warn_current_not_suspend_task(struct task_struct *curr)
{
	WARN_ON(!(curr->flags & PF_SUSPEND_TASK));
}

/* Keep TPS/tracepoint_string metadata at the two original event call sites. */
void lupos_process_trace_thaw_begin(void)
{
	trace_suspend_resume(TPS("thaw_processes"), 0, true);
}

void lupos_process_trace_thaw_end(void)
{
	trace_suspend_resume(TPS("thaw_processes"), 0, false);
}

/* Preserve printk levels, literal formats, argument widths and ordering. */
void lupos_process_log_freezing(const char *what)
{
	pr_info("Freezing %s\n", what);
}

void lupos_process_log_failure(const char *what, const char *result,
	unsigned int seconds, unsigned int millis, unsigned int refusing, bool wq_busy)
{
	pr_err("Freezing %s %s after %d.%03d seconds "
	       "(%d tasks refusing to freeze, wq_busy=%d):\n", what, result,
	       seconds, millis, refusing, wq_busy);
}

void lupos_process_log_complete(const char *what, unsigned int seconds,
				unsigned int millis)
{
	pr_info("Freezing %s completed (elapsed %d.%03d seconds)\n",
		what, seconds, millis);
}

void lupos_process_log_restart_begin(void)
{
	pr_info("Restarting tasks: Starting\n");
}

void lupos_process_log_restart_done(void)
{
	pr_info("Restarting tasks: Done\n");
}

void lupos_process_log_restart_kernel_begin(void)
{
	pr_info("Restarting kernel threads ...\n");
}

void lupos_process_log_restart_kernel_done(void)
{
	pr_info("Done restarting kernel threads.\n");
}
