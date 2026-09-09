// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2003-2013 Jozsef Kadlecsik <kadlec@netfilter.org> */
/* Kernel module implementing an IP set type: the hash:net type */

// Linux kernel dependencies supplied by the surrounding translation unit.

pub const IPSET_TYPE_REV_MIN: u32 = 0;
pub const IPSET_TYPE_REV_MAX: u32 = 7;

pub const HTYPE: &str = "hash_net";

#[repr(C)]
pub struct hash_net4_elem {
    pub ip: u32,
    pub padding0: u16,
    pub nomatch: u8,
    pub cidr: u8,
}

unsafe fn hash_net4_data_equal(
    ip1: *const hash_net4_elem,
    ip2: *const hash_net4_elem,
    _multi: *mut u32,
) -> bool {
    (*ip1).ip == (*ip2).ip && (*ip1).cidr == (*ip2).cidr
}

unsafe fn hash_net4_do_data_match(elem: *const hash_net4_elem) -> i32 {
    if (*elem).nomatch != 0 { -ENOTEMPTY } else { 1 }
}

unsafe fn hash_net4_data_set_flags(elem: *mut hash_net4_elem, flags: u32) {
    (*elem).nomatch = ((flags >> 16) & IPSET_FLAG_NOMATCH) as u8;
}

unsafe fn hash_net4_data_reset_flags(elem: *mut hash_net4_elem, flags: *mut u8) {
    core::mem::swap(&mut *flags, &mut (*elem).nomatch);
}

unsafe fn hash_net4_data_netmask(elem: *mut hash_net4_elem, cidr: u8) {
    (*elem).ip &= ip_set_netmask(cidr);
    (*elem).cidr = cidr;
}

unsafe fn hash_net4_data_list(skb: *mut sk_buff, data: *const hash_net4_elem) -> bool {
    let flags: u32 = if (*data).nomatch != 0 { IPSET_FLAG_NOMATCH } else { 0 };
    if nla_put_ipaddr4(skb, IPSET_ATTR_IP, (*data).ip) != 0
        || nla_put_u8(skb, IPSET_ATTR_CIDR, (*data).cidr) != 0
        || (flags != 0 && nla_put_net32(skb, IPSET_ATTR_CADT_FLAGS, htonl(flags)) != 0)
    {
        return true;
    }
    false
}

unsafe fn hash_net4_data_next(next: *mut hash_net4_elem, d: *const hash_net4_elem) {
    (*next).ip = (*d).ip;
}

// The included generic hash implementation supplies hash_net4 declarations and functions.

unsafe fn hash_net4_kadt(
    set: *mut ip_set,
    skb: *const sk_buff,
    _par: *const xt_action_param,
    adt: ipset_adt,
    opt: *mut ip_set_adt_opt,
) -> i32 {
    let h = (*set).data as *const hash_net4;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e = hash_net4_elem { ip: 0, padding0: 0, nomatch: 0, cidr: INIT_CIDR((*h).rnets[0], 32) };
    let mut ext = IP_SET_INIT_KEXT(skb, opt, set);
    if e.cidr == 0 { return -EINVAL; }
    if adt == IPSET_TEST { e.cidr = 32; }
    ip4addrptr(skb, (*opt).flags & IPSET_DIM_ONE_SRC, &mut e.ip);
    e.ip &= ip_set_netmask(e.cidr);
    adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut (*opt).ext, (*opt).cmdflags)
}

