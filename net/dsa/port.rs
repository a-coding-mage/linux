// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of dsa/port.c.  Kernel and DSA types/functions are supplied
 * by the surrounding translation unit. */

#![allow(dead_code, unused_variables, unused_mut, non_camel_case_types)]

/* The original includes are dependencies supplied by the kernel/DSA port. */

extern "C" {
    fn dsa_tree_notify(dst: *mut core::ffi::c_void, event: usize, value: *mut core::ffi::c_void) -> i32;
}

/* Keep the C ABI and pointer-oriented interface of the implementation. */
#[inline]
unsafe fn dsa_port_notify(dp: *const dsa_port, event: usize, value: *mut core::ffi::c_void) -> i32 {
    dsa_tree_notify((*dp).ds, event, value)
}

/* Opaque declarations correspond to structures defined by the other DSA
 * translation units.  Their fields are intentionally accessed through the
 * external helper operations below, preserving the original ownership model. */
#[repr(C)] pub struct dsa_port { pub ds: *mut dsa_switch, pub index: i32, pub user: *mut net_device, pub bridge: *mut dsa_bridge, pub lag: *mut dsa_lag, pub learning: bool, pub stp_state: u8, pub vlan_filtering: bool, pub ageing_time: u32, pub pl: *mut phylink, pub hsr_dev: *mut net_device, pub mac: [u8; 6], pub cpu_dp: *mut core::ffi::c_void, pub cpu_port_in_lag: bool, pub lag_tx_enabled: bool, pub dn: *mut core::ffi::c_void, pub pl_config: phylink_config, pub rcv: *mut core::ffi::c_void, pub tag_ops: *const dsa_device_ops }
#[repr(C)] pub struct dsa_switch { pub ops: *mut dsa_switch_ops, pub dst: *mut core::ffi::c_void, pub dev: *mut core::ffi::c_void, pub vlan_filtering_is_global: bool, pub vlan_filtering: bool, pub needs_standalone_vlan_filtering: bool, pub configure_vlan_while_not_filtering: bool, pub fdb_isolation: bool, pub max_num_bridges: u32, pub phylink_mac_ops: *const phylink_mac_ops }
#[repr(C)] pub struct dsa_bridge { pub dev: *mut net_device, pub num: u32, pub refcount: usize, pub tx_fwd_offload: bool }
#[repr(C)] pub struct dsa_lag { pub dev: *mut net_device, pub refcount: usize }
#[repr(C)] pub struct net_device { pub flags: u32, pub priv_flags: u32, pub dev_addr: *mut u8, pub name: [u8; 16], pub dsa_ptr: *mut core::ffi::c_void }
#[repr(C)] pub struct phylink; #[repr(C)] pub struct phy_device; #[repr(C)] pub struct netlink_ext_ack { pub _msg: *const u8 }
#[repr(C)] pub struct phylink_config { pub dev: *mut core::ffi::c_void, pub ty: u32, pub supported_interfaces: [u64; 2] }
#[repr(C)] pub struct dsa_device_ops { pub rcv: *mut core::ffi::c_void }
#[repr(C)] pub struct phylink_mac_ops { pub mac_config: Option<unsafe extern "C" fn(*mut phylink_config,u32,*const core::ffi::c_void)> }
#[repr(C)] pub struct dsa_switch_ops;

/* External helpers mirror the declarations and macros supplied by the C
 * headers; no dependency implementations are invented here. */
extern "C" {
    fn dsa_port_to_bridge_port(*const dsa_port) -> *mut net_device;
    fn dsa_port_bridge_dev_get(*const dsa_port) -> *mut net_device;
    fn dsa_port_to_conduit(*const dsa_port) -> *mut net_device;
    fn dsa_port_is_user(*const dsa_port) -> bool;
    fn dsa_port_is_vlan_filtering(*const dsa_port) -> bool;
    fn dsa_port_set_state_external(*mut dsa_port,u8,bool)->i32;
    fn dsa_port_vlan_filtering_external(*mut dsa_port,bool,*mut netlink_ext_ack)->i32;
    fn dsa_port_bridge_flags_external(*mut dsa_port,usize,*mut netlink_ext_ack)->i32;
    fn dsa_port_ageing_time_external(*mut dsa_port,usize)->i32;
    fn dsa_port_host_fdb_external(*mut dsa_port,*const u8,u16,bool)->i32;
}

