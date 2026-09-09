// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C)2003-2006 Helsinki University of Technology
 * Copyright (C)2003-2006 USAGI/WIDE Project
 */
/*
 * Authors:
 *	Noriaki TAKAMIYA @USAGI
 *	Masahide NAKAMURA @USAGI
 */

// pr_fmt(fmt) KBUILD_MODNAME ": " fmt
// Linux kernel headers supplied by the surrounding translation unit.

#[inline]
unsafe fn calc_padlen(len: u32, n: u32) -> u32 {
    (n.wrapping_sub(len).wrapping_add(16)) & 0x7
}

#[inline]
unsafe fn mip6_padn(data: *mut u8, padlen: u8) -> *mut u8 {
    if data.is_null() { return core::ptr::null_mut(); }
    if padlen == 1 {
        *data = IPV6_TLV_PAD1 as u8;
    } else if padlen > 1 {
        *data = IPV6_TLV_PADN as u8;
        *data.add(1) = padlen.wrapping_sub(2);
        if padlen > 2 {
            core::ptr::write_bytes(data.add(2), 0, *data.add(1) as usize);
        }
    }
    data.add(padlen as usize)
}

#[inline]
unsafe fn mip6_param_prob(skb: *mut sk_buff, code: u8, pos: i32) {
    icmpv6_send(skb, ICMPV6_PARAMPROB, code, pos);
}

unsafe fn mip6_mh_len(typ: i32) -> i32 {
    match typ {
        IP6_MH_TYPE_BRR => 0,
        IP6_MH_TYPE_HOTI | IP6_MH_TYPE_COTI | IP6_MH_TYPE_BU | IP6_MH_TYPE_BACK => 1,
        IP6_MH_TYPE_HOT | IP6_MH_TYPE_COT | IP6_MH_TYPE_BERROR => 2,
        _ => 0,
    }
}

unsafe fn mip6_mh_filter(_sk: *mut sock, skb: *mut sk_buff) -> i32 {
    let mut hdr = core::mem::MaybeUninit::<ip6_mh>::uninit();
    let mh = skb_header_pointer(skb, skb_transport_offset(skb), core::mem::size_of::<ip6_mh>(), hdr.as_mut_ptr() as *mut _);
    if mh.is_null() { return -1; }
    if ((((*mh).ip6mh_hdrlen as usize + 1) << 3) as u32) > (*skb).len { return -1; }
    if (*mh).ip6mh_hdrlen as i32 < mip6_mh_len((*mh).ip6mh_type as i32) {
        net_dbg_ratelimited!("mip6: MH message too short: %d vs >=%d\n", (*mh).ip6mh_hdrlen, mip6_mh_len((*mh).ip6mh_type as i32));
        mip6_param_prob(skb, 0, core::mem::offset_of!(ip6_mh, ip6mh_hdrlen) as i32 + skb_network_header_len(skb));
        return -1;
    }
    if (*mh).ip6mh_proto != IPPROTO_NONE {
        net_dbg_ratelimited!("mip6: MH invalid payload proto = %d\n", (*mh).ip6mh_proto);
        mip6_param_prob(skb, 0, core::mem::offset_of!(ip6_mh, ip6mh_proto) as i32 + skb_network_header_len(skb));
        return -1;
    }
    0
}

#[repr(C)]
struct mip6_report_rate_limiter {
    lock: spinlock_t,
    stamp: ktime_t,
    iif: i32,
    src: in6_addr,
    dst: in6_addr,
}

static mut mip6_report_rl: mip6_report_rate_limiter = mip6_report_rate_limiter {
    lock: __SPIN_LOCK_UNLOCKED!(mip6_report_rl.lock),
    stamp: 0,
    iif: 0,
    src: unsafe { core::mem::zeroed() },
    dst: unsafe { core::mem::zeroed() },
};

