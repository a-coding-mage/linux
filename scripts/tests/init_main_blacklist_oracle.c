/* SPDX-License-Identifier: GPL-2.0-only */
#include <linux/init.h>
#include <linux/kallsyms.h>
#include <linux/list.h>
#include <linux/memblock.h>
#include <linux/printk.h>
#include <linux/string.h>
#include "original.inc"

int original_blacklist(char *, initcall_t, int);
struct list_head *original_head(void);
char *original_entry_buffer(struct list_head *);

int original_blacklist(char *value, initcall_t function, int action)
{
	return action ? initcall_blacklisted(function) : initcall_blacklist(value);
}

struct list_head *original_head(void)
{
#ifdef CONFIG_KALLSYMS
	return &blacklisted_initcalls;
#else
	return NULL;
#endif
}

char *original_entry_buffer(struct list_head *node)
{
#ifdef CONFIG_KALLSYMS
	return list_entry(node, struct blacklist_entry, next)->buf;
#else
	return NULL;
#endif
}
