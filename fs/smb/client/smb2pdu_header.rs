/* SPDX-License-Identifier: LGPL-2.1 */
/*
 *
 *   Copyright (c) International Business Machines  Corp., 2009, 2013
 *                 Etersoft, 2012
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *              Pavel Shilovsky (pshilovsky@samba.org) 2012
 */

// Dependencies supplied by the surrounding translation unit: net/sock.h, cifsacl.h.

/* 52 transform hdr + 64 hdr + 88 create rsp */
pub const SMB2_TRANSFORM_HEADER_SIZE: usize = 52;
pub const MAX_SMB2_HDR_SIZE: usize = 204;

/* The total header size for SMB2 read and write */
pub const SMB2_READWRITE_PDU_HEADER_SIZE: usize = 48 + core::mem::size_of::<smb2_hdr>();

/*
 * Definitions for SMB2 Protocol Data Units (network frames)
 *
 * See MS-SMB2.PDF specification for protocol details.
 * The Naming convention is the lower case version of the SMB2
 * command code name for the struct. Note that structures must be packed.
 */

pub const COMPOUND_FID: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const SYMLINK_ERROR_TAG: u32 = 0x4c4d5953;

#[repr(C, packed)]
pub struct smb2_symlink_err_rsp {
    pub SymLinkLength: u32,
    pub SymLinkErrorTag: u32,
    pub ReparseTag: u32,
    pub ReparseDataLength: u16,
    pub UnparsedPathLength: u16,
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub PrintNameOffset: u16,
    pub PrintNameLength: u16,
    pub Flags: u32,
    pub PathBuffer: [u8; 0],
}

#[repr(C, packed)]
pub struct smb2_error_context_rsp {
    pub ErrorDataLength: u32,
    pub ErrorId: u32,
    pub ErrorContextData: [u8; 0],
}

pub const SMB2_ERROR_ID_DEFAULT: u32 = 0x00000000;
pub const SMB2_ERROR_ID_SHARE_REDIRECT: u32 = 0x72645253;
pub const MOVE_DST_IPADDR_V4: u32 = 0x00000001;
pub const MOVE_DST_IPADDR_V6: u32 = 0x00000002;

#[repr(C, packed)]
pub struct move_dst_ipaddr {
    pub Type: u32,
    pub Reserved: u32,
    pub address: [u8; 16],
}

#[repr(C, packed)]
pub struct share_redirect_error_context_rsp {
    pub StructureSize: u32,
    pub NotificationType: u32,
    pub ResourceNameOffset: u32,
    pub ResourceNameLength: u32,
    pub Reserved: u16,
    pub TargetType: u16,
    pub IPAddrCount: u32,
    pub IpAddrMoveList: [move_dst_ipaddr; 0],
}

pub const SMB2_CREATE_IOV_SIZE: usize = 9;
pub const MAX_SMB2_CREATE_RESPONSE_SIZE: usize = 880;
pub const SMB2_LEASE_READ_CACHING_HE: u8 = 0x01;
pub const SMB2_LEASE_HANDLE_CACHING_HE: u8 = 0x02;
pub const SMB2_LEASE_WRITE_CACHING_HE: u8 = 0x04;

#[repr(C, packed)]
pub struct crt_twarp_ctxt {
    pub ccontext: create_context_hdr,
    pub Name: [u8; 8],
    pub Timestamp: u64,
}

#[repr(C, packed)]
pub struct crt_query_id_ctxt {
    pub ccontext: create_context_hdr,
    pub Name: [u8; 8],
}

#[repr(C, packed)]
pub struct crt_sd_ctxt {
    pub ccontext: create_context_hdr,
    pub Name: [u8; 8],
    pub sd: smb3_sd,
}

#[repr(C, packed)]
pub struct get_retrieval_pointer_count_req {
    pub StartingVcn: u64,
}

#[repr(C, packed)]
pub struct get_retrieval_pointer_count_rsp {
    pub ExtentCount: u32,
}

#[repr(C, packed)]
pub struct smb3_extents {
    pub NextVcn: u64,
    pub Lcn: u64,
}

#[repr(C, packed)]
pub struct get_retrieval_pointers_refcount_rsp {
    pub ExtentCount: u32,
    pub Reserved: u32,
    pub StartingVcn: u64,
    pub extents: [smb3_extents; 0],
}

#[repr(C, packed)]
pub struct fsctl_get_dfs_referral_req {
    pub MaxReferralLevel: u16,
    pub RequestFileName: [u8; 0],
}

