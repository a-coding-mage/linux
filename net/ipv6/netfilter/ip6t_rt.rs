// SPDX-License-Identifier: GPL-2.0-only
/* Kernel module to match ROUTING parameters. */

/* (C) 2001-2002 Andras Kis-Szabo <kisza@sch.bme.hu> */

// Kernel headers and build-time configuration provide the referenced types,
// constants, functions, and module-registration interfaces.

#[inline]
unsafe fn segsleft_match(min: u32, max: u32, id: u32, invert: bool) -> bool {
    ((id >= min && id <= max) as u8 != invert as u8)
}

unsafe fn rt_mt6(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let mut route: ipv6_rt_hdr = core::mem::zeroed();
    let mut rh: *const ipv6_rt_hdr;
    let rtinfo: *const ip6t_rt = (*par).matchinfo as *const ip6t_rt;
    let mut temp: u32;
    let mut ptr: u32 = 0;
    let mut hdrlen: u32 = 0;
    let mut ret = false;
    let mut addr: in6_addr = core::mem::zeroed();
    let mut ap: *const in6_addr;
    let err: i32;

    err = ipv6_find_hdr(skb, &mut ptr, NEXTHDR_ROUTING, core::ptr::null_mut(), core::ptr::null_mut());
    if err < 0 {
        if err != -ENOENT { (*par).hotdrop = true; }
        return false;
    }

    rh = skb_header_pointer(skb, ptr, core::mem::size_of::<ipv6_rt_hdr>(), &mut route as *mut _ as *mut core::ffi::c_void) as *const ipv6_rt_hdr;
    if rh.is_null() {
        (*par).hotdrop = true;
        return false;
    }

    hdrlen = ipv6_optlen(rh);
    if (*skb).len - ptr < hdrlen {
        /* Packet smaller than its length field */
        (*par).hotdrop = true;
        return false;
    }

    ret = segsleft_match((*rtinfo).segsleft[0], (*rtinfo).segsleft[1], (*rh).segments_left,
                         ((*rtinfo).invflags & IP6T_RT_INV_SGS) != 0)
        && (((*rtinfo).flags & IP6T_RT_LEN) == 0
            || (((*rtinfo).hdrlen == hdrlen) ^ (((*rtinfo).invflags & IP6T_RT_INV_LEN) != 0)))
        && (((*rtinfo).flags & IP6T_RT_TYP) == 0
            || (((*rtinfo).rt_type == (*rh).type) ^ (((*rtinfo).invflags & IP6T_RT_INV_TYP) != 0)));

    if ret && ((*rtinfo).flags & IP6T_RT_RES) != 0 {
        let mut reserved: u32 = 0;
        let rp = skb_header_pointer(skb, ptr + core::mem::offset_of!(rt0_hdr, reserved),
                                    core::mem::size_of::<u32>(), &mut reserved as *mut _ as *mut core::ffi::c_void) as *const u32;
        if rp.is_null() { (*par).hotdrop = true; return false; }
        ret = *rp == 0;
    }

    if ((*rtinfo).flags & IP6T_RT_FST) == 0 { return ret; }
    if ((*rtinfo).flags & IP6T_RT_FST_NSTRICT) != 0 {
        if (*rtinfo).addrnr > ((hdrlen - 8) / 16) { return false; }
        let mut i: u32 = 0;
        temp = 0;
        while temp < ((hdrlen - 8) / 16) {
            ap = skb_header_pointer(skb, ptr + core::mem::size_of::<rt0_hdr>() as u32 + temp * core::mem::size_of::<in6_addr>() as u32,
                                    core::mem::size_of::<in6_addr>(), &mut addr as *mut _ as *mut core::ffi::c_void) as *const in6_addr;
            if ap.is_null() { (*par).hotdrop = true; return false; }
            if ipv6_addr_equal(ap, &(*rtinfo).addrs[i as usize]) { i += 1; }
            if i == (*rtinfo).addrnr { break; }
            temp += 1;
        }
        return i == (*rtinfo).addrnr && ret;
    }

    if (*rtinfo).addrnr > ((hdrlen - 8) / 16) { return false; }
    temp = 0;
    while temp < (*rtinfo).addrnr {
        ap = skb_header_pointer(skb, ptr + core::mem::size_of::<rt0_hdr>() as u32 + temp * core::mem::size_of::<in6_addr>() as u32,
                                core::mem::size_of::<in6_addr>(), &mut addr as *mut _ as *mut core::ffi::c_void) as *const in6_addr;
        if ap.is_null() { (*par).hotdrop = true; return false; }
        if !ipv6_addr_equal(ap, &(*rtinfo).addrs[temp as usize]) { break; }
        temp += 1;
    }
    if temp == (*rtinfo).addrnr && temp == ((hdrlen - 8) / 16) { ret } else { false }
}

unsafe fn rt_mt6_check(par: *const xt_mtchk_param) -> i32 {
    let rtinfo = (*par).matchinfo as *const ip6t_rt;
    if ((*rtinfo).invflags & !IP6T_RT_INV_MASK) != 0 { pr_info_ratelimited!("unknown flags %X\n", (*rtinfo).invflags); return -EINVAL; }
    if (*rtinfo).addrnr > IP6T_RT_HOPS { pr_info_ratelimited!("too many addresses specified\n"); return -EINVAL; }
    if ((*rtinfo).flags & (IP6T_RT_RES | IP6T_RT_FST_MASK)) != 0 && (((*rtinfo).flags & IP6T_RT_TYP) == 0 || (*rtinfo).rt_type != 0 || ((*rtinfo).invflags & IP6T_RT_INV_TYP) != 0) {
        pr_info_ratelimited!("`--rt-type 0' required before `--rt-0-*'\n"); return -EINVAL;
    }
    0
}

static mut rt_mt6_reg: xt_match = xt_match {
    name: *b"rt\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    family: NFPROTO_IPV6,
    match_: Some(rt_mt6),
    matchsize: core::mem::size_of::<ip6t_rt>(),
    checkentry: Some(rt_mt6_check),
    me: THIS_MODULE,
};

unsafe fn rt_mt6_init() -> i32 { xt_register_match(&mut rt_mt6_reg) }
unsafe fn rt_mt6_exit() { xt_unregister_match(&mut rt_mt6_reg); }

module_init!(rt_mt6_init);
module_exit!(rt_mt6_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
