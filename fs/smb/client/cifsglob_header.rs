/* SPDX-License-Identifier: LGPL-2.1 */
/* Source-level Rust translation of cifsglob.h.  Kernel dependencies are supplied externally. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::{ffi::c_void, mem::MaybeUninit};

pub const SMB_PATH_MAX: usize = 260;
pub const CIFS_PORT: u16 = 445;
pub const RFC1001_PORT: u16 = 139;
pub const MAX_UID_INFO: usize = 16;
pub const MAX_SES_INFO: usize = 2;
pub const MAX_TCON_INFO: usize = 4;
pub const CIFS_MIN_RCV_POOL: usize = 4;
pub const MAX_REOPEN_ATT: usize = 5;
pub const CIFS_MAX_SLEEP: u32 = 2000;
pub const SMB3_MAX_HANDLE_TIMEOUT: u32 = 960000;
pub const CIFS_MAX_REQ: u32 = 32767;
pub const RFC1001_NAME_LEN: usize = 15;
pub const RFC1001_NAME_LEN_WITH_NULL: usize = RFC1001_NAME_LEN + 1;
pub const SERVER_NAME_LENGTH: usize = 80;
pub const SERVER_NAME_LEN_WITH_NULL: usize = SERVER_NAME_LENGTH + 1;
pub const SMB_ECHO_INTERVAL_MIN: u32 = 1;
pub const SMB_ECHO_INTERVAL_MAX: u32 = 600;
pub const SMB_ECHO_INTERVAL_DEFAULT: u32 = 60;
pub const SMB_INTERFACE_POLL_INTERVAL: u32 = 600;
pub const MAX_COMPOUND: usize = 10;
pub const SMB2_MAX_CREDITS_AVAILABLE: u32 = 32000;

/* Types and constants supplied by the kernel and sibling headers. */
pub type __u8 = u8; pub type __u16 = u16; pub type __u32 = u32; pub type __u64 = u64;
pub type __le16 = u16; pub type __le32 = u32; pub type __le64 = u64;
pub type u8_t = u8; pub type u16_t = u16; pub type u32_t = u32; pub type u64_t = u64;
pub type size_t = usize; pub type ssize_t = isize; pub type loff_t = i64; pub type time64_t = i64;
pub type umode_t = u32; pub type dev_t = u64; pub type pid_t = i32;
pub type kuid_t = u32; pub type kgid_t = u32;
macro_rules! opaque { ($($n:ident),* $(,)?) => { $(#[repr(C)] pub struct $n { _private: [u8; 0] })* } }
opaque!(crypto_aead, smb_sid, smb_ace, cifs_ntace, kvec, reparse_data_buffer, smb2_file_all_info,
    smb311_posix_qinfo, iov_iter, folio_queue, net, socket, sockaddr_storage, net_device,
    wait_queue_head_t, list_head, spinlock_t, mutex, task_struct, delayed_work, atomic_t,
    work_struct, workqueue_struct, mempool_t, kref, rb_node, rw_semaphore, netfs_inode, inode,
    super_block, file, dentry, file_lock, seq_file, kstatfs, nls_table, smbd_connection,
    fscache_volume, cached_fids, fiemap_extent_info, smb_ntsd, md5_ctx, hmac_sha256_ctx,
    aes_cmac_ctx, smb2_hdr, FILE_BASIC_INFO, FILE_ALL_INFO, FILE_SYSTEM_DEVICE_INFO,
    FILE_SYSTEM_ATTRIBUTE_INFO, FILE_SYSTEM_UNIX_INFO, slow_work_ops, smb_version_values,
    cifs_sb_info, smb3_fs_context, smb2_file_rename_info_hdr, smb2_file_link_info_hdr);

#[repr(C)] pub struct session_key { pub len: u32, pub response: *mut i8 }
#[repr(C)] pub struct cifs_secmech { pub enc: *mut crypto_aead, pub dec: *mut crypto_aead }
#[repr(C)] pub struct ntlmssp_auth { pub sesskey_per_smbsess: bool, pub client_flags: __u32, pub server_flags: __u32, pub ciphertext: [u8; 64], pub cryptkey: [i8; 64] }
#[repr(C)] pub struct cifs_cred { pub uid: i32, pub gid: i32, pub mode: i32, pub cecount: i32, pub osid: smb_sid, pub gsid: smb_sid, pub ntaces: *mut cifs_ntace, pub aces: *mut smb_ace }
#[repr(C)] pub struct cifs_open_info_data {
    pub adjust_tz: bool, pub reparse_point: bool, pub contains_posix_file_info: bool, pub unknown_nlink: bool,
    pub symlink_target: *mut i8, pub posix_owner: smb_sid, pub posix_group: smb_sid,
    pub reparse_tag: __u32,
}
#[repr(C)] pub struct smb_rqst { pub rq_iov: *mut kvec, pub rq_nvec: u32, pub rq_iter: iov_iter, pub rq_buffer: *mut folio_queue }

pub type mid_q_entry = c_void; pub type cifsFileInfo = c_void; pub type cifs_io_subrequest = c_void;
pub type cifs_io_parms = c_void; pub type cifs_search_info = c_void; pub type cifs_fid = c_void;

#[repr(C)] pub struct smb_version_operations {
    pub send_cancel: Option<unsafe extern "C" fn(*mut c_void,*mut c_void,*mut smb_rqst,*mut c_void,u32)->i32>,
    pub compare_fids: Option<unsafe extern "C" fn(*mut c_void,*mut c_void)->bool>,
    pub setup_request: Option<unsafe extern "C" fn(*mut c_void,*mut c_void,*mut smb_rqst)->*mut c_void>,
    pub check_receive: Option<unsafe extern "C" fn(*mut c_void,*mut c_void,bool)->i32>,
    pub add_credits: Option<unsafe extern "C" fn(*mut c_void,*mut cifs_credits,i32)>,
    pub set_credits: Option<unsafe extern "C" fn(*mut c_void,i32)>,
    pub get_credits_field: Option<unsafe extern "C" fn(*mut c_void,i32)->*mut i32>,
    pub get_credits: Option<unsafe extern "C" fn(*mut c_void)->u32>,
    pub get_next_mid: Option<unsafe extern "C" fn(*mut c_void)->u64>,
    pub adjust_credits: Option<unsafe extern "C" fn(*mut c_void,*mut c_void,u32)->i32>,
    pub _reserved: [usize; 96],
}
#[repr(C)] pub struct TCP_Server_Info { pub ops: *mut smb_version_operations, pub vals: *mut smb_version_values, pub req_lock: spinlock_t, pub in_flight: u32, pub nofs_flag: u32, pub reconnect: delayed_work, pub reconn_delay: usize, pub request_q: wait_queue_head_t, pub server_RFC1001_name: [i8; RFC1001_NAME_LEN_WITH_NULL], pub primary_server: *mut TCP_Server_Info }
#[repr(C)] pub struct cifs_credits { pub value: u32, pub instance: u32, pub in_flight_check: u32, pub rreq_debug_id: u32, pub rreq_debug_index: u32 }

#[repr(C)] pub enum statusEnum { CifsNew=0, CifsGood, CifsExiting, CifsNeedReconnect, CifsNeedNegotiate, CifsInNegotiate }
#[repr(C)] pub enum ses_status_enum { SES_NEW=0, SES_GOOD, SES_EXITING, SES_NEED_RECON, SES_IN_SETUP }
#[repr(C)] pub enum tid_status_enum { TID_NEW=0, TID_GOOD, TID_EXITING, TID_NEED_RECON, TID_NEED_TCON, TID_IN_TCON, TID_NEED_FILES_INVALIDATE, TID_IN_FILES_INVALIDATE }
#[repr(C)] pub enum securityEnum { Unspecified=0, NTLMv2, RawNTLMSSP, Kerberos, IAKerb }
#[repr(C)] pub enum upcall_target_enum { UPTARGET_UNSPECIFIED=0, UPTARGET_MOUNT, UPTARGET_APP }
#[repr(C)] pub enum cifs_reparse_type { CIFS_REPARSE_TYPE_NONE=0, CIFS_REPARSE_TYPE_NFS, CIFS_REPARSE_TYPE_WSL, CIFS_REPARSE_TYPE_DEFAULT=CIFS_REPARSE_TYPE_NFS }
#[repr(C)] pub enum cifs_symlink_type { CIFS_SYMLINK_TYPE_DEFAULT=0, CIFS_SYMLINK_TYPE_NONE, CIFS_SYMLINK_TYPE_NATIVE, CIFS_SYMLINK_TYPE_UNIX, CIFS_SYMLINK_TYPE_MFSYMLINKS, CIFS_SYMLINK_TYPE_SFU, CIFS_SYMLINK_TYPE_NFS, CIFS_SYMLINK_TYPE_WSL }

pub unsafe fn cifs_reparse_type_str(t: cifs_reparse_type) -> *const i8 { match t { cifs_reparse_type::CIFS_REPARSE_TYPE_NONE=>b"none\0".as_ptr() as _, cifs_reparse_type::CIFS_REPARSE_TYPE_NFS=>b"nfs\0".as_ptr() as _, cifs_reparse_type::CIFS_REPARSE_TYPE_WSL=>b"wsl\0".as_ptr() as _, _=>b"unknown\0".as_ptr() as _ } }
pub unsafe fn cifs_symlink_type_str(t: cifs_symlink_type) -> *const i8 { match t { cifs_symlink_type::CIFS_SYMLINK_TYPE_NONE=>b"none\0".as_ptr() as _, cifs_symlink_type::CIFS_SYMLINK_TYPE_NATIVE=>b"native\0".as_ptr() as _, cifs_symlink_type::CIFS_SYMLINK_TYPE_UNIX=>b"unix\0".as_ptr() as _, cifs_symlink_type::CIFS_SYMLINK_TYPE_MFSYMLINKS=>b"mfsymlinks\0".as_ptr() as _, cifs_symlink_type::CIFS_SYMLINK_TYPE_SFU=>b"sfu\0".as_ptr() as _, cifs_symlink_type::CIFS_SYMLINK_TYPE_NFS=>b"nfs\0".as_ptr() as _, cifs_symlink_type::CIFS_SYMLINK_TYPE_WSL=>b"wsl\0".as_ptr() as _, _=>b"unknown\0".as_ptr() as _ } }

#[repr(C)] pub struct cifs_chan { pub in_reconnect: u32, pub server: *mut TCP_Server_Info, pub iface: *mut cifs_server_iface, pub signkey: [u8; 64] }
#[repr(C)] pub struct cifs_server_iface { pub iface_head: list_head, pub refcount: kref, pub speed: usize, pub weight_fulfilled: usize, pub num_channels: u32, pub rdma_capable: u32, pub rss_capable: u32, pub is_active: u32, pub sockaddr: sockaddr_storage }
#[repr(C)] pub struct cifs_ses { pub server: *mut TCP_Server_Info, pub ses_status: ses_status_enum, pub ses_lock: spinlock_t, pub session_mutex: mutex, pub capabilities: u32, pub sectype: securityEnum, pub upcall_target: upcall_target_enum, pub workstation_name: [i8; 256], pub chans: [cifs_chan; 16], pub chan_count: usize, pub chans_need_reconnect: usize, pub dfs_root_ses: *mut cifs_ses }
#[repr(C)] pub struct cifs_tcon { pub ses: *mut cifs_ses, pub status: tid_status_enum, pub Flags: __u16, pub share_flags: u32, pub fsAttrInfo: FILE_SYSTEM_ATTRIBUTE_INFO, pub posix_extensions: bool, pub stat_lock: spinlock_t, pub bytes_read: u64, pub bytes_written: u64 }
#[repr(C)] pub struct tcon_link { pub tl_uid: kuid_t, pub tl_flags: usize, pub tl_time: usize, pub tl_count: atomic_t, pub tl_tcon: *mut cifs_tcon }
#[repr(C)] pub struct cifs_fid { pub netfid: u16, pub persistent_fid: u64, pub volatile_fid: u64, pub lease_key: [u8; 16], pub parent_lease_key: [u8; 16], pub pending_open: *mut c_void, pub epoch: u16, pub purge_cache: bool }
#[repr(C)] pub struct cifsFileInfo { pub uid: kuid_t, pub pid: u32, pub fid: cifs_fid, pub dentry: *mut dentry, pub tlink: *mut tcon_link, pub count: i32, pub file_info_lock: spinlock_t }
#[repr(C)] pub struct mid_q_entry { pub refcount: atomic_t, pub mid: u64, pub credits: u16, pub credits_received: u16, pub pid: u32, pub callback_data: *mut c_void, pub resp_buf: *mut c_void, pub mid_state: i32, pub mid_rc: i32, pub mid_lock: spinlock_t }

pub const CIFS_CACHE_READ_FLG:u32=1; pub const CIFS_CACHE_HANDLE_FLG:u32=2; pub const CIFS_CACHE_WRITE_FLG:u32=4;
pub const CIFS_MAX_CHANNELS:usize=16; pub const CIFS_INVAL_CHAN_INDEX:i32=-1;
pub const CIFS_OPLOCK_NO_CHANGE:u8=0xfe; pub const CIFS_INO_BLOCK_SIZE:u64=512;
pub const MID_FREE:u32=0; pub const MID_REQUEST_ALLOCATED:u32=1; pub const MID_REQUEST_SUBMITTED:u32=2; pub const MID_RESPONSE_RECEIVED:u32=4; pub const MID_RETRY_NEEDED:u32=8; pub const MID_RESPONSE_MALFORMED:u32=0x10; pub const MID_SHUTDOWN:u32=0x20; pub const MID_RESPONSE_READY:u32=0x40; pub const MID_RC:u32=0x80;
pub const CIFS_NO_BUFFER:u32=0; pub const CIFS_SMALL_BUFFER:u32=1; pub const CIFS_LARGE_BUFFER:u32=2; pub const CIFS_DYNAMIC_BUFFER:u32=3; pub const CIFS_IOVEC:u32=4;
pub const CIFSSEC_MAY_SIGN:u32=0x00001; pub const CIFSSEC_MAY_NTLMV2:u32=0x00004; pub const CIFSSEC_MAY_KRB5:u32=0x00008; pub const CIFSSEC_MAY_SEAL:u32=0x00040; pub const CIFSSEC_MAY_NTLMSSP:u32=0x00080;
pub const CIFSSEC_MUST_SIGN:u32=0x01001; pub const CIFSSEC_MUST_NTLMV2:u32=0x04004; pub const CIFSSEC_MUST_KRB5:u32=0x08008; pub const CIFSSEC_MUST_SEAL:u32=0x40040; pub const CIFSSEC_MUST_NTLMSSP:u32=0x80080;
pub const CIFSSEC_DEF:u32=CIFSSEC_MAY_SIGN|CIFSSEC_MAY_NTLMV2|CIFSSEC_MAY_NTLMSSP|CIFSSEC_MAY_SEAL; pub const CIFSSEC_MAX:u32=CIFSSEC_MAY_SIGN|CIFSSEC_MUST_KRB5|CIFSSEC_MAY_SEAL; pub const CIFSSEC_AUTH_MASK:u32=CIFSSEC_MAY_NTLMV2|CIFSSEC_MAY_KRB5|CIFSSEC_MAY_NTLMSSP;

extern "C" { pub static mut cifs_tcp_ses_list: list_head; pub static mut cifs_tcp_ses_lock: spinlock_t; pub static mut GlobalCurrentXid:u32; pub static mut GlobalTotalActiveXid:u32; pub static mut GlobalMaxActiveXid:u32; pub static mut GlobalMid_Lock:spinlock_t; pub static mut enable_oplocks:bool; pub static mut global_secflags:u32; pub static mut CIFSMaxBufSize:u32; }

pub unsafe fn cifsFileInfo_get_locked(f: *mut cifsFileInfo) { (*f).count = (*f).count.wrapping_add(1); }
pub unsafe fn cifs_flock_len(start:i64, end:i64) -> u64 { (end as u64).wrapping_sub(start as u64).wrapping_add(1) }
pub const CIFS_RECONN_DELAY_SECS:u64=30; pub const CIFS_MAX_RECONN_DELAY:u64=120;
pub const CIFS_INO_BYTES: fn(u64)->u64 = |blocks| blocks * CIFS_INO_BLOCK_SIZE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
