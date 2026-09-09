// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2022 Pablo Neira Ayuso <pablo@netfilter.org>
 */

#[repr(C)]
struct NftInnerTunCtxLocked {
    ctx: nft_inner_tun_ctx,
    bh_lock: local_lock_t,
}

// Same layout as nft_expr but it embeds the private expression data area.
#[repr(C)]
struct __nft_expr {
    ops: *const nft_expr_ops,
    payload_or_meta: __nft_expr_data,
}

#[repr(C)]
union __nft_expr_data {
    payload: nft_payload,
    meta: nft_meta,
}

const NFT_INNER_EXPR_PAYLOAD: u32 = 0;
const NFT_INNER_EXPR_META: u32 = 1;

#[repr(C)]
struct nft_inner {
    flags: u8,
    hdrsize: u8,
    type_: u8,
    expr_type: u8,
    expr: __nft_expr,
}

unsafe fn nft_inner_parse_l2l3(
    priv_: *const nft_inner,
    pkt: *const nft_pktinfo,
    ctx: *mut nft_inner_tun_ctx,
    mut off: u32,
) -> i32 {
    let mut llproto: __be16;
    let mut outer_llproto: __be16 = 0;
    let nhoff: u32;
    let mut thoff: u32;

    if (*priv_).flags as u32 & NFT_INNER_LL != 0 {
        let mut veth: *mut vlan_ethhdr;
        let mut _veth = core::mem::zeroed::<vlan_ethhdr>();
        let mut eth: *mut ethhdr;
        let mut _eth = core::mem::zeroed::<ethhdr>();
        let hdrsize: u32;

        eth = skb_header_pointer((*pkt).skb, off, core::mem::size_of::<ethhdr>(), &mut _eth as *mut _ as *mut _);
        if eth.is_null() { return -1; }
        match (*eth).h_proto {
            x if x == htons(ETH_P_IP) || x == htons(ETH_P_IPV6) => {
                llproto = (*eth).h_proto; hdrsize = core::mem::size_of::<ethhdr>() as u32;
            }
            x if x == htons(ETH_P_8021Q) => {
                veth = skb_header_pointer((*pkt).skb, off, core::mem::size_of::<vlan_ethhdr>(), &mut _veth as *mut _ as *mut _);
                if veth.is_null() { return -1; }
                outer_llproto = (*veth).h_vlan_encapsulated_proto;
                llproto = (*veth).h_vlan_proto;
                hdrsize = core::mem::size_of::<vlan_ethhdr>() as u32;
            }
            _ => return -1,
        }
        (*ctx).inner_lloff = off;
        (*ctx).flags |= NFT_PAYLOAD_CTX_INNER_LL;
        off += hdrsize;
    } else {
        let mut _version = 0u32;
        let iph = skb_header_pointer((*pkt).skb, off, core::mem::size_of::<u32>(), &mut _version as *mut _ as *mut iphdr);
        if iph.is_null() { return -1; }
        llproto = match (*iph).version { 4 => htons(ETH_P_IP), 6 => htons(ETH_P_IPV6), _ => return -1 };
    }
    (*ctx).llproto = llproto;
    if llproto == htons(ETH_P_8021Q) { llproto = outer_llproto; }
    nhoff = off;
    match llproto {
        x if x == htons(ETH_P_IP) => {
            let mut _iph = core::mem::zeroed::<iphdr>();
            let iph = skb_header_pointer((*pkt).skb, nhoff, core::mem::size_of::<iphdr>(), &mut _iph as *mut _ as *mut _);
            if iph.is_null() || (*iph).ihl < 5 || (*iph).version != 4 { return -1; }
            (*ctx).inner_nhoff = nhoff; (*ctx).flags |= NFT_PAYLOAD_CTX_INNER_NH;
            thoff = nhoff + ((*iph).ihl as u32 * 4);
            if (ntohs((*iph).frag_off) & IP_OFFSET) == 0 {
                (*ctx).flags |= NFT_PAYLOAD_CTX_INNER_TH; (*ctx).inner_thoff = thoff; (*ctx).l4proto = (*iph).protocol as i32;
            }
        }
        x if x == htons(ETH_P_IPV6) => {
            let mut _ip6h = core::mem::zeroed::<ipv6hdr>();
            let ip6h = skb_header_pointer((*pkt).skb, nhoff, core::mem::size_of::<ipv6hdr>(), &mut _ip6h as *mut _ as *mut _);
            if ip6h.is_null() || (*ip6h).version != 6 { return -1; }
            (*ctx).inner_nhoff = nhoff; (*ctx).flags |= NFT_PAYLOAD_CTX_INNER_NH;
            thoff = nhoff; let mut fragoff = 0u16; let mut fh_flags = IP6_FH_F_AUTH;
            let l4proto = ipv6_find_hdr((*pkt).skb, &mut thoff, -1, &mut fragoff, &mut fh_flags);
            if l4proto < 0 || thoff > U16_MAX as u32 { return -1; }
            if fragoff == 0 { (*ctx).flags |= NFT_PAYLOAD_CTX_INNER_TH; (*ctx).inner_thoff = thoff; (*ctx).l4proto = l4proto; }
        }
        _ => return -1,
    }
    0
}

