/* SPDX-License-Identifier: GPL-2.0 */
// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct nft_tproxy {
    pub sreg_addr: u8,
    pub sreg_port: u8,
    pub family: u8,
}

unsafe fn nft_tproxy_eval_v4(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_ = nft_expr_priv(expr);
    let skb = (*pkt).skb;
    let iph = ip_hdr(skb);
    let mut _hdr: udphdr = core::mem::zeroed();
    let mut hp: *mut udphdr;
    let mut taddr: __be32 = 0;
    let mut tport: __be16 = 0;
    let mut sk: *mut sock;

    if ((*pkt).tprot != IPPROTO_TCP && (*pkt).tprot != IPPROTO_UDP) || (*pkt).fragoff {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }

    hp = skb_header_pointer(skb, ip_hdrlen(skb), core::mem::size_of::<udphdr>(), &mut _hdr);
    if hp.is_null() {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }

    /* check if there's an ongoing connection on the packet addresses, this
     * happens if the redirect already happened and the current packet
     * belongs to an already established connection
     */
    sk = nf_tproxy_get_sock_v4(
        nft_net(pkt), skb, (*iph).protocol, (*iph).saddr, (*iph).daddr,
        (*hp).source, (*hp).dest, (*skb).dev, NF_TPROXY_LOOKUP_ESTABLISHED,
    );

    if (*priv_.sreg_addr != 0) {
        taddr = nft_reg_load_be32((*regs).data.as_ptr().add(*priv_.sreg_addr as usize));
    }
    taddr = nf_tproxy_laddr4(skb, taddr, (*iph).daddr);

    if (*priv_.sreg_port != 0) {
        tport = nft_reg_load_be16((*regs).data.as_ptr().add(*priv_.sreg_port as usize));
    }
    if tport == 0 { tport = (*hp).dest; }

    /* UDP has no TCP_TIME_WAIT state, so we never enter here */
    if !sk.is_null() && (*sk).sk_state == TCP_TIME_WAIT {
        /* reopening a TIME_WAIT connection needs special handling */
        sk = nf_tproxy_handle_time_wait4(nft_net(pkt), skb, taddr, tport, sk);
    } else if sk.is_null() {
        /* no, there's no established connection, check if
         * there's a listener on the redirected addr/port
         */
        sk = nf_tproxy_get_sock_v4(
            nft_net(pkt), skb, (*iph).protocol, (*iph).saddr, taddr,
            (*hp).source, tport, (*skb).dev, NF_TPROXY_LOOKUP_LISTENER,
        );
    }

    if !sk.is_null() && nf_tproxy_sk_is_transparent(sk) {
        nf_tproxy_assign_sock(skb, sk);
    } else {
        (*regs).verdict.code = NFT_BREAK;
    }
}

