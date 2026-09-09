// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2003-2013 Jozsef Kadlecsik <kadlec@netfilter.org> */
// Kernel module implementing an IP set type: the bitmap:port type

// Kernel and ipset dependencies are supplied by the surrounding translation.

pub const IPSET_TYPE_REV_MIN: u32 = 0;
pub const IPSET_TYPE_REV_MAX: u32 = 3;

#[repr(C)]
pub struct bitmap_port {
    pub members: *mut libc::c_ulong,
    pub first_port: u16,
    pub last_port: u16,
    pub elements: u32,
    pub memsize: usize,
    pub gc: timer_list,
    pub set: *mut ip_set,
    pub extensions: [u8; 0],
}

#[repr(C)]
pub struct bitmap_port_adt_elem { pub id: u16 }

#[inline]
unsafe fn port_to_id(m: *const bitmap_port, port: u16) -> u16 {
    port.wrapping_sub((*m).first_port)
}

unsafe fn bitmap_port_do_test(e: *const bitmap_port_adt_elem, map: *const bitmap_port, _dsize: usize) -> libc::c_int {
    test_bit_acquire((*e).id as usize, (*map).members) as libc::c_int
}

unsafe fn bitmap_port_gc_test(id: u16, map: *const bitmap_port, _dsize: usize) -> libc::c_int {
    test_bit(id as usize, (*map).members) as libc::c_int
}

unsafe fn bitmap_port_do_add(e: *const bitmap_port_adt_elem, map: *mut bitmap_port, _flags: u32, _dsize: usize) -> libc::c_int {
    test_bit((*e).id as usize, (*map).members) as libc::c_int
}

unsafe fn bitmap_port_do_del(e: *const bitmap_port_adt_elem, map: *mut bitmap_port) -> libc::c_int {
    (!test_and_clear_bit((*e).id as usize, (*map).members)) as libc::c_int
}

unsafe fn bitmap_port_do_list(skb: *mut sk_buff, map: *const bitmap_port, id: u32, _dsize: usize) -> libc::c_int {
    nla_put_net16(skb, IPSET_ATTR_PORT, htons((*map).first_port.wrapping_add(id as u16)))
}

unsafe fn bitmap_port_do_head(skb: *mut sk_buff, map: *const bitmap_port) -> libc::c_int {
    nla_put_net16(skb, IPSET_ATTR_PORT, htons((*map).first_port)) |
        nla_put_net16(skb, IPSET_ATTR_PORT_TO, htons((*map).last_port))
}

unsafe fn ip_set_get_ip_port(skb: *const sk_buff, pf: u8, src: bool, port: *mut __be16) -> bool {
    let mut proto: u8 = 0;
    let ret = match pf {
        NFPROTO_IPV4 => ip_set_get_ip4_port(skb, src, port, &mut proto),
        NFPROTO_IPV6 => ip_set_get_ip6_port(skb, src, port, &mut proto),
        _ => return false,
    };
    if !ret { return ret; }
    matches!(proto, IPPROTO_TCP | IPPROTO_UDP)
}

unsafe fn bitmap_port_kadt(set: *mut ip_set, skb: *const sk_buff, _par: *const xt_action_param, adt: ipset_adt, opt: *mut ip_set_adt_opt) -> libc::c_int {
    let map = (*set).data as *mut bitmap_port;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e = bitmap_port_adt_elem { id: 0 };
    let mut ext = IP_SET_INIT_KEXT(skb, opt, set);
    let mut port_net: __be16 = 0;
    if !ip_set_get_ip_port(skb, (*opt).family, (*opt).flags & IPSET_DIM_ONE_SRC != 0, &mut port_net) { return -EINVAL; }
    let port = ntohs(port_net);
    if port < (*map).first_port || port > (*map).last_port { return -IPSET_ERR_BITMAP_RANGE; }
    e.id = port_to_id(map, port);
    adtfn(set, &e, &mut ext, &mut (*opt).ext, (*opt).cmdflags)
}