unsafe fn nft_inner_parse_tunhdr(priv_: *const nft_inner, pkt: *const nft_pktinfo, ctx: *mut nft_inner_tun_ctx, off: *mut u32) -> i32 {
    if (*pkt).tprot == IPPROTO_GRE { (*ctx).inner_tunoff = (*pkt).thoff; (*ctx).flags |= NFT_PAYLOAD_CTX_INNER_TUN; return 0; }
    if (*pkt).tprot != IPPROTO_UDP { return -1; }
    (*ctx).inner_tunoff = *off; (*ctx).flags |= NFT_PAYLOAD_CTX_INNER_TUN; *off += (*priv_).hdrsize as u32;
    if (*priv_).type_ == NFT_INNER_GENEVE {
        let mut _gnvh = core::mem::zeroed::<genevehdr>();
        let gnvh = skb_header_pointer((*pkt).skb, (*pkt).inneroff, core::mem::size_of::<genevehdr>(), &mut _gnvh as *mut _ as *mut _);
        if gnvh.is_null() { return -1; }
        *off += (*gnvh).opt_len as u32 * 4;
    }
    0
}

unsafe fn nft_inner_parse(priv_: *const nft_inner, pkt: *mut nft_pktinfo, tun_ctx: *mut nft_inner_tun_ctx) -> i32 {
    let mut off = (*pkt).inneroff;
    if (*priv_).flags as u32 & NFT_INNER_HDRSIZE != 0 && nft_inner_parse_tunhdr(priv_, pkt, tun_ctx, &mut off) < 0 { return -1; }
    if (*priv_).flags as u32 & (NFT_INNER_LL | NFT_INNER_NH) != 0 {
        if nft_inner_parse_l2l3(priv_, pkt, tun_ctx, off) < 0 { return -1; }
    } else if (*priv_).flags as u32 & NFT_INNER_TH != 0 {
        (*tun_ctx).inner_thoff = off; (*tun_ctx).flags |= NFT_PAYLOAD_CTX_INNER_TH;
    }
    (*tun_ctx).type_ = (*priv_).type_; (*tun_ctx).cookie = (*pkt).skb as usize; (*pkt).flags |= NFT_PKTINFO_INNER_FULL; 0
}

unsafe fn nft_inner_restore_tun_ctx(pkt: *const nft_pktinfo, tun_ctx: *mut nft_inner_tun_ctx) -> bool { let this = this_cpu_ptr(&nft_pcpu_tun_ctx.ctx); (*this).cookie == (*pkt).skb as usize && { *tun_ctx = *this; true } }
unsafe fn nft_inner_save_tun_ctx(_pkt: *const nft_pktinfo, tun_ctx: *const nft_inner_tun_ctx) { let this = this_cpu_ptr(&nft_pcpu_tun_ctx.ctx); if (*this).cookie != (*tun_ctx).cookie { *this = *tun_ctx; } }
unsafe fn nft_inner_parse_needed(priv_: *const nft_inner, pkt: *const nft_pktinfo, tun_ctx: *mut nft_inner_tun_ctx) -> bool { !((*pkt).flags & NFT_PKTINFO_INNER_FULL != 0 && nft_inner_restore_tun_ctx(pkt, tun_ctx) && (*priv_).type_ == (*tun_ctx).type_) }

unsafe fn nft_inner_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr) as *const nft_inner; let mut tun_ctx = core::mem::zeroed::<nft_inner_tun_ctx>();
    if nft_payload_inner_offset(pkt) < 0 || (nft_inner_parse_needed(priv_, pkt, &mut tun_ctx) && nft_inner_parse(priv_, pkt as *mut _, &mut tun_ctx) < 0) { (*regs).verdict.code = NFT_BREAK; return; }
    match (*priv_).expr_type as u32 { NFT_INNER_EXPR_PAYLOAD => nft_payload_inner_eval(&(*priv_).expr as *const _ as *mut _, regs, pkt, &mut tun_ctx), NFT_INNER_EXPR_META => nft_meta_inner_eval(&(*priv_).expr as *const _ as *mut _, regs, pkt, &mut tun_ctx), _ => { DEBUG_NET_WARN_ON_ONCE(1); (*regs).verdict.code = NFT_BREAK; return; } }
    nft_inner_save_tun_ctx(pkt, &tun_ctx);
}

