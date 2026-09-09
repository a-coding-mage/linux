// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2003-2013 Jozsef Kadlecsik <kadlec@netfilter.org> */

/* Kernel module implementing an IP set type: the hash:ip,port type */

// Linux/kernel and ipset dependencies supplied externally.
// The C source includes ip_set_hash_gen.h twice with different MTYPE/HOST_MASK
// settings; its generated declarations and definitions are external here.

pub const IPSET_TYPE_REV_MIN: u8 = 0;
pub const IPSET_TYPE_REV_MAX: u8 = 7; // bitmask support added

pub const HOST_MASK_IPV4: u8 = 32;
pub const HOST_MASK_IPV6: u16 = 128;

#[repr(C)]
pub struct hash_ipport4_elem {
    pub ip: __be32,
    pub port: __be16,
    pub proto: u8,
    pub padding: u8,
}

unsafe fn hash_ipport4_data_equal(
    ip1: *const hash_ipport4_elem,
    ip2: *const hash_ipport4_elem,
    _multi: *mut u32,
) -> bool {
    (*ip1).ip == (*ip2).ip && (*ip1).port == (*ip2).port && (*ip1).proto == (*ip2).proto
}

unsafe fn hash_ipport4_data_list(
    skb: *mut sk_buff,
    data: *const hash_ipport4_elem,
) -> bool {
    if nla_put_ipaddr4(skb, IPSET_ATTR_IP, (*data).ip) != 0
        || nla_put_net16(skb, IPSET_ATTR_PORT, (*data).port) != 0
        || nla_put_u8(skb, IPSET_ATTR_PROTO, (*data).proto) != 0
    {
        return true;
    }
    false
}

unsafe fn hash_ipport4_data_next(
    next: *mut hash_ipport4_elem,
    d: *const hash_ipport4_elem,
) {
    (*next).ip = (*d).ip;
    (*next).port = (*d).port;
}

// Generated hash:ip,port IPv4 implementation from ip_set_hash_gen.h.

unsafe fn hash_ipport4_kadt(
    set: *mut ip_set,
    skb: *const sk_buff,
    _par: *const xt_action_param,
    adt: ipset_adt,
    opt: *mut ip_set_adt_opt,
) -> i32 {
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e: hash_ipport4_elem = core::mem::zeroed();
    let mut ext = IP_SET_INIT_KEXT(skb, opt, set);
    let h = (*set).data as *const hash_ipport4;
    if ip_set_get_ip4_port(skb, (*opt).flags & IPSET_DIM_TWO_SRC, &mut e.port, &mut e.proto) == 0 {
        return -EINVAL;
    }
    ip4addrptr(skb, (*opt).flags & IPSET_DIM_ONE_SRC, &mut e.ip);
    e.ip &= (*h).bitmask.ip;
    if e.ip == 0 { return -EINVAL; }
    adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut (*opt).ext, (*opt).cmdflags)
}

