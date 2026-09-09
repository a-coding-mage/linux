// SPDX-License-Identifier: GPL-2.0-only
//
// C dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

extern "C" {
    fn htons(value: u16) -> u16;
    fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> bool;
    fn skb_mac_offset(skb: *const sk_buff) -> usize;
    fn skb_mac_header(skb: *mut sk_buff) -> *mut c_void;
    fn nf_flow_pppoe_proto(skb: *mut sk_buff, proto: *mut u16) -> bool;
    fn nf_flow_offload_ip_hook(
        priv_: *mut c_void,
        skb: *mut sk_buff,
        state: *const nf_hook_state,
    ) -> u32;
    fn nf_flow_offload_ipv6_hook(
        priv_: *mut c_void,
        skb: *mut sk_buff,
        state: *const nf_hook_state,
    ) -> u32;
    fn nf_flow_rule_route_ipv4(
        net: *mut net,
        flow: *mut flow_offload,
        dir: flow_offload_tuple_dir,
        flow_rule: *mut nf_flow_rule,
    ) -> i32;
    fn nf_flow_rule_route_ipv6(
        net: *mut net,
        flow: *mut flow_offload,
        dir: flow_offload_tuple_dir,
        flow_rule: *mut nf_flow_rule,
    ) -> i32;
    fn nf_flow_table_init(flowtable: *mut nf_flowtable_type) -> i32;
    fn nf_flow_table_offload_setup(flowtable: *mut nf_flowtable_type, enable: bool) -> i32;
    fn nf_flow_table_free(flowtable: *mut nf_flowtable_type);
    fn nft_register_flowtable_type(flowtable: *mut nf_flowtable_type) -> i32;
    fn nft_unregister_flowtable_type(flowtable: *mut nf_flowtable_type);
}

#[repr(C)]
pub struct sk_buff {
    pub protocol: u16,
}

#[repr(C)]
pub struct nf_hook_state;
#[repr(C)]
pub struct net;
#[repr(C)]
pub struct nf_flow_rule;

#[repr(C)]
pub struct flow_offload {
    pub tuplehash: [flow_offload_tuplehash; 2],
}

#[repr(C)]
pub struct flow_offload_tuplehash {
    pub tuple: flow_offload_tuple,
}

#[repr(C)]
pub struct flow_offload_tuple {
    pub l3proto: u8,
}

#[repr(C)]
pub struct vlan_ethhdr {
    pub h_vlan_encapsulated_proto: u16,
}

