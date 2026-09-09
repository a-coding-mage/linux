// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2000-2002 Joakim Axelsson <gozem@linux.nu>
 *                         Patrick Schaaf <bof@bof.de>
 *                         Martin Josefsson <gandalf@wlug.westbo.se>
 */

/* Kernel module implementing an IP set type: the bitmap:ip,mac type */

// Kernel and ipset dependencies supplied externally.

pub const IPSET_TYPE_REV_MIN: u32 = 0;
/* 1: Counter support added */
/* 2: Comment support added */
pub const IPSET_TYPE_REV_MAX: u32 = 3; /* skbinfo support added */

pub const MTYPE: &str = "bitmap_ipmac";
pub const HOST_MASK: u32 = 32;

pub const MAC_UNSET: u8 = 0;
pub const MAC_FILLED: u8 = 1;

#[repr(C)]
pub struct bitmap_ipmac {
    pub members: *mut core::ffi::c_ulong,
    pub first_ip: u32,
    pub last_ip: u32,
    pub elements: u32,
    pub memsize: usize,
    pub gc: timer_list,
    pub set: *mut ip_set,
    // unsigned char extensions[] __aligned(__alignof__(u64));
    pub extensions: [u8; 0],
}

#[repr(C, align(2))]
pub struct bitmap_ipmac_adt_elem {
    pub ether: [u8; ETH_ALEN],
    pub id: u16,
    pub add_mac: u16,
}

#[repr(C, align(8))]
pub struct bitmap_ipmac_elem {
    pub ether: [u8; ETH_ALEN],
    pub filled: u8,
}

unsafe fn ip_to_id(m: *const bitmap_ipmac, ip: u32) -> u32 {
    ip.wrapping_sub((*m).first_ip)
}

unsafe fn get_elem(extensions: *mut u8, id: u32, dsize: usize) -> *mut bitmap_ipmac_elem {
    extensions.add((id as usize).wrapping_mul(dsize)) as *mut bitmap_ipmac_elem
}

unsafe fn get_const_elem(extensions: *const u8, id: u32, dsize: usize) -> *const bitmap_ipmac_elem {
    extensions.add((id as usize).wrapping_mul(dsize)) as *const bitmap_ipmac_elem
}

unsafe fn bitmap_ipmac_do_test(
    e: *const bitmap_ipmac_adt_elem,
    map: *const bitmap_ipmac,
    dsize: usize,
) -> i32 {
    if !test_bit_acquire((*e).id as usize, (*map).members as *const _) {
        return 0;
    }
    let elem = get_const_elem((*map).extensions.as_ptr(), (*e).id as u32, dsize);
    if (*e).add_mac != 0 && (*elem).filled == MAC_FILLED {
        return if ether_addr_equal((*e).ether.as_ptr(), (*elem).ether.as_ptr()) { 1 } else { 0 };
    }
    -EAGAIN
}

unsafe fn bitmap_ipmac_gc_test(id: u16, map: *const bitmap_ipmac, dsize: usize) -> i32 {
    if !test_bit(id as usize, (*map).members as *const _) {
        return 0;
    }
    let elem = get_const_elem((*map).extensions.as_ptr(), id as u32, dsize);
    (*elem).filled == MAC_FILLED as u8 as i32
}

unsafe fn bitmap_ipmac_is_filled(elem: *const bitmap_ipmac_elem) -> i32 {
    ((*elem).filled == MAC_FILLED) as i32
}

unsafe fn bitmap_ipmac_add_timeout(
    timeout: *mut core::ffi::c_ulong,
    e: *const bitmap_ipmac_adt_elem,
    ext: *const ip_set_ext,
    set: *mut ip_set,
    _map: *mut bitmap_ipmac,
    mode: i32,
) -> i32 {
    let mut t = (*ext).timeout;
    if mode == IPSET_ADD_START_STORED_TIMEOUT {
        if t == (*set).timeout { t = *timeout; }
        ip_set_timeout_set(timeout, t);
    } else if (*e).add_mac != 0 {
        ip_set_timeout_set(timeout, t);
    } else {
        *timeout = t;
    }
    0
}

unsafe fn bitmap_ipmac_do_add(e: *const bitmap_ipmac_adt_elem, map: *mut bitmap_ipmac, flags: u32, dsize: usize) -> i32 {
    let elem = get_elem((*map).extensions.as_mut_ptr(), (*e).id as u32, dsize);
    if test_bit((*e).id as usize, (*map).members as *const _) {
        if (*elem).filled == MAC_FILLED {
            if (*e).add_mac != 0 && (flags & IPSET_FLAG_EXIST) != 0 && !ether_addr_equal((*e).ether.as_ptr(), (*elem).ether.as_ptr()) {
                clear_bit((*e).id as usize, (*map).members);
                smp_mb__after_atomic();
                ether_addr_copy((*elem).ether.as_mut_ptr(), (*e).ether.as_ptr());
            }
            return IPSET_ADD_FAILED;
        } else if (*e).add_mac == 0 {
            return IPSET_ADD_FAILED;
        }
        clear_bit((*e).id as usize, (*map).members);
        smp_mb__after_atomic();
        ether_addr_copy((*elem).ether.as_mut_ptr(), (*e).ether.as_ptr());
        (*elem).filled = MAC_FILLED;
        return IPSET_ADD_START_STORED_TIMEOUT;
    } else if (*e).add_mac != 0 {
        ether_addr_copy((*elem).ether.as_mut_ptr(), (*e).ether.as_ptr());
        (*elem).filled = MAC_FILLED;
        return 0;
    }
    (*elem).filled = MAC_UNSET;
    IPSET_ADD_STORE_PLAIN_TIMEOUT
}

