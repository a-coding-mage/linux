// SPDX-License-Identifier: GPL-2.0
/* Copyright 2011-2014 Autronica Fire and Security AS
 * Device methods for creating, using and destroying virtual HSR or PRP devices.
 * The kernel types and helpers referenced below are supplied by the surrounding
 * translation unit.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

extern "C" {
    fn netdev_priv(dev: *mut net_device) -> *mut hsr_priv;
    fn netif_oper_up(dev: *mut net_device) -> bool;
    fn netif_running(dev: *mut net_device) -> bool;
    fn netif_set_operstate(dev: *mut net_device, state: c_int);
    fn netif_carrier_on(dev: *mut net_device);
    fn netif_carrier_off(dev: *mut net_device);
    fn hsr_port_get_hsr(hsr: *mut hsr_priv, ty: c_int) -> *mut hsr_port;
    fn hsr_del_port(port: *mut hsr_port);
    fn hsr_forward_skb(skb: *mut sk_buff, port: *mut hsr_port);
    fn hsr_init_skb_external(_: *mut hsr_port, _: c_int) -> *mut sk_buff;
    fn dev_alloc_skb(len: c_int) -> *mut sk_buff;
    fn dev_kfree_skb_any(skb: *mut sk_buff);
    fn kfree_skb(skb: *mut sk_buff);
    fn skb_put(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn skb_put_padto(skb: *mut sk_buff, len: usize) -> c_int;
    fn htons(v: u16) -> u16;
    fn hsr_is_node_in_db(db: *mut c_void, addr: *const u8) -> bool;
    fn hsr_addr_is_redbox(hsr: *mut hsr_priv, addr: *const u8) -> bool;
    fn eth_header(_: *mut sk_buff, _: *mut net_device, _: u16, _: *const u8, _: *const u8, _: usize) -> c_int;
    fn eth_header_parse(_: *mut sk_buff, _: *mut u8) -> c_int;
    fn hsr_create_tagged_frame(_: *mut c_void, _: *mut c_void) -> c_int;
    fn hsr_get_untagged_frame(_: *mut c_void, _: *mut c_void) -> c_int;
    fn hsr_drop_frame(_: *mut c_void) -> bool;
    fn hsr_fill_frame_info(_: *mut c_void, _: *mut c_void) -> c_int;
    fn hsr_invalid_dan_ingress_frame(_: *mut c_void, _: *mut c_void) -> bool;
    fn hsr_register_frame_out(_: *mut c_void, _: *mut c_void) -> c_int;
    fn prp_create_tagged_frame(_: *mut c_void, _: *mut c_void) -> c_int;
    fn prp_get_untagged_frame(_: *mut c_void, _: *mut c_void) -> c_int;
    fn prp_fill_frame_info(_: *mut c_void, _: *mut c_void) -> c_int;
    fn prp_handle_san_frame(_: *mut c_void, _: *mut c_void) -> c_int;
    fn prp_update_san_info(_: *mut c_void, _: *mut c_void) -> c_int;
}

#[repr(C)] pub struct net_device { pub flags: u32, pub mtu: c_int, pub dev_addr: [u8; 6], pub netdev_ops: *const c_void, pub lltx: bool, pub features: u64, pub hw_features: u64, pub priv_flags: u32, pub min_mtu: u32, pub header_ops: *const c_void, pub name: *const c_char }
#[repr(C)] pub struct sk_buff { pub dev: *mut net_device, pub priority: u32, pub len: usize }
#[repr(C)] pub struct hsr_port { pub hsr: *mut hsr_priv, pub dev: *mut net_device, pub ty: c_int }
#[repr(C)] pub struct hsr_priv { pub announce_count: u32, pub prot_version: u8, pub redbox: bool, pub sequence_nr: u16, pub sup_sequence_nr: u16, pub macaddress_redbox: [u8; 6], pub sup_multicast_addr: [u8; 6], pub announce_timer: c_void, pub announce_proxy_timer: c_void, pub proxy_node_db: c_void, pub seqnr_lock: c_void, pub proto_ops: *const hsr_proto_ops }
#[repr(C)] pub struct hsr_proto_ops { pub send_sv_frame: Option<unsafe extern "C" fn(*mut hsr_port, *mut c_ulong, *const u8)> }

const IFF_UP: u32 = 1; const IFF_ALLMULTI: u32 = 0x200; const HSR_PT_MASTER: c_int = 0; const HSR_PT_SLAVE_A: c_int = 1; const HSR_PT_SLAVE_B: c_int = 2; const HSR_PT_INTERLINK: c_int = 3;
const HSR_HLEN: c_int = 6; const ETH_DATA_LEN: u32 = 1500; const NETDEV_TX_OK: c_int = 0;

unsafe fn is_admin_up(dev: *mut net_device) -> bool { !dev.is_null() && ((*dev).flags & IFF_UP) != 0 }
unsafe fn is_slave_up(dev: *mut net_device) -> bool { !dev.is_null() && is_admin_up(dev) && netif_oper_up(dev) }

unsafe fn hsr_set_operstate(master: *mut hsr_port, has_carrier: bool) { let dev=(*master).dev; if !is_admin_up(dev) { netif_set_operstate(dev, 0); return; } netif_set_operstate(dev, if has_carrier { 1 } else { 2 }); }

unsafe fn hsr_check_carrier(master: *mut hsr_port) -> bool { let hsr=(*master).hsr; for ty in [HSR_PT_SLAVE_A,HSR_PT_SLAVE_B,HSR_PT_INTERLINK] { let p=hsr_port_get_hsr(hsr,ty); if !p.is_null() && is_slave_up((*p).dev) { netif_carrier_on((*master).dev); return true; } } netif_carrier_off((*master).dev); false }

unsafe fn hsr_check_announce(dev: *mut net_device) { let hsr=netdev_priv(dev); if netif_running(dev) && netif_oper_up(dev) { (*hsr).announce_count=0; } }

#[no_mangle] pub unsafe extern "C" fn hsr_check_carrier_and_operstate(hsr:*mut hsr_priv) { let master=hsr_port_get_hsr(hsr,HSR_PT_MASTER); let carrier=hsr_check_carrier(master); hsr_set_operstate(master,carrier); hsr_check_announce((*master).dev); }

#[no_mangle] pub unsafe extern "C" fn hsr_get_max_mtu(hsr:*mut hsr_priv) -> c_int { let mut max=ETH_DATA_LEN; for ty in [HSR_PT_SLAVE_A,HSR_PT_SLAVE_B,HSR_PT_INTERLINK] { let p=hsr_port_get_hsr(hsr,ty); if !p.is_null() { max=max.min((*p).dev.read().mtu as u32); } } if max < HSR_HLEN as u32 { 0 } else { (max-HSR_HLEN as u32) as c_int } }

unsafe fn hsr_dev_change_mtu(dev:*mut net_device,new_mtu:c_int)->c_int { let hsr=netdev_priv(dev); if new_mtu>hsr_get_max_mtu(hsr){return -22;} (*dev).mtu=new_mtu; 0 }
unsafe fn hsr_dev_open(_dev:*mut net_device)->c_int { 0 }
unsafe fn hsr_dev_close(_dev:*mut net_device)->c_int { 0 }
unsafe fn hsr_dev_xmit(skb:*mut sk_buff,dev:*mut net_device)->c_int { let hsr=netdev_priv(dev); let master=hsr_port_get_hsr(hsr,HSR_PT_MASTER); if !master.is_null(){(*skb).dev=(*master).dev; hsr_forward_skb(skb,master);}else{dev_kfree_skb_any(skb);} NETDEV_TX_OK }

unsafe fn send_hsr_supervision_frame(port:*mut hsr_port, interval:*mut c_ulong, addr:*const u8) { let hsr=(*port).hsr; *interval=0; let skb=hsr_init_skb_external(port,0); if skb.is_null(){return;} let tag=skb_put(skb,6); *tag=0; for i in 0..6 {*tag.add(i)=addr.add(i).read();} hsr_forward_skb(skb,port); (*hsr).sequence_nr=(*hsr).sequence_nr.wrapping_add(1); }
unsafe fn send_prp_supervision_frame(master:*mut hsr_port,interval:*mut c_ulong,addr:*const u8){send_hsr_supervision_frame(master,interval,addr)}

#[no_mangle] pub unsafe extern "C" fn hsr_del_ports(hsr:*mut hsr_priv){for ty in [HSR_PT_SLAVE_A,HSR_PT_SLAVE_B,HSR_PT_INTERLINK,HSR_PT_MASTER]{let p=hsr_port_get_hsr(hsr,ty);if !p.is_null(){hsr_del_port(p);}}}
#[no_mangle] pub unsafe extern "C" fn hsr_dev_setup(_dev:*mut net_device) {}
#[no_mangle] pub unsafe extern "C" fn is_hsr_master(dev:*mut net_device)->bool{!dev.is_null()}
#[no_mangle] pub unsafe extern "C" fn hsr_get_port_ndev(ndev:*mut net_device,pt:c_int)->*mut net_device{let h=netdev_priv(ndev);let p=hsr_port_get_hsr(h,pt);if p.is_null(){core::ptr::null_mut()}else{(*p).dev}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
