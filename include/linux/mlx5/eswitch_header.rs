/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/* Copyright (c) 2018 Mellanox Technologies. All rights reserved. */

// Dependencies supplied by the surrounding mlx5 and devlink headers are
// intentionally referenced but not defined here.

pub const MLX5_ESWITCH_LEGACY: i32 = 0;
pub const MLX5_ESWITCH_OFFLOADS: i32 = 1;

pub const REP_ETH: i32 = 0;
pub const REP_IB: i32 = 1;
pub const NUM_REP_TYPES: usize = 2;

pub const REP_UNREGISTERED: i32 = 0;
pub const REP_REGISTERED: i32 = 1;
pub const REP_LOADED: i32 = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_switchdev_event {
    MLX5_SWITCHDEV_EVENT_PAIR = 0,
    MLX5_SWITCHDEV_EVENT_UNPAIR = 1,
}

#[repr(C)]
pub struct mlx5_core_dev;
#[repr(C)]
pub struct mlx5_eswitch;
#[repr(C)]
pub struct mlx5_flow_handle;
#[repr(C)]
pub struct devlink_eswitch_encap_mode;
#[repr(C)]
pub struct atomic_t;

#[repr(C)]
pub struct mlx5_eswitch_rep_ops {
    pub load: Option<unsafe extern "C" fn(*mut mlx5_core_dev, *mut mlx5_eswitch_rep) -> i32>,
    pub unload: Option<unsafe extern "C" fn(*mut mlx5_eswitch_rep)>,
    pub get_proto_dev: Option<unsafe extern "C" fn(*mut mlx5_eswitch_rep) -> *mut core::ffi::c_void>,
    pub event: Option<unsafe extern "C" fn(*mut mlx5_eswitch, *mut mlx5_eswitch_rep, mlx5_switchdev_event, *mut core::ffi::c_void) -> i32>,
}

#[repr(C)]
pub struct mlx5_eswitch_rep_data {
    pub priv_: *mut core::ffi::c_void,
    pub state: atomic_t,
}

#[repr(C)]
pub struct mlx5_eswitch_rep {
    pub rep_data: [mlx5_eswitch_rep_data; NUM_REP_TYPES],
    pub vport: u16,
    pub vlan: u16,
    /* Only IB rep is using vport_index */
    pub vport_index: u16,
    pub vlan_refcount: u32,
    pub esw: *mut mlx5_eswitch,
}

extern "C" {
    pub fn mlx5_eswitch_register_vport_reps(esw: *mut mlx5_eswitch, ops: *const mlx5_eswitch_rep_ops, rep_type: u8);
    pub fn mlx5_eswitch_register_vport_reps_nested(esw: *mut mlx5_eswitch, ops: *const mlx5_eswitch_rep_ops, rep_type: u8);
    pub fn mlx5_eswitch_unregister_vport_reps(esw: *mut mlx5_eswitch, rep_type: u8);
    pub fn mlx5_eswitch_unregister_vport_reps_nested(esw: *mut mlx5_eswitch, rep_type: u8);
    pub fn mlx5_eswitch_get_proto_dev(esw: *mut mlx5_eswitch, vport_num: u16, rep_type: u8) -> *mut core::ffi::c_void;
    pub fn mlx5_eswitch_vport_rep(esw: *mut mlx5_eswitch, vport_num: u16) -> *mut mlx5_eswitch_rep;
    pub fn mlx5_eswitch_uplink_get_proto_dev(esw: *mut mlx5_eswitch, rep_type: u8) -> *mut core::ffi::c_void;
    pub fn mlx5_eswitch_add_send_to_vport_rule(on_esw: *mut mlx5_eswitch, from_esw: *mut mlx5_eswitch, rep: *mut mlx5_eswitch_rep, sqn: u32) -> *mut mlx5_flow_handle;
}

pub const ESW_VPORT_BITS: u32 = 12;
pub const ESW_PFNUM_BITS: u32 = 4;
pub const ESW_SOURCE_PORT_METADATA_BITS: u32 = ESW_PFNUM_BITS + ESW_VPORT_BITS;
pub const ESW_SOURCE_PORT_METADATA_OFFSET: u32 = 32 - ESW_SOURCE_PORT_METADATA_BITS;
pub const ESW_REG_C0_USER_DATA_METADATA_BITS: u32 = 32 - ESW_SOURCE_PORT_METADATA_BITS;
pub const ESW_REG_C0_USER_DATA_METADATA_MASK: u32 = (1u32 << ESW_REG_C0_USER_DATA_METADATA_BITS) - 1;

#[inline]
pub fn mlx5_eswitch_get_vport_metadata_mask() -> u32 {
    (((1u64 << ESW_SOURCE_PORT_METADATA_BITS) - 1) << (32 - ESW_SOURCE_PORT_METADATA_BITS)) as u32
}

