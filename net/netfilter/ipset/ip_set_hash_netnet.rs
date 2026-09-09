// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2003-2013 Jozsef Kadlecsik <kadlec@netfilter.org>
 * Copyright (C) 2013 Oliver Smith <oliver@8.c.9.b.0.7.4.0.1.0.0.2.ip6.arpa>
 */

/* Kernel module implementing an IP set type: the hash:net type */
// Linux kernel and ipset headers are supplied by the surrounding translation.

pub const IPSET_TYPE_REV_MIN: u32 = 0;
// 1: Forceadd support added; 2: skbinfo support added;
// 3: bucketsize, initval support added.
pub const IPSET_TYPE_REV_MAX: u32 = 4; // bitmask support added

pub const IPSET_NET_COUNT: usize = 2;
pub const HOST_MASK4: u8 = 32;
pub const HOST_MASK6: u8 = 128;

#[repr(C)]
pub union HashNetnet4Ip { pub ip: [__be32; 2], pub ipcmp: __be64 }
#[repr(C)]
pub union HashNetnet4Cidr { pub cidr: [u8; 2], pub ccmp: u16 }
#[repr(C)]
pub struct hash_netnet4_elem {
    pub ip: HashNetnet4Ip,
    pub nomatch: u8,
    pub padding: u8,
    pub cidr: HashNetnet4Cidr,
}

pub unsafe fn hash_netnet4_data_equal(ip1: *const hash_netnet4_elem, ip2: *const hash_netnet4_elem, _multi: *mut u32) -> bool {
    (*ip1).ipcmp == (*ip2).ipcmp && (*ip1).ccmp == (*ip2).ccmp
}
pub unsafe fn hash_netnet4_do_data_match(elem: *const hash_netnet4_elem) -> i32 { if (*elem).nomatch != 0 { -ENOTEMPTY } else { 1 } }
pub unsafe fn hash_netnet4_data_set_flags(elem: *mut hash_netnet4_elem, flags: u32) { (*elem).nomatch = ((flags >> 16) as u8) & IPSET_FLAG_NOMATCH as u8; }
pub unsafe fn hash_netnet4_data_reset_flags(elem: *mut hash_netnet4_elem, flags: *mut u8) { core::mem::swap(flags, &mut (*elem).nomatch); }
pub unsafe fn hash_netnet4_data_reset_elem(elem: *mut hash_netnet4_elem, orig: *mut hash_netnet4_elem) { (*elem).ip.ip[1] = (*orig).ip.ip[1]; }
pub unsafe fn hash_netnet4_data_netmask(elem: *mut hash_netnet4_elem, cidr: u8, inner: bool) {
    if inner { (*elem).ip.ip[1] &= ip_set_netmask(cidr); (*elem).cidr.cidr[1] = cidr; }
    else { (*elem).ip.ip[0] &= ip_set_netmask(cidr); (*elem).cidr.cidr[0] = cidr; }
}
pub unsafe fn hash_netnet4_data_list(skb: *mut sk_buff, data: *const hash_netnet4_elem) -> bool {
    let flags: u32 = if (*data).nomatch != 0 { IPSET_FLAG_NOMATCH } else { 0 };
    if nla_put_ipaddr4(skb, IPSET_ATTR_IP, (*data).ip.ip[0]) != 0 || nla_put_ipaddr4(skb, IPSET_ATTR_IP2, (*data).ip.ip[1]) != 0 || nla_put_u8(skb, IPSET_ATTR_CIDR, (*data).cidr.cidr[0]) != 0 || nla_put_u8(skb, IPSET_ATTR_CIDR2, (*data).cidr.cidr[1]) != 0 || (flags != 0 && nla_put_net32(skb, IPSET_ATTR_CADT_FLAGS, htonl(flags)) != 0) { return true; }
    false
}
pub unsafe fn hash_netnet4_data_next(next: *mut hash_netnet4_elem, d: *const hash_netnet4_elem) { (*next).ipcmp = (*d).ipcmp; }

// The C source includes ip_set_hash_gen.h with MTYPE=hash_netnet4 and HOST_MASK=32.
// Its generated declarations and definitions are part of the surrounding translation.

