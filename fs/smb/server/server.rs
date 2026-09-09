// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2016 Namjae Jeon <linkinjeon@kernel.org>
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// C dependencies supplied by the surrounding kernel/module translation.

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static mut ksmbd_debug_types: c_int;
    static mut server_conf: ksmbd_server_config;
}

#[repr(C)] pub struct ksmbd_server_config { pub conf: [*mut c_char; 4], pub state: c_int, pub enforced_signing: c_int, pub min_protocol: c_uint, pub max_protocol: c_uint, pub auth_mechs: c_uint, pub max_inflight_req: c_uint, pub tcp_port: c_uint, pub ipc_last_active: usize }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_work { pub work: work_struct, pub conn: *mut ksmbd_conn, pub request_buf: *mut c_void, pub response_buf: *mut c_void, pub sess: *mut ksmbd_session, pub tcon: *mut ksmbd_tree_connect, pub encrypted: bool, pub send_no_response: bool, pub credit_charge: u32, pub compress_response: bool }
#[repr(C)] pub struct ksmbd_conn { pub ops: *mut ksmbd_conn_ops, pub max_cmds: u16, pub cmds: *mut smb_version_cmds, pub request_buf: *mut c_void, pub credits_lock: c_void, pub outstanding_credits: u32, pub last_active: usize, pub dialect: u16, pub stats: ksmbd_stats }
#[repr(C)] pub struct ksmbd_stats { pub request_served: c_void }
#[repr(C)] pub struct ksmbd_session { pub sign: bool, pub enc: bool, pub dialect: u16 }
#[repr(C)] pub struct ksmbd_tree_connect { pub share_conf: *mut c_void }
#[repr(C)] pub struct smb_version_cmds { pub proc: Option<unsafe extern "C" fn(*mut ksmbd_work) -> c_int> }
#[repr(C)] pub struct smb_version_ops { pub set_rsp_status: Option<unsafe extern "C" fn(*mut ksmbd_work, c_uint)>, pub get_cmd_val: Option<unsafe extern "C" fn(*mut ksmbd_work) -> u16>, pub is_sign_req: Option<unsafe extern "C" fn(*mut ksmbd_work, u16) -> bool>, pub check_sign_req: Option<unsafe extern "C" fn(*mut ksmbd_work) -> c_int>, pub inc_reqs: Option<unsafe extern "C" fn(u16, c_void)>, pub decrypt_req: Option<unsafe extern "C" fn(*mut ksmbd_work) -> c_int>, pub allocate_rsp_buf: Option<unsafe extern "C" fn(*mut ksmbd_work) -> c_int>, pub init_rsp_hdr: Option<unsafe extern "C" fn(*mut ksmbd_work) -> c_int>, pub check_user_session: Option<unsafe extern "C" fn(*mut ksmbd_work) -> c_int>, pub get_ksmbd_tcon: Option<unsafe extern "C" fn(*mut ksmbd_work) -> c_int>, pub set_rsp_credits: Option<unsafe extern "C" fn(*mut ksmbd_work) -> c_int>, pub set_sign_rsp: Option<unsafe extern "C" fn(*mut ksmbd_work)>, pub encrypt_resp: Option<unsafe extern "C" fn(*mut ksmbd_work) -> c_int> }
#[repr(C)] pub struct ksmbd_conn_ops { pub process_fn: Option<unsafe extern "C" fn(*mut ksmbd_conn) -> c_int>, pub terminate_fn: Option<unsafe extern "C" fn(*mut ksmbd_conn) -> c_int> }
#[repr(C)] pub struct smb_hdr { pub Status: smb_status }
#[repr(C)] pub struct smb2_hdr { pub ProtocolId: u32, pub Flags: u32, pub Status: c_void }
#[repr(C)] pub struct smb_status { pub CifsError: u32 }
#[repr(C)] pub struct class { pub name: *const c_char, pub class_groups: *const *const c_void }
#[repr(C)] pub struct class_attribute { _private: [u8; 0] }
#[repr(C)] pub struct server_ctrl_struct { pub type_: c_int, pub ctrl_work: work_struct }

