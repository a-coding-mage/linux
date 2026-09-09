/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2012 Alexander Block.  All rights reserved.
 * Copyright (C) 2012 STRATO.  All rights reserved.
 */

// Dependencies: SZ_64K, SZ_16K, BTRFS_MAX_COMPRESSED, PAGE_SIZE, and ALIGN
// are supplied by the surrounding kernel translation.

pub struct btrfs_root;
pub struct btrfs_ioctl_send_args;

pub const BTRFS_SEND_STREAM_MAGIC: &[u8] = b"btrfs-stream\0";
/* Conditional support for the upcoming protocol version. */
#[cfg(feature = "CONFIG_BTRFS_EXPERIMENTAL")]
pub const BTRFS_SEND_STREAM_VERSION: u32 = 3;
#[cfg(not(feature = "CONFIG_BTRFS_EXPERIMENTAL"))]
pub const BTRFS_SEND_STREAM_VERSION: u32 = 2;

/*
 * In send stream v1, no command is larger than 64K. In send stream v2, no
 * limit should be assumed, the buffer size is set to be a header with
 * compressed extent size.
 */
pub const BTRFS_SEND_BUF_SIZE_V1: usize = SZ_64K;
pub const BTRFS_SEND_BUF_SIZE_V2: usize = ALIGN(SZ_16K + BTRFS_MAX_COMPRESSED, PAGE_SIZE);

#[repr(i32)]
pub enum btrfs_tlv_type {
    BTRFS_TLV_U8,
    BTRFS_TLV_U16,
    BTRFS_TLV_U32,
    BTRFS_TLV_U64,
    BTRFS_TLV_BINARY,
    BTRFS_TLV_STRING,
    BTRFS_TLV_UUID,
    BTRFS_TLV_TIMESPEC,
}

#[repr(C, packed)]
pub struct btrfs_stream_header {
    pub magic: [u8; 13],
    pub version: u32,
}

#[repr(C, packed)]
pub struct btrfs_cmd_header {
    /* len excluding the header */
    pub len: u32,
    pub cmd: u16,
    /* crc including the header with zero crc field */
    pub crc: u32,
}

#[repr(C, packed)]
pub struct btrfs_tlv_header {
    pub tlv_type: u16,
    /* len excluding the header */
    pub tlv_len: u16,
}

/* commands */
#[repr(i32)]
pub enum btrfs_send_cmd {
    BTRFS_SEND_C_UNSPEC = 0,

    /* Version 1 */
    BTRFS_SEND_C_SUBVOL = 1,
    BTRFS_SEND_C_SNAPSHOT = 2,
    BTRFS_SEND_C_MKFILE = 3,
    BTRFS_SEND_C_MKDIR = 4,
    BTRFS_SEND_C_MKNOD = 5,
    BTRFS_SEND_C_MKFIFO = 6,
    BTRFS_SEND_C_MKSOCK = 7,
    BTRFS_SEND_C_SYMLINK = 8,
    BTRFS_SEND_C_RENAME = 9,
    BTRFS_SEND_C_LINK = 10,
    BTRFS_SEND_C_UNLINK = 11,
    BTRFS_SEND_C_RMDIR = 12,
    BTRFS_SEND_C_SET_XATTR = 13,
    BTRFS_SEND_C_REMOVE_XATTR = 14,
    BTRFS_SEND_C_WRITE = 15,
    BTRFS_SEND_C_CLONE = 16,
    BTRFS_SEND_C_TRUNCATE = 17,
    BTRFS_SEND_C_CHMOD = 18,
    BTRFS_SEND_C_CHOWN = 19,
    BTRFS_SEND_C_UTIMES = 20,
    BTRFS_SEND_C_END = 21,
    BTRFS_SEND_C_UPDATE_EXTENT = 22,
    BTRFS_SEND_C_MAX_V1 = 22,

