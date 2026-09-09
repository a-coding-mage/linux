/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Translation of uapi/linux/tipc_config.h. */

use core::ffi::c_void;

pub const TIPC_CMD_NOOP: u16 = 0x0000;
pub const TIPC_CMD_GET_NODES: u16 = 0x0001;
pub const TIPC_CMD_GET_MEDIA_NAMES: u16 = 0x0002;
pub const TIPC_CMD_GET_BEARER_NAMES: u16 = 0x0003;
pub const TIPC_CMD_GET_LINKS: u16 = 0x0004;
pub const TIPC_CMD_SHOW_NAME_TABLE: u16 = 0x0005;
pub const TIPC_CMD_SHOW_PORTS: u16 = 0x0006;
pub const TIPC_CMD_SHOW_LINK_STATS: u16 = 0x000B;
pub const TIPC_CMD_SHOW_STATS: u16 = 0x000F;
pub const TIPC_CMD_GET_REMOTE_MNG: u16 = 0x4003;
pub const TIPC_CMD_GET_MAX_PORTS: u16 = 0x4004;
pub const TIPC_CMD_GET_MAX_PUBL: u16 = 0x4005;
pub const TIPC_CMD_GET_MAX_SUBSCR: u16 = 0x4006;
pub const TIPC_CMD_GET_MAX_ZONES: u16 = 0x4007;
pub const TIPC_CMD_GET_MAX_CLUSTERS: u16 = 0x4008;
pub const TIPC_CMD_GET_MAX_NODES: u16 = 0x4009;
pub const TIPC_CMD_GET_MAX_SLAVES: u16 = 0x400A;
pub const TIPC_CMD_GET_NETID: u16 = 0x400B;
pub const TIPC_CMD_ENABLE_BEARER: u16 = 0x4101;
pub const TIPC_CMD_DISABLE_BEARER: u16 = 0x4102;
pub const TIPC_CMD_SET_LINK_TOL: u16 = 0x4107;
pub const TIPC_CMD_SET_LINK_PRI: u16 = 0x4108;
pub const TIPC_CMD_SET_LINK_WINDOW: u16 = 0x4109;
pub const TIPC_CMD_SET_LOG_SIZE: u16 = 0x410A;
pub const TIPC_CMD_DUMP_LOG: u16 = 0x410B;
pub const TIPC_CMD_RESET_LINK_STATS: u16 = 0x410C;
pub const TIPC_CMD_SET_NODE_ADDR: u16 = 0x8001;
pub const TIPC_CMD_SET_REMOTE_MNG: u16 = 0x8003;
pub const TIPC_CMD_SET_MAX_PORTS: u16 = 0x8004;
pub const TIPC_CMD_SET_MAX_PUBL: u16 = 0x8005;
pub const TIPC_CMD_SET_MAX_SUBSCR: u16 = 0x8006;
pub const TIPC_CMD_SET_MAX_ZONES: u16 = 0x8007;
pub const TIPC_CMD_SET_MAX_CLUSTERS: u16 = 0x8008;
pub const TIPC_CMD_SET_MAX_NODES: u16 = 0x8009;
pub const TIPC_CMD_SET_MAX_SLAVES: u16 = 0x800A;
pub const TIPC_CMD_SET_NETID: u16 = 0x800B;
pub const TIPC_CMD_NOT_NET_ADMIN: u16 = 0xC001;