const SERVER_CTRL_TYPE_INIT: c_int = 0;
const SERVER_CTRL_TYPE_RESET: c_int = 1;
const SERVER_HANDLER_CONTINUE: c_int = 0;
const SERVER_HANDLER_ABORT: c_int = 1;
const SERVER_CONF_NETBIOS_NAME: usize = 0;
const SERVER_CONF_SERVER_STRING: usize = 1;
const SERVER_CONF_WORK_GROUP: usize = 2;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EKEYEXPIRED: c_int = 127;

extern "C" {
    fn kfree(p: *mut c_void); fn kstrdup(p: *const c_char, flags: c_uint) -> *mut c_char;
    fn ksmbd_conn_exiting(c: *mut ksmbd_conn) -> bool; fn ksmbd_conn_need_reconnect(c: *mut ksmbd_conn) -> bool;
    fn smb_get_msg(p: *mut c_void) -> *mut smb_hdr; fn ksmbd_verify_smb_message(w: *mut ksmbd_work) -> c_int;
    fn ksmbd_resp_buf_curr(w: *mut ksmbd_work) -> *mut smb2_hdr; fn ksmbd_conn_abort(c: *mut ksmbd_conn);
    fn ksmbd_decompress_work_request(w: *mut ksmbd_work) -> c_int; fn get_rfc1002_len(w: *mut c_void) -> usize;
    fn ksmbd_compress_response(w: *mut ksmbd_work) -> c_int; fn smb2_complete_request_open(w: *mut ksmbd_work);
    fn is_chained_smb2_message(w: *mut ksmbd_work) -> bool; fn smb3_11_final_sess_setup_resp(w: *mut ksmbd_work) -> bool;
    fn smb3_set_sign_rsp(w: *mut ksmbd_work); fn smb3_preauth_hash_rsp(w: *mut ksmbd_work);
    fn ksmbd_tree_connect_put(t: *mut ksmbd_tree_connect); fn ksmbd_user_session_put(s: *mut ksmbd_session);
    fn ksmbd_conn_write(w: *mut ksmbd_work); fn ksmbd_conn_try_dequeue_request(w: *mut ksmbd_work);
    fn ksmbd_free_work_struct(w: *mut ksmbd_work); fn ksmbd_conn_r_count_dec(c: *mut ksmbd_conn);
    fn ksmbd_init_smb_server(c: *mut ksmbd_conn) -> c_int; fn ksmbd_alloc_work_struct() -> *mut ksmbd_work;
    fn ksmbd_conn_enqueue_request(w: *mut ksmbd_work); fn ksmbd_conn_r_count_inc(c: *mut ksmbd_conn);
    fn ksmbd_queue_work(w: *mut ksmbd_work); fn ksmbd_sessions_deregister(c: *mut ksmbd_conn);
    fn destroy_lease_table(c: *mut ksmbd_conn); fn ksmbd_conn_init_server_callbacks(o: *mut ksmbd_conn_ops);
    fn ksmbd_min_protocol() -> c_uint; fn ksmbd_max_protocol() -> c_uint; fn ksmbd_proc_reset();
    fn ksmbd_conn_transport_init() -> c_int; fn server_queue_ctrl_reset_work() -> c_int; fn ksmbd_ipc_soft_reset();
    fn ksmbd_conn_transport_destroy(); fn ksmbd_stop_durable_scavenger(); fn ksmbd_ipc_release();
    fn ksmbd_workqueue_destroy(); fn ksmbd_proc_cleanup(); fn ksmbd_crypto_destroy(); fn ksmbd_free_global_file_table();
    fn ksmbd_work_pool_destroy(); fn ksmbd_exit_file_cache(); fn ksmbd_proc_init() -> c_int; fn create_proc_sessions() -> c_int;
    fn create_proc_shares() -> c_int; fn ksmbd_work_pool_init() -> c_int; fn ksmbd_init_file_cache() -> c_int;
    fn ksmbd_ipc_init() -> c_int; fn ksmbd_init_global_file_table() -> c_int; fn ksmbd_inode_hash_init() -> c_int;
    fn ksmbd_release_inode_hash(); fn ksmbd_crypto_create() -> c_int; fn ksmbd_workqueue_init() -> c_int;
    fn ksmbd_conn_wq_init() -> c_int; fn ksmbd_conn_wq_destroy(); fn rcu_barrier();
    fn class_register(c: *mut class) -> c_int; fn class_unregister(c: *mut class);
}