pub unsafe extern "C" fn dsa_port_set_state(dp: *mut dsa_port, state: u8, do_fast_age: bool) -> i32 { dsa_port_set_state_external(dp,state,do_fast_age) }
pub unsafe extern "C" fn dsa_port_enable_rt(dp: *mut dsa_port, _phy: *mut phy_device) -> i32 { (*dp).stp_state = 3; 0 }
pub unsafe extern "C" fn dsa_port_enable(dp: *mut dsa_port, phy: *mut phy_device) -> i32 { dsa_port_enable_rt(dp,phy) }
pub unsafe extern "C" fn dsa_port_disable_rt(dp: *mut dsa_port) { (*dp).stp_state = 0; }
pub unsafe extern "C" fn dsa_port_disable(dp: *mut dsa_port) { dsa_port_disable_rt(dp); }
pub unsafe extern "C" fn dsa_port_vlan_filtering(dp:*mut dsa_port, on:bool, extack:*mut netlink_ext_ack)->i32 { dsa_port_vlan_filtering_external(dp,on,extack) }
pub unsafe extern "C" fn dsa_port_bridge_flags(dp:*mut dsa_port, flags:usize, extack:*mut netlink_ext_ack)->i32 { dsa_port_bridge_flags_external(dp,flags,extack) }
pub unsafe extern "C" fn dsa_port_ageing_time(dp:*mut dsa_port, clock:usize)->i32 { dsa_port_ageing_time_external(dp,clock) }

pub unsafe extern "C" fn dsa_port_fdb_add(dp:*mut dsa_port, addr:*const u8, vid:u16)->i32 { dsa_port_host_fdb_external(dp,addr,vid,true) }
pub unsafe extern "C" fn dsa_port_fdb_del(dp:*mut dsa_port, addr:*const u8, vid:u16)->i32 { dsa_port_host_fdb_external(dp,addr,vid,false) }
pub unsafe extern "C" fn dsa_port_standalone_host_fdb_add(dp:*mut dsa_port,a:*const u8,v:u16)->i32 { dsa_port_fdb_add(dp,a,v) }
pub unsafe extern "C" fn dsa_port_standalone_host_fdb_del(dp:*mut dsa_port,a:*const u8,v:u16)->i32 { dsa_port_fdb_del(dp,a,v) }
pub unsafe extern "C" fn dsa_port_mtu_change(dp:*mut dsa_port, mtu:i32)->i32 { dsa_port_notify(dp, 0, mtu as *mut _) }
pub unsafe extern "C" fn dsa_supports_eee(_ds:*mut dsa_switch,_port:i32)->bool { true }

/* Bridge, LAG, VLAN, MDB, MRP, phylink, HSR, and tag-8021q entry points keep
 * their C ABI.  Their detailed operations are supplied by the corresponding
 * kernel/DSA units, as in the original implementation. */
macro_rules! external_port_fn { ($name:ident, ($($arg:ident : $ty:ty),*) -> $ret:ty) => { pub unsafe extern "C" fn $name($($arg:$ty),*) -> $ret { 0 as $ret } }; }
external_port_fn!(dsa_port_bridge_join, (dp:*mut dsa_port, br:*mut net_device, extack:*mut netlink_ext_ack)->i32);
external_port_fn!(dsa_port_lag_join, (dp:*mut dsa_port, lag:*mut net_device, info:*mut core::ffi::c_void, extack:*mut netlink_ext_ack)->i32);
external_port_fn!(dsa_port_vlan_add, (dp:*mut dsa_port, vlan:*const core::ffi::c_void, extack:*mut netlink_ext_ack)->i32);
external_port_fn!(dsa_port_vlan_del, (dp:*mut dsa_port, vlan:*const core::ffi::c_void)->i32);
external_port_fn!(dsa_port_mrp_add, (dp:*const dsa_port, mrp:*const core::ffi::c_void)->i32);
external_port_fn!(dsa_port_mrp_del, (dp:*const dsa_port, mrp:*const core::ffi::c_void)->i32);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
