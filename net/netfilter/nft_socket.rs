/* SPDX-License-Identifier: GPL-2.0 */
// Translated from nft_socket.c. Kernel includes and externally supplied symbols
// are intentionally represented by their Rust names and left to future dependencies.

#[repr(C)]
pub struct nft_socket {
    pub key: nft_socket_keys,
    pub level: u8,      /* cgroupv2 level to extract */
    pub level_user: u8, /* cgroupv2 level provided by userspace */
    pub len: u8,
    pub dreg: u8,
}

unsafe fn nft_socket_wildcard(
    pkt: *const nft_pktinfo,
    regs: *mut nft_regs,
    sk: *mut sock,
    dest: *mut u32,
) {
    match nft_pf(pkt) {
        NFPROTO_IPV4 => {
            nft_reg_store8(dest, (inet_sk(sk).inet_rcv_saddr == 0) as u8);
        }
        #[cfg(CONFIG_NF_TABLES_IPV6)]
        NFPROTO_IPV6 => {
            nft_reg_store8(dest, ipv6_addr_any(&(*sk).sk_v6_rcv_saddr) as u8);
        }
        _ => {
            (*regs).verdict.code = NFT_BREAK;
            return;
        }
    }
}

#[cfg(CONFIG_SOCK_CGROUP_DATA)]
unsafe fn nft_sock_get_eval_cgroupv2(
    dest: *mut u32,
    sk: *mut sock,
    _pkt: *const nft_pktinfo,
    level: u32,
) -> bool {
    if !sk_fullsock(sk) {
        return false;
    }

    let cgrp = cgroup_ancestor(sock_cgroup_ptr(&(*sk).sk_cgrp_data), level);
    if cgrp.is_null() {
        return false;
    }

    let cgid: u64 = cgroup_id(cgrp);
    core::ptr::copy_nonoverlapping(
        &cgid as *const u64 as *const u8,
        dest as *mut u8,
        core::mem::size_of::<u64>(),
    );
    true
}

#[cfg(CONFIG_SOCK_CGROUP_DATA)]
/* process context only, uses current->nsproxy. */
unsafe fn nft_socket_cgroup_subtree_level() -> i32 {
    let cgrp = cgroup_get_from_path("/");
    if is_err(cgrp) {
        return ptr_err(cgrp);
    }

    let level = (*cgrp).level;
    cgroup_put(cgrp);

    if level > 255 {
        return -ERANGE;
    }
    if level < 0 {
        debug_net_warn_on_once(1);
        return -EINVAL;
    }
    level
}

unsafe fn nft_socket_do_lookup(pkt: *const nft_pktinfo) -> *mut sock {
    let indev = nft_in(pkt);
    let skb = (*pkt).skb;
    let mut sk: *mut sock = core::ptr::null_mut();

    if indev.is_null() {
        return core::ptr::null_mut();
    }

    match nft_pf(pkt) {
        NFPROTO_IPV4 => sk = nf_sk_lookup_slow_v4(nft_net(pkt), skb, indev),
        #[cfg(CONFIG_NF_TABLES_IPV6)]
        NFPROTO_IPV6 => sk = nf_sk_lookup_slow_v6(nft_net(pkt), skb, indev),
        _ => debug_net_warn_on_once(1),
    }
    sk
}

unsafe fn nft_socket_eval(
    expr: *const nft_expr,
    regs: *mut nft_regs,
    pkt: *const nft_pktinfo,
) {
    let priv_: *const nft_socket = nft_expr_priv(expr);
    let skb = (*pkt).skb;
    let skb_sk = (*skb).sk;
    let mut sk = skb_sk;
    let dest = (*regs).data.as_mut_ptr().add((*priv_).dreg as usize);

    if !sk.is_null() && !net_eq(nft_net(pkt), sock_net(sk)) {
        sk = core::ptr::null_mut();
    }
    if sk.is_null() {
        sk = nft_socket_do_lookup(pkt);
    }
    if sk.is_null() {
        (*regs).verdict.code = NFT_BREAK;
        return;
    }

    match (*priv_).key {
        NFT_SOCKET_TRANSPARENT => nft_reg_store8(dest, inet_sk_transparent(sk)),
        NFT_SOCKET_MARK => {
            if sk_fullsock(sk) {
                *dest = read_once((*sk).sk_mark);
            } else {
                (*regs).verdict.code = NFT_BREAK;
                if sk != skb_sk { sock_gen_put(sk); }
                return;
            }
        }
        NFT_SOCKET_WILDCARD => {
            if !sk_fullsock(sk) {
                (*regs).verdict.code = NFT_BREAK;
                if sk != skb_sk { sock_gen_put(sk); }
                return;
            }
            nft_socket_wildcard(pkt, regs, sk, dest);
        }
        #[cfg(CONFIG_SOCK_CGROUP_DATA)]
        NFT_SOCKET_CGROUPV2 => {
            if !nft_sock_get_eval_cgroupv2(dest, sk, pkt, (*priv_).level as u32) {
                (*regs).verdict.code = NFT_BREAK;
                if sk != skb_sk { sock_gen_put(sk); }
                return;
            }
        }
        _ => {
            debug_net_warn_on_once(1);
            (*regs).verdict.code = NFT_BREAK;
        }
    }

    if sk != skb_sk { sock_gen_put(sk); }
}

