/* SPDX-License-Identifier: GPL-2.0-or-later */
/* inet6 interface/address list definitions */

// Dependencies supplied by the surrounding kernel translation.

pub const IF_RA_OTHERCONF: u32 = 0x80;
pub const IF_RA_MANAGED: u32 = 0x40;
pub const IF_RA_RCVD: u32 = 0x20;
pub const IF_RS_SENT: u32 = 0x10;
pub const IF_READY: u32 = 0x80000000;

pub const INET6_IFADDR_STATE_PREDAD: i32 = 0;
pub const INET6_IFADDR_STATE_DAD: i32 = 1;
pub const INET6_IFADDR_STATE_POSTDAD: i32 = 2;
pub const INET6_IFADDR_STATE_ERRDAD: i32 = 3;
pub const INET6_IFADDR_STATE_DEAD: i32 = 4;

#[repr(C)]
pub struct inet6_ifaddr {
    pub addr: in6_addr,
    pub prefix_len: u32,
    pub rt_priority: u32,
    pub valid_lft: u32,
    pub prefered_lft: u32,
    pub refcnt: refcount_t,
    pub lock: spinlock_t,
    pub state: i32,
    pub flags: u32,
    pub dad_probes: u8,
    pub stable_privacy_retry: u8,
    pub scope: u16,
    pub dad_nonce: u64,
    pub cstamp: usize,
    pub tstamp: usize,
    pub dad_work: delayed_work,
    pub idev: *mut inet6_dev,
    pub rt: *mut fib6_info,
    pub addr_lst: hlist_node,
    pub if_list: list_head,
    pub if_list_aux: list_head,
    pub tmp_list: list_head,
    pub ifpub: *mut inet6_ifaddr,
    pub regen_count: i32,
    pub tokenized: bool,
    pub ifa_proto: u8,
    pub rcu: rcu_head,
    pub peer_addr: in6_addr,
}

#[repr(C)]
pub struct ip6_sf_socklist {
    pub sl_max: u32,
    pub sl_count: u32,
    pub rcu: rcu_head,
    pub sl_addr: [in6_addr; 0],
}

pub const IP6_SFBLOCK: u32 = 10;