pub unsafe fn hash_netnet4_init(e: *mut hash_netnet4_elem) { (*e).cidr.cidr[0] = HOST_MASK4; (*e).cidr.cidr[1] = HOST_MASK4; }

// Direct translation of the kernel entry points. External kernel/ipset types and
// helper functions retain their source names and are supplied by dependencies.
pub unsafe fn hash_netnet4_kadt(set: *mut ip_set, skb: *const sk_buff, _par: *const xt_action_param, adt: ipset_adt, opt: *mut ip_set_adt_opt) -> i32 {
    let h = (*set).data as *const hash_netnet4;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e: hash_netnet4_elem = core::mem::zeroed();
    let mut ext = IP_SET_INIT_KEXT(skb, opt, set);
    rcu_read_lock_bh(); e.cidr.cidr[0] = INIT_CIDR((*h).rnets[0], HOST_MASK4); e.cidr.cidr[1] = INIT_CIDR((*h).rnets[1], HOST_MASK4); rcu_read_unlock_bh();
    if adt == IPSET_TEST { e.cidr.ccmp = ((HOST_MASK4 as u16) << (core::mem::size_of::<u8>() * 8)) | HOST_MASK4 as u16; }
    ip4addrptr(skb, (*opt).flags & IPSET_DIM_ONE_SRC, &mut e.ip.ip[0]); ip4addrptr(skb, (*opt).flags & IPSET_DIM_TWO_SRC, &mut e.ip.ip[1]);
    e.ip.ip[0] &= ip_set_netmask(e.cidr.cidr[0]) & (*h).bitmask.ip; e.ip.ip[1] &= ip_set_netmask(e.cidr.cidr[1]) & (*h).bitmask.ip;
    adtfn(set, &mut e, &mut ext, &mut (*opt).ext, (*opt).cmdflags)
}

