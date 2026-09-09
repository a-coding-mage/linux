/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Translated from switch.h. External types are supplied by dependent headers. */

#[repr(C)]
pub struct DsaNotifierAgeingTimeInfo {
    pub ageing_time: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct DsaNotifierBridgeInfo {
    pub dp: *const DsaPort,
    pub bridge: DsaBridge,
    pub tx_fwd_offload: bool,
    pub extack: *mut NetlinkExtAck,
}

#[repr(C)]
pub struct DsaNotifierFdbInfo {
    pub dp: *const DsaPort,
    pub addr: *const ::core::ffi::c_uchar,
    pub vid: u16,
    pub db: DsaDb,
}

#[repr(C)]
pub struct DsaNotifierLagFdbInfo {
    pub lag: *mut DsaLag,
    pub addr: *const ::core::ffi::c_uchar,
    pub vid: u16,
    pub db: DsaDb,
}

#[repr(C)]
pub struct DsaNotifierMdbInfo {
    pub dp: *const DsaPort,
    pub mdb: *const SwitchdevObjPortMdb,
    pub db: DsaDb,
}

#[repr(C)]
pub struct DsaNotifierLagInfo {
    pub dp: *const DsaPort,
    pub lag: DsaLag,
    pub info: *mut NetdevLagUpperInfo,
    pub extack: *mut NetlinkExtAck,
}

#[repr(C)]
pub struct DsaNotifierVlanInfo {
    pub dp: *const DsaPort,
    pub vlan: *const SwitchdevObjPortVlan,
    pub extack: *mut NetlinkExtAck,
}

#[repr(C)]
pub struct DsaNotifierMtuInfo {
    pub dp: *const DsaPort,
    pub mtu: ::core::ffi::c_int,
}

#[repr(C)]
pub struct DsaNotifierTagProtoInfo {
    pub tag_ops: *const DsaDeviceOps,
}

#[repr(C)]
pub struct DsaNotifierTag8021qVlanInfo {
    pub dp: *const DsaPort,
    pub vid: u16,
}

#[repr(C)]
pub struct DsaNotifierConduitStateInfo {
    pub conduit: *const NetDevice,
    pub operational: bool,
}

pub const DSA_NOTIFIER_AGEING_TIME: ::core::ffi::c_uint = 0;
pub const DSA_NOTIFIER_BRIDGE_JOIN: ::core::ffi::c_uint = 1;
pub const DSA_NOTIFIER_BRIDGE_LEAVE: ::core::ffi::c_uint = 2;
pub const DSA_NOTIFIER_FDB_ADD: ::core::ffi::c_uint = 3;
pub const DSA_NOTIFIER_FDB_DEL: ::core::ffi::c_uint = 4;
pub const DSA_NOTIFIER_HOST_FDB_ADD: ::core::ffi::c_uint = 5;
pub const DSA_NOTIFIER_HOST_FDB_DEL: ::core::ffi::c_uint = 6;
pub const DSA_NOTIFIER_LAG_FDB_ADD: ::core::ffi::c_uint = 7;
pub const DSA_NOTIFIER_LAG_FDB_DEL: ::core::ffi::c_uint = 8;
pub const DSA_NOTIFIER_LAG_CHANGE: ::core::ffi::c_uint = 9;
pub const DSA_NOTIFIER_LAG_JOIN: ::core::ffi::c_uint = 10;
pub const DSA_NOTIFIER_LAG_LEAVE: ::core::ffi::c_uint = 11;
pub const DSA_NOTIFIER_MDB_ADD: ::core::ffi::c_uint = 12;
pub const DSA_NOTIFIER_MDB_DEL: ::core::ffi::c_uint = 13;
pub const DSA_NOTIFIER_HOST_MDB_ADD: ::core::ffi::c_uint = 14;
pub const DSA_NOTIFIER_HOST_MDB_DEL: ::core::ffi::c_uint = 15;
pub const DSA_NOTIFIER_VLAN_ADD: ::core::ffi::c_uint = 16;
pub const DSA_NOTIFIER_VLAN_DEL: ::core::ffi::c_uint = 17;
pub const DSA_NOTIFIER_HOST_VLAN_ADD: ::core::ffi::c_uint = 18;
pub const DSA_NOTIFIER_HOST_VLAN_DEL: ::core::ffi::c_uint = 19;
pub const DSA_NOTIFIER_MTU: ::core::ffi::c_uint = 20;
pub const DSA_NOTIFIER_TAG_PROTO: ::core::ffi::c_uint = 21;
pub const DSA_NOTIFIER_TAG_PROTO_CONNECT: ::core::ffi::c_uint = 22;
pub const DSA_NOTIFIER_TAG_PROTO_DISCONNECT: ::core::ffi::c_uint = 23;
pub const DSA_NOTIFIER_TAG_8021Q_VLAN_ADD: ::core::ffi::c_uint = 24;
pub const DSA_NOTIFIER_TAG_8021Q_VLAN_DEL: ::core::ffi::c_uint = 25;
pub const DSA_NOTIFIER_CONDUIT_STATE_CHANGE: ::core::ffi::c_uint = 26;

extern "C" {
    pub fn dsa_vlan_find(
        vlan_list: *mut ListHead,
        vlan: *const SwitchdevObjPortVlan,
    ) -> *mut DsaVlan;

    pub fn dsa_tree_notify(dst: *mut DsaSwitchTree, e: ::core::ffi::c_ulong, v: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn dsa_broadcast(e: ::core::ffi::c_ulong, v: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;

    pub fn dsa_switch_register_notifier(ds: *mut DsaSwitch) -> ::core::ffi::c_int;
    pub fn dsa_switch_unregister_notifier(ds: *mut DsaSwitch);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
