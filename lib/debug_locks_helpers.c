// SPDX-License-Identifier: GPL-2.0-only
/*
 * Native leaf for the Rust debug_locks owner: __debug_locks_off(), the
 * configured architecture's fully ordered xchg() on the Rust-defined flag.
 */
#include <linux/debug_locks.h>

int lupos_debug_locks_xchg_off(void);

int lupos_debug_locks_xchg_off(void)
{
	return __debug_locks_off();
}
