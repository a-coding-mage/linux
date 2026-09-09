/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2016 Namjae Jeon <linkinjeon@kernel.org>
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

/* Dependency declarations are supplied by smb_common.h in the source tree. */
use core::ffi::c_char;

pub const OPLOCK_WAIT_TIME: usize = 35 * HZ;

/* Oplock states */
pub const OPLOCK_STATE_NONE: u32 = 0x00;
pub const OPLOCK_ACK_WAIT: u32 = 0x01;
pub const OPLOCK_CLOSING: u32 = 0x02;

pub const OPLOCK_WRITE_TO_READ: u32 = 0x01;
pub const OPLOCK_READ_HANDLE_TO_READ: u32 = 0x02;
pub const OPLOCK_WRITE_TO_NONE: u32 = 0x04;
pub const OPLOCK_READ_TO_NONE: u32 = 0x08;

#[repr(C)]
pub struct lease_ctx_info {
    pub lease_key: [u8; SMB2_LEASE_KEY_SIZE],
    pub req_state: __le32,
    pub flags: __le32,
    pub duration: __le64,
    pub parent_lease_key: [u8; SMB2_LEASE_KEY_SIZE],
    pub epoch: __le16,
    pub version: i32,
    pub is_dir: bool,
}

#[repr(C)]
pub struct lease_table {
    pub client_guid: [c_char; SMB2_CLIENT_GUID_SIZE],
    pub conn: *mut ksmbd_conn,
    pub lease_list: list_head,
    pub l_entry: list_head,
    pub lb_lock: spinlock_t,
}

#[repr(C)]
pub struct lease {
    pub lease_key: [u8; SMB2_LEASE_KEY_SIZE],
    pub state: __le32,
    pub new_state: __le32,
    pub flags: __le32,
    pub duration: __le64,
    pub parent_lease_key: [u8; SMB2_LEASE_KEY_SIZE],
    pub version: i32,
    pub epoch: u16,
    pub is_dir: bool,
    pub reuse_epoch: bool,
    pub ci: *mut ksmbd_inode,
    pub l_lb: *mut lease_table,
    pub l_entry: list_head,
    pub open_list: list_head,
    pub lock: spinlock_t,
    pub refcount: atomic_t,
}

#[repr(C)]
pub struct oplock_info {
    pub conn: *mut ksmbd_conn,
    pub sess: *mut ksmbd_session,
    pub work: *mut ksmbd_work,
    pub o_fp: *mut ksmbd_file,
    pub level: i32,
    pub op_state: i32,
    pub state_lock: spinlock_t,
    pub pending_break: usize,
    pub fid: u64,
    pub breaking_cnt: atomic_t,
    pub refcount: atomic_t,
    pub Tid: u16,
    pub is_lease: bool,
    pub open_trunc: bool, /* truncate on open */
    pub o_lease: *mut lease,
    pub op_entry: list_head,
    pub lease_entry: list_head,
    pub oplock_q: wait_queue_head_t, /* Other server threads */
    pub oplock_brk: wait_queue_head_t, /* oplock breaking wait */
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct lease_break_info {
    pub curr_state: __le32,
    pub new_state: __le32,
    pub epoch: __le16,
    pub lease_key: [c_char; SMB2_LEASE_KEY_SIZE],
}

#[repr(C)]
pub struct oplock_break_info {
    pub level: i32,
    pub open_trunc: i32,
    pub fid: i32,
}

extern "C" {
    pub fn smb_grant_oplock(work: *mut ksmbd_work, req_op_level: i32, pid: u64,
                            fp: *mut ksmbd_file, tid: u16,
                            lctx: *mut lease_ctx_info, share_ret: i32, replay: bool) -> i32;
    pub fn smb_break_all_levII_oplock(work: *mut ksmbd_work, fp: *mut ksmbd_file, is_trunc: i32);
    pub fn smb_break_all_levII_oplock_rename(work: *mut ksmbd_work, fp: *mut ksmbd_file);
    pub fn smb_break_all_levII_oplock_no_interim(work: *mut ksmbd_work, fp: *mut ksmbd_file, is_trunc: i32);
    pub fn smb_break_all_levII_oplock_for_delete(work: *mut ksmbd_work, fp: *mut ksmbd_file);
    pub fn opinfo_write_to_read(opinfo: *mut oplock_info) -> i32;
    pub fn opinfo_read_handle_to_read(opinfo: *mut oplock_info) -> i32;
    pub fn opinfo_write_to_none(opinfo: *mut oplock_info) -> i32;
    pub fn opinfo_read_to_none(opinfo: *mut oplock_info) -> i32;
    pub fn close_id_del_oplock(fp: *mut ksmbd_file);
    pub fn smb_break_all_oplock(work: *mut ksmbd_work, fp: *mut ksmbd_file);
    pub fn opinfo_get(fp: *mut ksmbd_file) -> *mut oplock_info;
    pub fn opinfo_put(opinfo: *mut oplock_info);

    /* Lease related functions */
    pub fn create_lease_buf(rbuf: *mut u8, lease: *mut lease);
    pub fn parse_lease_state(open_req: *mut core::ffi::c_void) -> *mut lease_ctx_info;
    pub fn smb2_map_lease_to_oplock(lease_state: __le32) -> u8;
    pub fn lease_update_oplock_levels(lease: *mut lease);
    pub fn lease_read_to_write(opinfo: *mut oplock_info) -> i32;

    /* Durable related functions */
    pub fn create_durable_rsp_buf(cc: *mut c_char);
    pub fn create_durable_v2_rsp_buf(cc: *mut c_char, fp: *mut ksmbd_file);
    pub fn create_mxac_rsp_buf(cc: *mut c_char, maximal_access: i32);
    pub fn create_disk_id_rsp_buf(cc: *mut c_char, file_id: u64, vol_id: u64);
    pub fn create_posix_rsp_buf(cc: *mut c_char, fp: *mut ksmbd_file);
    pub fn create_aapl_rsp_buf(cc: *mut c_char, vol_caps: u64, req_bitmap: u64, readdir_attr_v2: bool);
    pub fn smb2_find_context_vals(open_req: *mut core::ffi::c_void, tag: *const c_char, tag_len: i32) -> *mut create_context;
    pub fn lookup_lease_in_table(conn: *mut ksmbd_conn, lease_key: *mut c_char) -> *mut oplock_info;
    pub fn find_same_lease_key(conn: *mut ksmbd_conn, ci: *mut ksmbd_inode, lctx: *mut lease_ctx_info) -> i32;
    pub fn destroy_lease_table(conn: *mut ksmbd_conn);
    pub fn smb_send_parent_lease_break_noti(fp: *mut ksmbd_file, lctx: *mut lease_ctx_info);
    pub fn smb_lazy_parent_lease_break_close(fp: *mut ksmbd_file);
    pub fn smb2_check_durable_oplock(conn: *mut ksmbd_conn, share: *mut ksmbd_share_config,
                                     fp: *mut ksmbd_file, lctx: *mut lease_ctx_info,
                                     user: *mut ksmbd_user, name: *mut c_char) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
