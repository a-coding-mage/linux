/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of include/net/dsa.h. Kernel dependency types are external. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const DSA_TAG_PROTO_NONE_VALUE: i32 = 0;
pub const DSA_TAG_PROTO_BRCM_VALUE: i32 = 1;
pub const DSA_TAG_PROTO_BRCM_PREPEND_VALUE: i32 = 2;
pub const DSA_TAG_PROTO_DSA_VALUE: i32 = 3;
pub const DSA_TAG_PROTO_EDSA_VALUE: i32 = 4;
pub const DSA_TAG_PROTO_GSWIP_VALUE: i32 = 5;
pub const DSA_TAG_PROTO_KSZ9477_VALUE: i32 = 6;
pub const DSA_TAG_PROTO_KSZ9893_VALUE: i32 = 7;
pub const DSA_TAG_PROTO_LAN9303_VALUE: i32 = 8;
pub const DSA_TAG_PROTO_MTK_VALUE: i32 = 9;
pub const DSA_TAG_PROTO_QCA_VALUE: i32 = 10;
pub const DSA_TAG_PROTO_TRAILER_VALUE: i32 = 11;
pub const DSA_TAG_PROTO_8021Q_VALUE: i32 = 12;
pub const DSA_TAG_PROTO_SJA1105_VALUE: i32 = 13;
pub const DSA_TAG_PROTO_KSZ8795_VALUE: i32 = 14;
pub const DSA_TAG_PROTO_OCELOT_VALUE: i32 = 15;
pub const DSA_TAG_PROTO_AR9331_VALUE: i32 = 16;
pub const DSA_TAG_PROTO_RTL4_A_VALUE: i32 = 17;
pub const DSA_TAG_PROTO_HELLCREEK_VALUE: i32 = 18;
pub const DSA_TAG_PROTO_XRS700X_VALUE: i32 = 19;
pub const DSA_TAG_PROTO_OCELOT_8021Q_VALUE: i32 = 20;
pub const DSA_TAG_PROTO_SEVILLE_VALUE: i32 = 21;
pub const DSA_TAG_PROTO_BRCM_LEGACY_VALUE: i32 = 22;
pub const DSA_TAG_PROTO_SJA1110_VALUE: i32 = 23;
pub const DSA_TAG_PROTO_RTL8_4_VALUE: i32 = 24;
pub const DSA_TAG_PROTO_RTL8_4T_VALUE: i32 = 25;
pub const DSA_TAG_PROTO_RZN1_A5PSW_VALUE: i32 = 26;
pub const DSA_TAG_PROTO_LAN937X_VALUE: i32 = 27;
pub const DSA_TAG_PROTO_VSC73XX_8021Q_VALUE: i32 = 28;
pub const DSA_TAG_PROTO_BRCM_LEGACY_FCS_VALUE: i32 = 29;
pub const DSA_TAG_PROTO_YT921X_VALUE: i32 = 30;
pub const DSA_TAG_PROTO_MXL_GSW1XX_VALUE: i32 = 31;
pub const DSA_TAG_PROTO_MXL862_VALUE: i32 = 32;
pub const DSA_TAG_PROTO_NETC_VALUE: i32 = 33;
pub const DSA_TAG_PROTO_KSZ8463_VALUE: i32 = 34;
pub const DSA_TAG_PROTO_MT7628_VALUE: i32 = 35;

