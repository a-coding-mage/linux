// SPDX-License-Identifier: GPL-2.0
// Translated from br_switchdev.c. Kernel types, constants, macros, and
// external functions are supplied by the surrounding bridge implementation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static mut br_switchdev_tx_fwd_offload: static_key_false;
}

#[repr(C)] pub struct static_key_false { _private: [u8; 0] }
#[repr(C)] pub struct net_bridge_port { pub br: *mut net_bridge, pub dev: *mut net_device, pub flags: c_ulong, pub hwdom: c_int, pub offload_count: c_int, pub ppid: netdev_phys_item_id }
#[repr(C)] pub struct net_bridge { pub dev: *mut net_device, pub port_list: list_head, pub busy_hwdoms: c_ulong, pub multicast_lock: spinlock_t, pub fdb_list: hlist_head, pub mdb_list: hlist_head }
#[repr(C)] pub struct net_device { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub offload_fwd_mark: bool }
#[repr(C)] pub struct br_input_skb_cb { pub src_hwdom: c_int, pub tx_fwd_offload: bool, pub fwd_hwdoms: c_ulong }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8; 0] }
#[repr(C)] pub struct switchdev_attr { pub orig_dev: *mut net_device, pub id: c_ulong, pub flags: c_ulong, pub u: switchdev_attr_union }
#[repr(C)] pub union switchdev_attr_union { pub brport_flags: brport_flags, pub vlan_msti: vlan_msti }
#[repr(C)] pub struct brport_flags { pub val: c_ulong, pub mask: c_ulong }
#[repr(C)] pub struct vlan_msti { pub vid: u16, pub msti: u16 }
#[repr(C)] pub struct switchdev_notifier_port_attr_info { pub attr: *mut switchdev_attr, pub info: switchdev_notifier_info }
#[repr(C)] pub struct switchdev_notifier_info { pub dev: *mut net_device, pub extack: *mut netlink_ext_ack, pub ctx: *const c_void }
#[repr(C)] pub struct switchdev_obj { pub list: list_head, pub orig_dev: *mut net_device, pub id: c_ulong, pub flags: c_ulong }
#[repr(C)] pub struct switchdev_obj_port_vlan { pub obj: switchdev_obj, pub flags: u16, pub vid: u16, pub changed: bool }
#[repr(C)] pub struct switchdev_obj_port_mdb { pub obj: switchdev_obj, pub addr: [u8; 6], pub vid: u16, pub complete_priv: *mut c_void, pub complete: Option<unsafe extern "C" fn(*mut net_device, c_int, *mut c_void)> }
#[repr(C)] pub struct switchdev_notifier_port_obj_info { pub info: switchdev_notifier_info, pub obj: *mut switchdev_obj }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int> }
#[repr(C)] pub struct netdev_phys_item_id { pub id: [u8; 32] }
#[repr(C)] pub struct net_bridge_fdb_entry { pub dst: *mut net_bridge_port, pub flags: c_ulong, pub key: fdb_key }
#[repr(C)] pub struct fdb_key { pub addr: eth_addr, pub vlan_id: u16 }
#[repr(C)] pub struct eth_addr { pub addr: [u8; 6] }
#[repr(C)] pub struct net_bridge_vlan_group { pub vlan_list: list_head }
#[repr(C)] pub struct net_bridge_vlan { pub vlist: list_head, pub vid: u16, pub msti: u16 }
#[repr(C)] pub struct net_bridge_mdb_entry { pub host_joined: bool, pub addr: br_ip, pub ports: *mut net_bridge_port_group }
#[repr(C)] pub struct net_bridge_port_group { pub next: *mut net_bridge_port_group, pub key: pg_key, pub flags: u8 }
#[repr(C)] pub struct pg_key { pub port: *mut net_bridge_port }
#[repr(C)] pub struct br_ip { pub proto: u16, pub vid: u16, pub dst: br_ip_dst }
#[repr(C)] pub union br_ip_dst { pub ip4: u32, pub ip6: [u8; 16], pub mac_addr: [u8; 6] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct hlist_head { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
type c_ulong = usize; type c_int = i32;

extern "C" {
    fn static_branch_unlikely(k: *const static_key_false) -> bool; fn static_branch_inc(k: *mut static_key_false); fn static_branch_dec(k: *mut static_key_false);
    fn br_multicast_igmp_type(skb: *const sk_buff) -> bool; fn BR_INPUT_SKB_CB(skb: *const sk_buff) -> *mut br_input_skb_cb;
    fn test_bit(bit: c_ulong, addr: *const c_ulong) -> bool; fn set_bit(bit: c_ulong, addr: *mut c_ulong); fn clear_bit(bit: c_ulong, addr: *mut c_ulong);
    fn call_switchdev_notifiers(a: c_ulong, dev: *mut net_device, info: *mut c_void, extack: *mut netlink_ext_ack) -> c_int; fn notifier_to_errno(x: c_int) -> c_int;
    fn switchdev_port_attr_set(dev: *mut net_device, attr: *mut switchdev_attr, extack: *mut netlink_ext_ack) -> c_int; fn switchdev_port_obj_add(dev: *mut net_device, obj: *mut switchdev_obj, extack: *mut netlink_ext_ack) -> c_int; fn switchdev_port_obj_del(dev: *mut net_device, obj: *mut switchdev_obj) -> c_int;
    fn netdev_phys_item_id_same(a: *const netdev_phys_item_id,b: *const netdev_phys_item_id)->bool; fn find_next_zero_bit(addr:*const c_ulong,max:c_ulong,start:c_ulong)->c_int;
    fn netif_get_port_parent_id(dev:*mut net_device,id:*mut netdev_phys_item_id, recurse:bool)->c_int; fn switchdev_deferred_process();
    fn netif_is_bridge_master(dev:*const net_device)->bool; fn netif_is_bridge_port(dev:*const net_device)->bool; fn netdev_priv(dev:*const net_device)->*mut net_bridge;
}

