// SPDX-License-Identifier: GPL-2.0-only
/* Native macro and inline boundaries; no completion/wake algorithm fallback. */
#include "sched_waiting_bindings.h"

/* This source proposal must not silently become a runnable scheduler owner. */
#error "Lupos waiting source-only hold: native ABI/CFI/locking admission pending"

struct task_struct *lupos_waiting_current(void)
{
	return current;
}

void lupos_waiting_init_swait_entry(struct swait_queue *wait)
{
	/* __SWAITQUEUE_INITIALIZER in linux/swait.h, at final storage address. */
	wait->task = current;
	INIT_LIST_HEAD(&wait->task_list);
}

struct swait_queue *lupos_waiting_first_swait(struct list_head *head)
{
	return list_first_entry(head, struct swait_queue, task_list);
}

void lupos_waiting_init_list_head(struct list_head *head)
{
	INIT_LIST_HEAD(head);
}

bool lupos_waiting_list_empty(const struct list_head *head)
{
	return list_empty(head);
}

bool lupos_waiting_list_empty_careful(const struct list_head *head)
{
	return list_empty_careful(head);
}

void lupos_waiting_list_del_init(struct list_head *entry)
{
	list_del_init(entry);
}

void lupos_waiting_list_add_tail(struct list_head *entry, struct list_head *head)
{
	list_add_tail(entry, head);
}

void lupos_waiting_list_splice_init(struct list_head *list, struct list_head *head)
{
	list_splice_init(list, head);
}

void lupos_waiting_raw_spin_lock_init(raw_spinlock_t *lock)
{
	raw_spin_lock_init(lock);
}

void lupos_waiting_raw_lockdep_class(raw_spinlock_t *lock,
				   struct lock_class_key *key, const char *name)
{
	lockdep_set_class_and_name(lock, key, name);
}

void lupos_waiting_raw_spin_lock_irqsave(raw_spinlock_t *lock,
				      unsigned long *flags)
{
	raw_spin_lock_irqsave(lock, *flags);
}

void lupos_waiting_raw_spin_unlock_irqrestore(raw_spinlock_t *lock,
					   unsigned long flags)
{
	raw_spin_unlock_irqrestore(lock, flags);
}

void lupos_waiting_raw_spin_lock_irq(raw_spinlock_t *lock)
{
	raw_spin_lock_irq(lock);
}

void lupos_waiting_raw_spin_unlock_irq(raw_spinlock_t *lock)
{
	raw_spin_unlock_irq(lock);
}

/* One native macro expansion per original task-state source site.
 * The wrapper frame/IP and diagnostic file/line remain unqualified.
 */

/* kernel/sched/completion.c:98: do_wait_for_common */
void lupos_waiting_completion_wait_state(int state)
{
	__set_current_state(state);
}

/* kernel/sched/swait.c:98: prepare_to_swait_exclusive */
void lupos_waiting_swait_exclusive_state(int state)
{
	set_current_state(state);
}

/* kernel/sched/swait.c:118: prepare_to_swait_event */
void lupos_waiting_swait_event_state(int state)
{
	set_current_state(state);
}

/* kernel/sched/swait.c:128: __finish_swait */
void lupos_waiting_swait_finish_locked_state(void)
{
	__set_current_state(TASK_RUNNING);
}

/* kernel/sched/swait.c:137: finish_swait */
void lupos_waiting_swait_finish_state(void)
{
	__set_current_state(TASK_RUNNING);
}

/* kernel/sched/wait.c:257: prepare_to_wait */
void lupos_waiting_wait_prepare_state(int state)
{
	set_current_state(state);
}

/* kernel/sched/wait.c:275: prepare_to_wait_exclusive */
void lupos_waiting_wait_exclusive_state(int state)
{
	set_current_state(state);
}

/* kernel/sched/wait.c:318: prepare_to_wait_event */
void lupos_waiting_wait_event_state(int state)
{
	set_current_state(state);
}