#[cfg(CONFIG_NF_TABLES_IPV6)]
unsafe fn nft_tproxy_eval_v6(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_ = nft_expr_priv(expr);
    let skb = (*pkt).skb;
    let iph = ipv6_hdr(skb);
    let thoff = nft_thoff(pkt);
    let mut _hdr: udphdr = core::mem::zeroed();
    let mut taddr: in6_addr = core::mem::zeroed();
    let mut tport: __be16 = 0;
    let mut sk: *mut sock;
    let l4proto: i32;

    if ((*pkt).tprot != IPPROTO_TCP && (*pkt).tprot != IPPROTO_UDP) || (*pkt).fragoff {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }
    l4proto = (*pkt).tprot;

    let hp = skb_header_pointer(skb, thoff, core::mem::size_of::<udphdr>(), &mut _hdr);
    if hp.is_null() {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }

    /* check if there's an ongoing connection on the packet addresses, this
     * happens if the redirect already happened and the current packet
     * belongs to an already established connection
     */
    sk = nf_tproxy_get_sock_v6(
        nft_net(pkt), skb, thoff, l4proto, &(*iph).saddr, &(*iph).daddr,
        (*hp).source, (*hp).dest, nft_in(pkt), NF_TPROXY_LOOKUP_ESTABLISHED,
    );

    if *priv_.sreg_addr != 0 {
        core::ptr::copy_nonoverlapping(
            (*regs).data.as_ptr().add(*priv_.sreg_addr as usize),
            &mut taddr as *mut _ as *mut u8,
            core::mem::size_of::<in6_addr>(),
        );
    }
    taddr = *nf_tproxy_laddr6(skb, &mut taddr, &(*iph).daddr);
    if *priv_.sreg_port != 0 { tport = nft_reg_load_be16((*regs).data.as_ptr().add(*priv_.sreg_port as usize)); }
    if tport == 0 { tport = (*hp).dest; }

    /* UDP has no TCP_TIME_WAIT state, so we never enter here */
    if !sk.is_null() && (*sk).sk_state == TCP_TIME_WAIT {
        /* reopening a TIME_WAIT connection needs special handling */
        sk = nf_tproxy_handle_time_wait6(skb, l4proto, thoff, nft_net(pkt), &mut taddr, tport, sk);
    } else if sk.is_null() {
        /* no there's no established connection, check if
         * there's a listener on the redirected addr/port
         */
        sk = nf_tproxy_get_sock_v6(
            nft_net(pkt), skb, thoff, l4proto, &(*iph).saddr, &taddr,
            (*hp).source, tport, nft_in(pkt), NF_TPROXY_LOOKUP_LISTENER,
        );
    }

    /* NOTE: assign_sock consumes our sk reference */
    if !sk.is_null() && nf_tproxy_sk_is_transparent(sk) { nf_tproxy_assign_sock(skb, sk); }
    else { (*regs).verdict.code = NFT_BREAK; }
}

unsafe fn nft_tproxy_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr);
    match nft_pf(pkt) {
        NFPROTO_IPV4 => match (*priv_).family {
            NFPROTO_IPV4 | NFPROTO_UNSPEC => { nft_tproxy_eval_v4(expr, regs, pkt); return; }
            _ => {}
        },
        #[cfg(CONFIG_NF_TABLES_IPV6)]
        NFPROTO_IPV6 => match (*priv_).family {
            NFPROTO_IPV6 | NFPROTO_UNSPEC => { nft_tproxy_eval_v6(expr, regs, pkt); return; }
            _ => {}
        },
        _ => {}
    }
    (*regs).verdict.code = NFT_BREAK;
}

static nft_tproxy_policy: [nla_policy; NFTA_TPROXY_MAX as usize + 1] = [
    /* NFTA_TPROXY_FAMILY = NLA_POLICY_MAX(NLA_BE32, 255) */
    /* NFTA_TPROXY_REG_ADDR = { .type = NLA_U32 } */
    /* NFTA_TPROXY_REG_PORT = { .type = NLA_U32 } */
];

