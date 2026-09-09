/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) 2018 Samsung Electronics Co., Ltd. */

use core::ffi::c_void;

/* Dependencies supplied by the surrounding kernel/SMB implementation. */
pub type __le16 = u16;
pub type __le32 = u32;
pub type __le64 = u64;
pub type __u8 = u8;
pub type __u32 = u32;
pub type __u64 = u64;

/* ksmbd's Specific ERRNO */
pub const ESHARE: i32 = 50000;

pub const SMB1_PROT: i32 = 0;
pub const SMB2_PROT: i32 = 1;
pub const SMB21_PROT: i32 = 2;
/* multi-protocol negotiate request */
pub const SMB2X_PROT: i32 = 3;
pub const SMB30_PROT: i32 = 4;
pub const SMB302_PROT: i32 = 5;
pub const SMB311_PROT: i32 = 6;
pub const BAD_PROT: u16 = 0xFFFF;

/* HZ is supplied by the kernel. */
pub const SMB_ECHO_INTERVAL: i32 = 60 * HZ;
pub const MAX_STREAM_PROT_LEN: u32 = 0x00FF_FFFF;

/* Responses when opening a file. */
pub const F_SUPERSEDED: i32 = 0;
pub const F_OPENED: i32 = 1;
pub const F_CREATED: i32 = 2;
pub const F_OVERWRITTEN: i32 = 3;

pub const SET_FILE_READ_RIGHTS: u32 = FILE_READ_DATA | FILE_READ_EA | FILE_READ_ATTRIBUTES |
    DELETE | READ_CONTROL | WRITE_DAC | WRITE_OWNER | SYNCHRONIZE;
pub const SET_FILE_WRITE_RIGHTS: u32 = FILE_WRITE_DATA | FILE_APPEND_DATA | FILE_WRITE_EA |
    FILE_DELETE_CHILD | FILE_WRITE_ATTRIBUTES | DELETE | READ_CONTROL | WRITE_DAC |
    WRITE_OWNER | SYNCHRONIZE;

/* generic flags for file open */
pub const GENERIC_READ_FLAGS: u32 = READ_CONTROL | FILE_READ_DATA | FILE_READ_ATTRIBUTES |
    FILE_READ_EA | SYNCHRONIZE;
pub const GENERIC_WRITE_FLAGS: u32 = READ_CONTROL | FILE_WRITE_DATA | FILE_WRITE_ATTRIBUTES |
    FILE_WRITE_EA | FILE_APPEND_DATA | SYNCHRONIZE;
pub const GENERIC_EXECUTE_FLAGS: u32 = READ_CONTROL | FILE_EXECUTE | FILE_READ_ATTRIBUTES | SYNCHRONIZE;
pub const GENERIC_ALL_FLAGS: u32 = DELETE | READ_CONTROL | WRITE_DAC | WRITE_OWNER | SYNCHRONIZE |
    FILE_READ_DATA | FILE_WRITE_DATA | FILE_APPEND_DATA | FILE_READ_EA | FILE_WRITE_EA |
    FILE_EXECUTE | FILE_DELETE_CHILD | FILE_READ_ATTRIBUTES | FILE_WRITE_ATTRIBUTES;

pub const SMB_COM_NEGOTIATE: u8 = 0x72; /* See MS-CIFS 2.2.2.1 */
/* See MS-CIFS 2.2.3.1 */
pub const SMBFLG_RESPONSE: u8 = 0x80;
pub const SMBFLG2_IS_LONG_NAME: u16 = 0x40;
pub const SMBFLG2_EXT_SEC: u16 = 0x800;
pub const SMBFLG2_ERR_STATUS: u16 = 0x4000;
pub const SMBFLG2_UNICODE: u16 = 0x8000;

#[repr(C, packed)]
pub struct smb_negotiate_rsp {
    pub hdr: smb_hdr,
    pub DialectIndex: __le16,
    pub ByteCount: __le16,
}

pub const EXTENDED_INFO_MAGIC: u32 = 0x4367_7364;
pub const STRING_LENGTH: usize = 28;

#[repr(C, packed)]
pub struct fs_extended_info {
    pub magic: __le32,
    pub version: __le32,
    pub release: __le32,
    pub rel_date: __u64,
    pub version_string: [i8; STRING_LENGTH],
}

#[repr(C, packed)]
pub struct object_id_info {
    pub objid: [i8; 16],
    pub extended_info: fs_extended_info,
}

#[repr(C, packed)]
pub struct file_names_info {
    pub NextEntryOffset: __le32,
    pub FileIndex: __u32,
    pub FileNameLength: __le32,
    pub FileName: [i8; 0],
}

