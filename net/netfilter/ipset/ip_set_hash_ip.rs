// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2003-2013 Jozsef Kadlecsik <kadlec@netfilter.org> */
/* Kernel module implementing an IP set type: the hash:ip type */
// C dependencies and generated ip_set_hash_gen.h declarations are supplied externally.

pub const IPSET_TYPE_REV_MIN: u32 = 0;
pub const IPSET_TYPE_REV_MAX: u32 = 6;

// Type specific function prefix: hash_ip
// IP_SET_HASH_WITH_NETMASK
// IP_SET_HASH_WITH_BITMASK

#[repr(C)]
pub struct hash_ip4_elem { pub ip: u32 }

unsafe fn hash_ip4_data_equal(e1: *const hash_ip4_elem, e2: *const hash_ip4_elem, _multi: *mut u32) -> bool {
    (*e1).ip == (*e2).ip
}

unsafe fn hash_ip4_data_list(skb: *mut sk_buff, e: *const hash_ip4_elem) -> bool {
    if nla_put_ipaddr4(skb, IPSET_ATTR_IP, (*e).ip) { return true; }
    false
}

unsafe fn hash_ip4_data_next(next: *mut hash_ip4_elem, e: *const hash_ip4_elem) {
    (*next).ip = (*e).ip;
}

// Generated declarations from ip_set_hash_gen.h, with MTYPE=hash_ip4 and HOST_MASK=32.

unsafe fn hash_ip4_kadt(set: *mut ip_set, skb: *const sk_buff, _par: *const xt_action_param,
                        adt: ipset_adt, opt: *mut ip_set_adt_opt) -> i32 {
    let h = (*set).data as *const hash_ip4;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e = hash_ip4_elem { ip: 0 };
    let mut ext = IP_SET_INIT_KEXT(skb, opt, set);
    let mut ip: u32 = 0;
    ip4addrptr(skb, (*opt).flags & IPSET_DIM_ONE_SRC, &mut ip);
    ip &= (*h).bitmask.ip;
    if ip == 0 { return -EINVAL; }
    e.ip = ip;
    adtfn(set, &mut e as *mut _, &mut ext, &mut (*opt).ext, (*opt).cmdflags)
}

unsafe fn hash_ip4_uadt(set: *mut ip_set, tb: *mut *mut nlattr, adt: ipset_adt,
                        lineno: *mut u32, flags: u32, retried: bool) -> i32 {
    let h = (*set).data as *mut hash_ip4;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e = hash_ip4_elem { ip: 0 };
    let mut ext = IP_SET_INIT_UEXT(set);
    let mut ip: u32 = 0; let mut ip_to: u32 = 0; let mut hosts: u32; let mut i: u32 = 0; let mut ret: i32 = 0;
    if !(*tb.add(IPSET_ATTR_LINENO as usize)).is_null() { *lineno = nla_get_u32(*tb.add(IPSET_ATTR_LINENO as usize)); }
    if (*tb.add(IPSET_ATTR_IP as usize)).is_null() { return -IPSET_ERR_PROTOCOL; }
    ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP as usize), &mut ip); if ret != 0 { return ret; }
    ret = ip_set_get_extensions(set, tb, &mut ext); if ret != 0 { return ret; }
    ip &= ntohl((*h).bitmask.ip); e.ip = htonl(ip);
    if e.ip == 0 { return -IPSET_ERR_HASH_ELEM; }
    if adt == IPSET_TEST { return adtfn(set, &mut e, &mut ext, &mut ext, flags); }
    ip_to = ip;
    if !(*tb.add(IPSET_ATTR_IP_TO as usize)).is_null() {
        ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP_TO as usize), &mut ip_to); if ret != 0 { return ret; }
        if ip > ip_to { if ip_to == 0 { return -IPSET_ERR_HASH_ELEM; } core::mem::swap(&mut ip, &mut ip_to); }
    } else if !(*tb.add(IPSET_ATTR_CIDR as usize)).is_null() {
        let cidr = nla_get_u8(*tb.add(IPSET_ATTR_CIDR as usize));
        if cidr == 0 || cidr > 32 { return -IPSET_ERR_INVALID_CIDR; }
        ip_set_mask_from_to(&mut ip, &mut ip_to, cidr);
    }
    hosts = if (*h).netmask == 32 { 1 } else { 2u32 << (32 - (*h).netmask - 1) };
    if retried { ip = ntohl((*h).next.ip); }
    while ip <= ip_to {
        e.ip = htonl(ip);
        if i > IPSET_MAX_RANGE { hash_ip4_data_next(&mut (*h).next, &e); return -ERANGE; }
        ret = adtfn(set, &mut e, &mut ext, &mut ext, flags);
        if ret != 0 && !ip_set_eexist(ret, flags) { return ret; }
        ip += hosts; if ip == 0 { return 0; } ret = 0; i += 1;
    }
    ret
}

