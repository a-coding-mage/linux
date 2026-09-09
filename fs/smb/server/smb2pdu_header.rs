/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2016 Namjae Jeon <linkinjeon@kernel.org>
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// C dependencies: ntlmssp.h and smbacl.h provide the referenced types and constants.

pub const FILE_SUPERSEDED: u32 = 0x00000000;
pub const FILE_OPENED: u32 = 0x00000001;
pub const FILE_CREATED: u32 = 0x00000002;
pub const FILE_OVERWRITTEN: u32 = 0x00000003;
pub const SMB2_MAX_CREDITS: u32 = 8192;
pub const MAX_SMB2_HDR_SIZE: u32 = 0x78;
pub const SMB21_DEFAULT_IOSIZE: u32 = 1024 * 1024;
pub const SMB3_DEFAULT_TRANS_SIZE: u32 = 4 * 1024 * 1024;
pub const SMB3_MIN_IOSIZE: u32 = 64 * 1024;
pub const SMB3_MAX_IOSIZE: u32 = 8 * 1024 * 1024;
pub const SMB3_MAX_MSGSIZE: u32 = 4 * 4096;

#[repr(C, packed)]
pub struct preauth_integrity_info { pub Preauth_HashId: __le16, pub Preauth_HashValue: [__u8; SMB2_PREAUTH_HASH_SIZE as usize] }

// CONFIG_SMB_SERVER_KERBEROS5 selects 0xe0; otherwise the value is 0xd0.
pub const OFFSET_OF_NEG_CONTEXT: u32 = 0xd0;
pub const SMB2_SESSION_EXPIRED: u32 = 0;
pub const SMB2_SESSION_IN_PROGRESS: u32 = BIT(0);
pub const SMB2_SESSION_VALID: u32 = BIT(1);
pub const SMB2_SESSION_TIMEOUT: u32 = 10 * HZ;
pub const SMB2_CREATE_AAPL: &str = "AAPL";
pub const SMB2_CREATE_AAPL_LEN: u32 = 4;
pub const AAPL_READDIR_ATTR_V2_NO_XATTR: u8 = 0x01;
pub const AAPL_MODEL_MAX_CHARS: u32 = 31;
pub const AAPL_MODEL_UTF16_BYTES: u32 = AAPL_MODEL_MAX_CHARS * 2;
pub const AAPL_RSP_MAX_SIZE: u32 = 128;

pub const AAPL_SERVER_CAPS_KSMBD: u32 = SMB2_CRTCTX_AAPL_UNIX_BASED | SMB2_CRTCTX_AAPL_SUPPORTS_OSX_COPYFILE | SMB2_CRTCTX_AAPL_SUPPORTS_READ_DIR_ATTR;

#[repr(C, packed)]
pub struct aapl_server_query_req { pub cmd: __le32, pub reserved: __le32, pub req_bitmap: __le64, pub client_caps: __le64 }
#[repr(C, packed)]
pub struct create_aapl_rsp { pub ccontext: create_context_hdr, pub Name: [__u8; 4], pub Pad: [__u8; 4], pub cmd: __le32, pub reserved: __le32, pub reply_bitmap: __le64, pub server_caps: __le64, pub vol_caps: __le64 }
pub const DURABLE_HANDLE_MAX_TIMEOUT: u32 = 300000;
#[repr(C, packed)]
pub struct create_alloc_size_req { pub ccontext: create_context_hdr, pub Name: [__u8; 8], pub AllocationSize: __le64 }
#[repr(C)]
pub union create_durable_rsp_Data { pub Reserved: [__u8; 8], pub data: __u64 }
#[repr(C, packed)]
pub struct create_durable_rsp { pub ccontext: create_context_hdr, pub Name: [__u8; 8], pub Data: create_durable_rsp_Data }
#[repr(C, packed)]
pub struct create_posix_rsp { pub ccontext: create_context_hdr, pub Name: [__u8; 16], pub nlink: __le32, pub reparse_tag: __le32, pub mode: __le32, pub SidBuffer: [u8; 44] }
pub const SMB2_0_IOCTL_IS_FSCTL: u32 = 0x00000001;
#[repr(C)]
pub union sockaddr_storage_rsp_addr { pub addr4: smb_sockaddr_in, pub addr6: smb_sockaddr_in6 }
#[repr(C, packed)]
pub struct sockaddr_storage_rsp { pub Family: __le16, pub addr: sockaddr_storage_rsp_addr }
#[repr(C, packed)]
pub struct file_object_buf_type1_ioctl_rsp { pub ObjectId: [__u8; 16], pub BirthVolumeId: [__u8; 16], pub BirthObjectId: [__u8; 16], pub DomainId: [__u8; 16] }
#[repr(C, packed)] pub struct file_sparse { pub SetSparse: __u8 }

