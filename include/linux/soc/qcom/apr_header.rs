/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */

extern "C" {
    pub static aprbus: bus_type;
}

pub const fn APR_HDR_LEN(hdr_len: usize) -> usize { hdr_len / 4 }

/*
 * HEADER field
 * version:0:3
 * header_size : 4:7
 * message_type : 8:9
 * reserved: 10:15
 */
pub const fn APR_HDR_FIELD(msg_type: u32, hdr_len: u32, ver: u32) -> u32 {
    ((msg_type & 0x3) << 8) | ((hdr_len & 0xF) << 4) | (ver & 0xF)
}

/* Version */
pub const APR_PKT_VER: u32 = 0x0;

/* Command and Response Types */
pub const APR_MSG_TYPE_EVENT: u32 = 0x0;
pub const APR_MSG_TYPE_CMD_RSP: u32 = 0x1;
pub const APR_MSG_TYPE_SEQ_CMD: u32 = 0x2;
pub const APR_MSG_TYPE_NSEQ_CMD: u32 = 0x3;
pub const APR_MSG_TYPE_MAX: u32 = 0x04;

/* APR Basic Response Message */
pub const APR_BASIC_RSP_RESULT: u32 = 0x000110E8;
pub const APR_RSP_ACCEPTED: u32 = 0x000100BE;

#[repr(C)]
pub struct aprv2_ibasic_rsp_result_t {
    pub opcode: u32,
    pub status: u32,
}

/* hdr field Ver [0:3], Size [4:7], Message type [8:10] */
pub const fn APR_HDR_FIELD_VER(h: u16) -> u16 { h & 0x000F }
pub const fn APR_HDR_FIELD_SIZE(h: u16) -> u16 { (h & 0x00F0) >> 4 }
pub const fn APR_HDR_FIELD_SIZE_BYTES(h: u16) -> u16 { ((h & 0x00F0) >> 4) * 4 }
pub const fn APR_HDR_FIELD_MT(h: u16) -> u16 { (h & 0x0300) >> 8 }

#[repr(C, packed)]
pub struct apr_hdr {
    pub hdr_field: u16,
    pub pkt_size: u16,
    pub src_svc: u8,
    pub src_domain: u8,
    pub src_port: u16,
    pub dest_svc: u8,
    pub dest_domain: u8,
    pub dest_port: u16,
    pub token: u32,
    pub opcode: u32,
}

pub const APR_HDR_SIZE: usize = core::mem::size_of::<apr_hdr>();
pub const APR_SEQ_CMD_HDR_FIELD: u32 = APR_HDR_FIELD(
    APR_MSG_TYPE_SEQ_CMD,
    APR_HDR_LEN(APR_HDR_SIZE) as u32,
    APR_PKT_VER,
);

#[repr(C)]
pub struct apr_pkt {
    pub hdr: apr_hdr,
    pub payload: [u8; 0],
}

#[repr(C)]
pub struct apr_resp_pkt {
    pub hdr: apr_hdr,
    pub payload: *mut core::ffi::c_void,
    pub payload_size: i32,
}

#[repr(C, packed)]
pub struct gpr_hdr {
    /* C bit-fields: version:4, hdr_size:4, pkt_size:24. */
    pub version_hdr_size_pkt_size: u32,
    /* C bit-fields: dest_domain:8, src_domain:8, reserved:16. */
    pub dest_src_domain_reserved: u32,
    pub src_port: u32,
    pub dest_port: u32,
    pub token: u32,
    pub opcode: u32,
}

#[repr(C)]
pub struct gpr_pkt {
    pub hdr: gpr_hdr,
    pub payload: [u32; 0],
}

#[repr(C)]
pub struct gpr_resp_pkt {
    pub hdr: gpr_hdr,
    pub payload: *mut core::ffi::c_void,
    pub payload_size: i32,
}

pub const GPR_HDR_SIZE: usize = core::mem::size_of::<gpr_hdr>();
pub const GPR_PKT_VER: u32 = 0x0;
pub const GPR_PKT_HEADER_WORD_SIZE: usize = (core::mem::size_of::<gpr_pkt>() + 3) >> 2;
pub const GPR_PKT_HEADER_BYTE_SIZE: usize = GPR_PKT_HEADER_WORD_SIZE << 2;
pub const GPR_BASIC_RSP_RESULT: u32 = 0x02001005;

#[repr(C)]
pub struct gpr_ibasic_rsp_result_t {
    pub opcode: u32,
    pub status: u32,
}

pub const GPR_BASIC_EVT_ACCEPTED: u32 = 0x02001006;

#[repr(C)]
pub struct gpr_ibasic_rsp_accepted_t {
    pub opcode: u32,
}

/* Bits 0 to 15 -- Minor version, Bits 16 to 31 -- Major version */
pub const fn APR_SVC_MAJOR_VERSION(v: u32) -> u32 { (v >> 16) & 0xFF }
pub const fn APR_SVC_MINOR_VERSION(v: u32) -> u32 { v & 0xFF }

pub type gpr_port_cb = unsafe extern "C" fn(*const gpr_resp_pkt, *mut core::ffi::c_void, i32) -> i32;

pub enum packet_router {}

#[repr(C)]
pub struct pkt_router_svc {
    pub dev: *mut device,
    pub callback: Option<gpr_port_cb>,
    pub pr: *mut packet_router,
    pub lock: spinlock_t,
    pub id: i32,
    pub priv_: *mut core::ffi::c_void,
}

pub type gpr_port_t = pkt_router_svc;

#[repr(C)]
pub struct apr_device {
    pub dev: device,
    pub svc_id: u16,
    pub domain_id: u16,
    pub version: u32,
    pub name: [core::ffi::c_char; APR_NAME_SIZE],
    pub service_path: *const core::ffi::c_char,
    pub svc: pkt_router_svc,
    pub node: list_head,
}

pub type gpr_device_t = apr_device;

pub type apr_probe_fn = unsafe extern "C" fn(*mut apr_device) -> i32;
pub type apr_remove_fn = unsafe extern "C" fn(*mut apr_device);
pub type apr_callback_fn = unsafe extern "C" fn(*mut apr_device, *const apr_resp_pkt) -> i32;

#[repr(C)]
pub struct apr_driver {
    pub probe: Option<apr_probe_fn>,
    pub remove: Option<apr_remove_fn>,
    pub callback: Option<apr_callback_fn>,
    pub gpr_callback: Option<gpr_port_cb>,
    pub driver: device_driver,
    pub id_table: *const apr_device_id,
}

pub type gpr_driver_t = apr_driver;

extern "C" {
    pub fn __apr_driver_register(drv: *mut apr_driver, owner: *mut module) -> i32;
    pub fn apr_driver_unregister(drv: *mut apr_driver);
    pub fn apr_send_pkt(adev: *mut apr_device, pkt: *mut apr_pkt) -> i32;
    pub fn gpr_alloc_port(gdev: *mut gpr_device_t, dev: *mut device, cb: Option<gpr_port_cb>, priv_: *mut core::ffi::c_void) -> *mut gpr_port_t;
    pub fn gpr_free_port(port: *mut gpr_port_t);
    pub fn gpr_send_port_pkt(port: *mut gpr_port_t, pkt: *const gpr_pkt) -> i32;
    pub fn gpr_send_pkt(gdev: *mut gpr_device_t, pkt: *const gpr_pkt) -> i32;
}

/* C container_of/container_of_const and module registration macros are intentionally preserved as dependency-facing macros. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
