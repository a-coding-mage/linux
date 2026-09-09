// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2008-2009 Patrick McHardy <kaber@trash.net>
 * Copyright (c) 2012 Pablo Neira Ayuso <pablo@netfilter.org>
 * Copyright (c) 2012 Intel Corporation
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

#[repr(C)]
struct nft_nat {
    sreg_addr_min: u8,
    sreg_addr_max: u8,
    sreg_proto_min: u8,
    sreg_proto_max: u8,
    r#type: nf_nat_manip_type,
    family: u8,
    flags: u16,
}

unsafe fn nft_nat_setup_addr(range: *mut nf_nat_range2, regs: *const nft_regs, priv_: *const nft_nat) {
    match (*priv_).family as u32 {
        AF_INET => {
            (*range).min_addr.ip = (*regs).data[(*priv_).sreg_addr_min as usize] as __be32;
            (*range).max_addr.ip = (*regs).data[(*priv_).sreg_addr_max as usize] as __be32;
        }
        AF_INET6 => {
            memcpy((*range).min_addr.ip6.as_mut_ptr() as *mut _, &(*regs).data[(*priv_).sreg_addr_min as usize] as *const _ as *const _, core::mem::size_of_val(&(*range).min_addr.ip6));
            memcpy((*range).max_addr.ip6.as_mut_ptr() as *mut _, &(*regs).data[(*priv_).sreg_addr_max as usize] as *const _ as *const _, core::mem::size_of_val(&(*range).max_addr.ip6));
        }
        _ => {}
    }
}

unsafe fn nft_nat_setup_proto(range: *mut nf_nat_range2, regs: *const nft_regs, priv_: *const nft_nat) {
    (*range).min_proto.all = nft_reg_load16(&(*regs).data[(*priv_).sreg_proto_min as usize]) as __be16;
    (*range).max_proto.all = nft_reg_load16(&(*regs).data[(*priv_).sreg_proto_max as usize]) as __be16;
}

unsafe fn nft_nat_setup_netmap(range: *mut nf_nat_range2, pkt: *const nft_pktinfo, priv_: *const nft_nat) {
    let skb = (*pkt).skb;
    let mut new_addr: nf_inet_addr = core::mem::zeroed();
    let mut netmask: __be32;
    let mut len = 0usize;
    match (*priv_).r#type {
        NFT_NAT_SNAT => {
            if nft_pf(pkt) == NFPROTO_IPV4 { new_addr.ip = (*ip_hdr(skb)).saddr; len = core::mem::size_of::<in_addr>(); }
            else { new_addr.in6 = (*ipv6_hdr(skb)).saddr; len = core::mem::size_of::<in6_addr>(); }
        }
        NFT_NAT_DNAT => {
            if nft_pf(pkt) == NFPROTO_IPV4 { new_addr.ip = (*ip_hdr(skb)).daddr; len = core::mem::size_of::<in_addr>(); }
            else { new_addr.in6 = (*ipv6_hdr(skb)).daddr; len = core::mem::size_of::<in6_addr>(); }
        }
        _ => {}
    }
    for i in 0..(len / core::mem::size_of::<__be32>()) {
        netmask = !((*range).min_addr.ip6[i] ^ (*range).max_addr.ip6[i]);
        new_addr.ip6[i] &= !netmask;
        new_addr.ip6[i] |= (*range).min_addr.ip6[i] & netmask;
    }
    (*range).min_addr = new_addr;
    (*range).max_addr = new_addr;
}

unsafe fn nft_nat_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr) as *const nft_nat;
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let ct = nf_ct_get((*pkt).skb, &mut ctinfo);
    let mut range: nf_nat_range2 = core::mem::zeroed();
    if (*priv_).sreg_addr_min != 0 {
        nft_nat_setup_addr(&mut range, regs, priv_);
        if ((*priv_).flags as u32 & NF_NAT_RANGE_NETMAP) != 0 { nft_nat_setup_netmap(&mut range, pkt, priv_); }
    }
    if (*priv_).sreg_proto_min != 0 { nft_nat_setup_proto(&mut range, regs, priv_); }
    range.flags = (*priv_).flags;
    (*regs).verdict.code = nf_nat_setup_info(ct, &mut range, (*priv_).r#type);
}