    /* Version 2 */
    BTRFS_SEND_C_FALLOCATE = 23,
    BTRFS_SEND_C_FILEATTR = 24,
    BTRFS_SEND_C_ENCODED_WRITE = 25,
    BTRFS_SEND_C_MAX_V2 = 25,

    /* Version 3 */
    BTRFS_SEND_C_ENABLE_VERITY = 26,
    BTRFS_SEND_C_MAX_V3 = 26,
    /* End */
    BTRFS_SEND_C_MAX = 26,
}

/* attributes in send stream */
pub const BTRFS_SEND_A_UNSPEC: i32 = 0;
/* Version 1 */
pub const BTRFS_SEND_A_UUID: i32 = 1;
pub const BTRFS_SEND_A_CTRANSID: i32 = 2;
pub const BTRFS_SEND_A_INO: i32 = 3;
pub const BTRFS_SEND_A_SIZE: i32 = 4;
pub const BTRFS_SEND_A_MODE: i32 = 5;
pub const BTRFS_SEND_A_UID: i32 = 6;
pub const BTRFS_SEND_A_GID: i32 = 7;
pub const BTRFS_SEND_A_RDEV: i32 = 8;
pub const BTRFS_SEND_A_CTIME: i32 = 9;
pub const BTRFS_SEND_A_MTIME: i32 = 10;
pub const BTRFS_SEND_A_ATIME: i32 = 11;
pub const BTRFS_SEND_A_OTIME: i32 = 12;
pub const BTRFS_SEND_A_XATTR_NAME: i32 = 13;
pub const BTRFS_SEND_A_XATTR_DATA: i32 = 14;
pub const BTRFS_SEND_A_PATH: i32 = 15;
pub const BTRFS_SEND_A_PATH_TO: i32 = 16;
pub const BTRFS_SEND_A_PATH_LINK: i32 = 17;
pub const BTRFS_SEND_A_FILE_OFFSET: i32 = 18;
/* As of send stream v2, DATA must be the last attribute in a command. */
pub const BTRFS_SEND_A_DATA: i32 = 19;
pub const BTRFS_SEND_A_CLONE_UUID: i32 = 20;
pub const BTRFS_SEND_A_CLONE_CTRANSID: i32 = 21;
pub const BTRFS_SEND_A_CLONE_PATH: i32 = 22;
pub const BTRFS_SEND_A_CLONE_OFFSET: i32 = 23;
pub const BTRFS_SEND_A_CLONE_LEN: i32 = 24;
pub const BTRFS_SEND_A_MAX_V1: i32 = 24;
/* Version 2 */
pub const BTRFS_SEND_A_FALLOCATE_MODE: i32 = 25;
/* File attributes translated to BTRFS inode flag bits. */
pub const BTRFS_SEND_A_FILEATTR: i32 = 26;
pub const BTRFS_SEND_A_UNENCODED_FILE_LEN: i32 = 27;
pub const BTRFS_SEND_A_UNENCODED_LEN: i32 = 28;
pub const BTRFS_SEND_A_UNENCODED_OFFSET: i32 = 29;
/* COMPRESSION and ENCRYPTION default to NONE (0) if omitted. */
pub const BTRFS_SEND_A_COMPRESSION: i32 = 30;
pub const BTRFS_SEND_A_ENCRYPTION: i32 = 31;
pub const BTRFS_SEND_A_MAX_V2: i32 = 31;
/* Version 3 */
pub const BTRFS_SEND_A_VERITY_ALGORITHM: i32 = 32;
pub const BTRFS_SEND_A_VERITY_BLOCK_SIZE: i32 = 33;
pub const BTRFS_SEND_A_VERITY_SALT_DATA: i32 = 34;
pub const BTRFS_SEND_A_VERITY_SIG_DATA: i32 = 35;
pub const BTRFS_SEND_A_MAX_V3: i32 = 35;
pub const __BTRFS_SEND_A_MAX: i32 = 35;

extern "C" {
    pub fn btrfs_ioctl_send(
        send_root: *mut btrfs_root,
        arg: *const btrfs_ioctl_send_args,
    ) -> libc::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
