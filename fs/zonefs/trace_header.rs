/* SPDX-License-Identifier: GPL-2.0 */
/*
 * zonefs filesystem driver tracepoints.
 *
 * Copyright (C) 2021 Western Digital Corporation or its affiliates.
 */

/* C header guard and tracepoint includes are supplied by the surrounding build. */

/* Equivalent of: #define show_dev(dev) MAJOR(dev), MINOR(dev) */
#[inline]
pub unsafe fn show_dev(dev: dev_t) -> (u32, u32) {
    (major(dev), minor(dev))
}

#[repr(C)]
pub struct ZonefsZoneMgmtEntry {
    pub dev: dev_t,
    pub ino: u64,
    pub op: req_op,
    pub sector: sector_t,
    pub nr_sectors: sector_t,
}

#[repr(C)]
pub struct ZonefsFileDioAppendEntry {
    pub dev: dev_t,
    pub ino: u64,
    pub sector: sector_t,
    pub size: ssize_t,
    pub wpoffset: loff_t,
    pub ret: ssize_t,
}

#[repr(C)]
pub struct ZonefsIomapBeginEntry {
    pub dev: dev_t,
    pub ino: u64,
    pub addr: u64,
    pub offset: loff_t,
    pub length: u64,
}

/*
 * TRACE_EVENT(zonefs_zone_mgmt,
 *     TP_PROTO(struct super_block *sb, struct zonefs_zone *z,
 *              enum req_op op),
 *     TP_ARGS(sb, z, op),
 *     TP_fast_assign(
 *         __entry->dev = sb->s_dev;
 *         __entry->ino = z->z_sector >> ZONEFS_SB(sb)->s_zone_sectors_shift;
 *         __entry->op = op;
 *         __entry->sector = z->z_sector;
 *         __entry->nr_sectors = z->z_size >> SECTOR_SHIFT;
 *     ),
 *     TP_printk("bdev=(%d,%d), ino=%llu op=%s, sector=%llu, nr_sectors=%llu",
 *               show_dev(__entry->dev), __entry->ino,
 *               blk_op_str(__entry->op), __entry->sector,
 *               __entry->nr_sectors)
 * )
 */

/*
 * TRACE_EVENT(zonefs_file_dio_append,
 *     TP_PROTO(struct inode *inode, ssize_t size, ssize_t ret),
 *     TP_ARGS(inode, size, ret),
 *     TP_fast_assign(
 *         __entry->dev = inode->i_sb->s_dev;
 *         __entry->ino = inode->i_ino;
 *         __entry->sector = zonefs_inode_zone(inode)->z_sector;
 *         __entry->size = size;
 *         __entry->wpoffset = zonefs_inode_zone(inode)->z_wpoffset;
 *         __entry->ret = ret;
 *     ),
 *     TP_printk("bdev=(%d, %d), ino=%llu, sector=%llu, size=%zu, wpoffset=%llu, ret=%zu",
 *               show_dev(__entry->dev), __entry->ino, __entry->sector,
 *               __entry->size, __entry->wpoffset, __entry->ret)
 * )
 */

/*
 * TRACE_EVENT(zonefs_iomap_begin,
 *     TP_PROTO(struct inode *inode, struct iomap *iomap),
 *     TP_ARGS(inode, iomap),
 *     TP_fast_assign(
 *         __entry->dev = inode->i_sb->s_dev;
 *         __entry->ino = inode->i_ino;
 *         __entry->addr = iomap->addr;
 *         __entry->offset = iomap->offset;
 *         __entry->length = iomap->length;
 *     ),
 *     TP_printk("bdev=(%d,%d), ino=%llu, addr=%llu, offset=%llu, length=%llu",
 *               show_dev(__entry->dev), __entry->ino, __entry->addr,
 *               __entry->offset, __entry->length)
 * )
 */

/* TRACE_INCLUDE_PATH ., TRACE_INCLUDE_FILE trace, and define_trace are build-time directives. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
