// SPDX-License-Identifier: GPL-2.0
/*
 * Native leaves for the Rust seq_buf owner: the WARN_ON(), WARN_ON_ONCE()
 * and BUG_ON() sites of seq_buf.c and the seq_buf.h inlines. Buffer
 * accounting, formatting and copying live in lib/seq_buf.rs.
 */
#include <linux/bug.h>
#include <linux/build_bug.h>
#include <linux/printk.h>
#include <linux/seq_buf.h>
#include <linux/stddef.h>

/* lib/seq_buf.rs spells these values and reads the generated layout. */
static_assert(DUMP_PREFIX_ADDRESS == 1);
static_assert(DUMP_PREFIX_OFFSET == 2);
static_assert(offsetof(struct seq_buf, buffer) == 0);
static_assert(offsetof(struct seq_buf, size) == sizeof(char *));
static_assert(offsetof(struct seq_buf, len) == sizeof(char *) + sizeof(size_t));
static_assert(sizeof(struct seq_buf) == sizeof(char *) + 2 * sizeof(size_t));

void lupos_seq_buf_warn_on(void);
void lupos_seq_buf_warn_on_once(void);
void __noreturn lupos_seq_buf_bug(void);

void lupos_seq_buf_warn_on(void)
{
	WARN_ON(1);
}

/* seq_buf_putmem_hex(): j == 0 || j/2 > len. */
void lupos_seq_buf_warn_on_once(void)
{
	WARN_ON_ONCE(1);
}

/* seq_buf_commit(): num must be negative on overflow. */
void __noreturn lupos_seq_buf_bug(void)
{
	BUG();
}