unsafe fn bitmap_port_uadt(set: *mut ip_set, tb: *mut *mut nlattr, adt: ipset_adt, lineno: *mut u32, flags: u32, _retried: bool) -> libc::c_int {
    let map = (*set).data as *mut bitmap_port;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e = bitmap_port_adt_elem { id: 0 };
    let mut ext = IP_SET_INIT_UEXT(set);
    let mut port: u32;
    let mut port_to: u16;
    let mut ret = 0;
    if !(*tb.add(IPSET_ATTR_LINENO as usize)).is_null() { *lineno = nla_get_u32(*tb.add(IPSET_ATTR_LINENO as usize)); }
    if unlikely(!ip_set_attr_netorder(tb, IPSET_ATTR_PORT) || !ip_set_optattr_netorder(tb, IPSET_ATTR_PORT_TO)) { return -IPSET_ERR_PROTOCOL; }
    port = ip_set_get_h16(*tb.add(IPSET_ATTR_PORT as usize)) as u32;
    if port < (*map).first_port as u32 || port > (*map).last_port as u32 { return -IPSET_ERR_BITMAP_RANGE; }
    ret = ip_set_get_extensions(set, tb, &mut ext); if ret != 0 { return ret; }
    if adt == IPSET_TEST { e.id = port_to_id(map, port as u16); return adtfn(set, &e, &mut ext, &mut ext, flags); }
    if !(*tb.add(IPSET_ATTR_PORT_TO as usize)).is_null() {
        port_to = ip_set_get_h16(*tb.add(IPSET_ATTR_PORT_TO as usize));
        if port > port_to as u32 { core::mem::swap(&mut port, &mut (port_to as u32)); port_to = port as u16; if port < (*map).first_port as u32 { return -IPSET_ERR_BITMAP_RANGE; } }
    } else { port_to = port as u16; }
    if port_to > (*map).last_port { return -IPSET_ERR_BITMAP_RANGE; }
    while port <= port_to as u32 { e.id = port_to_id(map, port as u16); ret = adtfn(set, &e, &mut ext, &mut ext, flags); if ret != 0 && !ip_set_eexist(ret, flags) { return ret; } ret = 0; port = port.wrapping_add(1); }
    ret
}

unsafe fn bitmap_port_same_set(a: *const ip_set, b: *const ip_set) -> bool {
    let x = (*a).data as *const bitmap_port; let y = (*b).data as *const bitmap_port;
    (*x).first_port == (*y).first_port && (*x).last_port == (*y).last_port && (*a).timeout == (*b).timeout && (*a).extensions == (*b).extensions
}

#[repr(C)] pub struct bitmap_port_elem {}

// Contents supplied by the generated bitmap implementation header:
// #include "ip_set_bitmap_gen.h"

unsafe fn init_map_port(set: *mut ip_set, map: *mut bitmap_port, first_port: u16, last_port: u16) -> bool {
    (*map).members = bitmap_zalloc((*map).elements, GFP_KERNEL | __GFP_NOWARN);
    if (*map).members.is_null() { return false; }
    (*map).first_port = first_port; (*map).last_port = last_port; (*set).timeout = IPSET_NO_TIMEOUT; (*map).set = set; (*set).data = map as *mut _; (*set).family = NFPROTO_UNSPEC; true
}

unsafe fn bitmap_port_create(net: *mut net, set: *mut ip_set, tb: *mut *mut nlattr, _flags: u32) -> libc::c_int {
    if unlikely(!ip_set_attr_netorder(tb, IPSET_ATTR_PORT) || !ip_set_attr_netorder(tb, IPSET_ATTR_PORT_TO) || !ip_set_optattr_netorder(tb, IPSET_ATTR_TIMEOUT) || !ip_set_optattr_netorder(tb, IPSET_ATTR_CADT_FLAGS)) { return -IPSET_ERR_PROTOCOL; }
    let mut first_port = ip_set_get_h16(*tb.add(IPSET_ATTR_PORT as usize)); let mut last_port = ip_set_get_h16(*tb.add(IPSET_ATTR_PORT_TO as usize)); if first_port > last_port { core::mem::swap(&mut first_port, &mut last_port); }
    let elements = last_port as u32 - first_port as u32 + 1; (*set).dsize = ip_set_elem_len(set, tb, 0, 0);
    let map = ip_set_alloc(core::mem::size_of::<bitmap_port>() + elements as usize * (*set).dsize) as *mut bitmap_port; if map.is_null() { return -ENOMEM; }
    (*map).elements = elements; (*map).memsize = BITS_TO_LONGS(elements) as usize * core::mem::size_of::<libc::c_ulong>(); (*set).variant = &bitmap_port;
    if !init_map_port(set, map, first_port, last_port) { ip_set_free(map as *mut _); return -ENOMEM; }
    if !(*tb.add(IPSET_ATTR_TIMEOUT as usize)).is_null() { (*set).timeout = ip_set_timeout_uget(*tb.add(IPSET_ATTR_TIMEOUT as usize)); bitmap_port_gc_init(set, bitmap_port_gc); }
    0
}

// C aggregate equivalent; policy arrays and module metadata are supplied by
// the surrounding kernel/ipset bindings.
#[allow(non_upper_case_globals)]
pub static mut bitmap_port_type: ip_set_type = ip_set_type {
    name: "bitmap:port",
    protocol: IPSET_PROTOCOL,
    features: IPSET_TYPE_PORT,
    dimension: IPSET_DIM_ONE,
    family: NFPROTO_UNSPEC,
    revision_min: IPSET_TYPE_REV_MIN,
    revision_max: IPSET_TYPE_REV_MAX,
    create: Some(bitmap_port_create),
    create_policy: unsafe { core::mem::zeroed() },
    adt_policy: unsafe { core::mem::zeroed() },
    me: THIS_MODULE,
};

unsafe fn bitmap_port_init() -> libc::c_int { ip_set_type_register(&bitmap_port_type) }
unsafe fn bitmap_port_fini() { rcu_barrier(); ip_set_type_unregister(&bitmap_port_type); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
