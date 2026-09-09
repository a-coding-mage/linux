/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

/* Translated from connection.h. Kernel and project dependencies are supplied externally. */

pub const KSMBD_SOCKET_BACKLOG: u32 = 16;

/* Size of the per-connection SMB2 command sequence window. */
pub const KSMBD_CMD_SEQ_WINDOW: usize = 8192;

pub const KSMBD_SESS_NEW: i32 = 0;
pub const KSMBD_SESS_GOOD: i32 = 1;
pub const KSMBD_SESS_EXITING: i32 = 2;
pub const KSMBD_SESS_NEED_RECONNECT: i32 = 3;
pub const KSMBD_SESS_NEED_NEGOTIATE: i32 = 4;
pub const KSMBD_SESS_NEED_SETUP: i32 = 5;
pub const KSMBD_SESS_RELEASING: i32 = 6;

pub struct SmbdirectBufferDescriptorV1;
pub struct KsmbdSession;
pub struct KsmbdTransport;

#[repr(C)]
pub struct KsmbdConnStats {
    pub open_files_count: atomic_t,
    pub request_served: atomic64_t,
}

#[repr(C)]
pub struct KsmbdConn {
    pub vals: *mut smb_version_values,
    pub ops: *mut smb_version_ops,
    pub cmds: *mut smb_version_cmds,
    pub max_cmds: u32,
    pub srv_mutex: mutex,
    pub status: i32,
    pub cli_cap: u32,
    pub stop_called: bool,
    pub inet_addr: __be32,
    /* CONFIG_IPV6 conditionally adds inet6_addr: [u8; 16] to this union. */
    pub inet_hash: u32,
    pub request_buf: *mut ::std::ffi::c_char,
    pub transport: *mut KsmbdTransport,
    pub local_nls: *mut nls_table,
    pub um: *mut unicode_map,
    pub hlist: hlist_node,
    pub session_lock: rw_semaphore,
    pub sessions: xarray,
    pub last_active: ::std::ffi::c_ulong,
    pub req_running: atomic_t,
    pub r_count: atomic_t,
    pub total_credits: u32,
    pub outstanding_credits: u32,
    pub credits_lock: spinlock_t,
    pub seq_low: u64,
    pub seq_high: u64,
    pub seq_bitmap: [::std::ffi::c_ulong; KSMBD_CMD_SEQ_WINDOW / (::std::mem::size_of::<::std::ffi::c_ulong>() * 8)],
    pub req_running_q: wait_queue_head_t,
    pub r_count_q: wait_queue_head_t,
    pub request_lock: spinlock_t,
    pub requests: list_head,
    pub async_requests: list_head,
    pub connection_type: i32,
    pub stats: KsmbdConnStats,
    pub ClientGUID: [::std::ffi::c_char; SMB2_CLIENT_GUID_SIZE],
    pub ntlmssp: ntlmssp_auth,
    pub llist_lock: spinlock_t,
    pub lock_list: list_head,
    pub preauth_info: *mut preauth_integrity_info,
    pub need_neg: bool,
    pub auth_mechs: u32,
    pub preferred_auth_mech: u32,
    pub sign: bool,
    pub use_spnego: bool,
    pub cli_sec_mode: __u16,
    pub srv_sec_mode: __u16,
    pub dialect: __u16,
    pub mechToken: *mut ::std::ffi::c_char,
    pub mechTokenLen: u32,
    pub conn_ops: *mut KsmbdConnOps,
    pub preauth_sess_table: list_head,
    pub peer_addr: sockaddr_storage,
    pub async_ida: ida,
    pub cipher_type: __le16,
    pub compress_algorithm: __le16,
    pub compress_chained: bool,
    pub compress_pattern: bool,
    pub rdma_transform_ids: ::std::ffi::c_ulong,
    pub rdma_transform_negotiated: bool,
    pub posix_ext_supported: bool,
    pub signing_negotiated: bool,
    pub signing_algorithm: __le16,
    pub binding: bool,
    pub refcnt: atomic_t,
    pub is_aapl: bool,
    pub aapl_readdir_attr: bool,
    pub aapl_readdir_attr_v2: bool,
    pub release_work: work_struct,
}

#[repr(C)]
pub struct KsmbdConnOps {
    pub process_fn: Option<unsafe extern "C" fn(*mut KsmbdConn) -> i32>,
    pub terminate_fn: Option<unsafe extern "C" fn(*mut KsmbdConn) -> i32>,
}

#[repr(C)]
pub struct KsmbdTransportWrite {
    pub iov: *mut kvec,
    pub iov_cnt: i32,
    pub size: i32,
    pub need_invalidate_rkey: bool,
    pub remote_key: u32,
    pub msg_flags: i32,
}

#[repr(C)]
pub struct KsmbdTransportOps {
    pub disconnect: Option<unsafe extern "C" fn(*mut KsmbdTransport)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut KsmbdTransport)>,
    pub read: Option<unsafe extern "C" fn(*mut KsmbdTransport, *mut ::std::ffi::c_char, u32, i32) -> i32>,
    pub writev: Option<unsafe extern "C" fn(*mut KsmbdTransport, *const KsmbdTransportWrite) -> i32>,
    pub rdma_read: Option<unsafe extern "C" fn(*mut KsmbdTransport, *mut ::std::ffi::c_void, u32, *mut SmbdirectBufferDescriptorV1, u32) -> i32>,
    pub rdma_write: Option<unsafe extern "C" fn(*mut KsmbdTransport, *mut ::std::ffi::c_void, u32, *mut SmbdirectBufferDescriptorV1, u32) -> i32>,
    pub free_transport: Option<unsafe extern "C" fn(*mut KsmbdTransport)>,
}

