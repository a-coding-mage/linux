/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Disc Record at disc address 0xc00
 *
 * The Linux __u8, __le16, and __le32 types are supplied by the surrounding
 * UAPI translation environment.
 */
#[repr(C, packed, align(4))]
pub struct adfs_discrecord {
    pub log2secsize: __u8,
    pub secspertrack: __u8,
    pub heads: __u8,
    pub density: __u8,
    pub idlen: __u8,
    pub log2bpmb: __u8,
    pub skew: __u8,
    pub bootoption: __u8,
    pub lowsector: __u8,
    pub nzones: __u8,
    pub zone_spare: __le16,
    pub root: __le32,
    pub disc_size: __le32,
    pub disc_id: __le16,
    pub disc_name: [__u8; 10],
    pub disc_type: __le32,
    pub disc_size_high: __le32,
    /* C bit-fields: log2sharesize occupies bits 0..=3; unused40 bits 4..=7. */
    pub sharesize_flags: __u8,
    /* C bit-fields: big_flag occupies bit 0; unused41 occupies bits 1..=7. */
    pub big_flag: __u8,
    pub nzones_high: __u8,
    pub reserved43: __u8,
    pub format_version: __le32,
    pub root_size: __le32,
    pub unused52: [__u8; 60 - 52],
}

pub const ADFS_DISCRECORD: usize = 0xc00;
pub const ADFS_DR_OFFSET: usize = 0x1c0;
pub const ADFS_DR_SIZE: usize = 60;
pub const ADFS_DR_SIZE_BITS: usize = ADFS_DR_SIZE << 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
