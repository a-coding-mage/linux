/* SPDX-License-Identifier: GPL-2.0 */
/* C header guard and __KERNEL__ conditional omitted; declarations retained. */

pub const MC_HASH_SZ_LOG: usize = 9;

#[repr(C)]
pub struct ipv4_devconf {
    pub sysctl: *mut core::ffi::c_void,
    pub data: [core::ffi::c_int; IPV4_DEVCONF_MAX],
    pub state: [core::ffi::c_ulong; BITS_TO_LONGS(IPV4_DEVCONF_MAX)],
}

#[repr(C)]
pub struct in_device {
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub refcnt: refcount_t,
    pub dead: core::ffi::c_int,
    pub ifa_list: *mut in_ifaddr,
    pub mc_list: *mut ip_mc_list,
    pub mc_hash: *mut *mut ip_mc_list,
    pub mc_count: core::ffi::c_int,
    pub mc_tomb_lock: spinlock_t,
    pub mc_tomb: *mut ip_mc_list,
    pub mr_v1_seen: core::ffi::c_ulong,
    pub mr_v2_seen: core::ffi::c_ulong,
    pub mr_qi: core::ffi::c_ulong,
    pub mr_qri: core::ffi::c_ulong,
    pub mr_qrv: u8,
    pub mr_gq_running: u8,
    pub mr_maxdelay: u32,
    pub mr_ifc_count: u32,
    pub mr_gq_timer: timer_list,
    pub mr_ifc_timer: timer_list,
    pub arp_parms: *mut neigh_parms,
    pub cnf: ipv4_devconf,
    pub rcu_head: rcu_head,
}

#[repr(C)]
pub struct in_ifaddr {
    pub addr_lst: hlist_node,
    pub ifa_next: *mut in_ifaddr,
    pub ifa_dev: *mut in_device,
    pub rcu_head: rcu_head,
    pub ifa_local: __be32,
    pub ifa_address: __be32,
    pub ifa_mask: __be32,
    pub ifa_rt_priority: __u32,
    pub ifa_broadcast: __be32,
    pub ifa_scope: u8,
    pub ifa_prefixlen: u8,
    pub ifa_proto: u8,
    pub ifa_flags: __u32,
    pub ifa_label: [core::ffi::c_char; IFNAMSIZ],
    pub ifa_valid_lft: __u32,
    pub ifa_preferred_lft: __u32,
    pub ifa_cstamp: core::ffi::c_ulong,
    pub ifa_tstamp: core::ffi::c_ulong,
}

#[repr(C)]
pub struct in_validator_info {
    pub ivi_addr: __be32,
    pub ivi_dev: *mut in_device,
    pub extack: *mut netlink_ext_ack,
}

#[inline]
pub unsafe fn ipv4_devconf_get(in_dev: *const in_device, index: core::ffi::c_int) -> core::ffi::c_int {
    core::ptr::read_volatile((*in_dev).cnf.data.as_ptr().add((index - 1) as usize))
}

#[inline]
pub unsafe fn ipv4_devconf_set(in_dev: *mut in_device, index: core::ffi::c_int, val: core::ffi::c_int) {
    set_bit((index - 1) as usize, (*in_dev).cnf.state.as_mut_ptr());
    core::ptr::write_volatile((*in_dev).cnf.data.as_mut_ptr().add((index - 1) as usize), val);
}

#[inline]
pub unsafe fn ipv4_devconf_setall(in_dev: *mut in_device) {
    bitmap_fill((*in_dev).cnf.state.as_mut_ptr(), IPV4_DEVCONF_MAX);
}

#[inline]
pub unsafe fn ip_dev_find(net: *mut net, addr: __be32) -> *mut net_device {
    __ip_dev_find(net, addr, true)
}

#[inline]
pub unsafe fn inet_ifa_match(addr: __be32, ifa: *const in_ifaddr) -> bool {
    !((addr ^ (*ifa).ifa_address) & (*ifa).ifa_mask != 0)
}

/* Check if a mask is acceptable. */
#[inline]
pub unsafe fn bad_mask(mut mask: __be32, addr: __be32) -> bool {
    mask = !mask;
    if addr & mask != 0 { return true; }
    let hmask = ntohl(mask);
    (hmask & (hmask + 1)) != 0
}