pub unsafe fn ___server_conf_set(idx: usize, val: *mut c_char) -> c_int { if idx >= server_conf.conf.len() || val.is_null() || *val == 0 { return -EINVAL; } kfree(server_conf.conf[idx] as *mut c_void); server_conf.conf[idx] = kstrdup(val, 0); if server_conf.conf[idx].is_null() { return -ENOMEM; } 0 }
pub unsafe fn ksmbd_set_netbios_name(v: *mut c_char) -> c_int { ___server_conf_set(SERVER_CONF_NETBIOS_NAME, v) }
pub unsafe fn ksmbd_set_server_string(v: *mut c_char) -> c_int { ___server_conf_set(SERVER_CONF_SERVER_STRING, v) }
pub unsafe fn ksmbd_set_work_group(v: *mut c_char) -> c_int { ___server_conf_set(SERVER_CONF_WORK_GROUP, v) }
pub unsafe fn ksmbd_netbios_name() -> *mut c_char { server_conf.conf[SERVER_CONF_NETBIOS_NAME] }
pub unsafe fn ksmbd_server_string() -> *mut c_char { server_conf.conf[SERVER_CONF_SERVER_STRING] }
pub unsafe fn ksmbd_work_group() -> *mut c_char { server_conf.conf[SERVER_CONF_WORK_GROUP] }

// check_conn_state() - check state of server thread connection
unsafe fn check_conn_state(work: *mut ksmbd_work) -> c_int { if ksmbd_conn_exiting((*work).conn) || ksmbd_conn_need_reconnect((*work).conn) { (*smb_get_msg((*work).response_buf)).Status.CifsError = 0; return 1; } 0 }

unsafe fn __process_request(work: *mut ksmbd_work, conn: *mut ksmbd_conn, cmd: *mut u16) -> c_int {
    if check_conn_state(work) != 0 { return SERVER_HANDLER_CONTINUE; }
    let ops = &*(*conn).ops; let command = (ops.get_cmd_val.unwrap())(work); *cmd = command;
    if command >= (*conn).max_cmds { (ops.set_rsp_status.unwrap())(work, 0); return SERVER_HANDLER_ABORT; }
    let cmds = &*(*conn).cmds.add(command as usize); if cmds.proc.is_none() { (ops.set_rsp_status.unwrap())(work, 0); return SERVER_HANDLER_ABORT; }
    let signed_req = ops.is_sign_req.map(|f| f(work, command)).unwrap_or(false);
    if !(*work).sess.is_null() && (*(*work).sess).sign && !(*work).encrypted && !signed_req { (ops.set_rsp_status.unwrap())(work, 0); return SERVER_HANDLER_ABORT; }
    if !(*work).sess.is_null() && signed_req && (ops.check_sign_req.unwrap())(work) == 0 { (ops.set_rsp_status.unwrap())(work, 0); return SERVER_HANDLER_ABORT; }
    let ret = (cmds.proc.unwrap())(work); if ret > 0 { *cmd = ret as u16; return __process_request(work, conn, cmd); } if (*work).send_no_response { SERVER_HANDLER_ABORT } else { SERVER_HANDLER_CONTINUE }
}

