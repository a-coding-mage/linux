/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel translation units:
// linux/mutex.h, linux/netdevice.h, linux/wait.h, linux/refcount.h,
// and uapi/linux/rtnetlink.h.

extern "C" {
    pub fn rtnetlink_send(skb: *mut sk_buff, net: *mut net, pid: u32, group: u32, echo: ::core::ffi::c_int) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn rtnetlink_maybe_send(skb: *mut sk_buff, net: *mut net, pid: u32, group: u32, echo: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if skb.is_null() { 0 } else { rtnetlink_send(skb, net, pid, group, echo) }
}

extern "C" {
    pub fn rtnl_unicast(skb: *mut sk_buff, net: *mut net, pid: u32) -> ::core::ffi::c_int;
    pub fn rtnl_notify(skb: *mut sk_buff, net: *mut net, pid: u32, group: u32, nlh: *const nlmsghdr, flags: gfp_t);
    pub fn rtnl_set_sk_err(net: *mut net, group: u32, error: ::core::ffi::c_int);
    pub fn rtnetlink_put_metrics(skb: *mut sk_buff, metrics: *mut u32) -> ::core::ffi::c_int;
    pub fn rtnl_put_cacheinfo(skb: *mut sk_buff, dst: *mut dst_entry, id: u32, expires: isize, error: u32) -> ::core::ffi::c_int;

    pub fn rtmsg_ifinfo(ty: ::core::ffi::c_int, dev: *mut net_device, change: u32, flags: gfp_t, portid: u32, nlh: *const nlmsghdr);
    pub fn rtmsg_ifinfo_newnet(ty: ::core::ffi::c_int, dev: *mut net_device, change: u32, flags: gfp_t, new_nsid: *mut ::core::ffi::c_int, new_ifindex: ::core::ffi::c_int);
    pub fn rtmsg_ifinfo_build_skb(ty: ::core::ffi::c_int, dev: *mut net_device, change: u32, event: u32, flags: gfp_t, new_nsid: *mut ::core::ffi::c_int, new_ifindex: ::core::ffi::c_int, portid: u32, nlh: *const nlmsghdr) -> *mut sk_buff;
    pub fn rtmsg_ifinfo_send(skb: *mut sk_buff, dev: *mut net_device, flags: gfp_t, portid: u32, nlh: *const nlmsghdr);

    /* RTNL is used as a global lock for all changes to network configuration. */
    pub fn rtnl_lock();
    pub fn rtnl_unlock();
    pub fn rtnl_trylock() -> ::core::ffi::c_int;
    pub fn rtnl_is_locked() -> ::core::ffi::c_int;
    pub fn rtnl_lock_interruptible() -> ::core::ffi::c_int;
    pub fn rtnl_lock_killable() -> ::core::ffi::c_int;
    pub fn refcount_dec_and_rtnl_lock(r: *mut refcount_t) -> bool;

    pub static mut netdev_unregistering_wq: wait_queue_head_t;
    pub static mut dev_unreg_count: atomic_t;
    pub static mut pernet_ops_rwsem: rw_semaphore;
    pub static mut net_rwsem: rw_semaphore;
}

#[macro_export]
macro_rules! ASSERT_RTNL { () => { WARN_ONCE!(!$crate::rtnl_is_locked(), "RTNL: assertion failed at {} ({})\n", file!(), line!()) }; }

#[cfg(CONFIG_PROVE_LOCKING)]
extern "C" { pub fn lockdep_rtnl_is_held() -> bool; }
#[cfg(not(CONFIG_PROVE_LOCKING))]
#[inline]
pub fn lockdep_rtnl_is_held() -> bool { true }

#[macro_export]
macro_rules! rcu_dereference_rtnl { ($p:expr) => { rcu_dereference_check!($p, lockdep_rtnl_is_held()) }; }
#[macro_export]
macro_rules! rtnl_dereference { ($p:expr) => { rcu_dereference_protected!($p, lockdep_rtnl_is_held()) }; }
#[macro_export]
macro_rules! rcu_replace_pointer_rtnl { ($rp:expr, $p:expr) => { rcu_replace_pointer!($rp, $p, lockdep_rtnl_is_held()) }; }

#[cfg(CONFIG_DEBUG_NET_SMALL_RTNL)]
extern "C" {
    pub fn __rtnl_net_lock(net: *mut net);
    pub fn __rtnl_net_unlock(net: *mut net);
    pub fn rtnl_net_lock(net: *mut net);
    pub fn rtnl_net_unlock(net: *mut net);
    pub fn rtnl_net_trylock(net: *mut net) -> ::core::ffi::c_int;
    pub fn rtnl_net_lock_killable(net: *mut net) -> ::core::ffi::c_int;
    pub fn rtnl_net_lock_cmp_fn(a: *const lockdep_map, b: *const lockdep_map) -> ::core::ffi::c_int;
    pub fn rtnl_net_is_locked(net: *mut net) -> bool;
    pub fn lockdep_rtnl_net_is_held(net: *mut net) -> bool;
    pub fn rtnl_net_queue_work(net: *mut net);
    pub fn rtnl_net_flush_workqueue();
    pub fn rtnl_net_work_func(work: *mut work_struct);
}

#[macro_export]
macro_rules! ASSERT_RTNL_NET { ($net:expr) => { WARN_ONCE!(!$crate::rtnl_net_is_locked($net), "RTNL_NET: assertion failed at {} ({})\n", file!(), line!()) }; }
#[macro_export]
macro_rules! rcu_dereference_rtnl_net { ($net:expr, $p:expr) => { rcu_dereference_check!($p, lockdep_rtnl_net_is_held($net)) }; }
#[macro_export]
macro_rules! rtnl_net_dereference { ($net:expr, $p:expr) => { rcu_dereference_protected!($p, lockdep_rtnl_net_is_held($net)) }; }
#[macro_export]
macro_rules! rcu_replace_pointer_rtnl_net { ($net:expr, $rp:expr, $p:expr) => { rcu_replace_pointer!($rp, $p, lockdep_rtnl_net_is_held($net)) }; }

#[cfg(not(CONFIG_DEBUG_NET_SMALL_RTNL))]
#[inline] pub unsafe fn __rtnl_net_lock(_net: *mut net) {}
#[cfg(not(CONFIG_DEBUG_NET_SMALL_RTNL))]
#[inline] pub unsafe fn __rtnl_net_unlock(_net: *mut net) {}
#[cfg(not(CONFIG_DEBUG_NET_SMALL_RTNL))]
#[inline] pub unsafe fn rtnl_net_lock(_net: *mut net) { rtnl_lock(); }
#[cfg(not(CONFIG_DEBUG_NET_SMALL_RTNL))]
#[inline] pub unsafe fn rtnl_net_unlock(_net: *mut net) { rtnl_unlock(); }
#[cfg(not(CONFIG_DEBUG_NET_SMALL_RTNL))]
#[inline] pub unsafe fn rtnl_net_trylock(_net: *mut net) -> ::core::ffi::c_int { rtnl_trylock() }
#[cfg(not(CONFIG_DEBUG_NET_SMALL_RTNL))]
#[inline] pub unsafe fn rtnl_net_lock_killable(_net: *mut net) -> ::core::ffi::c_int { rtnl_lock_killable() }
#[cfg(not(CONFIG_DEBUG_NET_SMALL_RTNL))]
#[inline] pub unsafe fn ASSERT_RTNL_NET(_net: *mut net) { ASSERT_RTNL!(); }
#[cfg(not(CONFIG_DEBUG_NET_SMALL_RTNL))]
#[inline] pub unsafe fn rtnl_net_flush_workqueue() {}
#[cfg(not(CONFIG_DEBUG_NET_SMALL_RTNL))]
#[macro_export] macro_rules! rcu_dereference_rtnl_net { ($net:expr, $p:expr) => { rcu_dereference_rtnl!($p) }; }
#[cfg(not(CONFIG_DEBUG_NET_SMALL_RTNL))]
#[macro_export] macro_rules! rtnl_net_dereference { ($net:expr, $p:expr) => { rtnl_dereference!($p) }; }
#[cfg(not(CONFIG_DEBUG_NET_SMALL_RTNL))]
#[macro_export] macro_rules! rcu_replace_pointer_rtnl_net { ($net:expr, $rp:expr, $p:expr) => { rcu_replace_pointer_rtnl!($rp, $p) }; }

#[inline]
pub unsafe fn dev_ingress_queue(dev: *mut net_device) -> *mut netdev_queue { rtnl_dereference!((*dev).ingress_queue) }
#[inline]
pub unsafe fn dev_ingress_queue_rcu(dev: *mut net_device) -> *mut netdev_queue { rcu_dereference!((*dev).ingress_queue) }

extern "C" {
    pub fn dev_ingress_queue_create(dev: *mut net_device) -> *mut netdev_queue;
    #[cfg(CONFIG_NET_INGRESS)] pub fn net_inc_ingress_queue();
    #[cfg(CONFIG_NET_INGRESS)] pub fn net_dec_ingress_queue();
    #[cfg(CONFIG_NET_EGRESS)] pub fn net_inc_egress_queue();
    #[cfg(CONFIG_NET_EGRESS)] pub fn net_dec_egress_queue();
    #[cfg(CONFIG_NET_EGRESS)] pub fn netdev_xmit_skip_txqueue(skip: bool);
    pub fn rtnetlink_init();
    pub fn __rtnl_unlock();
    pub fn rtnl_kfree_skbs(head: *mut sk_buff, tail: *mut sk_buff);
}

/* Shared by rtnl_fdb_dump() and various ndo_fdb_dump() helpers. */
#[repr(C)]
pub struct ndo_fdb_dump_context {
    pub ifindex: ::core::ffi::c_ulong,
    pub fdb_idx: ::core::ffi::c_ulong,
}

extern "C" {
    pub fn ndo_dflt_fdb_dump(skb: *mut sk_buff, cb: *mut netlink_callback, dev: *mut net_device, filter_dev: *mut net_device, idx: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn ndo_dflt_fdb_add(ndm: *mut ndmsg, tb: *mut *mut nlattr, dev: *mut net_device, addr: *const u8, vid: u16, flags: u16) -> ::core::ffi::c_int;
    pub fn ndo_dflt_fdb_del(ndm: *mut ndmsg, tb: *mut *mut nlattr, dev: *mut net_device, addr: *const u8, vid: u16) -> ::core::ffi::c_int;
    pub fn ndo_dflt_bridge_getlink(skb: *mut sk_buff, pid: u32, seq: u32, dev: *mut net_device, mode: u16, flags: u32, mask: u32, nlflags: ::core::ffi::c_int, filter_mask: u32, vlan_fill: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device, u32) -> ::core::ffi::c_int>) -> ::core::ffi::c_int;
    pub fn rtnl_offload_xstats_notify(dev: *mut net_device);
}

#[inline]
pub unsafe fn rtnl_has_listeners(net: *const net, group: u32) -> bool { netlink_has_listeners((*net).rtnl, group) }

#[inline]
pub unsafe fn rtnl_notify_needed(net: *const net, nlflags: u16, group: u32) -> bool { (nlflags & NLM_F_ECHO) != 0 || rtnl_has_listeners(net, group) }

extern "C" { pub fn netif_set_operstate(dev: *mut net_device, newstate: ::core::ffi::c_int); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
