/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2011  Intel Corporation. All rights reserved.
 */

use core::ffi::c_void;

#[repr(i32)]
pub enum llcp_state {
    LLCP_CONNECTED = 1,
    LLCP_CONNECTING,
    LLCP_CLOSED,
    LLCP_BOUND,
    LLCP_LISTEN,
}

pub const LLCP_DEFAULT_LTO: u32 = 100;
pub const LLCP_DEFAULT_RW: u32 = 1;
pub const LLCP_DEFAULT_MIU: u32 = 128;
pub const LLCP_MAX_LTO: u32 = 0xff;
pub const LLCP_MAX_RW: u32 = 15;
pub const LLCP_MAX_MIUX: u32 = 0x7ff;
pub const LLCP_MAX_MIU: u32 = LLCP_MAX_MIUX + 128;
pub const LLCP_WKS_NUM_SAP: u32 = 16;
pub const LLCP_SDP_NUM_SAP: u32 = 16;
pub const LLCP_LOCAL_NUM_SAP: u32 = 32;
pub const LLCP_LOCAL_SAP_OFFSET: u32 = LLCP_WKS_NUM_SAP + LLCP_SDP_NUM_SAP;
pub const LLCP_MAX_SAP: u32 = LLCP_WKS_NUM_SAP + LLCP_SDP_NUM_SAP + LLCP_LOCAL_NUM_SAP;
pub const LLCP_SDP_UNBOUND: u32 = LLCP_MAX_SAP + 1;

pub const LLCP_HEADER_SIZE: u32 = 2;
pub const LLCP_SEQUENCE_SIZE: u32 = 1;
pub const LLCP_AGF_PDU_HEADER_SIZE: u32 = 2;
pub const LLCP_VERSION_10: u32 = 0x10;
pub const LLCP_VERSION_11: u32 = 0x11;
pub const LLCP_PDU_SYMM: u32 = 0x0;
pub const LLCP_PDU_PAX: u32 = 0x1;
pub const LLCP_PDU_AGF: u32 = 0x2;
pub const LLCP_PDU_UI: u32 = 0x3;
pub const LLCP_PDU_CONNECT: u32 = 0x4;
pub const LLCP_PDU_DISC: u32 = 0x5;
pub const LLCP_PDU_CC: u32 = 0x6;
pub const LLCP_PDU_DM: u32 = 0x7;
pub const LLCP_PDU_FRMR: u32 = 0x8;
pub const LLCP_PDU_SNL: u32 = 0x9;
pub const LLCP_PDU_I: u32 = 0xc;
pub const LLCP_PDU_RR: u32 = 0xd;
pub const LLCP_PDU_RNR: u32 = 0xe;
pub const LLCP_TLV_VERSION: u32 = 0x1;
pub const LLCP_TLV_MIUX: u32 = 0x2;
pub const LLCP_TLV_WKS: u32 = 0x3;
pub const LLCP_TLV_LTO: u32 = 0x4;
pub const LLCP_TLV_RW: u32 = 0x5;
pub const LLCP_TLV_SN: u32 = 0x6;
pub const LLCP_TLV_OPT: u32 = 0x7;
pub const LLCP_TLV_SDREQ: u32 = 0x8;
pub const LLCP_TLV_SDRES: u32 = 0x9;
pub const LLCP_TLV_MAX: u32 = 0xa;
pub const LLCP_SAP_SDP: u32 = 0x1;
pub const LLCP_SAP_IP: u32 = 0x2;
pub const LLCP_SAP_OBEX: u32 = 0x3;
pub const LLCP_SAP_SNEP: u32 = 0x4;
pub const LLCP_SAP_MAX: u32 = 0xff;
pub const LLCP_DM_DISC: u32 = 0x00;
pub const LLCP_DM_NOCONN: u32 = 0x01;
pub const LLCP_DM_NOBOUND: u32 = 0x02;
pub const LLCP_DM_REJ: u32 = 0x03;

pub struct nfc_llcp_sock;

#[repr(C)]
pub struct llcp_sock_list { pub head: hlist_head, pub lock: rwlock_t }

#[repr(C)]
pub struct nfc_llcp_sdp_tlv {
    pub tlv: *mut u8, pub tlv_len: u8, pub uri: *mut i8, pub tid: u8, pub sap: u8,
    pub time: usize, pub node: hlist_node,
}

#[repr(C)]
pub struct nfc_llcp_local {
    pub list: list_head, pub dev: *mut nfc_dev, pub ref_: kref, pub sdp_lock: mutex,
    pub link_timer: timer_list, pub tx_queue: sk_buff_head, pub tx_work: work_struct,
    pub rx_work: work_struct, pub rx_pending: *mut sk_buff, pub timeout_work: work_struct,
    pub target_idx: u32, pub rf_mode: u8, pub comm_mode: u8, pub lto: u8, pub rw: u8,
    pub miux: __be16, pub local_wks: usize, pub local_sdp: usize, pub local_sap: usize,
    pub local_sdp_cnt: [atomic_t; LLCP_SDP_NUM_SAP as usize], pub gb: [u8; NFC_MAX_GT_LEN as usize],
    pub gb_len: u8, pub remote_gb: [u8; NFC_MAX_GT_LEN as usize], pub remote_gb_len: u8,
    pub remote_version: u8, pub remote_miu: u16, pub remote_lto: u16, pub remote_opt: u8,
    pub remote_wks: u16, pub sdreq_lock: mutex, pub pending_sdreqs: hlist_head,
    pub sdreq_timer: timer_list, pub sdreq_timeout_work: work_struct, pub sdreq_next_tid: u8,
    pub sockets: llcp_sock_list, pub connecting_sockets: llcp_sock_list, pub raw_sockets: llcp_sock_list,
}

