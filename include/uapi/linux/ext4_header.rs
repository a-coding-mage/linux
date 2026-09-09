/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the surrounding UAPI translation are intentionally
// referenced here rather than redefined.

/* ext4-specific ioctl commands */
pub const EXT4_IOC_GETVERSION: _ = _IOR(b'f', 3, core::ffi::c_long);
pub const EXT4_IOC_SETVERSION: _ = _IOW(b'f', 4, core::ffi::c_long);
pub const EXT4_IOC_GETVERSION_OLD: _ = FS_IOC_GETVERSION;
pub const EXT4_IOC_SETVERSION_OLD: _ = FS_IOC_SETVERSION;
pub const EXT4_IOC_GETRSVSZ: _ = _IOR(b'f', 5, core::ffi::c_long);
pub const EXT4_IOC_SETRSVSZ: _ = _IOW(b'f', 6, core::ffi::c_long);
pub const EXT4_IOC_GROUP_EXTEND: _ = _IOW(b'f', 7, core::ffi::c_ulong);
pub const EXT4_IOC_GROUP_ADD: _ = _IOW(b'f', 8, ext4_new_group_input);
pub const EXT4_IOC_MIGRATE: _ = _IO(b'f', 9);
// ioctl 10 reserved for an early version of the FIEMAP ioctl.
// ioctl 11 reserved for filesystem-independent FIEMAP ioctl.
pub const EXT4_IOC_ALLOC_DA_BLKS: _ = _IO(b'f', 12);
pub const EXT4_IOC_MOVE_EXT: _ = _IOWR(b'f', 15, move_extent);
pub const EXT4_IOC_RESIZE_FS: _ = _IOW(b'f', 16, u64);
pub const EXT4_IOC_SWAP_BOOT: _ = _IO(b'f', 17);
pub const EXT4_IOC_PRECACHE_EXTENTS: _ = _IO(b'f', 18);
// ioctl codes 19--39 are reserved for fscrypt.
pub const EXT4_IOC_CLEAR_ES_CACHE: _ = _IO(b'f', 40);
pub const EXT4_IOC_GETSTATE: _ = _IOW(b'f', 41, u32);
pub const EXT4_IOC_GET_ES_CACHE: _ = _IOWR(b'f', 42, fiemap);
pub const EXT4_IOC_CHECKPOINT: _ = _IOW(b'f', 43, u32);
pub const EXT4_IOC_GETFSUUID: _ = _IOR(b'f', 44, fsuuid);
pub const EXT4_IOC_SETFSUUID: _ = _IOW(b'f', 44, fsuuid);
pub const EXT4_IOC_GET_TUNE_SB_PARAM: _ = _IOR(b'f', 45, ext4_tune_sb_params);
pub const EXT4_IOC_SET_TUNE_SB_PARAM: _ = _IOW(b'f', 46, ext4_tune_sb_params);
pub const EXT4_IOC_SHUTDOWN: _ = _IOR(b'X', 125, u32);

/* ioctl commands in 32 bit emulation */
pub const EXT4_IOC32_GETVERSION: _ = _IOR(b'f', 3, i32);
pub const EXT4_IOC32_SETVERSION: _ = _IOW(b'f', 4, i32);
pub const EXT4_IOC32_GETRSVSZ: _ = _IOR(b'f', 5, i32);
pub const EXT4_IOC32_SETRSVSZ: _ = _IOW(b'f', 6, i32);
pub const EXT4_IOC32_GROUP_EXTEND: _ = _IOW(b'f', 7, u32);
pub const EXT4_IOC32_GROUP_ADD: _ = _IOW(b'f', 8, compat_ext4_new_group_input);
pub const EXT4_IOC32_GETVERSION_OLD: _ = FS_IOC32_GETVERSION;
pub const EXT4_IOC32_SETVERSION_OLD: _ = FS_IOC32_SETVERSION;

pub const EXT4_STATE_FLAG_EXT_PRECACHED: u32 = 0x00000001;
pub const EXT4_STATE_FLAG_NEW: u32 = 0x00000002;
pub const EXT4_STATE_FLAG_NEWENTRY: u32 = 0x00000004;
pub const EXT4_STATE_FLAG_DA_ALLOC_CLOSE: u32 = 0x00000008;

