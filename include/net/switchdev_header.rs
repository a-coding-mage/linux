/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from include/net/switchdev.h. External kernel types are dependencies. */

pub const SWITCHDEV_F_NO_RECURSE: u32 = 1 << 0;
pub const SWITCHDEV_F_SKIP_EOPNOTSUPP: u32 = 1 << 1;
pub const SWITCHDEV_F_DEFER: u32 = 1 << 2;
pub const SWITCHDEV_F_NO_FOREIGN: u32 = 1 << 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum switchdev_attr_id { SWITCHDEV_ATTR_ID_UNDEFINED, SWITCHDEV_ATTR_ID_PORT_STP_STATE, SWITCHDEV_ATTR_ID_PORT_MST_STATE, SWITCHDEV_ATTR_ID_PORT_BRIDGE_FLAGS, SWITCHDEV_ATTR_ID_PORT_PRE_BRIDGE_FLAGS, SWITCHDEV_ATTR_ID_PORT_MROUTER, SWITCHDEV_ATTR_ID_BRIDGE_AGEING_TIME, SWITCHDEV_ATTR_ID_BRIDGE_VLAN_FILTERING, SWITCHDEV_ATTR_ID_BRIDGE_VLAN_PROTOCOL, SWITCHDEV_ATTR_ID_BRIDGE_MC_DISABLED, SWITCHDEV_ATTR_ID_BRIDGE_MROUTER, SWITCHDEV_ATTR_ID_BRIDGE_MST, SWITCHDEV_ATTR_ID_MRP_PORT_ROLE, SWITCHDEV_ATTR_ID_VLAN_MSTI }

#[repr(C)] pub struct switchdev_mst_state { pub msti: u16, pub state: u8 }
#[repr(C)] pub struct switchdev_brport_flags { pub val: ::core::ffi::c_ulong, pub mask: ::core::ffi::c_ulong }
#[repr(C)] pub struct switchdev_vlan_msti { pub vid: u16, pub msti: u16 }

#[repr(C)] pub union switchdev_attr_u { pub stp_state: u8, pub mst_state: switchdev_mst_state, pub brport_flags: switchdev_brport_flags, pub mrouter: bool, pub ageing_time: ::core::ffi::c_long, pub vlan_filtering: bool, pub vlan_protocol: u16, pub mst: bool, pub mc_disabled: bool, pub mrp_port_role: u8, pub vlan_msti: switchdev_vlan_msti }
#[repr(C)] pub struct switchdev_attr { pub orig_dev: *mut net_device, pub id: switchdev_attr_id, pub flags: u32, pub complete_priv: *mut ::core::ffi::c_void, pub complete: Option<unsafe extern "C" fn(*mut net_device, i32, *mut ::core::ffi::c_void)>, pub u: switchdev_attr_u }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum switchdev_obj_id { SWITCHDEV_OBJ_ID_UNDEFINED, SWITCHDEV_OBJ_ID_PORT_VLAN, SWITCHDEV_OBJ_ID_PORT_MDB, SWITCHDEV_OBJ_ID_HOST_MDB, SWITCHDEV_OBJ_ID_MRP, SWITCHDEV_OBJ_ID_RING_TEST_MRP, SWITCHDEV_OBJ_ID_RING_ROLE_MRP, SWITCHDEV_OBJ_ID_RING_STATE_MRP, SWITCHDEV_OBJ_ID_IN_TEST_MRP, SWITCHDEV_OBJ_ID_IN_ROLE_MRP, SWITCHDEV_OBJ_ID_IN_STATE_MRP }
#[repr(C)] pub struct switchdev_obj { pub list: list_head, pub orig_dev: *mut net_device, pub id: switchdev_obj_id, pub flags: u32, pub complete_priv: *mut ::core::ffi::c_void, pub complete: Option<unsafe extern "C" fn(*mut net_device, i32, *mut ::core::ffi::c_void)> }
#[repr(C)] pub struct switchdev_obj_port_vlan { pub obj: switchdev_obj, pub flags: u16, pub vid: u16, pub changed: bool }
#[repr(C)] pub struct switchdev_obj_port_mdb { pub obj: switchdev_obj, pub addr: [u8; 6], pub vid: u16 }
#[repr(C)] pub struct switchdev_obj_mrp { pub obj: switchdev_obj, pub p_port: *mut net_device, pub s_port: *mut net_device, pub ring_id: u32, pub prio: u16 }
#[repr(C)] pub struct switchdev_obj_ring_test_mrp { pub obj: switchdev_obj, pub interval: u32, pub max_miss: u8, pub ring_id: u32, pub period: u32, pub monitor: bool }
#[repr(C)] pub struct switchdev_obj_ring_role_mrp { pub obj: switchdev_obj, pub ring_role: u8, pub ring_id: u32, pub sw_backup: u8 }
#[repr(C)] pub struct switchdev_obj_ring_state_mrp { pub obj: switchdev_obj, pub ring_state: u8, pub ring_id: u32 }
#[repr(C)] pub struct switchdev_obj_in_test_mrp { pub obj: switchdev_obj, pub interval: u32, pub in_id: u32, pub period: u32, pub max_miss: u8 }
#[repr(C)] pub struct switchdev_obj_in_role_mrp { pub obj: switchdev_obj, pub i_port: *mut net_device, pub ring_id: u32, pub in_id: u16, pub in_role: u8, pub sw_backup: u8 }
#[repr(C)] pub struct switchdev_obj_in_state_mrp { pub obj: switchdev_obj, pub in_id: u32, pub in_state: u8 }

