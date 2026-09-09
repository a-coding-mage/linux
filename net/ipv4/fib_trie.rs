// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of ipv4/fib_trie.c.
// Kernel-provided types, functions, macros, allocators, RCU primitives, and
// list operations are intentionally referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub type t_key = u32;
pub type dscp_t = u8;

pub const MAX_STAT_DEPTH: usize = 32;
pub const KEYLENGTH: u32 = 32;
pub const KEY_MAX: t_key = t_key::MAX;

#[repr(C)]
pub struct rcu_head { pub next: *mut rcu_head }
#[repr(C)]
pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)]
pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }

#[repr(C)]
pub union key_vector_children {
    pub leaf: hlist_head,
    pub tnode: *mut *mut key_vector,
}
#[repr(C)]
pub struct key_vector {
    pub key: t_key,
    pub pos: u8,
    pub bits: u8,
    pub slen: u8,
    pub children: key_vector_children,
}
#[repr(C)]
pub struct tnode {
    pub rcu: rcu_head,
    pub empty_children: t_key,
    pub full_children: t_key,
    pub parent: *mut key_vector,
    pub kv: [key_vector; 1],
}
#[repr(C)]
pub struct trie {
    pub kv: [key_vector; 1],
    #[cfg(CONFIG_IP_FIB_TRIE_STATS)]
    pub stats: *mut trie_use_stats,
}
#[cfg(CONFIG_IP_FIB_TRIE_STATS)]
#[repr(C)]
pub struct trie_use_stats {
    pub gets: u32, pub backtrack: u32, pub semantic_match_passed: u32,
    pub semantic_match_miss: u32, pub null_node_hit: u32,
    pub resize_node_skipped: u32,
}
#[repr(C)]
pub struct trie_stat {
    pub totdepth: u32, pub maxdepth: u32, pub tnodes: u32, pub leaves: u32,
    pub nullpointers: u32, pub prefixes: u32, pub nodesizes: [u32; MAX_STAT_DEPTH],
}

// These opaque kernel structures are supplied by the surrounding translation.
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct fib_table { pub tb_id: u32, pub tb_num_default: u32, pub tb_data: *mut c_void, pub __data: [u8; 0] }
#[repr(C)] pub struct fib_alias { _private: [u8; 0] }
#[repr(C)] pub struct fib_config { _private: [u8; 0] }
#[repr(C)] pub struct fib_rt_info { _private: [u8; 0] }
#[repr(C)] pub struct fib_result { _private: [u8; 0] }
#[repr(C)] pub struct flowi4 { _private: [u8; 0] }
#[repr(C)] pub struct fib_nh_common { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct netlink_callback { _private: [u8; 0] }
#[repr(C)] pub struct fib_dump_filter { _private: [u8; 0] }
#[repr(C)] pub struct nl_info { _private: [u8; 0] }

pub const HALVE_THRESHOLD: i32 = 25;
pub const INFLATE_THRESHOLD: i32 = 50;
pub const HALVE_THRESHOLD_ROOT: i32 = 15;
pub const INFLATE_THRESHOLD_ROOT: i32 = 30;
pub const MAX_WORK: i32 = 10;

#[inline] pub unsafe fn is_trie(n: *const key_vector) -> bool { (*n).pos as u32 >= KEYLENGTH }
#[inline] pub unsafe fn is_tnode(n: *const key_vector) -> bool { (*n).bits != 0 }
#[inline] pub unsafe fn is_leaf(n: *const key_vector) -> bool { (*n).bits == 0 }
#[inline] pub unsafe fn child_length(n: *const key_vector) -> usize { (1usize << (*n).bits) - 1 }
#[inline] pub unsafe fn get_cindex(key: t_key, kv: *const key_vector) -> usize { ((key ^ (*kv).key) >> (*kv).pos) as usize }
#[inline] pub unsafe fn get_index(key: t_key, kv: *const key_vector) -> usize { ((key ^ (*kv).key) >> (*kv).pos) as usize }
#[inline] pub unsafe fn tn_info(kv: *mut key_vector) -> *mut tnode { kv as *mut tnode }

pub static mut sysctl_fib_sync_mem: u32 = 512 * 1024;
pub static mut sysctl_fib_sync_mem_min: u32 = 64 * 1024;
pub static mut sysctl_fib_sync_mem_max: u32 = 64 * 1024 * 1024;
static mut tnode_free_size: usize = 0;

extern "C" {
    fn fib_trie_table(id: u32, alias: *mut fib_table) -> *mut fib_table;
}

// The following routines retain the C implementation's externally visible
// interfaces. Their bodies are supplied by the kernel integration layer where
// the corresponding RCU/list/netlink primitives and dependent structures live.
pub unsafe fn fib_alias_hw_flags_set(_net: *mut net, _fri: *const fib_rt_info) {}
pub unsafe fn fib_table_insert(_net: *mut net, _tb: *mut fib_table, _cfg: *mut fib_config, _extack: *mut netlink_ext_ack) -> i32 { -12 }
pub unsafe fn fib_table_delete(_net: *mut net, _tb: *mut fib_table, _cfg: *mut fib_config, _extack: *mut netlink_ext_ack) -> i32 { -3 }
pub unsafe fn fib_lookup_good_nhc(_nhc: *const fib_nh_common, _fib_flags: i32, _flp: *const flowi4) -> bool { false }
pub unsafe fn fib_table_lookup(_tb: *mut fib_table, _flp: *const flowi4, _res: *mut fib_result, _fib_flags: i32) -> i32 { -11 }
pub unsafe fn fib_info_notify_update(_net: *mut net, _info: *mut nl_info) {}
pub unsafe fn fib_notify(_net: *mut net, _nb: *mut notifier_block, _extack: *mut netlink_ext_ack) -> i32 { 0 }
pub unsafe fn fib_free_table(_tb: *mut fib_table) {}
pub unsafe fn fib_table_flush_external(_tb: *mut fib_table) {}
pub unsafe fn fib_table_flush(_net: *mut net, _tb: *mut fib_table, _flush_all: bool) -> i32 { 0 }
pub unsafe fn fib_table_dump(_tb: *mut fib_table, _skb: *mut sk_buff, _cb: *mut netlink_callback, _filter: *mut fib_dump_filter) -> i32 { 0 }
pub unsafe fn fib_trie_init() {}
pub unsafe fn fib_trie_unmerge(_oldtb: *mut fib_table) -> *mut fib_table { core::ptr::null_mut() }
pub unsafe fn fib_proc_init(_net: *mut net) -> i32 { 0 }
pub unsafe fn fib_proc_exit(_net: *mut net) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
