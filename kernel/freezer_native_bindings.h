/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_FREEZER_NATIVE_BINDINGS_H
#define LUPOS_FREEZER_NATIVE_BINDINGS_H

#include <linux/interrupt.h>
#include <linux/suspend.h>
#include <linux/export.h>
#include <linux/syscalls.h>
#include <linux/freezer.h>
#include <linux/oom.h>
#include <linux/kthread.h>
#include <linux/sched/signal.h>

#ifdef __BINDGEN__
#define FREEZER_UINT(alias, value) static const unsigned int LUPOS_FREEZER_##alias = value;
FREEZER_UINT(PF_NOFREEZE, PF_NOFREEZE)
FREEZER_UINT(PF_SUSPEND_TASK, PF_SUSPEND_TASK)
FREEZER_UINT(PF_KTHREAD, PF_KTHREAD)
FREEZER_UINT(TASK_FROZEN, TASK_FROZEN)
FREEZER_UINT(TASK_RUNNING, TASK_RUNNING)
FREEZER_UINT(TASK_NORMAL, TASK_NORMAL)
FREEZER_UINT(TASK_FREEZABLE, TASK_FREEZABLE)
FREEZER_UINT(TASK_STOPPED, __TASK_STOPPED)
FREEZER_UINT(TASK_TRACED, __TASK_TRACED)
#ifdef CONFIG_LOCKDEP
FREEZER_UINT(TASK_FREEZABLE_UNSAFE, __TASK_FREEZABLE_UNSAFE)
#endif
#undef FREEZER_UINT
#endif

spinlock_t *lupos_freezer_lock(void);
void lupos_freezer_spin_lock_irq(spinlock_t *lock) __acquires(lock);
void lupos_freezer_spin_unlock_irq(spinlock_t *lock) __releases(lock);
unsigned long lupos_freezer_spin_lock_irqsave(spinlock_t *lock) __acquires(lock);
void lupos_freezer_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags) __releases(lock);
void lupos_freezer_spin_lock_irq_disable(spinlock_t *lock) __acquires(lock);
void lupos_freezer_spin_unlock_irq_enable(spinlock_t *lock) __releases(lock);
void lupos_freezer_pi_lock_irq(struct task_struct *p) __acquires(&p->pi_lock);
void lupos_freezer_pi_unlock_irq(struct task_struct *p) __releases(&p->pi_lock);
struct task_struct *lupos_freezer_current(void);
unsigned int lupos_freezer_task_flags(struct task_struct *p);
bool lupos_freezer_oom_victim(struct task_struct *p);
bool lupos_freezer_cgroup_freezing(struct task_struct *p);
bool lupos_freezer_pm(void);
bool lupos_freezer_pm_nosig(void);
bool lupos_freezer_freezing(struct task_struct *p);
bool lupos_freezer_kthread_should_stop(void);
bool lupos_freezer_task_runnable(struct task_struct *p);
bool lupos_freezer_task_curr(struct task_struct *p);
unsigned int lupos_freezer_current_state(void);
unsigned int lupos_freezer_state_once(struct task_struct *p);
unsigned int lupos_freezer_saved_state(struct task_struct *p);
void lupos_freezer_write_state(struct task_struct *p, unsigned int state);
void lupos_freezer_write_saved(struct task_struct *p, unsigned int state);
void lupos_freezer_save_plain_state(struct task_struct *p);
void lupos_freezer_current_running(void);
void lupos_freezer_schedule(void);
void lupos_freezer_wake_state(struct task_struct *p, unsigned int state);
struct sighand_struct *lupos_freezer_lock_sighand(struct task_struct *p, unsigned long *flags)
	__cond_acquires(nonnull, &p->sighand->siglock);
void lupos_freezer_unlock_sighand(struct task_struct *p, unsigned long *flags)
	__releases(&p->sighand->siglock);
void lupos_freezer_signal_wake(struct task_struct *p);
void lupos_freezer_rcu_read_lock(void) __acquires_shared(RCU);
void lupos_freezer_rcu_read_unlock(void) __releases_shared(RCU);
struct task_struct *lupos_freezer_first_thread(struct task_struct *p);
struct task_struct *lupos_freezer_next_thread(struct task_struct *p, struct task_struct *t);
void lupos_freezer_clear_nofreeze(void);
void lupos_freezer_might_sleep(void);
bool lupos_freezer_try_to_freeze(void);
void lupos_freezer_debug_enter(void);
void lupos_freezer_debug_leave(void);
void lupos_freezer_warn_refrigerator(unsigned int state);
void lupos_freezer_warn_freezable(unsigned int state);
#ifdef CONFIG_LOCKDEP
void lupos_freezer_warn_locks(struct task_struct *p);
#endif

/* Fixed native callback entries call the sole Rust bodies synchronously. */
int lupos_freezer_set_task_frozen(struct task_struct *p, void *arg);
int lupos_freezer_restore_state(struct task_struct *p, void *arg);
int lupos_freezer_call_set_frozen(struct task_struct *p);
int lupos_freezer_call_restore(struct task_struct *p);

#endif /* LUPOS_FREEZER_NATIVE_BINDINGS_H */
