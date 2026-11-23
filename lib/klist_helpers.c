// SPDX-License-Identifier: GPL-2.0-only
/*
 * Native leaves for the Rust klist owner: lock initialization with its
 * lockdep class, lock/IRQ primitives, the static klist_remove_lock, the
 * configured list and kref inlines, task-state transitions and the three
 * warning sites. Node lifetime, iteration and removal live in lib/klist.rs.
 */
#include <linux/build_bug.h>
#include <linux/bug.h>
#include <linux/klist.h>
#include <linux/kref.h>
#include <linux/list.h>
#include <linux/sched.h>
#include <linux/spinlock.h>
#include <linux/stddef.h>

/* lib/klist.rs mirrors struct klist_waiter and tags n_klist's low bit. */
static_assert(offsetof(struct klist_node, n_klist) == 0);
static_assert(__alignof__(struct klist) >= 2);

static DEFINE_SPINLOCK(klist_remove_lock);

void lupos_klist_lock_init(struct klist *k);
void lupos_klist_lock(struct klist *k);
void lupos_klist_unlock(struct klist *k);
unsigned long lupos_klist_lock_irqsave(struct klist *k);
void lupos_klist_unlock_irqrestore(struct klist *k, unsigned long flags);
void lupos_klist_remove_lock(void);
void lupos_klist_remove_unlock(void);
void lupos_klist_init_list_head(struct list_head *list);
void lupos_klist_list_add(struct list_head *new, struct list_head *head);
void lupos_klist_list_add_tail(struct list_head *new, struct list_head *head);
void lupos_klist_list_del(struct list_head *entry);
void lupos_klist_kref_init(struct kref *kref);
void lupos_klist_kref_get(struct kref *kref);
int lupos_klist_kref_get_unless_zero(struct kref *kref);
bool lupos_klist_kref_dec_and_test(struct kref *kref);
void lupos_klist_set_current_uninterruptible(void);
void lupos_klist_set_current_running(void);
void lupos_klist_warn_set_dead(void);
void lupos_klist_warn_kill_dead(void);
void lupos_klist_warn_release_live(void);

/* One static lockdep class for every klist lock, as in klist_init(). */
void lupos_klist_lock_init(struct klist *k)
{
	spin_lock_init(&k->k_lock);
}

void lupos_klist_lock(struct klist *k)
{
	spin_lock(&k->k_lock);
}

void lupos_klist_unlock(struct klist *k)
{
	spin_unlock(&k->k_lock);
}

unsigned long lupos_klist_lock_irqsave(struct klist *k)
{
	unsigned long flags;

	spin_lock_irqsave(&k->k_lock, flags);
	return flags;
}

void lupos_klist_unlock_irqrestore(struct klist *k, unsigned long flags)
{
	spin_unlock_irqrestore(&k->k_lock, flags);
}

void lupos_klist_remove_lock(void)
{
	spin_lock(&klist_remove_lock);
}

void lupos_klist_remove_unlock(void)
{
	spin_unlock(&klist_remove_lock);
}

void lupos_klist_init_list_head(struct list_head *list)
{
	INIT_LIST_HEAD(list);
}

void lupos_klist_list_add(struct list_head *new, struct list_head *head)
{
	list_add(new, head);
}

void lupos_klist_list_add_tail(struct list_head *new, struct list_head *head)
{
	list_add_tail(new, head);
}

void lupos_klist_list_del(struct list_head *entry)
{
	list_del(entry);
}

void lupos_klist_kref_init(struct kref *kref)
{
	kref_init(kref);
}

void lupos_klist_kref_get(struct kref *kref)
{
	kref_get(kref);
}

int lupos_klist_kref_get_unless_zero(struct kref *kref)
{
	return kref_get_unless_zero(kref);
}

/* The decrement of kref_put(); the release call stays in lib/klist.rs. */
bool lupos_klist_kref_dec_and_test(struct kref *kref)
{
	return refcount_dec_and_test(&kref->refcount);
}

void lupos_klist_set_current_uninterruptible(void)
{
	set_current_state(TASK_UNINTERRUPTIBLE);
}

void lupos_klist_set_current_running(void)
{
	__set_current_state(TASK_RUNNING);
}

/* knode_set_klist(): no knode deserves to start its life dead. */
void lupos_klist_warn_set_dead(void)
{
	WARN_ON(1);
}

/* knode_kill(): no knode should die twice. */
void lupos_klist_warn_kill_dead(void)
{
	WARN_ON(1);
}

/* klist_release(): only dead nodes are released. */
void lupos_klist_warn_release_live(void)
{
	WARN_ON(1);
}