static mut nft_nat_policy: [nla_policy; NFTA_NAT_MAX as usize + 1] = [nla_policy { r#type: NLA_U32 }; NFTA_NAT_MAX as usize + 1];

unsafe fn nft_nat_validate(ctx: *const nft_ctx, expr: *const nft_expr) -> i32 {
    let priv_ = nft_expr_priv(expr) as *const nft_nat;
    if (*ctx).family != NFPROTO_IPV4 && (*ctx).family != NFPROTO_IPV6 && (*ctx).family != NFPROTO_INET { return -EOPNOTSUPP; }
    let mut err = nft_chain_validate_dependency((*ctx).chain, NFT_CHAIN_T_NAT);
    if err < 0 { return err; }
    match (*priv_).r#type {
        NFT_NAT_SNAT => { err = nft_chain_validate_hooks((*ctx).chain, (1 << NF_INET_POST_ROUTING) | (1 << NF_INET_LOCAL_IN)); }
        NFT_NAT_DNAT => { err = nft_chain_validate_hooks((*ctx).chain, (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_LOCAL_OUT)); }
        _ => {}
    }
    err
}

unsafe fn nft_nat_init(ctx: *const nft_ctx, expr: *const nft_expr, tb: *const *const nlattr) -> i32 {
    let priv_ = nft_expr_priv(expr) as *mut nft_nat;
    let mut alen: u32;
    let mut plen: u32;
    let mut family: u32;
    if (*tb.add(NFTA_NAT_TYPE as usize)).is_null() || ((*tb.add(NFTA_NAT_REG_ADDR_MIN as usize)).is_null() && (*tb.add(NFTA_NAT_REG_PROTO_MIN as usize)).is_null()) { return -EINVAL; }
    match ntohl(nla_get_be32(*tb.add(NFTA_NAT_TYPE as usize))) {
        NFT_NAT_SNAT => (*priv_).r#type = NF_NAT_MANIP_SRC,
        NFT_NAT_DNAT => (*priv_).r#type = NF_NAT_MANIP_DST,
        _ => return -EOPNOTSUPP,
    }
    if (*tb.add(NFTA_NAT_FAMILY as usize)).is_null() { return -EINVAL; }
    family = ntohl(nla_get_be32(*tb.add(NFTA_NAT_FAMILY as usize)));
    if (*ctx).family != NFPROTO_INET && (*ctx).family != family { return -EOPNOTSUPP; }
    match family { NFPROTO_IPV4 => alen = core::mem::size_of::<__be32>() as u32, NFPROTO_IPV6 => alen = (4 * core::mem::size_of::<__be32>()) as u32, _ => { if !(*tb.add(NFTA_NAT_REG_ADDR_MIN as usize)).is_null() { return -EAFNOSUPPORT; } alen = 0; } }
    (*priv_).family = family as u8;
    if !(*tb.add(NFTA_NAT_REG_ADDR_MIN as usize)).is_null() {
        let mut err = nft_parse_register_load(ctx, *tb.add(NFTA_NAT_REG_ADDR_MIN as usize), &mut (*priv_).sreg_addr_min, alen); if err < 0 { return err; }
        if !(*tb.add(NFTA_NAT_REG_ADDR_MAX as usize)).is_null() { err = nft_parse_register_load(ctx, *tb.add(NFTA_NAT_REG_ADDR_MAX as usize), &mut (*priv_).sreg_addr_max, alen); if err < 0 { return err; } } else { (*priv_).sreg_addr_max = (*priv_).sreg_addr_min; }
        (*priv_).flags |= NF_NAT_RANGE_MAP_IPS as u16;
    }
    plen = core::mem::size_of::<__be16>() as u32;
    if !(*tb.add(NFTA_NAT_REG_PROTO_MIN as usize)).is_null() {
        let mut err = nft_parse_register_load(ctx, *tb.add(NFTA_NAT_REG_PROTO_MIN as usize), &mut (*priv_).sreg_proto_min, plen); if err < 0 { return err; }
        if !(*tb.add(NFTA_NAT_REG_PROTO_MAX as usize)).is_null() { err = nft_parse_register_load(ctx, *tb.add(NFTA_NAT_REG_PROTO_MAX as usize), &mut (*priv_).sreg_proto_max, plen); if err < 0 { return err; } } else { (*priv_).sreg_proto_max = (*priv_).sreg_proto_min; }
        (*priv_).flags |= NF_NAT_RANGE_PROTO_SPECIFIED as u16;
    }
    if !(*tb.add(NFTA_NAT_FLAGS as usize)).is_null() { (*priv_).flags |= ntohl(nla_get_be32(*tb.add(NFTA_NAT_FLAGS as usize))) as u16; }
    nf_ct_netns_get((*ctx).net, family)
}

unsafe fn nft_nat_dump(skb: *mut sk_buff, expr: *const nft_expr, _reset: bool) -> i32 {
    let priv_ = nft_expr_priv(expr) as *const nft_nat;
    let kind = match (*priv_).r#type { NF_NAT_MANIP_SRC => NFT_NAT_SNAT, NF_NAT_MANIP_DST => NFT_NAT_DNAT, _ => 0 };
    if nla_put_be32(skb, NFTA_NAT_TYPE, htonl(kind)) != 0 || nla_put_be32(skb, NFTA_NAT_FAMILY, htonl((*priv_).family as u32)) != 0 { return -1; }
    if (*priv_).sreg_addr_min != 0 && (nft_dump_register(skb, NFTA_NAT_REG_ADDR_MIN, (*priv_).sreg_addr_min) != 0 || nft_dump_register(skb, NFTA_NAT_REG_ADDR_MAX, (*priv_).sreg_addr_max) != 0) { return -1; }
    if (*priv_).sreg_proto_min != 0 && (nft_dump_register(skb, NFTA_NAT_REG_PROTO_MIN, (*priv_).sreg_proto_min) != 0 || nft_dump_register(skb, NFTA_NAT_REG_PROTO_MAX, (*priv_).sreg_proto_max) != 0) { return -1; }
    if (*priv_).flags != 0 && nla_put_be32(skb, NFTA_NAT_FLAGS, htonl((*priv_).flags as u32)) != 0 { return -1; }
    0
}

unsafe fn nft_nat_destroy(ctx: *const nft_ctx, expr: *const nft_expr) {
    let priv_ = nft_expr_priv(expr) as *const nft_nat;
    nf_ct_netns_put((*ctx).net, (*priv_).family as u32);
}

static mut nft_nat_type: nft_expr_type = nft_expr_type::zeroed();

#[cfg(CONFIG_NF_TABLES_INET)]
unsafe fn nft_nat_inet_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = nft_expr_priv(expr) as *const nft_nat;
    if (*priv_).family as u32 == nft_pf(pkt) || (*priv_).family as u32 == NFPROTO_INET { nft_nat_eval(expr, regs, pkt); }
}