// The range-processing body is preserved below in source-level form.
pub unsafe fn hash_netnet4_uadt(set: *mut ip_set, tb: *mut *mut nlattr, adt: ipset_adt, lineno: *mut u32, mut flags: u32, retried: bool) -> i32 {
    let h = (*set).data as *mut hash_netnet4; let adtfn = (*(*set).variant).adt[adt as usize]; let mut e: hash_netnet4_elem = core::mem::zeroed(); let mut ext = IP_SET_INIT_UEXT(set); let mut ip: u32 = 0; let mut ip_to: u32 = 0; let mut ip2: u32 = 0; let mut ip2_from: u32 = 0; let mut ip2_to: u32 = 0; let mut i: u32 = 0; let mut ret: i32;
    if !(*tb.add(IPSET_ATTR_LINENO as usize)).is_null() { *lineno = nla_get_u32(*tb.add(IPSET_ATTR_LINENO as usize)); }
    hash_netnet4_init(&mut e); if (*tb.add(IPSET_ATTR_IP as usize)).is_null() || (*tb.add(IPSET_ATTR_IP2 as usize)).is_null() || !ip_set_optattr_netorder(tb, IPSET_ATTR_CADT_FLAGS) { return -IPSET_ERR_PROTOCOL; }
    ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP as usize), &mut ip); if ret != 0 { return ret; } ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP2 as usize), &mut ip2_from); if ret != 0 { return ret; } ret = ip_set_get_extensions(set, tb, &mut ext); if ret != 0 { return ret; }
    if !(*tb.add(IPSET_ATTR_CIDR as usize)).is_null() { e.cidr.cidr[0] = nla_get_u8(*tb.add(IPSET_ATTR_CIDR as usize)); if e.cidr.cidr[0] == 0 || e.cidr.cidr[0] > HOST_MASK4 { return -IPSET_ERR_INVALID_CIDR; } }
    if !(*tb.add(IPSET_ATTR_CIDR2 as usize)).is_null() { e.cidr.cidr[1] = nla_get_u8(*tb.add(IPSET_ATTR_CIDR2 as usize)); if e.cidr.cidr[1] == 0 || e.cidr.cidr[1] > HOST_MASK4 { return -IPSET_ERR_INVALID_CIDR; } }
    if !(*tb.add(IPSET_ATTR_CADT_FLAGS as usize)).is_null() && ip_set_get_h32(*tb.add(IPSET_ATTR_CADT_FLAGS as usize)) & IPSET_FLAG_NOMATCH != 0 { flags |= IPSET_FLAG_NOMATCH << 16; }
    if adt == IPSET_TEST || ((*tb.add(IPSET_ATTR_IP_TO as usize)).is_null() && (*tb.add(IPSET_ATTR_IP2_TO as usize)).is_null()) { e.ip.ip[0] = htonl(ip & ntohl((*h).bitmask.ip) & ip_set_hostmask(e.cidr.cidr[0])); e.ip.ip[1] = htonl(ip2_from & ntohl((*h).bitmask.ip) & ip_set_hostmask(e.cidr.cidr[1])); ret = adtfn(set, &mut e, &mut ext, &mut ext, flags); return if ip_set_enomatch(ret, flags, adt, set) { -ret } else if ip_set_eexist(ret, flags) { 0 } else { ret }; }
    ip_to = ip; if !(*tb.add(IPSET_ATTR_IP_TO as usize)).is_null() { ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP_TO as usize), &mut ip_to); if ret != 0 { return ret; } if ip_to < ip { core::mem::swap(&mut ip, &mut ip_to); } if ip.wrapping_add(u32::MAX) == ip_to { return -IPSET_ERR_HASH_RANGE; } } else { ip_set_mask_from_to(ip, &mut ip_to, e.cidr.cidr[0]); }
    ip2_to = ip2_from; if !(*tb.add(IPSET_ATTR_IP2_TO as usize)).is_null() { ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP2_TO as usize), &mut ip2_to); if ret != 0 { return ret; } if ip2_to < ip2_from { core::mem::swap(&mut ip2_from, &mut ip2_to); } if ip2_from.wrapping_add(u32::MAX) == ip2_to { return -IPSET_ERR_HASH_RANGE; } } else { ip_set_mask_from_to(ip2_from, &mut ip2_to, e.cidr.cidr[1]); }
    if retried { ip = ntohl((*h).next.ip.ip[0]); ip2 = ntohl((*h).next.ip.ip[1]); } else { ip2 = ip2_from; }
    loop { e.ip.ip[0] = htonl(ip); ip = ip_set_range_to_cidr(ip, ip_to, &mut e.cidr.cidr[0]); loop { i += 1; e.ip.ip[1] = htonl(ip2); if i > IPSET_MAX_RANGE { hash_netnet4_data_next(&mut (*h).next, &e); return -ERANGE; } ip2 = ip_set_range_to_cidr(ip2, ip2_to, &mut e.cidr.cidr[1]); ret = adtfn(set, &mut e, &mut ext, &mut ext, flags); if ret != 0 && !ip_set_eexist(ret, flags) { return ret; } ret = 0; if ip2.wrapping_add(1) > ip2_to { break; } ip2 = ip2.wrapping_add(1); } ip2 = ip2_from; if ip.wrapping_add(1) > ip_to { break; } ip = ip.wrapping_add(1); } ret
}

// IPv6 structures, helpers, generated hash implementation, type registration,
// and module init/fini retain the same declarations and control flow as the C
// source; dependent kernel definitions are intentionally left external.
#[repr(C)] pub struct hash_netnet6_elem { pub ip: [nf_inet_addr; 2], pub nomatch: u8, pub padding: u8, pub cidr: HashNetnet4Cidr }
pub unsafe fn hash_netnet6_init(e: *mut hash_netnet6_elem) { (*e).cidr.cidr[0] = HOST_MASK6; (*e).cidr.cidr[1] = HOST_MASK6; }

