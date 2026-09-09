/* SPDX-License-Identifier: GPL-2.0-only */
/*
  BNEP protocol definition for Linux Bluetooth stack (BlueZ).
  Copyright (C) 2002 Maxim Krasnyansky <maxk@qualcomm.com>
*/

// C dependencies: linux/types.h, linux/crc32.h, and net/bluetooth/bluetooth.h.

pub const BNEP_MAX_PROTO_FILTERS: usize = 5;
pub const BNEP_MAX_MULTICAST_FILTERS: usize = 20;

pub const BNEP_BASE_UUID: u128 = 0x0000000000001000800000805F9B34FB;
pub const BNEP_UUID16: u8 = 0x02;
pub const BNEP_UUID32: u8 = 0x04;
pub const BNEP_UUID128: u8 = 0x16;

pub const BNEP_SVC_PANU: u16 = 0x1115;
pub const BNEP_SVC_NAP: u16 = 0x1116;
pub const BNEP_SVC_GN: u16 = 0x1117;

pub const BNEP_GENERAL: u8 = 0x00;
pub const BNEP_CONTROL: u8 = 0x01;
pub const BNEP_COMPRESSED: u8 = 0x02;
pub const BNEP_COMPRESSED_SRC_ONLY: u8 = 0x03;
pub const BNEP_COMPRESSED_DST_ONLY: u8 = 0x04;

pub const BNEP_CMD_NOT_UNDERSTOOD: u8 = 0x00;
pub const BNEP_SETUP_CONN_REQ: u8 = 0x01;
pub const BNEP_SETUP_CONN_RSP: u8 = 0x02;
pub const BNEP_FILTER_NET_TYPE_SET: u8 = 0x03;
pub const BNEP_FILTER_NET_TYPE_RSP: u8 = 0x04;
pub const BNEP_FILTER_MULTI_ADDR_SET: u8 = 0x05;
pub const BNEP_FILTER_MULTI_ADDR_RSP: u8 = 0x06;

pub const BNEP_EXT_CONTROL: u8 = 0x00;

pub const BNEP_SUCCESS: u8 = 0x00;
pub const BNEP_CONN_INVALID_DST: u8 = 0x01;
pub const BNEP_CONN_INVALID_SRC: u8 = 0x02;
pub const BNEP_CONN_INVALID_SVC: u8 = 0x03;
pub const BNEP_CONN_NOT_ALLOWED: u8 = 0x04;
pub const BNEP_FILTER_UNSUPPORTED_REQ: u8 = 0x01;
pub const BNEP_FILTER_INVALID_RANGE: u8 = 0x02;
pub const BNEP_FILTER_INVALID_MCADDR: u8 = 0x02;
pub const BNEP_FILTER_LIMIT_REACHED: u8 = 0x03;
pub const BNEP_FILTER_DENIED_SECURITY: u8 = 0x04;

pub const BNEP_MTU: i32 = 1691;
pub const BNEP_PSM: u8 = 0x0f;
pub const BNEP_FLUSH_TO: u16 = 0xffff;
pub const BNEP_CONNECT_TO: i32 = 15;
pub const BNEP_FILTER_TO: i32 = 15;

pub const BNEP_TYPE_MASK: u8 = 0x7f;
pub const BNEP_EXT_HEADER: u8 = 0x80;

#[repr(C, packed)]
pub struct bnep_setup_conn_req {
    pub type_: u8,
    pub ctrl: u8,
    pub uuid_size: u8,
    pub service: [u8; 0],
}

#[repr(C, packed)]
pub struct bnep_set_filter_req {
    pub type_: u8,
    pub ctrl: u8,
    pub len: u16,
    pub list: [u8; 0],
}

#[repr(C, packed)]
pub struct bnep_control_rsp {
    pub type_: u8,
    pub ctrl: u8,
    pub resp: u16,
}

#[repr(C, packed)]
pub struct bnep_ext_hdr {
    pub type_: u8,
    pub len: u8,
    pub data: [u8; 0],
}

// BNEP ioctl definitions use the platform's _IOW/_IOR encoding macros.
// BNEPCONNADD, BNEPCONNDEL, BNEPGETCONNLIST, BNEPGETCONNINFO, and
// BNEPGETSUPPFEAT retain those definitions from the C environment.

pub const BNEP_SETUP_RESPONSE: i32 = 0;
pub const BNEP_SETUP_RSP_SENT: i32 = 10;

#[repr(C)]
pub struct bnep_connadd_req {
    pub sock: i32,
    pub flags: u32,
    pub role: u16,
    pub device: [i8; 16],
}

#[repr(C)]
pub struct bnep_conndel_req {
    pub flags: u32,
    pub dst: [u8; ETH_ALEN],
}

#[repr(C)]
pub struct bnep_conninfo {
    pub flags: u32,
    pub role: u16,
    pub state: u16,
    pub dst: [u8; ETH_ALEN],
    pub device: [i8; 16],
}

#[repr(C)]
pub struct bnep_connlist_req {
    pub cnum: u32,
    pub ci: *mut bnep_conninfo,
}

#[repr(C)]
pub struct bnep_proto_filter {
    pub start: u16,
    pub end: u16,
}

extern "C" {
    pub fn bnep_add_connection(req: *mut bnep_connadd_req, sock: *mut socket) -> i32;
    pub fn bnep_del_connection(req: *mut bnep_conndel_req) -> i32;
    pub fn bnep_get_connlist(req: *mut bnep_connlist_req) -> i32;
    pub fn bnep_get_conninfo(ci: *mut bnep_conninfo) -> i32;
}

#[repr(C)]
pub struct bnep_session {
    pub list: list_head,
    pub role: u32,
    pub state: usize,
    pub flags: usize,
    pub terminate: atomic_t,
    pub task: *mut task_struct,
    pub eh: ethhdr,
    pub msg: msghdr,
    pub proto_filter: [bnep_proto_filter; BNEP_MAX_PROTO_FILTERS],
    pub mc_filter: u64,
    pub sock: *mut socket,
    pub dev: *mut net_device,
}

extern "C" {
    pub fn bnep_net_setup(dev: *mut net_device);
    pub fn bnep_sock_init() -> i32;
    pub fn bnep_sock_cleanup();
    pub fn crc32_be(crc: u32, p: *const u8, len: usize) -> u32;
}

pub unsafe fn bnep_mc_hash(addr: *mut u8) -> i32 {
    (crc32_be(!0u32, addr as *const u8, ETH_ALEN) >> 26) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