unsafe fn mip6_destopt_input(x: *mut xfrm_state, skb: *mut sk_buff) -> i32 {
    let iph = ipv6_hdr(skb);
    let destopt = (*skb).data as *mut ipv6_destopt_hdr;
    let mut err = (*destopt).nexthdr as i32;
    spin_lock(&mut (*x).lock);
    if !ipv6_addr_equal(&(*iph).saddr, (*x).coaddr as *const in6_addr) && !ipv6_addr_any((*x).coaddr as *const in6_addr) { err = -ENOENT; }
    spin_unlock(&mut (*x).lock);
    err
}

unsafe fn mip6_destopt_output(x: *mut xfrm_state, skb: *mut sk_buff) -> i32 {
    skb_push(skb, -(skb_network_offset(skb) as i32));
    let iph = ipv6_hdr(skb);
    let nexthdr = *skb_mac_header(skb);
    *skb_mac_header(skb) = IPPROTO_DSTOPTS as u8;
    let dstopt = skb_transport_header(skb) as *mut ipv6_destopt_hdr;
    (*dstopt).nexthdr = nexthdr;
    let hao = mip6_padn((dstopt.add(1)) as *mut u8, calc_padlen(core::mem::size_of::<ipv6_destopt_hdr>() as u32, 6) as u8) as *mut ipv6_destopt_hao;
    (*hao).type_ = IPV6_TLV_HAO as u8;
    assert!(core::mem::size_of::<ipv6_destopt_hao>() == 18);
    (*hao).length = (core::mem::size_of::<ipv6_destopt_hao>() - 2) as u8;
    core::ptr::copy_nonoverlapping(&(*iph).saddr, &mut (*hao).addr, 1);
    spin_lock_bh(&mut (*x).lock);
    core::ptr::copy_nonoverlapping((*x).coaddr as *const in6_addr, &mut (*iph).saddr, 1);
    spin_unlock_bh(&mut (*x).lock);
    WARN_ON!((((hao as *mut u8).offset(-(dstopt as *mut u8 as isize)) as usize) + core::mem::size_of::<ipv6_destopt_hao>()) as u32 != (*x).props.header_len);
    (*dstopt).hdrlen = ((*x).props.header_len >> 3).wrapping_sub(1) as u8;
    0
}

#[inline]
unsafe fn mip6_report_rl_allow(stamp: ktime_t, dst: *const in6_addr, src: *const in6_addr, iif: i32) -> i32 {
    let mut allow = 0;
    spin_lock_bh(&mut mip6_report_rl.lock);
    if mip6_report_rl.stamp != stamp || mip6_report_rl.iif != iif || !ipv6_addr_equal(&mip6_report_rl.src, src) || !ipv6_addr_equal(&mip6_report_rl.dst, dst) {
        mip6_report_rl.stamp = stamp; mip6_report_rl.iif = iif; mip6_report_rl.src = *src; mip6_report_rl.dst = *dst; allow = 1;
    }
    spin_unlock_bh(&mut mip6_report_rl.lock); allow
}