unsafe fn hash_ipport4_uadt(
    set: *mut ip_set, tb: *mut *mut nlattr, adt: ipset_adt,
    lineno: *mut u32, flags: u32, retried: bool,
) -> i32 {
    let h = (*set).data as *mut hash_ipport4;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e: hash_ipport4_elem = core::mem::zeroed();
    let mut ext = IP_SET_INIT_UEXT(set);
    let mut ip_to: u32 = 0; let mut p: u32 = 0; let mut i: u32 = 0;
    let mut ip: u32; let mut port: u32; let mut port_to: u32;
    let mut with_ports = false; let mut ret: i32;
    if !(*tb.add(IPSET_ATTR_LINENO as usize)).is_null() { *lineno = nla_get_u32(*tb.add(IPSET_ATTR_LINENO as usize)); }
    if (*tb.add(IPSET_ATTR_IP as usize)).is_null() || ip_set_attr_netorder(tb, IPSET_ATTR_PORT) == 0 || ip_set_optattr_netorder(tb, IPSET_ATTR_PORT_TO) == 0 { return -IPSET_ERR_PROTOCOL; }
    ret = ip_set_get_ipaddr4(*tb.add(IPSET_ATTR_IP as usize), &mut e.ip); if ret != 0 { return ret; }
    ret = ip_set_get_extensions(set, tb, &mut ext); if ret != 0 { return ret; }
    e.ip &= (*h).bitmask.ip; if e.ip == 0 { return -EINVAL; }
    e.port = nla_get_be16(*tb.add(IPSET_ATTR_PORT as usize));
    if !(*tb.add(IPSET_ATTR_PROTO as usize)).is_null() { e.proto = nla_get_u8(*tb.add(IPSET_ATTR_PROTO as usize)); with_ports = ip_set_proto_with_ports(e.proto); if e.proto == 0 { return -IPSET_ERR_INVALID_PROTO; } } else { return -IPSET_ERR_MISSING_PROTO; }
    if !(with_ports || e.proto as i32 == IPPROTO_ICMP) { e.port = 0; }
    if adt == IPSET_TEST || ((*tb.add(IPSET_ATTR_IP_TO as usize)).is_null() && (*tb.add(IPSET_ATTR_CIDR as usize)).is_null() && (*tb.add(IPSET_ATTR_PORT_TO as usize)).is_null()) { ret = adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut ext, flags); return if ip_set_eexist(ret, flags) { 0 } else { ret }; }
    ip = ntohl(e.ip); ip_to = ip;
    if !(*tb.add(IPSET_ATTR_IP_TO as usize)).is_null() { ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP_TO as usize), &mut ip_to); if ret != 0 { return ret; } if ip > ip_to { core::mem::swap(&mut ip, &mut ip_to); } } else if !(*tb.add(IPSET_ATTR_CIDR as usize)).is_null() { let cidr = nla_get_u8(*tb.add(IPSET_ATTR_CIDR as usize)); if cidr == 0 || cidr > HOST_MASK_IPV4 { return -IPSET_ERR_INVALID_CIDR; } ip_set_mask_from_to(ip, ip_to, cidr); }
    port = ntohs(e.port); port_to = port; if with_ports && !(*tb.add(IPSET_ATTR_PORT_TO as usize)).is_null() { port_to = ip_set_get_h16(*tb.add(IPSET_ATTR_PORT_TO as usize)); if port > port_to { core::mem::swap(&mut port, &mut port_to); } }
    if retried { ip = ntohl((*h).next.ip); }
    while ip <= ip_to { p = if retried && ip == ntohl((*h).next.ip) { ntohs((*h).next.port) } else { port }; while p <= port_to { e.ip = htonl(ip); e.port = htons(p); if i > IPSET_MAX_RANGE { hash_ipport4_data_next(&mut (*h).next, &e); return -ERANGE; } ret = adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut ext, flags); if ret != 0 && !ip_set_eexist(ret, flags) { return ret; } ret = 0; p += 1; i += 1; } if ip == ip_to { break; } ip += 1; }
    ret
}

#[repr(C)]
pub struct hash_ipport6_elem { pub ip: nf_inet_addr, pub port: __be16, pub proto: u8, pub padding: u8 }

unsafe fn hash_ipport6_data_equal(ip1: *const hash_ipport6_elem, ip2: *const hash_ipport6_elem, _multi: *mut u32) -> bool { ipv6_addr_equal(&(*ip1).ip.in6, &(*ip2).ip.in6) && (*ip1).port == (*ip2).port && (*ip1).proto == (*ip2).proto }
unsafe fn hash_ipport6_data_list(skb: *mut sk_buff, data: *const hash_ipport6_elem) -> bool { nla_put_ipaddr6(skb, IPSET_ATTR_IP, &(*data).ip.in6) != 0 || nla_put_net16(skb, IPSET_ATTR_PORT, (*data).port) != 0 || nla_put_u8(skb, IPSET_ATTR_PROTO, (*data).proto) != 0 }
unsafe fn hash_ipport6_data_next(next: *mut hash_ipport6_elem, d: *const hash_ipport6_elem) { (*next).port = (*d).port; }

// Generated hash:ip,port IPv6 implementation from ip_set_hash_gen.h.

