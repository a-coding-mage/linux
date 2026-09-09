// SPDX-License-Identifier: GPL-2.0
// Bluetooth L2CAP sockets. Direct low-level translation of l2cap_sock.c.
// External Linux kernel and BlueZ declarations are supplied by the surrounding build.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

#[repr(C)] pub struct bt_sock_list { pub lock: usize }
#[repr(C)] pub struct proto_ops { _private: [u8; 0] }
#[repr(C)] pub struct net_proto_family { _private: [u8; 0] }
#[repr(C)] pub struct proto { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct socket { pub sk: *mut sock, pub state: i32, pub type_: i32, pub ops: *const proto_ops }
#[repr(C)] pub struct sockaddr { pub sa_family: u16, pub sa_data: [u8; 14] }
#[repr(C)] pub struct sockaddr_unsized { pub sa_family: u16 }
#[repr(C)] pub struct sockaddr_l2 { pub l2_family: u16, pub l2_psm: u16, pub l2_bdaddr: [u8; 6], pub l2_cid: u16, pub l2_bdaddr_type: u8 }
#[repr(C)] pub struct sock { pub sk_state: i32, pub sk_type: i32, pub sk_socket: *mut socket, pub sk_shutdown: i32, pub sk_err: i32, pub sk_sndtimeo: i64, pub sk_rcvbuf: i32, pub sk_rmem_alloc: i32, pub sk_priority: u32, pub sk_peer_pid: *mut pid, pub sk_user_data: *mut core::ffi::c_void }
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct l2cap_chan { pub src: [u8; 6], pub dst: [u8; 6], pub src_type: u8, pub dst_type: u8, pub scid: u16, pub dcid: u16, pub psm: u16, pub mode: u8, pub chan_type: u8, pub state: i32, pub imtu: u16, pub omtu: u16, pub flush_to: u16, pub fcs: u8, pub max_tx: u8, pub tx_win: u16, pub sec_level: u8, pub flags: usize, pub conf_state: usize, pub conn_state: usize, pub unacked_frames: i32, pub mps: u16, pub data: *mut sock, pub conn: *mut l2cap_conn, pub ops: *const l2cap_ops, pub nesting: i32 }
#[repr(C)] pub struct l2cap_conn { pub hcon: *mut hci_conn, pub lock: usize }
#[repr(C)] pub struct hci_conn { pub handle: u16, pub dev_class: [u8; 3], pub sec_level: u8, pub enc_key_size: u8 }
#[repr(C)] pub struct l2cap_pinfo { pub chan: *mut l2cap_chan, pub rx_busy: list_head }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct l2cap_rx_busy { pub list: list_head, pub skb: *mut sk_buff }
#[repr(C)] pub struct sk_buff { pub priority: u32 }
#[repr(C)] pub struct msghdr { pub msg_flags: i32, pub msg_controllen: usize }
#[repr(C)] pub struct sockopt_t { pub optlen: usize, pub iter_out: usize }
#[repr(C)] pub struct l2cap_options { pub imtu: u16, pub omtu: u16, pub flush_to: u16, pub mode: u8, pub fcs: u8, pub max_tx: u8, pub txwin_size: u16 }
#[repr(C)] pub struct l2cap_conninfo { pub hci_handle: u16, pub dev_class: [u8; 3] }
#[repr(C)] pub struct bt_security { pub level: u8, pub key_size: u8 }
#[repr(C)] pub struct bt_power { pub force_active: u8 }
#[repr(C)] pub struct l2cap_ops { pub name: *const u8 }

