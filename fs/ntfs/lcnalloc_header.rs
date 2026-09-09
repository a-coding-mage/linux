/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Exports for NTFS kernel cluster (de)allocation.
 *
 * Copyright (c) 2004-2005 Anton Altaparmakov
 */

/* C header dependencies: <linux/sched/mm.h> and "attrib.h". */

/*
 * enum zone_type - Zone identifiers for cluster allocation policy
 *
 * FIRST_ZONE     For sanity checking.
 * MFT_ZONE       Allocate from $MFT zone.
 * DATA_ZONE      Allocate from $DATA zone.
 * LAST_ZONE      For sanity checking.
 */
pub const FIRST_ZONE: i32 = 0;
pub const MFT_ZONE: i32 = 0;
pub const DATA_ZONE: i32 = 1;
pub const LAST_ZONE: i32 = 1;

extern "C" {
    pub fn ntfs_cluster_alloc(
        vol: *mut ntfs_volume,
        start_vcn: i64,
        count: i64,
        start_lcn: i64,
        zone: i32,
        is_extension: bool,
        is_contig: bool,
        is_dealloc: bool,
    ) -> *mut runlist_element;

    pub fn __ntfs_cluster_free(
        ni: *mut ntfs_inode,
        start_vcn: i64,
        count: i64,
        ctx: *mut ntfs_attr_search_ctx,
        is_rollback: bool,
    ) -> i64;

    pub fn ntfs_cluster_free_from_rl_nolock(
        vol: *mut ntfs_volume,
        rl: *const runlist_element,
    ) -> i32;

    pub fn memalloc_nofs_save() -> u32;
    pub fn memalloc_nofs_restore(flags: u32);
    pub fn down_write(lock: *mut core::ffi::c_void);
    pub fn up_write(lock: *mut core::ffi::c_void);
}

/* The following types are supplied by the translated attrib.h dependency. */
#[allow(non_camel_case_types)]
pub enum ntfs_volume {}
#[allow(non_camel_case_types)]
pub enum ntfs_inode {}
#[allow(non_camel_case_types)]
pub enum ntfs_attr_search_ctx {}
#[allow(non_camel_case_types)]
pub enum runlist_element {}

/*
 * ntfs_cluster_free - free clusters on an ntfs volume
 * See the C declaration's documentation for locking, mapping, and error
 * semantics.  This wrapper preserves the rollback argument as false.
 */
#[inline]
pub unsafe fn ntfs_cluster_free(
    ni: *mut ntfs_inode,
    start_vcn: i64,
    count: i64,
    ctx: *mut ntfs_attr_search_ctx,
) -> i64 {
    __ntfs_cluster_free(ni, start_vcn, count, ctx, false)
}

/*
 * ntfs_cluster_free_from_rl - free clusters from runlist
 * The caller must hold the runlist lock; this wrapper takes and releases the
 * volume lcn bitmap write lock and preserves the no-filesystem-allocation
 * memory context around the operation.
 */
#[inline]
pub unsafe fn ntfs_cluster_free_from_rl(
    vol: *mut ntfs_volume,
    rl: *const runlist_element,
) -> i32 {
    let memalloc_flags: u32 = memalloc_nofs_save();
    /* Field access corresponds to vol->lcnbmp_lock in the C source. */
    down_write(core::ptr::addr_of_mut!((*vol).lcnbmp_lock) as *mut core::ffi::c_void);
    let ret: i32 = ntfs_cluster_free_from_rl_nolock(vol, rl);
    up_write(core::ptr::addr_of_mut!((*vol).lcnbmp_lock) as *mut core::ffi::c_void);
    memalloc_nofs_restore(memalloc_flags);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
