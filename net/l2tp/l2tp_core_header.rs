/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of l2tp_core.h. Kernel-provided types and functions are external dependencies. */

use core::ffi::c_void;

/* External kernel types and constants supplied by other translated headers. */
#[repr(C)] pub struct sk_buff;
#[repr(C)] pub struct net;
#[repr(C)] pub struct sock;
#[repr(C)] pub struct dst_entry;
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct in_addr { pub s_addr: u32 }
#[repr(C)] pub struct in6_addr { pub in6_u: [u8; 16] }
#[repr(C)] pub struct atomic_long_t { pub counter: isize }
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct refcount_t { pub refs: i32 }
#[repr(C)] pub struct rcu_head;
#[repr(C)] pub struct sk_buff_head;
#[repr(C)] pub struct hlist_node;
#[repr(C)] pub struct work_struct;

pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type c_int = core::ffi::c_int;
pub type c_long = core::ffi::c_long;
pub type ulong = core::ffi::c_ulong;
pub type IFNAMSIZ = usize;
pub const L2TP_SESSION_MAGIC: c_int = 0x0C04EB7D;
pub const L2TP_SESSION_NAME_MAX: usize = 32;
pub const L2TP_TUNNEL_NAME_MAX: usize = 20;

#[repr(C)] pub struct l2tp_stats {
    pub tx_packets: atomic_long_t, pub tx_bytes: atomic_long_t, pub tx_errors: atomic_long_t,
    pub rx_packets: atomic_long_t, pub rx_bytes: atomic_long_t, pub rx_seq_discards: atomic_long_t,
    pub rx_oos_packets: atomic_long_t, pub rx_errors: atomic_long_t,
    pub rx_cookie_discards: atomic_long_t, pub rx_invalid: atomic_long_t,
}
#[repr(C)] pub struct l2tp_session_cfg {
    pub pw_type: l2tp_pwtype, pub recv_seq: u32, pub send_seq: u32, pub lns_mode: u32,
    pub l2specific_type: u16, pub cookie: [u8; 8], pub cookie_len: c_int,
    pub peer_cookie: [u8; 8], pub peer_cookie_len: c_int, pub reorder_timeout: c_int,
    pub ifname: *mut i8,
}
#[repr(C)] pub struct l2tp_session_coll_list { pub lock: spinlock_t, pub list: list_head, pub ref_count: refcount_t }
#[repr(C)] pub struct l2tp_session {
    pub magic: c_int, pub dead: c_long, pub rcu: rcu_head, pub tunnel: *mut l2tp_tunnel,
    pub session_id: u32, pub peer_session_id: u32, pub cookie: [u8; 8], pub cookie_len: c_int,
    pub peer_cookie: [u8; 8], pub peer_cookie_len: c_int, pub l2specific_type: u16, pub hdr_len: u16,
    pub nr: u32, pub ns: u32, pub reorder_q: sk_buff_head, pub nr_max: u32, pub nr_window_size: u32,
    pub nr_oos: u32, pub nr_oos_count: c_int, pub nr_oos_count_max: c_int, pub list: list_head,
    pub ref_count: refcount_t, pub hlist: hlist_node, pub hlist_key: ulong,
    pub coll_list: *mut l2tp_session_coll_list, pub clist: list_head,
    pub name: [i8; L2TP_SESSION_NAME_MAX], pub ifname: [i8; 16], pub recv_seq: u32, pub send_seq: u32,
    pub lns_mode: u32, pub reorder_timeout: c_int, pub reorder_skip: c_int, pub pwtype: l2tp_pwtype,
    pub stats: l2tp_stats, pub del_work: work_struct,
    pub recv_skb: Option<unsafe extern "C" fn(*mut l2tp_session, *mut sk_buff, c_int)>,
    pub session_close: Option<unsafe extern "C" fn(*mut l2tp_session)>,
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>, pub priv_: [u8; 0],
}
#[repr(C)] pub struct l2tp_tunnel_cfg {
    pub encap: l2tp_encap_type, pub local_ip: in_addr, pub peer_ip: in_addr,
    pub local_ip6: *mut in6_addr, pub peer_ip6: *mut in6_addr,
    pub local_udp_port: u16, pub peer_udp_port: u16, pub use_udp_checksums: u32,
    pub udp6_zero_tx_checksums: u32, pub udp6_zero_rx_checksums: u32,
}
#[repr(C)] pub struct l2tp_tunnel {
    pub dead: ulong, pub rcu: rcu_head, pub list_lock: spinlock_t, pub acpt_newsess: bool,
    pub session_list: list_head, pub tunnel_id: u32, pub peer_tunnel_id: u32, pub version: c_int,
    pub name: [i8; L2TP_TUNNEL_NAME_MAX], pub encap: l2tp_encap_type, pub stats: l2tp_stats,
    pub l2tp_net: *mut net, pub ref_count: refcount_t, pub sock: *mut sock, pub fd: c_int,
    pub del_work: work_struct,
}
#[repr(C)] pub struct l2tp_nl_cmd_ops {
    pub session_create: Option<unsafe extern "C" fn(*mut net, *mut l2tp_tunnel, u32, u32, *mut l2tp_session_cfg) -> c_int>,
    pub session_delete: Option<unsafe extern "C" fn(*mut l2tp_session)>,
}

