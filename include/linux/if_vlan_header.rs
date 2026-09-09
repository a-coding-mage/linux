/* SPDX-License-Identifier: GPL-2.0-or-later */
// Rust translation of linux/if_vlan.h. Kernel dependencies are supplied externally.

pub const VLAN_HLEN: u32 = 4;
pub const VLAN_ETH_HLEN: u32 = 18;
pub const VLAN_ETH_ZLEN: u32 = 64;
pub const VLAN_ETH_DATA_LEN: u32 = 1500;
pub const VLAN_ETH_FRAME_LEN: u32 = 1518;
pub const VLAN_MAX_DEPTH: u32 = 8;

#[repr(C)]
pub struct vlan_hdr { pub h_vlan_TCI: __be16, pub h_vlan_encapsulated_proto: __be16 }

#[repr(C)]
pub struct vlan_ethhdr {
    pub h_dest: [u8; ETH_ALEN], pub h_source: [u8; ETH_ALEN],
    pub h_vlan_proto: __be16, pub h_vlan_TCI: __be16,
    pub h_vlan_encapsulated_proto: __be16,
}

pub unsafe fn vlan_eth_hdr(skb: *const sk_buff) -> *mut vlan_ethhdr {
    skb_mac_header(skb) as *mut vlan_ethhdr
}
pub unsafe fn skb_vlan_eth_hdr(skb: *const sk_buff) -> *mut vlan_ethhdr {
    (*skb).data as *mut vlan_ethhdr
}

pub const VLAN_PRIO_MASK: u16 = 0xe000;
pub const VLAN_PRIO_SHIFT: u32 = 13;
pub const VLAN_CFI_MASK: u16 = 0x1000;
pub const VLAN_VID_MASK: u16 = 0x0fff;
pub const VLAN_N_VID: u32 = 4096;

extern "C" { pub fn vlan_ioctl_set(hook: Option<unsafe extern "C" fn(*mut net, *mut core::ffi::c_void) -> i32>); }

#[inline] pub unsafe fn skb_vlan_tag_present(skb: *const sk_buff) -> bool { (*skb).vlan_all != 0 }
#[inline] pub unsafe fn skb_vlan_tag_get(skb: *const sk_buff) -> u16 { (*skb).vlan_tci }
#[inline] pub unsafe fn skb_vlan_tag_get_id(skb: *const sk_buff) -> u16 { (*skb).vlan_tci & VLAN_VID_MASK }
#[inline] pub unsafe fn skb_vlan_tag_get_cfi(skb: *const sk_buff) -> bool { (*skb).vlan_tci & VLAN_CFI_MASK != 0 }
#[inline] pub unsafe fn skb_vlan_tag_get_prio(skb: *const sk_buff) -> u16 { ((*skb).vlan_tci & VLAN_PRIO_MASK) >> VLAN_PRIO_SHIFT }

pub unsafe fn vlan_get_rx_ctag_filter_info(dev: *mut net_device) -> i32 { ASSERT_RTNL(); notifier_to_errno(call_netdevice_notifiers(NETDEV_CVLAN_FILTER_PUSH_INFO, dev)) }
pub unsafe fn vlan_drop_rx_ctag_filter_info(dev: *mut net_device) { ASSERT_RTNL(); call_netdevice_notifiers(NETDEV_CVLAN_FILTER_DROP_INFO, dev); }
pub unsafe fn vlan_get_rx_stag_filter_info(dev: *mut net_device) -> i32 { ASSERT_RTNL(); notifier_to_errno(call_netdevice_notifiers(NETDEV_SVLAN_FILTER_PUSH_INFO, dev)) }
pub unsafe fn vlan_drop_rx_stag_filter_info(dev: *mut net_device) { ASSERT_RTNL(); call_netdevice_notifiers(NETDEV_SVLAN_FILTER_DROP_INFO, dev); }

#[repr(C)] pub struct vlan_pcpu_stats { pub rx_packets: u64_stats_t, pub rx_bytes: u64_stats_t, pub rx_multicast: u64_stats_t, pub tx_packets: u64_stats_t, pub tx_bytes: u64_stats_t, pub syncp: u64_stats_sync, pub rx_errors: u32, pub tx_dropped: u32 }

#[repr(C)] pub struct vlan_priority_tci_mapping { pub priority: u32, pub vlan_qos: u16, pub next: *mut vlan_priority_tci_mapping, pub rcu: rcu_head }
pub enum proc_dir_entry {}
pub enum netpoll {}
#[repr(C)] pub struct vlan_dev_priv { pub nr_ingress_mappings: u32, pub ingress_priority_map: [u32;8], pub nr_egress_mappings: u32, pub egress_priority_map: [*mut vlan_priority_tci_mapping;16], pub vlan_proto: __be16, pub vlan_id: u16, pub flags: u16, pub real_dev: *mut net_device, pub dev_tracker: netdevice_tracker, pub real_dev_addr: [u8;ETH_ALEN], pub dent: *mut proc_dir_entry, pub vlan_pcpu_stats: *mut vlan_pcpu_stats, #[cfg(CONFIG_NET_POLL_CONTROLLER)] pub netpoll: *mut netpoll }