pub const TIPC_TLV_NONE: u16 = 0; pub const TIPC_TLV_VOID: u16 = 1;
pub const TIPC_TLV_UNSIGNED: u16 = 2; pub const TIPC_TLV_STRING: u16 = 3;
pub const TIPC_TLV_LARGE_STRING: u16 = 4; pub const TIPC_TLV_ULTRA_STRING: u16 = 5;
pub const TIPC_TLV_ERROR_STRING: u16 = 16; pub const TIPC_TLV_NET_ADDR: u16 = 17;
pub const TIPC_TLV_MEDIA_NAME: u16 = 18; pub const TIPC_TLV_BEARER_NAME: u16 = 19;
pub const TIPC_TLV_LINK_NAME: u16 = 20; pub const TIPC_TLV_NODE_INFO: u16 = 21;
pub const TIPC_TLV_LINK_INFO: u16 = 22; pub const TIPC_TLV_BEARER_CONFIG: u16 = 23;
pub const TIPC_TLV_LINK_CONFIG: u16 = 24; pub const TIPC_TLV_NAME_TBL_QUERY: u16 = 25;
pub const TIPC_TLV_PORT_REF: u16 = 26;
pub const TIPC_MIN_LINK_PRI: u32 = 0; pub const TIPC_DEF_LINK_PRI: u32 = 10;
pub const TIPC_MAX_LINK_PRI: u32 = 31; pub const TIPC_MEDIA_LINK_PRI: u32 = TIPC_MAX_LINK_PRI + 1;
pub const TIPC_MIN_LINK_TOL: u32 = 50; pub const TIPC_DEF_LINK_TOL: u32 = 1500; pub const TIPC_MAX_LINK_TOL: u32 = 30000;
pub const TIPC_MIN_LINK_WIN: u32 = 16; pub const TIPC_DEF_LINK_WIN: u32 = 50; pub const TIPC_MAX_LINK_WIN: u32 = 8191;
pub const TIPC_DEF_LINK_UDP_MTU: u32 = 14000;

pub const TIPC_NTQ_ALLTYPES: u32 = 0x80000000;
pub const TIPC_CFG_TLV_ERROR: &[u8] = b"\x80\0";
pub const TIPC_CFG_NOT_NET_ADMIN: u8 = 0x81; pub const TIPC_CFG_NOT_ZONE_MSTR: u8 = 0x82;
pub const TIPC_CFG_NO_REMOTE: u8 = 0x83; pub const TIPC_CFG_NOT_SUPPORTED: u8 = 0x84; pub const TIPC_CFG_INVALID_VALUE: u8 = 0x85;

#[repr(C)]
pub struct tipc_node_info { pub addr: u32, pub up: u32 }
#[repr(C)]
pub struct tipc_link_info { pub dest: u32, pub up: u32, pub str_: [u8; TIPC_MAX_LINK_NAME as usize] }
#[repr(C)]
pub struct tipc_bearer_config { pub priority: u32, pub disc_domain: u32, pub name: [u8; TIPC_MAX_BEARER_NAME as usize] }
#[repr(C)]
pub struct tipc_link_config { pub value: u32, pub name: [u8; TIPC_MAX_LINK_NAME as usize] }
#[repr(C)]
pub struct tipc_name_table_query { pub depth: u32, pub type_: u32, pub lowbound: u32, pub upbound: u32 }
#[repr(C)]
pub struct tlv_desc { pub tlv_len: u16, pub tlv_type: u16 }

pub const TLV_ALIGNTO: usize = 4;
pub const fn tlv_align(d: usize) -> usize { (d + 3) & !3 }
pub const fn tlv_length(d: usize) -> usize { core::mem::size_of::<tlv_desc>() + d }
pub const fn tlv_space(d: usize) -> usize { tlv_align(tlv_length(d)) }
pub unsafe fn tlv_data(tlv: *mut tlv_desc) -> *mut c_void { (tlv as *mut u8).add(tlv_length(0)) as *mut c_void }
pub unsafe fn tlv_ok(tlv: *const c_void, space: u16) -> i32 { ((space as usize >= tlv_space(0)) && u16::from_be((* (tlv as *const tlv_desc)).tlv_len) as usize <= space as usize) as i32 }
pub unsafe fn tlv_check(tlv: *const c_void, space: u16, exp_type: u16) -> i32 { (tlv_ok(tlv, space) != 0 && u16::from_be((* (tlv as *const tlv_desc)).tlv_type) == exp_type) as i32 }
pub unsafe fn tlv_get_len(tlv: *const tlv_desc) -> i32 { u16::from_be((*tlv).tlv_len) as i32 }
pub unsafe fn tlv_set_len(tlv: *mut tlv_desc, len: u16) { (*tlv).tlv_len = len.to_be(); }
pub unsafe fn tlv_check_type(tlv: *const tlv_desc, ty: u16) -> i32 { (u16::from_be((*tlv).tlv_type) == ty) as i32 }
pub unsafe fn tlv_set_type(tlv: *mut tlv_desc, ty: u16) { (*tlv).tlv_type = ty.to_be(); }
pub unsafe fn tlv_set(tlv: *mut c_void, ty: u16, data: *const c_void, len: u16) -> i32 { let n = tlv_length(len as usize); let p = tlv as *mut tlv_desc; (*p).tlv_type=ty.to_be(); (*p).tlv_len=(n as u16).to_be(); if len != 0 && !data.is_null() { core::ptr::copy_nonoverlapping(data as *const u8, tlv_data(p) as *mut u8, len as usize); core::ptr::write_bytes((tlv_data(p) as *mut u8).add(len as usize), 0, tlv_space(len as usize)-n); } tlv_space(len as usize) as i32 }

