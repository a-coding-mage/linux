/* SPDX-License-Identifier: GPL-2.0 */

pub const NDISC_ROUTER_SOLICITATION: u32 = 133;
pub const NDISC_ROUTER_ADVERTISEMENT: u32 = 134;
pub const NDISC_NEIGHBOUR_SOLICITATION: u32 = 135;
pub const NDISC_NEIGHBOUR_ADVERTISEMENT: u32 = 136;
pub const NDISC_REDIRECT: u32 = 137;

pub const NDISC_NODETYPE_UNSPEC: u32 = 0;
pub const NDISC_NODETYPE_HOST: u32 = 1;
pub const NDISC_NODETYPE_NODEFAULT: u32 = 2;
pub const NDISC_NODETYPE_DEFAULT: u32 = 3;

pub const __ND_OPT_PREFIX_INFO_END: usize = 0;
pub const ND_OPT_SOURCE_LL_ADDR: usize = 1;
pub const ND_OPT_TARGET_LL_ADDR: usize = 2;
pub const ND_OPT_PREFIX_INFO: usize = 3;
pub const ND_OPT_REDIRECT_HDR: usize = 4;
pub const ND_OPT_MTU: usize = 5;
pub const ND_OPT_NONCE: usize = 14;
pub const __ND_OPT_ARRAY_MAX: usize = 15;
pub const ND_OPT_ROUTE_INFO: usize = 24;
pub const ND_OPT_RDNSS: usize = 25;
pub const ND_OPT_DNSSL: usize = 31;
pub const ND_OPT_6CO: usize = 34;
pub const ND_OPT_CAPTIVE_PORTAL: usize = 37;
pub const ND_OPT_PREF64: usize = 38;
pub const __ND_OPT_MAX: usize = 39;

// HZ is supplied by the kernel configuration.
pub const MAX_RTR_SOLICITATION_DELAY: u32 = HZ;
pub const ND_REACHABLE_TIME: u32 = 30 * HZ;
pub const ND_RETRANS_TIMER: u32 = HZ;

#[repr(C)]
pub struct ctl_table;
#[repr(C)]
pub struct inet6_dev;
#[repr(C)]
pub struct net_device;
#[repr(C)]
pub struct net_proto_family;
#[repr(C)]
pub struct sk_buff;
#[repr(C)]
pub struct prefix_info;
#[repr(C)]
pub struct neighbour;
#[repr(C)]
pub struct net;
#[repr(C)]
pub struct icmp6hdr;
#[repr(C)]
pub struct in6_addr { pub s6_addr: [u8; 16] }

pub type __u8 = u8;
pub type __u32 = u32;
pub type u8 = u8;
pub type u32 = u32;
pub type u64 = u64;
pub type __be32 = u32;

extern "C" {
    pub static mut nd_tbl: neigh_table;
}
#[repr(C)] pub struct neigh_table;

#[repr(C)]
pub struct nd_msg { pub icmph: icmp6hdr, pub target: in6_addr, pub opt: [u8; 0] }
#[repr(C)]
pub struct rs_msg { pub icmph: icmp6hdr, pub opt: [u8; 0] }
#[repr(C)]
pub struct ra_msg { pub icmph: icmp6hdr, pub reachable_time: __be32, pub retrans_timer: __be32 }
#[repr(C)]
pub struct rd_msg { pub icmph: icmp6hdr, pub target: in6_addr, pub dest: in6_addr, pub opt: [u8; 0] }

#[repr(C, packed)]
pub struct nd_opt_hdr { pub nd_opt_type: u8, pub nd_opt_len: u8 }

#[repr(C)]
pub struct ndisc_options {
    pub nd_opt_array: [*mut nd_opt_hdr; __ND_OPT_ARRAY_MAX],
    // CONFIG_IPV6_ROUTE_INFO fields, when enabled
    pub nd_opts_ri: *mut nd_opt_hdr,
    pub nd_opts_ri_end: *mut nd_opt_hdr,
    pub nd_useropts: *mut nd_opt_hdr,
    pub nd_useropts_end: *mut nd_opt_hdr,
    // CONFIG_IEEE802154_6LOWPAN field, when enabled
    pub nd_802154_opt_array: [*mut nd_opt_hdr; ND_OPT_TARGET_LL_ADDR + 1],
}

pub const NDISC_OPS_REDIRECT_DATA_SPACE: usize = 2;

