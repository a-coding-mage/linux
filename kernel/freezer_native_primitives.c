// SPDX-License-Identifier: GPL-2.0-only
/* Native storage and primitives; existing Rust retains all freezer decisions. */
#error "SOURCE ONLY HOLD: freezer native ABI, state and instrumentation are unqualified"
#include "freezer_native_bindings.h"

/* Original native storage, names and initializers. No projected Rust objects. */
DEFINE_STATIC_KEY_FALSE(freezer_active);
bool pm_freezing;
bool pm_nosig_freezing;
static DEFINE_SPINLOCK(freezer_lock);

spinlock_t *lupos_freezer_lock(void) { return &freezer_lock; }
void lupos_freezer_spin_lock_irq(spinlock_t *lock) { spin_lock_irq(lock); }
void lupos_freezer_spin_unlock_irq(spinlock_t *lock) { spin_unlock_irq(lock); }
unsigned long lupos_freezer_spin_lock_irqsave(spinlock_t *lock)
{
	unsigned long flags;
	spin_lock_irqsave(lock, flags);
	return flags;
}
void lupos_freezer_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags)
{
	spin_unlock_irqrestore(lock, flags);
}
/*
 * Exact operations of guard(spinlock_irqsave), including !RT IRQ nesting.
 */
void lupos_freezer_spin_lock_irq_disable(spinlock_t *lock)
{
	spin_lock_irq_disable(lock);
}
void lupos_freezer_spin_unlock_irq_enable(spinlock_t *lock)
{
	spin_unlock_irq_enable(lock);
}
void lupos_freezer_pi_lock_irq(struct task_struct *p) { raw_spin_lock_irq(&p->pi_lock); }
void lupos_freezer_pi_unlock_irq(struct task_struct *p) { raw_spin_unlock_irq(&p->pi_lock); }
struct task_struct *lupos_freezer_current(void) { return current; }
unsigned int lupos_freezer_task_flags(struct task_struct *p) { return p->flags; }
bool lupos_freezer_oom_victim(struct task_struct *p) { return tsk_is_oom_victim(p); }
bool lupos_freezer_cgroup_freezing(struct task_struct *p) { return cgroup1_freezing(p); }
bool lupos_freezer_pm(void) { return pm_freezing; }
bool lupos_freezer_pm_nosig(void) { return pm_nosig_freezing; }
bool lupos_freezer_freezing(struct task_struct *p) { return freezing(p); }
bool lupos_freezer_kthread_should_stop(void) { return kthread_should_stop(); }
bool lupos_freezer_task_runnable(struct task_struct *p) { return task_is_runnable(p); }
bool lupos_freezer_task_curr(struct task_struct *p) { return task_curr(p); }
unsigned int lupos_freezer_current_state(void) { return get_current_state(); }
unsigned int lupos_freezer_state_once(struct task_struct *p) { return READ_ONCE(p->__state); }
unsigned int lupos_freezer_saved_state(struct task_struct *p) { return p->saved_state; }
void lupos_freezer_write_state(struct task_struct *p, unsigned int state) { WRITE_ONCE(p->__state, state); }
void lupos_freezer_write_saved(struct task_struct *p, unsigned int state) { p->saved_state = state; }
void lupos_freezer_save_plain_state(struct task_struct *p) { p->saved_state = p->__state; }
void lupos_freezer_current_running(void) { __set_current_state(TASK_RUNNING); }
void lupos_freezer_schedule(void) { schedule(); }
void lupos_freezer_wake_state(struct task_struct *p, unsigned int state) { wake_up_state(p, state); }
struct sighand_struct *lupos_freezer_lock_sighand(struct task_struct *p, unsigned long *flags)
{
	return lock_task_sighand(p, flags);
}
void lupos_freezer_unlock_sighand(struct task_struct *p, unsigned long *flags)
{
	unlock_task_sighand(p, flags);
}
void lupos_freezer_signal_wake(struct task_struct *p) { signal_wake_up(p, 0); }
void lupos_freezer_rcu_read_lock(void) { rcu_read_lock(); }
void lupos_freezer_rcu_read_unlock(void) { rcu_read_unlock(); }
struct task_struct *lupos_freezer_first_thread(struct task_struct *p)
{
	struct task_struct *t;
	__list_check_rcu(dummy, lockdep_is_held(&tasklist_lock), 0);
	t = list_entry_rcu(p->signal->thread_head.next, struct task_struct, thread_node);
	return &t->thread_node == &p->signal->thread_head ? NULL : t;
}
struct task_struct *lupos_freezer_next_thread(struct task_struct *p, struct task_struct *t)
{
	t = list_entry_rcu(t->thread_node.next, struct task_struct, thread_node);
	return &t->thread_node == &p->signal->thread_head ? NULL : t;
}
void lupos_freezer_clear_nofreeze(void) { current->flags &= ~PF_NOFREEZE; }
void lupos_freezer_might_sleep(void) { might_sleep(); }
bool lupos_freezer_try_to_freeze(void) { return try_to_freeze(); }

/* Distinct original diagnostic operands and sites, not original PC attribution. */
void lupos_freezer_debug_enter(void) { pr_debug("%s entered refrigerator\n", current->comm); }
void lupos_freezer_debug_leave(void) { pr_debug("%s left refrigerator\n", current->comm); }
void lupos_freezer_warn_refrigerator(unsigned int state) { WARN_ON_ONCE(state && !(state & TASK_NORMAL)); }
void lupos_freezer_warn_freezable(unsigned int state) { WARN_ON_ONCE(!(state & TASK_NORMAL)); }
#ifdef CONFIG_LOCKDEP
void lupos_freezer_warn_locks(struct task_struct *p) { WARN_ON_ONCE(debug_locks && p->lockdep_depth); }
#endif

/* Actual C signatures, no callback-valued parameter through the Rust FFI. */
int lupos_freezer_call_set_frozen(struct task_struct *p)
{
	return task_call_func(p, lupos_freezer_set_task_frozen, NULL);
}
int lupos_freezer_call_restore(struct task_struct *p)
{
	return task_call_func(p, lupos_freezer_restore_state, NULL);
}
