/* SPDX-License-Identifier: GPL-2.0 */

pub const MAX_RTR_SOLICITATIONS: i32 = -1; /* unlimited */
pub const RTR_SOLICITATION_INTERVAL: u64 = 4 * HZ;
pub const RTR_SOLICITATION_MAX_INTERVAL: u64 = 3600 * HZ; /* 1 hour */

pub const MIN_VALID_LIFETIME: u32 = 2 * 3600; /* 2 hours */

/* TEMP_VALID_LIFETIME default value as specified in RFC 8981 3.8 */
pub const TEMP_VALID_LIFETIME: u32 = 2 * 86400; /* 2 days */
pub const TEMP_PREFERRED_LIFETIME: u32 = 86400; /* 24 hours */
pub const REGEN_MIN_ADVANCE: u32 = 2; /* 2 seconds */
pub const REGEN_MAX_RETRY: u32 = 3;
pub const MAX_DESYNC_FACTOR: u32 = 600;

pub const ADDR_CHECK_FREQUENCY: u64 = 120 * HZ;
pub const IPV6_MAX_ADDRESSES: u32 = 16;
pub const ADDRCONF_TIMER_FUZZ_MINUS: u64 = if HZ > 50 { HZ / 50 } else { 1 };
pub const ADDRCONF_TIMER_FUZZ: u64 = HZ / 4;
pub const ADDRCONF_TIMER_FUZZ_MAX: u64 = HZ;
pub const ADDRCONF_NOTIFY_PRIORITY: i32 = 0;

#[repr(C, packed)]
pub union PrefixInfoFlags {
    pub flags: __u8,
    /* C bitfields occupy the same single byte; use the raw representation. */
    pub bits: __u8,
}

#[repr(C, packed)]
pub struct prefix_info {
    pub type_: __u8,
    pub length: __u8,
    pub prefix_len: __u8,
    pub flags: PrefixInfoFlags,
    pub valid: __be32,
    pub prefered: __be32,
    pub reserved2: __be32,
    pub prefix: in6_addr,
}

/* rfc4861 4.6.2: IPv6 PIO is 32 bytes in size */

#[repr(C)]
pub struct in6_validator_info {
    pub i6vi_addr: in6_addr,
    pub i6vi_dev: *mut inet6_dev,
    pub extack: *mut netlink_ext_ack,
}