unsafe fn mip6_destopt_reject(x: *mut xfrm_state, skb: *mut sk_buff, fl: *const flowi) -> i32 {
    let net = xs_net(x); let opt = (*skb).cb.as_mut_ptr() as *mut inet6_skb_parm; let fl6 = &(*fl).u.ip6;
    let mut hao: *mut ipv6_destopt_hao = core::ptr::null_mut();
    if fl6.flowi6_proto == IPPROTO_MH && fl6.fl6_mh_type <= IP6_MH_TYPE_MAX { return 0; }
    if (*opt).dsthao {
        let offset = ipv6_find_tlv(skb, (*opt).dsthao, IPV6_TLV_HAO);
        if offset >= 0 { hao = (skb_network_header(skb).add(offset as usize)) as *mut ipv6_destopt_hao; }
    }
    let stamp = skb_get_ktime(skb); let iph = ipv6_hdr(skb);
    if mip6_report_rl_allow(stamp, &(*iph).daddr, if hao.is_null() { &(*iph).saddr } else { &(*hao).addr }, (*opt).iif) == 0 { return 0; }
    let mut sel: xfrm_selector = core::mem::zeroed();
    core::ptr::copy_nonoverlapping(&(*iph).daddr as *const _ as *const u8, &mut sel.daddr as *mut _ as *mut u8, core::mem::size_of_val(&sel.daddr)); sel.prefixlen_d = 128;
    core::ptr::copy_nonoverlapping(&(*iph).saddr as *const _ as *const u8, &mut sel.saddr as *mut _ as *mut u8, core::mem::size_of_val(&sel.saddr)); sel.prefixlen_s = 128; sel.family = AF_INET6; sel.proto = fl6.flowi6_proto;
    sel.dport = xfrm_flowi_dport(fl, &fl6.uli); if sel.dport != 0 { sel.dport_mask = htons(!0); }
    sel.sport = xfrm_flowi_sport(fl, &fl6.uli); if sel.sport != 0 { sel.sport_mask = htons(!0); } sel.ifindex = fl6.flowi6_oif;
    km_report(net, IPPROTO_DSTOPTS, &sel, if hao.is_null() { core::ptr::null() } else { &(*hao).addr as *const _ as *const xfrm_address_t })
}

// The remaining XFRM callbacks and module registration are retained with the
// same signatures, ordering, and failure cleanup as the kernel implementation.
// External kernel declarations and structure definitions are provided by the
// surrounding translated headers.

unsafe fn mip6_destopt_init_state(x: *mut xfrm_state, extack: *mut netlink_ext_ack) -> i32 {
    if (*x).id.spi != 0 { NL_SET_ERR_MSG!(extack, "SPI must be 0"); return -EINVAL; }
    if (*x).props.mode != XFRM_MODE_ROUTEOPTIMIZATION { NL_SET_ERR_MSG!(extack, "XFRM mode must be XFRM_MODE_ROUTEOPTIMIZATION"); return -EINVAL; }
    (*x).props.header_len = (core::mem::size_of::<ipv6_destopt_hdr>() as u32) + calc_padlen(core::mem::size_of::<ipv6_destopt_hdr>() as u32, 6) + core::mem::size_of::<ipv6_destopt_hao>() as u32;
    WARN_ON!((*x).props.header_len != 24);
    0
}

unsafe fn mip6_destopt_destroy(_x: *mut xfrm_state) {}

unsafe fn mip6_rthdr_input(x: *mut xfrm_state, skb: *mut sk_buff) -> i32 {
    let iph = ipv6_hdr(skb); let rt2 = (*skb).data as *mut rt2_hdr; let mut err = (*rt2).rt_hdr.nexthdr as i32;
    spin_lock(&mut (*x).lock);
    if !ipv6_addr_equal(&(*iph).daddr, (*x).coaddr as *const in6_addr) && !ipv6_addr_any((*x).coaddr as *const in6_addr) { err = -ENOENT; }
    spin_unlock(&mut (*x).lock); err
}

unsafe fn mip6_rthdr_output(x: *mut xfrm_state, skb: *mut sk_buff) -> i32 {
    skb_push(skb, -(skb_network_offset(skb) as i32)); let iph = ipv6_hdr(skb);
    let nexthdr = *skb_mac_header(skb); *skb_mac_header(skb) = IPPROTO_ROUTING as u8;
    let rt2 = skb_transport_header(skb) as *mut rt2_hdr;
    (*rt2).rt_hdr.nexthdr = nexthdr; (*rt2).rt_hdr.hdrlen = ((*x).props.header_len >> 3).wrapping_sub(1) as u8; (*rt2).rt_hdr.type_ = IPV6_SRCRT_TYPE_2 as u8; (*rt2).rt_hdr.segments_left = 1;
    core::ptr::write_bytes(&mut (*rt2).reserved as *mut _, 0, core::mem::size_of_val(&(*rt2).reserved)); WARN_ON!((*rt2).rt_hdr.hdrlen != 2);
    core::ptr::copy_nonoverlapping(&(*iph).daddr, &mut (*rt2).addr, 1); spin_lock_bh(&mut (*x).lock); core::ptr::copy_nonoverlapping((*x).coaddr as *const in6_addr, &mut (*iph).daddr, 1); spin_unlock_bh(&mut (*x).lock); 0
}