#[repr(C)]
pub struct ipv6_mc_socklist {
    pub addr: in6_addr,
    pub ifindex: i32,
    pub sfmode: u32,
    pub next: *mut ipv6_mc_socklist,
    pub sflist: *mut ip6_sf_socklist,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct ip6_sf_list {
    pub sf_next: *mut ip6_sf_list,
    pub sf_addr: in6_addr,
    pub sf_count: [usize; 2],
    pub sf_gsresp: u8,
    pub sf_oldin: u8,
    pub sf_crcount: u8,
    pub rcu: rcu_head,
}

pub const MAF_TIMER_RUNNING: u32 = 0x01;
pub const MAF_LAST_REPORTER: u32 = 0x02;
pub const MAF_LOADED: u32 = 0x04;
pub const MAF_NOREPORT: u32 = 0x08;
pub const MAF_GSQUERY: u32 = 0x10;

#[repr(C)]
pub struct ifmcaddr6 {
    pub mca_addr: in6_addr,
    pub idev: *mut inet6_dev,
    pub next: *mut ifmcaddr6,
    pub mca_sources: *mut ip6_sf_list,
    pub mca_tomb: *mut ip6_sf_list,
    pub mca_sfmode: u32,
    pub mca_crcount: u8,
    pub mca_sfcount: [usize; 2],
    pub mca_work: delayed_work,
    pub mca_flags: u32,
    pub mca_users: i32,
    pub mca_refcnt: refcount_t,
    pub mca_cstamp: usize,
    pub mca_tstamp: usize,
    pub rcu: rcu_head,
}

#[repr(C)]
pub struct ipv6_ac_socklist {
    pub acl_addr: in6_addr,
    pub acl_ifindex: i32,
    pub acl_next: *mut ipv6_ac_socklist,
}

#[repr(C)]
pub struct ifacaddr6 {
    pub aca_addr: in6_addr,
    pub aca_rt: *mut fib6_info,
    pub aca_next: *mut ifacaddr6,
    pub aca_addr_lst: hlist_node,
    pub aca_users: i32,
    pub aca_refcnt: refcount_t,
    pub aca_cstamp: usize,
    pub aca_tstamp: usize,
    pub rcu: rcu_head,
}

pub const IFA_HOST: u32 = IPV6_ADDR_LOOPBACK;
pub const IFA_LINK: u32 = IPV6_ADDR_LINKLOCAL;
pub const IFA_SITE: u32 = IPV6_ADDR_SITELOCAL;

#[repr(C)]
pub struct ipv6_devstat {
    pub proc_dir_entry: *mut proc_dir_entry,
    pub ipv6: ipstats_mib,
    pub icmpv6dev: icmpv6_mib_device,
    pub icmpv6msgdev: icmpv6msg_mib_device,
}

#[repr(C)]
pub struct inet6_dev {
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub addr_list: list_head,
    pub mc_list: *mut ifmcaddr6,
    pub mc_tomb: *mut ifmcaddr6,
    pub mc_qrv: u8,
    pub mc_gq_running: u8,
    pub mc_ifc_count: u8,
    pub mc_dad_count: u8,
    pub mc_v1_seen: usize,
    pub mc_qi: usize,
    pub mc_qri: usize,
    pub mc_maxdelay: usize,
    pub mc_gq_work: delayed_work,
    pub mc_ifc_work: delayed_work,
    pub mc_dad_work: delayed_work,
    pub mc_query_work: delayed_work,
    pub mc_report_work: delayed_work,
    pub mc_query_queue: sk_buff_head,
    pub mc_report_queue: sk_buff_head,
    pub mc_query_lock: spinlock_t,
    pub mc_report_lock: spinlock_t,
    pub mc_lock: mutex,
    pub ac_list: *mut ifacaddr6,
    pub lock: rwlock_t,
    pub refcnt: refcount_t,
    pub if_flags: u32,
    pub dead: i32,
    pub desync_factor: u32,
    pub tempaddr_list: list_head,
    pub token: in6_addr,
    pub nd_parms: *mut neigh_parms,
    pub cnf: ipv6_devconf,
    pub stats: ipv6_devstat,
    pub rs_timer: timer_list,
    pub rs_interval: i32,
    pub rs_probes: u8,
    pub tstamp: usize,
    pub rcu: rcu_head,
    pub ra_mtu: u32,
}

pub unsafe fn ipv6_eth_mc_map(addr: *const in6_addr, buf: *mut i8) {
    *buf.add(0) = 0x33;
    *buf.add(1) = 0x33;
    core::ptr::copy_nonoverlapping(((*addr).s6_addr32.as_ptr().add(3)) as *const u8, buf.add(2) as *mut u8, core::mem::size_of::<u32>());
}

pub unsafe fn ipv6_arcnet_mc_map(_addr: *const in6_addr, _buf: *mut i8) {}

pub unsafe fn ipv6_ib_mc_map(addr: *const in6_addr, broadcast: *const u8, buf: *mut i8) {
    let scope = *broadcast.add(5) & 0xF;
    *buf.add(0) = 0;
    *buf.add(1) = 0xffu8 as i8;
    *buf.add(2) = 0xffu8 as i8;
    *buf.add(3) = 0xffu8 as i8;
    *buf.add(4) = 0xffu8 as i8;
    *buf.add(5) = (0x10 | scope) as i8;
    *buf.add(6) = 0x60;
    *buf.add(7) = 0x1b;
    *buf.add(8) = *broadcast.add(8) as i8;
    *buf.add(9) = *broadcast.add(9) as i8;
    core::ptr::copy_nonoverlapping((*addr).s6_addr.as_ptr().add(6), buf.add(10) as *mut u8, 10);
}

pub unsafe fn ipv6_ipgre_mc_map(addr: *const in6_addr, broadcast: *const u8, buf: *mut i8) -> i32 {
    if (*broadcast.add(0) | *broadcast.add(1) | *broadcast.add(2) | *broadcast.add(3)) != 0 {
        core::ptr::copy_nonoverlapping(broadcast, buf as *mut u8, 4);
    } else {
        if ((*addr).s6_addr32[0] | (*addr).s6_addr32[1] | ((*addr).s6_addr32[2] ^ htonl(0x0000ffff))) != 0 {
            return -EINVAL;
        }
        core::ptr::copy_nonoverlapping(((*addr).s6_addr32.as_ptr().add(3)) as *const u8, buf as *mut u8, 4);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