pub type dsa_tag_protocol = i32;
pub const DSA_TAG_PROTO_NONE: dsa_tag_protocol = 0;
pub const DSA_TAG_PROTO_BRCM: dsa_tag_protocol = 1;
pub const DSA_TAG_PROTO_BRCM_LEGACY: dsa_tag_protocol = 22;
pub const DSA_TAG_PROTO_BRCM_LEGACY_FCS: dsa_tag_protocol = 29;
pub const DSA_TAG_PROTO_BRCM_PREPEND: dsa_tag_protocol = 2;
pub const DSA_TAG_PROTO_DSA: dsa_tag_protocol = 3;
pub const DSA_TAG_PROTO_EDSA: dsa_tag_protocol = 4;
pub const DSA_TAG_PROTO_GSWIP: dsa_tag_protocol = 5;
pub const DSA_TAG_PROTO_KSZ9477: dsa_tag_protocol = 6;
pub const DSA_TAG_PROTO_KSZ9893: dsa_tag_protocol = 7;
pub const DSA_TAG_PROTO_LAN9303: dsa_tag_protocol = 8;
pub const DSA_TAG_PROTO_MTK: dsa_tag_protocol = 9;
pub const DSA_TAG_PROTO_QCA: dsa_tag_protocol = 10;
pub const DSA_TAG_PROTO_TRAILER: dsa_tag_protocol = 11;
pub const DSA_TAG_PROTO_8021Q: dsa_tag_protocol = 12;
pub const DSA_TAG_PROTO_SJA1105: dsa_tag_protocol = 13;
pub const DSA_TAG_PROTO_KSZ8795: dsa_tag_protocol = 14;
pub const DSA_TAG_PROTO_OCELOT: dsa_tag_protocol = 15;
pub const DSA_TAG_PROTO_AR9331: dsa_tag_protocol = 16;
pub const DSA_TAG_PROTO_RTL4_A: dsa_tag_protocol = 17;
pub const DSA_TAG_PROTO_HELLCREEK: dsa_tag_protocol = 18;
pub const DSA_TAG_PROTO_XRS700X: dsa_tag_protocol = 19;
pub const DSA_TAG_PROTO_OCELOT_8021Q: dsa_tag_protocol = 20;
pub const DSA_TAG_PROTO_SEVILLE: dsa_tag_protocol = 21;
pub const DSA_TAG_PROTO_SJA1110: dsa_tag_protocol = 23;
pub const DSA_TAG_PROTO_RTL8_4: dsa_tag_protocol = 24;
pub const DSA_TAG_PROTO_RTL8_4T: dsa_tag_protocol = 25;
pub const DSA_TAG_PROTO_RZN1_A5PSW: dsa_tag_protocol = 26;
pub const DSA_TAG_PROTO_LAN937X: dsa_tag_protocol = 27;
pub const DSA_TAG_PROTO_VSC73XX_8021Q: dsa_tag_protocol = 28;
pub const DSA_TAG_PROTO_YT921X: dsa_tag_protocol = 30;
pub const DSA_TAG_PROTO_MXL_GSW1XX: dsa_tag_protocol = 31;
pub const DSA_TAG_PROTO_MXL862: dsa_tag_protocol = 32;
pub const DSA_TAG_PROTO_NETC: dsa_tag_protocol = 33;
pub const DSA_TAG_PROTO_KSZ8463: dsa_tag_protocol = 34;
pub const DSA_TAG_PROTO_MT7628: dsa_tag_protocol = 35;

extern "C" {
    pub fn dsa_unregister_switch(ds: *mut dsa_switch);
    pub fn dsa_register_switch(ds: *mut dsa_switch) -> i32;
    pub fn dsa_switch_shutdown(ds: *mut dsa_switch);
    pub fn dsa_switch_find(tree_index: i32, sw_index: i32) -> *mut dsa_switch;
    pub fn dsa_flush_workqueue();
    pub fn dsa_devlink_param_get(dl: *mut devlink, id: u32, ctx: *mut devlink_param_gset_ctx, extack: *mut netlink_ext_ack) -> i32;
    pub fn dsa_devlink_param_set(dl: *mut devlink, id: u32, ctx: *mut devlink_param_gset_ctx, extack: *mut netlink_ext_ack) -> i32;
}