#[repr(C)]
pub struct ndisc_ops {
    pub parse_options: Option<unsafe extern "C" fn(*const net_device, *mut nd_opt_hdr, *mut ndisc_options) -> i32>,
    pub update: Option<unsafe extern "C" fn(*const net_device, *mut neighbour, u32, u8, *const ndisc_options)>,
    pub opt_addr_space: Option<unsafe extern "C" fn(*const net_device, u8, *mut neighbour, *mut u8, *mut *mut u8) -> i32>,
    pub fill_addr_option: Option<unsafe extern "C" fn(*const net_device, *mut sk_buff, u8, *const u8)>,
    pub prefix_rcv_add_addr: Option<unsafe extern "C" fn(*mut net, *mut net_device, *const prefix_info, *mut inet6_dev, *mut in6_addr, i32, u32, bool, bool, u32, u32, bool)>,
}

pub const fn ndisc_opt_space(len: usize) -> usize { (len + 2 + 7) & !7 }

// Inline operations below retain the original control-flow and require the
// corresponding kernel object fields and helper functions from other headers.
pub unsafe fn ndisc_addr_option_pad(_type: u16) -> i32 {
    if _type == ARPHRD_INFINIBAND { 2 } else { 0 }
}
pub unsafe fn __ndisc_opt_addr_space(addr_len: u8, pad: i32) -> i32 {
    ndisc_opt_space((addr_len as i32 + pad) as usize) as i32
}
pub unsafe fn __ndisc_opt_addr_data(p: *mut nd_opt_hdr, addr_len: u8, prepad: i32) -> *mut u8 {
    let lladdr = (p as *mut u8).add(core::mem::size_of::<nd_opt_hdr>());
    let lladdrlen = ((*p).nd_opt_len as i32) << 3;
    if lladdrlen != __ndisc_opt_addr_space(addr_len, prepad) { core::ptr::null_mut() } else { lladdr.add(prepad as usize) }
}

extern "C" {
    pub fn ndisc_parse_options(dev: *const net_device, opt: *mut u8, opt_len: i32, ndopts: *mut ndisc_options) -> *mut ndisc_options;
    pub fn __ndisc_fill_addr_option(skb: *mut sk_buff, type_: i32, data: *const core::ffi::c_void, data_len: i32, pad: i32);
    pub fn ndisc_init() -> i32;
    pub fn ndisc_late_init() -> i32;
    pub fn ndisc_late_cleanup();
    pub fn ndisc_cleanup();
    pub fn ndisc_rcv(skb: *mut sk_buff) -> skb_drop_reason;
    pub fn ndisc_ns_create(dev: *mut net_device, solicit: *const in6_addr, saddr: *const in6_addr, nonce: u64) -> *mut sk_buff;
    pub fn ndisc_send_ns(dev: *mut net_device, solicit: *const in6_addr, daddr: *const in6_addr, saddr: *const in6_addr, nonce: u64);
    pub fn ndisc_send_skb(skb: *mut sk_buff, daddr: *const in6_addr, saddr: *const in6_addr);
    pub fn ndisc_send_rs(dev: *mut net_device, saddr: *const in6_addr, daddr: *const in6_addr);
    pub fn ndisc_send_na(dev: *mut net_device, daddr: *const in6_addr, solicited_addr: *const in6_addr, router: bool, solicited: bool, override_: bool, inc_opt: bool);
    pub fn ndisc_send_redirect(skb: *mut sk_buff, target: *const in6_addr);
    pub fn ndisc_mc_map(addr: *const in6_addr, buf: *mut i8, dev: *mut net_device, dir: i32) -> i32;
    pub fn ndisc_update(dev: *const net_device, neigh: *mut neighbour, lladdr: *const u8, new_: u8, flags: u32, icmp6_type: u8, ndopts: *mut ndisc_options);
    pub fn ndisc_check_ns_na(skb: *mut sk_buff) -> i32;
    pub fn igmp6_init() -> i32;
    pub fn igmp6_late_init() -> i32;
    pub fn igmp6_cleanup();
    pub fn igmp6_late_cleanup();
    pub fn igmp6_event_query(skb: *mut sk_buff);
    pub fn igmp6_event_report(skb: *mut sk_buff);
    pub fn inet6_ifinfo_notify(event: i32, idev: *mut inet6_dev);
}

#[repr(C)] pub struct skb_drop_reason;
extern "C" { pub static ARPHRD_INFINIBAND: u16; pub static HZ: u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