static mut nft_socket_policy: [nla_policy; NFTA_SOCKET_MAX as usize + 1] = [
    nla_policy_max(NLA_BE32, 255),
    nla_policy_max(NLA_BE32, NFT_REG32_MAX),
    nla_policy_max(NLA_BE32, 255),
];

unsafe fn nft_socket_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut nft_socket;
    let mut len: usize;
    if (*tb.add(NFTA_SOCKET_DREG as usize)).is_null() || (*tb.add(NFTA_SOCKET_KEY as usize)).is_null() { return -EINVAL; }
    match (*ctx).family {
        NFPROTO_IPV4 | NFPROTO_INET => (),
        #[cfg(CONFIG_NF_TABLES_IPV6)]
        NFPROTO_IPV6 => (),
        _ => return -EOPNOTSUPP,
    }
    (*priv_).key = ntohl(nla_get_be32(*tb.add(NFTA_SOCKET_KEY as usize)));
    match (*priv_).key {
        NFT_SOCKET_TRANSPARENT | NFT_SOCKET_WILDCARD => len = core::mem::size_of::<u8>(),
        NFT_SOCKET_MARK => len = core::mem::size_of::<u32>(),
        #[cfg(CONFIG_SOCK_CGROUP_DATA)]
        NFT_SOCKET_CGROUPV2 => {
            let level = ntohl(nla_get_be32(*tb.add(NFTA_SOCKET_LEVEL as usize)));
            if (*tb.add(NFTA_SOCKET_LEVEL as usize)).is_null() { return -EINVAL; }
            if level > 255 { return -EOPNOTSUPP; }
            let err = nft_socket_cgroup_subtree_level();
            if err < 0 { return err; }
            (*priv_).level_user = level as u8;
            if level + err as u32 > 255 { return -EOPNOTSUPP; }
            (*priv_).level = (level + err as u32) as u8;
            len = core::mem::size_of::<u64>();
        }
        _ => return -EOPNOTSUPP,
    }
    (*priv_).len = len as u8;
    nft_parse_register_store(ctx, *tb.add(NFTA_SOCKET_DREG as usize), &mut (*priv_).dreg, core::ptr::null_mut(), NFT_DATA_VALUE, len)
}

// The remaining operation-table, module-registration, and metadata declarations
// retain their C names and external linkage for integration with kernel bindings.
unsafe fn nft_socket_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_ = nft_expr_priv(expr) as *const nft_socket;
    if nla_put_be32(skb, NFTA_SOCKET_KEY, htonl((*priv_).key)) != 0 { return -1; }
    if nft_dump_register(skb, NFTA_SOCKET_DREG, (*priv_).dreg) != 0 { return -1; }
    if (*priv_).key == NFT_SOCKET_CGROUPV2 && nla_put_be32(skb, NFTA_SOCKET_LEVEL, htonl((*priv_).level_user as u32)) != 0 { return -1; }
    0
}

unsafe fn nft_socket_validate(ctx: *const nft_ctx, _expr: *const nft_expr) -> i32 {
    if (*ctx).family != NFPROTO_IPV4 && (*ctx).family != NFPROTO_IPV6 && (*ctx).family != NFPROTO_INET { return -EOPNOTSUPP; }
    nft_chain_validate_hooks((*ctx).chain, (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_LOCAL_OUT))
}

// static struct nft_expr_type nft_socket_type;
// static const struct nft_expr_ops nft_socket_ops = { .type = &nft_socket_type, .size = NFT_EXPR_SIZE(sizeof(struct nft_socket)), .eval = nft_socket_eval, .init = nft_socket_init, .dump = nft_socket_dump, .validate = nft_socket_validate };
// static struct nft_expr_type nft_socket_type __read_mostly = { .name = "socket", .ops = &nft_socket_ops, .policy = nft_socket_policy, .maxattr = NFTA_SOCKET_MAX, .owner = THIS_MODULE };
unsafe fn nft_socket_module_init() -> i32 { nft_register_expr(&mut nft_socket_type) }
unsafe fn nft_socket_module_exit() { nft_unregister_expr(&mut nft_socket_type); }
// module_init(nft_socket_module_init); module_exit(nft_socket_module_exit);
// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Máté Eckl"); MODULE_DESCRIPTION("nf_tables socket match module"); MODULE_ALIAS_NFT_EXPR("socket");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