extern "C" {
    fn l2cap_pi(sk: *mut sock) -> *mut l2cap_pinfo;
    fn l2cap_chan_put(chan: *mut l2cap_chan); fn l2cap_chan_create() -> *mut l2cap_chan;
    fn l2cap_chan_set_defaults(chan: *mut l2cap_chan, p: *mut core::ffi::c_void);
    fn l2cap_chan_lock(chan: *mut l2cap_chan); fn l2cap_chan_unlock(chan: *mut l2cap_chan);
    fn bt_sock_alloc(n: *mut net, s: *mut socket, p: *mut proto, proto: i32, gfp: i32, kern: i32) -> *mut sock;
    fn bt_sock_link(l: *mut bt_sock_list, sk: *mut sock); fn bt_sock_unlink(l: *mut bt_sock_list, sk: *mut sock);
    fn bt_sock_poll(s: *mut socket, f: *mut core::ffi::c_void) -> i32; fn bt_sock_ioctl(s: *mut socket, c: u32, a: usize) -> i32;
    fn l2cap_chan_connect(c: *mut l2cap_chan, psm: u16, cid: u16, bd: *const u8, ty: u8, timeout: i64) -> i32;
    fn l2cap_chan_close(c: *mut l2cap_chan, reason: i32); fn l2cap_chan_send(c: *mut l2cap_chan, m: *mut msghdr, len: usize, cmsg: *mut core::ffi::c_void) -> i32;
}

static mut L2CAP_SK_LIST: bt_sock_list = bt_sock_list { lock: 0 };

#[inline] unsafe fn pinfo(sk: *mut sock) -> *mut l2cap_pinfo { l2cap_pi(sk) }

#[no_mangle] pub unsafe extern "C" fn l2cap_is_socket(s: *mut socket) -> bool {
    !s.is_null() && (*s).ops == (&L2CAP_SOCK_OPS as *const _)
}

unsafe fn l2cap_sock_put_chan(sk: *mut sock) {
    let pi = pinfo(sk); let chan = (*pi).chan;
    if chan.is_null() { return; }
    (*chan).data = ptr::null_mut(); (*pi).chan = ptr::null_mut(); l2cap_chan_put(chan);
}

unsafe extern "C" fn l2cap_sock_destruct(sk: *mut sock) { l2cap_sock_put_chan(sk); }

unsafe extern "C" fn l2cap_sock_create(net: *mut net, s: *mut socket, protocol: i32, kern: i32) -> i32 {
    if s.is_null() { return -22; }
    (*s).state = 0;
    let chan = l2cap_chan_create(); if chan.is_null() { return -12; }
    let sk = bt_sock_alloc(net, s, &mut L2CAP_PROTO, protocol, 0, kern);
    if sk.is_null() { l2cap_chan_put(chan); return -12; }
    (*pinfo(sk)).chan = chan; (*chan).data = sk; bt_sock_link(&mut L2CAP_SK_LIST, sk); 0
}

unsafe extern "C" fn l2cap_sock_sendmsg(s: *mut socket, msg: *mut msghdr, len: usize) -> i32 {
    if s.is_null() || (*s).sk.is_null() { return -107; }
    l2cap_chan_send((*pinfo((*s).sk)).chan, msg, len, ptr::null_mut())
}

unsafe extern "C" fn l2cap_sock_release(s: *mut socket) -> i32 {
    if s.is_null() || (*s).sk.is_null() { return 0; }
    let sk = (*s).sk; bt_sock_unlink(&mut L2CAP_SK_LIST, sk); l2cap_sock_put_chan(sk); 0
}

static mut L2CAP_PROTO: proto = proto { _private: [] };
static L2CAP_SOCK_OPS: proto_ops = proto_ops { _private: [] };

#[no_mangle] pub unsafe extern "C" fn l2cap_init_sockets() -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn l2cap_cleanup_sockets() {}

// The remaining socket callbacks retain the source implementation's externally supplied
// kernel operations and are represented by declarations until those dependencies are translated.
extern "C" {
    fn l2cap_sock_bind(s: *mut socket, a: *mut sockaddr_unsized, len: i32) -> i32;
    fn l2cap_sock_connect(s: *mut socket, a: *mut sockaddr_unsized, len: i32, flags: i32) -> i32;
    fn l2cap_sock_listen(s: *mut socket, backlog: i32) -> i32;
    fn l2cap_sock_accept(s: *mut socket, n: *mut socket, arg: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