unsafe fn hash_net4_uadt(
    set: *mut ip_set, tb: *mut *mut nlattr, adt: ipset_adt,
    lineno: *mut u32, mut flags: u32, retried: bool,
) -> i32 {
    let h = (*set).data as *mut hash_net4;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e = hash_net4_elem { ip: 0, padding0: 0, nomatch: 0, cidr: 32 };
    let mut ext = IP_SET_INIT_UEXT(set);
    let mut ip = 0u32; let mut ip_to = 0u32; let mut i = 0u32;
    if !(*tb.add(IPSET_ATTR_LINENO as usize)).is_null() { *lineno = nla_get_u32(*tb.add(IPSET_ATTR_LINENO as usize)); }
    if (*tb.add(IPSET_ATTR_IP as usize)).is_null() || !ip_set_optattr_netorder(tb, IPSET_ATTR_CADT_FLAGS) { return -IPSET_ERR_PROTOCOL; }
    let mut ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP as usize), &mut ip); if ret != 0 { return ret; }
    ret = ip_set_get_extensions(set, tb, &mut ext); if ret != 0 { return ret; }
    if !(*tb.add(IPSET_ATTR_CIDR as usize)).is_null() { e.cidr = nla_get_u8(*tb.add(IPSET_ATTR_CIDR as usize)); if e.cidr == 0 || e.cidr > 32 { return -IPSET_ERR_INVALID_CIDR; } }
    if !(*tb.add(IPSET_ATTR_CADT_FLAGS as usize)).is_null() && (ip_set_get_h32(*tb.add(IPSET_ATTR_CADT_FLAGS as usize)) & IPSET_FLAG_NOMATCH) != 0 { flags |= IPSET_FLAG_NOMATCH << 16; }
    if adt == IPSET_TEST || (*tb.add(IPSET_ATTR_IP_TO as usize)).is_null() {
        e.ip = htonl(ip & ip_set_hostmask(e.cidr)); ret = adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut ext as *mut _, flags);
        return if ip_set_enomatch(ret, flags, adt, set) { -ret } else if ip_set_eexist(ret, flags) { 0 } else { ret };
    }
    ip_to = ip; ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP_TO as usize), &mut ip_to); if ret != 0 { return ret; }
    if ip_to < ip { core::mem::swap(&mut ip, &mut ip_to); }
    if ip.wrapping_add(u32::MAX) == ip_to { return -IPSET_ERR_HASH_RANGE; }
    if retried { ip = ntohl((*h).next.ip); }
    loop { i += 1; e.ip = htonl(ip); if i > IPSET_MAX_RANGE { hash_net4_data_next(&mut (*h).next, &e); return -ERANGE; }
        ip = ip_set_range_to_cidr(ip, ip_to, &mut e.cidr); ret = adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut ext as *mut _, flags); if ret != 0 && !ip_set_eexist(ret, flags) { return ret; } ret = 0; if !(ip < ip_to) { break; } ip = ip.wrapping_add(1); }
    ret
}

#[repr(C)]
pub union nf_inet_addr { pub in6: [u32; 4], pub all: [u32; 4] }
#[repr(C)]
pub struct hash_net6_elem { pub ip: nf_inet_addr, pub padding0: u16, pub nomatch: u8, pub cidr: u8 }

unsafe fn hash_net6_data_equal(ip1: *const hash_net6_elem, ip2: *const hash_net6_elem, _multi: *mut u32) -> bool { ipv6_addr_equal(&(*ip1).ip.in6, &(*ip2).ip.in6) && (*ip1).cidr == (*ip2).cidr }
unsafe fn hash_net6_do_data_match(elem: *const hash_net6_elem) -> i32 { if (*elem).nomatch != 0 { -ENOTEMPTY } else { 1 } }
unsafe fn hash_net6_data_set_flags(elem: *mut hash_net6_elem, flags: u32) { (*elem).nomatch = ((flags >> 16) & IPSET_FLAG_NOMATCH) as u8; }
unsafe fn hash_net6_data_reset_flags(elem: *mut hash_net6_elem, flags: *mut u8) { core::mem::swap(&mut *flags, &mut (*elem).nomatch); }
unsafe fn hash_net6_data_netmask(elem: *mut hash_net6_elem, cidr: u8) { ip6_netmask(&mut (*elem).ip, cidr); (*elem).cidr = cidr; }
unsafe fn hash_net6_data_list(skb: *mut sk_buff, data: *const hash_net6_elem) -> bool { let flags = if (*data).nomatch != 0 { IPSET_FLAG_NOMATCH } else { 0 }; if nla_put_ipaddr6(skb, IPSET_ATTR_IP, &(*data).ip.in6) != 0 || nla_put_u8(skb, IPSET_ATTR_CIDR, (*data).cidr) != 0 || (flags != 0 && nla_put_net32(skb, IPSET_ATTR_CADT_FLAGS, htonl(flags)) != 0) { return true; } false }
unsafe fn hash_net6_data_next(_next: *mut hash_net6_elem, _d: *const hash_net6_elem) {}

