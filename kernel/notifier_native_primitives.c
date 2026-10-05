// SPDX-License-Identifier: GPL-2.0-only
/*
 * Inactive native layout/storage, primitive, diagnostic and callback leaves.
 * The existing Rust owner retains every chain walk and lifecycle decision.
 * Original compilation/instrumentation origin: kernel/notifier.o.
 */
#include "notifier_native_bindings.h"
#include "notifier_rust_guard.h"
#include <linux/bug.h>
#include <linux/kernel.h>
#include <linux/rcupdate.h>
#include <linux/vmalloc.h>
#include <trace/events/notifier.h>

/* Preserve the original storage linkage and macro operand. The name is part
 * of the configured spinlock initializer, including its lockdep name.
 */
static ATOMIC_NOTIFIER_HEAD(die_chain);

struct atomic_notifier_head *lupos_notifier_die_head(void)
{
	return &die_chain;
}

struct notifier_block **
lupos_notifier_atomic_head(struct atomic_notifier_head *nh)
{
	return &nh->head;
}

struct notifier_block **
lupos_notifier_blocking_head(struct blocking_notifier_head *nh)
{
	return &nh->head;
}

struct notifier_block **lupos_notifier_raw_head(struct raw_notifier_head *nh)
{
	return &nh->head;
}

struct notifier_block **lupos_notifier_srcu_head(struct srcu_notifier_head *nh)
{
	return &nh->head;
}

struct notifier_block **lupos_notifier_next_slot(struct notifier_block *n)
{
	return &n->next;
}

struct notifier_block *lupos_notifier_load(struct notifier_block **slot)
{
	/* Original writer-side plain load, not READ_ONCE or an RCU operation. */
	return *slot;
}

void lupos_notifier_store(struct notifier_block **slot, struct notifier_block *n)
{
	/* Initialization only, before publication. */
	*slot = n;
}

struct notifier_block *lupos_notifier_next(struct notifier_block *n)
{
	return n->next;
}

void lupos_notifier_set_next(struct notifier_block *n, struct notifier_block *next)
{
	n->next = next;
}

int lupos_notifier_priority(struct notifier_block *n)
{
	return n->priority;
}

bool lupos_notifier_duplicate(struct notifier_block **slot,
			      struct notifier_block *n)
{
	return unlikely(*slot == n);
}

struct notifier_block *
lupos_notifier_dereference_raw(struct notifier_block **slot)
{
	return rcu_dereference_raw(*slot);
}

struct notifier_block *lupos_notifier_access(struct notifier_block **slot)
{
	return rcu_access_pointer(*slot);
}

void lupos_notifier_assign(struct notifier_block **slot, struct notifier_block *n)
{
	rcu_assign_pointer(*slot, n);
}

void lupos_notifier_warn_duplicate(struct notifier_block *n)
{
	WARN(1, "notifier callback %ps already registered", n->notifier_call);
}

#ifdef CONFIG_DEBUG_NOTIFIERS
bool lupos_notifier_invalid_callback(struct notifier_block *nb)
{
	/* Keep the configured native function-descriptor/text validation. */
	return unlikely(!func_ptr_is_kernel_text(nb->notifier_call));
}

void lupos_notifier_warn_invalid(void)
{
	WARN(1, "Invalid notifier called!");
}
#endif

void lupos_notifier_trace_register(struct notifier_block *n)
{
	trace_notifier_register((void *)n->notifier_call);
}

void lupos_notifier_trace_unregister(struct notifier_block *n)
{
	trace_notifier_unregister((void *)n->notifier_call);
}

void lupos_notifier_trace_run(struct notifier_block *nb)
{
	trace_notifier_run((void *)nb->notifier_call);
}

int lupos_notifier_invoke(struct notifier_block *nb, unsigned long val, void *v)
{
	/* Actual notifier_fn_t indirect call; no fabricated Option/CFI identity,
	 * null-skip branch, old-C traversal or instrumentation exemption.
	 */
	return nb->notifier_call(nb, val, v);
}