/* kernel/sched/wait.c:338: do_wait_intr */
void lupos_waiting_wait_intr_state(void)
{
	set_current_state(TASK_INTERRUPTIBLE);
}

/* kernel/sched/wait.c:355: do_wait_intr_irq */
void lupos_waiting_wait_intr_irq_state(void)
{
	set_current_state(TASK_INTERRUPTIBLE);
}

/* kernel/sched/wait.c:380: finish_wait */
void lupos_waiting_wait_finish_state(void)
{
	__set_current_state(TASK_RUNNING);
}

/* kernel/sched/wait.c:441: wait_woken */
void lupos_waiting_wait_woken_sleep_state(unsigned int mode)
{
	set_current_state(mode);
}

/* kernel/sched/wait.c:444: wait_woken */
void lupos_waiting_wait_woken_running_state(void)
{
	__set_current_state(TASK_RUNNING);
}


bool lupos_waiting_signal_pending_state(int state, struct task_struct *task)
{
	return signal_pending_state(state, task);
}

void lupos_waiting_might_sleep(void)
{
	might_sleep();
}

void lupos_waiting_complete_acquire(struct completion *x)
{
	complete_acquire(x);
}

void lupos_waiting_complete_release(struct completion *x)
{
	complete_release(x);
}

void lupos_waiting_assert_rt_threaded(void)
{
	lockdep_assert_RT_in_threaded_ctx();
}

unsigned int lupos_waiting_read_completion_done(struct completion *x)
{
	return READ_ONCE(x->done);
}

/* Plain accesses with wait.lock held, unlike the lockless READ_ONCE above. */
unsigned int lupos_waiting_completion_done_locked(struct completion *x)
{
	return x->done;
}

void lupos_waiting_set_completion_done_locked(struct completion *x,
					    unsigned int value)
{
	x->done = value;
}

/* The original C EXPORT_SYMBOL ownership, and no additional exports. */
EXPORT_SYMBOL(complete);
EXPORT_SYMBOL(complete_all);
EXPORT_SYMBOL(wait_for_completion);
EXPORT_SYMBOL(wait_for_completion_timeout);
EXPORT_SYMBOL(wait_for_completion_io);
EXPORT_SYMBOL(wait_for_completion_io_timeout);
EXPORT_SYMBOL(wait_for_completion_interruptible);
EXPORT_SYMBOL(wait_for_completion_interruptible_timeout);
EXPORT_SYMBOL(wait_for_completion_killable);
EXPORT_SYMBOL(wait_for_completion_state);
EXPORT_SYMBOL(wait_for_completion_killable_timeout);
EXPORT_SYMBOL(try_wait_for_completion);
EXPORT_SYMBOL(completion_done);
EXPORT_SYMBOL(__init_swait_queue_head);
EXPORT_SYMBOL(swake_up_locked);
EXPORT_SYMBOL(swake_up_one);
EXPORT_SYMBOL(swake_up_all);
EXPORT_SYMBOL(prepare_to_swait_exclusive);
EXPORT_SYMBOL(prepare_to_swait_event);
EXPORT_SYMBOL(finish_swait);

/* wait.c macro, list-iteration and callback boundaries. */
void lupos_waiting_spin_lock_init(spinlock_t *lock)
{
	spin_lock_init(lock);
}

void lupos_waiting_spin_lockdep_class(spinlock_t *lock,
				    struct lock_class_key *key, const char *name)
{
	lockdep_set_class_and_name(lock, key, name);
}

void lupos_waiting_spin_lock_irqsave(spinlock_t *lock, unsigned long *flags)
{
	spin_lock_irqsave(lock, *flags);
}

void lupos_waiting_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags)
{
	spin_unlock_irqrestore(lock, flags);
}

void lupos_waiting_spin_lock_irq(spinlock_t *lock)
{
	spin_lock_irq(lock);
}

