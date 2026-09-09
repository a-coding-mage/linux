// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2009, Christoph Hellwig
 * All Rights Reserved.
 */

// Dependencies supplied by the surrounding XFS implementation:
// xfs_platform.h, xfs_fs.h, xfs_shared.h, xfs_bit.h, xfs_format.h,
// xfs_log_format.h, xfs_trans_resv.h, xfs_mount.h, xfs_group.h,
// xfs_defer.h, xfs_da_format.h, xfs_inode.h, xfs_btree.h, xfs_da_btree.h,
// xfs_alloc.h, xfs_bmap.h, xfs_attr.h, xfs_trans.h, xfs_log.h,
// xfs_log_priv.h, xfs_trans_priv.h, xfs_buf_item.h, xfs_quota.h,
// xfs_dquot_item.h, xfs_dquot.h, xfs_log_recover.h, xfs_filestream.h,
// xfs_fsmap.h, xfs_btree_staging.h, xfs_icache.h, xfs_iunlink_item.h,
// xfs_ag.h, xfs_ag_resv.h, xfs_error.h, linux/iomap.h, xfs_iomap.h,
// xfs_buf_mem.h, xfs_btree_mem.h, xfs_exchmaps.h, xfs_exchrange.h,
// xfs_parent.h, xfs_rmap.h, xfs_refcount.h, xfs_metafile.h, xfs_metadir.h,
// xfs_rtgroup.h, xfs_zone_alloc.h, xfs_zone_priv.h, xfs_health.h,
// xfs_healthmon.h, xfs_notify_failure.h, xfs_file.h, and linux/fserror.h.

// The C source defines CREATE_TRACE_POINTS before including xfs_trace.h so
// that the trace event implementations are emitted.  The corresponding
// trace declarations and implementations are supplied by that dependency.
// CREATE_TRACE_POINTS


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
