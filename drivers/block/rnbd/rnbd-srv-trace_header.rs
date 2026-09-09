/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * RDMA Network Block Driver
 *
 * Copyright (c) 2022 1&1 IONOS SE. All rights reserved.
 */

// TRACE_SYSTEM: rnbd_srv
// The Linux tracepoint include and tracepoint declaration DSL are represented
// below as Rust declarations and payload layouts.  The referenced kernel types
// and conversion helpers are supplied by other translation units.

use core::ffi::c_char;

#[repr(C)]
pub struct RnbdSrvSession {
    pub queue_depth: i32,
    pub sessname: *const c_char,
    pub ver: u8,
}

#[repr(C)]
pub struct RtrsSrvOp {
    pub dir: u8,
}

#[repr(C)]
pub struct RnbdMsgIo {
    pub device_id: u32,
    pub sector: u64,
    pub bi_size: u32,
    pub rw: u32,
    pub prio: u16,
}

#[repr(C)]
pub struct RnbdMsgSessInfo {
    pub ver: u8,
}

#[repr(C)]
pub struct RnbdMsgOpen {
    pub access_mode: u8,
    pub dev_name: *const c_char,
}

#[repr(C)]
pub struct RnbdMsgClose {
    pub device_id: u32,
}

// TRACE_DEFINE_ENUM(RNBD_ACCESS_RO);
// TRACE_DEFINE_ENUM(RNBD_ACCESS_RW);
// TRACE_DEFINE_ENUM(RNBD_ACCESS_MIGRATION);
// These constants are provided by the protocol definitions.
extern "C" {
    pub static RNBD_ACCESS_RO: u8;
    pub static RNBD_ACCESS_RW: u8;
    pub static RNBD_ACCESS_MIGRATION: u8;
    pub static RNBD_PROTO_VER_MAJOR: u8;
}

#[repr(C)]
pub struct RnbdSrvLinkTraceEntry {
    pub qdepth: i32,
    pub sessname: *const c_char,
}

#[repr(C)]
pub struct ProcessRdmaTraceEntry {
    pub sessname: *const c_char,
    pub dir: u8,
    pub ver: u8,
    pub device_id: u32,
    pub sector: u64,
    pub flags: u32,
    pub bi_size: u32,
    pub ioprio: u16,
    pub datalen: u32,
    pub usrlen: usize,
}

#[repr(C)]
pub struct ProcessMsgSessInfoTraceEntry {
    pub proto_ver: u8,
    pub clt_ver: u8,
    pub srv_ver: u8,
    pub sessname: *const c_char,
}

#[repr(C)]
pub struct ProcessMsgOpenTraceEntry {
    pub access_mode: u8,
    pub sessname: *const c_char,
    pub dev_name: *const c_char,
}

#[repr(C)]
pub struct ProcessMsgCloseTraceEntry {
    pub device_id: u32,
    pub sessname: *const c_char,
}

// DECLARE_EVENT_CLASS(rnbd_srv_link_class)
// DEFINE_LINK_EVENT(create_sess)
// DEFINE_LINK_EVENT(destroy_sess)
// TRACE_EVENT(process_rdma)
// TRACE_EVENT(process_msg_sess_info)
// TRACE_EVENT(process_msg_open)
// TRACE_EVENT(process_msg_close)
//
// The TP_PROTO, TP_ARGS, TP_fast_assign, and TP_printk bodies are retained
// here as comments because they are Linux tracepoint registration metadata,
// not callable C functions.  Their exact source-level behavior is:
//
// rnbd_srv_link_class:
//   qdepth = srv->queue_depth; sessname = srv->sessname;
// process_rdma:
//   sessname = srv->sessname; dir = id->dir; ver = srv->ver;
//   device_id = le32_to_cpu(msg->device_id); sector = le64_to_cpu(msg->sector);
//   bi_size = le32_to_cpu(msg->bi_size); flags = le32_to_cpu(msg->rw);
//   ioprio = le16_to_cpu(msg->prio); datalen = datalen; usrlen = usrlen;
// process_msg_sess_info:
//   proto_ver = srv->ver; clt_ver = msg->ver;
//   srv_ver = RNBD_PROTO_VER_MAJOR; sessname = srv->sessname;
// process_msg_open:
//   access_mode = msg->access_mode; sessname = srv->sessname;
//   dev_name = msg->dev_name;
// process_msg_close:
//   device_id = le32_to_cpu(msg->device_id); sessname = srv->sessname;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