unsafe fn bitmap_ipmac_do_del(e: *const bitmap_ipmac_adt_elem, map: *mut bitmap_ipmac) -> i32 {
    (!test_and_clear_bit((*e).id as usize, (*map).members)) as i32
}

unsafe fn bitmap_ipmac_do_list(skb: *mut sk_buff, map: *const bitmap_ipmac, id: u32, dsize: usize) -> i32 {
    let elem = get_const_elem((*map).extensions.as_ptr(), id, dsize);
    (nla_put_ipaddr4(skb, IPSET_ATTR_IP, htonl((*map).first_ip.wrapping_add(id))) != 0 ||
        ((*elem).filled == MAC_FILLED && nla_put(skb, IPSET_ATTR_ETHER, ETH_ALEN, (*elem).ether.as_ptr()) != 0)) as i32
}

unsafe fn bitmap_ipmac_do_head(skb: *mut sk_buff, map: *const bitmap_ipmac) -> i32 {
    (nla_put_ipaddr4(skb, IPSET_ATTR_IP, htonl((*map).first_ip)) != 0 ||
        nla_put_ipaddr4(skb, IPSET_ATTR_IP_TO, htonl((*map).last_ip)) != 0) as i32
}

unsafe fn bitmap_ipmac_kadt(set: *mut ip_set, skb: *const sk_buff, par: *const xt_action_param, adt: ipset_adt, opt: *mut ip_set_adt_opt) -> i32 {
    let map = (*set).data as *mut bitmap_ipmac;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e = bitmap_ipmac_adt_elem { ether: [0; ETH_ALEN], id: 0, add_mac: 1 };
    let mut ext = IP_SET_INIT_KEXT(skb, opt, set);
    let ip = ntohl(ip4addr(skb, (*opt).flags & IPSET_DIM_ONE_SRC));
    if ip < (*map).first_ip || ip > (*map).last_ip { return -IPSET_ERR_BITMAP_RANGE; }
    if (*skb).dev.is_null() || (*(*skb).dev).type_ != ARPHRD_ETHER || !skb_mac_header_was_set(skb) || skb_mac_header_len(skb) < ETH_HLEN { return -EINVAL; }
    e.id = ip_to_id(map, ip) as u16;
    if ((*opt).flags & IPSET_DIM_TWO_SRC) != 0 { ether_addr_copy(e.ether.as_mut_ptr(), (*eth_hdr(skb)).h_source.as_ptr()); } else { ether_addr_copy(e.ether.as_mut_ptr(), (*eth_hdr(skb)).h_dest.as_ptr()); }
    if is_zero_ether_addr(e.ether.as_ptr()) { return -EINVAL; }
    adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut (*opt).ext, (*opt).cmdflags)
}

unsafe fn bitmap_ipmac_uadt(set: *mut ip_set, tb: *mut *mut nlattr, adt: ipset_adt, lineno: *mut u32, flags: u32, _retried: bool) -> i32 {
    let map = (*set).data as *const bitmap_ipmac;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e = bitmap_ipmac_adt_elem { ether: [0; ETH_ALEN], id: 0, add_mac: 0 };
    let mut ext = IP_SET_INIT_UEXT(set);
    let mut ip = 0u32;
    if !(*tb.add(IPSET_ATTR_LINENO as usize)).is_null() { *lineno = nla_get_u32(*tb.add(IPSET_ATTR_LINENO as usize)); }
    if (*tb.add(IPSET_ATTR_IP as usize)).is_null() { return -IPSET_ERR_PROTOCOL; }
    let mut ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP as usize), &mut ip); if ret != 0 { return ret; }
    ret = ip_set_get_extensions(set, tb, &mut ext); if ret != 0 { return ret; }
    if ip < (*map).first_ip || ip > (*map).last_ip { return -IPSET_ERR_BITMAP_RANGE; }
    e.id = ip_to_id(map, ip) as u16;
    if !(*tb.add(IPSET_ATTR_ETHER as usize)).is_null() { if nla_len(*tb.add(IPSET_ATTR_ETHER as usize)) != ETH_ALEN { return -IPSET_ERR_PROTOCOL; } memcpy(e.ether.as_mut_ptr(), nla_data(*tb.add(IPSET_ATTR_ETHER as usize)), ETH_ALEN); e.add_mac = 1; }
    ret = adtfn(set, &mut e as *mut _ as *mut _, &mut ext, &mut ext, flags);
    if ip_set_eexist(ret, flags) { 0 } else { ret }
}