pub const FILE_DIRECTORY_INFORMATION_SIZE: u32 = 1; pub const FILE_FULL_DIRECTORY_INFORMATION_SIZE: u32 = 2; pub const FILE_BOTH_DIRECTORY_INFORMATION_SIZE: u32 = 3;
pub const FILE_BASIC_INFORMATION_SIZE: u32 = 40; pub const FILE_STANDARD_INFORMATION_SIZE: u32 = 24; pub const FILE_INTERNAL_INFORMATION_SIZE: u32 = 8; pub const FILE_EA_INFORMATION_SIZE: u32 = 4; pub const FILE_ACCESS_INFORMATION_SIZE: u32 = 4; pub const FILE_NAME_INFORMATION_SIZE: u32 = 9; pub const FILE_RENAME_INFORMATION_SIZE: u32 = 10; pub const FILE_LINK_INFORMATION_SIZE: u32 = 11; pub const FILE_NAMES_INFORMATION_SIZE: u32 = 12; pub const FILE_DISPOSITION_INFORMATION_SIZE: u32 = 13; pub const FILE_POSITION_INFORMATION_SIZE: u32 = 14; pub const FILE_FULL_EA_INFORMATION_SIZE: u32 = 15; pub const FILE_MODE_INFORMATION_SIZE: u32 = 4; pub const FILE_ALIGNMENT_INFORMATION_SIZE: u32 = 4; pub const FILE_ALL_INFORMATION_SIZE: u32 = 104; pub const FILE_ALLOCATION_INFORMATION_SIZE: u32 = 19; pub const FILE_END_OF_FILE_INFORMATION_SIZE: u32 = 20; pub const FILE_ALTERNATE_NAME_INFORMATION_SIZE: u32 = 8; pub const FILE_STREAM_INFORMATION_SIZE: u32 = 32; pub const FILE_PIPE_INFORMATION_SIZE: u32 = 23; pub const FILE_PIPE_LOCAL_INFORMATION_SIZE: u32 = 24; pub const FILE_PIPE_REMOTE_INFORMATION_SIZE: u32 = 25; pub const FILE_MAILSLOT_QUERY_INFORMATION_SIZE: u32 = 26; pub const FILE_MAILSLOT_SET_INFORMATION_SIZE: u32 = 27; pub const FILE_COMPRESSION_INFORMATION_SIZE: u32 = 16; pub const FILE_OBJECT_ID_INFORMATION_SIZE: u32 = 29; pub const FILE_MOVE_CLUSTER_INFORMATION_SIZE: u32 = 31; pub const FILE_QUOTA_INFORMATION_SIZE: u32 = 32; pub const FILE_REPARSE_POINT_INFORMATION_SIZE: u32 = 33; pub const FILE_NETWORK_OPEN_INFORMATION_SIZE: u32 = 56; pub const FILE_ATTRIBUTE_TAG_INFORMATION_SIZE: u32 = 8;
pub const FS_DEVICE_INFORMATION_SIZE: u32 = 8; pub const FS_ATTRIBUTE_INFORMATION_SIZE: u32 = 16; pub const FS_VOLUME_INFORMATION_SIZE: u32 = 24; pub const FS_SIZE_INFORMATION_SIZE: u32 = 24; pub const FS_FULL_SIZE_INFORMATION_SIZE: u32 = 32; pub const FS_SECTOR_SIZE_INFORMATION_SIZE: u32 = 28; pub const FS_OBJECT_ID_INFORMATION_SIZE: u32 = 64; pub const FS_CONTROL_INFORMATION_SIZE: u32 = 48; pub const FS_POSIX_INFORMATION_SIZE: u32 = 56; pub const FS_TYPE_SUPPORT_SIZE: u32 = 44;