#[repr(C)] pub struct tlv_list_desc { pub tlv_ptr: *mut tlv_desc, pub tlv_space: u32 }
pub unsafe fn tlv_list_init(l: *mut tlv_list_desc, data: *mut c_void, space: u32) { (*l).tlv_ptr=data as *mut tlv_desc; (*l).tlv_space=space; }
pub unsafe fn tlv_list_empty(l: *const tlv_list_desc) -> i32 { ((*l).tlv_space == 0) as i32 }
pub unsafe fn tlv_list_check(l: *const tlv_list_desc, ty: u16) -> i32 { tlv_check((*l).tlv_ptr as *const c_void, (*l).tlv_space as u16, ty) }
pub unsafe fn tlv_list_data(l: *const tlv_list_desc) -> *mut c_void { tlv_data((*l).tlv_ptr) }
pub unsafe fn tlv_list_step(l: *mut tlv_list_desc) { let n=tlv_align(u16::from_be((*(*l).tlv_ptr).tlv_len) as usize); (*l).tlv_ptr=((*l).tlv_ptr as *mut u8).add(n) as *mut tlv_desc; (*l).tlv_space-=n as u32; }

pub const TIPC_GENL_NAME: &[u8] = b"TIPC\0"; pub const TIPC_GENL_VERSION: u16 = 1; pub const TIPC_GENL_CMD: u8 = 1;
#[repr(C)] pub struct tipc_genlmsghdr { pub dest: u32, pub cmd: u16, pub reserved: u16 }
#[repr(C)] pub struct tipc_cfg_msg_hdr { pub tcm_len: u32, pub tcm_type: u16, pub tcm_flags: u16, pub tcm_reserved: [u8;8] }
pub const TCM_F_REQUEST: u16=1; pub const TCM_F_MORE: u16=2;
pub const fn tcm_align(d: usize)->usize {(d+3)&!3} pub const fn tcm_length(d: usize)->usize {core::mem::size_of::<tipc_cfg_msg_hdr>()+d} pub const fn tcm_space(d: usize)->usize {tcm_align(tcm_length(d))}
pub unsafe fn tcm_data(h: *mut tipc_cfg_msg_hdr)->*mut c_void {(h as *mut u8).add(tcm_length(0)) as *mut c_void}
pub unsafe fn tcm_set(msg:*mut c_void, cmd:u16, flags:u16, data:*const c_void, len:u16)->i32 {let n=tcm_length(len as usize);let h=msg as *mut tipc_cfg_msg_hdr;(*h).tcm_len=(n as u32).to_be();(*h).tcm_type=cmd.to_be();(*h).tcm_flags=flags.to_be();if len!=0&&!data.is_null(){core::ptr::copy_nonoverlapping(data as *const u8,tcm_data(h) as *mut u8,len as usize);core::ptr::write_bytes((tcm_data(h) as *mut u8).add(len as usize),0,tcm_space(len as usize)-n);}tcm_space(len as usize) as i32}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
