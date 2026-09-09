// SPDX-License-Identifier: GPL-2.0-or-later
/* Ethernet-type device handling. */

// Linux kernel dependencies supplied by other translated units.

use core::ffi::{c_char, c_int, c_uint, c_void};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type Be16 = u16;
type SizeT = usize;

#[repr(C)] pub struct sk_buff { pub dev: *mut net_device, pub data: *mut U8, pub encapsulation: bool }
#[repr(C)] pub struct net_device {
    pub dev_addr: *mut U8, pub broadcast: *mut U8, pub header_ops: *const header_ops,
    pub type_: c_int, pub hard_header_len: c_int, pub min_header_len: c_int,
    pub mtu: c_int, pub min_mtu: c_int, pub max_mtu: c_int, pub addr_len: c_int,
    pub tx_queue_len: c_uint, pub flags: c_uint, pub priv_flags: c_uint,
}
#[repr(C)] pub struct ethhdr { pub h_dest: [U8; 6], pub h_source: [U8; 6], pub h_proto: Be16 }
#[repr(C)] pub struct sockaddr { pub sa_data: [U8; 14] }
#[repr(C)] pub struct neighbour { pub dev: *const net_device, pub ha: *const U8 }
#[repr(C)] pub struct hh_cache { pub hh_data: [U8; 16], pub hh_len: U16 }
#[repr(C)] pub struct device { pub of_node: *mut c_void }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct packet_offload { pub type_: Be16, pub priority: c_int, pub callbacks: packet_offload_callbacks }
#[repr(C)] pub struct packet_offload_callbacks {
    pub gro_receive: Option<unsafe extern "C" fn(*mut list_head, *mut sk_buff) -> *mut sk_buff>,
    pub gro_complete: Option<unsafe extern "C" fn(*mut sk_buff, c_int) -> c_int>,
}
#[repr(C)] pub struct header_ops {
    pub create: Option<unsafe extern "C" fn(*mut sk_buff, *mut net_device, U16, *const c_void, *const c_void, c_uint) -> c_int>,
    pub parse: Option<unsafe extern "C" fn(*const sk_buff, *const net_device, *mut U8) -> c_int>,
    pub cache: Option<unsafe extern "C" fn(*const neighbour, *mut hh_cache, Be16) -> c_int>,
    pub cache_update: Option<unsafe extern "C" fn(*mut hh_cache, *const net_device, *const U8)>,
    pub parse_protocol: Option<unsafe extern "C" fn(*const sk_buff) -> Be16>,
}
#[repr(C)] pub struct flow_keys_basic { pub control: flow_keys_control }
#[repr(C)] pub struct flow_keys_control { pub thoff: U32 }

