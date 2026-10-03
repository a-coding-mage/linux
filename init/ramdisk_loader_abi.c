// SPDX-License-Identifier: GPL-2.0-only
/*
 * Compiler/architecture boundaries only. kmalloc expands allocation-tag and
 * typed-slab compiler macros at the two original callsites. Typed local
 * destinations retain the original unsigned-char/char allocation inference.
 * i_size_read invokes
 * the canonical inode-size concurrency primitive (architecture/config dependent
 * acquire load, seqcount, or preemption protection). No image recognition,
 * copying, decompression, error handling or ownership logic lives here.
 */
#include <linux/init.h>
#include <linux/slab.h>
#include "ramdisk_loader_abi.h"

unsigned char *__init rust_init_rd_alloc_probe(void)
{
	unsigned char *buffer = kmalloc(512, GFP_KERNEL);

	return buffer;
}

char *__init rust_init_rd_alloc_copy(void)
{
	char *buffer = kmalloc(BLOCK_SIZE, GFP_KERNEL);

	return buffer;
}

/* nr_blocks(), unlike the loader functions, was not annotated __init. */
loff_t rust_init_rd_i_size_read(const struct inode *inode)
{
	return i_size_read(inode);
}