#[repr(C)] pub struct dsa_switch { pub dev: *mut device, pub dst: *mut dsa_switch_tree, pub index: u32, pub setup: u32, pub priv_: *mut core::ffi::c_void, pub ops: *const dsa_switch_ops, pub num_ports: u32 }
#[repr(C)] pub struct dsa_switch_tree { pub list: list_head, pub ports: list_head, pub index: u32, pub lags: *mut *mut dsa_lag, pub tag_ops: *const dsa_device_ops, pub default_proto: dsa_tag_protocol, pub setup: bool, pub rtable: list_head, pub lags_len: u32, pub last_switch: u32 }
#[repr(C)] pub struct dsa_device_ops { pub xmit: Option<unsafe extern "C" fn(*mut sk_buff,*mut net_device)->*mut sk_buff>, pub rcv: Option<unsafe extern "C" fn(*mut sk_buff,*mut net_device)->*mut sk_buff>, pub needed_headroom: u32, pub needed_tailroom: u32, pub name: *const i8, pub proto: dsa_tag_protocol, pub promisc_on_conduit: bool }
#[repr(C)] pub struct dsa_lag { pub dev: *mut net_device, pub id: u32, pub fdb_lock: mutex, pub fdbs: list_head, pub refcount: refcount_t }
#[repr(C)] pub struct dsa_bridge { pub dev: *mut net_device, pub num: u32, pub tx_fwd_offload: bool, pub refcount: refcount_t }
#[repr(C)] pub struct dsa_link { pub dp: *mut dsa_port, pub link_dp: *mut dsa_port, pub list: list_head }
#[repr(C)] pub struct dsa_port { pub conduit: *mut net_device, pub tag_ops: *const dsa_device_ops, pub dst: *mut dsa_switch_tree, pub rcv: Option<unsafe extern "C" fn(*mut sk_buff,*mut net_device)->*mut sk_buff>, pub ds: *mut dsa_switch, pub index: u32, pub r#type: i32, pub name: *const i8, pub cpu_dp: *mut dsa_port, pub mac: [u8; 6], pub stp_state: u8, pub vlan_filtering: u8, pub learning: u8, pub lag_tx_enabled: u8, pub conduit_admin_up: u8, pub conduit_oper_up: u8, pub cpu_port_in_lag: u8, pub setup: u8, pub dn: *mut device_node, pub ageing_time: u32, pub bridge: *mut dsa_bridge, pub lag: *mut dsa_lag, pub hsr_dev: *mut net_device, pub list: list_head, pub fdbs: list_head, pub mdbs: list_head, pub vlans: list_head }
pub const DSA_PORT_TYPE_UNUSED: i32=0; pub const DSA_PORT_TYPE_CPU:i32=1; pub const DSA_PORT_TYPE_DSA:i32=2; pub const DSA_PORT_TYPE_USER:i32=3;
#[repr(C)] pub struct dsa_db { pub r#type: i32, pub dp: *const dsa_port }
pub const DSA_DB_PORT:i32=0; pub const DSA_DB_LAG:i32=1; pub const DSA_DB_BRIDGE:i32=2;
#[repr(C)] pub struct dsa_mall_mirror_tc_entry { pub to_local_port:u8, pub ingress:bool }
#[repr(C)] pub struct dsa_mall_tc_entry { pub list:list_head, pub cookie:usize, pub r#type:i32, pub mirror:dsa_mall_mirror_tc_entry }
#[repr(C)] pub struct dsa_switch_ops { pub get_tag_protocol: Option<unsafe extern "C" fn(*mut dsa_switch,i32,dsa_tag_protocol)->dsa_tag_protocol>, pub change_tag_protocol: Option<unsafe extern "C" fn(*mut dsa_switch,dsa_tag_protocol)->i32>, pub setup: Option<unsafe extern "C" fn(*mut dsa_switch)->i32>, pub teardown: Option<unsafe extern "C" fn(*mut dsa_switch)>, pub port_setup: Option<unsafe extern "C" fn(*mut dsa_switch,i32)->i32>, pub port_teardown: Option<unsafe extern "C" fn(*mut dsa_switch,i32)>, pub port_enable: Option<unsafe extern "C" fn(*mut dsa_switch,i32,*mut phy_device)->i32>, pub port_disable: Option<unsafe extern "C" fn(*mut dsa_switch,i32)> }

pub type dsa_fdb_dump_cb_t = unsafe extern "C" fn(*const u8,u16,bool,*mut core::ffi::c_void)->i32;
pub const ETH_ALEN: usize = 6;
pub enum device{} pub enum net_device{} pub enum sk_buff{} pub enum device_node{} pub enum phy_device{} pub enum devlink{} pub enum devlink_param_gset_ctx{} pub enum netlink_ext_ack{}
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct mutex { _private: [u8;0] } #[repr(C)] pub struct refcount_t { pub refs:i32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
