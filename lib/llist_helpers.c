// SPDX-License-Identifier: GPL-2.0-only
/*
 * Native leaves for the Rust lock-less list owner: the configured
 * architecture's acquire load and fully ordered pointer try_cmpxchg().
 * The list algorithms live in lib/llist.rs.
 */
#include <linux/atomic.h>
#include <linux/build_bug.h>
#include <linux/llist.h>
#include <linux/stddef.h>

/* lib/llist.rs reads these fields through the generated layouts. */
static_assert(offsetof(struct llist_head, first) == 0);
static_assert(offsetof(struct llist_node, next) == 0);

struct llist_node *lupos_llist_load_acquire(struct llist_node **first);
bool lupos_llist_try_cmpxchg(struct llist_node **first, struct llist_node **old,
			     struct llist_node *new);

struct llist_node *lupos_llist_load_acquire(struct llist_node **first)
{
	return smp_load_acquire(first);
}

bool lupos_llist_try_cmpxchg(struct llist_node **first, struct llist_node **old,
			     struct llist_node *new)
{
	return try_cmpxchg(first, old, new);
}
