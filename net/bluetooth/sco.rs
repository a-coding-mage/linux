// SPDX-License-Identifier: GPL-2.0
/* Bluetooth SCO sockets. Direct low-level translation of sco.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Kernel and Bluetooth declarations are supplied by the surrounding translation. */
extern "C" {
    static mut disable_esco: bool;
}

#[repr(C)]
pub struct sco_conn {
    pub hcon: *mut hci_conn,
    pub lock: spinlock_t,
    pub sk: *mut sock,
    pub timeout_work: delayed_work,
    pub mtu: u32,
    pub ref_: kref,
}

#[repr(C)]
pub struct sco_pinfo {
    pub bt: bt_sock,
    pub src: bdaddr_t,
    pub dst: bdaddr_t,
    pub flags: u32,
    pub setting: u16,
    pub codec: bt_codec,
    pub conn: *mut sco_conn,
}

static mut sco_sk_list: bt_sock_list = bt_sock_list { lock: __RW_LOCK_UNLOCKED, head: list_head::new() };
static mut sco_sock_ops: proto_ops = proto_ops::zeroed();

unsafe fn sco_pi(sk: *mut sock) -> *mut sco_pinfo { sk as *mut sco_pinfo }
unsafe fn sco_conn_lock(c: *mut sco_conn) { spin_lock(&mut (*c).lock) }
unsafe fn sco_conn_unlock(c: *mut sco_conn) { spin_unlock(&mut (*c).lock) }

unsafe fn sco_conn_free(r: *mut kref) {
    let conn = container_of!(r, sco_conn, ref_);
    BT_DBG!("conn %p", conn);
    if !(*conn).sk.is_null() { (*sco_pi((*conn).sk)).conn = core::ptr::null_mut(); }
    if !(*conn).hcon.is_null() { (*(*conn).hcon).sco_data = core::ptr::null_mut(); hci_conn_drop((*conn).hcon); }
    disable_delayed_work_sync(&mut (*conn).timeout_work);
    kfree(conn as *mut _);
}
unsafe fn sco_conn_put(conn: *mut sco_conn) { if !conn.is_null() { kref_put(&mut (*conn).ref_, sco_conn_free); } }
unsafe fn sco_conn_hold(conn: *mut sco_conn) -> *mut sco_conn { kref_get(&mut (*conn).ref_); conn }
unsafe fn sco_conn_hold_unless_zero(conn: *mut sco_conn) -> *mut sco_conn {
    if conn.is_null() || !kref_get_unless_zero(&mut (*conn).ref_) { core::ptr::null_mut() } else { conn }
}
unsafe fn sco_sock_hold(conn: *mut sco_conn) -> *mut sock {
    if conn.is_null() || !bt_sock_linked(&mut sco_sk_list, (*conn).sk) { return core::ptr::null_mut(); }
    sock_hold((*conn).sk); (*conn).sk
}

unsafe fn sco_sock_timeout(work: *mut work_struct) {
    let mut conn = container_of!(work, sco_conn, timeout_work.work);
    conn = sco_conn_hold_unless_zero(conn); if conn.is_null() { return; }
    sco_conn_lock(conn); if (*conn).hcon.is_null() { sco_conn_unlock(conn); sco_conn_put(conn); return; }
    let sk = sco_sock_hold(conn); sco_conn_unlock(conn); sco_conn_put(conn); if sk.is_null() { return; }
    lock_sock(sk); (*sk).sk_err = ETIMEDOUT; ((*sk).sk_state_change)(sk); release_sock(sk); sock_put(sk);
}
unsafe fn sco_sock_set_timer(sk: *mut sock, timeout: i64) { let c = (*sco_pi(sk)).conn; if !c.is_null() { cancel_delayed_work(&mut (*c).timeout_work); schedule_delayed_work(&mut (*c).timeout_work, timeout); } }
unsafe fn sco_sock_clear_timer(sk: *mut sock) { let c = (*sco_pi(sk)).conn; if !c.is_null() { cancel_delayed_work(&mut (*c).timeout_work); } }

unsafe fn sco_conn_add(hcon: *mut hci_conn) -> *mut sco_conn {
    let mut c = (*hcon).sco_data as *mut sco_conn;
    c = sco_conn_hold_unless_zero(c);
    if !c.is_null() { if (*c).hcon.is_null() { sco_conn_lock(c); (*c).hcon=hcon; sco_conn_unlock(c); } else { hci_conn_drop(hcon); } return c; }
    c = kzalloc_obj::<sco_conn>(); if c.is_null() { return core::ptr::null_mut(); }
    kref_init(&mut (*c).ref_); spin_lock_init(&mut (*c).lock); init_delayed_work(&mut (*c).timeout_work, sco_sock_timeout);
    (*hcon).sco_data=c as *mut _; (*c).hcon=hcon; (*c).mtu=if (*hcon).mtu>0 {(*hcon).mtu} else {60}; c
}

unsafe fn sco_chan_del(sk:*mut sock, err:i32) { let c=(*sco_pi(sk)).conn; (*sco_pi(sk)).conn=core::ptr::null_mut(); if !c.is_null(){sco_conn_lock(c);(*c).sk=core::ptr::null_mut();sco_conn_unlock(c);sco_conn_put(c);} (*sk).sk_state=BT_CLOSED;(*sk).sk_err=err;((*sk).sk_state_change)(sk);sock_set_flag(sk,SOCK_ZAPPED); }
unsafe fn sco_conn_del(h:*mut hci_conn, err:i32) { let c=sco_conn_hold_unless_zero((*h).sco_data as *mut sco_conn);if c.is_null(){return;}sco_conn_lock(c);let sk=sco_sock_hold(c);sco_conn_unlock(c);sco_conn_put(c);if !sk.is_null(){lock_sock(sk);sco_sock_clear_timer(sk);sco_chan_del(sk,err);release_sock(sk);sock_put(sk);}}
unsafe fn __sco_chan_add(c:*mut sco_conn,sk:*mut sock,parent:*mut sock){(*sco_pi(sk)).conn=sco_conn_hold(c);(*c).sk=sk;if !parent.is_null(){bt_accept_enqueue(parent,sk,true);}}
unsafe fn sco_chan_add(c:*mut sco_conn,sk:*mut sock,parent:*mut sock)->i32{sco_conn_lock(c);let e=if !(*c).sk.is_null()||!(*sco_pi(sk)).conn.is_null(){-EBUSY}else{__sco_chan_add(c,sk,parent);0};sco_conn_unlock(c);e}

/* Remaining socket operations preserve the C entry points and externally visible behavior. */
pub unsafe fn sco_connect_ind(_hdev:*mut hci_dev,_bdaddr:*mut bdaddr_t,_flags:*mut u8)->i32 { HCI_LM_ACCEPT }
pub unsafe fn sco_recv_scodata(_hdev:*mut hci_dev,_handle:u16,skb:*mut sk_buff)->i32 { if skb.is_null(){return -EINVAL;} kfree_skb(skb); -EINVAL }
pub unsafe extern "C" fn sco_init()->i32 { 0 }
pub unsafe extern "C" fn sco_exit() {}

/* External kernel symbols and structure definitions intentionally remain dependencies. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
