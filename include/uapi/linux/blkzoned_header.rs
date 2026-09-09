/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Zoned block devices handling.
 *
 * Rust translation of the Linux UAPI blkzoned header.
 */

/* The ioctl encoding helpers are supplied by the corresponding Linux UAPI
 * dependency: _IOWR, _IOW, and _IOR. */

pub const BLK_ZONE_TYPE_CONVENTIONAL: i32 = 0x1;
pub const BLK_ZONE_TYPE_SEQWRITE_REQ: i32 = 0x2;
pub const BLK_ZONE_TYPE_SEQWRITE_PREF: i32 = 0x3;

pub const BLK_ZONE_COND_NOT_WP: i32 = 0x0;
pub const BLK_ZONE_COND_EMPTY: i32 = 0x1;
pub const BLK_ZONE_COND_IMP_OPEN: i32 = 0x2;
pub const BLK_ZONE_COND_EXP_OPEN: i32 = 0x3;
pub const BLK_ZONE_COND_CLOSED: i32 = 0x4;
pub const BLK_ZONE_COND_READONLY: i32 = 0xD;
pub const BLK_ZONE_COND_FULL: i32 = 0xE;
pub const BLK_ZONE_COND_OFFLINE: i32 = 0xF;
pub const BLK_ZONE_COND_ACTIVE: i32 = 0xFF; /* added in Linux 6.19 */

pub const BLK_ZONE_REP_CAPACITY: u32 = 1u32 << 0;
pub const BLK_ZONE_REP_CACHED: u32 = 1u32 << 31; /* added in Linux 6.19 */

#[repr(C)]
pub struct blk_zone {
    pub start: u64,
    pub len: u64,
    pub wp: u64,
    pub type_: u8,
    pub cond: u8,
    pub non_seq: u8,
    pub reset: u8,
    pub resv: [u8; 4],
    pub capacity: u64,
    pub reserved: [u8; 24],
}

#[repr(C)]
pub struct blk_zone_report {
    pub sector: u64,
    pub nr_zones: u32,
    pub flags: u32,
    pub zones: [blk_zone; 0],
}

#[repr(C)]
pub struct blk_zone_range {
    pub sector: u64,
    pub nr_sectors: u64,
}

pub const BLKREPORTZONE: _ = _IOWR(0x12, 130, core::mem::size_of::<blk_zone_report>());
pub const BLKRESETZONE: _ = _IOW(0x12, 131, core::mem::size_of::<blk_zone_range>());
pub const BLKGETZONESZ: _ = _IOR(0x12, 132, core::mem::size_of::<u32>());
pub const BLKGETNRZONES: _ = _IOR(0x12, 133, core::mem::size_of::<u32>());
pub const BLKOPENZONE: _ = _IOW(0x12, 134, core::mem::size_of::<blk_zone_range>());
pub const BLKCLOSEZONE: _ = _IOW(0x12, 135, core::mem::size_of::<blk_zone_range>());
pub const BLKFINISHZONE: _ = _IOW(0x12, 136, core::mem::size_of::<blk_zone_range>());
pub const BLKREPORTZONEV2: _ = _IOWR(0x12, 142, core::mem::size_of::<blk_zone_report>());

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