unsigned long lupos_notifier_atomic_lock(struct atomic_notifier_head *nh)
	__acquires(&nh->lock)
{
	unsigned long flags;

	spin_lock_irqsave(&nh->lock, flags);
	return flags;
}

void lupos_notifier_atomic_unlock(struct atomic_notifier_head *nh,
				 unsigned long flags)
	__releases(&nh->lock)
{
	spin_unlock_irqrestore(&nh->lock, flags);
}

void lupos_notifier_synchronize_rcu(void)
{
	synchronize_rcu();
}

void lupos_notifier_rcu_read_lock(void)
	__acquires_shared(RCU)
{
	rcu_read_lock();
}

void lupos_notifier_rcu_read_unlock(void)
	__releases_shared(RCU)
{
	rcu_read_unlock();
}

/* The original four boot checks have separate unlikely profiling sites.
 * Keep that separation; new leaf attribution and Rust branch coverage still
 * require qualification and are not supplied by these native expressions.
 */
bool lupos_notifier_blocking_register_booting(void)
{
	return unlikely(system_state == SYSTEM_BOOTING);
}

bool lupos_notifier_blocking_unregister_booting(void)
{
	return unlikely(system_state == SYSTEM_BOOTING);
}

bool lupos_notifier_srcu_register_booting(void)
{
	return unlikely(system_state == SYSTEM_BOOTING);
}

bool lupos_notifier_srcu_unregister_booting(void)
{
	return unlikely(system_state == SYSTEM_BOOTING);
}

void lupos_notifier_down_write(struct blocking_notifier_head *nh)
	__acquires(&nh->rwsem)
{
	down_write(&nh->rwsem);
}

void lupos_notifier_up_write(struct blocking_notifier_head *nh)
	__releases(&nh->rwsem)
{
	up_write(&nh->rwsem);
}

void lupos_notifier_down_read(struct blocking_notifier_head *nh)
	__acquires_shared(&nh->rwsem)
{
	down_read(&nh->rwsem);
}

void lupos_notifier_up_read(struct blocking_notifier_head *nh)
	__releases_shared(&nh->rwsem)
{
	up_read(&nh->rwsem);
}

void lupos_notifier_mutex_lock(struct srcu_notifier_head *nh)
	__acquires(&nh->mutex)
{
	mutex_lock(&nh->mutex);
}

void lupos_notifier_mutex_unlock(struct srcu_notifier_head *nh)
	__releases(&nh->mutex)
{
	mutex_unlock(&nh->mutex);
}

void lupos_notifier_synchronize_srcu(struct srcu_notifier_head *nh)
{
	synchronize_srcu(&nh->srcu);
}

int lupos_notifier_srcu_read_lock(struct srcu_notifier_head *nh)
	__acquires_shared(&nh->srcu)
{
	return srcu_read_lock(&nh->srcu);
}

void lupos_notifier_srcu_read_unlock(struct srcu_notifier_head *nh, int idx)
	__releases_shared(&nh->srcu)
{
	srcu_read_unlock(&nh->srcu, idx);
}

void lupos_notifier_mutex_init(struct srcu_notifier_head *nh)
{
	mutex_init(&nh->mutex);
}

int lupos_notifier_init_srcu(struct srcu_notifier_head *nh)
{
	return init_srcu_struct(&nh->srcu);
}

void lupos_notifier_bug_init_srcu(void)
{
	BUG();
}

void lupos_notifier_warn_die_rcu(void)
{
	/* Keep the operand in the original macro: CONFIG_PROVE_RCU and lockdep
	 * short-circuiting decide whether rcu_is_watching() is evaluated.
	 */
	RCU_LOCKDEP_WARN(!rcu_is_watching(),
			 "notify_die called but RCU thinks we're quiescent");
}
