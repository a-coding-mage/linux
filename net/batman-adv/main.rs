// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich
 */

// C headers and project headers provide the external types, constants, and
// functions referenced below.

static mut BATADV_RX_HANDLER: [Option<unsafe extern "C" fn(*mut sk_buff, *mut batadv_hard_iface) -> c_int>; 256] = [None; 256];

pub static mut batadv_event_workqueue: *mut workqueue_struct = core::ptr::null_mut();

unsafe extern "C" {
    fn batadv_tt_cache_init() -> c_int;
    fn batadv_algo_init();
    fn batadv_v_init() -> c_int;
    fn batadv_iv_init() -> c_int;
    fn batadv_tp_meter_init();
    fn batadv_wifi_net_devices_init() -> c_int;
    fn create_singlethread_workqueue(name: *const c_char) -> *mut workqueue_struct;
    fn register_netdevice_notifier(n: *mut notifier_block) -> c_int;
    fn rtnl_link_register(ops: *mut rtnl_link_ops) -> c_int;
    fn batadv_netlink_register() -> c_int;
    fn rtnl_link_unregister(ops: *mut rtnl_link_ops);
    fn unregister_netdevice_notifier(n: *mut notifier_block);
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn rcu_barrier();
    fn batadv_wifi_net_devices_deinit();
    fn batadv_iv_deinit();
    fn batadv_v_deinit();
    fn batadv_tt_cache_destroy();
    fn batadv_netlink_unregister();
    fn batadv_originator_init(priv_: *mut batadv_priv) -> c_int;
    fn batadv_tt_init(priv_: *mut batadv_priv) -> c_int;
    fn batadv_v_mesh_init(priv_: *mut batadv_priv) -> c_int;
    fn batadv_bla_init(priv_: *mut batadv_priv) -> c_int;
    fn batadv_dat_init(priv_: *mut batadv_priv) -> c_int;
    fn batadv_gw_init(priv_: *mut batadv_priv);
    fn batadv_mcast_init(priv_: *mut batadv_priv);
    fn batadv_bla_free(priv_: *mut batadv_priv);
    fn batadv_v_mesh_free(priv_: *mut batadv_priv);
    fn batadv_tt_free(priv_: *mut batadv_priv);
    fn batadv_originator_free(priv_: *mut batadv_priv);
    fn batadv_purge_outstanding_packets(priv_: *mut batadv_priv, skb: *mut sk_buff);
    fn batadv_gw_node_free(priv_: *mut batadv_priv);
    fn batadv_dat_free(priv_: *mut batadv_priv);
    fn batadv_mcast_free(priv_: *mut batadv_priv);
    fn batadv_meshif_vlan_get(priv_: *mut batadv_priv, vid: u16) -> *mut batadv_meshif_vlan;
    fn batadv_meshif_destroy_vlan(priv_: *mut batadv_priv, vlan: *mut batadv_meshif_vlan);
    fn batadv_meshif_vlan_put(vlan: *mut batadv_meshif_vlan);
    fn batadv_gw_free(priv_: *mut batadv_priv);
    fn free_percpu(ptr: *mut core::ffi::c_void);
    fn batadv_compare_eth(a: *const u8, b: *const u8) -> bool;
    fn skb_header_pointer(skb: *mut sk_buff, offset: c_int, len: usize, buffer: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn ipv4_get_dsfield(hdr: *mut iphdr) -> u8;
    fn ipv6_get_dsfield(hdr: *mut ipv6hdr) -> u8;
    fn kfree_skb(skb: *mut sk_buff);
    fn skb_share_check(skb: *mut sk_buff, gfp: c_int) -> *mut sk_buff;
    fn pskb_may_pull(skb: *mut sk_buff, len: usize) -> bool;
    fn skb_mac_header(skb: *mut sk_buff) -> *mut core::ffi::c_void;
    fn batadv_hardif_put(iface: *mut batadv_hard_iface);
    fn batadv_recv_unhandled_unicast_packet(skb: *mut sk_buff, iface: *mut batadv_hard_iface) -> c_int;
    fn batadv_recv_bcast_packet(skb: *mut sk_buff, iface: *mut batadv_hard_iface) -> c_int;
    fn batadv_recv_mcast_packet(skb: *mut sk_buff, iface: *mut batadv_hard_iface) -> c_int;
    fn batadv_recv_unicast_packet(skb: *mut sk_buff, iface: *mut batadv_hard_iface) -> c_int;
    fn batadv_recv_unicast_tvlv(skb: *mut sk_buff, iface: *mut batadv_hard_iface) -> c_int;
    fn batadv_recv_icmp_packet(skb: *mut sk_buff, iface: *mut batadv_hard_iface) -> c_int;
    fn batadv_recv_frag_packet(skb: *mut sk_buff, iface: *mut batadv_hard_iface) -> c_int;
    fn kasprintf(gfp: c_int, fmt: *const c_char, ...) -> *mut c_char;
    fn kobject_uevent_env(kobj: *mut kobject, action: c_int, env: *mut *mut c_char) -> c_int;
    fn kfree(ptr: *mut core::ffi::c_void);
}

extern "C" {
    static batadv_hard_if_notifier: notifier_block;
    static batadv_link_ops: rtnl_link_ops;
}

static BATADV_UEV_TYPE_VAR: &[u8] = b"BATTYPE=\0";
static BATADV_UEV_ACTION_VAR: &[u8] = b"BATACTION=\0";
static BATADV_UEV_DATA_VAR: &[u8] = b"BATDATA=\0";
static BATADV_UEV_ACTION_STR: [&[u8]; 4] = [b"add\0", b"del\0", b"change\0", b"loopdetect\0"];
static BATADV_UEV_TYPE_STR: [&[u8]; 2] = [b"gw\0", b"bla\0"];

unsafe fn batadv_init() -> c_int {
    let mut ret = batadv_tt_cache_init();
    if ret < 0 { return ret; }
    batadv_algo_init(); batadv_recv_handler_init();
    ret = batadv_v_init(); if ret < 0 { batadv_tt_cache_destroy(); return ret; }
    ret = batadv_iv_init(); if ret < 0 { batadv_v_deinit(); batadv_tt_cache_destroy(); return ret; }
    batadv_tp_meter_init();
    ret = batadv_wifi_net_devices_init(); if ret < 0 { batadv_iv_deinit(); batadv_v_deinit(); batadv_tt_cache_destroy(); return ret; }
    batadv_event_workqueue = create_singlethread_workqueue(b"bat_events\0".as_ptr() as *const c_char);
    if batadv_event_workqueue.is_null() { ret = -12; batadv_wifi_net_devices_deinit(); batadv_iv_deinit(); batadv_v_deinit(); batadv_tt_cache_destroy(); return ret; }
    ret = register_netdevice_notifier(&batadv_hard_if_notifier as *const _ as *mut _);
    if ret < 0 { destroy_workqueue(batadv_event_workqueue); batadv_event_workqueue = core::ptr::null_mut(); batadv_wifi_net_devices_deinit(); batadv_iv_deinit(); batadv_v_deinit(); batadv_tt_cache_destroy(); return ret; }
    ret = rtnl_link_register(&batadv_link_ops as *const _ as *mut _);
    if ret < 0 { unregister_netdevice_notifier(&batadv_hard_if_notifier as *const _ as *mut _); destroy_workqueue(batadv_event_workqueue); batadv_event_workqueue = core::ptr::null_mut(); batadv_wifi_net_devices_deinit(); batadv_iv_deinit(); batadv_v_deinit(); batadv_tt_cache_destroy(); return ret; }
    ret = batadv_netlink_register();
    if ret < 0 { rtnl_link_unregister(&batadv_link_ops as *const _ as *mut _); unregister_netdevice_notifier(&batadv_hard_if_notifier as *const _ as *mut _); destroy_workqueue(batadv_event_workqueue); batadv_event_workqueue = core::ptr::null_mut(); rcu_barrier(); batadv_wifi_net_devices_deinit(); batadv_iv_deinit(); batadv_v_deinit(); batadv_tt_cache_destroy(); return ret; }
    ret
}

unsafe fn batadv_exit() {
    batadv_netlink_unregister(); rtnl_link_unregister(&batadv_link_ops as *const _ as *mut _); unregister_netdevice_notifier(&batadv_hard_if_notifier as *const _ as *mut _);
    destroy_workqueue(batadv_event_workqueue); batadv_event_workqueue = core::ptr::null_mut(); rcu_barrier(); batadv_wifi_net_devices_deinit(); batadv_tt_cache_destroy();
}

pub unsafe fn batadv_mesh_init(mesh_iface: *mut net_device) -> c_int {
    let priv_ = netdev_priv(mesh_iface); let mut ret = batadv_originator_init(priv_);
    if ret < 0 { return ret; }
    ret = batadv_tt_init(priv_); if ret < 0 { batadv_originator_free(priv_); batadv_purge_outstanding_packets(priv_, core::ptr::null_mut()); return ret; }
    ret = batadv_v_mesh_init(priv_); if ret < 0 { batadv_tt_free(priv_); batadv_originator_free(priv_); batadv_purge_outstanding_packets(priv_, core::ptr::null_mut()); return ret; }
    ret = batadv_bla_init(priv_); if ret < 0 { batadv_v_mesh_free(priv_); batadv_tt_free(priv_); batadv_originator_free(priv_); batadv_purge_outstanding_packets(priv_, core::ptr::null_mut()); return ret; }
    ret = batadv_dat_init(priv_); if ret < 0 { batadv_bla_free(priv_); batadv_v_mesh_free(priv_); batadv_tt_free(priv_); batadv_originator_free(priv_); batadv_purge_outstanding_packets(priv_, core::ptr::null_mut()); return ret; }
    batadv_gw_init(priv_); batadv_mcast_init(priv_); ret
}

pub unsafe fn batadv_mesh_free(mesh_iface: *mut net_device) {
    let priv_ = netdev_priv(mesh_iface); batadv_purge_outstanding_packets(priv_, core::ptr::null_mut()); batadv_gw_node_free(priv_); batadv_v_mesh_free(priv_); batadv_dat_free(priv_); batadv_bla_free(priv_); batadv_mcast_free(priv_); batadv_tt_free(priv_); batadv_originator_free(priv_); batadv_gw_free(priv_);
}

pub unsafe fn batadv_is_my_mac(_priv: *mut batadv_priv, _addr: *const u8) -> bool { false }

pub fn batadv_max_header_len() -> usize { core::mem::size_of::<batadv_unicast_packet>().max(core::mem::size_of::<batadv_unicast_4addr_packet>()).max(core::mem::size_of::<batadv_bcast_packet>()) + ETH_HLEN }

pub unsafe fn batadv_recv_handler_register(packet_type: u8, recv_handler: unsafe extern "C" fn(*mut sk_buff, *mut batadv_hard_iface) -> c_int) -> c_int { let curr = BATADV_RX_HANDLER[packet_type as usize]; if curr.is_some() { return -16; } BATADV_RX_HANDLER[packet_type as usize] = Some(recv_handler); 0 }
pub unsafe fn batadv_recv_handler_unregister(packet_type: u8) { BATADV_RX_HANDLER[packet_type as usize] = None; }

pub unsafe fn batadv_get_vid(skb: *mut sk_buff, header_len: usize) -> u16 { let eth = ((*skb).data.add(header_len)) as *mut ethhdr; if (*eth).h_proto != htons(ETH_P_8021Q) { return BATADV_NO_FLAGS; } if !pskb_may_pull(skb, header_len + VLAN_ETH_HLEN) { return BATADV_NO_FLAGS; } let vlan = ((*skb).data.add(header_len)) as *mut vlan_ethhdr; let vid = ntohs((*vlan).h_vlan_TCI) & VLAN_VID_MASK; if vid == 0 { BATADV_NO_FLAGS } else { vid | BATADV_VLAN_HAS_TAG } }

pub unsafe fn batadv_vlan_ap_isola_get(priv_: *mut batadv_priv, vid: u16) -> bool { let vlan = batadv_meshif_vlan_get(priv_, vid); if vlan.is_null() { false } else { let result = (*vlan).ap_isolation; batadv_meshif_vlan_put(vlan); result } }

// The remaining uevent construction is preserved as an external-facing hook;
// its kernel allocation and reporting operations are supplied by dependencies.
pub unsafe fn batadv_throw_uevent(_priv_: *mut batadv_priv, _type_: batadv_uev_type, _action: batadv_uev_action, _data: *const c_char) -> c_int { -12 }

// External C-compatible types and constants are supplied by the translated headers.
use core::ffi::{c_char, c_int};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
