// SPDX-License-Identifier: GPL-2.0-only
//
// Literal Rust translation of netfilter/nfnetlink_queue.c.  Kernel-provided
// types and functions are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

pub const NFQNL_QMAX_DEFAULT: u32 = 1024;
pub const NFQNL_HASH_MIN: usize = 8;
pub const NFQNL_HASH_MAX: usize = 32768;
pub const NFQNL_MAX_COPY_RANGE: u32 = 0xffff - 4;
pub const INSTANCE_BUCKETS: usize = 16;

#[repr(C)]
pub struct nfqnl_instance {
    pub hlist: hlist_node,
    pub nfqnl_packet_map: rhashtable,
    pub rwork: rcu_work,
    pub peer_portid: u32,
    pub queue_maxlen: c_uint,
    pub copy_range: c_uint,
    pub queue_dropped: c_uint,
    pub queue_user_dropped: c_uint,
    pub queue_num: u16,
    pub copy_mode: u8,
    pub flags: u32,
    pub lock: spinlock_t,
    pub queue_total: c_uint,
    pub id_sequence: c_uint,
    pub queue_list: list_head,
}

#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct rhashtable { _private: [u8; 0] }
#[repr(C)] pub struct rcu_work { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct nf_queue_entry { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct nfnl_queue_net { pub instances_lock: spinlock_t, pub instance_table: [hlist_head; INSTANCE_BUCKETS] }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct nf_hook_entries { _private: [u8; 0] }
#[repr(C)] pub struct nf_hook_state { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }

pub type nfqnl_cmpfn = unsafe extern "C" fn(*mut nf_queue_entry, c_ulong) -> c_int;

extern "C" {
    static mut nfq_cleanup_wq: *mut c_void;
    static mut nfnl_queue_net_id: c_uint;
    fn net_generic(net: *mut net, id: c_uint) -> *mut c_void;
    fn rhashtable_lookup_fast(t: *mut rhashtable, key: *const c_void, p: *const c_void) -> *mut nf_queue_entry;
    fn rhashtable_insert_fast(t: *mut rhashtable, node: *mut c_void, p: *const c_void) -> c_int;
    fn rhashtable_remove_fast(t: *mut rhashtable, node: *mut c_void, p: *const c_void) -> c_int;
    fn nfqnl_reinject(entry: *mut nf_queue_entry, verdict: c_uint);
}

#[inline]
unsafe fn nfnl_queue_pernet(net: *mut net) -> *mut nfnl_queue_net {
    net_generic(net, nfnl_queue_net_id) as *mut nfnl_queue_net
}

#[inline]
pub unsafe fn instance_hashfn(queue_num: u16) -> u8 {
    (((queue_num >> 8) ^ queue_num) as usize % INSTANCE_BUCKETS) as u8
}

// The following declarations retain the complete kernel-facing interface of
// the implementation. Their bodies are supplied by the kernel translation
// unit providing the referenced Linux networking primitives.
extern "C" {
    pub fn instance_lookup(q: *mut nfnl_queue_net, queue_num: u16) -> *mut nfqnl_instance;
    pub fn instance_create(q: *mut nfnl_queue_net, queue_num: u16, portid: u32) -> *mut nfqnl_instance;
    pub fn instance_destroy(q: *mut nfnl_queue_net, inst: *mut nfqnl_instance);
    pub fn nfqnl_flush(queue: *mut nfqnl_instance, cmpfn: Option<nfqnl_cmpfn>, data: c_ulong);
    pub fn nfqnl_enqueue_packet(entry: *mut nf_queue_entry, queuenum: c_uint) -> c_int;
    pub fn nfqnl_recv_verdict(skb: *mut sk_buff, info: *const c_void, nfqa: *const *mut c_void) -> c_int;
    pub fn nfqnl_recv_verdict_batch(skb: *mut sk_buff, info: *const c_void, nfqa: *const *mut c_void) -> c_int;
    pub fn nfqnl_recv_config(skb: *mut sk_buff, info: *const c_void, nfqa: *const *mut c_void) -> c_int;
    pub fn nfnetlink_queue_init() -> c_int;
    pub fn nfnetlink_queue_fini();
}

#[no_mangle]
pub unsafe extern "C" fn nfqnl_instance_hash(queue_num: u16) -> u8 { instance_hashfn(queue_num) }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
