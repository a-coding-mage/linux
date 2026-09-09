// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2000-2002 Joakim Axelsson <gozem@linux.nu>
 *                         Patrick Schaaf <bof@bof.de>
 * Copyright (C) 2003-2013 Jozsef Kadlecsik <kadlec@netfilter.org>
 */

/* Kernel module implementing an IP set type: the bitmap:ip type */

// Kernel and ipset headers from the original translation unit provide the
// external types, constants, macros, and functions referenced below.

pub const IPSET_TYPE_REV_MIN: u32 = 0;
pub const IPSET_TYPE_REV_MAX: u32 = 3;
pub const HOST_MASK: u8 = 32;

#[repr(C)]
pub struct bitmap_ip {
    pub members: *mut core::ffi::c_ulong,
    pub first_ip: u32,
    pub last_ip: u32,
    pub elements: u32,
    pub hosts: u32,
    pub memsize: usize,
    pub netmask: u8,
    pub gc: timer_list,
    pub set: *mut ip_set,
    pub extensions: [u8; 0],
}

#[repr(C)]
pub struct bitmap_ip_adt_elem {
    pub id: u16,
}

unsafe fn ip_to_id(m: *const bitmap_ip, ip: u32) -> u32 {
    ((ip & ip_set_hostmask((*m).netmask)) - (*m).first_ip) / (*m).hosts
}

unsafe fn bitmap_ip_do_test(e: *const bitmap_ip_adt_elem, map: *mut bitmap_ip, _dsize: usize) -> i32 {
    (!0i32) * (test_bit_acquire((*e).id as usize, (*map).members) != 0) as i32
}

unsafe fn bitmap_ip_gc_test(id: u16, map: *const bitmap_ip, _dsize: usize) -> i32 {
    (!0i32) * (test_bit(id as usize, (*map).members) != 0) as i32
}

unsafe fn bitmap_ip_do_add(e: *const bitmap_ip_adt_elem, map: *mut bitmap_ip, _flags: u32, _dsize: usize) -> i32 {
    (!0i32) * (test_bit((*e).id as usize, (*map).members) != 0) as i32
}

unsafe fn bitmap_ip_do_del(e: *const bitmap_ip_adt_elem, map: *mut bitmap_ip) -> i32 {
    (test_and_clear_bit((*e).id as usize, (*map).members) == 0) as i32
}

unsafe fn bitmap_ip_do_list(skb: *mut sk_buff, map: *const bitmap_ip, id: u32, _dsize: usize) -> i32 {
    nla_put_ipaddr4(skb, IPSET_ATTR_IP, htonl((*map).first_ip + id * (*map).hosts))
}

unsafe fn bitmap_ip_do_head(skb: *mut sk_buff, map: *const bitmap_ip) -> i32 {
    nla_put_ipaddr4(skb, IPSET_ATTR_IP, htonl((*map).first_ip)) |
    nla_put_ipaddr4(skb, IPSET_ATTR_IP_TO, htonl((*map).last_ip)) |
    if (*map).netmask != 32 { nla_put_u8(skb, IPSET_ATTR_NETMASK, (*map).netmask) } else { 0 }
}

unsafe fn bitmap_ip_kadt(set: *mut ip_set, skb: *const sk_buff, _par: *const xt_action_param,
                         adt: ipset_adt, opt: *mut ip_set_adt_opt) -> i32 {
    let map = (*set).data as *mut bitmap_ip;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e = bitmap_ip_adt_elem { id: 0 };
    let mut ext = IP_SET_INIT_KEXT(skb, opt, set);
    let ip = ntohl(ip4addr(skb, (*opt).flags & IPSET_DIM_ONE_SRC));
    if ip < (*map).first_ip || ip > (*map).last_ip { return -IPSET_ERR_BITMAP_RANGE; }
    e.id = ip_to_id(map, ip) as u16;
    adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut (*opt).ext, (*opt).cmdflags)
}