extern "C" {
    fn htons(x: U16) -> U16;
    fn cpu_to_be16(x: U16) -> U16;
    fn skb_push(skb: *mut sk_buff, len: usize) -> *mut ethhdr;
    fn skb_reset_mac_header(skb: *mut sk_buff);
    fn eth_skb_pull_mac(skb: *mut sk_buff) -> *const ethhdr;
    fn eth_skb_pkt_type(skb: *mut sk_buff, dev: *mut net_device);
    fn skb_header_pointer(skb: *const sk_buff, offset: c_int, len: usize, buffer: *mut c_void) -> *const U16;
    fn eth_hdr(skb: *const sk_buff) -> *const ethhdr;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn eth_zero_addr(addr: *mut U8);
    fn is_valid_ether_addr(addr: *const U8) -> bool;
    fn netif_running(dev: *const net_device) -> bool;
    fn eth_hw_addr_set(dev: *mut net_device, addr: *const U8);
    fn alloc_netdev_mqs(size: c_int, name: *const c_char, name_assign_type: c_uint, setup: unsafe extern "C" fn(*mut net_device), txqs: c_uint, rxqs: c_uint) -> *mut net_device;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn dev_net(dev: *const net_device) -> *mut c_void;
    fn skb_flow_dissect_flow_keys_basic(net: *mut c_void, skb: *mut sk_buff, keys: *mut flow_keys_basic, data: *const c_void, proto: Be16, nhoff: usize, len: U32, flags: U32) -> bool;
    fn __skb_get_poff(skb: *mut sk_buff, data: *const c_void, keys: *mut flow_keys_basic, len: U32) -> U32;
    fn netdev_uses_dsa(dev: *const net_device) -> bool;
    fn eth_proto_is_802_3(proto: Be16) -> bool;
    fn HH_DATA_OFF(size: usize) -> usize;
    fn smp_store_release(dst: *mut U16, value: U16);
    fn skb_gro_offset(skb: *const sk_buff) -> U32;
    fn skb_gro_header(skb: *mut sk_buff, hlen: U32, off: U32) -> *mut ethhdr;
    fn compare_ether_header(a: *const ethhdr, b: *const ethhdr) -> c_int;
    fn gro_find_receive_by_type(ty: Be16) -> *mut packet_offload;
    fn skb_gro_pull(skb: *mut sk_buff, len: usize);
    fn skb_gro_postpull_rcsum(skb: *mut sk_buff, start: *const c_void, len: usize);
    fn indirect_call_gro_receive_inet(f: Option<unsafe extern "C" fn(*mut list_head, *mut sk_buff) -> *mut sk_buff>, a: unsafe extern "C" fn(*mut list_head, *mut sk_buff) -> *mut sk_buff, b: unsafe extern "C" fn(*mut list_head, *mut sk_buff) -> *mut sk_buff, head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff;
    fn skb_gro_flush_final(skb: *mut sk_buff, pp: *mut sk_buff, flush: c_int);
    fn gro_find_complete_by_type(ty: Be16) -> *mut packet_offload;
    fn skb_set_inner_mac_header(skb: *mut sk_buff, nhoff: c_int);
    fn INDIRECT_CALL_INET(f: Option<unsafe extern "C" fn(*mut sk_buff, c_int) -> c_int>, a: unsafe extern "C" fn(*mut sk_buff, c_int) -> c_int, b: unsafe extern "C" fn(*mut sk_buff, c_int) -> c_int, skb: *mut sk_buff, nhoff: c_int) -> c_int;
    fn dev_add_offload(offload: *mut packet_offload) -> c_int;
    fn of_get_mac_address(node: *mut c_void, addr: *mut U8) -> c_int;
    fn ether_addr_copy(dst: *mut U8, src: *const U8);
    fn eth_broadcast_addr(addr: *mut U8);
    fn ipv6_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff;
    fn inet_gro_receive(head: *mut list_head, skb: *mut sk_buff) -> *mut sk_buff;
    fn ipv6_gro_complete(skb: *mut sk_buff, nhoff: c_int) -> c_int;
    fn inet_gro_complete(skb: *mut sk_buff, nhoff: c_int) -> c_int;
}

const ETH_HLEN: c_int = 14; const ETH_ALEN: usize = 6; const ETH_DATA_LEN: c_int = 1500;
const ETH_MIN_MTU: c_int = 68; const ETH_P_802_3: U16 = 1; const ETH_P_802_2: U16 = 4;
const ETH_P_XDSA: U16 = 0x00f1; const ETH_P_TEB: U16 = 0x6558; const ARPHRD_ETHER: c_int = 1;
const IFF_LOOPBACK: c_uint = 0x8; const IFF_NOARP: c_uint = 0x80; const IFF_BROADCAST: c_uint = 2;
const IFF_MULTICAST: c_uint = 0x1000; const IFF_TX_SKB_SHARING: c_uint = 0x1000000;
const IFF_LIVE_ADDR_CHANGE: c_uint = 0x400000; const DEFAULT_TX_QUEUE_LEN: c_uint = 1000;
const NET_NAME_ENUM: c_uint = 1; const FLOW_DISSECTOR_F_PARSE_1ST_FRAG: U32 = 1; const ENOBUSY: c_int = 16;
const EADDRNOTAVAIL: c_int = 99; const ENODEV: c_int = 19; const ENOSYS: c_int = 38;

#[no_mangle] pub unsafe extern "C" fn eth_header(skb: *mut sk_buff, dev: *mut net_device, mut type_: U16, daddr: *const c_void, mut saddr: *const c_void, len: c_uint) -> c_int {
    let eth = skb_push(skb, ETH_HLEN as usize);
    eth.as_mut().unwrap().h_proto = if type_ != ETH_P_802_3 && type_ != ETH_P_802_2 { htons(type_) } else { htons(len as U16) };
    if saddr.is_null() { saddr = (*dev).dev_addr as *const c_void; }
    memcpy((*eth).h_source.as_mut_ptr() as *mut c_void, saddr, ETH_ALEN);
    if !daddr.is_null() { memcpy((*eth).h_dest.as_mut_ptr() as *mut c_void, daddr, ETH_ALEN); return ETH_HLEN; }
    if (*dev).flags & (IFF_LOOPBACK | IFF_NOARP) != 0 { eth_zero_addr((*eth).h_dest.as_mut_ptr()); return ETH_HLEN; }
    -ETH_HLEN
}