unsafe fn bitmap_ipmac_same_set(a: *const ip_set, b: *const ip_set) -> bool {
    let x = (*a).data as *const bitmap_ipmac; let y = (*b).data as *const bitmap_ipmac;
    (*x).first_ip == (*y).first_ip && (*x).last_ip == (*y).last_ip && (*a).timeout == (*b).timeout && (*a).extensions == (*b).extensions
}

// Plain variant: declarations generated by ip_set_bitmap_gen.h are supplied by the surrounding translation unit.

extern "C" {
    fn bitmap_ipmac_gc_init(set: *mut ip_set, gc: unsafe extern "C" fn(*mut timer_list));
    fn bitmap_ipmac_gc(_timer: *mut timer_list);
}

unsafe fn init_map_ipmac(set: *mut ip_set, map: *mut bitmap_ipmac, first_ip: u32, last_ip: u32, elements: u32) -> bool {
    (*map).members = bitmap_zalloc(elements as usize, GFP_KERNEL | __GFP_NOWARN);
    if (*map).members.is_null() { return false; }
    (*map).first_ip = first_ip; (*map).last_ip = last_ip; (*map).elements = elements; (*set).timeout = IPSET_NO_TIMEOUT;
    (*map).set = set; (*set).data = map as *mut _; (*set).family = NFPROTO_IPV4; true
}

unsafe extern "C" fn bitmap_ipmac_create(net: *mut net, set: *mut ip_set, tb: *mut *mut nlattr, _flags: u32) -> i32 {
    let mut first_ip = 0u32;
    let mut last_ip = 0u32;
    let mut elements: u64;
    let map: *mut bitmap_ipmac;
    let mut ret: i32;
    if (*tb.add(IPSET_ATTR_IP as usize)).is_null() || !ip_set_optattr_netorder(tb, IPSET_ATTR_TIMEOUT) || !ip_set_optattr_netorder(tb, IPSET_ATTR_CADT_FLAGS) { return -IPSET_ERR_PROTOCOL; }
    ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP as usize), &mut first_ip); if ret != 0 { return ret; }
    if !(*tb.add(IPSET_ATTR_IP_TO as usize)).is_null() {
        ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP_TO as usize), &mut last_ip); if ret != 0 { return ret; }
        if first_ip > last_ip { core::mem::swap(&mut first_ip, &mut last_ip); }
    } else if !(*tb.add(IPSET_ATTR_CIDR as usize)).is_null() {
        let cidr = nla_get_u8(*tb.add(IPSET_ATTR_CIDR as usize));
        if cidr as u32 >= HOST_MASK { return -IPSET_ERR_INVALID_CIDR; }
        ip_set_mask_from_to(first_ip, &mut last_ip, cidr);
    } else { return -IPSET_ERR_PROTOCOL; }
    elements = (last_ip as u64).wrapping_sub(first_ip as u64).wrapping_add(1);
    if elements > (IPSET_BITMAP_MAX_RANGE as u64).wrapping_add(1) { return -IPSET_ERR_BITMAP_RANGE_SIZE; }
    (*set).dsize = ip_set_elem_len(set, tb, core::mem::size_of::<bitmap_ipmac_elem>(), core::mem::align_of::<bitmap_ipmac_elem>());
    map = ip_set_alloc(core::mem::size_of::<bitmap_ipmac>().wrapping_add(elements as usize * (*set).dsize)) as *mut bitmap_ipmac;
    if map.is_null() { return -ENOMEM; }
    (*map).memsize = bits_to_longs(elements as usize) * core::mem::size_of::<core::ffi::c_ulong>();
    (*set).variant = &mut bitmap_ipmac;
    if !init_map_ipmac(set, map, first_ip, last_ip, elements as u32) { ip_set_free(map as *mut _); return -ENOMEM; }
    if !(*tb.add(IPSET_ATTR_TIMEOUT as usize)).is_null() { (*set).timeout = ip_set_timeout_uget(*tb.add(IPSET_ATTR_TIMEOUT as usize)); bitmap_ipmac_gc_init(set, bitmap_ipmac_gc); }
    0
}

// C's policy/type initializer is retained as an external descriptor because its kernel ABI types are supplied by dependencies.
extern "C" {
    static mut bitmap_ipmac_type: ip_set_type;
}

// The remaining kernel type descriptor and module registration preserve the C initializer topology.
extern "C" {
    static mut bitmap_ipmac: ip_set_type;
    fn ip_set_type_register(t: *mut ip_set_type) -> i32;
    fn ip_set_type_unregister(t: *mut ip_set_type);
    fn rcu_barrier();
}

unsafe extern "C" fn bitmap_ipmac_init() -> i32 { ip_set_type_register(&mut bitmap_ipmac) }
unsafe extern "C" fn bitmap_ipmac_fini() { rcu_barrier(); ip_set_type_unregister(&mut bitmap_ipmac); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