#[inline]
pub unsafe fn __in_dev_get_rcu(dev: *const net_device) -> *mut in_device {
    rcu_dereference((*dev).ip_ptr)
}

#[inline]
pub unsafe fn in_dev_get(dev: *const net_device) -> *mut in_device {
    rcu_read_lock();
    let in_dev = __in_dev_get_rcu(dev);
    if !in_dev.is_null() { refcount_inc(&mut (*in_dev).refcnt); }
    rcu_read_unlock();
    in_dev
}

#[inline]
pub unsafe fn __in_dev_get_rtnl(dev: *const net_device) -> *mut in_device { rtnl_dereference((*dev).ip_ptr) }
#[inline]
pub unsafe fn __in_dev_get_rtnl_net(dev: *const net_device) -> *mut in_device { rtnl_net_dereference(dev_net(dev), (*dev).ip_ptr) }

#[inline]
pub unsafe fn ip_ignore_linkdown(dev: *const net_device) -> bool {
    let in_dev = rcu_dereference_rtnl((*dev).ip_ptr);
    !in_dev.is_null() && IN_DEV_IGNORE_ROUTES_WITH_LINKDOWN(in_dev)
}

#[inline]
pub unsafe fn __in_dev_arp_parms_get_rcu(dev: *const net_device) -> *mut neigh_parms {
    let in_dev = __in_dev_get_rcu(dev);
    if in_dev.is_null() { core::ptr::null_mut() } else { (*in_dev).arp_parms }
}

#[inline]
pub unsafe fn in_dev_put(idev: *mut in_device) {
    if refcount_dec_and_test(&mut (*idev).refcnt) { in_dev_finish_destroy(idev); }
}
#[inline]
pub unsafe fn __in_dev_put(idev: *mut in_device) { refcount_dec(&mut (*idev).refcnt); }
#[inline]
pub unsafe fn in_dev_hold(idev: *mut in_device) { refcount_inc(&mut (*idev).refcnt); }
#[inline]
pub unsafe fn in_dev_hold_safe(idev: *mut in_device) -> bool { refcount_inc_not_zero(&mut (*idev).refcnt) }

#[inline]
pub fn inet_make_mask(logmask: core::ffi::c_int) -> __be32 {
    if logmask != 0 { htonl(!((1u32 << (32 - logmask)) - 1)) } else { 0 }
}

#[inline]
pub fn inet_mask_len(mask: __be32) -> core::ffi::c_int {
    let hmask = ntohl(mask);
    if hmask == 0 { 0 } else { 32 - ffz(!hmask) as core::ffi::c_int }
}

extern "C" {
    pub fn register_inetaddr_notifier(nb: *mut notifier_block) -> core::ffi::c_int;
    pub fn unregister_inetaddr_notifier(nb: *mut notifier_block) -> core::ffi::c_int;
    pub fn register_inetaddr_validator_notifier(nb: *mut notifier_block) -> core::ffi::c_int;
    pub fn unregister_inetaddr_validator_notifier(nb: *mut notifier_block) -> core::ffi::c_int;
    pub fn inet_netconf_notify_devconf(net: *mut net, event: core::ffi::c_int, type_: core::ffi::c_int, ifindex: core::ffi::c_int, devconf: *mut ipv4_devconf);
    pub fn __ip_dev_find(net: *mut net, addr: __be32, devref: bool) -> *mut net_device;
    pub fn inet_addr_onlink(in_dev: *mut in_device, a: __be32, b: __be32) -> core::ffi::c_int;
    pub fn devinet_ioctl(net: *mut net, cmd: core::ffi::c_uint, ifr: *mut ifreq) -> core::ffi::c_int;
    pub fn devinet_init();
    pub fn inetdev_by_index(net: *mut net, index: core::ffi::c_int) -> *mut in_device;
    pub fn inet_select_addr(dev: *const net_device, dst: __be32, scope: core::ffi::c_int) -> __be32;
    pub fn inet_confirm_addr(net: *mut net, in_dev: *mut in_device, dst: __be32, local: __be32, scope: core::ffi::c_int) -> __be32;
    pub fn inet_ifa_byprefix(in_dev: *mut in_device, prefix: __be32, mask: __be32) -> *mut in_ifaddr;
    pub fn inet_lookup_ifaddr_rcu(net: *mut net, addr: __be32) -> *mut in_ifaddr;
    pub fn in_dev_finish_destroy(idev: *mut in_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
