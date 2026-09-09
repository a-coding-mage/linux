// SPDX-License-Identifier: GPL-2.0
/* Rust translation of netlink.c. External kernel and batman-adv symbols are
 * intentionally left as dependencies supplied by the surrounding tree. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct genl_family { _private: [u8; 0] }
#[repr(C)] pub struct genl_multicast_group { pub name: *const c_char }
#[repr(C)] pub struct nla_policy { pub kind: u16, pub len: u16 }
#[repr(C)] pub struct sk_buff { pub len: u32 }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct nlmsghdr { _private: [u8; 0] }
#[repr(C)] pub struct genl_info { pub user_ptr: [*mut c_void; 2], pub attrs: *mut *mut nlattr, pub snd_portid: u32, pub snd_seq: u32 }
#[repr(C)] pub struct netlink_callback { pub skb: *mut sk_buff, pub nlh: *mut nlmsghdr, pub args: [u64; 8], pub seq: u32 }
#[repr(C)] pub struct net_device { pub ifindex: u32, pub name: [c_char; 16], pub dev_addr: *mut u8 }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct batadv_priv { pub mesh_iface: *mut net_device, _private: [u8; 0] }
#[repr(C)] pub struct batadv_hard_iface { pub net_dev: *mut net_device, pub mesh_iface: *mut net_device, _private: [u8; 0] }
#[repr(C)] pub struct batadv_meshif_vlan { pub vid: u16, pub ap_isolation: u8, _private: [u8; 0] }

pub const BATADV_NL_MCGRP_CONFIG: usize = 0;
pub const BATADV_NL_MCGRP_TPMETER: usize = 1;
pub const BATADV_FLAG_NEED_MESH: u32 = 1;
pub const BATADV_FLAG_NEED_HARDIF: u32 = 2;
pub const BATADV_FLAG_NEED_VLAN: u32 = 4;

#[repr(u32)] pub enum batadv_netlink_multicast_groups { Config = 0, Tpmeter = 1 }
pub static mut batadv_netlink_family: genl_family = genl_family { _private: [] };

extern "C" {
    fn nlmsg_find_attr(*const nlmsghdr, u32, c_int) -> *mut nlattr;
    fn nla_len(*mut nlattr) -> usize; fn nla_get_u8(*mut nlattr) -> u8; fn nla_get_u16(*mut nlattr) -> u16;
    fn nla_get_u32(*mut nlattr) -> u32; fn nla_data(*mut nlattr) -> *mut u8;
    fn nlmsg_new(usize, u32) -> *mut sk_buff; fn nlmsg_free(*mut sk_buff);
    fn genlmsg_put(*mut sk_buff,u32,u32,*mut genl_family,c_int,u32)->*mut c_void;
    fn genlmsg_end(*mut sk_buff,*mut c_void); fn genlmsg_cancel(*mut sk_buff,*mut c_void);
    fn genlmsg_reply(*mut sk_buff,*mut genl_info)->c_int;
    fn genlmsg_multicast_netns(*mut genl_family,*mut net,*mut sk_buff,u32,u32,u32)->c_int;
    fn nla_put_u8(*mut sk_buff,c_int,u8)->c_int; fn nla_put_u16(*mut sk_buff,c_int,u16)->c_int;
    fn nla_put_u32(*mut sk_buff,c_int,u32)->c_int; fn nla_put_string(*mut sk_buff,c_int,*const c_char)->c_int;
    fn nla_put(*mut sk_buff,c_int,usize,*const u8)->c_int; fn nla_put_flag(*mut sk_buff,c_int)->c_int;
    fn nla_put_u64_64bit(*mut sk_buff,c_int,u64,c_int)->c_int;
    fn dev_net(*mut net_device)->*mut net; fn dev_get_by_index(*mut net,c_int)->*mut net_device; fn dev_put(*mut net_device);
    fn sock_net(*mut c_void)->*mut net; fn netdev_priv(*mut net_device)->*mut batadv_priv;
    fn batadv_meshif_is_valid(*mut net_device)->bool; fn batadv_meshif_vlan_get(*mut batadv_priv,u16)->*mut batadv_meshif_vlan;
    fn batadv_meshif_vlan_put(*mut batadv_meshif_vlan); fn batadv_hardif_get_by_netdev(*mut net_device)->*mut batadv_hard_iface;
    fn batadv_hardif_put(*mut batadv_hard_iface); fn batadv_tp_start(*mut batadv_priv,*const u8,u32,*mut u32);
    fn batadv_tp_stop(*mut batadv_priv,*const u8,u8); fn rtnl_lock(); fn rtnl_unlock();
    fn genl_register_family(*mut genl_family)->c_int; fn genl_unregister_family(*mut genl_family);
}

pub unsafe fn batadv_netlink_get_ifindex(nlh: *const nlmsghdr, attrtype: c_int) -> c_int {
    let attr = nlmsg_find_attr(nlh, 4, attrtype);
    if !attr.is_null() && nla_len(attr) == 4 { nla_get_u32(attr) as c_int } else { 0 }
}

pub unsafe fn batadv_netlink_tpmeter_notify(bat_priv: *mut batadv_priv, dst: *const u8,
    result: u8, test_time: u32, total_bytes: u64, cookie: u32) -> c_int {
    let msg = nlmsg_new(4096, 0); if msg.is_null() { return -12; }
    let hdr = genlmsg_put(msg, 0, 0, &mut batadv_netlink_family, 0, 3);
    if hdr.is_null() { nlmsg_free(msg); return -105; }
    if nla_put_u32(msg, 3, cookie) != 0 || nla_put_u32(msg, 4, test_time) != 0 ||
       nla_put_u64_64bit(msg, 5, total_bytes, 0) != 0 || nla_put_u8(msg, 6, result) != 0 ||
       nla_put(msg, 8, 6, dst) != 0 { genlmsg_cancel(msg,hdr); nlmsg_free(msg); return -90; }
    genlmsg_end(msg,hdr); genlmsg_multicast_netns(&mut batadv_netlink_family,
        dev_net((*bat_priv).mesh_iface), msg, 0, BATADV_NL_MCGRP_TPMETER as u32, 0); 0
}

pub unsafe fn batadv_netlink_get_meshif_from_ifindex(net: *mut net, ifindex: c_int) -> *mut net_device {
    let dev = dev_get_by_index(net, ifindex); if dev.is_null() { return (-19isize) as *mut net_device; }
    if batadv_meshif_is_valid(dev) { dev } else { dev_put(dev); (-22isize) as *mut net_device }
}

pub unsafe fn batadv_netlink_get_meshif(cb: *mut netlink_callback) -> *mut net_device {
    let i = batadv_netlink_get_ifindex((*cb).nlh, 3); if i == 0 { return (-64isize) as *mut net_device; }
    batadv_netlink_get_meshif_from_ifindex(sock_net((*cb).skb as *mut c_void), i)
}

pub unsafe fn batadv_netlink_register() -> c_int {
    let ret = genl_register_family(&mut batadv_netlink_family); ret
}
pub unsafe fn batadv_netlink_unregister() { genl_unregister_family(&mut batadv_netlink_family); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
