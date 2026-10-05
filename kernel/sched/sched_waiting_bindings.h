/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_SCHED_WAITING_BINDINGS_H
#define LUPOS_SCHED_WAITING_BINDINGS_H

/* Configured native headers, never surrogate Rust object layouts. */
#include <linux/completion.h>
#include <linux/hash.h>
#include <linux/jiffies.h>
#include <linux/kthread.h>
#include <linux/poll.h>
#include <linux/wait_bit.h>
#include <linux/sched/debug.h>
#include <linux/sched/signal.h>
#include <linux/swait.h>
#include "sched.h"

static const unsigned int LUPOS_WAITING_UINT_MAX = UINT_MAX;
static const int LUPOS_WAITING_WF_CURRENT_CPU = WF_CURRENT_CPU;
static const long LUPOS_WAITING_ERESTARTSYS = ERESTARTSYS;
static const long LUPOS_WAITING_MAX_SCHEDULE_TIMEOUT = MAX_SCHEDULE_TIMEOUT;
static const int LUPOS_WAITING_TASK_UNINTERRUPTIBLE = TASK_UNINTERRUPTIBLE;
static const int LUPOS_WAITING_TASK_INTERRUPTIBLE = TASK_INTERRUPTIBLE;
static const int LUPOS_WAITING_TASK_KILLABLE = TASK_KILLABLE;
static const int LUPOS_WAITING_TASK_RUNNING = TASK_RUNNING;
static const unsigned int LUPOS_WAITING_TASK_NORMAL = TASK_NORMAL;

struct task_struct *lupos_waiting_current(void);
void lupos_waiting_init_swait_entry(struct swait_queue *wait);
struct swait_queue *lupos_waiting_first_swait(struct list_head *head);
void lupos_waiting_init_list_head(struct list_head *head);
bool lupos_waiting_list_empty(const struct list_head *head);
bool lupos_waiting_list_empty_careful(const struct list_head *head);
void lupos_waiting_list_del_init(struct list_head *entry);
void lupos_waiting_list_add_tail(struct list_head *entry, struct list_head *head);
void lupos_waiting_list_splice_init(struct list_head *list, struct list_head *head);
void lupos_waiting_raw_spin_lock_init(raw_spinlock_t *lock);
void lupos_waiting_raw_lockdep_class(raw_spinlock_t *lock,
				   struct lock_class_key *key, const char *name);
void lupos_waiting_raw_spin_lock_irqsave(raw_spinlock_t *lock,
				      unsigned long *flags);
void lupos_waiting_raw_spin_unlock_irqrestore(raw_spinlock_t *lock,
					   unsigned long flags);
void lupos_waiting_raw_spin_lock_irq(raw_spinlock_t *lock);
void lupos_waiting_raw_spin_unlock_irq(raw_spinlock_t *lock);
void lupos_waiting_completion_wait_state(int state);
void lupos_waiting_swait_exclusive_state(int state);
void lupos_waiting_swait_event_state(int state);
void lupos_waiting_swait_finish_locked_state(void);
void lupos_waiting_swait_finish_state(void);
void lupos_waiting_wait_prepare_state(int state);
void lupos_waiting_wait_exclusive_state(int state);
void lupos_waiting_wait_event_state(int state);
void lupos_waiting_wait_intr_state(void);
void lupos_waiting_wait_intr_irq_state(void);
void lupos_waiting_wait_finish_state(void);
void lupos_waiting_wait_woken_sleep_state(unsigned int mode);
void lupos_waiting_wait_woken_running_state(void);

bool lupos_waiting_signal_pending_state(int state, struct task_struct *task);
void lupos_waiting_might_sleep(void);
void lupos_waiting_complete_acquire(struct completion *x);
void lupos_waiting_complete_release(struct completion *x);
void lupos_waiting_assert_rt_threaded(void);
unsigned int lupos_waiting_read_completion_done(struct completion *x);
unsigned int lupos_waiting_completion_done_locked(struct completion *x);
void lupos_waiting_set_completion_done_locked(struct completion *x,
					    unsigned int value);