// External kernel declarations and registration objects are supplied by the surrounding translation unit.

#[repr(C)] struct nft_expr_info { ops: *const nft_expr_ops, attr: *const nlattr, tb: [*mut nlattr; NFT_EXPR_MAXATTR as usize + 1] }
static nft_inner_policy: [nla_policy; NFTA_INNER_MAX as usize + 1] = [nla_policy::default(); NFTA_INNER_MAX as usize + 1];

unsafe fn nft_inner_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut nft_inner;
    if (*tb.add(NFTA_INNER_FLAGS as usize)).is_null() || (*tb.add(NFTA_INNER_NUM as usize)).is_null() || (*tb.add(NFTA_INNER_HDRSIZE as usize)).is_null() || (*tb.add(NFTA_INNER_TYPE as usize)).is_null() || (*tb.add(NFTA_INNER_EXPR as usize)).is_null() { return -EINVAL; }
    let flags = ntohl(nla_get_be32(*tb.add(NFTA_INNER_FLAGS as usize))); if flags & !NFT_INNER_MASK != 0 { return -EOPNOTSUPP; }
    let num = ntohl(nla_get_be32(*tb.add(NFTA_INNER_NUM as usize))); if num != 0 { return -EOPNOTSUPP; }
    let hdrsize = ntohl(nla_get_be32(*tb.add(NFTA_INNER_HDRSIZE as usize))); let type_ = ntohl(nla_get_be32(*tb.add(NFTA_INNER_TYPE as usize)));
    if type_ > U8_MAX as u32 { return -EINVAL; } if flags & NFT_INNER_HDRSIZE != 0 && (hdrsize == 0 || hdrsize > 64) { return -EOPNOTSUPP; }
    (*priv_).flags = flags as u8; (*priv_).hdrsize = hdrsize as u8; (*priv_).type_ = type_ as u8;
    let mut info = core::mem::zeroed::<nft_expr_info>(); let err = nft_expr_inner_parse(ctx, *tb.add(NFTA_INNER_EXPR as usize), &mut info); if err < 0 { return err; }
    (*priv_).expr.ops = info.ops; let name = (*(*info.ops).type_).name;
    if strcmp(name, b"payload\0".as_ptr() as *const _) == 0 { (*priv_).expr_type = NFT_INNER_EXPR_PAYLOAD as u8; } else if strcmp(name, b"meta\0".as_ptr() as *const _) == 0 { (*priv_).expr_type = NFT_INNER_EXPR_META as u8; } else { return -EINVAL; }
    ((*(*info.ops).init)(ctx, &mut (*priv_).expr as *mut _ as *mut _, info.tb.as_ptr() as *const *const nlattr))
}

unsafe fn nft_inner_dump(skb: *mut sk_buff, expr: *const nft_expr, reset: bool) -> i32 {
    let priv_ = nft_expr_priv(expr) as *const nft_inner;
    if nla_put_be32(skb, NFTA_INNER_NUM, htonl(0)) != 0 || nla_put_be32(skb, NFTA_INNER_TYPE, htonl((*priv_).type_ as u32)) != 0 || nla_put_be32(skb, NFTA_INNER_FLAGS, htonl((*priv_).flags as u32)) != 0 || nla_put_be32(skb, NFTA_INNER_HDRSIZE, htonl((*priv_).hdrsize as u32)) != 0 { return -1; }
    if nft_expr_dump(skb, NFTA_INNER_EXPR, &(*priv_).expr as *const _ as *mut _, reset) < 0 { return -1; } 0
}

static nft_inner_ops: nft_expr_ops = nft_expr_ops { type_: &nft_inner_type, size: NFT_EXPR_SIZE(core::mem::size_of::<nft_inner>()), eval: nft_inner_eval, init: nft_inner_init, dump: nft_inner_dump };
static mut nft_inner_type: nft_expr_type = nft_expr_type { name: b"inner\0".as_ptr() as *const _, ops: &nft_inner_ops, policy: nft_inner_policy, maxattr: NFTA_INNER_MAX, owner: THIS_MODULE };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
