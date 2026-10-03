/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef _INIT_RAMDISK_LOADER_ABI_H
#define _INIT_RAMDISK_LOADER_ABI_H

#include <linux/fs.h>

unsigned char *rust_init_rd_alloc_probe(void);
char *rust_init_rd_alloc_copy(void);
loff_t rust_init_rd_i_size_read(const struct inode *inode);

#endif
