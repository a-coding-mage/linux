/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef _LUPOS_ZLIB_KERNEL_ALLOC_H
#define _LUPOS_ZLIB_KERNEL_ALLOC_H

#include <linux/types.h>

void *lupos_zlib_kmalloc(size_t size, gfp_t flags);
void lupos_zlib_kfree(void *pointer);

#endif
