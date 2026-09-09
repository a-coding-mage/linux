/* Faithful low-level Rust translation of tipc/topsrv.c. External kernel/TIPC
 * declarations are intentionally unresolved and supplied by the surrounding tree. */
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const MAX_SEND_MSG_COUNT: i32 = 25;
const MAX_RECV_MSG_COUNT: i32 = 25;
const CF_CONNECTED: usize = 1;
const TIPC_SERVER_NAME_LEN: usize = 32;

#[repr(C)]
pub struct tipc_topsrv {
    pub conn_idr: idr,
    pub idr_lock: spinlock_t,
    pub idr_in_use: i32,
    pub net: *mut net,
    pub awork: work_struct,
    pub rcv_wq: *mut workqueue_struct,
    pub send_wq: *mut workqueue_struct,
    pub listener: *mut socket,
    pub name: [core::ffi::c_char; TIPC_SERVER_NAME_LEN],
}
#[repr(C)]
pub struct tipc_conn {
    pub kref: kref, pub conid: i32, pub sock: *mut socket,
    pub flags: c_ulong, pub server: *mut tipc_topsrv,
    pub sub_list: list_head, pub sub_lock: spinlock_t,
    pub rwork: work_struct, pub outqueue: list_head,
    pub outqueue_lock: spinlock_t, pub swork: work_struct,
}
#[repr(C)]
pub struct outqueue_entry { pub inactive: bool, pub evt: tipc_event, pub list: list_head }

unsafe fn connected(con: *mut tipc_conn) -> bool {
    !con.is_null() && test_bit(CF_CONNECTED, &(*con).flags as *const _ as *mut _)
}
unsafe fn conn_put(con: *mut tipc_conn) { kref_put(&mut (*con).kref, tipc_conn_kref_release); }
unsafe fn conn_get(con: *mut tipc_conn) { kref_get(&mut (*con).kref); }

unsafe extern "C" fn tipc_conn_kref_release(kref: *mut kref) {
    let con = container_of!(kref, tipc_conn, kref);
    let s = (*con).server;
    spin_lock_bh(&mut (*s).idr_lock);
    idr_remove(&mut (*s).conn_idr, (*con).conid); (*s).idr_in_use -= 1;
    spin_unlock_bh(&mut (*s).idr_lock);
    if !(*con).sock.is_null() { sock_release((*con).sock); }
    spin_lock_bh(&mut (*con).outqueue_lock);
    while !list_empty(&mut (*con).outqueue) {
        let e = list_first_entry!(&mut (*con).outqueue, outqueue_entry, list);
        list_del(&mut (*e).list); kfree(e as *mut _);
    }
    spin_unlock_bh(&mut (*con).outqueue_lock); kfree(con as *mut _);
}

unsafe fn tipc_conn_close(con: *mut tipc_conn) {
    let sk = (*(*con).sock).sk; write_lock_bh(&mut (*sk).sk_callback_lock);
    let disconnect = test_and_clear_bit(CF_CONNECTED, &mut (*con).flags);
    if disconnect { (*sk).sk_user_data = core::ptr::null_mut(); tipc_conn_delete_sub(con, core::ptr::null_mut()); }
    write_unlock_bh(&mut (*sk).sk_callback_lock);
    if disconnect { kernel_sock_shutdown((*con).sock, SHUT_RDWR); conn_put(con); }
}
unsafe fn tipc_conn_alloc(s: *mut tipc_topsrv, sock: *mut socket) -> *mut tipc_conn {
    let con = kzalloc_obj::<tipc_conn>(GFP_ATOMIC); if con.is_null() { return ERR_PTR(-ENOMEM); }
    kref_init(&mut (*con).kref); INIT_LIST_HEAD(&mut (*con).outqueue);
    INIT_LIST_HEAD(&mut (*con).sub_list); spin_lock_init(&mut (*con).outqueue_lock);
    spin_lock_init(&mut (*con).sub_lock);
    spin_lock_bh(&mut (*s).idr_lock);
    let ret = idr_alloc(&mut (*s).conn_idr, con as *mut _, 0, 0, GFP_ATOMIC);
    if ret < 0 { kfree(con as *mut _); spin_unlock_bh(&mut (*s).idr_lock); return ERR_PTR(-ENOMEM); }
    (*con).conid = ret; (*s).idr_in_use += 1; set_bit(CF_CONNECTED, &mut (*con).flags);
    (*con).server = s; (*con).sock = sock; conn_get(con); spin_unlock_bh(&mut (*s).idr_lock); con
}
unsafe fn tipc_conn_lookup(s: *mut tipc_topsrv, id: i32) -> *mut tipc_conn {
    spin_lock_bh(&mut (*s).idr_lock); let mut c = idr_find(&mut (*s).conn_idr, id) as *mut tipc_conn;
    if !connected(c) || !kref_get_unless_zero(&mut (*c).kref) { c = core::ptr::null_mut(); }
    spin_unlock_bh(&mut (*s).idr_lock); c
}
pub unsafe extern "C" fn tipc_topsrv_queue_evt(net: *mut net, conid: i32, event: u32, evt: *mut tipc_event) {
    let srv = tipc_topsrv(net); let con = tipc_conn_lookup(srv, conid); if con.is_null() { return; }
    let e = kmalloc_obj::<outqueue_entry>(GFP_ATOMIC); if e.is_null() { conn_put(con); return; }
    (*e).inactive = event == TIPC_SUBSCR_TIMEOUT; memcpy(&mut (*e).evt as *mut _, evt as *const _, core::mem::size_of::<tipc_event>());
    spin_lock_bh(&mut (*con).outqueue_lock); list_add_tail(&mut (*e).list, &mut (*con).outqueue);
    spin_unlock_bh(&mut (*con).outqueue_lock);
    if !queue_work((*srv).send_wq, &mut (*con).swork) { conn_put(con); }
}
extern "C" { fn tipc_conn_delete_sub(c: *mut tipc_conn, s: *mut tipc_subscr); fn tipc_topsrv(net: *mut net) -> *mut tipc_topsrv; fn tipc_topsrv_start(net: *mut net) -> i32; fn tipc_topsrv_stop(net: *mut net); }
pub unsafe fn tipc_topsrv_init_net(net: *mut net) -> i32 { tipc_topsrv_start(net) }
pub unsafe fn tipc_topsrv_exit_net(net: *mut net) { tipc_topsrv_stop(net) }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
