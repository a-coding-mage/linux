// SPDX-License-Identifier: GPL-2.0-only

// Linux kernel includes translated as external dependencies supplied elsewhere.

unsafe fn get_ifindex(dev: *const net_device) -> i32 {
    if !dev.is_null() { (*dev).ifindex } else { 0 }
}

unsafe fn nft_fib6_flowi_init(
    fl6: *mut flowi6,
    priv_: *const nft_fib,
    pkt: *const nft_pktinfo,
    dev: *const net_device,
    iph: *mut ipv6hdr,
) -> i32 {
    let mut lookup_flags: i32 = 0;

    if (*priv_).flags & NFTA_FIB_F_DADDR != 0 {
        (*fl6).daddr = (*iph).daddr;
        (*fl6).saddr = (*iph).saddr;
    } else {
        if nft_hook(pkt) == NF_INET_FORWARD && (*priv_).flags & NFTA_FIB_F_IIF != 0 {
            (*fl6).flowi6_iif = (*nft_out(pkt)).ifindex as _;
        }

        (*fl6).daddr = (*iph).saddr;
        (*fl6).saddr = (*iph).daddr;
    }

    if ipv6_addr_type(&(*fl6).daddr) & IPV6_ADDR_LINKLOCAL != 0 {
        lookup_flags |= RT6_LOOKUP_F_IFACE;
        let selected_dev = if !dev.is_null() { dev } else { (*(*pkt).skb).dev };
        (*fl6).flowi6_oif = get_ifindex(selected_dev) as _;
    }

    if ipv6_addr_type(&(*fl6).saddr) & IPV6_ADDR_UNICAST != 0 {
        lookup_flags |= RT6_LOOKUP_F_HAS_SADDR;
    }

    if (*priv_).flags & NFTA_FIB_F_MARK != 0 {
        (*fl6).flowi6_mark = (*(*pkt).skb).mark;
    }

    (*fl6).flowlabel = (*(iph as *const u32)) & IPV6_FLOWINFO_MASK;
    (*fl6).flowi6_l3mdev = nft_fib_l3mdev_master_ifindex_rcu(pkt, dev);

    lookup_flags | RT6_LOOKUP_F_DST_NOREF
}

unsafe fn nft_fib6_lookup(
    net: *mut net,
    fl6: *mut flowi6,
    res: *mut fib6_result,
    flags: i32,
) -> i32 {
    fib6_lookup(net, (*fl6).flowi6_oif, fl6, res, flags)
}

unsafe fn __nft_fib6_eval_type(
    priv_: *const nft_fib,
    pkt: *const nft_pktinfo,
    iph: *mut ipv6hdr,
) -> u32 {
    let mut dev: *const net_device = core::ptr::null();
    let mut res: fib6_result = core::mem::zeroed();
    let mut route_err: i32;
    let mut addrtype: i32;
    let mut fl6: flowi6 = core::mem::zeroed();
    fl6.flowi6_iif = LOOPBACK_IFINDEX;
    fl6.flowi6_proto = (*pkt).tprot;
    fl6.flowi6_uid = sock_net_uid(nft_net(pkt), core::ptr::null_mut());
    let mut ret: u32 = 0;

    if (*priv_).flags & NFTA_FIB_F_IIF != 0 {
        dev = nft_in(pkt);
    } else if (*priv_).flags & NFTA_FIB_F_OIF != 0 {
        dev = nft_out(pkt);
    }

    let lookup_flags = nft_fib6_flowi_init(&mut fl6, priv_, pkt, dev, iph);

    if !dev.is_null() && nf_ipv6_chk_addr(nft_net(pkt), &fl6.daddr, dev, true) {
        ret = RTN_LOCAL;
    }

    route_err = nft_fib6_lookup(nft_net(pkt), &mut fl6, &mut res, lookup_flags);
    if route_err != 0 {
        return match route_err {
            -EINVAL => RTN_BLACKHOLE,
            -EACCES => RTN_PROHIBIT,
            -EAGAIN => RTN_THROW,
            _ => RTN_UNREACHABLE,
        };
    }

    if res.fib6_flags & RTF_REJECT != 0 { return res.fib6_type; }

    if __ipv6_anycast_destination(&(*res.f6i).fib6_dst, res.fib6_flags, &fl6.daddr) {
        ret = RTN_ANYCAST;
    } else if dev.is_null() && res.fib6_flags & RTF_LOCAL != 0 {
        ret = RTN_LOCAL;
    }

    if ret != 0 { return ret; }

    addrtype = ipv6_addr_type(&fl6.daddr);
    if addrtype & IPV6_ADDR_MULTICAST != 0 { return RTN_MULTICAST; }
    if addrtype & IPV6_ADDR_UNICAST != 0 { return RTN_UNICAST; }
    return RTN_UNSPEC;

}

unsafe fn nft_fib6_eval_type(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr);
    let noff = skb_network_offset((*pkt).skb);
    let dest = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize);
    let mut iph: *mut ipv6hdr;
    let mut _iph: ipv6hdr = core::mem::zeroed();

    iph = skb_header_pointer((*pkt).skb, noff, core::mem::size_of::<ipv6hdr>(), &mut _iph);
    if iph.is_null() {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }
    *dest = __nft_fib6_eval_type(priv_, pkt, iph);
}

unsafe fn nft_fib_v6_skip_icmpv6(skb: *const sk_buff, next: u8, iph: *const ipv6hdr) -> bool {
    if likely(next != IPPROTO_ICMPV6) { return false; }
    if ipv6_addr_type(&(*iph).saddr) != IPV6_ADDR_ANY { return false; }
    ipv6_addr_type(&(*iph).daddr) & IPV6_ADDR_LINKLOCAL != 0
}