pub const ESW_RESERVED_BITS: u32 = 1;
pub const ESW_ZONE_ID_BITS: u32 = 8;
pub const ESW_TUN_OPTS_BITS: u32 = 11;
pub const ESW_TUN_ID_BITS: u32 = 12;
pub const ESW_TUN_OPTS_OFFSET: u32 = ESW_ZONE_ID_BITS;
pub const ESW_TUN_OFFSET: u32 = ESW_TUN_OPTS_OFFSET;
pub const ESW_ZONE_ID_MASK: u32 = (1u32 << ESW_ZONE_ID_BITS) - 1;
pub const ESW_TUN_OPTS_MASK: u32 = (((1u64 << ESW_TUN_OPTS_BITS) - 1) << ESW_TUN_OPTS_OFFSET) as u32;
pub const ESW_TUN_MASK: u32 = (((1u64 << ESW_TUN_ID_BITS + ESW_TUN_OPTS_BITS) - 1) << ESW_TUN_OFFSET) as u32;
pub const ESW_TUN_ID_SLOW_TABLE_GOTO_VPORT: u32 = 0;
pub const ESW_TUN_ID_BRIDGE_INGRESS_PUSH_VLAN: u32 = ESW_TUN_ID_SLOW_TABLE_GOTO_VPORT;
pub const ESW_TUN_OPTS_SLOW_TABLE_GOTO_VPORT: u32 = (1u32 << ESW_TUN_OPTS_BITS) - 1;
pub const ESW_TUN_SLOW_TABLE_GOTO_VPORT: u32 = (ESW_TUN_ID_SLOW_TABLE_GOTO_VPORT << ESW_TUN_OPTS_BITS) | ESW_TUN_OPTS_SLOW_TABLE_GOTO_VPORT;
pub const ESW_TUN_SLOW_TABLE_GOTO_VPORT_MARK: u32 = ESW_TUN_OPTS_MASK;
pub const ESW_TUN_OPTS_BRIDGE_INGRESS_PUSH_VLAN: u32 = ESW_TUN_OPTS_SLOW_TABLE_GOTO_VPORT - 1;
pub const ESW_TUN_BRIDGE_INGRESS_PUSH_VLAN: u32 = (ESW_TUN_ID_BRIDGE_INGRESS_PUSH_VLAN << ESW_TUN_OPTS_BITS) | ESW_TUN_OPTS_BRIDGE_INGRESS_PUSH_VLAN;
pub const ESW_TUN_BRIDGE_INGRESS_PUSH_VLAN_MARK: u32 = ESW_TUN_OPTS_MASK & !1;
pub const ESW_IPSEC_RX_MAPPED_ID_MASK: u32 = (1u32 << ESW_TUN_OPTS_BITS) - 1;
pub const ESW_IPSEC_RX_MAPPED_ID_MATCH_MASK: u32 = (((1u64 << (ESW_TUN_ID_BITS + ESW_TUN_OPTS_BITS)) - 1) << ESW_ZONE_ID_BITS) as u32;

extern "C" {
    pub fn mlx5_eswitch_mode(dev: *const mlx5_core_dev) -> u8;
    pub fn mlx5_eswitch_get_total_vports(dev: *const mlx5_core_dev) -> u16;
    pub fn mlx5_eswitch_get_core_dev(esw: *mut mlx5_eswitch) -> *mut mlx5_core_dev;
    pub fn mlx5_eswitch_get_encap_mode(dev: *const mlx5_core_dev) -> devlink_eswitch_encap_mode;
    pub fn mlx5_eswitch_reg_c1_loopback_enabled(esw: *const mlx5_eswitch) -> bool;
    pub fn mlx5_eswitch_vport_match_metadata_enabled(esw: *const mlx5_eswitch) -> bool;
    pub fn mlx5_eswitch_get_vport_metadata_for_match(esw: *mut mlx5_eswitch, vport_num: u16) -> u32;
    pub fn mlx5_eswitch_get_vport_metadata_for_set(esw: *mut mlx5_eswitch, vport_num: u16) -> u32;
    pub fn mlx5_core_is_ecpf_esw_manager(dev: *mut mlx5_core_dev) -> bool;
}

#[inline]
pub unsafe fn is_mdev_legacy_mode(dev: *mut mlx5_core_dev) -> bool { mlx5_eswitch_mode(dev) == MLX5_ESWITCH_LEGACY as u8 }
#[inline]
pub unsafe fn is_mdev_switchdev_mode(dev: *mut mlx5_core_dev) -> bool { mlx5_eswitch_mode(dev) == MLX5_ESWITCH_OFFLOADS as u8 }

/* The returned number is valid only when the dev is eswitch manager. */
pub const MLX5_VPORT_ECPF: u16 = 0;
pub const MLX5_VPORT_HOST_PF: u16 = 0;
#[inline]
pub unsafe fn mlx5_eswitch_manager_vport(dev: *mut mlx5_core_dev) -> u16 {
    if mlx5_core_is_ecpf_esw_manager(dev) { MLX5_VPORT_ECPF } else { MLX5_VPORT_HOST_PF }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