unsafe fn bitmap_ip_uadt(set: *mut ip_set, tb: *mut *mut nlattr, adt: ipset_adt,
                         lineno: *mut u32, flags: u32, _retried: bool) -> i32 {
    let map = (*set).data as *mut bitmap_ip;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut ip = 0u32; let mut ip_to = 0u32;
    let mut e = bitmap_ip_adt_elem { id: 0 };
    let mut ext = IP_SET_INIT_UEXT(set);
    let mut ret = 0;
    if !(*tb.add(IPSET_ATTR_LINENO as usize)).is_null() { *lineno = nla_get_u32(*tb.add(IPSET_ATTR_LINENO as usize)); }
    if (*tb.add(IPSET_ATTR_IP as usize)).is_null() { return -IPSET_ERR_PROTOCOL; }
    ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP as usize), &mut ip); if ret != 0 { return ret; }
    ret = ip_set_get_extensions(set, tb, &mut ext); if ret != 0 { return ret; }
    if ip < (*map).first_ip || ip > (*map).last_ip { return -IPSET_ERR_BITMAP_RANGE; }
    if adt == IPSET_TEST { e.id = ip_to_id(map, ip) as u16; return adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut ext, flags); }
    if !(*tb.add(IPSET_ATTR_IP_TO as usize)).is_null() { ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP_TO as usize), &mut ip_to); if ret != 0 { return ret; } if ip > ip_to { core::mem::swap(&mut ip, &mut ip_to); } }
    else if !(*tb.add(IPSET_ATTR_CIDR as usize)).is_null() { let cidr = nla_get_u8(*tb.add(IPSET_ATTR_CIDR as usize)); if cidr == 0 || cidr > HOST_MASK { return -IPSET_ERR_INVALID_CIDR; } ip_set_mask_from_to(&mut ip, &mut ip_to, cidr); }
    else { ip_to = ip; }
    if ip < (*map).first_ip || ip_to > (*map).last_ip { return -IPSET_ERR_BITMAP_RANGE; }
    while !before(ip_to, ip) { e.id = ip_to_id(map, ip) as u16; ret = adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut ext, flags); if ret != 0 && !ip_set_eexist(ret, flags) { return ret; } ret = 0; ip += (*map).hosts; }
    ret
}

unsafe fn bitmap_ip_same_set(a: *const ip_set, b: *const ip_set) -> bool {
    let x = (*a).data as *const bitmap_ip; let y = (*b).data as *const bitmap_ip;
    (*x).first_ip == (*y).first_ip && (*x).last_ip == (*y).last_ip && (*x).netmask == (*y).netmask && (*a).timeout == (*b).timeout && (*a).extensions == (*b).extensions
}

#[repr(C)] pub struct bitmap_ip_elem {}

// The original source includes ip_set_bitmap_gen.h here; its generated
// variant declarations and implementations remain external dependencies.

unsafe fn init_map_ip(set: *mut ip_set, map: *mut bitmap_ip, first_ip: u32, last_ip: u32, elements: u32, hosts: u32, netmask: u8) -> bool {
    (*map).members = bitmap_zalloc(elements, GFP_KERNEL | __GFP_NOWARN); if (*map).members.is_null() { return false; }
    (*map).first_ip = first_ip; (*map).last_ip = last_ip; (*map).elements = elements; (*map).hosts = hosts; (*map).netmask = netmask;
    (*set).timeout = IPSET_NO_TIMEOUT; (*map).set = set; (*set).data = map as *mut _; (*set).family = NFPROTO_IPV4; true
}

unsafe fn range_to_mask(from: u32, to: u32, bits: *mut u8) -> u32 {
    let mut mask = 0xFFFF_FFFEu32; *bits = 32;
    while { *bits -= 1; *bits > 0 && mask != 0 && (to & mask) != from } { mask <<= 1; }
    mask
}

