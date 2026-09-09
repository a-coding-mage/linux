/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Linux INET6 implementation. Literal Rust translation of ip6_fib.h. */

// External kernel types, constants, and helpers are supplied by other translated units.

#[cfg(feature = "CONFIG_IPV6_MULTIPLE_TABLES")]
pub const FIB6_TABLE_HASHSZ: usize = 256;
#[cfg(not(feature = "CONFIG_IPV6_MULTIPLE_TABLES"))]
pub const FIB6_TABLE_HASHSZ: usize = 1;
pub const RT6_DEBUG: i32 = 2;

pub struct rt6_info;
pub struct fib6_info;

#[repr(C)]
pub struct fib6_config {
    pub fc_table: u32, pub fc_metric: u32, pub fc_dst_len: i32, pub fc_src_len: i32,
    pub fc_ifindex: i32, pub fc_flags: u32, pub fc_protocol: u32, pub fc_type: u16,
    pub fc_delete_all_nh: u16, pub fc_ignore_dev_down: u16, pub fc_nh_id: u32,
    pub fc_dst: in6_addr, pub fc_src: in6_addr, pub fc_prefsrc: in6_addr, pub fc_gateway: in6_addr,
    pub fc_expires: usize, pub fc_mx: *mut nlattr, pub fc_mx_len: i32, pub fc_mp_len: i32,
    pub fc_mp: *mut nlattr, pub fc_nlinfo: nl_info, pub fc_encap: *mut nlattr,
    pub fc_encap_type: u16, pub fc_is_fdb: bool,
}

#[repr(C)]
pub struct fib6_node {
    pub parent: *mut fib6_node, pub left: *mut fib6_node, pub right: *mut fib6_node,
    // CONFIG_IPV6_SUBTREES: subtree is present when enabled.
    pub subtree: *mut fib6_node, pub leaf: *mut fib6_info,
    pub fn_bit: u16, pub fn_flags: u16, pub fn_sernum: i32, pub rr_ptr: *mut fib6_info,
    pub rcu: rcu_head,
}
#[repr(C)] pub struct fib6_gc_args { pub timeout: i32, pub more: i32 }

// CONFIG_IPV6_SUBTREES controls FIB6_SUBTREE and the source-route helpers.
pub const FIB6_EXCEPTION_BUCKET_SIZE_SHIFT: u32 = 10;
pub const FIB6_EXCEPTION_BUCKET_SIZE: u32 = 1 << FIB6_EXCEPTION_BUCKET_SIZE_SHIFT;
pub const FIB6_MAX_DEPTH: i32 = 5;

#[repr(C)] pub struct rt6key { pub addr: in6_addr, pub plen: i32 }
pub struct fib6_table;
#[repr(C)] pub struct rt6_exception_bucket { pub chain: hlist_head, pub depth: i32 }
#[repr(C)] pub struct rt6_exception { pub hlist: hlist_node, pub rt6i: *mut rt6_info, pub stamp: usize, pub rcu: rcu_head }

#[repr(C)] pub struct fib6_nh {
    pub nh_common: fib_nh_common,
    pub last_probe: usize,
    pub rt6i_pcpu: *mut *mut rt6_info,
    pub rt6i_exception_bucket: *mut rt6_exception_bucket,
}

#[repr(C)]
pub struct fib6_info {
    pub fib6_table: *mut fib6_table, pub fib6_next: *mut fib6_info, pub fib6_node: *mut fib6_node,
    pub fib6_siblings: list_head, pub fib6_nsiblings: u32, pub fib6_ref: refcount_t,
    pub expires: usize, pub gc_link: hlist_node, pub fib6_metrics: *mut dst_metrics,
    pub fib6_dst: rt6key, pub fib6_flags: u32, pub fib6_src: rt6key, pub fib6_prefsrc: rt6key,
    pub fib6_metric: u32, pub fib6_protocol: u8, pub fib6_type: u8, pub offload: u8,
    pub trap: u8, pub offload_failed: u8, pub should_flush: u8, pub dst_nocount: u8,
    pub dst_nopolicy: u8, pub fib6_destroying: u8, pub unused: u8, pub purge_link: list_head,
    pub rcu: rcu_head, pub nh: *mut nexthop, pub fib6_nh: [fib6_nh; 0],
}