unsafe fn nft_fib6_info_nh_dev_match(nh_dev: *const net_device, dev: *const net_device) -> bool {
    nh_dev == dev || l3mdev_master_ifindex_rcu(nh_dev) == (*dev).ifindex
}

unsafe extern "C" fn nft_fib6_nh_match_dev_cb(nh: *mut fib6_nh, arg: *mut core::ffi::c_void) -> i32 {
    let dev = arg as *const net_device;
    nft_fib6_info_nh_dev_match((*nh).fib_nh_dev, dev) as i32
}

unsafe fn nft_fib6_info_nh_uses_dev(rt: *mut fib6_info, dev: *const net_device) -> bool {
    let mut nh_dev: *const net_device;
    let mut iter: *mut fib6_info;

    /* External nexthop: fib6_siblings slot aliases nh_list, walk via nh. */
    if !(*rt).nh.is_null() {
        return nexthop_for_each_fib6_nh((*rt).nh, Some(nft_fib6_nh_match_dev_cb), dev as *mut _);
    }

    nh_dev = fib6_info_nh_dev(rt);
    if nft_fib6_info_nh_dev_match(nh_dev, dev) { return true; }
    if READ_ONCE((*rt).fib6_nsiblings) == 0 { return false; }

    list_for_each_entry_rcu!(iter, (*rt).fib6_siblings, fib6_siblings, {
        nh_dev = fib6_info_nh_dev(iter);
        if nft_fib6_info_nh_dev_match(nh_dev, dev) { return true; }
        if READ_ONCE((*rt).fib6_nsiblings) == 0 { return false; }
    });
    false
}

unsafe fn nft_fib6_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr);
    let noff = skb_network_offset((*pkt).skb);
    let mut found: *const net_device = core::ptr::null();
    let mut oif: *const net_device = core::ptr::null();
    let dest = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize);
    let mut res: fib6_result = core::mem::zeroed();
    let mut iph: *mut ipv6hdr;
    let mut _iph: ipv6hdr = core::mem::zeroed();
    let mut fl6: flowi6 = core::mem::zeroed();
    fl6.flowi6_iif = LOOPBACK_IFINDEX;
    fl6.flowi6_proto = (*pkt).tprot;
    fl6.flowi6_uid = sock_net_uid(nft_net(pkt), core::ptr::null_mut());

    if nft_fib_can_skip(pkt) {
        nft_fib_store_result(dest, priv_, nft_in(pkt));
        return;
    }
    if (*priv_).flags & NFTA_FIB_F_IIF != 0 { oif = nft_in(pkt); }
    else if (*priv_).flags & NFTA_FIB_F_OIF != 0 { oif = nft_out(pkt); }

    iph = skb_header_pointer((*pkt).skb, noff, core::mem::size_of::<ipv6hdr>(), &mut _iph);
    if iph.is_null() { (*regs).verdict.code = NFT_BREAK; return; }
    if nft_fib_v6_skip_icmpv6((*pkt).skb, (*pkt).tprot, iph) {
        nft_fib_store_result(dest, priv_, nft_in(pkt)); return;
    }

    let lookup_flags = nft_fib6_flowi_init(&mut fl6, priv_, pkt, oif, iph);
    nft_fib_store_result(dest, priv_, core::ptr::null());
    let ret = nft_fib6_lookup(nft_net(pkt), &mut fl6, &mut res, lookup_flags);
    if ret != 0 || res.fib6_flags & (RTF_REJECT | RTF_ANYCAST | RTF_LOCAL) != 0 { return; }
    if oif.is_null() { found = fib6_info_nh_dev(res.f6i); }
    else if nft_fib6_info_nh_uses_dev(res.f6i, oif) { found = oif; }
    nft_fib_store_result(dest, priv_, found);
}

static mut nft_fib6_type: nft_expr_type = unsafe { core::mem::zeroed() };

static mut nft_fib6_type_ops: nft_expr_ops = unsafe { core::mem::zeroed() };
static mut nft_fib6_ops: nft_expr_ops = unsafe { core::mem::zeroed() };

unsafe fn nft_fib6_select_ops(
    _ctx: *const nft_ctx,
    tb: *const *const nlattr,
) -> *const nft_expr_ops {
    if (*tb.add(NFTA_FIB_RESULT as usize)).is_null() { return ERR_PTR(-EINVAL); }
    match ntohl(nla_get_be32(*tb.add(NFTA_FIB_RESULT as usize))) {
        NFT_FIB_RESULT_OIF | NFT_FIB_RESULT_OIFNAME => &nft_fib6_ops,
        NFT_FIB_RESULT_ADDRTYPE => &nft_fib6_type_ops,
        _ => ERR_PTR(-EOPNOTSUPP),
    }
}

#[no_mangle]
pub unsafe extern "C" fn nft_fib6_module_init() -> i32 { nft_register_expr(&mut nft_fib6_type) }

#[no_mangle]
pub unsafe extern "C" fn nft_fib6_module_exit() { nft_unregister_expr(&mut nft_fib6_type); }

// EXPORT_SYMBOL_GPL(nft_fib6_eval_type), EXPORT_SYMBOL_GPL(nft_fib6_eval),
// module_init/module_exit, MODULE_LICENSE("GPL"), MODULE_AUTHOR,
// MODULE_ALIAS_NFT_AF_EXPR(10, "fib"), and MODULE_DESCRIPTION are retained
// as build-system/module metadata supplied by the surrounding kernel bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
