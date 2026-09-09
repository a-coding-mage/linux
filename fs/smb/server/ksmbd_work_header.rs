/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2019 Samsung Electronics Co., Ltd.
 */

// Linux dependencies: ctype and workqueue declarations are supplied externally.

use core::ffi::c_void;

#[repr(C)]
pub struct ksmbd_conn { _private: [u8; 0] }
#[repr(C)]
pub struct ksmbd_session { _private: [u8; 0] }
#[repr(C)]
pub struct ksmbd_tree_connect { _private: [u8; 0] }
#[repr(C)]
pub struct ksmbd_file { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct kvec { pub iov_base: *mut c_void, pub iov_len: usize }
#[repr(C)]
pub struct cred { _private: [u8; 0] }
#[repr(C)]
pub struct work_struct { _private: [u8; 0] }

pub type __le16 = u16;
pub type __le32 = u32;
pub type u64 = u64;

pub const KSMBD_WORK_INLINE_IOVS: usize = 4;

pub const KSMBD_WORK_ACTIVE: u32 = 0;
pub const KSMBD_WORK_CANCELLED: u32 = 1;
pub const KSMBD_WORK_CLOSED: u32 = 2;

#[repr(C)]
pub struct aux_read {
    pub buf: *mut c_void,
    pub entry: list_head,
}

/* one of these for every pending CIFS request at the connection */
#[repr(C)]
pub struct ksmbd_work {
    /* Server corresponding to this mid */
    pub conn: *mut ksmbd_conn,
    pub sess: *mut ksmbd_session,
    pub tcon: *mut ksmbd_tree_connect,

    /* Pointer to received SMB header */
    pub request_buf: *mut c_void,
    /* Response buffer */
    pub response_buf: *mut c_void,

    pub aux_read_list: list_head,

    pub iov: *mut kvec,
    pub iov_alloc_cnt: i32,
    pub iov_cnt: i32,
    pub iov_idx: i32,
    pub iov_inline: [kvec; KSMBD_WORK_INLINE_IOVS],

    /* Next cmd hdr in compound req buf*/
    pub next_smb2_rcv_hdr_off: i32,
    /* Next cmd hdr in compound rsp buf*/
    pub next_smb2_rsp_hdr_off: i32,
    /* Current cmd hdr in compound rsp buf*/
    pub curr_smb2_rsp_hdr_off: i32,

    /*
     * Current Local FID assigned compound response if SMB2 CREATE
     * command is present in compound request
     */
    pub compound_fid: u64,
    pub compound_pfid: u64,
    pub compound_sid: u64,
    pub compound_status: __le32,

    pub saved_cred: *const cred,
    /* Number of granted credits */
    pub credits_granted: u32,
    /*
     * Credit charge added to conn->outstanding_credits at receive time
     * for the SMB2 PDU currently being processed, pending release.  Zero
     * once the charge has been returned (on the response or error path).
     */
    pub credit_charge: u16,
    /* response smb header size */
    pub response_sz: u32,

    pub tr_buf: *mut c_void,
    /* Contiguous SMB2 compression transform owned by this work item. */
    pub compress_buf: *mut c_void,

    pub state: u8,
    /* C bit-fields are represented as individual byte flags. */
    pub send_no_response: bool,
    pub encrypted: bool,
    pub compress_response: bool,
    pub asynchronous: bool,
    pub owns_conn_ref: bool,
    pub need_invalidate_rkey: bool,
    pub request_open_chseq_tracked: bool,
    pub session_setup_reauth: bool,

    pub remote_key: u32,
    /* cancel works */
    pub async_id: i32,
    pub cancel_argv: *mut *mut c_void,
    pub cancel_fn: Option<unsafe extern "C" fn(argv: *mut *mut c_void)>,

    /*
     * Refcounted open associated with the SMB2 command currently being
     * processed.
     */
    pub request_open: *mut ksmbd_file,
    pub request_open_chseq: __le16,

    pub work: work_struct,
    /* List head at conn->requests */
    pub request_entry: list_head,
    /* List head at conn->async_requests */
    pub async_request_entry: list_head,
    pub fp_entry: list_head,
    /* List head at ksmbd_file->notify_pendings */
    pub notify_entry: list_head,
}

/**
 * ksmbd_resp_buf_next - Get next buffer on compound response.
 * @work: smb work containing response buffer
 */
#[inline]
pub unsafe fn ksmbd_resp_buf_next(work: *mut ksmbd_work) -> *mut c_void {
    ((*work).response_buf as *mut u8).add((*work).next_smb2_rsp_hdr_off as usize + 4) as *mut c_void
}

/**
 * ksmbd_resp_buf_curr - Get current buffer on compound response.
 * @work: smb work containing response buffer
 */
#[inline]
pub unsafe fn ksmbd_resp_buf_curr(work: *mut ksmbd_work) -> *mut c_void {
    ((*work).response_buf as *mut u8).add((*work).curr_smb2_rsp_hdr_off as usize + 4) as *mut c_void
}

/**
 * ksmbd_req_buf_next - Get next buffer on compound request.
 * @work: smb work containing response buffer
 */
#[inline]
pub unsafe fn ksmbd_req_buf_next(work: *mut ksmbd_work) -> *mut c_void {
    ((*work).request_buf as *mut u8).add((*work).next_smb2_rcv_hdr_off as usize + 4) as *mut c_void
}

unsafe extern "C" {
    pub fn ksmbd_alloc_work_struct() -> *mut ksmbd_work;
    pub fn ksmbd_free_work_struct(work: *mut ksmbd_work);
    pub fn ksmbd_work_pool_destroy();
    pub fn ksmbd_work_pool_init() -> i32;
    pub fn ksmbd_workqueue_init() -> i32;
    pub fn ksmbd_workqueue_destroy();
    pub fn ksmbd_queue_work(work: *mut ksmbd_work) -> bool;
    pub fn ksmbd_iov_pin_rsp_read(work: *mut ksmbd_work, ib: *mut c_void, len: i32,
                                  aux_buf: *mut c_void, aux_size: u32) -> i32;
    pub fn ksmbd_iov_pin_rsp(work: *mut ksmbd_work, ib: *mut c_void, len: i32) -> i32;
    pub fn allocate_interim_rsp_buf(work: *mut ksmbd_work) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