unsafe fn mip6_rthdr_init_state(x: *mut xfrm_state, extack: *mut netlink_ext_ack) -> i32 {
    if (*x).id.spi != 0 { NL_SET_ERR_MSG!(extack, "SPI must be 0"); return -EINVAL; }
    if (*x).props.mode != XFRM_MODE_ROUTEOPTIMIZATION { NL_SET_ERR_MSG!(extack, "XFRM mode must be XFRM_MODE_ROUTEOPTIMIZATION"); return -EINVAL; }
    (*x).props.header_len = core::mem::size_of::<rt2_hdr>() as u32; 0
}
unsafe fn mip6_rthdr_destroy(_x: *mut xfrm_state) {}

static mip6_destopt_type: xfrm_type = xfrm_type {
    owner: THIS_MODULE, proto: IPPROTO_DSTOPTS, flags: XFRM_TYPE_NON_FRAGMENT | XFRM_TYPE_LOCAL_COADDR,
    init_state: Some(mip6_destopt_init_state), destructor: Some(mip6_destopt_destroy), input: Some(mip6_destopt_input), output: Some(mip6_destopt_output), reject: Some(mip6_destopt_reject),
};
static mip6_rthdr_type: xfrm_type = xfrm_type {
    owner: THIS_MODULE, proto: IPPROTO_ROUTING, flags: XFRM_TYPE_NON_FRAGMENT | XFRM_TYPE_REMOTE_COADDR,
    init_state: Some(mip6_rthdr_init_state), destructor: Some(mip6_rthdr_destroy), input: Some(mip6_rthdr_input), output: Some(mip6_rthdr_output),
};

unsafe fn mip6_init() -> i32 {
    pr_info!("Mobile IPv6\n");
    if xfrm_register_type(&mip6_destopt_type, AF_INET6) < 0 { pr_info!("%s: can't add xfrm type(destopt)\n", "mip6_init"); return -EAGAIN; }
    if xfrm_register_type(&mip6_rthdr_type, AF_INET6) < 0 { pr_info!("%s: can't add xfrm type(rthdr)\n", "mip6_init"); xfrm_unregister_type(&mip6_destopt_type, AF_INET6); return -EAGAIN; }
    if rawv6_mh_filter_register(mip6_mh_filter) < 0 { pr_info!("%s: can't add rawv6 mh filter\n", "mip6_init"); xfrm_unregister_type(&mip6_rthdr_type, AF_INET6); xfrm_unregister_type(&mip6_destopt_type, AF_INET6); return -EAGAIN; }
    0
}
unsafe fn mip6_fini() { if rawv6_mh_filter_unregister(mip6_mh_filter) < 0 { pr_info!("%s: can't remove rawv6 mh filter\n", "mip6_fini"); } xfrm_unregister_type(&mip6_rthdr_type, AF_INET6); xfrm_unregister_type(&mip6_destopt_type, AF_INET6); }

// module_init(mip6_init); module_exit(mip6_fini);
// MODULE_DESCRIPTION("IPv6 Mobility driver"); MODULE_LICENSE("GPL");
// MODULE_ALIAS_XFRM_TYPE(AF_INET6, XFRM_PROTO_DSTOPTS);
// MODULE_ALIAS_XFRM_TYPE(AF_INET6, XFRM_PROTO_ROUTING);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
