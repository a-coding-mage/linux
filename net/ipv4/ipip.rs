// SPDX-License-Identifier: GPL-2.0-or-later
/* Linux NET3: IP/IP protocol decoder. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::c_void;

// Kernel headers and build-time configuration are supplied by the surrounding translation.
extern "C" {
    static mut log_ecn_error: bool;
    static mut ipip_net_id: u32;
    static mut jiffies: usize;
}

#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct ip_tunnel_net { _private: [u8; 0] }
#[repr(C)] pub struct ip_tunnel { _private: [u8; 0] }
#[repr(C)] pub struct iphdr { pub version: u8, pub ihl: u8, pub tos: u8, pub protocol: u8, pub saddr: u32, pub daddr: u32, pub ttl: u8, pub frag_off: u16 }
#[repr(C)] pub struct tnl_ptk_info { pub proto: u16 }
#[repr(C)] pub struct metadata_dst { pub u: tun_union }
#[repr(C)] pub struct tun_union { pub tun_info: [u8; 0] }
#[repr(C)] pub struct ip_tunnel_parm_kern { pub iph: iphdr, pub link: u32, pub i_key: u32, pub o_key: u32, pub i_flags: [usize; 1], pub o_flags: [usize; 1] }
#[repr(C)] pub struct ip_tunnel_encap { pub r#type: u16, pub sport: u16, pub dport: u16, pub flags: u16 }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct rtnl_newlink_params { pub data: *mut *mut nlattr, pub tb: *mut *mut nlattr, pub link_net: *mut net }
#[repr(C)] pub struct net_device_path_ctx { pub dev: *mut net_device, pub ether_type: u16 }
#[repr(C)] pub struct net_device_path { pub r#type: i32, pub tun: [u8; 64], pub dev: *mut net_device }
#[repr(C)] pub struct rtable { pub dst: dst_entry }
#[repr(C)] pub struct dst_entry { pub dev: *mut net_device }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct xfrm_tunnel { pub handler: Option<unsafe extern "C" fn(*mut sk_buff) -> i32>, pub err_handler: Option<unsafe extern "C" fn(*mut sk_buff, u32) -> i32>, pub priority: i32 }
#[repr(C)] pub struct nla_policy { pub r#type: u16 }
#[repr(C)] pub struct rtnl_link_ops { _private: [u8; 0] }
#[repr(C)] pub struct netdev_tx_t(pub i32);

extern "C" {
    fn dev_net(*mut net_device) -> *mut net;
    fn net_generic(*mut net, u32) -> *mut ip_tunnel_net;
    fn icmp_hdr(*mut sk_buff) -> *mut c_void;
    fn ip_tunnel_lookup(*mut ip_tunnel_net, i32, *mut usize, u32, u32, u32) -> *mut ip_tunnel;
    fn ipv4_update_pmtu(*mut sk_buff, *mut net, u32, u32, u8);
    fn ipv4_redirect(*mut sk_buff, *mut net, u32, u8);
    fn ip_hdr(*mut sk_buff) -> *const iphdr;
    fn xfrm4_policy_check(*mut c_void, i32, *mut sk_buff) -> bool;
    fn iptunnel_pull_header(*mut sk_buff, u32, u16, bool) -> i32;
    fn ip_tun_rx_dst(*mut sk_buff, *mut usize, u32, u32) -> *mut metadata_dst;
    fn ip_tunnel_md_udp_encap(*mut sk_buff, *mut c_void);
    fn skb_reset_mac_header(*mut sk_buff);
    fn ip_tunnel_rcv(*mut ip_tunnel, *mut sk_buff, *const tnl_ptk_info, *mut metadata_dst, bool) -> i32;
    fn kfree_skb(*mut sk_buff);
    fn pskb_inet_may_pull(*mut sk_buff) -> bool;
    fn iptunnel_handle_offloads(*mut sk_buff, u32) -> i32;
    fn skb_set_inner_ipproto(*mut sk_buff, u8);
    fn ip_md_tunnel_xmit(*mut sk_buff, *mut net_device, u8, u32);
    fn ip_tunnel_xmit(*mut sk_buff, *mut net_device, *const iphdr, u8);
    fn ip_tunnel_ctl(*mut net_device, *mut ip_tunnel_parm_kern, i32) -> i32;
    fn ip_route_output(*mut net, u32, u32, u8, u32, i32) -> *mut rtable;
    fn ip_tunnel_uninit(*mut net_device); fn ip_tunnel_siocdevprivate(); fn ip_tunnel_change_mtu(); fn dev_get_tstats64(); fn ip_tunnel_get_iflink();
    fn ip_tunnel_setup(*mut net_device, u32); fn ip_tunnel_init(*mut net_device) -> i32;
    fn nla_get_u8(*mut nlattr) -> u8; fn nla_get_u32(*mut nlattr) -> u32;
    fn ip_tunnel_netlink_parms(*mut *mut nlattr, *mut ip_tunnel_parm_kern);
    fn ip_tunnel_netlink_encap_parms(*mut *mut nlattr, *mut ip_tunnel_encap) -> bool;
    fn ip_tunnel_encap_setup(*mut ip_tunnel, *mut ip_tunnel_encap) -> i32;
    fn ip_tunnel_newlink(*mut net, *mut net_device, *mut *mut nlattr, *mut ip_tunnel_parm_kern, u32) -> i32;
    fn ip_tunnel_changelink(*mut net_device, *mut *mut nlattr, *mut ip_tunnel_parm_kern, u32) -> i32;
    fn ip_tunnel_dellink(); fn ip_tunnel_get_link_net();
    fn nla_total_size(u32) -> usize; fn nla_put_u32(*mut sk_buff, u16, u32) -> i32; fn nla_put_in_addr(*mut sk_buff, u16, u32) -> i32; fn nla_put_u8(*mut sk_buff, u16, u8) -> i32; fn nla_put_u16(*mut sk_buff, u16, u16) -> i32; fn nla_put_be16(*mut sk_buff, u16, u16) -> i32; fn nla_put_flag(*mut sk_buff, u16) -> i32;
    fn register_pernet_device(*mut c_void) -> i32; fn unregister_pernet_device(*mut c_void); fn xfrm4_tunnel_register(*mut xfrm_tunnel, i32) -> i32; fn xfrm4_tunnel_deregister(*mut xfrm_tunnel, i32) -> i32; fn rtnl_link_register(*mut rtnl_link_ops) -> i32; fn rtnl_link_unregister(*mut rtnl_link_ops); fn ip_tunnel_init_net(*mut net, u32, *mut rtnl_link_ops, *const u8) -> i32; fn ip_tunnel_delete_net(*mut net, u32, *mut rtnl_link_ops, *mut list_head);
}

static mut ipip_tpi: tnl_ptk_info = tnl_ptk_info { proto: 0x0008u16.to_be() };
#[cfg(CONFIG_MPLS)] static mut mplsip_tpi: tnl_ptk_info = tnl_ptk_info { proto: 0x8847u16.to_be() };

unsafe fn ipip_err(skb: *mut sk_buff, info: u32) -> i32 {
    let net = dev_net(core::ptr::null_mut()); let itn = net_generic(net, ipip_net_id); let iph = ip_hdr(skb); let mut flags = 0usize;
    let type_ = *(icmp_hdr(skb) as *const u8); let code = *((icmp_hdr(skb) as *const u8).add(1));
    flags |= 1usize; let t = ip_tunnel_lookup(itn, 0, &mut flags, (*iph).daddr, (*iph).saddr, 0); let mut err = 0;
    if t.is_null() { return -2; }
    if type_ == 3 && code == 5 { return 0; }
    if type_ == 11 && code != 0 { return 0; }
    if type_ != 3 && type_ != 11 && type_ != 5 { return 0; }
    if type_ == 3 && code == 4 { ipv4_update_pmtu(skb, net, info, 0, (*iph).protocol); return 0; }
    if type_ == 5 { ipv4_redirect(skb, net, 0, (*iph).protocol); return 0; }
    err
}

unsafe fn ipip_tunnel_rcv(skb: *mut sk_buff, ipproto: u8) -> i32 {
    let net = dev_net(core::ptr::null_mut()); let itn = net_generic(net, ipip_net_id); let mut flags = 1usize; let iph = ip_hdr(skb);
    let tunnel = ip_tunnel_lookup(itn, 0, &mut flags, (*iph).saddr, (*iph).daddr, 0);
    if tunnel.is_null() { return -1; }
    if iptunnel_pull_header(skb, 0, ipip_tpi.proto, false) != 0 { kfree_skb(skb); return 0; }
    skb_reset_mac_header(skb); ip_tunnel_rcv(tunnel, skb, &ipip_tpi, core::ptr::null_mut(), log_ecn_error)
}
unsafe fn ipip_rcv(skb: *mut sk_buff) -> i32 { ipip_tunnel_rcv(skb, 4) }
#[cfg(CONFIG_MPLS)] unsafe fn mplsip_rcv(skb: *mut sk_buff) -> i32 { ipip_tunnel_rcv(skb, 137) }

unsafe fn ipip_tunnel_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    if !pskb_inet_may_pull(skb) { kfree_skb(skb); return netdev_tx_t(0); }
    if iptunnel_handle_offloads(skb, 0) != 0 { kfree_skb(skb); return netdev_tx_t(0); }
    skb_set_inner_ipproto(skb, 4); ip_tunnel_xmit(skb, dev, core::ptr::null(), 4); netdev_tx_t(0)
}
unsafe fn ipip_tunnel_ioctl_verify_protocol(ipproto: u8) -> bool { ipproto == 0 || ipproto == 4 || ipproto == 137 }

unsafe fn ipip_tunnel_ctl(dev: *mut net_device, p: *mut ip_tunnel_parm_kern, cmd: i32) -> i32 {
    if cmd == 0x89f0 || cmd == 0x89f1 { if (*p).iph.version != 4 || !ipip_tunnel_ioctl_verify_protocol((*p).iph.protocol) || (*p).iph.ihl != 5 { return -22; } }
    (*p).i_key = 0; (*p).o_key = 0; (*p).i_flags = [0; 1]; (*p).o_flags = [0; 1]; ip_tunnel_ctl(dev, p, cmd)
}

unsafe fn ipip_tunnel_init(dev: *mut net_device) -> i32 { ip_tunnel_init(dev) }
unsafe fn ipip_init_net(net: *mut net) -> i32 { ip_tunnel_init_net(net, ipip_net_id, core::ptr::null_mut(), b"tunl0\0".as_ptr()) }
unsafe fn ipip_exit_rtnl(net: *mut net, dev_to_kill: *mut list_head) { ip_tunnel_delete_net(net, ipip_net_id, core::ptr::null_mut(), dev_to_kill); }
unsafe fn ipip_init() -> i32 { register_pernet_device(core::ptr::null_mut()) }
unsafe fn ipip_fini() { rtnl_link_unregister(core::ptr::null_mut()); unregister_pernet_device(core::ptr::null_mut()); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
