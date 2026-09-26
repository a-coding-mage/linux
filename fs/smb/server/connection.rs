// SPDX-License-Identifier: GPL-2.0-or-later
/* Translation of connection.c; kernel and project dependencies are external. */

use core::ffi::c_void;

#[repr(C)] pub struct ksmbd_conn { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_work { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_session { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_transport { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_transport_write { _private: [u8; 0] }
#[repr(C)] pub struct smbdirect_buffer_descriptor_v1 { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct ksmbd_conn_ops { pub process_fn: Option<unsafe extern "C" fn(*mut ksmbd_conn) -> i32>, pub terminate_fn: Option<unsafe extern "C" fn(*mut ksmbd_conn)> }

extern "C" {
    static mut default_conn_ops: ksmbd_conn_ops;
    static mut ksmbd_conn_wq: *mut c_void;
    fn ksmbd_conn_set_new(c: *mut ksmbd_conn); fn ksmbd_conn_set_exiting(c: *mut ksmbd_conn);
    fn ksmbd_conn_set_releasing(c: *mut ksmbd_conn); fn ksmbd_conn_exiting(c: *mut ksmbd_conn) -> bool;
    fn ksmbd_conn_releasing(c: *mut ksmbd_conn) -> bool;
    fn ksmbd_server_running() -> bool; fn kthread_should_stop() -> bool;
    fn ksmbd_max_allowed_pdu_size(c: *mut ksmbd_conn) -> u32;
    fn ksmbd_smb_request(c: *mut ksmbd_conn) -> bool; fn ksmbd_decompress_request(c: *mut ksmbd_conn) -> i32;
    fn ksmbd_preauth_session_destroy(c: *mut ksmbd_conn); fn release_async_work(w: *mut ksmbd_work);
    fn ksmbd_tcp_init() -> i32; fn ksmbd_rdma_init() -> i32; fn ksmbd_tcp_destroy(); fn ksmbd_rdma_stop_listening();
}

const EINVAL: i32 = 22; const EIO: i32 = 5;
const SMB2_CANCEL_HE: u32 = 0xffff; const SMB3_COMPRESS_NONE: u32 = 0;
const SMB2_CLIENT_GUID_SIZE: usize = 16; const MAX_STREAM_PROT_LEN: u32 = 0x00ff_ffff;
const SMB_ECHO_INTERVAL: u64 = 60;

pub unsafe fn ksmbd_conn_get(conn: *mut ksmbd_conn) -> *mut ksmbd_conn {
    if conn.is_null() { return core::ptr::null_mut(); }
    // atomic_inc(&conn->refcnt)
    extern_refcnt_inc(conn); conn
}
pub unsafe fn ksmbd_conn_put(conn: *mut ksmbd_conn) { if !conn.is_null() && extern_refcnt_dec_test(conn) { extern_queue_release(conn); } }
pub unsafe fn ksmbd_conn_free(conn: *mut ksmbd_conn) {
    extern_conn_list_del(conn); extern_xa_destroy(conn); extern_kvfree_request(conn);
    extern_kfree_sensitive_preauth(conn); extern_kfree_mechtoken(conn); ksmbd_preauth_session_destroy(conn); ksmbd_conn_put(conn);
}
pub unsafe fn ksmbd_conn_alloc() -> *mut ksmbd_conn {
    let c = extern_zalloc_conn(); if c.is_null() { return core::ptr::null_mut(); }
    extern_need_neg(c); ksmbd_conn_set_new(c); extern_load_nls(c); extern_load_unicode(c);
    extern_init_release_work(c); extern_atomic_init(c); extern_seq_init(c); extern_wait_init(c);
    extern_lists_init(c); extern_locks_init(c); extern_ida_init(c); extern_xa_init(c); c
}
pub unsafe fn ksmbd_conn_lookup_dialect(c: *mut ksmbd_conn) -> bool { extern_lookup_dialect(c) }
pub unsafe fn ksmbd_conn_enqueue_request(w: *mut ksmbd_work) { extern_enqueue(w); }
pub unsafe fn ksmbd_conn_try_dequeue_request(w: *mut ksmbd_work) { extern_dequeue(w); }
pub unsafe fn ksmbd_conn_lock(c: *mut ksmbd_conn) { extern_mutex_lock(c); }
pub unsafe fn ksmbd_conn_unlock(c: *mut ksmbd_conn) { extern_mutex_unlock(c); }
pub unsafe fn ksmbd_all_conn_set_status(s: *mut ksmbd_session, status: u32) { extern_set_all_status(s,status); }
pub unsafe fn ksmbd_conn_abort(c: *mut ksmbd_conn) { extern_abort(c); }
pub unsafe fn ksmbd_conn_wait_idle(c: *mut ksmbd_conn) { extern_wait_idle(c); }
pub unsafe fn ksmbd_conn_wait_idle_sess(c: *mut ksmbd_conn, s: *mut ksmbd_session) -> i32 { extern_wait_idle_sess(c,s) }
pub unsafe fn ksmbd_conn_write(w: *mut ksmbd_work) -> i32 { extern_write(w,0) }
pub unsafe fn ksmbd_conn_write_eor(w: *mut ksmbd_work) -> i32 { extern_write(w,1) }
pub unsafe fn ksmbd_conn_rdma_read(c:*mut ksmbd_conn,b:*mut c_void,n:u32,d:*mut smbdirect_buffer_descriptor_v1,l:u32)->i32 { extern_rdma_read(c,b,n,d,l) }
pub unsafe fn ksmbd_conn_rdma_write(c:*mut ksmbd_conn,b:*mut c_void,n:u32,d:*mut smbdirect_buffer_descriptor_v1,l:u32)->i32 { extern_rdma_write(c,b,n,d,l) }
pub unsafe fn ksmbd_conn_alive(c:*mut ksmbd_conn)->bool { extern_alive(c) }
pub unsafe fn ksmbd_conn_handler_loop(p:*mut c_void)->i32 { extern_handler_loop(p) }
pub unsafe fn ksmbd_conn_init_server_callbacks(ops:*mut ksmbd_conn_ops) { default_conn_ops.process_fn=(*ops).process_fn; default_conn_ops.terminate_fn=(*ops).terminate_fn; }
pub unsafe fn ksmbd_conn_r_count_inc(c:*mut ksmbd_conn){ extern_rcount_inc(c); }
pub unsafe fn ksmbd_conn_r_count_dec(c:*mut ksmbd_conn){ extern_rcount_dec(c); }
pub unsafe fn ksmbd_conn_transport_init()->i32 { let r=ksmbd_tcp_init(); if r!=0{return r} ksmbd_rdma_init() }
pub unsafe fn ksmbd_conn_transport_destroy(){ extern_delete_proc(); ksmbd_tcp_destroy(); ksmbd_rdma_stop_listening(); extern_stop_sessions(); }