#[repr(C)]
pub struct KsmbdTransport {
    pub conn: *mut KsmbdConn,
    pub ops: *const KsmbdTransportOps,
}

pub const KSMBD_TCP_RECV_TIMEOUT: u32 = 7 * HZ;
pub const KSMBD_TCP_SEND_TIMEOUT: u32 = 5 * HZ;
pub const CONN_HASH_BITS: u32 = 12;

extern "C" {
    pub static mut conn_list: hashtable;
    pub static mut conn_list_lock: rw_semaphore;
    pub fn ksmbd_conn_alive(conn: *mut KsmbdConn) -> bool;
    pub fn ksmbd_conn_wait_idle(conn: *mut KsmbdConn);
    pub fn ksmbd_conn_wait_idle_sess(conn: *mut KsmbdConn, sess: *mut KsmbdSession) -> i32;
    pub fn ksmbd_conn_alloc() -> *mut KsmbdConn;
    pub fn ksmbd_conn_free(conn: *mut KsmbdConn);
    pub fn ksmbd_conn_get(conn: *mut KsmbdConn) -> *mut KsmbdConn;
    pub fn ksmbd_conn_put(conn: *mut KsmbdConn);
    pub fn ksmbd_conn_abort(conn: *mut KsmbdConn);
    pub fn ksmbd_conn_wq_init() -> i32;
    pub fn ksmbd_conn_wq_destroy();
    pub fn ksmbd_conn_lookup_dialect(c: *mut KsmbdConn) -> bool;
    pub fn ksmbd_conn_write(work: *mut ksmbd_work) -> i32;
    pub fn ksmbd_conn_write_eor(work: *mut ksmbd_work) -> i32;
    pub fn ksmbd_conn_rdma_read(conn: *mut KsmbdConn, buf: *mut ::std::ffi::c_void, buflen: u32, desc: *mut SmbdirectBufferDescriptorV1, desc_len: u32) -> i32;
    pub fn ksmbd_conn_rdma_write(conn: *mut KsmbdConn, buf: *mut ::std::ffi::c_void, buflen: u32, desc: *mut SmbdirectBufferDescriptorV1, desc_len: u32) -> i32;
    pub fn ksmbd_conn_enqueue_request(work: *mut ksmbd_work);
    pub fn ksmbd_conn_try_dequeue_request(work: *mut ksmbd_work);
    pub fn ksmbd_conn_init_server_callbacks(ops: *mut KsmbdConnOps);
    pub fn ksmbd_conn_handler_loop(p: *mut ::std::ffi::c_void) -> i32;
    pub fn ksmbd_conn_transport_init() -> i32;
    pub fn ksmbd_conn_transport_destroy();
    pub fn ksmbd_conn_lock(conn: *mut KsmbdConn);
    pub fn ksmbd_conn_unlock(conn: *mut KsmbdConn);
    pub fn ksmbd_conn_r_count_inc(conn: *mut KsmbdConn);
    pub fn ksmbd_conn_r_count_dec(conn: *mut KsmbdConn);
    pub fn ksmbd_all_conn_set_status(sess: *mut KsmbdSession, status: u32);
}

#[inline]
pub unsafe fn ksmbd_conn_new(conn: *mut KsmbdConn) -> bool { (*conn).status == KSMBD_SESS_NEW }
#[inline]
pub unsafe fn ksmbd_conn_good(conn: *mut KsmbdConn) -> bool { (*conn).status == KSMBD_SESS_GOOD }
#[inline]
pub unsafe fn ksmbd_max_allowed_pdu_size(conn: *mut KsmbdConn) -> u32 { if ksmbd_conn_good(conn) { SMB3_MAX_MSGSIZE + (*(*conn).vals).max_write_size } else { SMB3_MAX_MSGSIZE } }
#[inline]
pub unsafe fn ksmbd_conn_need_negotiate(conn: *mut KsmbdConn) -> bool { (*conn).status == KSMBD_SESS_NEED_NEGOTIATE }
#[inline]
pub unsafe fn ksmbd_conn_need_setup(conn: *mut KsmbdConn) -> bool { (*conn).status == KSMBD_SESS_NEED_SETUP }
#[inline]
pub unsafe fn ksmbd_conn_need_reconnect(conn: *mut KsmbdConn) -> bool { (*conn).status == KSMBD_SESS_NEED_RECONNECT }
#[inline]
pub unsafe fn ksmbd_conn_exiting(conn: *mut KsmbdConn) -> bool { (*conn).status == KSMBD_SESS_EXITING }
#[inline]
pub unsafe fn ksmbd_conn_releasing(conn: *mut KsmbdConn) -> bool { (*conn).status == KSMBD_SESS_RELEASING }

macro_rules! conn_status_setter { ($name:ident, $value:expr) => { #[inline] pub unsafe fn $name(conn: *mut KsmbdConn) { (*conn).status = $value; } }; }
conn_status_setter!(ksmbd_conn_set_new, KSMBD_SESS_NEW);
conn_status_setter!(ksmbd_conn_set_good, KSMBD_SESS_GOOD);
conn_status_setter!(ksmbd_conn_set_need_negotiate, KSMBD_SESS_NEED_NEGOTIATE);
conn_status_setter!(ksmbd_conn_set_need_setup, KSMBD_SESS_NEED_SETUP);
conn_status_setter!(ksmbd_conn_set_need_reconnect, KSMBD_SESS_NEED_RECONNECT);
conn_status_setter!(ksmbd_conn_set_exiting, KSMBD_SESS_EXITING);
conn_status_setter!(ksmbd_conn_set_releasing, KSMBD_SESS_RELEASING);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
