// SPDX-License-Identifier: GPL-2.0

// Kernel dependencies supplied by the surrounding translation unit.

#[cfg(feature = "CONFIG_NF_TABLES_IPV4")]
unsafe fn nf_route_table_hook4(
    priv_: *mut core::ffi::c_void,
    skb: *mut sk_buff,
    state: *const nf_hook_state,
) -> c_uint {
    let mut pkt: nft_pktinfo = core::mem::zeroed();
    let mut saddr: __be32;
    let mut daddr: __be32;
    let mut ret: c_uint;
    let mark: u32;
    let mut err: i32;
    let tos: u8;

    nft_set_pktinfo(&mut pkt, skb, state);
    nft_set_pktinfo_ipv4(&mut pkt);

    mark = (*skb).mark;
    let iph = ip_hdr(skb);
    saddr = (*iph).saddr;
    daddr = (*iph).daddr;
    tos = (*iph).tos;

    ret = nft_do_chain(&mut pkt, priv_);
    if ret == NF_ACCEPT {
        let iph = ip_hdr(skb);
        if (*iph).saddr != saddr
            || (*iph).daddr != daddr
            || (*skb).mark != mark
            || (*iph).tos != tos
        {
            err = ip_route_me_harder((*state).net, (*state).sk, skb, RTN_UNSPEC);
            if err < 0 {
                ret = NF_DROP_ERR(err);
            }
        }
    }
    ret
}

#[cfg(feature = "CONFIG_NF_TABLES_IPV4")]
static nft_chain_route_ipv4: nft_chain_type = nft_chain_type {
    name: "route",
    type_: NFT_CHAIN_T_ROUTE,
    family: NFPROTO_IPV4,
    hook_mask: 1 << NF_INET_LOCAL_OUT,
    hooks: [nf_route_table_hook4],
};

#[cfg(feature = "CONFIG_NF_TABLES_IPV6")]
unsafe fn nf_route_table_hook6(
    priv_: *mut core::ffi::c_void,
    skb: *mut sk_buff,
    state: *const nf_hook_state,
) -> c_uint {
    let mut saddr: in6_addr = core::mem::zeroed();
    let mut daddr: in6_addr = core::mem::zeroed();
    let mut pkt: nft_pktinfo = core::mem::zeroed();
    let mark: u32;
    let flowlabel: u32;
    let mut ret: c_uint;
    let hop_limit: u8;
    let mut err: i32;

    nft_set_pktinfo(&mut pkt, skb, state);
    nft_set_pktinfo_ipv6(&mut pkt);

    // save source/dest address, mark, hoplimit, flowlabel, priority
    core::ptr::copy_nonoverlapping(
        &(*ipv6_hdr(skb)).saddr,
        &mut saddr,
        1,
    );
    core::ptr::copy_nonoverlapping(
        &(*ipv6_hdr(skb)).daddr,
        &mut daddr,
        1,
    );
    mark = (*skb).mark;
    hop_limit = (*ipv6_hdr(skb)).hop_limit;

    // flowlabel and prio (includes version, which shouldn't change either)
    flowlabel = *(ipv6_hdr(skb) as *const u32);

    ret = nft_do_chain(&mut pkt, priv_);
    if ret == NF_ACCEPT
        && (core::memcmp(
            &(*ipv6_hdr(skb)).saddr as *const _ as *const u8,
            &saddr as *const _ as *const u8,
            core::mem::size_of::<in6_addr>(),
        ) != 0
            || core::memcmp(
                &(*ipv6_hdr(skb)).daddr as *const _ as *const u8,
                &daddr as *const _ as *const u8,
                core::mem::size_of::<in6_addr>(),
            ) != 0
            || (*skb).mark != mark
            || (*ipv6_hdr(skb)).hop_limit != hop_limit
            || flowlabel != *(ipv6_hdr(skb) as *const u32))
    {
        err = nf_ip6_route_me_harder((*state).net, (*state).sk, skb);
        if err < 0 {
            ret = NF_DROP_ERR(err);
        }
    }

    ret
}

#[cfg(feature = "CONFIG_NF_TABLES_IPV6")]
static nft_chain_route_ipv6: nft_chain_type = nft_chain_type {
    name: "route",
    type_: NFT_CHAIN_T_ROUTE,
    family: NFPROTO_IPV6,
    hook_mask: 1 << NF_INET_LOCAL_OUT,
    hooks: [nf_route_table_hook6],
};

#[cfg(feature = "CONFIG_NF_TABLES_INET")]
unsafe fn nf_route_table_inet(
    priv_: *mut core::ffi::c_void,
    skb: *mut sk_buff,
    state: *const nf_hook_state,
) -> c_uint {
    let mut pkt: nft_pktinfo = core::mem::zeroed();

    match (*state).pf {
        NFPROTO_IPV4 => nf_route_table_hook4(priv_, skb, state),
        NFPROTO_IPV6 => nf_route_table_hook6(priv_, skb, state),
        _ => {
            nft_set_pktinfo(&mut pkt, skb, state);
            nft_do_chain(&mut pkt, priv_)
        }
    }
}

#[cfg(feature = "CONFIG_NF_TABLES_INET")]
static nft_chain_route_inet: nft_chain_type = nft_chain_type {
    name: "route",
    type_: NFT_CHAIN_T_ROUTE,
    family: NFPROTO_INET,
    hook_mask: 1 << NF_INET_LOCAL_OUT,
    hooks: [nf_route_table_inet],
};

unsafe fn nft_chain_route_init() {
    #[cfg(feature = "CONFIG_NF_TABLES_IPV6")]
    nft_register_chain_type(&nft_chain_route_ipv6);
    #[cfg(feature = "CONFIG_NF_TABLES_IPV4")]
    nft_register_chain_type(&nft_chain_route_ipv4);
    #[cfg(feature = "CONFIG_NF_TABLES_INET")]
    nft_register_chain_type(&nft_chain_route_inet);
}

unsafe fn nft_chain_route_fini() {
    #[cfg(feature = "CONFIG_NF_TABLES_IPV6")]
    nft_unregister_chain_type(&nft_chain_route_ipv6);
    #[cfg(feature = "CONFIG_NF_TABLES_IPV4")]
    nft_unregister_chain_type(&nft_chain_route_ipv4);
    #[cfg(feature = "CONFIG_NF_TABLES_INET")]
    nft_unregister_chain_type(&nft_chain_route_inet);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