#[repr(C, packed)]
pub struct network_resiliency_req {
    pub Timeout: u32,
    pub Reserved: u32,
}

pub const NO_FILE_ID: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const SMB2_IOCTL_IOV_SIZE: usize = 2;

#[repr(C, packed)]
pub struct smb2_file_full_ea_info {
    pub next_entry_offset: u32,
    pub flags: u8,
    pub ea_name_length: u8,
    pub ea_value_length: u16,
    pub ea_data: [core::ffi::c_char; 0],
}

#[repr(C, packed)]
pub struct smb2_file_reparse_point_info {
    pub IndexNumber: u64,
    pub Tag: u32,
}

#[repr(C, packed)]
pub struct smb2_file_id_information {
    pub VolumeSerialNumber: u64,
    pub PersistentFileId: u64,
    pub VolatileFileId: u64,
}

#[repr(C, packed)]
pub struct smb2_file_id_extd_directory_info {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: u64,
    pub LastAccessTime: u64,
    pub LastWriteTime: u64,
    pub ChangeTime: u64,
    pub EndOfFile: u64,
    pub AllocationSize: u64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub UniqueId: u64,
    pub FileName: [core::ffi::c_char; 0],
}

extern "C" {
    pub static mut smb2_padding: [core::ffi::c_char; 7];
}

#[repr(C, packed)]
pub struct create_posix_rsp {
    pub nlink: u32,
    pub reparse_tag: u32,
    pub mode: u32,
    pub owner: smb_sid,
    pub group: smb_sid,
}

pub const SMB2_QUERY_DIRECTORY_IOV_SIZE: usize = 2;

#[repr(C, packed)]
pub struct smb2_posix_info {
    pub NextEntryOffset: u32,
    pub Ignored: u32,
    pub CreationTime: u64,
    pub LastAccessTime: u64,
    pub LastWriteTime: u64,
    pub ChangeTime: u64,
    pub EndOfFile: u64,
    pub AllocationSize: u64,
    pub DosAttributes: u32,
    pub Inode: u64,
    pub DeviceId: u32,
    pub Zero: u32,
    pub HardLinks: u32,
    pub ReparseTag: u32,
    pub Mode: u32,
}

pub struct smb2_posix_info_parsed {
    pub base: *const smb2_posix_info,
    pub size: usize,
    pub owner: smb_sid,
    pub group: smb_sid,
    pub name_len: i32,
    pub name: *const u8,
}

#[repr(C, packed)]
pub struct smb2_create_ea_ctx {
    pub ctx: create_context_hdr,
    pub name: [u8; 8],
    pub ea: smb2_file_full_ea_info,
}

pub const SMB2_WSL_XATTR_UID: &str = "$LXUID";
pub const SMB2_WSL_XATTR_GID: &str = "$LXGID";
pub const SMB2_WSL_XATTR_MODE: &str = "$LXMOD";
pub const SMB2_WSL_XATTR_DEV: &str = "$LXDEV";
pub const SMB2_WSL_XATTR_NAME_LEN: usize = 6;
pub const SMB2_WSL_NUM_XATTRS: usize = 4;
pub const SMB2_WSL_XATTR_UID_SIZE: usize = 4;
pub const SMB2_WSL_XATTR_GID_SIZE: usize = 4;
pub const SMB2_WSL_XATTR_MODE_SIZE: usize = 4;
pub const SMB2_WSL_XATTR_DEV_SIZE: usize = 8;

// ALIGN(..., 4) is retained explicitly for the WSL EA response sizing macros.
pub const SMB2_WSL_MIN_QUERY_EA_RESP_SIZE: usize =
    (((SMB2_WSL_NUM_XATTRS - 1) *
        (SMB2_WSL_XATTR_NAME_LEN + 1 + core::mem::size_of::<smb2_file_full_ea_info>())
        + 3) & !3)
        + SMB2_WSL_XATTR_NAME_LEN + 1 + core::mem::size_of::<smb2_file_full_ea_info>();

pub const SMB2_WSL_MAX_QUERY_EA_RESP_SIZE: usize =
    ((SMB2_WSL_MIN_QUERY_EA_RESP_SIZE + SMB2_WSL_XATTR_UID_SIZE +
        SMB2_WSL_XATTR_GID_SIZE + SMB2_WSL_XATTR_MODE_SIZE + SMB2_WSL_XATTR_DEV_SIZE + 3)
        & !3);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
