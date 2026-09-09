/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _XFS_ZONE_PRIV_H

#[repr(C)]
pub struct xfs_open_zone {
    /*
     * Entry in the open zone list and refcount.  Protected by
     * zi_open_zones_lock in struct xfs_zone_info.
     */
    pub oz_entry: list_head,
    pub oz_ref: atomic_t,

    /*
     * oz_allocated is the amount of space already allocated out of the zone
     * and is protected by oz_alloc_lock.
     *
     * For conventional zones it also is the offset of the next write.
     */
    pub oz_alloc_lock: spinlock_t,
    pub oz_allocated: xfs_rgblock_t,

    /*
     * oz_written is the number of blocks for which we've received a write
     * completion.  oz_written must always be <= oz_allocated and is
     * protected by the ILOCK of the rmap inode.
     */
    pub oz_written: xfs_rgblock_t,

    /*
     * Write hint (data temperature) assigned to this zone, or
     * WRITE_LIFE_NOT_SET if none was set.
     */
    pub oz_write_hint: rw_hint,

    /* Is this open zone used for garbage collection? */
    pub oz_is_gc: bool,

    /*
     * Pointer to the RT groups structure for this open zone.  Constant over
     * the life time of an open zone.
     */
    pub oz_rtg: *mut xfs_rtgroup,

    pub oz_rcu: rcu_head,
}

/*
 * Number of bitmap buckets to track reclaimable zones.  There are 10 buckets
 * so that each 10% of the usable capacity get their own bucket and GC can
 * only has to walk the bitmaps of the lesser used zones if there are any.
 */
pub const XFS_ZONE_USED_BUCKETS: u32 = 10u32;

#[repr(C)]
pub struct xfs_zone_info {
    /*
     * List of pending space reservations:
     */
    pub zi_reservation_lock: spinlock_t,
    pub zi_reclaim_reservations: list_head,

    /*
     * List and number of open zones:
     */
    pub zi_open_zones_lock: spinlock_t,
    pub zi_open_zones: list_head,
    pub zi_nr_open_zones: u32,
    pub zi_nr_open_gc_zones: u32,

    /*
     * Free zone search cursor and number of free zones:
     */
    pub zi_nr_free_zones: atomic_t,

    /*
     * Wait queue to wait for free zones or open zone resources to become
     * available:
     */
    pub zi_zone_wait: wait_queue_head_t,

    /*
     * Pointer to the GC thread.
     */
    pub zi_gc_thread: *mut task_struct,

    /*
     * List of zones that need a reset:
     */
    pub zi_reset_list_lock: spinlock_t,
    pub zi_reset_list: *mut xfs_group,

    /*
     * A set of bitmaps to bucket-sort reclaimable zones by used blocks to help
     * garbage collection to quickly find the best candidate for reclaim.
     */
    pub zi_used_buckets_lock: spinlock_t,
    pub zi_used_bucket_entries: [u32; XFS_ZONE_USED_BUCKETS as usize],
    pub zi_used_bucket_bitmap: [*mut c_ulong; XFS_ZONE_USED_BUCKETS as usize],
}

extern "C" {
    pub fn xfs_open_zone(
        mp: *mut xfs_mount,
        write_hint: rw_hint,
        is_gc: bool,
    ) -> *mut xfs_open_zone;

    pub fn xfs_zone_gc_reset_sync(rtg: *mut xfs_rtgroup) -> i32;
    pub fn xfs_zoned_need_gc(mp: *mut xfs_mount) -> bool;
    pub fn xfs_zoned_have_reclaimable(zi: *mut xfs_zone_info) -> bool;
    pub fn xfs_zone_gc_mount(mp: *mut xfs_mount) -> i32;
    pub fn xfs_zone_gc_unmount(mp: *mut xfs_mount);

    pub fn xfs_zoned_resv_wake_all(mp: *mut xfs_mount);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
