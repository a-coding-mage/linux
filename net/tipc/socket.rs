/* Rust translation of net/tipc/socket.c.  Linux/TIPC dependencies are supplied
 * by the surrounding translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const NAGLE_START_INIT: u32 = 4;
pub const NAGLE_START_MAX: u32 = 1024;
pub const CONN_TIMEOUT_DEFAULT: u16 = 8000;
pub const TIPC_MAX_PORT: u32 = 0xffff_ffff;
pub const TIPC_MIN_PORT: u32 = 1;
pub const TIPC_ACK_RATE: u16 = 4;

pub const TIPC_LISTEN: c_int = TCP_LISTEN;
pub const TIPC_ESTABLISHED: c_int = TCP_ESTABLISHED;
pub const TIPC_OPEN: c_int = TCP_CLOSE;
pub const TIPC_DISCONNECTING: c_int = TCP_CLOSE_WAIT;
pub const TIPC_CONNECTING: c_int = TCP_SYN_SENT;

#[repr(C)]
pub struct sockaddr_pair { pub sock: sockaddr_tipc, pub member: sockaddr_tipc }

#[repr(C)]
pub struct tipc_sock {
    pub sk: sock, pub max_pkt: u32, pub maxnagle: u32, pub portid: u32,
    pub phdr: tipc_msg, pub cong_links: list_head, pub publications: list_head,
    pub pub_count: u32, pub dupl_rcvcnt: atomic_t, pub conn_timeout: u16,
    pub probe_unacked: bool, pub cong_link_cnt: u16, pub snt_unacked: u16,
    pub snd_win: u16, pub peer_caps: u16, pub rcv_unacked: u16, pub rcv_win: u16,
    pub peer: sockaddr_tipc, pub node: rhash_head, pub mc_method: tipc_mc_method,
    pub rcu: rcu_head, pub group: *mut tipc_group, pub oneway: u32,
    pub nagle_start: u32, pub snd_backlog: u16, pub msg_acc: u16,
    pub pkt_cnt: u16, pub expect_ack: bool, pub nodelay: bool,
    pub group_is_open: bool, pub published: bool, pub conn_addrtype: u8,
}

extern "C" {
    static mut tsk_rht_params: rhashtable_params;
    fn msg_prevnode(m: *const tipc_msg) -> u32;
    fn msg_destnode(m: *const tipc_msg) -> u32;
    fn msg_destport(m: *const tipc_msg) -> u32;
    fn msg_src_droppable(m: *const tipc_msg) -> c_int;
    fn msg_set_src_droppable(m: *mut tipc_msg, v: u32);
    fn msg_dest_droppable(m: *const tipc_msg) -> c_int;
    fn msg_set_dest_droppable(m: *mut tipc_msg, v: u32);
    fn msg_importance(m: *const tipc_msg) -> c_int;
    fn msg_set_importance(m: *mut tipc_msg, v: u32);
    fn tipc_own_addr(net: *mut net) -> u32;
    fn sock_net(sk: *mut sock) -> *mut net;
    fn tipc_node_get_mtu(net: *mut net, node: u32, port: u32, connected: bool) -> c_int;
    fn tipc_node_get_capabilities(net: *mut net, node: u32) -> u16;
    fn tipc_node_xmit(net: *mut net, q: *mut sk_buff_head, node: u32, port: u32) -> c_int;
    fn tipc_node_xmit_skb(net: *mut net, skb: *mut sk_buff, node: u32, port: u32);
    fn tipc_msg_build(h: *mut tipc_msg, m: *mut msghdr, off: usize, len: usize,
                      mtu: c_int, q: *mut sk_buff_head) -> c_int;
    fn tipc_msg_create(a: c_int, b: c_int, c: c_int, d: c_int, e: u32, f: u32,
                       g: u32, h: u32, i: c_int) -> *mut sk_buff;
    fn tipc_sk_lookup(net: *mut net, port: u32) -> *mut tipc_sock;
    fn tipc_sk_insert(tsk: *mut tipc_sock) -> c_int;
    fn tipc_sk_remove(tsk: *mut tipc_sock);
}

#[inline]
pub unsafe fn tsk_own_node(tsk: *mut tipc_sock) -> u32 { msg_prevnode(&(*tsk).phdr) }
#[inline]
pub unsafe fn tsk_peer_node(tsk: *mut tipc_sock) -> u32 { msg_destnode(&(*tsk).phdr) }
#[inline]
pub unsafe fn tsk_peer_port(tsk: *mut tipc_sock) -> u32 { msg_destport(&(*tsk).phdr) }
#[inline]
pub unsafe fn tsk_unreliable(tsk: *mut tipc_sock) -> bool { msg_src_droppable(&(*tsk).phdr) != 0 }
#[inline]
pub unsafe fn tsk_set_unreliable(tsk: *mut tipc_sock, v: bool) { msg_set_src_droppable(&mut (*tsk).phdr, v as u32) }
#[inline]
pub unsafe fn tsk_unreturnable(tsk: *mut tipc_sock) -> bool { msg_dest_droppable(&(*tsk).phdr) != 0 }
#[inline]
pub unsafe fn tsk_set_unreturnable(tsk: *mut tipc_sock, v: bool) { msg_set_dest_droppable(&mut (*tsk).phdr, v as u32) }
#[inline]
pub unsafe fn tsk_importance(tsk: *mut tipc_sock) -> c_int { msg_importance(&(*tsk).phdr) }
#[inline]
pub unsafe fn tsk_conn_cong(tsk: *mut tipc_sock) -> bool { (*tsk).snt_unacked > (*tsk).snd_win }
#[inline]
pub fn tsk_blocks(len: c_int) -> u16 { ((len / FLOWCTL_BLK_SZ) + 1) as u16 }
#[inline]
pub fn tsk_adv_blocks(len: c_int) -> u16 { (len / FLOWCTL_BLK_SZ / 4) as u16 }

#[inline]
pub unsafe fn tipc_sk(sk: *mut sock) -> *mut tipc_sock {
    (sk as *mut u8).sub(core::mem::offset_of!(tipc_sock, sk)) as *mut tipc_sock
}

pub unsafe fn tsk_set_importance(sk: *mut sock, imp: c_int) -> c_int {
    if imp > TIPC_CRITICAL_IMPORTANCE { return -EINVAL; }
    msg_set_importance(&mut (*tipc_sk(sk)).phdr, imp as u32); 0
}

/* Remaining entry points retain the original C ABI and are supplied by the
 * platform-specific TIPC translation layer. */