#[repr(C, packed)] pub struct fs_type_info { pub fs_name: *mut i8, pub magic_number: i64 }
#[repr(C, packed)] pub struct smb2_file_access_info { pub AccessFlags: __le32 }
#[repr(C, packed)] pub struct smb2_file_alignment_info { pub AlignmentRequirement: __le32 }
#[repr(C, packed)] pub struct smb2_file_alt_name_info { pub FileNameLength: __le32, pub FileName: [i8; 0] }
#[repr(C, packed)] pub struct smb2_file_stream_info { pub NextEntryOffset: __le32, pub StreamNameLength: __le32, pub StreamSize: __le64, pub StreamAllocationSize: __le64, pub StreamName: [i8; 0] }
#[repr(C, packed)] pub struct srv_snapshot_array { pub NumberOfSnapShots: __le32, pub NumberOfSnapShotsReturned: __le32, pub SnapShotArraySize: __le32, pub Reserved: __le32 }
#[repr(C, packed)] pub struct smb2_file_standard_info { pub AllocationSize: __le64, pub EndOfFile: __le64, pub NumberOfLinks: __le32, pub DeletePending: __u8, pub Directory: __u8, pub Reserved: __le16 }
#[repr(C, packed)] pub struct smb2_file_ea_info { pub EASize: __le32 }
#[repr(C, packed)] pub struct smb2_file_disposition_info { pub DeletePending: __u8 }
#[repr(C, packed)] pub struct smb2_file_pos_info { pub CurrentByteOffset: __le64 }
pub const FILE_MODE_INFO_MASK: __le32 = cpu_to_le32(0x0000100e);
#[repr(C, packed)] pub struct smb2_file_mode_info { pub Mode: __le32 }
#[repr(C, packed)] pub struct smb2_file_comp_info { pub CompressedFileSize: __le64, pub CompressionFormat: __le16, pub CompressionUnitShift: __u8, pub ChunkShift: __u8, pub ClusterShift: __u8, pub Reserved: [__u8; 3] }
#[repr(C, packed)] pub struct smb2_file_attr_tag_info { pub FileAttributes: __le32, pub ReparseTag: __le32 }
pub const SL_RESTART_SCAN: u32 = 0x00000001; pub const SL_RETURN_SINGLE_ENTRY: u32 = 0x00000002; pub const SL_INDEX_SPECIFIED: u32 = 0x00000004;
#[repr(C, packed)] pub struct smb2_ea_info_req { pub NextEntryOffset: __le32, pub EaNameLength: __u8, pub name: [i8; 0] }
#[repr(C, packed)] pub struct smb2_ea_info { pub NextEntryOffset: __le32, pub Flags: __u8, pub EaNameLength: __u8, pub EaValueLength: __le16, pub name: [i8; 0] }
#[repr(C, packed)] pub struct create_ea_buf_req { pub ccontext: create_context_hdr, pub Name: [__u8; 8], pub ea: smb2_ea_info }
#[repr(C, packed)] pub struct create_sd_buf_req { pub ccontext: create_context_hdr, pub Name: [__u8; 8], pub ntsd: smb_ntsd }
#[repr(C, packed)] pub struct smb2_posix_info { pub NextEntryOffset: __le32, pub Ignored: __u32, pub CreationTime: __le64, pub LastAccessTime: __le64, pub LastWriteTime: __le64, pub ChangeTime: __le64, pub EndOfFile: __le64, pub AllocationSize: __le64, pub DosAttributes: __le32, pub Inode: __le64, pub DeviceId: __le32, pub Zero: __le32, pub HardLinks: __le32, pub ReparseTag: __le32, pub Mode: __le32, pub SidBuffer: [u8; 32], pub name_len: __le32, pub name: [u8; 0] }