#[repr(C)]
pub struct nfc_llcp_sock {
    pub sk: sock, pub dev: *mut nfc_dev, pub local: *mut nfc_llcp_local, pub target_idx: u32,
    pub nfc_protocol: u32, pub ssap: u8, pub dsap: u8, pub service_name: *mut i8,
    pub service_name_len: size_t, pub rw: u8, pub miux: __be16, pub remote_rw: u8,
    pub remote_miu: u16, pub send_n: u8, pub send_ack_n: u8, pub recv_n: u8, pub recv_ack_n: u8,
    pub remote_ready: u8, pub reserved_ssap: u8, pub tx_queue: sk_buff_head,
    pub tx_pending_queue: sk_buff_head, pub accept_queue: list_head, pub parent: *mut sock,
}

#[repr(C)] pub struct nfc_llcp_ui_cb { pub dsap: u8, pub ssap: u8 }

/* C macro equivalents. */
#[inline] pub unsafe fn nfc_llcp_ui_skb_cb(__skb: *mut sk_buff) -> *mut nfc_llcp_ui_cb { &mut (*__skb).cb[0] as *mut _ as *mut nfc_llcp_ui_cb }
#[inline] pub unsafe fn nfc_llcp_sock_fn(sk: *mut sock) -> *mut nfc_llcp_sock { sk as *mut nfc_llcp_sock }
#[inline] pub unsafe fn nfc_llcp_dev(sk: *mut sock) -> *mut nfc_dev { (*nfc_llcp_sock_fn(sk)).dev }

extern "C" {
    pub fn nfc_llcp_sock_link(l: *mut llcp_sock_list, s: *mut sock);
    pub fn nfc_llcp_sock_unlink(l: *mut llcp_sock_list, s: *mut sock);
    pub fn nfc_llcp_socket_remote_param_init(sock: *mut nfc_llcp_sock);
    pub fn nfc_llcp_find_local(dev: *mut nfc_dev) -> *mut nfc_llcp_local;
    pub fn nfc_llcp_local_put(local: *mut nfc_llcp_local) -> i32;
    pub fn nfc_llcp_get_sdp_ssap(local: *mut nfc_llcp_local, sock: *mut nfc_llcp_sock) -> u8;
    pub fn nfc_llcp_get_local_ssap(local: *mut nfc_llcp_local) -> u8;
    pub fn nfc_llcp_put_ssap(local: *mut nfc_llcp_local, ssap: u8);
    pub fn nfc_llcp_queue_i_frames(sock: *mut nfc_llcp_sock) -> i32;
    pub fn nfc_llcp_send_to_raw_sock(local: *mut nfc_llcp_local, skb: *mut sk_buff, direction: u8);
    pub fn nfc_llcp_sock_alloc(sock: *mut socket, type_: i32, gfp: gfp_t, kern: i32) -> *mut sock;
    pub fn nfc_llcp_sock_free(sock: *mut nfc_llcp_sock);
    pub fn nfc_llcp_accept_unlink(sk: *mut sock);
    pub fn nfc_llcp_accept_enqueue(parent: *mut sock, sk: *mut sock);
    pub fn nfc_llcp_accept_dequeue(sk: *mut sock, newsock: *mut socket) -> *mut sock;
    pub fn nfc_llcp_parse_gb_tlv(local: *mut nfc_llcp_local, tlv_array: *const u8, tlv_array_len: u16) -> i32;
    pub fn nfc_llcp_parse_connection_tlv(sock: *mut nfc_llcp_sock, tlv_array: *const u8, tlv_array_len: u16) -> i32;
    pub fn nfc_llcp_recv(data: *mut c_void, skb: *mut sk_buff, err: i32);
    pub fn nfc_llcp_build_tlv(type_: u8, value: *const u8, value_length: u8, tlv_length: *mut u8) -> *mut u8;
    pub fn nfc_llcp_build_sdres_tlv(tid: u8, sap: u8) -> *mut nfc_llcp_sdp_tlv;
    pub fn nfc_llcp_build_sdreq_tlv(tid: u8, uri: *const i8, uri_len: size_t) -> *mut nfc_llcp_sdp_tlv;
    pub fn nfc_llcp_free_sdp_tlv(sdp: *mut nfc_llcp_sdp_tlv);
    pub fn nfc_llcp_free_sdp_tlv_list(sdp_head: *mut hlist_head);
    pub fn nfc_llcp_send_symm(dev: *mut nfc_dev) -> i32;
    pub fn nfc_llcp_send_connect(sock: *mut nfc_llcp_sock) -> i32;
    pub fn nfc_llcp_send_cc(sock: *mut nfc_llcp_sock) -> i32;
    pub fn nfc_llcp_send_snl_sdres(local: *mut nfc_llcp_local, tlv_list: *mut hlist_head, tlvs_len: size_t) -> i32;
    pub fn nfc_llcp_send_snl_sdreq(local: *mut nfc_llcp_local, tlv_list: *mut hlist_head, tlvs_len: size_t) -> i32;
    pub fn nfc_llcp_send_dm(local: *mut nfc_llcp_local, ssap: u8, dsap: u8, reason: u8) -> i32;
    pub fn nfc_llcp_send_disconnect(sock: *mut nfc_llcp_sock) -> i32;
    pub fn nfc_llcp_send_i_frame(sock: *mut nfc_llcp_sock, msg: *mut msghdr, len: size_t) -> i32;
    pub fn nfc_llcp_send_ui_frame(sock: *mut nfc_llcp_sock, ssap: u8, dsap: u8, msg: *mut msghdr, len: size_t) -> i32;
    pub fn nfc_llcp_send_rr(sock: *mut nfc_llcp_sock) -> i32;
    pub fn nfc_llcp_sock_init() -> i32;
    pub fn nfc_llcp_sock_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