#[repr(C)] pub struct rt6_info_full {
    pub dst: dst_entry, pub from: *mut fib6_info, pub sernum: i32, pub rt6i_dst: rt6key,
    pub rt6i_src: rt6key, pub rt6i_gateway: in6_addr, pub rt6i_idev: *mut inet6_dev,
    pub rt6i_flags: u32, pub rt6i_nfheader_len: u16,
}
#[repr(C)] pub struct fib6_result { pub nh: *mut fib6_nh, pub f6i: *mut fib6_info, pub fib6_flags: u32, pub fib6_type: u8, pub rt6: *mut rt6_info }

pub const RTN_TL_ROOT: u32 = 0x0001;
pub const RTN_ROOT: u32 = 0x0002;
pub const RTN_RTINFO: u32 = 0x0004;
pub const RT6_TABLE_HAS_DFLT_ROUTER: u32 = 1 << 0;
pub const RT6_TABLE_UNSPEC: u32 = RT_TABLE_UNSPEC;
pub const RT6_TABLE_MAIN: u32 = RT_TABLE_MAIN;
pub const RT6_TABLE_DFLT: u32 = RT6_TABLE_MAIN;
pub const RT6_TABLE_INFO: u32 = RT6_TABLE_MAIN;
pub const RT6_TABLE_PREFIX: u32 = RT6_TABLE_MAIN;

#[repr(C)] pub struct fib6_table_full {
    pub tb6_hlist: hlist_node, pub tb6_id: u32, pub tb6_lock: spinlock_t,
    pub tb6_root: fib6_node, pub tb6_peers: inet_peer_base, pub flags: u32,
    pub fib_seq: u32, pub tb6_gc_hlist: hlist_head,
}

#[cfg(feature = "CONFIG_IPV6_MULTIPLE_TABLES")] pub const FIB6_TABLE_MIN: u32 = 1;
#[cfg(feature = "CONFIG_IPV6_MULTIPLE_TABLES")] pub const FIB6_TABLE_MAX: u32 = RT_TABLE_MAX;
#[cfg(feature = "CONFIG_IPV6_MULTIPLE_TABLES")] pub const RT6_TABLE_LOCAL: u32 = RT_TABLE_LOCAL;
#[cfg(not(feature = "CONFIG_IPV6_MULTIPLE_TABLES"))] pub const FIB6_TABLE_MIN: u32 = RT_TABLE_MAIN;
#[cfg(not(feature = "CONFIG_IPV6_MULTIPLE_TABLES"))] pub const FIB6_TABLE_MAX: u32 = FIB6_TABLE_MIN;
#[cfg(not(feature = "CONFIG_IPV6_MULTIPLE_TABLES"))] pub const RT6_TABLE_LOCAL: u32 = RT6_TABLE_MAIN;

pub type pol_lookup_t = unsafe extern "C" fn(*mut net, *mut fib6_table, *mut flowi6, *const sk_buff, i32) -> *mut rt6_info;

#[repr(C)] pub struct fib6_entry_notifier_info { pub info: fib_notifier_info, pub rt: *mut fib6_info, pub nsiblings: u32 }