/* Private wait_bit.c constants copied exactly, not architecture guesses. */
#define LUPOS_WAITING_WAIT_TABLE_BITS 8
#define LUPOS_WAITING_WAIT_TABLE_SIZE (1 << LUPOS_WAITING_WAIT_TABLE_BITS)
static const unsigned int LUPOS_WAITING_BITS_PER_LONG = BITS_PER_LONG;
static const unsigned int LUPOS_WAITING_WQ_FLAG_EXCLUSIVE = WQ_FLAG_EXCLUSIVE;
static const unsigned int LUPOS_WAITING_WQ_FLAG_PRIORITY = WQ_FLAG_PRIORITY;
static const unsigned int LUPOS_WAITING_WQ_FLAG_WOKEN = WQ_FLAG_WOKEN;
static const int LUPOS_WAITING_WF_SYNC = WF_SYNC;
static const int LUPOS_WAITING_EBUSY = EBUSY;
static const int LUPOS_WAITING_EINTR = EINTR;
static const int LUPOS_WAITING_EAGAIN = EAGAIN;

void lupos_waiting_spin_lock_init(spinlock_t *lock);
void lupos_waiting_spin_lockdep_class(spinlock_t *lock,
				    struct lock_class_key *key, const char *name);
void lupos_waiting_spin_lock_irqsave(spinlock_t *lock, unsigned long *flags);
void lupos_waiting_spin_unlock_irqrestore(spinlock_t *lock, unsigned long flags);
void lupos_waiting_spin_lock_irq(spinlock_t *lock);
void lupos_waiting_spin_unlock_irq(spinlock_t *lock);
void lupos_waiting_spin_lock(spinlock_t *lock);
void lupos_waiting_spin_unlock(spinlock_t *lock);
void lupos_waiting_list_add(struct list_head *entry, struct list_head *head);
void lupos_waiting_list_del_init_careful(struct list_head *entry);
void lupos_waiting_add_wait_queue(struct wait_queue_head *head,
				struct wait_queue_entry *entry);
void lupos_waiting_add_wait_queue_tail(struct wait_queue_head *head,
				     struct wait_queue_entry *entry);
void lupos_waiting_remove_wait_queue(struct wait_queue_head *head,
				   struct wait_queue_entry *entry);
struct wait_queue_entry *lupos_waiting_first_wait(struct list_head *head);
struct wait_queue_entry *lupos_waiting_next_wait(struct wait_queue_entry *entry,
						 struct list_head *head);
void lupos_waiting_assert_wait_lock(struct wait_queue_head *head);
int lupos_waiting_call_wait_func(struct wait_queue_entry *entry,
				 unsigned int mode, int flags, void *key);
void *lupos_waiting_pollfree_key(void);
void lupos_waiting_warn_pollfree_active(struct wait_queue_head *head);
bool lupos_waiting_signal_pending(struct task_struct *task);
void lupos_waiting_store_mb_flags(unsigned int *flags, unsigned int value);
unsigned int lupos_waiting_read_wait_flags(struct wait_queue_entry *entry);
void lupos_waiting_and_wait_flags(struct wait_queue_entry *entry, unsigned int mask);
void lupos_waiting_or_wait_flags(struct wait_queue_entry *entry, unsigned int mask);
void lupos_waiting_smp_mb(void);
wait_queue_head_t *lupos_waiting_bit_table(void);
unsigned long lupos_waiting_hash_long(unsigned long value, unsigned int bits);
unsigned long lupos_waiting_hash_ptr(const void *ptr, unsigned int bits);
struct wait_bit_queue_entry *lupos_waiting_bit_entry(struct wait_queue_entry *entry);
void lupos_waiting_init_bit_entry(struct wait_bit_queue_entry *entry,
				unsigned long *word, int bit);
int lupos_waiting_call_bit_action(wait_bit_action_f *action,
				  struct wait_bit_key *key, int mode);
bool lupos_waiting_test_bit(int bit, const unsigned long *word);
bool lupos_waiting_test_bit_acquire(int bit, const unsigned long *word);
bool lupos_waiting_test_and_set_bit(int bit, unsigned long *word);
bool lupos_waiting_waitqueue_active(struct wait_queue_head *head);
unsigned long lupos_waiting_read_jiffies(void);
bool lupos_waiting_time_after_eq(unsigned long now, unsigned long timeout);
void lupos_waiting_init_bit_waitqueue_head(struct wait_queue_head *head);

#endif /* LUPOS_SCHED_WAITING_BINDINGS_H */