unsafe fn hash_ipport6_kadt(set: *mut ip_set, skb: *const sk_buff, _par: *const xt_action_param, adt: ipset_adt, opt: *mut ip_set_adt_opt) -> i32 {
    let adtfn = (*(*set).variant).adt[adt as usize]; let mut e: hash_ipport6_elem = core::mem::zeroed(); let mut ext = IP_SET_INIT_KEXT(skb, opt, set); let h = (*set).data as *const hash_ipport6;
    if ip_set_get_ip6_port(skb, (*opt).flags & IPSET_DIM_TWO_SRC, &mut e.port, &mut e.proto) == 0 { return -EINVAL; } ip6addrptr(skb, (*opt).flags & IPSET_DIM_ONE_SRC, &mut e.ip.in6); nf_inet_addr_mask_inplace(&mut e.ip, &(*h).bitmask); if ipv6_addr_any(&e.ip.in6) { return -EINVAL; } adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut (*opt).ext, (*opt).cmdflags)
}

unsafe fn hash_ipport6_uadt(set: *mut ip_set, tb: *mut *mut nlattr, adt: ipset_adt, lineno: *mut u32, flags: u32, retried: bool) -> i32 {
    let h = (*set).data as *const hash_ipport6; let adtfn = (*(*set).variant).adt[adt as usize]; let mut e: hash_ipport6_elem = core::mem::zeroed(); let mut ext = IP_SET_INIT_UEXT(set); let mut ret: i32; let mut port: u32; let mut port_to: u32; let mut with_ports = false;
    if !(*tb.add(IPSET_ATTR_LINENO as usize)).is_null() { *lineno = nla_get_u32(*tb.add(IPSET_ATTR_LINENO as usize)); }
    if (*tb.add(IPSET_ATTR_IP as usize)).is_null() || ip_set_attr_netorder(tb, IPSET_ATTR_PORT) == 0 || ip_set_optattr_netorder(tb, IPSET_ATTR_PORT_TO) == 0 { return -IPSET_ERR_PROTOCOL; } if !(*tb.add(IPSET_ATTR_IP_TO as usize)).is_null() { return -IPSET_ERR_HASH_RANGE_UNSUPPORTED; } if !(*tb.add(IPSET_ATTR_CIDR as usize)).is_null() && nla_get_u8(*tb.add(IPSET_ATTR_CIDR as usize)) != HOST_MASK_IPV6 as u8 { return -IPSET_ERR_INVALID_CIDR; }
    ret = ip_set_get_ipaddr6(*tb.add(IPSET_ATTR_IP as usize), &mut e.ip); if ret != 0 { return ret; } ret = ip_set_get_extensions(set, tb, &mut ext); if ret != 0 { return ret; } nf_inet_addr_mask_inplace(&mut e.ip, &(*h).bitmask); if ipv6_addr_any(&e.ip.in6) { return -EINVAL; } e.port = nla_get_be16(*tb.add(IPSET_ATTR_PORT as usize)); if !(*tb.add(IPSET_ATTR_PROTO as usize)).is_null() { e.proto = nla_get_u8(*tb.add(IPSET_ATTR_PROTO as usize)); with_ports = ip_set_proto_with_ports(e.proto); if e.proto == 0 { return -IPSET_ERR_INVALID_PROTO; } } else { return -IPSET_ERR_MISSING_PROTO; } if !(with_ports || e.proto as i32 == IPPROTO_ICMPV6) { e.port = 0; }
    if adt == IPSET_TEST || !with_ports || (*tb.add(IPSET_ATTR_PORT_TO as usize)).is_null() { ret = adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut ext, flags); return if ip_set_eexist(ret, flags) { 0 } else { ret }; } port = ntohs(e.port); port_to = ip_set_get_h16(*tb.add(IPSET_ATTR_PORT_TO as usize)); if port > port_to { core::mem::swap(&mut port, &mut port_to); } if retried { port = ntohs((*h).next.port); } while port <= port_to { e.port = htons(port); ret = adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut ext, flags); if ret != 0 && !ip_set_eexist(ret, flags) { return ret; } ret = 0; port += 1; } ret
}

// The following type registration and module entry points are supplied using the
// corresponding external kernel/ipset Rust bindings.
extern "C" { fn ip_set_type_register(t: *mut ip_set_type) -> i32; fn ip_set_type_unregister(t: *mut ip_set_type); fn rcu_barrier(); }

unsafe fn hash_ipport_init() -> i32 { ip_set_type_register(&mut hash_ipport_type) }
unsafe fn hash_ipport_fini() { rcu_barrier(); ip_set_type_unregister(&mut hash_ipport_type); }

extern "C" { static mut hash_ipport_type: ip_set_type; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
