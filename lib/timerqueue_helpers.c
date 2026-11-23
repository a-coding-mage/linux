// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Native leaves for the Rust timerqueue owner: the two WARN_ON_ONCE() sites
 * of timerqueue.c. Queue ordering and the rbtree inlines live in
 * lib/timerqueue.rs.
 */
#include <linux/bug.h>
#include <linux/build_bug.h>
#include <linux/stddef.h>
#include <linux/timerqueue.h>

/* lib/timerqueue.rs compares ktime_t expiries as signed 64-bit values. */
static_assert(sizeof(ktime_t) == 8 && (ktime_t)-1 < 0);

void lupos_timerqueue_warn_add_linked_node(void);
void lupos_timerqueue_warn_del_empty_node(void);

/* timerqueue_add(): make sure we don't add nodes that are already added. */
void lupos_timerqueue_warn_add_linked_node(void)
{
	WARN_ON_ONCE(1);
}

/* timerqueue_del(): the node must be queued. */
void lupos_timerqueue_warn_del_empty_node(void)
{
	WARN_ON_ONCE(1);
}
