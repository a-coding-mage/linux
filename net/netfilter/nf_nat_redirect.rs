// SPDX-License-Identifier: GPL-2.0-only
/*
 * (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2006 Netfilter Core Team <coreteam@netfilter.org>
 * Copyright (c) 2011 Patrick McHardy <kaber@trash.net>
 *
 * Based on Rusty Russell's IPv4 REDIRECT target. Development of IPv6
 * NAT funded by Astaro.
 */

// Declarations and constants supplied by the Linux kernel dependencies.

#[repr(C)]
pub union nf_inet_addr {
    pub ip: u32,
    pub in6: in6_addr,
}

#[repr(C)]
pub struct nf_nat_range2 {
    pub flags: u32,
    pub min_addr: nf_inet_addr,
    pub max_addr: nf_inet_addr,
    pub min_proto: nf_conntrack_man_proto,
    pub max_proto: nf_conntrack_man_proto,
}

extern "C" {
    fn nf_ct_get(skb: *mut sk_buff, ctinfo: *mut ip_conntrack_info) -> *mut nf_conn;
    fn nf_nat_setup_info(
        ct: *mut nf_conn,
        range: *const nf_nat_range2,
        manip: u32,
    ) -> u32;
    fn __in_dev_get_rcu(dev: *mut net_device) -> *const in_device;
    fn rcu_dereference<T>(ptr: *const T) -> *const T;
    fn __in6_dev_get(dev: *mut net_device) -> *mut inet6_dev;
    fn ipv6_addr_type(addr: *const in6_addr) -> u32;
    fn ipv6_addr_scope(addr: *const in6_addr) -> u32;
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ip6_hdr;
}

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct nf_conntrack_man_proto {
    pub all: [u16; 4],
}

#[repr(C)]
pub struct sk_buff {
    pub dev: *mut net_device,
}

#[repr(C)]
pub struct net_device;
#[repr(C)]
pub struct nf_conn;
#[repr(C)]
pub struct in_device {
    pub ifa_list: *mut in_ifaddr,
}
#[repr(C)]
pub struct in_ifaddr {
    pub ifa_local: u32,
}
#[repr(C)]
pub struct inet6_ifaddr {
    pub addr: in6_addr,
    pub flags: u32,
}
#[repr(C)]
pub struct inet6_dev {
    pub addr_list: list_head,
    pub lock: raw_spinlock,
}
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[repr(C)]
pub struct raw_spinlock;
#[repr(C)]
pub struct ip6_hdr {
    pub daddr: in6_addr,
}

pub type ip_conntrack_info = u32;

pub const NF_NAT_RANGE_MAP_IPS: u32 = 1 << 0;
pub const NF_NAT_MANIP_DST: u32 = 1;
pub const NF_INET_PRE_ROUTING: u32 = 0;
pub const NF_INET_LOCAL_OUT: u32 = 3;
pub const NF_DROP: u32 = 0;
pub const INADDR_LOOPBACK: u32 = 0x7f000001;
pub const IPV6_ADDR_MAPPED: u32 = 0x0001;
pub const IFA_F_TENTATIVE: u32 = 0x40;
pub const IFA_F_OPTIMISTIC: u32 = 0x04;
pub const IPV6_ADDR_SCOPE_MASK: u32 = 0x0c00;

static loopback_addr: in6_addr = in6_addr { s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1] };

unsafe fn nf_nat_redirect(
    skb: *mut sk_buff,
    range: *const nf_nat_range2,
    newdst: *const nf_inet_addr,
) -> u32 {
    let mut newrange: nf_nat_range2 = core::mem::zeroed();
    let mut ctinfo: ip_conntrack_info = 0;
    let ct = nf_ct_get(skb, &mut ctinfo);

    newrange.flags = (*range).flags | NF_NAT_RANGE_MAP_IPS;
    newrange.min_addr = *newdst;
    newrange.max_addr = *newdst;
    newrange.min_proto = (*range).min_proto;
    newrange.max_proto = (*range).max_proto;

    nf_nat_setup_info(ct, &newrange, NF_NAT_MANIP_DST)
}

pub unsafe fn nf_nat_redirect_ipv4(
    skb: *mut sk_buff,
    range: *const nf_nat_range2,
    hooknum: u32,
) -> u32 {
    let mut newdst: nf_inet_addr = core::mem::zeroed();

    if hooknum == NF_INET_LOCAL_OUT {
        newdst.ip = INADDR_LOOPBACK.to_be();
    } else {
        let indev = __in_dev_get_rcu((*skb).dev);
        if !indev.is_null() {
            let ifa = rcu_dereference((*indev).ifa_list);
            if !ifa.is_null() {
                newdst.ip = (*ifa).ifa_local;
            }
        }

        if newdst.ip == 0 {
            return NF_DROP;
        }
    }

    nf_nat_redirect(skb, range, &newdst)
}

unsafe fn nf_nat_redirect_ipv6_usable(ifa: *const inet6_ifaddr, scope: u32) -> bool {
    let ifa_addr_type = ipv6_addr_type(&(*ifa).addr);

    if ifa_addr_type & IPV6_ADDR_MAPPED != 0 {
        return false;
    }

    if ((*ifa).flags & IFA_F_TENTATIVE != 0) && ((*ifa).flags & IFA_F_OPTIMISTIC == 0) {
        return false;
    }

    if scope != 0 {
        let ifa_scope = ifa_addr_type & IPV6_ADDR_SCOPE_MASK;
        if scope & ifa_scope == 0 {
            return false;
        }
    }

    true
}

pub unsafe fn nf_nat_redirect_ipv6(
    skb: *mut sk_buff,
    range: *const nf_nat_range2,
    hooknum: u32,
) -> u32 {
    let mut newdst: nf_inet_addr = core::mem::zeroed();

    if hooknum == NF_INET_LOCAL_OUT {
        newdst.in6 = loopback_addr;
    } else {
        let scope = ipv6_addr_scope(&(*ipv6_hdr(skb)).daddr);
        let idev = __in6_dev_get((*skb).dev);
        let mut addr = false;

        if !idev.is_null() {
            let mut ifa = (*idev).addr_list.next as *const inet6_ifaddr;
            while !ifa.is_null() {
                if nf_nat_redirect_ipv6_usable(ifa, scope) {
                    newdst.in6 = (*ifa).addr;
                    addr = true;
                    break;
                }
                ifa = (*(ifa as *const inet6_ifaddr as *const list_head)).next as *const inet6_ifaddr;
            }
        }

        if !addr {
            return NF_DROP;
        }
    }

    nf_nat_redirect(skb, range, &newdst)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