extern "C" { pub fn __vlan_find_dev_deep_rcu(dev:*mut net_device, proto:__be16, id:u16)->*mut net_device; pub fn vlan_for_each(dev:*mut net_device, action:Option<unsafe extern "C" fn(*mut net_device,i32,*mut core::ffi::c_void)->i32>, arg:*mut core::ffi::c_void)->i32; pub fn vlan_dev_real_dev(dev:*const net_device)->*mut net_device; pub fn vlan_dev_vlan_id(dev:*const net_device)->u16; pub fn vlan_dev_vlan_proto(dev:*const net_device)->__be16; pub fn vlan_do_receive(skb:*mut *mut sk_buff)->bool; pub fn vlan_vid_add(dev:*mut net_device,proto:__be16,vid:u16)->i32; pub fn vlan_vid_del(dev:*mut net_device,proto:__be16,vid:u16); pub fn vlan_vids_add_by_dev(dev:*mut net_device,by:*const net_device)->i32; pub fn vlan_vids_del_by_dev(dev:*mut net_device,by:*const net_device); pub fn vlan_uses_dev(dev:*const net_device)->bool; }

pub unsafe fn is_vlan_dev(dev:*const net_device)->bool { (*dev).priv_flags & IFF_802_1Q_VLAN != 0 }
pub unsafe fn vlan_dev_priv(dev:*const net_device)->*mut vlan_dev_priv { netdev_priv(dev) as *mut vlan_dev_priv }
pub unsafe fn vlan_dev_get_egress_qos_mask(dev:*mut net_device, skprio:u32)->u16 { let mut p=rcu_dereference((*vlan_dev_priv(dev)).egress_priority_map[(skprio&0xf) as usize]); rcu_read_lock(); let mut q=0; while !p.is_null(){ if (*p).priority==skprio { q=READ_ONCE((*p).vlan_qos); break } p=rcu_dereference((*p).next); } rcu_read_unlock(); q }

#[repr(C)] pub struct vlan_type_depth { pub type_: __be16, pub depth:u16 }
extern "C" { pub fn __vlan_get_protocol_offset(skb:*const sk_buff, ty:__be16, mac_offset:i32)->vlan_type_depth; }

pub unsafe fn eth_type_vlan(ethertype:__be16)->bool { ethertype==htons(ETH_P_8021Q)||ethertype==htons(ETH_P_8021AD) }
pub unsafe fn vlan_hw_offload_capable(features:netdev_features_t, proto:__be16)->bool { (proto==htons(ETH_P_8021Q)&&features&NETIF_F_HW_VLAN_CTAG_TX!=0)||(proto==htons(ETH_P_8021AD)&&features&NETIF_F_HW_VLAN_STAG_TX!=0) }

pub unsafe fn __vlan_hwaccel_clear_tag(skb:*mut sk_buff){(*skb).vlan_all=0}
pub unsafe fn __vlan_hwaccel_copy_tag(dst:*mut sk_buff,src:*const sk_buff){(*dst).vlan_all=(*src).vlan_all}
pub unsafe fn __vlan_hwaccel_put_tag(skb:*mut sk_buff,proto:__be16,tci:u16){(*skb).vlan_proto=proto;(*skb).vlan_tci=tci}

// The remaining inline operations retain their C ABI dependencies and pointer semantics.
extern "C" { pub fn __vlan_insert_inner_tag(skb:*mut sk_buff,proto:__be16,tci:u16,mac_len:u32)->i32; pub fn vlan_insert_inner_tag(skb:*mut sk_buff,proto:__be16,tci:u16,mac_len:u32)->*mut sk_buff; pub fn vlan_insert_tag(skb:*mut sk_buff,proto:__be16,tci:u16)->*mut sk_buff; pub fn vlan_insert_tag_set_proto(skb:*mut sk_buff,proto:__be16,tci:u16)->*mut sk_buff; pub fn __vlan_get_tag(skb:*const sk_buff,tci:*mut u16)->i32; pub fn __vlan_hwaccel_get_tag(skb:*const sk_buff,tci:*mut u16)->i32; pub fn vlan_get_tag(skb:*const sk_buff,tci:*mut u16)->i32; pub fn vlan_get_protocol(skb:*const sk_buff)->__be16; pub fn vlan_get_protocol_and_depth(skb:*mut sk_buff,ty:__be16,depth:*mut i32)->__be16; pub fn skb_protocol(skb:*const sk_buff,skip_vlan:bool)->__be16; pub fn vlan_remove_tag(skb:*mut sk_buff,tci:*mut u16); pub fn skb_vlan_tagged(skb:*const sk_buff)->bool; pub fn skb_vlan_tagged_multi(skb:*mut sk_buff)->bool; pub fn vlan_features_check(skb:*mut sk_buff,features:netdev_features_t)->netdev_features_t; pub fn compare_vlan_header(a:*const vlan_hdr,b:*const vlan_hdr)->u64; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
