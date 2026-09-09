/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2025 LG Electronics Co., Ltd.
 */

// Dependencies supplied by the Linux kernel and NTFS sources:
// #include <linux/pagemap.h>
// #include <linux/iomap.h>
// #include "volume.h"
// #include "inode.h"

unsafe extern "C" {
    pub static ntfs_write_iomap_ops: iomap_ops;
    pub static ntfs_read_iomap_ops: iomap_ops;
    pub static ntfs_seek_iomap_ops: iomap_ops;
    pub static ntfs_page_mkwrite_iomap_ops: iomap_ops;
    pub static ntfs_dio_iomap_ops: iomap_ops;
    pub static ntfs_writeback_ops: iomap_writeback_ops;
    pub static ntfs_iomap_folio_ops: iomap_write_ops;
    pub fn ntfs_dio_zero_range(inode: *mut inode, offset: i64, length: i64) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