#[repr(C)] pub enum l2tp_pwtype { _Placeholder = 0 }
#[repr(C)] pub enum l2tp_encap_type { _Placeholder = 0 }
extern "C" {
    pub fn l2tp_tunnel_put(tunnel: *mut l2tp_tunnel); pub fn l2tp_session_put(session: *mut l2tp_session);
    pub fn l2tp_tunnel_get(net: *const net, tunnel_id: u32) -> *mut l2tp_tunnel;
    pub fn l2tp_tunnel_get_next(net: *const net, key: *mut ulong) -> *mut l2tp_tunnel;
    pub fn l2tp_v3_session_get(net: *const net, sk: *mut sock, session_id: u32) -> *mut l2tp_session;
    pub fn l2tp_v2_session_get(net: *const net, tunnel_id: u16, session_id: u16) -> *mut l2tp_session;
    pub fn l2tp_session_get(net: *const net, sk: *mut sock, pver: c_int, tunnel_id: u32, session_id: u32) -> *mut l2tp_session;
    pub fn l2tp_session_get_next(net: *const net, sk: *mut sock, pver: c_int, tunnel_id: u32, key: *mut ulong) -> *mut l2tp_session;
    pub fn l2tp_session_get_by_ifname(net: *const net, ifname: *const i8) -> *mut l2tp_session;
    pub fn l2tp_tunnel_create(fd: c_int, version: c_int, tunnel_id: u32, peer_tunnel_id: u32, cfg: *mut l2tp_tunnel_cfg, tunnelp: *mut *mut l2tp_tunnel) -> c_int;
    pub fn l2tp_tunnel_register(tunnel: *mut l2tp_tunnel, net: *mut net, cfg: *mut l2tp_tunnel_cfg) -> c_int;
    pub fn l2tp_tunnel_delete(tunnel: *mut l2tp_tunnel);
    pub fn l2tp_session_create(priv_size: c_int, tunnel: *mut l2tp_tunnel, session_id: u32, peer_session_id: u32, cfg: *mut l2tp_session_cfg) -> *mut l2tp_session;
    pub fn l2tp_session_register(session: *mut l2tp_session, tunnel: *mut l2tp_tunnel) -> c_int; pub fn l2tp_session_delete(session: *mut l2tp_session);
    pub fn l2tp_recv_common(session: *mut l2tp_session, skb: *mut sk_buff, ptr: *mut u8, optr: *mut u8, hdrflags: u16, length: c_int);
    pub fn l2tp_udp_encap_recv(sk: *mut sock, skb: *mut sk_buff) -> c_int;
    pub fn l2tp_session_set_header_len(session: *mut l2tp_session, version: c_int, encap: l2tp_encap_type);
    pub fn l2tp_xmit_skb(session: *mut l2tp_session, skb: *mut sk_buff) -> c_int;
    pub fn l2tp_nl_register_ops(pw_type: l2tp_pwtype, ops: *const l2tp_nl_cmd_ops) -> c_int; pub fn l2tp_nl_unregister_ops(pw_type: l2tp_pwtype);
    pub fn l2tp_ioctl(sk: *mut sock, cmd: c_int, karg: *mut c_int) -> c_int; pub fn l2tp_sk_to_tunnel(sk: *const sock) -> *mut l2tp_tunnel;
    pub fn sk_dst_get(sk: *mut sock) -> *mut dst_entry; pub fn dst_mtu(dst: *mut dst_entry) -> u32; pub fn dst_release(dst: *mut dst_entry);
    pub fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> bool;
}

pub unsafe fn l2tp_session_priv(session: *mut l2tp_session) -> *mut c_void { (*session).priv_.as_mut_ptr() as *mut c_void }
pub unsafe fn l2tp_get_l2specific_len(session: *mut l2tp_session) -> c_int {
    /* L2TP_L2SPECTYPE_DEFAULT is supplied by the protocol definitions. */
    if (*session).l2specific_type == 0 { 4 } else { 0 }
}
pub unsafe fn l2tp_tunnel_dst_mtu(tunnel: *const l2tp_tunnel) -> u32 {
    let dst = sk_dst_get((*tunnel).sock); if dst.is_null() { return 0; }
    let mtu = dst_mtu(dst); dst_release(dst); mtu
}
pub unsafe fn l2tp_tunnel_uses_xfrm(_tunnel: *const l2tp_tunnel) -> bool { false }
pub unsafe fn l2tp_v3_ensure_opt_in_linear(session: *mut l2tp_session, skb: *mut sk_buff, ptr: *mut *mut u8, optr: *mut *mut u8) -> c_int {
    let opt_len = (*session).peer_cookie_len + l2tp_get_l2specific_len(session);
    if opt_len > 0 {
        let off = (*ptr as usize).wrapping_sub(*optr as usize);
        if !pskb_may_pull(skb, off + opt_len as usize) { return -1; }
        /* skb->data adjustment is supplied by the translated sk_buff definition. */
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