#[repr(C)] pub struct hash_ip6_elem { pub ip: nf_inet_addr }
unsafe fn hash_ip6_data_equal(a: *const hash_ip6_elem, b: *const hash_ip6_elem, _multi: *mut u32) -> bool { ipv6_addr_equal(&(*a).ip.in6, &(*b).ip.in6) }
unsafe fn hash_ip6_data_list(skb: *mut sk_buff, e: *const hash_ip6_elem) -> bool { if nla_put_ipaddr6(skb, IPSET_ATTR_IP, &(*e).ip.in6) { return true; } false }
unsafe fn hash_ip6_data_next(_next: *mut hash_ip6_elem, _e: *const hash_ip6_elem) {}

// Generated declarations from ip_set_hash_gen.h, with MTYPE=hash_ip6 and HOST_MASK=128.
// IP_SET_EMIT_CREATE

unsafe fn hash_ip6_kadt(set: *mut ip_set, skb: *const sk_buff, _par: *const xt_action_param, adt: ipset_adt, opt: *mut ip_set_adt_opt) -> i32 {
    let h = (*set).data as *const hash_ip6; let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e: hash_ip6_elem = core::mem::zeroed(); let mut ext = IP_SET_INIT_KEXT(skb, opt, set);
    ip6addrptr(skb, (*opt).flags & IPSET_DIM_ONE_SRC, &mut e.ip.in6); nf_inet_addr_mask_inplace(&mut e.ip, &(*h).bitmask);
    if ipv6_addr_any(&e.ip.in6) { return -EINVAL; } adtfn(set, &mut e, &mut ext, &mut (*opt).ext, (*opt).cmdflags)
}

unsafe fn hash_ip6_uadt(set: *mut ip_set, tb: *mut *mut nlattr, adt: ipset_adt, lineno: *mut u32, flags: u32, _retried: bool) -> i32 {
    let h = (*set).data as *const hash_ip6; let adtfn = (*(*set).variant).adt[adt as usize]; let mut e: hash_ip6_elem = core::mem::zeroed(); let mut ext = IP_SET_INIT_UEXT(set); let mut ret: i32;
    if !(*tb.add(IPSET_ATTR_LINENO as usize)).is_null() { *lineno = nla_get_u32(*tb.add(IPSET_ATTR_LINENO as usize)); }
    if (*tb.add(IPSET_ATTR_IP as usize)).is_null() { return -IPSET_ERR_PROTOCOL; }
    if !(*tb.add(IPSET_ATTR_IP_TO as usize)).is_null() { return -IPSET_ERR_HASH_RANGE_UNSUPPORTED; }
    if !(*tb.add(IPSET_ATTR_CIDR as usize)).is_null() && nla_get_u8(*tb.add(IPSET_ATTR_CIDR as usize)) != 128 { return -IPSET_ERR_INVALID_CIDR; }
    ret = ip_set_get_ipaddr6(*tb.add(IPSET_ATTR_IP as usize), &mut e.ip); if ret != 0 { return ret; }
    ret = ip_set_get_extensions(set, tb, &mut ext); if ret != 0 { return ret; }
    nf_inet_addr_mask_inplace(&mut e.ip, &(*h).bitmask); if ipv6_addr_any(&e.ip.in6) { return -IPSET_ERR_HASH_ELEM; }
    ret = adtfn(set, &mut e, &mut ext, &mut ext, flags); if ip_set_eexist(ret, flags) { 0 } else { ret }
}

// The remaining ip_set_type registration structure and module init/exit are declarations
// expressed through the external kernel-facing Rust bindings.
extern "C" { static mut hash_ip_type: ip_set_type; fn ip_set_type_register(t: *mut ip_set_type) -> i32; fn ip_set_type_unregister(t: *mut ip_set_type); fn rcu_barrier(); }
unsafe fn hash_ip_init() -> i32 { ip_set_type_register(&mut hash_ip_type) }
unsafe fn hash_ip_fini() { rcu_barrier(); ip_set_type_unregister(&mut hash_ip_type); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