// The remaining declarations are external kernel interfaces from the original header.
extern "C" {
    pub fn fib6_info_alloc(gfp_flags: gfp_t, with_fib6_nh: bool) -> *mut fib6_info;
    pub fn fib6_info_destroy_rcu(head: *mut rcu_head);
    pub fn fib6_get_table(net: *mut net, id: u32) -> *mut fib6_table;
    pub fn fib6_new_table(net: *mut net, id: u32) -> *mut fib6_table;
    pub fn fib6_lookup(net: *mut net, oif: i32, fl6: *mut flowi6, res: *mut fib6_result, flags: i32) -> i32;
    pub fn fib6_table_lookup(net: *mut net, table: *mut fib6_table, oif: i32, fl6: *mut flowi6, res: *mut fib6_result, strict: i32) -> i32;
    pub fn fib6_init() -> i32;
    pub fn fib6_node_lookup(root: *mut fib6_node, daddr: *const in6_addr, saddr: *const in6_addr) -> *mut fib6_node;
    pub fn fib6_locate(root: *mut fib6_node, daddr: *const in6_addr, dst_len: i32, saddr: *const in6_addr, src_len: i32, exact_match: bool) -> *mut fib6_node;
    pub fn fib6_add(root: *mut fib6_node, rt: *mut fib6_info, info: *mut nl_info, extack: *mut netlink_ext_ack) -> i32;
    pub fn fib6_del(rt: *mut fib6_info, info: *mut nl_info, del_reason: rt_del_reason) -> i32;
    pub fn fib6_age_exceptions(rt: *mut fib6_info, args: *mut fib6_gc_args, now: usize);
    pub fn fib6_run_gc(expires: usize, net: *mut net, force: bool);
    pub fn fib6_gc_cleanup();
    pub fn fib6_tables_seq_read(net: *const net) -> u32;
    pub fn fib6_update_sernum(net: *mut net, rt: *mut fib6_info);
    pub fn fib6_metric_set(f6i: *mut fib6_info, metric: i32, val: u32);
    pub fn fib6_info_hw_flags_set(net: *mut net, f6i: *mut fib6_info, offload: bool, trap: bool, offload_failed: bool);
}

#[inline] pub unsafe fn fib6_requires_src(rt: *const fib6_info) -> bool { (*rt).fib6_src.plen > 0 }
#[inline] pub unsafe fn fib6_clean_expires(f6i: *mut fib6_info) { (*f6i).fib6_flags &= !RTF_EXPIRES; (*f6i).expires = 0; }
#[inline] pub unsafe fn fib6_set_expires(f6i: *mut fib6_info, expires: usize) { (*f6i).expires = expires; (*f6i).fib6_flags |= RTF_EXPIRES; }

#[repr(C)] pub enum fib6_walk_state { FWS_L, FWS_R, FWS_C, FWS_U }
#[repr(C)] pub struct fib6_walker {
    pub lh: list_head, pub root: *mut fib6_node, pub node: *mut fib6_node, pub leaf: *mut fib6_info,
    pub state: fib6_walk_state, pub skip: u32, pub count: u32, pub skip_in_node: u32,
    pub func: Option<unsafe extern "C" fn(*mut fib6_walker) -> i32>, pub args: *mut core::ffi::c_void,
}
#[repr(C)] pub struct rt6_statistics {
    pub fib_nodes: u32, pub fib_route_nodes: u32, pub fib_rt_entries: u32,
    pub fib_rt_cache: u32, pub fib_discarded_routes: u32, pub fib_rt_alloc: atomic_t,
}
#[repr(C)] pub struct ipv6_route_iter { pub p: seq_net_private, pub w: fib6_walker, pub skip: i64, pub tbl: *mut fib6_table, pub sernum: i32 }

#[inline] pub unsafe fn fib6_metric_locked(f6i: *const fib6_info, metric: i32) -> bool {
    ((*(*f6i).fib6_metrics).metrics[(RTAX_LOCK - 1) as usize] & (1u32 << metric)) != 0
}

// CONFIG_IPV6 and CONFIG_IPV6_MULTIPLE_TABLES conditional declarations are retained by cfg intent.
#[cfg(feature = "CONFIG_IPV6_MULTIPLE_TABLES")]
#[inline] pub unsafe fn fib6_has_custom_rules(net: *const net) -> bool { (*net).ipv6.fib6_has_custom_rules }
#[cfg(not(feature = "CONFIG_IPV6_MULTIPLE_TABLES"))]
#[inline] pub unsafe fn fib6_has_custom_rules(_net: *const net) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
