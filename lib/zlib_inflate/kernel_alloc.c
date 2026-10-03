// SPDX-License-Identifier: GPL-2.0-only
/* Allocation macro boundary only; all decompression lives in Rust. */
#include <linux/slab.h>
#include "kernel_alloc.h"

void *lupos_zlib_kmalloc(size_t size, gfp_t flags)
{
	return kmalloc(size, flags);
}

void lupos_zlib_kfree(void *pointer)
{
	kfree(pointer);
}
