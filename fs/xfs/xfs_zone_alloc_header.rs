/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies and build-time configuration are supplied by the
// surrounding translation unit.

#[repr(C)]
pub struct xfs_zone_alloc_ctx {
    pub open_zone: *mut xfs_open_zone,
    pub reserved_blocks: xfs_filblks_t,
}

/*
 * Grab any available space, even if it is less than what the caller asked for.
 */
pub const XFS_ZR_GREEDY: ::core::ffi::c_uint = 1u32 << 0;
/*
 * Only grab instantly available space, don't wait or GC.
 */
pub const XFS_ZR_NOWAIT: ::core::ffi::c_uint = 1u32 << 1;
/*
 * Dip into the reserved pool.
 */
pub const XFS_ZR_RESERVED: ::core::ffi::c_uint = 1u32 << 2;

extern "C" {
    pub fn xfs_zoned_space_reserve(
        mp: *mut xfs_mount,
        count_fsb: xfs_filblks_t,
        flags: ::core::ffi::c_uint,
        ac: *mut xfs_zone_alloc_ctx,
    ) -> ::core::ffi::c_int;
    pub fn xfs_zoned_space_unreserve(mp: *mut xfs_mount, ac: *mut xfs_zone_alloc_ctx);
    pub fn xfs_zoned_add_available(mp: *mut xfs_mount, count_fsb: xfs_filblks_t);

    pub fn xfs_zone_alloc_and_submit(ioend: *mut iomap_ioend, oz: *mut *mut xfs_open_zone);
    pub fn xfs_zone_free_blocks(
        tp: *mut xfs_trans,
        rtg: *mut xfs_rtgroup,
        fsbno: xfs_fsblock_t,
        len: xfs_filblks_t,
    ) -> ::core::ffi::c_int;
    pub fn xfs_zoned_end_io(
        ip: *mut xfs_inode,
        offset: xfs_off_t,
        count: xfs_off_t,
        daddr: xfs_daddr_t,
        oz: *mut xfs_open_zone,
        old_startblock: xfs_fsblock_t,
    ) -> ::core::ffi::c_int;
    pub fn xfs_open_zone_put(oz: *mut xfs_open_zone);

    pub fn xfs_zoned_wake_all(mp: *mut xfs_mount);
    pub fn xfs_zone_rgbno_is_valid(rtg: *mut xfs_rtgroup, rgbno: xfs_rgnumber_t) -> bool;
    pub fn xfs_mark_rtg_boundary(ioend: *mut iomap_ioend);

    pub fn xfs_zone_mark_free(rtg: *mut xfs_rtgroup);
    pub fn xfs_zoned_default_resblks(mp: *mut xfs_mount, ctr: xfs_free_counter) -> u64;
    pub fn xfs_zoned_show_stats(m: *mut seq_file, mp: *mut xfs_mount);
}

#[cfg(feature = "CONFIG_XFS_RT")]
extern "C" {
    pub fn xfs_mount_zones(mp: *mut xfs_mount) -> ::core::ffi::c_int;
    pub fn xfs_unmount_zones(mp: *mut xfs_mount);
    pub fn xfs_zone_gc_start(mp: *mut xfs_mount);
    pub fn xfs_zone_gc_stop(mp: *mut xfs_mount);
    pub fn xfs_zone_gc_wakeup(mp: *mut xfs_mount);
}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
pub unsafe fn xfs_mount_zones(_mp: *mut xfs_mount) -> ::core::ffi::c_int {
    -EIO
}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
pub unsafe fn xfs_unmount_zones(_mp: *mut xfs_mount) {}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
pub unsafe fn xfs_zone_gc_start(_mp: *mut xfs_mount) {}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
pub unsafe fn xfs_zone_gc_stop(_mp: *mut xfs_mount) {}

#[cfg(not(feature = "CONFIG_XFS_RT"))]
pub unsafe fn xfs_zone_gc_wakeup(_mp: *mut xfs_mount) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