unsafe fn bitmap_ip_create(net: *mut net, set: *mut ip_set, tb: *mut *mut nlattr, _flags: u32) -> i32 {
    let mut first_ip = 0u32; let mut last_ip = 0u32; let mut hosts: u32; let mut elements: u64; let mut netmask = 32u8;
    if (*tb.add(IPSET_ATTR_IP as usize)).is_null() || !ip_set_optattr_netorder(tb, IPSET_ATTR_TIMEOUT) || !ip_set_optattr_netorder(tb, IPSET_ATTR_CADT_FLAGS) { return -IPSET_ERR_PROTOCOL; }
    let mut ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP as usize), &mut first_ip); if ret != 0 { return ret; }
    if !(*tb.add(IPSET_ATTR_IP_TO as usize)).is_null() { ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP_TO as usize), &mut last_ip); if ret != 0 { return ret; } if first_ip > last_ip { core::mem::swap(&mut first_ip, &mut last_ip); } }
    else if !(*tb.add(IPSET_ATTR_CIDR as usize)).is_null() { let cidr = nla_get_u8(*tb.add(IPSET_ATTR_CIDR as usize)); if cidr >= HOST_MASK { return -IPSET_ERR_INVALID_CIDR; } ip_set_mask_from_to(&mut first_ip, &mut last_ip, cidr); }
    else { return -IPSET_ERR_PROTOCOL; }
    if !(*tb.add(IPSET_ATTR_NETMASK as usize)).is_null() { netmask = nla_get_u8(*tb.add(IPSET_ATTR_NETMASK as usize)); if netmask > HOST_MASK { return -IPSET_ERR_INVALID_NETMASK; } let mask = ip_set_hostmask(netmask); first_ip &= mask; last_ip |= !mask; }
    if netmask == 32 { hosts = 1; elements = last_ip as u64 - first_ip as u64 + 1; }
    else { let mut mask_bits = 0u8; let mask = range_to_mask(first_ip, last_ip, &mut mask_bits); if (mask == 0 && (first_ip != 0 || last_ip != 0xFFFF_FFFF)) || netmask <= mask_bits { return -IPSET_ERR_BITMAP_RANGE; } hosts = 2u32 << (32 - netmask - 1); elements = 2u64 << (netmask - mask_bits - 1); }
    if elements > IPSET_BITMAP_MAX_RANGE as u64 + 1 { return -IPSET_ERR_BITMAP_RANGE_SIZE; }
    (*set).dsize = ip_set_elem_len(set, tb, 0, 0); let map = ip_set_alloc(core::mem::size_of::<bitmap_ip>() + elements as usize * (*set).dsize) as *mut bitmap_ip; if map.is_null() { return -ENOMEM; }
    (*map).memsize = BITS_TO_LONGS(elements as usize) * core::mem::size_of::<core::ffi::c_ulong>();
    (*set).variant = &bitmap_ip as *const _ as *mut _;
    if !init_map_ip(set, map, first_ip, last_ip, elements as u32, hosts, netmask) { ip_set_free(map as *mut _); return -ENOMEM; }
    if !(*tb.add(IPSET_ATTR_TIMEOUT as usize)).is_null() { (*set).timeout = ip_set_timeout_uget(*tb.add(IPSET_ATTR_TIMEOUT as usize)); bitmap_ip_gc_init(set, bitmap_ip_gc); }
    0
}

unsafe fn bitmap_ip_init() -> i32 { ip_set_type_register(&bitmap_ip_type) }
unsafe fn bitmap_ip_fini() { rcu_barrier(); ip_set_type_unregister(&bitmap_ip_type); }

// MODULE_LICENSE("GPL"), MODULE_AUTHOR(...), IP_SET_MODULE_DESC(...),
// MODULE_ALIAS(...), module_init(bitmap_ip_init), and module_exit(bitmap_ip_fini)
// are kernel build metadata retained as comments because they have no direct
// Rust executable equivalent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