#[no_mangle] pub unsafe extern "C" fn eth_get_headlen(dev: *const net_device, data: *const c_void, len: U32) -> U32 {
    let eth = data as *const ethhdr; let mut keys = flow_keys_basic { control: flow_keys_control { thoff: 0 } };
    if len < core::mem::size_of::<ethhdr>() as U32 { return len; }
    if !skb_flow_dissect_flow_keys_basic(dev_net(dev), core::ptr::null_mut(), &mut keys, data, (*eth).h_proto, core::mem::size_of::<ethhdr>(), len, FLOW_DISSECTOR_F_PARSE_1ST_FRAG) { return keys.control.thoff.max(core::mem::size_of::<ethhdr>() as U32); }
    __skb_get_poff(core::ptr::null_mut(), data, &mut keys, len).min(len)
}

#[no_mangle] pub unsafe extern "C" fn eth_type_trans(skb: *mut sk_buff, dev: *mut net_device) -> Be16 {
    (*skb).dev = dev; skb_reset_mac_header(skb); let eth = eth_skb_pull_mac(skb); eth_skb_pkt_type(skb, dev);
    if netdev_uses_dsa(dev) { return htons(ETH_P_XDSA); }
    if eth_proto_is_802_3((*eth).h_proto) { return (*eth).h_proto; }
    let mut storage = (*skb).dev; let sap = skb_header_pointer(skb, 0, 2, &mut storage as *mut _ as *mut c_void); (*skb).dev = dev;
    if !sap.is_null() && *sap == 0xffff { htons(ETH_P_802_3) } else { htons(ETH_P_802_2) }
}

#[no_mangle] pub unsafe extern "C" fn eth_header_parse(skb: *const sk_buff, _dev: *const net_device, haddr: *mut U8) -> c_int { memcpy(haddr as *mut c_void, (*eth_hdr(skb)).h_source.as_ptr() as *const c_void, ETH_ALEN); ETH_ALEN as c_int }
#[no_mangle] pub unsafe extern "C" fn eth_header_cache(neigh: *const neighbour, hh: *mut hh_cache, type_: Be16) -> c_int { if type_ == htons(ETH_P_802_3) { return -1; } let eth = ((*hh).hh_data.as_mut_ptr().add(HH_DATA_OFF(core::mem::size_of::<ethhdr>())) as *mut ethhdr); (*eth).h_proto = type_; memcpy((*eth).h_source.as_mut_ptr() as *mut c_void, (*(*neigh).dev).dev_addr as *const c_void, ETH_ALEN); memcpy((*eth).h_dest.as_mut_ptr() as *mut c_void, (*neigh).ha as *const c_void, ETH_ALEN); smp_store_release(&mut (*hh).hh_len, ETH_HLEN as U16); 0 }
#[no_mangle] pub unsafe extern "C" fn eth_header_cache_update(hh: *mut hh_cache, _dev: *const net_device, haddr: *const U8) { memcpy((*hh).hh_data.as_mut_ptr().add(HH_DATA_OFF(core::mem::size_of::<ethhdr>())) as *mut c_void, haddr as *const c_void, ETH_ALEN); }
#[no_mangle] pub unsafe extern "C" fn eth_header_parse_protocol(skb: *const sk_buff) -> Be16 { (*eth_hdr(skb)).h_proto }
#[no_mangle] pub unsafe extern "C" fn eth_prepare_mac_addr_change(dev: *mut net_device, p: *mut c_void) -> c_int { let addr = p as *mut sockaddr; if (*dev).priv_flags & IFF_LIVE_ADDR_CHANGE == 0 && netif_running(dev) { return -ENOBUSY; } if !is_valid_ether_addr((*addr).sa_data.as_ptr()) { return -EADDRNOTAVAIL; } 0 }
#[no_mangle] pub unsafe extern "C" fn eth_commit_mac_addr_change(dev: *mut net_device, p: *mut c_void) { eth_hw_addr_set(dev, (*(p as *mut sockaddr)).sa_data.as_ptr()); }
#[no_mangle] pub unsafe extern "C" fn eth_mac_addr(dev: *mut net_device, p: *mut c_void) -> c_int { let ret = eth_prepare_mac_addr_change(dev, p); if ret < 0 { return ret; } eth_commit_mac_addr_change(dev, p); 0 }
#[no_mangle] pub unsafe extern "C" fn eth_validate_addr(dev: *mut net_device) -> c_int { if !is_valid_ether_addr((*dev).dev_addr) { -EADDRNOTAVAIL } else { 0 } }

