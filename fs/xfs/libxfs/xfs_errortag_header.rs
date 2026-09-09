/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (c) 2000-2002,2005 Silicon Graphics, Inc.
 * Copyright (C) 2017 Oracle.
 * All Rights Reserved.
 */

/*
 * Error injection tags.  The numbers are consecutive because arrays are
 * sized based on the maximum.
 */
pub const XFS_ERRTAG_NOERROR: i32 = 0;
pub const XFS_ERRTAG_IFLUSH_1: i32 = 1;
pub const XFS_ERRTAG_IFLUSH_2: i32 = 2;
pub const XFS_ERRTAG_IFLUSH_3: i32 = 3;
pub const XFS_ERRTAG_IFLUSH_4: i32 = 4;
pub const XFS_ERRTAG_IFLUSH_5: i32 = 5;
pub const XFS_ERRTAG_IFLUSH_6: i32 = 6;
pub const XFS_ERRTAG_DA_READ_BUF: i32 = 7;
pub const XFS_ERRTAG_BTREE_CHECK_LBLOCK: i32 = 8;
pub const XFS_ERRTAG_BTREE_CHECK_SBLOCK: i32 = 9;
pub const XFS_ERRTAG_ALLOC_READ_AGF: i32 = 10;
pub const XFS_ERRTAG_IALLOC_READ_AGI: i32 = 11;
pub const XFS_ERRTAG_ITOBP_INOTOBP: i32 = 12;
pub const XFS_ERRTAG_IUNLINK: i32 = 13;
pub const XFS_ERRTAG_IUNLINK_REMOVE: i32 = 14;
pub const XFS_ERRTAG_DIR_INO_VALIDATE: i32 = 15;
pub const XFS_ERRTAG_BULKSTAT_READ_CHUNK: i32 = 16;
pub const XFS_ERRTAG_IODONE_IOERR: i32 = 17;
pub const XFS_ERRTAG_STRATREAD_IOERR: i32 = 18;
pub const XFS_ERRTAG_STRATCMPL_IOERR: i32 = 19;
pub const XFS_ERRTAG_DIOWRITE_IOERR: i32 = 20;
pub const XFS_ERRTAG_BMAPIFORMAT: i32 = 21;
pub const XFS_ERRTAG_FREE_EXTENT: i32 = 22;
pub const XFS_ERRTAG_RMAP_FINISH_ONE: i32 = 23;
pub const XFS_ERRTAG_REFCOUNT_CONTINUE_UPDATE: i32 = 24;
pub const XFS_ERRTAG_REFCOUNT_FINISH_ONE: i32 = 25;
pub const XFS_ERRTAG_BMAP_FINISH_ONE: i32 = 26;
pub const XFS_ERRTAG_AG_RESV_CRITICAL: i32 = 27;
/* Retained so xfs_errortag_add() can reject this removed error injection tag. */
pub const XFS_ERRTAG_DROP_WRITES: i32 = 28;
pub const XFS_ERRTAG_LOG_BAD_CRC: i32 = 29;
pub const XFS_ERRTAG_LOG_ITEM_PIN: i32 = 30;
pub const XFS_ERRTAG_BUF_LRU_REF: i32 = 31;
pub const XFS_ERRTAG_FORCE_SCRUB_REPAIR: i32 = 32;
pub const XFS_ERRTAG_FORCE_SUMMARY_RECALC: i32 = 33;
pub const XFS_ERRTAG_IUNLINK_FALLBACK: i32 = 34;
pub const XFS_ERRTAG_BUF_IOERROR: i32 = 35;
pub const XFS_ERRTAG_REDUCE_MAX_IEXTENTS: i32 = 36;
pub const XFS_ERRTAG_BMAP_ALLOC_MINLEN_EXTENT: i32 = 37;
pub const XFS_ERRTAG_AG_RESV_FAIL: i32 = 38;
pub const XFS_ERRTAG_LARP: i32 = 39;
pub const XFS_ERRTAG_DA_LEAF_SPLIT: i32 = 40;
pub const XFS_ERRTAG_ATTR_LEAF_TO_NODE: i32 = 41;
pub const XFS_ERRTAG_WB_DELAY_MS: i32 = 42;
pub const XFS_ERRTAG_WRITE_DELAY_MS: i32 = 43;
pub const XFS_ERRTAG_EXCHMAPS_FINISH_ONE: i32 = 44;
pub const XFS_ERRTAG_METAFILE_RESV_CRITICAL: i32 = 45;
pub const XFS_ERRTAG_FORCE_ZERO_RANGE: i32 = 46;
pub const XFS_ERRTAG_ZONE_RESET: i32 = 47;
pub const XFS_ERRTAG_MAX: i32 = 48;