pub unsafe fn hash_netnet6_data_equal(a: *const hash_netnet6_elem, b: *const hash_netnet6_elem, _multi: *mut u32) -> bool {
    ipv6_addr_equal(&(*a).ip[0].in6, &(*b).ip[0].in6) && ipv6_addr_equal(&(*a).ip[1].in6, &(*b).ip[1].in6) && (*a).cidr.ccmp == (*b).cidr.ccmp
}
pub unsafe fn hash_netnet6_do_data_match(e: *const hash_netnet6_elem) -> i32 { if (*e).nomatch != 0 { -ENOTEMPTY } else { 1 } }
pub unsafe fn hash_netnet6_data_set_flags(e: *mut hash_netnet6_elem, flags: u32) { (*e).nomatch = ((flags >> 16) as u8) & IPSET_FLAG_NOMATCH as u8; }
pub unsafe fn hash_netnet6_data_reset_flags(e: *mut hash_netnet6_elem, flags: *mut u8) { core::mem::swap(flags, &mut (*e).nomatch); }
pub unsafe fn hash_netnet6_data_reset_elem(e: *mut hash_netnet6_elem, orig: *mut hash_netnet6_elem) { (*e).ip[1] = (*orig).ip[1]; }
pub unsafe fn hash_netnet6_data_netmask(e: *mut hash_netnet6_elem, cidr: u8, inner: bool) { if inner { ip6_netmask(&mut (*e).ip[1], cidr); (*e).cidr.cidr[1] = cidr; } else { ip6_netmask(&mut (*e).ip[0], cidr); (*e).cidr.cidr[0] = cidr; } }
pub unsafe fn hash_netnet6_data_list(skb: *mut sk_buff, d: *const hash_netnet6_elem) -> bool { let f = if (*d).nomatch != 0 { IPSET_FLAG_NOMATCH } else { 0 }; if nla_put_ipaddr6(skb, IPSET_ATTR_IP, &(*d).ip[0].in6) != 0 || nla_put_ipaddr6(skb, IPSET_ATTR_IP2, &(*d).ip[1].in6) != 0 || nla_put_u8(skb, IPSET_ATTR_CIDR, (*d).cidr.cidr[0]) != 0 || nla_put_u8(skb, IPSET_ATTR_CIDR2, (*d).cidr.cidr[1]) != 0 || (f != 0 && nla_put_net32(skb, IPSET_ATTR_CADT_FLAGS, htonl(f)) != 0) { true } else { false } }
pub unsafe fn hash_netnet6_data_next(_next: *mut hash_netnet6_elem, _d: *const hash_netnet6_elem) {}

// MTYPE=hash_netnet6, HOST_MASK=128, IP_SET_EMIT_CREATE; generated hash
// declarations from ip_set_hash_gen.h are supplied by the surrounding build.
pub unsafe fn hash_netnet6_kadt(set: *mut ip_set, skb: *const sk_buff, _par: *const xt_action_param, adt: ipset_adt, opt: *mut ip_set_adt_opt) -> i32 {
    let h = (*set).data as *const hash_netnet6; let f = (*(*set).variant).adt[adt as usize]; let mut e: hash_netnet6_elem = core::mem::zeroed(); let mut ext = IP_SET_INIT_KEXT(skb, opt, set);
    rcu_read_lock_bh(); e.cidr.cidr[0] = INIT_CIDR((*h).rnets[0], HOST_MASK6); e.cidr.cidr[1] = INIT_CIDR((*h).rnets[1], HOST_MASK6); rcu_read_unlock_bh();
    if adt == IPSET_TEST { e.cidr.ccmp = ((HOST_MASK6 as u16) << (core::mem::size_of::<u8>() * 8)) | HOST_MASK6 as u16; }
    ip6addrptr(skb, (*opt).flags & IPSET_DIM_ONE_SRC, &mut e.ip[0].in6); ip6addrptr(skb, (*opt).flags & IPSET_DIM_TWO_SRC, &mut e.ip[1].in6); ip6_netmask(&mut e.ip[0], e.cidr.cidr[0]); ip6_netmask(&mut e.ip[1], e.cidr.cidr[1]); nf_inet_addr_mask_inplace(&mut e.ip[0], &(*h).bitmask); nf_inet_addr_mask_inplace(&mut e.ip[1], &(*h).bitmask); if e.cidr.cidr[0] == HOST_MASK6 && ipv6_addr_any(&e.ip[0].in6) { return -EINVAL; } f(set, &mut e, &mut ext, &mut (*opt).ext, (*opt).cmdflags)
}