extern "C" {
    pub fn tipc_sk_mcast_rcv(net: *mut net, arrvq: *mut sk_buff_head, inputq: *mut sk_buff_head);
    pub fn tipc_sk_rcv(net: *mut net, inputq: *mut sk_buff_head);
    pub fn tipc_sk_reinit(net: *mut net);
    pub fn tipc_sk_rht_init(net: *mut net) -> c_int;
    pub fn tipc_sk_rht_destroy(net: *mut net);
    pub fn tipc_socket_init() -> c_int;
    pub fn tipc_socket_stop();
    pub fn tipc_nl_sk_walk(skb: *mut sk_buff, cb: *mut netlink_callback, handler: *mut c_void) -> c_int;
    pub fn tipc_dump_start(cb: *mut netlink_callback) -> c_int;
    pub fn __tipc_dump_start(cb: *mut netlink_callback, net: *mut net) -> c_int;
    pub fn tipc_dump_done(cb: *mut netlink_callback) -> c_int;
    pub fn tipc_nl_sk_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
    pub fn tipc_nl_publ_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
    pub fn tipc_sk_filtering(sk: *mut sock) -> bool;
    pub fn tipc_sock_get_portid(sk: *mut sock) -> u32;
    pub fn tipc_sk_overlimit1(sk: *mut sock, skb: *mut sk_buff) -> bool;
    pub fn tipc_sk_overlimit2(sk: *mut sock, skb: *mut sk_buff) -> bool;
    pub fn tipc_sk_dump(sk: *mut sock, queues: u16, buf: *mut c_char) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
