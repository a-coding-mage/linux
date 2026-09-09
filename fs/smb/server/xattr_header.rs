/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2021 Samsung Electronics Co., Ltd.
 */

/*
 * These are on-disk structures to store additional metadata into xattr to
 * reproduce windows filesystem semantics. And they are encoded with NDR to
 * compatible with samba's xattr meta format. The compatibility with samba
 * is important because it can lose the information(file attribute,
 * creation time, acls) about the existing files when switching between
 * ksmbd and samba.
 */

/*
 * Dos attribute flags used for what variable is valid.
 */
pub const XATTR_DOSINFO_ATTRIB: u32 = 0x00000001;
pub const XATTR_DOSINFO_EA_SIZE: u32 = 0x00000002;
pub const XATTR_DOSINFO_SIZE: u32 = 0x00000004;
pub const XATTR_DOSINFO_ALLOC_SIZE: u32 = 0x00000008;
pub const XATTR_DOSINFO_CREATE_TIME: u32 = 0x00000010;
pub const XATTR_DOSINFO_CHANGE_TIME: u32 = 0x00000020;
pub const XATTR_DOSINFO_ITIME: u32 = 0x00000040;

/*
 * Dos attribute structure which is compatible with samba's one.
 * Storing it into the xattr named "DOSATTRIB" separately from inode
 * allows ksmbd to faithfully reproduce windows filesystem semantics
 * on top of a POSIX filesystem.
 */
#[repr(C)]
pub struct xattr_dos_attrib {
    pub version: u16, /* version 3 or version 4 */
    pub flags: u32, /* valid flags */
    pub attr: u32, /* Dos attribute */
    pub ea_size: u32, /* EA size */
    pub size: u64,
    pub alloc_size: u64,
    pub create_time: u64, /* File creation time */
    pub change_time: u64, /* File change time */
    pub itime: u64, /* Invented/Initial time */
}

/*
 * Enumeration is used for computing posix acl hash.
 */
pub const SMB_ACL_TAG_INVALID: i32 = 0;
pub const SMB_ACL_USER: i32 = 1;
pub const SMB_ACL_USER_OBJ: i32 = 2;
pub const SMB_ACL_GROUP: i32 = 3;
pub const SMB_ACL_GROUP_OBJ: i32 = 4;
pub const SMB_ACL_OTHER: i32 = 5;
pub const SMB_ACL_MASK: i32 = 6;

pub const SMB_ACL_READ: i32 = 4;
pub const SMB_ACL_WRITE: i32 = 2;
pub const SMB_ACL_EXECUTE: i32 = 1;

#[repr(C)]
pub struct xattr_acl_entry {
    pub type_: i32,
    pub uid: uid_t,
    pub gid: gid_t,
    pub perm: mode_t,
}

/*
 * xattr_smb_acl structure is used for computing posix acl hash.
 */
#[repr(C)]
pub struct xattr_smb_acl {
    pub count: i32,
    pub next: i32,
    pub entries: [xattr_acl_entry; 0],
}

/* 64bytes hash in xattr_ntacl is computed with sha256 */
pub const XATTR_SD_HASH_TYPE_SHA256: u32 = 0x1;
pub const XATTR_SD_HASH_SIZE: usize = 64;

/*
 * xattr_ntacl is used for storing ntacl and hashes.
 * Hash is used for checking valid posix acl and ntacl in xattr.
 */
#[repr(C)]
pub struct xattr_ntacl {
    pub version: u16, /* version 4*/
    pub sd_buf: *mut core::ffi::c_void,
    pub sd_size: u32,
    pub hash_type: u16, /* hash type */
    pub desc: [u8; 10], /* posix_acl description */
    pub desc_len: u16,
    pub current_time: u64,
    pub hash: [u8; XATTR_SD_HASH_SIZE], /* 64bytes hash for ntacl */
    pub posix_acl_hash: [u8; XATTR_SD_HASH_SIZE], /* 64bytes hash for posix acl */
}

/* DOS ATTRIBUTE XATTR PREFIX */
pub const DOS_ATTRIBUTE_PREFIX: &str = "DOSATTRIB";
pub const DOS_ATTRIBUTE_PREFIX_LEN: usize = DOS_ATTRIBUTE_PREFIX.len();
pub const XATTR_NAME_DOS_ATTRIBUTE: &str = concat!(XATTR_USER_PREFIX, DOS_ATTRIBUTE_PREFIX);
pub const XATTR_NAME_DOS_ATTRIBUTE_LEN: usize = XATTR_NAME_DOS_ATTRIBUTE.len();

/* STREAM XATTR PREFIX */
pub const STREAM_PREFIX: &str = "DosStream.";
pub const STREAM_PREFIX_LEN: usize = STREAM_PREFIX.len();
pub const XATTR_NAME_STREAM: &str = concat!(XATTR_USER_PREFIX, STREAM_PREFIX);
pub const XATTR_NAME_STREAM_LEN: usize = XATTR_NAME_STREAM.len();

/* SECURITY DESCRIPTOR(NTACL) XATTR PREFIX */
pub const SD_PREFIX: &str = "NTACL";
pub const SD_PREFIX_LEN: usize = SD_PREFIX.len();
pub const XATTR_NAME_SD: &str = concat!(XATTR_SECURITY_PREFIX, SD_PREFIX);
pub const XATTR_NAME_SD_LEN: usize = XATTR_NAME_SD.len();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
