/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations supplied by other translation units.
#[repr(C)]
pub struct xfs_rtgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct blk_zone {
    _private: [u8; 0],
}

/*
 * In order to guarantee forward progress for GC we need to reserve at least
 * two zones: one that will be used for moving data into and one spare zone
 * making sure that we have enough space to relocate a nearly-full zone.
 * To allow for slightly sloppy accounting for when we need to reserve the
 * second zone, we actually reserve three as that is easier than doing fully
 * accurate bookkeeping.
 */
pub const XFS_GC_ZONES: u32 = 3;

/*
 * In addition we need two zones for user writes, one open zone for writing
 * and one to still have available blocks without resetting the open zone
 * when data in the open zone has been freed.
 */
pub const XFS_RESERVED_ZONES: u32 = XFS_GC_ZONES + 1;
pub const XFS_MIN_ZONES: u32 = XFS_RESERVED_ZONES + 1;

/*
 * Always keep one zone out of the general open zone pool to allow for GC to
 * happen while other writers are waiting for free space.
 */
pub const XFS_OPEN_GC_ZONES: u32 = 1;
pub const XFS_MIN_OPEN_ZONES: u32 = XFS_OPEN_GC_ZONES + 1;

/*
 * For zoned devices that do not have a limit on the number of open zones, and
 * for regular devices using the zoned allocator, use the most common SMR disks
 * limit (128) as the default limit on the number of open zones.
 */
pub const XFS_DEFAULT_MAX_OPEN_ZONES: u32 = 128;

extern "C" {
    pub fn xfs_validate_blk_zone(
        mp: *mut xfs_mount,
        zone: *mut blk_zone,
        zone_no: ::core::ffi::c_uint,
        expected_size: u32,
        expected_capacity: u32,
        write_pointer: *mut xfs_rgblock_t,
    ) -> bool;
}

// `xfs_mount` and `xfs_rgblock_t` are supplied by other translation units.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