unsafe fn __handle_ksmbd_work(work: *mut ksmbd_work, conn: *mut ksmbd_conn) { let mut command: u16 = 0; let rc = __process_request(work, conn, &mut command); if rc == SERVER_HANDLER_ABORT { smb2_complete_request_open(work); } smb2_complete_request_open(work); if (*work).credit_charge != 0 { (*conn).outstanding_credits = (*conn).outstanding_credits.wrapping_sub((*work).credit_charge); (*work).credit_charge = 0; } if !(*work).tcon.is_null() { ksmbd_tree_connect_put((*work).tcon); } smb3_preauth_hash_rsp(work); if (*work).compress_response { let _ = ksmbd_compress_response(work); } if !(*work).sess.is_null() { ksmbd_user_session_put((*work).sess); } ksmbd_conn_write(work); let _ = rc; }
unsafe fn handle_ksmbd_work(wk: *mut work_struct) { let work = wk as *mut ksmbd_work; let conn = (*work).conn; __handle_ksmbd_work(work, conn); ksmbd_conn_try_dequeue_request(work); ksmbd_free_work_struct(work); ksmbd_conn_r_count_dec(conn); }

unsafe fn queue_ksmbd_work(conn: *mut ksmbd_conn) -> c_int { let err = ksmbd_init_smb_server(conn); if err != 0 { return 0; } let work = ksmbd_alloc_work_struct(); if work.is_null() { return -ENOMEM; } (*work).conn = conn; (*work).request_buf = (*conn).request_buf; (*conn).request_buf = core::ptr::null_mut(); ksmbd_conn_enqueue_request(work); ksmbd_conn_r_count_inc(conn); ksmbd_queue_work(work); 0 }
unsafe fn ksmbd_server_process_request(c: *mut ksmbd_conn) -> c_int { queue_ksmbd_work(c) }
unsafe fn ksmbd_server_terminate_conn(c: *mut ksmbd_conn) -> c_int { ksmbd_sessions_deregister(c); destroy_lease_table(c); 0 }
unsafe fn ksmbd_server_tcp_callbacks_init() { let mut ops = ksmbd_conn_ops { process_fn: Some(ksmbd_server_process_request), terminate_fn: Some(ksmbd_server_terminate_conn) }; ksmbd_conn_init_server_callbacks(&mut ops); }
unsafe fn server_conf_free() { for p in server_conf.conf.iter_mut() { kfree(*p as *mut c_void); *p = core::ptr::null_mut(); } }
unsafe fn server_conf_init() -> c_int { server_conf.state = 0; server_conf.enforced_signing = 0; server_conf.min_protocol = ksmbd_min_protocol(); server_conf.max_protocol = ksmbd_max_protocol(); server_conf.auth_mechs = 1; server_conf.max_inflight_req = 65535; 0 }

pub unsafe fn server_queue_ctrl_init_work() -> c_int { ksmbd_proc_reset(); 0 }
pub unsafe fn server_queue_ctrl_reset_work_public() -> c_int { server_conf_free(); server_conf_init() }

pub unsafe fn ksmbd_server_shutdown() -> c_int { server_conf.state = 3; ksmbd_workqueue_destroy(); ksmbd_ipc_release(); ksmbd_conn_transport_destroy(); ksmbd_proc_cleanup(); ksmbd_crypto_destroy(); ksmbd_free_global_file_table(); destroy_lease_table(core::ptr::null_mut()); ksmbd_work_pool_destroy(); ksmbd_exit_file_cache(); server_conf_free(); 0 }
pub unsafe fn ksmbd_server_init() -> c_int { let mut ret = server_conf_init(); if ret != 0 { return ret; } ksmbd_server_tcp_callbacks_init(); ret = ksmbd_work_pool_init(); if ret != 0 { return ret; } ret = ksmbd_init_file_cache(); if ret != 0 { return ret; } ret = ksmbd_ipc_init(); if ret != 0 { return ret; } ret = ksmbd_init_global_file_table(); if ret != 0 { return ret; } ret = ksmbd_inode_hash_init(); if ret != 0 { return ret; } ret = ksmbd_crypto_create(); if ret != 0 { return ret; } ret = ksmbd_workqueue_init(); if ret != 0 { return ret; } ksmbd_conn_wq_init() }
pub unsafe fn ksmbd_server_exit() { ksmbd_server_shutdown(); rcu_barrier(); ksmbd_conn_wq_destroy(); ksmbd_release_inode_hash(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