#[repr(C)]
pub struct nf_flowtable_type {
    pub family: u8,
    pub init: Option<unsafe extern "C" fn(*mut nf_flowtable_type) -> i32>,
    pub setup: Option<unsafe extern "C" fn(*mut nf_flowtable_type, bool) -> i32>,
    pub action: Option<unsafe extern "C" fn(
        *mut net,
        *mut flow_offload,
        flow_offload_tuple_dir,
        *mut nf_flow_rule,
    ) -> i32>,
    pub free: Option<unsafe extern "C" fn(*mut nf_flowtable_type)>,
    pub hook: Option<unsafe extern "C" fn(*mut c_void, *mut sk_buff, *const nf_hook_state) -> u32>,
    pub owner: *mut c_void,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum flow_offload_tuple_dir {
    Original = 0,
    Reply = 1,
}

const NF_ACCEPT: u32 = 1;
const NFPROTO_IPV4: u8 = 2;
const NFPROTO_IPV6: u8 = 10;
const NFPROTO_INET: u8 = 1;
const ETH_P_8021Q: u16 = 0x8100;
const ETH_P_PPP_SES: u16 = 0x8864;
const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86dd;

unsafe extern "C" fn nf_flow_offload_inet_hook(
    priv_: *mut c_void,
    skb: *mut sk_buff,
    state: *const nf_hook_state,
) -> u32 {
    let mut proto: u16;

    match (*skb).protocol {
        p if p == htons(ETH_P_8021Q) => {
            if !pskb_may_pull(
                skb,
                skb_mac_offset(skb) + core::mem::size_of::<vlan_ethhdr>(),
            ) {
                return NF_ACCEPT;
            }

            let veth = skb_mac_header(skb) as *mut vlan_ethhdr;
            proto = (*veth).h_vlan_encapsulated_proto;
        }
        p if p == htons(ETH_P_PPP_SES) => {
            if !nf_flow_pppoe_proto(skb, &mut proto) {
                return NF_ACCEPT;
            }
        }
        _ => {
            proto = (*skb).protocol;
        }
    }

    match proto {
        p if p == htons(ETH_P_IP) => nf_flow_offload_ip_hook(priv_, skb, state),
        p if p == htons(ETH_P_IPV6) => nf_flow_offload_ipv6_hook(priv_, skb, state),
        _ => NF_ACCEPT,
    }
}

unsafe extern "C" fn nf_flow_rule_route_inet(
    net: *mut net,
    flow: *mut flow_offload,
    dir: flow_offload_tuple_dir,
    flow_rule: *mut nf_flow_rule,
) -> i32 {
    let flow_tuple = &(*flow).tuplehash[dir as usize].tuple;

    match flow_tuple.l3proto {
        NFPROTO_IPV4 => nf_flow_rule_route_ipv4(net, flow, dir, flow_rule),
        NFPROTO_IPV6 => nf_flow_rule_route_ipv6(net, flow, dir, flow_rule),
        _ => -1,
    }
}

static mut flowtable_inet: nf_flowtable_type = nf_flowtable_type {
    family: NFPROTO_INET,
    init: Some(nf_flow_table_init),
    setup: Some(nf_flow_table_offload_setup),
    action: Some(nf_flow_rule_route_inet),
    free: Some(nf_flow_table_free),
    hook: Some(nf_flow_offload_inet_hook),
    owner: core::ptr::null_mut(),
};

static mut flowtable_ipv4: nf_flowtable_type = nf_flowtable_type {
    family: NFPROTO_IPV4,
    init: Some(nf_flow_table_init),
    setup: Some(nf_flow_table_offload_setup),
    action: Some(nf_flow_rule_route_ipv4),
    free: Some(nf_flow_table_free),
    hook: Some(nf_flow_offload_ip_hook),
    owner: core::ptr::null_mut(),
};

static mut flowtable_ipv6: nf_flowtable_type = nf_flowtable_type {
    family: NFPROTO_IPV6,
    init: Some(nf_flow_table_init),
    setup: Some(nf_flow_table_offload_setup),
    action: Some(nf_flow_rule_route_ipv6),
    free: Some(nf_flow_table_free),
    hook: Some(nf_flow_offload_ipv6_hook),
    owner: core::ptr::null_mut(),
};

unsafe extern "C" fn nf_flow_inet_module_init() -> i32 {
    nft_register_flowtable_type(&mut flowtable_ipv4);
    nft_register_flowtable_type(&mut flowtable_ipv6);
    nft_register_flowtable_type(&mut flowtable_inet);
    0
}

unsafe extern "C" fn nf_flow_inet_module_exit() {
    nft_unregister_flowtable_type(&mut flowtable_inet);
    nft_unregister_flowtable_type(&mut flowtable_ipv6);
    nft_unregister_flowtable_type(&mut flowtable_ipv4);
}

// module_init(nf_flow_inet_module_init);
// module_exit(nf_flow_inet_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Pablo Neira Ayuso <pablo@netfilter.org>");
// MODULE_ALIAS_NF_FLOWTABLE(AF_INET);
// MODULE_ALIAS_NF_FLOWTABLE(AF_INET6);
// MODULE_ALIAS_NF_FLOWTABLE(1); /* NFPROTO_INET */
// MODULE_DESCRIPTION("Netfilter flow table mixed IPv4/IPv6 module");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