/* Random factors for the above tags: 1 means always, 2 means half the time. */
pub const XFS_RANDOM_DEFAULT: i32 = 100;

/*
 * C XFS_ERRTAGS expansion table.  Invoke with a macro accepting
 * (tag_without_prefix, sysfs_name, default_value).
 */
#[macro_export]
macro_rules! XFS_ERRTAGS {
    ($m:ident) => {
        $m!(NOERROR, noerror, XFS_RANDOM_DEFAULT);
        $m!(IFLUSH_1, iflush1, XFS_RANDOM_DEFAULT);
        $m!(IFLUSH_2, iflush2, XFS_RANDOM_DEFAULT);
        $m!(IFLUSH_3, iflush3, XFS_RANDOM_DEFAULT);
        $m!(IFLUSH_4, iflush4, XFS_RANDOM_DEFAULT);
        $m!(IFLUSH_5, iflush5, XFS_RANDOM_DEFAULT);
        $m!(IFLUSH_6, iflush6, XFS_RANDOM_DEFAULT);
        $m!(DA_READ_BUF, dareadbuf, XFS_RANDOM_DEFAULT);
        $m!(BTREE_CHECK_LBLOCK, btree_chk_lblk, XFS_RANDOM_DEFAULT / 4);
        $m!(BTREE_CHECK_SBLOCK, btree_chk_sblk, XFS_RANDOM_DEFAULT);
        $m!(ALLOC_READ_AGF, readagf, XFS_RANDOM_DEFAULT);
        $m!(IALLOC_READ_AGI, readagi, XFS_RANDOM_DEFAULT);
        $m!(ITOBP_INOTOBP, itobp, XFS_RANDOM_DEFAULT);
        $m!(IUNLINK, iunlink, XFS_RANDOM_DEFAULT);
        $m!(IUNLINK_REMOVE, iunlinkrm, XFS_RANDOM_DEFAULT);
        $m!(DIR_INO_VALIDATE, dirinovalid, XFS_RANDOM_DEFAULT);
        $m!(BULKSTAT_READ_CHUNK, bulkstat, XFS_RANDOM_DEFAULT);
        $m!(IODONE_IOERR, logiodone, XFS_RANDOM_DEFAULT / 10);
        $m!(STRATREAD_IOERR, stratread, XFS_RANDOM_DEFAULT / 10);
        $m!(STRATCMPL_IOERR, stratcmpl, XFS_RANDOM_DEFAULT / 10);
        $m!(DIOWRITE_IOERR, diowrite, XFS_RANDOM_DEFAULT / 10);
        $m!(BMAPIFORMAT, bmapifmt, XFS_RANDOM_DEFAULT);
        $m!(FREE_EXTENT, free_extent, 1);
        $m!(RMAP_FINISH_ONE, rmap_finish_one, 1);
        $m!(REFCOUNT_CONTINUE_UPDATE, refcount_continue_update, 1);
        $m!(REFCOUNT_FINISH_ONE, refcount_finish_one, 1);
        $m!(BMAP_FINISH_ONE, bmap_finish_one, 1);
        $m!(AG_RESV_CRITICAL, ag_resv_critical, 4);
        $m!(LOG_BAD_CRC, log_bad_crc, 1);
        $m!(LOG_ITEM_PIN, log_item_pin, 1);
        $m!(BUF_LRU_REF, buf_lru_ref, 2);
        $m!(FORCE_SCRUB_REPAIR, force_repair, 1);
        $m!(FORCE_SUMMARY_RECALC, bad_summary, 1);
        $m!(IUNLINK_FALLBACK, iunlink_fallback, XFS_RANDOM_DEFAULT / 10);
        $m!(BUF_IOERROR, buf_ioerror, XFS_RANDOM_DEFAULT);
        $m!(REDUCE_MAX_IEXTENTS, reduce_max_iextents, 1);
        $m!(BMAP_ALLOC_MINLEN_EXTENT, bmap_alloc_minlen_extent, 1);
        $m!(AG_RESV_FAIL, ag_resv_fail, 1);
        $m!(LARP, larp, 1);
        $m!(DA_LEAF_SPLIT, da_leaf_split, 1);
        $m!(ATTR_LEAF_TO_NODE, attr_leaf_to_node, 1);
        $m!(WB_DELAY_MS, wb_delay_ms, 3000);
        $m!(WRITE_DELAY_MS, write_delay_ms, 3000);
        $m!(EXCHMAPS_FINISH_ONE, exchmaps_finish_one, 1);
        $m!(METAFILE_RESV_CRITICAL, metafile_resv_crit, 4);
        $m!(FORCE_ZERO_RANGE, force_zero_range, 4);
        $m!(ZONE_RESET, zone_reset, 1);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