pub const EXT4_IOC_CHECKPOINT_FLAG_DISCARD: u32 = 0x1;
pub const EXT4_IOC_CHECKPOINT_FLAG_ZEROOUT: u32 = 0x2;
pub const EXT4_IOC_CHECKPOINT_FLAG_DRY_RUN: u32 = 0x4;
pub const EXT4_IOC_CHECKPOINT_FLAG_VALID: u32 = EXT4_IOC_CHECKPOINT_FLAG_DISCARD | EXT4_IOC_CHECKPOINT_FLAG_ZEROOUT | EXT4_IOC_CHECKPOINT_FLAG_DRY_RUN;

#[repr(C)]
pub struct fsuuid {
    pub fsu_len: u32,
    pub fsu_flags: u32,
    pub fsu_uuid: [u8; 0],
}

#[repr(C)]
pub struct move_extent {
    pub reserved: u32,
    pub donor_fd: u32,
    pub orig_start: u64,
    pub donor_start: u64,
    pub len: u64,
    pub moved_len: u64,
}

pub const EXT4_GOING_FLAGS_DEFAULT: u32 = 0x0;
pub const EXT4_GOING_FLAGS_LOGFLUSH: u32 = 0x1;
pub const EXT4_GOING_FLAGS_NOLOGFLUSH: u32 = 0x2;

#[repr(C)]
pub struct ext4_new_group_input {
    pub group: u32,
    pub block_bitmap: u64,
    pub inode_bitmap: u64,
    pub inode_table: u64,
    pub blocks_count: u32,
    pub reserved_blocks: u16,
    pub unused: u16,
}

#[repr(C)]
pub struct ext4_tune_sb_params {
    pub set_flags: u32,
    pub checkinterval: u32,
    pub errors_behavior: u16,
    pub mnt_count: u16,
    pub max_mnt_count: u16,
    pub raid_stride: u16,
    pub last_check_time: u64,
    pub reserved_blocks: u64,
    pub blocks_count: u64,
    pub default_mnt_opts: u32,
    pub reserved_uid: u32,
    pub reserved_gid: u32,
    pub raid_stripe_width: u32,
    pub encoding: u16,
    pub encoding_flags: u16,
    pub def_hash_alg: u8,
    pub pad_1: u8,
    pub pad_2: u16,
    pub feature_compat: u32,
    pub feature_incompat: u32,
    pub feature_ro_compat: u32,
    pub set_feature_compat_mask: u32,
    pub set_feature_incompat_mask: u32,
    pub set_feature_ro_compat_mask: u32,
    pub clear_feature_compat_mask: u32,
    pub clear_feature_incompat_mask: u32,
    pub clear_feature_ro_compat_mask: u32,
    pub mount_opts: [u8; 64],
    pub pad: [u8; 68],
}

pub const EXT4_TUNE_FL_ERRORS_BEHAVIOR: u32 = 0x00000001;
pub const EXT4_TUNE_FL_MNT_COUNT: u32 = 0x00000002;
pub const EXT4_TUNE_FL_MAX_MNT_COUNT: u32 = 0x00000004;
pub const EXT4_TUNE_FL_CHECKINTRVAL: u32 = 0x00000008;
pub const EXT4_TUNE_FL_LAST_CHECK_TIME: u32 = 0x00000010;
pub const EXT4_TUNE_FL_RESERVED_BLOCKS: u32 = 0x00000020;
pub const EXT4_TUNE_FL_RESERVED_UID: u32 = 0x00000040;
pub const EXT4_TUNE_FL_RESERVED_GID: u32 = 0x00000080;
pub const EXT4_TUNE_FL_DEFAULT_MNT_OPTS: u32 = 0x00000100;
pub const EXT4_TUNE_FL_DEF_HASH_ALG: u32 = 0x00000200;
pub const EXT4_TUNE_FL_RAID_STRIDE: u32 = 0x00000400;
pub const EXT4_TUNE_FL_RAID_STRIPE_WIDTH: u32 = 0x00000800;
pub const EXT4_TUNE_FL_MOUNT_OPTS: u32 = 0x00001000;
pub const EXT4_TUNE_FL_FEATURES: u32 = 0x00002000;
pub const EXT4_TUNE_FL_EDIT_FEATURES: u32 = 0x00004000;
pub const EXT4_TUNE_FL_FORCE_FSCK: u32 = 0x00008000;
pub const EXT4_TUNE_FL_ENCODING: u32 = 0x00010000;
pub const EXT4_TUNE_FL_ENCODING_FLAGS: u32 = 0x00020000;

// Returned by EXT4_IOC_GET_ES_CACHE as an additional possible flag. It
// indicates that the entry in extent status cache is for a hole.
pub const EXT4_FIEMAP_EXTENT_HOLE: u32 = 0x08000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
