/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef LUPOS_NOTIFIER_NATIVE_BINDINGS_H
#define LUPOS_NOTIFIER_NATIVE_BINDINGS_H

/* The configured native headers alone own layouts, enum ABI and constants. */
#include <linux/errno.h>
#include <linux/kdebug.h>
#include <linux/notifier.h>

#ifdef __BINDGEN__
static const int LUPOS_NOTIFIER_EEXIST = EEXIST;
static const int LUPOS_NOTIFIER_EBUSY = EBUSY;
static const int LUPOS_NOTIFIER_ENOENT = ENOENT;
static const int LUPOS_NOTIFIER_DONE = NOTIFY_DONE;
static const int LUPOS_NOTIFIER_STOP_MASK = NOTIFY_STOP_MASK;
#endif

/* The original static die_chain has native initialized storage. Only its
 * address crosses the language boundary; no Rust spinlock is projected.
 */
struct atomic_notifier_head *lupos_notifier_die_head(void);

struct notifier_block **
lupos_notifier_atomic_head(struct atomic_notifier_head *nh);
struct notifier_block **
lupos_notifier_blocking_head(struct blocking_notifier_head *nh);
struct notifier_block **lupos_notifier_raw_head(struct raw_notifier_head *nh);
struct notifier_block **lupos_notifier_srcu_head(struct srcu_notifier_head *nh);
struct notifier_block **lupos_notifier_next_slot(struct notifier_block *n);
struct notifier_block *lupos_notifier_load(struct notifier_block **slot);
void lupos_notifier_store(struct notifier_block **slot, struct notifier_block *n);
struct notifier_block *lupos_notifier_next(struct notifier_block *n);
void lupos_notifier_set_next(struct notifier_block *n, struct notifier_block *next);
int lupos_notifier_priority(struct notifier_block *n);
bool lupos_notifier_duplicate(struct notifier_block **slot,
			      struct notifier_block *n);
struct notifier_block *
lupos_notifier_dereference_raw(struct notifier_block **slot);
struct notifier_block *lupos_notifier_access(struct notifier_block **slot);
void lupos_notifier_assign(struct notifier_block **slot, struct notifier_block *n);

/* No notifier_fn_t crosses Rust. These leaves evaluate the native field at
 * each original expression site; no function-pointer cache or null policy.
 */
void lupos_notifier_warn_duplicate(struct notifier_block *n);
#ifdef CONFIG_DEBUG_NOTIFIERS
bool lupos_notifier_invalid_callback(struct notifier_block *nb);
void lupos_notifier_warn_invalid(void);
#endif
void lupos_notifier_trace_register(struct notifier_block *n);
void lupos_notifier_trace_unregister(struct notifier_block *n);
void lupos_notifier_trace_run(struct notifier_block *nb);
int lupos_notifier_invoke(struct notifier_block *nb, unsigned long val, void *v);

unsigned long lupos_notifier_atomic_lock(struct atomic_notifier_head *nh)
	__acquires(&nh->lock);
void lupos_notifier_atomic_unlock(struct atomic_notifier_head *nh,
				 unsigned long flags) __releases(&nh->lock);
void lupos_notifier_synchronize_rcu(void);
void lupos_notifier_rcu_read_lock(void) __acquires_shared(RCU);
void lupos_notifier_rcu_read_unlock(void) __releases_shared(RCU);
bool lupos_notifier_blocking_register_booting(void);
bool lupos_notifier_blocking_unregister_booting(void);
bool lupos_notifier_srcu_register_booting(void);
bool lupos_notifier_srcu_unregister_booting(void);
void lupos_notifier_down_write(struct blocking_notifier_head *nh)
	__acquires(&nh->rwsem);
void lupos_notifier_up_write(struct blocking_notifier_head *nh)
	__releases(&nh->rwsem);
void lupos_notifier_down_read(struct blocking_notifier_head *nh)
	__acquires_shared(&nh->rwsem);
void lupos_notifier_up_read(struct blocking_notifier_head *nh)
	__releases_shared(&nh->rwsem);
void lupos_notifier_mutex_lock(struct srcu_notifier_head *nh)
	__acquires(&nh->mutex);
void lupos_notifier_mutex_unlock(struct srcu_notifier_head *nh)
	__releases(&nh->mutex);
void lupos_notifier_synchronize_srcu(struct srcu_notifier_head *nh);
int lupos_notifier_srcu_read_lock(struct srcu_notifier_head *nh)
	__acquires_shared(&nh->srcu);
void lupos_notifier_srcu_read_unlock(struct srcu_notifier_head *nh, int idx)
	__releases_shared(&nh->srcu);
void lupos_notifier_mutex_init(struct srcu_notifier_head *nh);
int lupos_notifier_init_srcu(struct srcu_notifier_head *nh);
void lupos_notifier_bug_init_srcu(void);
void lupos_notifier_warn_die_rcu(void);

/* Actual Rust body named for native blacklist metadata; not a C wrapper. */
int lupos_notifier_call_chain(struct notifier_block **nl, unsigned long val,
			      void *v, int nr_to_call, int *nr_calls);

#endif /* LUPOS_NOTIFIER_NATIVE_BINDINGS_H */
