/* SPDX-License-Identifier: GPL-2.0 */
/* Operations on the network namespace.  C preprocessor configuration is
 * represented by cfg attributes/comments where applicable. */

pub const NETDEV_HASHBITS: usize = 8;
pub const NETDEV_HASHENTRIES: usize = 1usize << NETDEV_HASHBITS;

#[repr(C)]
pub struct net {
    pub passive: refcount_t,
    pub rules_mod_lock: spinlock_t,
    pub dev_base_seq: c_uint,
    pub ifindex: u32,
    pub nsid_lock: spinlock_t,
    pub fnhe_genid: atomic_t,
    pub list: list_head,
    pub exit_list: list_head,
    pub defer_free_list: llist_node,
    pub cleanup_list: llist_node,
    pub ptype_all: list_head,
    pub ptype_specific: list_head,
    #[cfg(feature = "CONFIG_KEYS")]
    pub key_domain: *mut key_tag,
    pub user_ns: *mut user_namespace,
    pub ucounts: *mut ucounts,
    pub netns_ids: idr,
    pub ns: ns_common,
    pub refcnt_tracker: ref_tracker_dir,
    pub notrefcnt_tracker: ref_tracker_dir,
    pub dev_base_head: list_head,
    pub proc_net: *mut proc_dir_entry,
    pub proc_net_stat: *mut proc_dir_entry,
    #[cfg(feature = "CONFIG_SYSCTL")]
    pub sysctls: ctl_table_set,
    pub rtnl: *mut sock,
    pub genl_sock: *mut sock,
    pub uevent_sock: *mut uevent_sock,
    pub dev_name_head: *mut hlist_head,
    pub dev_index_head: *mut hlist_head,
    pub dev_by_index: xarray,
    pub netdev_chain: raw_notifier_head,
    pub hash_mix: u32,
    pub is_dying: bool,
    pub loopback_dev: *mut net_device,
    pub rules_ops: list_head,
    pub core: netns_core,
    pub mib: netns_mib,
    pub packet: netns_packet,
    #[cfg(feature = "CONFIG_UNIX")]
    pub unx: netns_unix,
    pub nexthop: netns_nexthop,
    pub ipv4: netns_ipv4,
    #[cfg(feature = "CONFIG_IPV6")]
    pub ipv6: netns_ipv6,
    #[cfg(feature = "CONFIG_IEEE802154_6LOWPAN")]
    pub ieee802154_lowpan: netns_ieee802154_lowpan,
    #[cfg(any(feature = "CONFIG_IP_SCTP", feature = "CONFIG_IP_SCTP_MODULE"))]
    pub sctp: netns_sctp,
    #[cfg(feature = "CONFIG_NETFILTER")]
    pub nf: netns_nf,
    #[cfg(any(feature = "CONFIG_NF_CONNTRACK", feature = "CONFIG_NF_CONNTRACK_MODULE"))]
    pub ct: netns_ct,
    #[cfg(any(feature = "CONFIG_NF_TABLES", feature = "CONFIG_NF_TABLES_MODULE"))]
    pub nft: netns_nftables,
    #[cfg(feature = "CONFIG_NF_FLOW_TABLE")]
    pub ft: netns_ft,
    #[cfg(feature = "CONFIG_WEXT_CORE")]
    pub wext_nlevents: sk_buff_head,
    pub gen: *mut net_generic,
    pub bpf: netns_bpf,
    #[cfg(feature = "CONFIG_XFRM")]
    pub xfrm: netns_xfrm,
    pub net_cookie: u64,
    #[cfg(feature = "CONFIG_IP_VS")]
    pub ipvs: *mut netns_ipvs,
    #[cfg(feature = "CONFIG_MPLS")]
    pub mpls: netns_mpls,
    #[cfg(feature = "CONFIG_CAN")]
    pub can: netns_can,
    #[cfg(feature = "CONFIG_XDP_SOCKETS")]
    pub xdp: netns_xdp,
    #[cfg(feature = "CONFIG_MCTP")]
    pub mctp: netns_mctp,
    #[cfg(feature = "CONFIG_CRYPTO_USER")]
    pub crypto_nlsk: *mut sock,
    pub diag_nlsk: *mut sock,
    #[cfg(feature = "CONFIG_SMC")]
    pub smc: netns_smc,
    #[cfg(feature = "CONFIG_DEBUG_NET_SMALL_RTNL")]
    pub rtnl_mutex: mutex,
    #[cfg(feature = "CONFIG_DEBUG_NET_SMALL_RTNL")]
    pub rtnl_work: work_struct,
    #[cfg(feature = "CONFIG_DEBUG_NET_SMALL_RTNL")]
    pub dev_unreg_head: list_head,
    #[cfg(feature = "CONFIG_DEBUG_NET_SMALL_RTNL")]
    pub dev_unreg_lock: spinlock_t,
    #[cfg(feature = "CONFIG_VSOCKETS")]
    pub vsock: netns_vsock,
}

