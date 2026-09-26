// SPDX-License-Identifier: GPL-2.0

/*
 * Helpers for C circular doubly linked list implementation.
 */

#include <linux/list.h>

__rust_helper void rust_helper_INIT_LIST_HEAD(struct list_head *list)
{
	INIT_LIST_HEAD(list);
}

__rust_helper void rust_helper_list_add_tail(struct list_head *new, struct list_head *head)
{
	list_add_tail(new, head);
}

#ifdef CONFIG_LIST_HARDENED
/* Rust and bindgen cannot describe __preserve_most. Keep only the calling
 * convention transition here; callers implement the inline list logic. */
__rust_helper bool rust_helper___list_add_valid_or_report(struct list_head *new,
						       struct list_head *prev,
						       struct list_head *next)
{
	return __list_add_valid_or_report(new, prev, next);
}
#endif
