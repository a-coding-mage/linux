// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2020 Laura Garcia Liebana <nevola@gmail.com>
 * Copyright (c) 2020 Jose M. Guisado <guigom@riseup.net>
 */

// Linux kernel dependencies supplied externally.

use core::ffi::c_void;

#[repr(C)]
pub struct sk_buff {
    pub dev: *mut net_device,
    pub protocol: u16,
    pub len: u32,
}

#[repr(C)]
pub struct net;
#[repr(C)]
pub struct net_device;
#[repr(C)]
pub struct nft_expr;
#[repr(C)]
pub struct nft_pktinfo {
    pub skb: *mut sk_buff,
}
#[repr(C)]
pub struct nft_ctx {
    pub chain: *mut c_void,
}
#[repr(C)]
pub struct nft_regs {
    pub verdict: nft_verdict,
}
#[repr(C)]
pub struct nft_verdict {
    pub code: i32,
}
#[repr(C)]
pub struct ethhdr {
    pub h_dest: [u8; 6],
    pub h_source: [u8; 6],
    pub h_proto: u16,
}
#[repr(C)]
pub struct nft_reject {
    pub type_: u32,
    pub icmp_code: u8,
}
#[repr(C)]
pub struct nft_expr_type {
    pub family: u8,
    pub name: *const u8,
    pub ops: *const nft_expr_ops,
    pub policy: *const c_void,
    pub maxattr: u32,
    pub owner: *mut c_void,
}
#[repr(C)]
pub struct nft_expr_ops {
    pub type_: *mut nft_expr_type,
    pub size: usize,
    pub eval: Option<unsafe extern "C" fn(*const nft_expr, *mut nft_regs, *const nft_pktinfo)>,
    pub init: Option<unsafe extern "C" fn() -> i32>,
    pub dump: Option<unsafe extern "C" fn()>,
    pub validate: Option<unsafe extern "C" fn(*const nft_ctx, *const nft_expr) -> i32>,
}

extern "C" {
    fn dev_hard_header(skb: *mut sk_buff, dev: *mut net_device, protocol: u16,
                       source: *const u8, dest: *const u8, len: u32);
    fn dev_queue_xmit(skb: *mut sk_buff) -> i32;
    fn eth_hdr(skb: *mut sk_buff) -> *mut ethhdr;
    fn nf_reject_skb_v4_tcp_reset(net: *mut net, oldskb: *mut sk_buff,
                                  dev: *const net_device, hook: i32) -> *mut sk_buff;
    fn nf_reject_skb_v4_unreach(net: *mut net, oldskb: *mut sk_buff,
                                dev: *const net_device, hook: i32, code: u8) -> *mut sk_buff;
    fn nf_reject_skb_v6_tcp_reset(net: *mut net, oldskb: *mut sk_buff,
                                  dev: *const net_device, hook: i32) -> *mut sk_buff;
    fn nf_reject_skb_v6_unreach(net: *mut net, oldskb: *mut sk_buff,
                                dev: *const net_device, hook: i32, code: u8) -> *mut sk_buff;
    fn nft_expr_priv(expr: *const nft_expr) -> *mut nft_reject;
    fn is_broadcast_ether_addr(addr: *const u8) -> bool;
    fn is_multicast_ether_addr(addr: *const u8) -> bool;
    fn nft_net(pkt: *const nft_pktinfo) -> *mut net;
    fn nft_in(pkt: *const nft_pktinfo) -> *const net_device;
    fn nft_hook(pkt: *const nft_pktinfo) -> i32;
    fn nft_reject_icmp_code(code: u8) -> u8;
    fn nft_reject_icmpv6_code(code: u8) -> u8;
    fn nft_chain_validate_hooks(chain: *mut c_void, hooks: u32) -> i32;
    fn nft_reject_init() -> i32;
    fn nft_reject_dump();
    fn nft_register_expr(ty: *mut nft_expr_type) -> i32;
    fn nft_unregister_expr(ty: *mut nft_expr_type);
}

const ETH_P_IP: u16 = 0x0800;
const ETH_P_IPV6: u16 = 0x86dd;
const NFT_REJECT_ICMP_UNREACH: u32 = 0;
const NFT_REJECT_TCP_RST: u32 = 1;
const NFT_REJECT_ICMPX_UNREACH: u32 = 2;
const NF_DROP: i32 = 0;
const NF_NETDEV_INGRESS: u32 = 0;
const NFPROTO_NETDEV: u8 = 5;
const NFTA_REJECT_MAX: u32 = 0;
static mut nft_reject_policy: *const c_void = core::ptr::null();
static mut THIS_MODULE: *mut c_void = core::ptr::null_mut();

unsafe fn nft_reject_queue_xmit(nskb: *mut sk_buff, oldskb: *mut sk_buff) {
    let old_eth = eth_hdr(oldskb);
    dev_hard_header(nskb, (*nskb).dev, u16::from_be((*oldskb).protocol),
                    (*old_eth).h_source.as_ptr(), (*old_eth).h_dest.as_ptr(), (*nskb).len);
    dev_queue_xmit(nskb);
}