#[repr(C)]
pub struct ifa6_config {
    pub pfx: *const in6_addr,
    pub plen: c_uint,
    pub ifa_proto: u8,
    pub peer_pfx: *const in6_addr,
    pub rt_priority: u32,
    pub ifa_flags: u32,
    pub preferred_lft: u32,
    pub valid_lft: u32,
    pub scope: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum addr_type_t {
    UNICAST_ADDR,
    MULTICAST_ADDR,
    ANYCAST_ADDR,
}

#[repr(C)]
pub struct inet6_fill_args {
    pub portid: u32,
    pub seq: u32,
    pub event: c_int,
    pub flags: c_uint,
    pub netnsid: c_int,
    pub ifindex: c_int,
    pub type_: addr_type_t,
    pub force_rt_scope_universe: bool,
}

extern "C" {
    pub fn addrconf_init() -> c_int;
    pub fn addrconf_cleanup();
    pub fn addrconf_add_ifaddr(net: *mut net, arg: *mut c_void) -> c_int;
    pub fn addrconf_del_ifaddr(net: *mut net, arg: *mut c_void) -> c_int;
    pub fn addrconf_set_dstaddr(net: *mut net, arg: *mut c_void) -> c_int;
    pub fn ipv6_chk_addr(net: *mut net, addr: *const in6_addr, dev: *const net_device, strict: c_int) -> c_int;
    pub fn ipv6_chk_addr_and_flags(net: *mut net, addr: *const in6_addr, dev: *const net_device, skip_dev_check: bool, strict: c_int, banned_flags: u32) -> c_int;
    /* Present when CONFIG_IPV6_MIP6 or CONFIG_IPV6_MIP6_MODULE is enabled. */
    #[cfg(any(CONFIG_IPV6_MIP6, CONFIG_IPV6_MIP6_MODULE))]
    pub fn ipv6_chk_home_addr(net: *mut net, addr: *const in6_addr) -> c_int;
    pub fn ipv6_chk_rpl_srh_loop(net: *mut net, segs: *const in6_addr, nsegs: u8) -> c_int;
    pub fn ipv6_chk_custom_prefix(addr: *const in6_addr, prefix_len: c_uint, dev: *mut net_device) -> bool;
    pub fn ipv6_chk_prefix(addr: *const in6_addr, dev: *mut net_device) -> c_int;
    pub fn ipv6_dev_find(net: *mut net, addr: *const in6_addr, dev: *mut net_device) -> *mut net_device;
    pub fn ipv6_get_ifaddr(net: *mut net, addr: *const in6_addr, dev: *mut net_device, strict: c_int) -> *mut inet6_ifaddr;
    pub fn ipv6_dev_get_saddr(net: *mut net, dev: *const net_device, daddr: *const in6_addr, srcprefs: c_uint, saddr: *mut in6_addr) -> c_int;
    pub fn ipv6_get_lladdr(dev: *mut net_device, addr: *mut in6_addr, banned_flags: u32) -> c_int;
    pub fn inet_rcv_saddr_equal(sk: *const sock, sk2: *const sock, match_wildcard: bool) -> bool;
    pub fn inet_rcv_saddr_any(sk: *const sock) -> bool;
    pub fn addrconf_join_solict(dev: *mut net_device, addr: *const in6_addr);
    pub fn addrconf_leave_solict(idev: *mut inet6_dev, addr: *const in6_addr);
    pub fn addrconf_add_linklocal(idev: *mut inet6_dev, addr: *const in6_addr, flags: u32);
    pub fn addrconf_prefix_rcv_add_addr(net: *mut net, dev: *mut net_device, pinfo: *const prefix_info, in6_dev: *mut inet6_dev, addr: *const in6_addr, addr_type: c_int, addr_flags: u32, sllao: bool, tokenized: bool, valid_lft: __u32, prefered_lft: u32) -> c_int;
    pub fn ipv6_addr_label_init() -> c_int;
    pub fn ipv6_addr_label_cleanup();
    pub fn ipv6_addr_label_rtnl_register() -> c_int;
    pub fn ipv6_addr_label(net: *mut net, addr: *const in6_addr, type_: c_int, ifindex: c_int) -> u32;
}

#[inline]
pub unsafe fn addrconf_addr_eui48_base(eui: *mut u8, addr: *const c_char) {
    memcpy(eui as *mut c_void, addr as *const c_void, 3);
    *eui.add(3) = 0xFF;
    *eui.add(4) = 0xFE;
    memcpy(eui.add(5) as *mut c_void, addr.add(3) as *const c_void, 3);
}

#[inline]
pub unsafe fn addrconf_addr_eui48(eui: *mut u8, addr: *const c_char) {
    addrconf_addr_eui48_base(eui, addr);
    *eui ^= 2;
}

#[inline]
pub unsafe fn addrconf_ifid_eui48(eui: *mut u8, dev: *mut net_device) -> c_int {
    if (*dev).addr_len != ETH_ALEN { return -1; }

    /*
     * The zSeries OSA network cards can be shared among various OS instances,
     * but the OSA cards have only one MAC address. This leads to duplicate
     * address conflicts in conjunction with IPv6 if more than one instance
     * uses the same card.
     *
     * The driver for these cards can deliver a unique 16-bit identifier for
     * each instance sharing the same card. It is placed instead of 0xFFFE in
     * the interface identifier. The "u" bit of the interface identifier is
     * not inverted in this case. Hence the resulting interface identifier has
     * local scope according to RFC2373.
     */
    addrconf_addr_eui48_base(eui, (*dev).dev_addr);
    if (*dev).dev_id != 0 {
        *eui.add(3) = ((*dev).dev_id >> 8) as u8;
        *eui.add(4) = (*dev).dev_id as u8;
    } else {
        *eui ^= 2;
    }
    0
}

pub const INFINITY_LIFE_TIME: u32 = 0xFFFFFFFF;

#[inline]
pub fn addrconf_timeout_fixup(timeout: u32, unit: c_uint) -> c_ulong {
    if timeout == INFINITY_LIFE_TIME { return !0 as c_ulong; }
    if 0xfffffffe > LONG_MAX / unit && timeout as c_ulong > LONG_MAX / unit {
        return LONG_MAX / unit;
    }
    timeout as c_ulong
}

#[inline]
pub fn addrconf_finite_timeout(timeout: c_ulong) -> c_int { (!timeout) as c_int }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