// File-local mappings for kernel primitives and structures supplied by other translation units.
extern "C" { fn extern_refcnt_inc(_: *mut ksmbd_conn); fn extern_refcnt_dec_test(_: *mut ksmbd_conn)->bool; fn extern_queue_release(_: *mut ksmbd_conn); fn extern_conn_list_del(_: *mut ksmbd_conn); fn extern_xa_destroy(_: *mut ksmbd_conn); fn extern_kvfree_request(_: *mut ksmbd_conn); fn extern_kfree_sensitive_preauth(_: *mut ksmbd_conn); fn extern_kfree_mechtoken(_: *mut ksmbd_conn); fn extern_zalloc_conn()->*mut ksmbd_conn; fn extern_need_neg(_: *mut ksmbd_conn); fn extern_load_nls(_: *mut ksmbd_conn); fn extern_load_unicode(_: *mut ksmbd_conn); fn extern_init_release_work(_: *mut ksmbd_conn); fn extern_atomic_init(_: *mut ksmbd_conn); fn extern_seq_init(_: *mut ksmbd_conn); fn extern_wait_init(_: *mut ksmbd_conn); fn extern_lists_init(_: *mut ksmbd_conn); fn extern_locks_init(_: *mut ksmbd_conn); fn extern_ida_init(_: *mut ksmbd_conn); fn extern_xa_init(_: *mut ksmbd_conn); fn extern_lookup_dialect(_: *mut ksmbd_conn)->bool; fn extern_enqueue(_: *mut ksmbd_work); fn extern_dequeue(_: *mut ksmbd_work); fn extern_mutex_lock(_: *mut ksmbd_conn); fn extern_mutex_unlock(_: *mut ksmbd_conn); fn extern_set_all_status(_: *mut ksmbd_session,_: u32); fn extern_abort(_: *mut ksmbd_conn); fn extern_wait_idle(_: *mut ksmbd_conn); fn extern_wait_idle_sess(_: *mut ksmbd_conn,_: *mut ksmbd_session)->i32; fn extern_write(_: *mut ksmbd_work,_: i32)->i32; fn extern_rdma_read(_: *mut ksmbd_conn,_: *mut c_void,_: u32,_: *mut smbdirect_buffer_descriptor_v1,_: u32)->i32; fn extern_rdma_write(_: *mut ksmbd_conn,_: *mut c_void,_: u32,_: *mut smbdirect_buffer_descriptor_v1,_: u32)->i32; fn extern_alive(_: *mut ksmbd_conn)->bool; fn extern_handler_loop(_: *mut c_void)->i32; fn extern_rcount_inc(_: *mut ksmbd_conn); fn extern_rcount_dec(_: *mut ksmbd_conn); fn extern_delete_proc(); fn extern_stop_sessions(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