void lupos_waiting_spin_unlock_irq(spinlock_t *lock)
{
	spin_unlock_irq(lock);
}

void lupos_waiting_spin_lock(spinlock_t *lock)
{
	spin_lock(lock);
}

void lupos_waiting_spin_unlock(spinlock_t *lock)
{
	spin_unlock(lock);
}

void lupos_waiting_list_add(struct list_head *entry, struct list_head *head)
{
	list_add(entry, head);
}

void lupos_waiting_list_del_init_careful(struct list_head *entry)
{
	list_del_init_careful(entry);
}

void lupos_waiting_add_wait_queue(struct wait_queue_head *head,
				struct wait_queue_entry *entry)
{
	__add_wait_queue(head, entry);
}

void lupos_waiting_add_wait_queue_tail(struct wait_queue_head *head,
				     struct wait_queue_entry *entry)
{
	__add_wait_queue_entry_tail(head, entry);
}

void lupos_waiting_remove_wait_queue(struct wait_queue_head *head,
				   struct wait_queue_entry *entry)
{
	__remove_wait_queue(head, entry);
}

struct wait_queue_entry *lupos_waiting_first_wait(struct list_head *head)
{
	return list_first_entry_or_null(head, struct wait_queue_entry, entry);
}

struct wait_queue_entry *lupos_waiting_next_wait(struct wait_queue_entry *entry,
						 struct list_head *head)
{
	if (list_is_last(&entry->entry, head))
		return NULL;
	return list_next_entry(entry, entry);
}

void lupos_waiting_assert_wait_lock(struct wait_queue_head *head)
{
	lockdep_assert_held(&head->lock);
}

int lupos_waiting_call_wait_func(struct wait_queue_entry *entry,
				 unsigned int mode, int flags, void *key)
{
	/* Dispatch only: keep the native callback type and CFI checks intact. */
	return entry->func(entry, mode, flags, key);
}

void *lupos_waiting_pollfree_key(void)
{
	return poll_to_key(EPOLLHUP | POLLFREE);
}

void lupos_waiting_warn_pollfree_active(struct wait_queue_head *head)
{
	WARN_ON_ONCE(waitqueue_active(head));
}

bool lupos_waiting_signal_pending(struct task_struct *task)
{
	return signal_pending(task);
}

void lupos_waiting_store_mb_flags(unsigned int *flags, unsigned int value)
{
	smp_store_mb(*flags, value);
}

unsigned int lupos_waiting_read_wait_flags(struct wait_queue_entry *entry)
{
	/* Preserve plain native accesses, including pre-lock prepare RMW races. */
	return entry->flags;
}

void lupos_waiting_and_wait_flags(struct wait_queue_entry *entry, unsigned int mask)
{
	entry->flags &= mask;
}

void lupos_waiting_or_wait_flags(struct wait_queue_entry *entry, unsigned int mask)
{
	entry->flags |= mask;
}

void lupos_waiting_smp_mb(void)
{
	smp_mb();
}

/* Native data-only owner keeps wait_bit.c's configured cacheline alignment. */
static wait_queue_head_t lupos_waiting_bit_wait_table[LUPOS_WAITING_WAIT_TABLE_SIZE]
	__cacheline_aligned;

wait_queue_head_t *lupos_waiting_bit_table(void)
{
	return lupos_waiting_bit_wait_table;
}

unsigned long lupos_waiting_hash_long(unsigned long value, unsigned int bits)
{
	return hash_long(value, bits);
}

unsigned long lupos_waiting_hash_ptr(const void *ptr, unsigned int bits)
{
	return hash_ptr(ptr, bits);
}

struct wait_bit_queue_entry *lupos_waiting_bit_entry(struct wait_queue_entry *entry)
{
	return container_of(entry, struct wait_bit_queue_entry, wq_entry);
}

