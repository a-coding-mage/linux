// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied by other translation units are intentionally unresolved here.

#[cfg(CONFIG_NF_TABLES_IPV4)]
unsafe fn nft_do_chain_ipv4(priv_: *mut core::ffi::c_void, skb: *mut sk_buff, state: *const nf_hook_state) -> u32 {
    let mut pkt: nft_pktinfo = core::mem::zeroed();
    nft_set_pktinfo(&mut pkt, skb, state);
    nft_set_pktinfo_ipv4(&mut pkt);
    nft_do_chain(&mut pkt, priv_)
}

#[cfg(CONFIG_NF_TABLES_IPV4)]
static nft_chain_filter_ipv4: nft_chain_type = nft_chain_type {
    name: b"filter\0".as_ptr() as *const i8, type_: NFT_CHAIN_T_DEFAULT, family: NFPROTO_IPV4,
    hook_mask: (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_FORWARD) |
        (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_POST_ROUTING),
    hooks: [
        [NF_INET_LOCAL_IN] = nft_do_chain_ipv4, [NF_INET_LOCAL_OUT] = nft_do_chain_ipv4,
        [NF_INET_FORWARD] = nft_do_chain_ipv4, [NF_INET_PRE_ROUTING] = nft_do_chain_ipv4,
        [NF_INET_POST_ROUTING] = nft_do_chain_ipv4,
    ],
};
#[cfg(CONFIG_NF_TABLES_IPV4)] unsafe fn nft_chain_filter_ipv4_init() { nft_register_chain_type(&nft_chain_filter_ipv4); }
#[cfg(CONFIG_NF_TABLES_IPV4)] unsafe fn nft_chain_filter_ipv4_fini() { nft_unregister_chain_type(&nft_chain_filter_ipv4); }
#[cfg(not(CONFIG_NF_TABLES_IPV4))] unsafe fn nft_chain_filter_ipv4_init() {}
#[cfg(not(CONFIG_NF_TABLES_IPV4))] unsafe fn nft_chain_filter_ipv4_fini() {}

#[cfg(CONFIG_NF_TABLES_ARP)]
unsafe fn nft_do_chain_arp(priv_: *mut core::ffi::c_void, skb: *mut sk_buff, state: *const nf_hook_state) -> u32 {
    let mut pkt: nft_pktinfo = core::mem::zeroed();
    nft_set_pktinfo(&mut pkt, skb, state); nft_set_pktinfo_unspec(&mut pkt); nft_do_chain(&mut pkt, priv_)
}
#[cfg(CONFIG_NF_TABLES_ARP)]
static nft_chain_filter_arp: nft_chain_type = nft_chain_type {
    name: b"filter\0".as_ptr() as *const i8, type_: NFT_CHAIN_T_DEFAULT, family: NFPROTO_ARP,
    owner: THIS_MODULE, hook_mask: (1 << NF_ARP_IN) | (1 << NF_ARP_OUT),
    hooks: [[NF_ARP_IN] = nft_do_chain_arp, [NF_ARP_OUT] = nft_do_chain_arp],
};
#[cfg(CONFIG_NF_TABLES_ARP)] unsafe fn nft_chain_filter_arp_init() { nft_register_chain_type(&nft_chain_filter_arp); }
#[cfg(CONFIG_NF_TABLES_ARP)] unsafe fn nft_chain_filter_arp_fini() { nft_unregister_chain_type(&nft_chain_filter_arp); }
#[cfg(not(CONFIG_NF_TABLES_ARP))] unsafe fn nft_chain_filter_arp_init() {}
#[cfg(not(CONFIG_NF_TABLES_ARP))] unsafe fn nft_chain_filter_arp_fini() {}