// The included generic hash implementation supplies hash_net6 declarations and functions.

unsafe fn hash_net6_kadt(set: *mut ip_set, skb: *const sk_buff, _par: *const xt_action_param, adt: ipset_adt, opt: *mut ip_set_adt_opt) -> i32 {
    let h = (*set).data as *const hash_net6; let adtfn = (*(*set).variant).adt[adt as usize]; let mut e = hash_net6_elem { ip: nf_inet_addr { all: [0; 4] }, padding0: 0, nomatch: 0, cidr: INIT_CIDR((*h).rnets[0], 128) }; let mut ext = IP_SET_INIT_KEXT(skb, opt, set); if e.cidr == 0 { return -EINVAL; } if adt == IPSET_TEST { e.cidr = 128; } ip6addrptr(skb, (*opt).flags & IPSET_DIM_ONE_SRC, &mut e.ip.in6); ip6_netmask(&mut e.ip, e.cidr); adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut (*opt).ext, (*opt).cmdflags)
}

unsafe fn hash_net6_uadt(set: *mut ip_set, tb: *mut *mut nlattr, adt: ipset_adt, lineno: *mut u32, mut flags: u32, _retried: bool) -> i32 {
    let adtfn = (*(*set).variant).adt[adt as usize]; let mut e = hash_net6_elem { ip: nf_inet_addr { all: [0; 4] }, padding0: 0, nomatch: 0, cidr: 128 }; let mut ext = IP_SET_INIT_UEXT(set); if !(*tb.add(IPSET_ATTR_LINENO as usize)).is_null() { *lineno = nla_get_u32(*tb.add(IPSET_ATTR_LINENO as usize)); } if (*tb.add(IPSET_ATTR_IP as usize)).is_null() || !ip_set_optattr_netorder(tb, IPSET_ATTR_CADT_FLAGS) { return -IPSET_ERR_PROTOCOL; } if !(*tb.add(IPSET_ATTR_IP_TO as usize)).is_null() { return -IPSET_ERR_HASH_RANGE_UNSUPPORTED; } let mut ret = ip_set_get_ipaddr6(*tb.add(IPSET_ATTR_IP as usize), &mut e.ip); if ret != 0 { return ret; } ret = ip_set_get_extensions(set, tb, &mut ext); if ret != 0 { return ret; } if !(*tb.add(IPSET_ATTR_CIDR as usize)).is_null() { e.cidr = nla_get_u8(*tb.add(IPSET_ATTR_CIDR as usize)); if e.cidr == 0 || e.cidr > 128 { return -IPSET_ERR_INVALID_CIDR; } } ip6_netmask(&mut e.ip, e.cidr); if !(*tb.add(IPSET_ATTR_CADT_FLAGS as usize)).is_null() && (ip_set_get_h32(*tb.add(IPSET_ATTR_CADT_FLAGS as usize)) & IPSET_FLAG_NOMATCH) != 0 { flags |= IPSET_FLAG_NOMATCH << 16; } ret = adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut ext as *mut _, flags); if ip_set_enomatch(ret, flags, adt, set) { -ret } else if ip_set_eexist(ret, flags) { 0 } else { ret }
}

// `hash_net_type` and the module registration macros are supplied by the translated
// kernel/ipset support headers and generic implementation.
unsafe fn hash_net_init() -> i32 { ip_set_type_register(&mut hash_net_type) }
unsafe fn hash_net_fini() { rcu_barrier(); ip_set_type_unregister(&mut hash_net_type); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