pub unsafe fn hash_netnet6_uadt(set: *mut ip_set, tb: *mut *mut nlattr, adt: ipset_adt, lineno: *mut u32, mut flags: u32, _retried: bool) -> i32 {
    let h = (*set).data as *const hash_netnet6; let f = (*(*set).variant).adt[adt as usize]; let mut e: hash_netnet6_elem = core::mem::zeroed(); let mut ext = IP_SET_INIT_UEXT(set); let mut ret: i32;
    if !(*tb.add(IPSET_ATTR_LINENO as usize)).is_null() { *lineno = nla_get_u32(*tb.add(IPSET_ATTR_LINENO as usize)); } hash_netnet6_init(&mut e);
    if (*tb.add(IPSET_ATTR_IP as usize)).is_null() || (*tb.add(IPSET_ATTR_IP2 as usize)).is_null() || !ip_set_optattr_netorder(tb, IPSET_ATTR_CADT_FLAGS) { return -IPSET_ERR_PROTOCOL; } if !(*tb.add(IPSET_ATTR_IP_TO as usize)).is_null() || !(*tb.add(IPSET_ATTR_IP2_TO as usize)).is_null() { return -IPSET_ERR_HASH_RANGE_UNSUPPORTED; }
    ret = ip_set_get_ipaddr6(*tb.add(IPSET_ATTR_IP as usize), &mut e.ip[0]); if ret != 0 { return ret; } ret = ip_set_get_ipaddr6(*tb.add(IPSET_ATTR_IP2 as usize), &mut e.ip[1]); if ret != 0 { return ret; } ret = ip_set_get_extensions(set, tb, &mut ext); if ret != 0 { return ret; }
    if !(*tb.add(IPSET_ATTR_CIDR as usize)).is_null() { e.cidr.cidr[0] = nla_get_u8(*tb.add(IPSET_ATTR_CIDR as usize)); if e.cidr.cidr[0] == 0 || e.cidr.cidr[0] > HOST_MASK6 { return -IPSET_ERR_INVALID_CIDR; } } if !(*tb.add(IPSET_ATTR_CIDR2 as usize)).is_null() { e.cidr.cidr[1] = nla_get_u8(*tb.add(IPSET_ATTR_CIDR2 as usize)); if e.cidr.cidr[1] == 0 || e.cidr.cidr[1] > HOST_MASK6 { return -IPSET_ERR_INVALID_CIDR; } }
    ip6_netmask(&mut e.ip[0], e.cidr.cidr[0]); ip6_netmask(&mut e.ip[1], e.cidr.cidr[1]); nf_inet_addr_mask_inplace(&mut e.ip[0], &(*h).bitmask); nf_inet_addr_mask_inplace(&mut e.ip[1], &(*h).bitmask); if e.cidr.cidr[0] == HOST_MASK6 && ipv6_addr_any(&e.ip[0].in6) { return -IPSET_ERR_HASH_ELEM; } if !(*tb.add(IPSET_ATTR_CADT_FLAGS as usize)).is_null() && ip_set_get_h32(*tb.add(IPSET_ATTR_CADT_FLAGS as usize) & IPSET_FLAG_NOMATCH) != 0 { flags |= IPSET_FLAG_NOMATCH << 16; }
    ret = f(set, &mut e, &mut ext, &mut ext, flags); if ip_set_enomatch(ret, flags, adt, set) { -ret } else if ip_set_eexist(ret, flags) { 0 } else { ret }
}

// struct ip_set_type hash_netnet_type and its create/adt policies are a direct
// kernel registration descriptor; policy constants and generated create code
// remain external dependencies.
extern "C" {
    static mut hash_netnet_type: ip_set_type;
}
pub unsafe fn hash_netnet_init() -> i32 { ip_set_type_register(&mut hash_netnet_type) }
pub unsafe fn hash_netnet_fini() { rcu_barrier(); ip_set_type_unregister(&mut hash_netnet_type); }

// module_init(hash_netnet_init); module_exit(hash_netnet_fini);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
