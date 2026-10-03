// SPDX-License-Identifier: GPL-2.0-only
/*
 * kmalloc's compiler boundary for the Rust root-mount owner. Allocation hooks
 * emit per-callsite/per-CPU metadata, and typed slab partitions use a Clang
 * allocation-token builtin unavailable in Rust. Keep each original allocation
 * expression separate. All string handling, errors, lifetime, root selection,
 * retries and namespace transitions are implemented in do_mounts.rs.
 */
#include <linux/init.h>
#include <linux/slab.h>

char *rust_init_root_mounts_alloc_data_page(void);
char *rust_init_root_mounts_alloc_fs_names(void);
char *rust_init_root_mounts_alloc_nodev_names(void);

char *__init rust_init_root_mounts_alloc_data_page(void)
{
	return kmalloc(PAGE_SIZE, GFP_KERNEL);
}

char *__init rust_init_root_mounts_alloc_fs_names(void)
{
	return kmalloc(PAGE_SIZE, GFP_KERNEL);
}

char *__init rust_init_root_mounts_alloc_nodev_names(void)
{
	return kmalloc(PAGE_SIZE, GFP_KERNEL);
}

/* SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783 */