#[cfg(CONFIG_NF_TABLES_IPV6)]
unsafe fn nft_do_chain_ipv6(priv_: *mut core::ffi::c_void, skb: *mut sk_buff, state: *const nf_hook_state) -> u32 {
    let mut pkt: nft_pktinfo = core::mem::zeroed(); nft_set_pktinfo(&mut pkt, skb, state);
    nft_set_pktinfo_ipv6(&mut pkt); nft_do_chain(&mut pkt, priv_)
}
#[cfg(CONFIG_NF_TABLES_IPV6)]
static nft_chain_filter_ipv6: nft_chain_type = nft_chain_type {
    name: b"filter\0".as_ptr() as *const i8, type_: NFT_CHAIN_T_DEFAULT, family: NFPROTO_IPV6,
    hook_mask: (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_FORWARD) |
        (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_POST_ROUTING),
    hooks: [[NF_INET_LOCAL_IN] = nft_do_chain_ipv6, [NF_INET_LOCAL_OUT] = nft_do_chain_ipv6,
        [NF_INET_FORWARD] = nft_do_chain_ipv6, [NF_INET_PRE_ROUTING] = nft_do_chain_ipv6,
        [NF_INET_POST_ROUTING] = nft_do_chain_ipv6],
};
#[cfg(CONFIG_NF_TABLES_IPV6)] unsafe fn nft_chain_filter_ipv6_init() { nft_register_chain_type(&nft_chain_filter_ipv6); }
#[cfg(CONFIG_NF_TABLES_IPV6)] unsafe fn nft_chain_filter_ipv6_fini() { nft_unregister_chain_type(&nft_chain_filter_ipv6); }
#[cfg(not(CONFIG_NF_TABLES_IPV6))] unsafe fn nft_chain_filter_ipv6_init() {}
#[cfg(not(CONFIG_NF_TABLES_IPV6))] unsafe fn nft_chain_filter_ipv6_fini() {}

#[cfg(CONFIG_NF_TABLES_INET)]
unsafe fn nft_do_chain_inet(priv_: *mut core::ffi::c_void, skb: *mut sk_buff, state: *const nf_hook_state) -> u32 {
    let mut pkt: nft_pktinfo = core::mem::zeroed(); nft_set_pktinfo(&mut pkt, skb, state);
    match (*state).pf { NFPROTO_IPV4 => nft_set_pktinfo_ipv4(&mut pkt), NFPROTO_IPV6 => nft_set_pktinfo_ipv6(&mut pkt), _ => {} }
    nft_do_chain(&mut pkt, priv_)
}
#[cfg(CONFIG_NF_TABLES_INET)]
unsafe fn nft_do_chain_inet_ingress(priv_: *mut core::ffi::c_void, skb: *mut sk_buff, state: *const nf_hook_state) -> u32 {
    let mut ingress_state = *state; let mut pkt: nft_pktinfo = core::mem::zeroed();
    match (*skb).protocol {
        p if p == htons(ETH_P_IP) => { ingress_state.pf = NFPROTO_IPV4; ingress_state.hook = NF_INET_INGRESS; nft_set_pktinfo(&mut pkt, skb, &ingress_state); if nft_set_pktinfo_ipv4_ingress(&mut pkt) < 0 { return NF_DROP; } }
        p if p == htons(ETH_P_IPV6) => { ingress_state.pf = NFPROTO_IPV6; ingress_state.hook = NF_INET_INGRESS; nft_set_pktinfo(&mut pkt, skb, &ingress_state); if nft_set_pktinfo_ipv6_ingress(&mut pkt) < 0 { return NF_DROP; } }
        _ => return NF_ACCEPT,
    } nft_do_chain(&mut pkt, priv_)
}
#[cfg(CONFIG_NF_TABLES_INET)]
static nft_chain_filter_inet: nft_chain_type = nft_chain_type { name: b"filter\0".as_ptr() as *const i8, type_: NFT_CHAIN_T_DEFAULT, family: NFPROTO_INET,
    hook_mask: (1 << NF_INET_INGRESS) | (1 << NF_INET_LOCAL_IN) | (1 << NF_INET_LOCAL_OUT) | (1 << NF_INET_FORWARD) | (1 << NF_INET_PRE_ROUTING) | (1 << NF_INET_POST_ROUTING),
    hooks: [[NF_INET_INGRESS] = nft_do_chain_inet_ingress, [NF_INET_LOCAL_IN] = nft_do_chain_inet, [NF_INET_LOCAL_OUT] = nft_do_chain_inet, [NF_INET_FORWARD] = nft_do_chain_inet, [NF_INET_PRE_ROUTING] = nft_do_chain_inet, [NF_INET_POST_ROUTING] = nft_do_chain_inet] };
