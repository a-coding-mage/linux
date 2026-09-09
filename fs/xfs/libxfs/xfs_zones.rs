// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2023-2025 Christoph Hellwig.
 * Copyright (c) 2024-2025, Western Digital Corporation or its affiliates.
 */

// Dependencies supplied by the surrounding XFS translation.

unsafe fn xfs_validate_blk_zone_seq(
    mp: *mut xfs_mount,
    zone: *mut blk_zone,
    zone_no: u32,
    write_pointer: *mut xfs_rgblock_t,
) -> bool {
    match (*zone).cond {
        BLK_ZONE_COND_EMPTY => {
            *write_pointer = 0;
            true
        }
        BLK_ZONE_COND_IMP_OPEN
        | BLK_ZONE_COND_EXP_OPEN
        | BLK_ZONE_COND_CLOSED
        | BLK_ZONE_COND_ACTIVE => {
            if (*zone).wp < (*zone).start
                || (*zone).wp >= (*zone).start + (*zone).capacity
            {
                xfs_warn(
                    mp,
                    "zone %u write pointer (%llu) outside of zone.",
                    zone_no,
                    (*zone).wp,
                );
                return false;
            }

            *write_pointer = XFS_BB_TO_FSB(mp, (*zone).wp - (*zone).start);
            true
        }
        BLK_ZONE_COND_FULL => {
            *write_pointer = XFS_BB_TO_FSB(mp, (*zone).capacity);
            true
        }
        BLK_ZONE_COND_NOT_WP | BLK_ZONE_COND_OFFLINE | BLK_ZONE_COND_READONLY => {
            xfs_warn(
                mp,
                "zone %u has unsupported zone condition 0x%x.",
                zone_no,
                (*zone).cond,
            );
            false
        }
        _ => {
            xfs_warn(
                mp,
                "zone %u has unknown zone condition 0x%x.",
                zone_no,
                (*zone).cond,
            );
            false
        }
    }
}

unsafe fn xfs_validate_blk_zone_conv(
    mp: *mut xfs_mount,
    zone: *mut blk_zone,
    zone_no: u32,
) -> bool {
    match (*zone).cond {
        BLK_ZONE_COND_NOT_WP => true,
        _ => {
            xfs_warn(
                mp,
                "conventional zone %u has unsupported zone condition 0x%x.",
                zone_no,
                (*zone).cond,
            );
            false
        }
    }
}

pub unsafe fn xfs_validate_blk_zone(
    mp: *mut xfs_mount,
    zone: *mut blk_zone,
    zone_no: u32,
    expected_size: u32,
    expected_capacity: u32,
    write_pointer: *mut xfs_rgblock_t,
) -> bool {
    /*
     * Check that the zone capacity matches the rtgroup size stored in the
     * superblock.  Note that all zones including the last one must have a
     * uniform capacity.
     */
    if XFS_BB_TO_FSB(mp, (*zone).capacity) != expected_capacity {
        xfs_warn(
            mp,
            "zone %u capacity (%llu) does not match RT group size (%u).",
            zone_no,
            XFS_BB_TO_FSB(mp, (*zone).capacity),
            expected_capacity,
        );
        return false;
    }

    if XFS_BB_TO_FSB(mp, (*zone).len) != expected_size {
        xfs_warn(
            mp,
            "zone %u length (%llu) does not match geometry (%u).",
            zone_no,
            XFS_BB_TO_FSB(mp, (*zone).len),
            expected_size,
        );
        return false;
    }

    match (*zone).type {
        BLK_ZONE_TYPE_CONVENTIONAL => xfs_validate_blk_zone_conv(mp, zone, zone_no),
        BLK_ZONE_TYPE_SEQWRITE_REQ => {
            xfs_validate_blk_zone_seq(mp, zone, zone_no, write_pointer)
        }
        _ => {
            xfs_warn(
                mp,
                "zoned %u has unsupported type 0x%x.",
                zone_no,
                (*zone).type,
            );
            false
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