unsafe fn nft_tproxy_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    let priv_ = nft_expr_priv(expr);
    let mut alen: usize = 0;
    let mut err: i32;
    if (*tb.add(NFTA_TPROXY_FAMILY as usize)).is_null()
        || ((*tb.add(NFTA_TPROXY_REG_ADDR as usize)).is_null()
            && (*tb.add(NFTA_TPROXY_REG_PORT as usize)).is_null()) { return -EINVAL; }
    (*priv_).family = ntohl(nla_get_be32(*tb.add(NFTA_TPROXY_FAMILY as usize))) as u8;
    match (*ctx).family {
        NFPROTO_IPV4 => if (*priv_).family != NFPROTO_IPV4 { return -EINVAL; },
        #[cfg(CONFIG_NF_TABLES_IPV6)]
        NFPROTO_IPV6 => if (*priv_).family != NFPROTO_IPV6 { return -EINVAL; },
        NFPROTO_INET => {},
        _ => return -EOPNOTSUPP,
    }
    /* Address is specified but the rule family is not set accordingly */
    if (*priv_).family == NFPROTO_UNSPEC && !(*tb.add(NFTA_TPROXY_REG_ADDR as usize)).is_null() { return -EINVAL; }
    match (*priv_).family {
        NFPROTO_IPV4 => { alen = core::mem::size_of::<__be32>(); err = nf_defrag_ipv4_enable((*ctx).net); if err != 0 { return err; } },
        #[cfg(CONFIG_NF_TABLES_IPV6)]
        NFPROTO_IPV6 => { alen = core::mem::size_of::<in6_addr>(); err = nf_defrag_ipv6_enable((*ctx).net); if err != 0 { return err; } },
        NFPROTO_UNSPEC => {
            err = nf_defrag_ipv4_enable((*ctx).net); if err != 0 { return err; }
            #[cfg(CONFIG_NF_TABLES_IPV6)] { err = nf_defrag_ipv6_enable((*ctx).net); if err != 0 { return err; } }
        },
        _ => return -EOPNOTSUPP,
    }
    if !(*tb.add(NFTA_TPROXY_REG_ADDR as usize)).is_null() {
        err = nft_parse_register_load(ctx, *tb.add(NFTA_TPROXY_REG_ADDR as usize), &mut (*priv_).sreg_addr, alen); if err < 0 { return err; }
    }
    if !(*tb.add(NFTA_TPROXY_REG_PORT as usize)).is_null() {
        err = nft_parse_register_load(ctx, *tb.add(NFTA_TPROXY_REG_PORT as usize), &mut (*priv_).sreg_port, core::mem::size_of::<u16>()); if err < 0 { return err; }
    }
    0
}

unsafe fn nft_tproxy_destroy(ctx: *const nft_ctx, expr: *const nft_expr) {
    match (*nft_expr_priv(expr)).family {
        NFPROTO_IPV4 => { nf_defrag_ipv4_disable((*ctx).net); }
        #[cfg(CONFIG_NF_TABLES_IPV6)] NFPROTO_IPV6 => { nf_defrag_ipv6_disable((*ctx).net); }
        NFPROTO_UNSPEC => { nf_defrag_ipv4_disable((*ctx).net); #[cfg(CONFIG_NF_TABLES_IPV6)] nf_defrag_ipv6_disable((*ctx).net); }
        _ => {}
    }
}

unsafe fn nft_tproxy_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_ = nft_expr_priv(expr);
    if nla_put_be32(skb, NFTA_TPROXY_FAMILY, htonl((*priv_).family as u32)) != 0 { return -1; }
    if (*priv_).sreg_addr != 0 && nft_dump_register(skb, NFTA_TPROXY_REG_ADDR, (*priv_).sreg_addr) != 0 { return -1; }
    if (*priv_).sreg_port != 0 && nft_dump_register(skb, NFTA_TPROXY_REG_PORT, (*priv_).sreg_port) != 0 { return -1; }
    0
}

unsafe fn nft_tproxy_validate(ctx: *const nft_ctx, _expr: *const nft_expr) -> i32 {
    if (*ctx).family != NFPROTO_IPV4 && (*ctx).family != NFPROTO_IPV6 && (*ctx).family != NFPROTO_INET { return -EOPNOTSUPP; }
    nft_chain_validate_hooks((*ctx).chain, 1 << NF_INET_PRE_ROUTING)
}

// C static operation/type objects and module_init/module_exit metadata are
// represented as external kernel integration declarations in the Rust port.
extern "C" {
    static mut nft_tproxy_type: nft_expr_type;
    fn nft_register_expr(ty: *mut nft_expr_type) -> i32;
    fn nft_unregister_expr(ty: *mut nft_expr_type);
}

unsafe fn nft_tproxy_module_init() -> i32 { nft_register_expr(&mut nft_tproxy_type) }
unsafe fn nft_tproxy_module_exit() { nft_unregister_expr(&mut nft_tproxy_type); }

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Máté Eckl");
// MODULE_DESCRIPTION("nf_tables tproxy support module");
// MODULE_ALIAS_NFT_EXPR("tproxy");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