extern "C" {
    pub static mut init_net: net;
    pub static mut net_namespace_list: list_head;
    pub static mut cleanup_net_task: *mut task_struct;
    pub static mut __fib6_flush_trees: Option<unsafe extern "C" fn(*mut net)>;
    pub fn __put_net(net: *mut net);
    pub fn copy_net_ns(flags: u64, user_ns: *mut user_namespace, old_net: *mut net) -> *mut net;
    pub fn net_ns_get_ownership(net: *const net, uid: *mut kuid_t, gid: *mut kgid_t);
    pub fn net_ns_barrier();
    pub fn get_net_ns(ns: *mut ns_common) -> *mut ns_common;
    pub fn get_net_ns_by_fd(fd: c_int) -> *mut net;
    pub fn get_net_ns_by_pid(pid: pid_t) -> *mut net;
    pub fn net_drop_ns(ns: *mut ns_common);
    pub fn net_passive_dec(net: *mut net);
    pub fn peernet2id_alloc(net: *mut net, peer: *mut net, gfp: gfp_t) -> c_int;
    pub fn peernet2id(net: *const net, peer: *mut net) -> c_int;
    pub fn peernet_has_id(net: *const net, peer: *mut net) -> bool;
    pub fn get_net_ns_by_id(net: *const net, id: c_int) -> *mut net;
    pub fn register_pernet_subsys(ops: *mut pernet_operations) -> c_int;
    pub fn unregister_pernet_subsys(ops: *mut pernet_operations);
    pub fn register_pernet_device(ops: *mut pernet_operations) -> c_int;
    pub fn unregister_pernet_device(ops: *mut pernet_operations);
    pub fn net_sysctl_init() -> c_int;
    pub fn register_net_sysctl_sz(net: *mut net, path: *const c_char, table: *const ctl_table, size: usize) -> *mut ctl_table_header;
    pub fn unregister_net_sysctl_table(header: *mut ctl_table_header);
    pub fn net_ns_init();
}

#[repr(C)]
pub struct pernet_operations {
    pub list: list_head,
    pub init: Option<unsafe extern "C" fn(*mut net) -> c_int>,
    pub pre_exit: Option<unsafe extern "C" fn(*mut net)>,
    pub exit: Option<unsafe extern "C" fn(*mut net)>,
    pub exit_batch: Option<unsafe extern "C" fn(*mut list_head)>,
    pub exit_rtnl: Option<unsafe extern "C" fn(*mut net, *mut list_head)>,
    pub id: *mut c_uint,
    pub size: usize,
}

#[repr(C)]
pub struct possible_net_t {
    pub net: *mut net,
}

#[inline]
pub unsafe fn to_net_ns(ns: *mut ns_common) -> *mut net {
    (ns as *mut u8).sub(core::mem::offset_of!(net, ns)) as *mut net
}

#[inline]
pub unsafe fn get_net(n: *mut net) -> *mut net { ns_ref_inc(n); n }
#[inline]
pub unsafe fn maybe_get_net(mut n: *mut net) -> *mut net { if !ns_ref_get(n) { n = core::ptr::null_mut(); } n }
#[inline]
pub unsafe fn put_net(n: *mut net) { if ns_ref_put(n) { __put_net(n); } }
#[inline]
pub unsafe fn net_eq(a: *const net, b: *const net) -> c_int { (a == b) as c_int }
#[inline]
pub unsafe fn check_net(n: *const net) -> c_int { (ns_ref_read(n) != 0) as c_int }
#[inline]
pub unsafe fn net_passive_inc(n: *mut net) { refcount_inc(&mut (*n).passive); }
#[inline]
pub unsafe fn net_initialized(n: *const net) -> bool { !(*n).list.next.is_null() }
#[inline]
pub unsafe fn get_net_track(n: *mut net, tracker: *mut netns_tracker, gfp: gfp_t) -> *mut net { get_net(n); netns_tracker_alloc(n, tracker, gfp); n }
#[inline]
pub unsafe fn put_net_track(n: *mut net, tracker: *mut netns_tracker) { __netns_tracker_free(n, tracker, true); put_net(n); }

#[inline]
pub unsafe fn write_pnet(p: *mut possible_net_t, n: *mut net) { (*p).net = n; }
#[inline]
pub unsafe fn read_pnet(p: *const possible_net_t) -> *mut net { (*p).net }
#[inline]
pub unsafe fn read_pnet_rcu(p: *const possible_net_t) -> *mut net { (*p).net }

#[inline] pub unsafe fn rt_genid_ipv4(n: *const net) -> c_int { atomic_read(&(*n).ipv4.rt_genid) }
#[inline] pub unsafe fn rt_genid_bump_ipv4(n: *mut net) { atomic_inc(&mut (*n).ipv4.rt_genid); }
#[inline] pub unsafe fn rt_genid_bump_ipv6(n: *mut net) { if let Some(f) = __fib6_flush_trees { f(n); } }
#[inline] pub unsafe fn rt_genid_bump_all(n: *mut net) { rt_genid_bump_ipv4(n); rt_genid_bump_ipv6(n); }
#[inline] pub unsafe fn fnhe_genid(n: *const net) -> c_int { atomic_read(&(*n).fnhe_genid) }
#[inline] pub unsafe fn fnhe_genid_bump(n: *mut net) { atomic_inc(&mut (*n).fnhe_genid); }

// External dependency types and primitives are supplied by the corresponding kernel translations.
extern "C" {
    fn ns_ref_inc(net: *mut net);
    fn ns_ref_get(net: *mut net) -> bool;
    fn ns_ref_put(net: *mut net) -> bool;
    fn ns_ref_read(net: *const net) -> c_int;
    fn refcount_inc(r: *mut refcount_t);
    fn atomic_read(a: *const atomic_t) -> c_int;
    fn atomic_inc(a: *mut atomic_t);
    fn netns_tracker_alloc(net: *mut net, tracker: *mut netns_tracker, gfp: gfp_t);
    fn __netns_tracker_free(net: *mut net, tracker: *mut netns_tracker, refcounted: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
