/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Note this file is present in e2fsprogs/lib/ext2fs/fast_commit.h and
 * linux/fs/ext4/fast_commit.h. These file should always be byte identical.
 */

/* Fast commit tags */
pub const EXT4_FC_TAG_ADD_RANGE: __u16 = 0x0001;
pub const EXT4_FC_TAG_DEL_RANGE: __u16 = 0x0002;
pub const EXT4_FC_TAG_CREAT: __u16 = 0x0003;
pub const EXT4_FC_TAG_LINK: __u16 = 0x0004;
pub const EXT4_FC_TAG_UNLINK: __u16 = 0x0005;
pub const EXT4_FC_TAG_INODE: __u16 = 0x0006;
pub const EXT4_FC_TAG_PAD: __u16 = 0x0007;
pub const EXT4_FC_TAG_TAIL: __u16 = 0x0008;
pub const EXT4_FC_TAG_HEAD: __u16 = 0x0009;

pub const EXT4_FC_SUPPORTED_FEATURES: u32 = 0x0;

/* On disk fast commit tlv value structures */

/* Fast commit on disk tag length structure */
#[repr(C)]
pub struct ext4_fc_tl {
    pub fc_tag: __le16,
    pub fc_len: __le16,
}

/* Value structure for tag EXT4_FC_TAG_HEAD. */
#[repr(C)]
pub struct ext4_fc_head {
    pub fc_features: __le32,
    pub fc_tid: __le32,
}

/* Value structure for EXT4_FC_TAG_ADD_RANGE. */
#[repr(C)]
pub struct ext4_fc_add_range {
    pub fc_ino: __le32,
    pub fc_ex: [__u8; 12],
}

/* Value structure for tag EXT4_FC_TAG_DEL_RANGE. */
#[repr(C)]
pub struct ext4_fc_del_range {
    pub fc_ino: __le32,
    pub fc_lblk: __le32,
    pub fc_len: __le32,
}

/*
 * This is the value structure for tags EXT4_FC_TAG_CREAT, EXT4_FC_TAG_LINK
 * and EXT4_FC_TAG_UNLINK.
 */
#[repr(C)]
pub struct ext4_fc_dentry_info {
    pub fc_parent_ino: __le32,
    pub fc_ino: __le32,
    pub fc_dname: [__u8; 0],
}

/* Value structure for EXT4_FC_TAG_INODE. */
#[repr(C)]
pub struct ext4_fc_inode {
    pub fc_ino: __le32,
    pub fc_raw_inode: [__u8; 0],
}

/* Value structure for tag EXT4_FC_TAG_TAIL. */
#[repr(C)]
pub struct ext4_fc_tail {
    pub fc_tid: __le32,
    pub fc_crc: __le32,
}

/* Tag base length */
pub const EXT4_FC_TAG_BASE_LEN: usize = core::mem::size_of::<ext4_fc_tl>();

/* Fast commit status codes */
pub const EXT4_FC_STATUS_OK: i32 = 0;
pub const EXT4_FC_STATUS_INELIGIBLE: i32 = 1;
pub const EXT4_FC_STATUS_SKIPPED: i32 = 2;
pub const EXT4_FC_STATUS_FAILED: i32 = 3;

/* Fast commit ineligiblity reasons: */
pub const EXT4_FC_REASON_XATTR: i32 = 0;
pub const EXT4_FC_REASON_CROSS_RENAME: i32 = 1;
pub const EXT4_FC_REASON_JOURNAL_FLAG_CHANGE: i32 = 2;
pub const EXT4_FC_REASON_NOMEM: i32 = 3;
pub const EXT4_FC_REASON_SWAP_BOOT: i32 = 4;
pub const EXT4_FC_REASON_RESIZE: i32 = 5;
pub const EXT4_FC_REASON_RENAME_DIR: i32 = 6;
pub const EXT4_FC_REASON_FALLOC_RANGE: i32 = 7;
pub const EXT4_FC_REASON_INODE_JOURNAL_DATA: i32 = 8;
pub const EXT4_FC_REASON_ENCRYPTED_FILENAME: i32 = 9;
pub const EXT4_FC_REASON_MIGRATE: i32 = 10;
pub const EXT4_FC_REASON_VERITY: i32 = 11;
pub const EXT4_FC_REASON_MOVE_EXT: i32 = 12;
pub const EXT4_FC_REASON_MAX: usize = 13;

/* The following declarations are present only when building in the kernel. */
#[cfg(feature = "__KERNEL__")]
#[repr(C)]
pub struct ext4_fc_dentry_update {
    pub fcd_op: core::ffi::c_int,
    pub fcd_parent: core::ffi::c_int,
    pub fcd_ino: core::ffi::c_int,
    pub fcd_name: name_snapshot,
    pub fcd_list: list_head,
    pub fcd_dilist: list_head,
}

#[cfg(feature = "__KERNEL__")]
#[repr(C)]
pub struct ext4_fc_stats {
    pub fc_ineligible_reason_count: [core::ffi::c_uint; EXT4_FC_REASON_MAX],
    pub fc_num_commits: core::ffi::c_ulong,
    pub fc_ineligible_commits: core::ffi::c_ulong,
    pub fc_failed_commits: core::ffi::c_ulong,
    pub fc_skipped_commits: core::ffi::c_ulong,
    pub fc_numblks: core::ffi::c_ulong,
    pub s_fc_avg_commit_time: u64,
}

#[cfg(feature = "__KERNEL__")]
pub const EXT4_FC_REPLAY_REALLOC_INCREMENT: i32 = 4;

#[cfg(feature = "__KERNEL__")]
#[repr(C)]
pub struct ext4_fc_alloc_region {
    pub lblk: ext4_lblk_t,
    pub pblk: ext4_fsblk_t,
    pub ino: core::ffi::c_int,
    pub len: core::ffi::c_int,
}

#[cfg(feature = "__KERNEL__")]
#[repr(C)]
pub struct ext4_fc_replay_state {
    pub fc_replay_num_tags: core::ffi::c_int,
    pub fc_replay_expected_off: core::ffi::c_int,
    pub fc_current_pass: core::ffi::c_int,
    pub fc_cur_tag: core::ffi::c_int,
    pub fc_crc: core::ffi::c_int,
    pub fc_regions: *mut ext4_fc_alloc_region,
    pub fc_regions_size: core::ffi::c_int,
    pub fc_regions_used: core::ffi::c_int,
    pub fc_regions_valid: core::ffi::c_int,
    pub fc_modified_inodes: *mut core::ffi::c_int,
    pub fc_modified_inodes_used: core::ffi::c_int,
    pub fc_modified_inodes_size: core::ffi::c_int,
}

#[cfg(feature = "__KERNEL__")]
pub unsafe fn region_last(__region: *const ext4_fc_alloc_region) -> ext4_lblk_t {
    (*__region).lblk + (*__region).len - 1
}

pub fn tag2str(tag: __u16) -> &'static str {
    match tag {
        EXT4_FC_TAG_LINK => "ADD_ENTRY",
        EXT4_FC_TAG_UNLINK => "DEL_ENTRY",
        EXT4_FC_TAG_ADD_RANGE => "ADD_RANGE",
        EXT4_FC_TAG_CREAT => "CREAT_DENTRY",
        EXT4_FC_TAG_DEL_RANGE => "DEL_RANGE",
        EXT4_FC_TAG_INODE => "INODE",
        EXT4_FC_TAG_PAD => "PAD",
        EXT4_FC_TAG_TAIL => "TAIL",
        EXT4_FC_TAG_HEAD => "HEAD",
        _ => "ERROR",
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