#[repr(C)] pub struct switchdev_brport { pub dev: *mut net_device, pub ctx: *const ::core::ffi::c_void, pub atomic_nb: *mut notifier_block, pub blocking_nb: *mut notifier_block, pub tx_fwd_offload: bool }
#[repr(C)] #[derive(Copy, Clone)] pub enum switchdev_notifier_type { SWITCHDEV_FDB_ADD_TO_BRIDGE = 1, SWITCHDEV_FDB_DEL_TO_BRIDGE, SWITCHDEV_FDB_ADD_TO_DEVICE, SWITCHDEV_FDB_DEL_TO_DEVICE, SWITCHDEV_FDB_OFFLOADED, SWITCHDEV_FDB_FLUSH_TO_BRIDGE, SWITCHDEV_PORT_OBJ_ADD, SWITCHDEV_PORT_OBJ_DEL, SWITCHDEV_PORT_ATTR_SET, SWITCHDEV_VXLAN_FDB_ADD_TO_BRIDGE, SWITCHDEV_VXLAN_FDB_DEL_TO_BRIDGE, SWITCHDEV_VXLAN_FDB_ADD_TO_DEVICE, SWITCHDEV_VXLAN_FDB_DEL_TO_DEVICE, SWITCHDEV_VXLAN_FDB_OFFLOADED, SWITCHDEV_BRPORT_OFFLOADED, SWITCHDEV_BRPORT_UNOFFLOADED, SWITCHDEV_BRPORT_REPLAY }
#[repr(C)] pub struct switchdev_notifier_info { pub dev: *mut net_device, pub extack: *mut netlink_ext_ack, pub ctx: *const ::core::ffi::c_void }
#[repr(C)] pub struct switchdev_notifier_fdb_info { pub info: switchdev_notifier_info, pub addr: *const u8, pub vid: u16, pub added_by_user: u8, pub is_local: u8, pub locked: u8, pub offloaded: u8 }
#[repr(C)] pub struct switchdev_notifier_port_obj_info { pub info: switchdev_notifier_info, pub obj: *const switchdev_obj, pub handled: bool }
#[repr(C)] pub struct switchdev_notifier_port_attr_info { pub info: switchdev_notifier_info, pub attr: *const switchdev_attr, pub handled: bool }
#[repr(C)] pub struct switchdev_notifier_brport_info { pub info: switchdev_notifier_info, pub brport: switchdev_brport }

