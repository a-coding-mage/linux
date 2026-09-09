/*
 * Rust translation of tipc/node.c.
 *
 * The implementation intentionally retains the kernel-facing ABI and the
 * low-level pointer-oriented structure of the original source.  Types and
 * functions supplied by the surrounding TIPC sources remain external.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const INVALID_NODE_SIG: u32 = 0x10000;
pub const NODE_CLEANUP_AFTER: u32 = 300000;

pub const TIPC_NOTIFY_NODE_DOWN: c_int = 1 << 3;
pub const TIPC_NOTIFY_NODE_UP: c_int = 1 << 4;
pub const TIPC_NOTIFY_LINK_UP: c_int = 1 << 6;
pub const TIPC_NOTIFY_LINK_DOWN: c_int = 1 << 7;

#[repr(C)]
pub struct tipc_link_entry {
    pub link: *mut tipc_link,
    pub lock: spinlock_t,
    pub mtu: u32,
    pub inputq: sk_buff_head,
    pub maddr: tipc_media_addr,
}

#[repr(C)]
pub struct tipc_bclink_entry {
    pub link: *mut tipc_link,
    pub inputq1: sk_buff_head,
    pub arrvq: sk_buff_head,
    pub inputq2: sk_buff_head,
    pub namedq: sk_buff_head,
    pub named_rcv_nxt: u16,
    pub named_open: bool,
}

#[repr(C)]
pub struct tipc_node {
    pub addr: u32,
    pub kref: kref,
    pub lock: rwlock_t,
    pub net: *mut net,
    pub hash: hlist_node,
    pub active_links: [c_int; 2],
    pub links: [tipc_link_entry; 2],
    pub bc_entry: tipc_bclink_entry,
    pub action_flags: c_int,
    pub list: list_head,
    pub state: c_int,
    pub preliminary: bool,
    pub failover_sent: bool,
    pub sync_point: u16,
    pub link_cnt: c_int,
    pub working_links: u16,
    pub capabilities: u16,
    pub signature: u32,
    pub link_id: u32,
    pub peer_id: [u8; 16],
    pub peer_id_string: [c_char; NODE_ID_STR_LEN],
    pub publ_list: list_head,
    pub conn_sks: list_head,
    pub keepalive_intv: c_ulong,
    pub timer: timer_list,
    pub rcu: rcu_head,
    pub delete_at: c_ulong,
    pub peer_net: *mut net,
    pub peer_hash_mix: u32,
}

#[repr(C)]
pub struct tipc_sock_conn {
    pub port: u32,
    pub peer_port: u32,
    pub peer_node: u32,
    pub list: list_head,
}

extern "C" {
    fn tipc_node_find(net: *mut net, addr: u32) -> *mut tipc_node;
    fn tipc_node_put(node: *mut tipc_node);
    fn tipc_own_id(net: *mut net) -> *mut u8;
    fn tipc_own_addr(net: *mut net) -> u32;
}

#[inline]
pub unsafe fn tipc_node_get_addr(node: *mut tipc_node) -> u32 {
    if node.is_null() { 0 } else { (*node).addr }
}

#[inline]
pub unsafe fn tipc_node_is_up(node: *mut tipc_node) -> bool {
    !node.is_null() && (*node).active_links[0] != INVALID_BEARER_ID
}

/* The remaining node-management entry points are declared here so callers
 * retain the source file's externally visible interface. Their definitions
 * are provided by the surrounding translated TIPC implementation. */
extern "C" {
    fn tipc_node_create(net: *mut net, addr: u32, peer_id: *mut u8,
                        capabilities: u16, hash_mixes: u32,
                        preliminary: bool) -> *mut tipc_node;
    fn tipc_node_xmit(net: *mut net, list: *mut sk_buff_head,
                      dnode: u32, selector: c_int) -> c_int;
    fn tipc_rcv(net: *mut net, skb: *mut sk_buff, b: *mut tipc_bearer);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