void lupos_waiting_init_bit_entry(struct wait_bit_queue_entry *entry,
				unsigned long *word, int bit)
{
	/* DEFINE_WAIT_BIT in linux/wait_bit.h, initialized at final storage. */
	*entry = (struct wait_bit_queue_entry) {
		.key = __WAIT_BIT_KEY_INITIALIZER(word, bit),
		.wq_entry = {
			.private = current,
			.func = wake_bit_function,
			.entry = LIST_HEAD_INIT(entry->wq_entry.entry),
		},
	};
}

bool lupos_waiting_test_bit(int bit, const unsigned long *word)
{
	return test_bit(bit, word);
}

int lupos_waiting_call_bit_action(wait_bit_action_f *action,
				  struct wait_bit_key *key, int mode)
{
	/* Called only at the original action sites selected by the Rust owner. */
	return action(key, mode);
}

bool lupos_waiting_test_bit_acquire(int bit, const unsigned long *word)
{
	return test_bit_acquire(bit, word);
}

bool lupos_waiting_test_and_set_bit(int bit, unsigned long *word)
{
	return test_and_set_bit(bit, word);
}

bool lupos_waiting_waitqueue_active(struct wait_queue_head *head)
{
	return waitqueue_active(head);
}

unsigned long lupos_waiting_read_jiffies(void)
{
	return READ_ONCE(jiffies);
}

bool lupos_waiting_time_after_eq(unsigned long now, unsigned long timeout)
{
	return time_after_eq(now, timeout);
}

void __init lupos_waiting_init_bit_waitqueue_head(struct wait_queue_head *head)
{
	init_waitqueue_head(head);
}
EXPORT_SYMBOL(__init_waitqueue_head);
EXPORT_SYMBOL(add_wait_queue);
EXPORT_SYMBOL(add_wait_queue_exclusive);
EXPORT_SYMBOL_GPL(add_wait_queue_priority);
EXPORT_SYMBOL_GPL(add_wait_queue_priority_exclusive);
EXPORT_SYMBOL(remove_wait_queue);
EXPORT_SYMBOL(__wake_up);
EXPORT_SYMBOL_GPL(__wake_up_locked);
EXPORT_SYMBOL_GPL(__wake_up_locked_key);
EXPORT_SYMBOL_GPL(__wake_up_sync_key);
EXPORT_SYMBOL_GPL(__wake_up_locked_sync_key);
EXPORT_SYMBOL_GPL(__wake_up_sync);	/* For internal use only */
EXPORT_SYMBOL(prepare_to_wait);
EXPORT_SYMBOL(prepare_to_wait_exclusive);
EXPORT_SYMBOL(init_wait_entry);
EXPORT_SYMBOL(prepare_to_wait_event);
EXPORT_SYMBOL(do_wait_intr);
EXPORT_SYMBOL(do_wait_intr_irq);
EXPORT_SYMBOL(finish_wait);
EXPORT_SYMBOL(autoremove_wake_function);
EXPORT_SYMBOL(wait_woken);
EXPORT_SYMBOL(woken_wake_function);
EXPORT_SYMBOL(woken_wake_bit_function);
EXPORT_SYMBOL(bit_waitqueue);
EXPORT_SYMBOL(wake_bit_function);
EXPORT_SYMBOL(__wait_on_bit);
EXPORT_SYMBOL(out_of_line_wait_on_bit);
EXPORT_SYMBOL_GPL(out_of_line_wait_on_bit_timeout);
EXPORT_SYMBOL(__wait_on_bit_lock);
EXPORT_SYMBOL(out_of_line_wait_on_bit_lock);
EXPORT_SYMBOL(__wake_up_bit);
EXPORT_SYMBOL(wake_up_bit);
EXPORT_SYMBOL(__var_waitqueue);
EXPORT_SYMBOL(init_wait_var_entry);
EXPORT_SYMBOL(wake_up_var);
EXPORT_SYMBOL(bit_wait);
EXPORT_SYMBOL(bit_wait_io);
EXPORT_SYMBOL_GPL(bit_wait_timeout);