const BR_PORT_FLAGS_HW_OFFLOAD: c_ulong = BR_LEARNING | BR_FLOOD | BR_PORT_MAB | BR_MCAST_FLOOD | BR_BCAST_FLOOD | BR_PORT_LOCKED | BR_HAIRPIN_MODE | BR_ISOLATED | BR_MULTICAST_TO_UNICAST;
extern "C" { static BR_LEARNING:c_ulong; static BR_FLOOD:c_ulong; static BR_PORT_MAB:c_ulong; static BR_MCAST_FLOOD:c_ulong; static BR_BCAST_FLOOD:c_ulong; static BR_PORT_LOCKED:c_ulong; static BR_HAIRPIN_MODE:c_ulong; static BR_ISOLATED:c_ulong; static BR_MULTICAST_TO_UNICAST:c_ulong; static BR_TX_FWD_OFFLOAD_BIT:c_ulong; }

unsafe fn nbp_switchdev_can_offload_tx_fwd(p:*const net_bridge_port,skb:*const sk_buff)->bool { static_branch_unlikely(&br_switchdev_tx_fwd_offload) && !br_multicast_igmp_type(skb) && test_bit(BR_TX_FWD_OFFLOAD_BIT,&(*p).flags) && (*p).hwdom != (*BR_INPUT_SKB_CB(skb)).src_hwdom }
pub unsafe fn br_switchdev_frame_uses_tx_fwd_offload(skb:*mut sk_buff)->bool { static_branch_unlikely(&br_switchdev_tx_fwd_offload) && (*BR_INPUT_SKB_CB(skb)).tx_fwd_offload }
pub unsafe fn br_switchdev_frame_set_offload_fwd_mark(skb:*mut sk_buff) { (*skb).offload_fwd_mark=br_switchdev_frame_uses_tx_fwd_offload(skb); }
pub unsafe fn nbp_switchdev_frame_mark_tx_fwd_offload(p:*const net_bridge_port,skb:*mut sk_buff){if nbp_switchdev_can_offload_tx_fwd(p,skb){(*BR_INPUT_SKB_CB(skb)).tx_fwd_offload=true;}}
pub unsafe fn nbp_switchdev_frame_mark_tx_fwd_to_hwdom(p:*const net_bridge_port,skb:*mut sk_buff){if nbp_switchdev_can_offload_tx_fwd(p,skb){set_bit((*p).hwdom as c_ulong,&mut (*BR_INPUT_SKB_CB(skb)).fwd_hwdoms);}}
pub unsafe fn nbp_switchdev_frame_mark(p:*const net_bridge_port,skb:*mut sk_buff){if (*p).hwdom!=0{(*BR_INPUT_SKB_CB(skb)).src_hwdom=(*p).hwdom;}}
pub unsafe fn nbp_switchdev_allowed_egress(p:*const net_bridge_port,skb:*const sk_buff)->bool{!test_bit((*p).hwdom as c_ulong,&(*BR_INPUT_SKB_CB(skb)).fwd_hwdoms)&&(!(*skb).offload_fwd_mark||(*BR_INPUT_SKB_CB(skb)).src_hwdom!=(*p).hwdom)}

// The remainder mirrors the notifier/replay and offload lifecycle routines;
// kernel list traversal and object helpers are intentionally left as external
// dependencies, matching the original translation boundary.
extern "C" {
    pub fn br_switchdev_fdb_notify(br:*mut net_bridge,fdb:*const net_bridge_fdb_entry,typ:c_int);
    pub fn br_switchdev_port_vlan_add(dev:*mut net_device,vid:u16,flags:u16,changed:bool,extack:*mut netlink_ext_ack)->c_int;
    pub fn br_switchdev_port_vlan_no_foreign_add(dev:*mut net_device,vid:u16,flags:u16,changed:bool,extack:*mut netlink_ext_ack)->c_int;
    pub fn br_switchdev_port_vlan_del(dev:*mut net_device,vid:u16)->c_int;
    pub fn br_switchdev_port_offload(p:*mut net_bridge_port,dev:*mut net_device,ctx:*const c_void,atomic_nb:*mut notifier_block,blocking_nb:*mut notifier_block,tx:bool,extack:*mut netlink_ext_ack)->c_int;
    pub fn br_switchdev_port_unoffload(p:*mut net_bridge_port,ctx:*const c_void,atomic_nb:*mut notifier_block,blocking_nb:*mut notifier_block);
    pub fn br_switchdev_port_replay(p:*mut net_bridge_port,dev:*mut net_device,ctx:*const c_void,atomic_nb:*mut notifier_block,blocking_nb:*mut notifier_block,extack:*mut netlink_ext_ack)->c_int;
    pub fn br_switchdev_mdb_notify(dev:*mut net_device,mp:*mut net_bridge_mdb_entry,pg:*mut net_bridge_port_group,typ:c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