#[repr(C, packed)]
pub struct file_id_both_directory_info {
    pub NextEntryOffset: __le32,
    pub FileIndex: __u32,
    pub CreationTime: __le64,
    pub LastAccessTime: __le64,
    pub LastWriteTime: __le64,
    pub ChangeTime: __le64,
    pub EndOfFile: __le64,
    pub AllocationSize: __le64,
    pub ExtFileAttributes: __le32,
    pub FileNameLength: __le32,
    pub EaSize: __le32,
    pub ShortNameLength: __u8,
    pub Reserved: __u8,
    pub ShortName: [__u8; 24],
    pub Reserved2: __le16,
    pub UniqueId: __le64,
    pub FileName: [i8; 0],
}

#[repr(C)]
pub struct smb_version_ops {
    pub get_cmd_val: Option<unsafe extern "C" fn(*mut ksmbd_work) -> u16>,
    pub inc_reqs: Option<unsafe extern "C" fn(u32, __le32)>,
    pub init_rsp_hdr: Option<unsafe extern "C" fn(*mut ksmbd_work) -> i32>,
    pub set_rsp_status: Option<unsafe extern "C" fn(*mut ksmbd_work, __le32)>,
    pub allocate_rsp_buf: Option<unsafe extern "C" fn(*mut ksmbd_work) -> i32>,
    pub set_rsp_credits: Option<unsafe extern "C" fn(*mut ksmbd_work) -> i32>,
    pub check_user_session: Option<unsafe extern "C" fn(*mut ksmbd_work) -> i32>,
    pub get_ksmbd_tcon: Option<unsafe extern "C" fn(*mut ksmbd_work) -> i32>,
    pub is_sign_req: Option<unsafe extern "C" fn(*mut ksmbd_work, u32) -> bool>,
    pub check_sign_req: Option<unsafe extern "C" fn(*mut ksmbd_work) -> i32>,
    pub set_sign_rsp: Option<unsafe extern "C" fn(*mut ksmbd_work)>,
    pub generate_signingkey: Option<unsafe extern "C" fn(*mut ksmbd_session, *mut ksmbd_conn) -> i32>,
    pub generate_encryptionkey: Option<unsafe extern "C" fn(*mut ksmbd_conn, *mut ksmbd_session)>,
    pub is_transform_hdr: Option<unsafe extern "C" fn(*mut c_void) -> bool>,
    pub decrypt_req: Option<unsafe extern "C" fn(*mut ksmbd_work) -> i32>,
    pub encrypt_resp: Option<unsafe extern "C" fn(*mut ksmbd_work) -> i32>,
}

#[repr(C)]
pub struct smb_version_cmds {
    pub proc: Option<unsafe extern "C" fn(*mut ksmbd_work) -> i32>,
}

extern "C" {
    pub fn ksmbd_min_protocol() -> i32;
    pub fn ksmbd_max_protocol() -> i32;
    pub fn ksmbd_get_protocol_string(version: i32) -> *const i8;
    pub fn ksmbd_lookup_protocol_idx(str_: *mut i8) -> i32;
    pub fn ksmbd_verify_smb_message(work: *mut ksmbd_work) -> i32;
    pub fn ksmbd_smb_request(conn: *mut ksmbd_conn) -> bool;
    pub fn ksmbd_lookup_dialect_by_id(cli_dialects: *mut __le16, dialects_count: __le16) -> i32;
    pub fn ksmbd_init_smb_server(conn: *mut ksmbd_conn) -> i32;
    pub fn ksmbd_populate_dot_dotdot_entries(work: *mut ksmbd_work, info_level: i32, dir: *mut ksmbd_file,
        d_info: *mut ksmbd_dir_info, search_pattern: *mut i8,
        fn_: Option<unsafe extern "C" fn(*mut ksmbd_conn, i32, *mut ksmbd_dir_info, *mut ksmbd_kstat) -> i32>) -> i32;
    pub fn ksmbd_extract_shortname(conn: *mut ksmbd_conn, longname: *const i8, shortname: *mut i8) -> i32;
    pub fn ksmbd_smb_negotiate_common(work: *mut ksmbd_work, command: u32) -> i32;
    pub fn ksmbd_smb_check_shared_mode(filp: *mut file, curr_fp: *mut ksmbd_file) -> i32;
    pub fn __ksmbd_override_fsids(work: *mut ksmbd_work, share: *mut ksmbd_share_config) -> i32;
    pub fn ksmbd_override_fsids(work: *mut ksmbd_work) -> i32;
    pub fn ksmbd_revert_fsids(work: *mut ksmbd_work);
    pub fn ksmbd_server_side_copy_max_chunk_count() -> u32;
    pub fn ksmbd_server_side_copy_max_chunk_size() -> u32;
    pub fn ksmbd_server_side_copy_max_total_size() -> u32;
    pub fn is_asterisk(p: *mut i8) -> bool;
    pub fn smb_map_generic_desired_access(daccess: __le32) -> __le32;
}

pub unsafe fn smb_get_msg(buf: *mut c_void) -> *mut c_void {
    (buf as *mut u8).add(4) as *mut c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
