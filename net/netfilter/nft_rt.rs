// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016 Anders K. Pedersen <akp@cohaesio.com>
 */

// Linux kernel headers supplied by the surrounding translation unit.

#[repr(C)]
pub struct nft_rt {
    pub key: nft_rt_keys,
    pub dreg: u8,
}

unsafe fn get_tcpmss(pkt: *const nft_pktinfo, skbdst: *const dst_entry) -> u16 {
    let mut minlen: u32 = core::mem::size_of::<ipv6hdr>() as u32;
    let mut mtu: u32 = dst_mtu(skbdst);
    let skb: *const sk_buff = (*pkt).skb;
    let mut dst: *mut dst_entry = core::ptr::null_mut();
    let mut fl: flowi = core::mem::zeroed();

    match nft_pf(pkt) {
        NFPROTO_IPV4 => {
            (*fl.u.ip4_mut()).daddr = (*ip_hdr(skb)).saddr;
            minlen = (core::mem::size_of::<iphdr>() + core::mem::size_of::<tcphdr>()) as u32;
        }
        NFPROTO_IPV6 => {
            (*fl.u.ip6_mut()).daddr = (*ipv6_hdr(skb)).saddr;
            minlen = (core::mem::size_of::<ipv6hdr>() + core::mem::size_of::<tcphdr>()) as u32;
        }
        _ => {}
    }

    nf_route(nft_net(pkt), &mut dst, &mut fl, false, nft_pf(pkt));
    if !dst.is_null() {
        mtu = core::cmp::min(mtu, dst_mtu(dst));
        dst_release(dst);
    }

    if mtu <= minlen || mtu > 0xffff {
        return TCP_MSS_DEFAULT;
    }
    (mtu - minlen) as u16
}

pub unsafe fn nft_rt_get_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *const nft_rt = nft_expr_priv(expr);
    let skb: *const sk_buff = (*pkt).skb;
    let dest: *mut u32 = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize);
    let dst: *const dst_entry;

    if !skb_valid_dst(skb) {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }
    dst = skb_dst(skb);

    match (*priv_).key {
        // CONFIG_IP_ROUTE_CLASSID
        NFT_RT_CLASSID => {
            *dest = (*dst).tclassid;
        }
        NFT_RT_NEXTHOP4 => {
            if nft_pf(pkt) != NFPROTO_IPV4 {
                (*regs).verdict.code = NFT_BREAK;
                return;
            }
            *dest = rt_nexthop(dst_rtable(dst), (*ip_hdr(skb)).daddr) as u32;
        }
        NFT_RT_NEXTHOP6 => {
            if nft_pf(pkt) != NFPROTO_IPV6 {
                (*regs).verdict.code = NFT_BREAK;
                return;
            }
            core::ptr::copy_nonoverlapping(
                rt6_nexthop(dst_rt6_info(dst), &(*ipv6_hdr(skb)).daddr),
                dest as *mut u8,
                core::mem::size_of::<in6_addr>(),
            );
        }
        NFT_RT_TCPMSS => {
            nft_reg_store16(dest, get_tcpmss(pkt, dst));
        }
        // CONFIG_XFRM
        NFT_RT_XFRM => {
            nft_reg_store8(dest, if !(*dst).xfrm.is_null() { 1 } else { 0 });
        }
        _ => {
            DEBUG_NET_WARN_ON_ONCE(1);
            (*regs).verdict.code = NFT_BREAK;
        }
    }
}

pub static nft_rt_policy: [nla_policy; NFTA_RT_MAX as usize + 1] = [
    // [NFTA_RT_DREG] = NLA_POLICY_MAX(NLA_BE32, NFT_REG32_MAX),
    // [NFTA_RT_KEY] = NLA_POLICY_MAX(NLA_BE32, 255),
];

unsafe fn nft_rt_get_init(
    ctx: *const nft_ctx,
    expr: *const nft_expr,
    tb: *const *const nlattr,
) -> c_int {
    let priv_: *mut nft_rt = nft_expr_priv_mut(expr);
    let len: usize;
    if (*tb.add(NFTA_RT_KEY as usize)).is_null() || (*tb.add(NFTA_RT_DREG as usize)).is_null() {
        return -EINVAL;
    }

    (*priv_).key = ntohl(nla_get_be32(*tb.add(NFTA_RT_KEY as usize)));
    len = match (*priv_).key {
        NFT_RT_CLASSID | NFT_RT_NEXTHOP4 => core::mem::size_of::<u32>(),
        NFT_RT_NEXTHOP6 => core::mem::size_of::<in6_addr>(),
        NFT_RT_TCPMSS => core::mem::size_of::<u16>(),
        NFT_RT_XFRM => core::mem::size_of::<u8>(),
        _ => return -EOPNOTSUPP,
    };
    nft_parse_register_store(ctx, *tb.add(NFTA_RT_DREG as usize), &mut (*priv_).dreg, core::ptr::null_mut(), NFT_DATA_VALUE, len)
}

unsafe fn nft_rt_get_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> c_int {
    let priv_: *const nft_rt = nft_expr_priv(expr);
    if nla_put_be32(skb, NFTA_RT_KEY, htonl((*priv_).key)) != 0 {
        return -1;
    }
    if nft_dump_register(skb, NFTA_RT_DREG, (*priv_).dreg) != 0 {
        return -1;
    }
    0
}

unsafe fn nft_rt_validate(ctx: *const nft_ctx, expr: *const nft_expr) -> c_int {
    let priv_: *const nft_rt = nft_expr_priv(expr);
    let hooks: u32;
    if (*ctx).family != NFPROTO_IPV4 && (*ctx).family != NFPROTO_IPV6 && (*ctx).family != NFPROTO_INET {
        return -EOPNOTSUPP;
    }
    match (*priv_).key {
        NFT_RT_NEXTHOP4 | NFT_RT_NEXTHOP6 | NFT_RT_CLASSID | NFT_RT_XFRM => return 0,
        NFT_RT_TCPMSS => {
            hooks = (1 << NF_INET_FORWARD) | (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_POST_ROUTING);
        }
        _ => return -EINVAL,
    }
    nft_chain_validate_hooks((*ctx).chain, hooks)
}

pub static nft_rt_get_ops: nft_expr_ops = nft_expr_ops {
    type_: &nft_rt_type,
    size: NFT_EXPR_SIZE(core::mem::size_of::<nft_rt>()),
    eval: nft_rt_get_eval,
    init: nft_rt_get_init,
    dump: nft_rt_get_dump,
    validate: nft_rt_validate,
};

pub static mut nft_rt_type: nft_expr_type = nft_expr_type {
    name: "rt",
    ops: &nft_rt_get_ops,
    policy: &nft_rt_policy,
    maxattr: NFTA_RT_MAX,
    owner: THIS_MODULE,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