#[cfg(CONFIG_NF_TABLES_INET)]
static mut nft_inet_nat_type: nft_expr_type = nft_expr_type::zeroed();

#[cfg(CONFIG_NF_TABLES_INET)]
unsafe fn nft_nat_inet_module_init() -> i32 { nft_register_expr(&mut nft_inet_nat_type) }
#[cfg(not(CONFIG_NF_TABLES_INET))]
unsafe fn nft_nat_inet_module_init() -> i32 { 0 }
#[cfg(CONFIG_NF_TABLES_INET)]
unsafe fn nft_nat_inet_module_exit() { nft_unregister_expr(&mut nft_inet_nat_type); }
#[cfg(not(CONFIG_NF_TABLES_INET))]
unsafe fn nft_nat_inet_module_exit() {}

#[no_mangle]
pub unsafe extern "C" fn nft_nat_module_init() -> i32 {
    let ret = nft_nat_inet_module_init();
    if ret != 0 { return ret; }
    let ret = nft_register_expr(&mut nft_nat_type);
    if ret != 0 { nft_nat_inet_module_exit(); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn nft_nat_module_exit() {
    nft_nat_inet_module_exit();
    nft_unregister_expr(&mut nft_nat_type);
}

// module_init(nft_nat_module_init); module_exit(nft_nat_module_exit);
// MODULE_LICENSE("GPL"); MODULE_AUTHOR("Tomasz Bursztyka <tomasz.bursztyka@linux.intel.com>");
// MODULE_ALIAS_NFT_EXPR("nat"); MODULE_DESCRIPTION("Network Address Translation support");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