// Function declarations retained as external interfaces.
extern "C" {
    pub fn init_smb2_1_server(conn: *mut ksmbd_conn); pub fn init_smb3_0_server(conn: *mut ksmbd_conn); pub fn init_smb3_02_server(conn: *mut ksmbd_conn); pub fn init_smb3_11_server(conn: *mut ksmbd_conn) -> i32;
    pub fn init_smb2_max_read_size(sz: u32); pub fn init_smb2_max_write_size(sz: u32); pub fn init_smb2_max_trans_size(sz: u32); pub fn init_smb2_max_credits(sz: u32);
    pub fn is_smb2_neg_cmd(work: *mut ksmbd_work) -> bool; pub fn is_smb2_rsp(work: *mut ksmbd_work) -> bool; pub fn get_smb2_cmd_val(work: *mut ksmbd_work) -> u16; pub fn set_smb2_rsp_status(work: *mut ksmbd_work, err: __le32); pub fn init_smb2_rsp_hdr(work: *mut ksmbd_work) -> i32; pub fn smb2_allocate_rsp_buf(work: *mut ksmbd_work) -> i32; pub fn is_chained_smb2_message(work: *mut ksmbd_work) -> bool; pub fn init_smb2_neg_rsp(work: *mut ksmbd_work) -> i32; pub fn smb2_set_err_rsp(work: *mut ksmbd_work); pub fn smb2_check_user_session(work: *mut ksmbd_work) -> i32; pub fn smb2_get_ksmbd_tcon(work: *mut ksmbd_work) -> i32; pub fn smb2_is_sign_req(work: *mut ksmbd_work, command: u32) -> bool; pub fn smb2_check_sign_req(work: *mut ksmbd_work) -> i32; pub fn smb2_set_sign_rsp(work: *mut ksmbd_work); pub fn smb3_check_sign_req(work: *mut ksmbd_work) -> i32; pub fn smb3_set_sign_rsp(work: *mut ksmbd_work); pub fn find_matching_smb2_dialect(start_index: i32, cli_dialects: *mut __le16, dialects_count: __le16) -> i32; pub fn smb_flock_init(f: *mut file) -> *mut file_lock; pub fn setup_async_work(work: *mut ksmbd_work, f: Option<unsafe extern "C" fn(*mut *mut core::ffi::c_void)>, arg: *mut *mut core::ffi::c_void) -> i32; pub fn release_async_work(work: *mut ksmbd_work); pub fn smb2_send_interim_resp(work: *mut ksmbd_work, status: __le32); pub fn lookup_chann_list(sess: *mut ksmbd_session, conn: *mut ksmbd_conn) -> *mut channel; pub fn smb3_preauth_hash_rsp(work: *mut ksmbd_work); pub fn smb3_is_transform_hdr(buf: *mut core::ffi::c_void) -> bool; pub fn smb3_decrypt_req(work: *mut ksmbd_work) -> i32; pub fn smb3_encrypt_resp(work: *mut ksmbd_work) -> i32; pub fn smb3_11_final_sess_setup_resp(work: *mut ksmbd_work) -> bool; pub fn smb2_set_rsp_credits(work: *mut ksmbd_work) -> i32; pub fn smb3_encryption_negotiated(conn: *mut ksmbd_conn) -> bool;
    pub fn ksmbd_smb2_check_message(work: *mut ksmbd_work) -> i32; pub fn smb2_complete_request_open(work: *mut ksmbd_work);
    pub fn smb2_handle_negotiate(work: *mut ksmbd_work) -> i32; pub fn smb2_negotiate_request(work: *mut ksmbd_work) -> i32; pub fn smb2_sess_setup(work: *mut ksmbd_work) -> i32; pub fn smb2_tree_connect(work: *mut ksmbd_work) -> i32; pub fn smb2_tree_disconnect(work: *mut ksmbd_work) -> i32; pub fn smb2_session_logoff(work: *mut ksmbd_work) -> i32; pub fn smb2_open(work: *mut ksmbd_work) -> i32; pub fn smb2_query_info(work: *mut ksmbd_work) -> i32; pub fn smb2_query_dir(work: *mut ksmbd_work) -> i32; pub fn smb2_close(work: *mut ksmbd_work) -> i32; pub fn smb2_echo(work: *mut ksmbd_work) -> i32; pub fn smb2_set_info(work: *mut ksmbd_work) -> i32; pub fn smb2_read(work: *mut ksmbd_work) -> i32; pub fn smb2_write(work: *mut ksmbd_work) -> i32; pub fn smb2_flush(work: *mut ksmbd_work) -> i32; pub fn smb2_cancel(work: *mut ksmbd_work) -> i32; pub fn smb2_lock(work: *mut ksmbd_work) -> i32; pub fn smb2_ioctl(work: *mut ksmbd_work) -> i32; pub fn smb2_oplock_break(work: *mut ksmbd_work) -> i32; pub fn smb2_notify(work: *mut ksmbd_work) -> i32;
}

pub const POSIX_TYPE_FILE: u32 = 0; pub const POSIX_TYPE_DIR: u32 = 1; pub const POSIX_TYPE_SYMLINK: u32 = 2; pub const POSIX_TYPE_CHARDEV: u32 = 3; pub const POSIX_TYPE_BLKDEV: u32 = 4; pub const POSIX_TYPE_FIFO: u32 = 5; pub const POSIX_TYPE_SOCKET: u32 = 6; pub const POSIX_FILETYPE_SHIFT: u32 = 12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