unsafe fn nft_reject_netdev_send_v4_tcp_reset(net_: *mut net, oldskb: *mut sk_buff,
                                               dev: *const net_device, hook: i32) {
    let nskb = nf_reject_skb_v4_tcp_reset(net_, oldskb, dev, hook);
    if !nskb.is_null() { nft_reject_queue_xmit(nskb, oldskb); }
}

unsafe fn nft_reject_netdev_send_v4_unreach(net_: *mut net, oldskb: *mut sk_buff,
                                            dev: *const net_device, hook: i32, code: u8) {
    let nskb = nf_reject_skb_v4_unreach(net_, oldskb, dev, hook, code);
    if !nskb.is_null() { nft_reject_queue_xmit(nskb, oldskb); }
}

unsafe fn nft_reject_netdev_send_v6_tcp_reset(net_: *mut net, oldskb: *mut sk_buff,
                                               dev: *const net_device, hook: i32) {
    let nskb = nf_reject_skb_v6_tcp_reset(net_, oldskb, dev, hook);
    if !nskb.is_null() { nft_reject_queue_xmit(nskb, oldskb); }
}

unsafe fn nft_reject_netdev_send_v6_unreach(net_: *mut net, oldskb: *mut sk_buff,
                                            dev: *const net_device, hook: i32, code: u8) {
    let nskb = nf_reject_skb_v6_unreach(net_, oldskb, dev, hook, code);
    if !nskb.is_null() { nft_reject_queue_xmit(nskb, oldskb); }
}

unsafe extern "C" fn nft_reject_netdev_eval(expr: *const nft_expr, regs: *mut nft_regs,
                                             pkt: *const nft_pktinfo) {
    let eth = eth_hdr((*pkt).skb);
    let priv_ = nft_expr_priv(expr);
    let dest = (*eth).h_dest.as_ptr();
    if is_broadcast_ether_addr(dest) || is_multicast_ether_addr(dest) {
        (*regs).verdict.code = NF_DROP;
        return;
    }
    match u16::from_be((*eth).h_proto) {
        ETH_P_IP => match (*priv_).type_ {
            NFT_REJECT_ICMP_UNREACH => nft_reject_netdev_send_v4_unreach(nft_net(pkt), (*pkt).skb, nft_in(pkt), nft_hook(pkt), (*priv_).icmp_code),
            NFT_REJECT_TCP_RST => nft_reject_netdev_send_v4_tcp_reset(nft_net(pkt), (*pkt).skb, nft_in(pkt), nft_hook(pkt)),
            NFT_REJECT_ICMPX_UNREACH => nft_reject_netdev_send_v4_unreach(nft_net(pkt), (*pkt).skb, nft_in(pkt), nft_hook(pkt), nft_reject_icmp_code((*priv_).icmp_code)),
            _ => (),
        },
        ETH_P_IPV6 => match (*priv_).type_ {
            NFT_REJECT_ICMP_UNREACH => nft_reject_netdev_send_v6_unreach(nft_net(pkt), (*pkt).skb, nft_in(pkt), nft_hook(pkt), (*priv_).icmp_code),
            NFT_REJECT_TCP_RST => nft_reject_netdev_send_v6_tcp_reset(nft_net(pkt), (*pkt).skb, nft_in(pkt), nft_hook(pkt)),
            NFT_REJECT_ICMPX_UNREACH => nft_reject_netdev_send_v6_unreach(nft_net(pkt), (*pkt).skb, nft_in(pkt), nft_hook(pkt), nft_reject_icmpv6_code((*priv_).icmp_code)),
            _ => (),
        },
        _ => (), // No explicit way to reject this protocol, drop it.
    }
    (*regs).verdict.code = NF_DROP;
}

unsafe extern "C" fn nft_reject_netdev_validate(ctx: *const nft_ctx, _expr: *const nft_expr) -> i32 {
    nft_chain_validate_hooks((*ctx).chain, 1 << NF_NETDEV_INGRESS)
}

static mut nft_reject_netdev_type: nft_expr_type = nft_expr_type { family: NFPROTO_NETDEV, name: b"reject\0".as_ptr(), ops: &nft_reject_netdev_ops, policy: core::ptr::null(), maxattr: NFTA_REJECT_MAX, owner: core::ptr::null_mut() };
static nft_reject_netdev_ops: nft_expr_ops = nft_expr_ops { type_: core::ptr::null_mut(), size: core::mem::size_of::<nft_reject>(), eval: Some(nft_reject_netdev_eval), init: Some(nft_reject_init), dump: Some(nft_reject_dump), validate: Some(nft_reject_netdev_validate) };

unsafe extern "C" fn nft_reject_netdev_module_init() -> i32 { nft_register_expr(&raw mut nft_reject_netdev_type) }
unsafe extern "C" fn nft_reject_netdev_module_exit() { nft_unregister_expr(&raw mut nft_reject_netdev_type); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