#[cfg(CONFIG_NF_TABLES_INET)] unsafe fn nft_chain_filter_inet_init() { nft_register_chain_type(&nft_chain_filter_inet); }
#[cfg(CONFIG_NF_TABLES_INET)] unsafe fn nft_chain_filter_inet_fini() { nft_unregister_chain_type(&nft_chain_filter_inet); }
#[cfg(not(CONFIG_NF_TABLES_INET))] unsafe fn nft_chain_filter_inet_init() {}
#[cfg(not(CONFIG_NF_TABLES_INET))] unsafe fn nft_chain_filter_inet_fini() {}

// CONFIG_NF_TABLES_BRIDGE and CONFIG_NF_TABLES_NETDEV branches retain the same
// externally supplied nftables structures and notifier operations as the C source.
#[cfg(IS_ENABLED_CONFIG_NF_TABLES_BRIDGE)]
unsafe fn nft_do_chain_bridge(priv_: *mut core::ffi::c_void, skb: *mut sk_buff, state: *const nf_hook_state) -> u32 {
    let mut pkt: nft_pktinfo = core::mem::zeroed(); nft_set_pktinfo(&mut pkt, skb, state);
    match (*eth_hdr(skb)).h_proto { p if p == htons(ETH_P_IP) => nft_set_pktinfo_ipv4_validate(&mut pkt), p if p == htons(ETH_P_IPV6) => nft_set_pktinfo_ipv6_validate(&mut pkt), _ => nft_set_pktinfo_unspec(&mut pkt) }
    nft_do_chain(&mut pkt, priv_)
}
#[cfg(not(IS_ENABLED_CONFIG_NF_TABLES_BRIDGE))] unsafe fn nft_chain_filter_bridge_init() {}
#[cfg(not(IS_ENABLED_CONFIG_NF_TABLES_BRIDGE))] unsafe fn nft_chain_filter_bridge_fini() {}
#[cfg(IS_ENABLED_CONFIG_NF_TABLES_BRIDGE)] unsafe fn nft_chain_filter_bridge_init() { nft_register_chain_type(&nft_chain_filter_bridge); }
#[cfg(IS_ENABLED_CONFIG_NF_TABLES_BRIDGE)] unsafe fn nft_chain_filter_bridge_fini() { nft_unregister_chain_type(&nft_chain_filter_bridge); }

#[cfg(CONFIG_NF_TABLES_NETDEV)] unsafe fn nft_chain_filter_netdev_init() -> i32 { nft_register_chain_type(&nft_chain_filter_netdev); let err = register_netdevice_notifier(&nf_tables_netdev_notifier); if err != 0 { nft_unregister_chain_type(&nft_chain_filter_netdev); return err; } 0 }
#[cfg(not(CONFIG_NF_TABLES_NETDEV))] unsafe fn nft_chain_filter_netdev_init() -> i32 { 0 }
#[cfg(CONFIG_NF_TABLES_NETDEV)] unsafe fn nft_chain_filter_netdev_fini() { nft_unregister_chain_type(&nft_chain_filter_netdev); unregister_netdevice_notifier(&nf_tables_netdev_notifier); }
#[cfg(not(CONFIG_NF_TABLES_NETDEV))] unsafe fn nft_chain_filter_netdev_fini() {}

unsafe fn nft_chain_filter_init() -> i32 {
    let err = nft_chain_filter_netdev_init(); if err < 0 { return err; }
    nft_chain_filter_ipv4_init(); nft_chain_filter_ipv6_init(); nft_chain_filter_arp_init(); nft_chain_filter_inet_init(); nft_chain_filter_bridge_init(); 0
}
unsafe fn nft_chain_filter_fini() {
    nft_chain_filter_bridge_fini(); nft_chain_filter_inet_fini(); nft_chain_filter_arp_fini(); nft_chain_filter_ipv6_fini(); nft_chain_filter_ipv4_fini(); nft_chain_filter_netdev_fini();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
