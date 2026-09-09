// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2008-2010, Dave Chinner
 * All Rights Reserved.
 */

/* in memory log item structure */
#[repr(C)]
pub struct xfs_icreate_item {
    pub ic_item: xfs_log_item,
    pub ic_format: xfs_icreate_log,
}

extern "C" {
    pub static mut xfs_icreate_cache: *mut kmem_cache; /* inode create item */

    pub fn xfs_icreate_log(
        tp: *mut xfs_trans,
        agno: xfs_agnumber_t,
        agbno: xfs_agblock_t,
        count: u32,
        inode_size: u32,
        length: xfs_agblock_t,
        generation: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
