// SPDX-License-Identifier: GPL-2.0

// Linux kernel dependencies are supplied by the surrounding translation.

unsafe fn nft_nat_do_chain(
    priv_: *mut core::ffi::c_void,
    skb: *mut sk_buff,
    state: *const nf_hook_state,
) -> u32 {
    let mut pkt: nft_pktinfo = core::mem::zeroed();

    nft_set_pktinfo(&mut pkt, skb, state);

    match (*state).pf {
        // CONFIG_NF_TABLES_IPV4
        NFPROTO_IPV4 => {
            nft_set_pktinfo_ipv4(&mut pkt);
        }
        // CONFIG_NF_TABLES_IPV6
        NFPROTO_IPV6 => {
            nft_set_pktinfo_ipv6(&mut pkt);
        }
        _ => {}
    }

    nft_do_chain(&mut pkt, priv_)
}

// CONFIG_NF_TABLES_IPV4
static nft_chain_nat_ipv4: nft_chain_type = nft_chain_type {
    name: "nat",
    type_: NFT_CHAIN_T_NAT,
    family: NFPROTO_IPV4,
    owner: THIS_MODULE,
    hook_mask: (1 << NF_INET_PRE_ROUTING)
        | (1 << NF_INET_POST_ROUTING)
        | (1 << NF_INET_LOCAL_OUT)
        | (1 << NF_INET_LOCAL_IN),
    hooks: [
        [NF_INET_PRE_ROUTING] = nft_nat_do_chain,
        [NF_INET_POST_ROUTING] = nft_nat_do_chain,
        [NF_INET_LOCAL_OUT] = nft_nat_do_chain,
        [NF_INET_LOCAL_IN] = nft_nat_do_chain,
    ],
    ops_register: nf_nat_ipv4_register_fn,
    ops_unregister: nf_nat_ipv4_unregister_fn,
};

// CONFIG_NF_TABLES_IPV6
static nft_chain_nat_ipv6: nft_chain_type = nft_chain_type {
    name: "nat",
    type_: NFT_CHAIN_T_NAT,
    family: NFPROTO_IPV6,
    owner: THIS_MODULE,
    hook_mask: (1 << NF_INET_PRE_ROUTING)
        | (1 << NF_INET_POST_ROUTING)
        | (1 << NF_INET_LOCAL_OUT)
        | (1 << NF_INET_LOCAL_IN),
    hooks: [
        [NF_INET_PRE_ROUTING] = nft_nat_do_chain,
        [NF_INET_POST_ROUTING] = nft_nat_do_chain,
        [NF_INET_LOCAL_OUT] = nft_nat_do_chain,
        [NF_INET_LOCAL_IN] = nft_nat_do_chain,
    ],
    ops_register: nf_nat_ipv6_register_fn,
    ops_unregister: nf_nat_ipv6_unregister_fn,
};

// CONFIG_NF_TABLES_INET
unsafe fn nft_nat_inet_reg(net: *mut net, ops: *const nf_hook_ops) -> i32 {
    nf_nat_inet_register_fn(net, ops)
}

unsafe fn nft_nat_inet_unreg(net: *mut net, ops: *const nf_hook_ops) {
    nf_nat_inet_unregister_fn(net, ops);
}

// CONFIG_NF_TABLES_INET
static nft_chain_nat_inet: nft_chain_type = nft_chain_type {
    name: "nat",
    type_: NFT_CHAIN_T_NAT,
    family: NFPROTO_INET,
    owner: THIS_MODULE,
    hook_mask: (1 << NF_INET_PRE_ROUTING)
        | (1 << NF_INET_LOCAL_IN)
        | (1 << NF_INET_LOCAL_OUT)
        | (1 << NF_INET_POST_ROUTING),
    hooks: [
        [NF_INET_PRE_ROUTING] = nft_nat_do_chain,
        [NF_INET_LOCAL_IN] = nft_nat_do_chain,
        [NF_INET_LOCAL_OUT] = nft_nat_do_chain,
        [NF_INET_POST_ROUTING] = nft_nat_do_chain,
    ],
    ops_register: nft_nat_inet_reg,
    ops_unregister: nft_nat_inet_unreg,
};

unsafe fn nft_chain_nat_init() -> i32 {
    // CONFIG_NF_TABLES_IPV6
    nft_register_chain_type(&nft_chain_nat_ipv6);
    // CONFIG_NF_TABLES_IPV4
    nft_register_chain_type(&nft_chain_nat_ipv4);
    // CONFIG_NF_TABLES_INET
    nft_register_chain_type(&nft_chain_nat_inet);

    0
}

unsafe fn nft_chain_nat_exit() {
    // CONFIG_NF_TABLES_IPV4
    nft_unregister_chain_type(&nft_chain_nat_ipv4);
    // CONFIG_NF_TABLES_IPV6
    nft_unregister_chain_type(&nft_chain_nat_ipv6);
    // CONFIG_NF_TABLES_INET
    nft_unregister_chain_type(&nft_chain_nat_inet);
}

module_init!(nft_chain_nat_init);
module_exit!(nft_chain_nat_exit);

module_license!("GPL");
module_description!("nftables network address translation support");
// CONFIG_NF_TABLES_IPV4
module_alias_nft_chain!(AF_INET, "nat");
// CONFIG_NF_TABLES_IPV6
module_alias_nft_chain!(AF_INET6, "nat");
// CONFIG_NF_TABLES_INET; NFPROTO_INET
module_alias_nft_chain!(1, "nat");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