#[inline] pub unsafe fn switchdev_notifier_info_to_dev(i: *const switchdev_notifier_info) -> *mut net_device { (*i).dev }
#[inline] pub unsafe fn switchdev_notifier_info_to_extack(i: *const switchdev_notifier_info) -> *mut netlink_ext_ack { (*i).extack }
#[inline] pub unsafe fn switchdev_fdb_is_dynamically_learned(i: *const switchdev_notifier_fdb_info) -> bool { (*i).added_by_user == 0 && (*i).is_local == 0 }

#[cfg(not(CONFIG_NET_SWITCHDEV))]
#[inline] pub unsafe fn switchdev_bridge_port_offload(_: *mut net_device, _: *mut net_device, _: *const ::core::ffi::c_void, _: *mut notifier_block, _: *mut notifier_block, _: bool, _: *mut netlink_ext_ack) -> i32 { -95 }
#[cfg(not(CONFIG_NET_SWITCHDEV))] #[inline] pub unsafe fn switchdev_bridge_port_unoffload(_: *mut net_device, _: *const ::core::ffi::c_void, _: *mut notifier_block, _: *mut notifier_block) {}
#[cfg(not(CONFIG_NET_SWITCHDEV))] #[inline] pub unsafe fn switchdev_deferred_process() {}
#[cfg(not(CONFIG_NET_SWITCHDEV))] #[inline] pub unsafe fn switchdev_port_attr_set(_: *mut net_device, _: *const switchdev_attr, _: *mut netlink_ext_ack) -> i32 { -95 }
#[cfg(not(CONFIG_NET_SWITCHDEV))] #[inline] pub unsafe fn switchdev_port_obj_add(_: *mut net_device, _: *const switchdev_obj, _: *mut netlink_ext_ack) -> i32 { -95 }
#[cfg(not(CONFIG_NET_SWITCHDEV))] #[inline] pub unsafe fn switchdev_port_obj_del(_: *mut net_device, _: *const switchdev_obj) -> i32 { -95 }
#[cfg(not(CONFIG_NET_SWITCHDEV))] #[inline] pub unsafe fn register_switchdev_notifier(_: *mut notifier_block) -> i32 { 0 }
#[cfg(not(CONFIG_NET_SWITCHDEV))] #[inline] pub unsafe fn unregister_switchdev_notifier(_: *mut notifier_block) -> i32 { 0 }
#[cfg(not(CONFIG_NET_SWITCHDEV))] #[inline] pub unsafe fn call_switchdev_notifiers(_: ::core::ffi::c_ulong, _: *mut net_device, _: *mut switchdev_notifier_info, _: *mut netlink_ext_ack) -> i32 { 0 }
#[cfg(not(CONFIG_NET_SWITCHDEV))] #[inline] pub unsafe fn register_switchdev_blocking_notifier(_: *mut notifier_block) -> i32 { 0 }
#[cfg(not(CONFIG_NET_SWITCHDEV))] #[inline] pub unsafe fn unregister_switchdev_blocking_notifier(_: *mut notifier_block) -> i32 { 0 }
#[cfg(not(CONFIG_NET_SWITCHDEV))] #[inline] pub unsafe fn call_switchdev_blocking_notifiers(_: ::core::ffi::c_ulong, _: *mut net_device, _: *mut switchdev_notifier_info, _: *mut netlink_ext_ack) -> i32 { 0 }