#[no_mangle] pub unsafe extern "C" fn ether_setup(dev: *mut net_device) { (*dev).header_ops = &eth_header_ops; (*dev).type_ = ARPHRD_ETHER; (*dev).hard_header_len = ETH_HLEN; (*dev).min_header_len = ETH_HLEN; (*dev).mtu = ETH_DATA_LEN; (*dev).min_mtu = ETH_MIN_MTU; (*dev).max_mtu = ETH_DATA_LEN; (*dev).addr_len = ETH_ALEN as c_int; (*dev).tx_queue_len = DEFAULT_TX_QUEUE_LEN; (*dev).flags = IFF_BROADCAST | IFF_MULTICAST; (*dev).priv_flags |= IFF_TX_SKB_SHARING; eth_broadcast_addr((*dev).broadcast); }
#[no_mangle] pub unsafe extern "C" fn alloc_etherdev_mqs(sizeof_priv: c_int, txqs: c_uint, rxqs: c_uint) -> *mut net_device { alloc_netdev_mqs(sizeof_priv, b"eth%d\0".as_ptr() as *const c_char, NET_NAME_ENUM, ether_setup, txqs, rxqs) }
#[no_mangle] pub unsafe extern "C" fn sysfs_format_mac(buf: *mut c_char, addr: *const U8, len: c_int) -> isize { sysfs_emit(buf, b"%*phC\n\0".as_ptr() as *const c_char, len, addr) }

#[no_mangle] pub unsafe extern "C" fn eth_gro_receive(_head: *mut list_head, _skb: *mut sk_buff) -> *mut sk_buff { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn eth_gro_complete(skb: *mut sk_buff, nhoff: c_int) -> c_int { if (*skb).encapsulation { skb_set_inner_mac_header(skb, nhoff); } -ENOSYS }

static mut eth_packet_offload: packet_offload = packet_offload { type_: 0x5865, priority: 10, callbacks: packet_offload_callbacks { gro_receive: Some(eth_gro_receive), gro_complete: Some(eth_gro_complete) } };
#[no_mangle] pub unsafe extern "C" fn eth_offload_init() -> c_int { dev_add_offload(&mut eth_packet_offload); 0 }

#[no_mangle] pub unsafe extern "C" fn eth_platform_get_mac_address(dev: *mut device, mac_addr: *mut U8) -> c_int { let ret = of_get_mac_address((*dev).of_node, mac_addr); if ret == 0 { return 0; } let addr = arch_get_platform_mac_address(); if addr.is_null() { return -ENODEV; } ether_addr_copy(mac_addr, addr); 0 }
#[no_mangle] pub unsafe extern "C" fn arch_get_platform_mac_address() -> *mut U8 { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn platform_get_ethdev_address(dev: *mut device, netdev: *mut net_device) -> c_int { let mut addr = [0u8; ETH_ALEN]; let ret = eth_platform_get_mac_address(dev, addr.as_mut_ptr()); if ret == 0 { eth_hw_addr_set(netdev, addr.as_ptr()); } ret }

pub static eth_header_ops: header_ops = header_ops { create: Some(eth_header), parse: Some(eth_header_parse), cache: Some(eth_header_cache), cache_update: Some(eth_header_cache_update), parse_protocol: Some(eth_header_parse_protocol) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