/* CONFIG_NET_SWITCHDEV declarations and the enabled implementations are supplied by the kernel translation unit. */
#[cfg(CONFIG_NET_SWITCHDEV)] extern "C" {
    pub fn switchdev_bridge_port_offload(*mut net_device,*mut net_device,*const ::core::ffi::c_void,*mut notifier_block,*mut notifier_block,bool,*mut netlink_ext_ack)->i32;
    pub fn switchdev_bridge_port_unoffload(*mut net_device,*const ::core::ffi::c_void,*mut notifier_block,*mut notifier_block);
    pub fn switchdev_deferred_process();
    pub fn switchdev_port_attr_set(*mut net_device,*const switchdev_attr,*mut netlink_ext_ack)->i32;
    pub fn switchdev_port_obj_add(*mut net_device,*const switchdev_obj,*mut netlink_ext_ack)->i32;
    pub fn switchdev_port_obj_del(*mut net_device,*const switchdev_obj)->i32;
    pub fn register_switchdev_notifier(*mut notifier_block)->i32;
    pub fn unregister_switchdev_notifier(*mut notifier_block)->i32;
    pub fn register_switchdev_blocking_notifier(*mut notifier_block)->i32;
    pub fn unregister_switchdev_blocking_notifier(*mut notifier_block)->i32;
    pub fn call_switchdev_notifiers(::core::ffi::c_ulong,*mut net_device,*mut switchdev_notifier_info,*mut netlink_ext_ack)->i32;
    pub fn call_switchdev_blocking_notifiers(::core::ffi::c_ulong,*mut net_device,*mut switchdev_notifier_info,*mut netlink_ext_ack)->i32;
    pub fn switchdev_handle_fdb_event_to_device(*mut net_device,::core::ffi::c_ulong,*const switchdev_notifier_fdb_info,Option<unsafe extern "C" fn(*const net_device)->bool>,Option<unsafe extern "C" fn(*const net_device,*const net_device)->bool>,Option<unsafe extern "C" fn(*mut net_device,*mut net_device,::core::ffi::c_ulong,*const ::core::ffi::c_void,*const switchdev_notifier_fdb_info)->i32>)->i32;
    pub fn switchdev_handle_port_obj_add(*mut net_device,*mut switchdev_notifier_port_obj_info,Option<unsafe extern "C" fn(*const net_device)->bool>,Option<unsafe extern "C" fn(*mut net_device,*const ::core::ffi::c_void,*const switchdev_obj,*mut netlink_ext_ack)->i32>)->i32;
    pub fn switchdev_handle_port_obj_del(*mut net_device,*mut switchdev_notifier_port_obj_info,Option<unsafe extern "C" fn(*const net_device)->bool>,Option<unsafe extern "C" fn(*mut net_device,*const ::core::ffi::c_void,*const switchdev_obj)->i32>)->i32;
    pub fn switchdev_handle_port_attr_set(*mut net_device,*mut switchdev_notifier_port_attr_info,Option<unsafe extern "C" fn(*const net_device)->bool>,Option<unsafe extern "C" fn(*mut net_device,*const ::core::ffi::c_void,*const switchdev_attr,*mut netlink_ext_ack)->i32>)->i32;
}

#[macro_export] macro_rules! SWITCHDEV_OBJ_PORT_VLAN { ($obj:expr) => { $obj as *mut switchdev_obj_port_vlan }; }
#[macro_export] macro_rules! SWITCHDEV_OBJ_PORT_MDB { ($obj:expr) => { $obj as *mut switchdev_obj_port_mdb }; }
#[macro_export] macro_rules! SWITCHDEV_OBJ_MRP { ($obj:expr) => { $obj as *mut switchdev_obj_mrp }; }
#[macro_export] macro_rules! SWITCHDEV_OBJ_RING_TEST_MRP { ($obj:expr) => { $obj as *mut switchdev_obj_ring_test_mrp }; }
#[macro_export] macro_rules! SWITCHDEV_OBJ_RING_ROLE_MRP { ($obj:expr) => { $obj as *mut switchdev_obj_ring_role_mrp }; }
#[macro_export] macro_rules! SWITCHDEV_OBJ_IN_TEST_MRP { ($obj:expr) => { $obj as *mut switchdev_obj_in_test_mrp }; }
#[macro_export] macro_rules! SWITCHDEV_OBJ_IN_ROLE_MRP { ($obj:expr) => { $obj as *mut switchdev_obj_in_role_mrp }; }
#[macro_export] macro_rules! SWITCHDEV_OBJ_IN_STATE_MRP { ($obj:expr) => { $obj as *mut switchdev_obj_in_state_mrp }; }

/* Future dependency declarations. */
extern "C" { pub type net_device; pub type notifier_block; pub type list_head; pub type netlink_ext_ack; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
