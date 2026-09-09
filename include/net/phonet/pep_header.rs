/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * File: pep.h
 *
 * Phonet Pipe End Point sockets definitions
 *
 * Copyright (C) 2008 Nokia Corporation.
 */

/* Declarations supplied by linux/skbuff.h and net/phonet/phonet.h. */

#[repr(C)]
pub struct pep_sock {
    pub pn_sk: pn_sock,

    /* XXX: union-ify listening vs connected stuff ? */
    /* Listening socket stuff: */
    pub hlist: hlist_head,

    /* Connected socket stuff: */
    pub listener: *mut sock,
    pub ctrlreq_queue: sk_buff_head,
    pub tx_credits: atomic_t,
    pub ifindex: ::core::ffi::c_int,
    pub peer_type: u16, /* peer type/subtype */
    pub pipe_handle: u8,

    pub rx_credits: u8,
    pub rx_fc: u8, /* RX flow control */
    pub tx_fc: u8, /* TX flow control */
    pub init_enable: u8, /* auto-enable at creation */
    pub aligned: u8,
}

pub const PNPIPE_CTRLREQ_MAX: ::core::ffi::c_int = 10;

#[inline]
pub unsafe fn pep_sk(sk: *mut sock) -> *mut pep_sock {
    sk as *mut pep_sock
}

pub unsafe extern "C" {
    pub static phonet_stream_ops: proto_ops;
}

/* Pipe protocol definitions */
#[repr(C)]
pub union pnpipehdr_state {
    pub state_after_connect: u8, /* connect request */
    pub state_after_reset: u8, /* reset request */
    pub error_code: u8, /* any response */
    pub pep_type: u8, /* status indication */
    pub data0: u8, /* anything else */
}

#[repr(C)]
pub struct pnpipehdr {
    pub utid: u8, /* transaction ID */
    pub message_id: u8,
    pub pipe_handle: u8,
    pub state: pnpipehdr_state,
    pub data: [u8; 0],
}

/* #define other_pep_type data[0] */
#[inline]
pub unsafe fn other_pep_type(hdr: *mut pnpipehdr) -> *mut u8 {
    (*hdr).data.as_mut_ptr()
}

#[inline]
pub unsafe fn pnp_hdr(skb: *mut sk_buff) -> *mut pnpipehdr {
    skb_transport_header(skb) as *mut pnpipehdr
}

pub const MAX_PNPIPE_HEADER: usize = MAX_PHONET_HEADER as usize + 4;

pub const PNS_PIPE_CREATE_REQ: u32 = 0x00;
pub const PNS_PIPE_CREATE_RESP: u32 = 0x01;
pub const PNS_PIPE_REMOVE_REQ: u32 = 0x02;
pub const PNS_PIPE_REMOVE_RESP: u32 = 0x03;
pub const PNS_PIPE_DATA: u32 = 0x20;
pub const PNS_PIPE_ALIGNED_DATA: u32 = 0x21;
pub const PNS_PEP_CONNECT_REQ: u32 = 0x40;
pub const PNS_PEP_CONNECT_RESP: u32 = 0x41;
pub const PNS_PEP_DISCONNECT_REQ: u32 = 0x42;
pub const PNS_PEP_DISCONNECT_RESP: u32 = 0x43;
pub const PNS_PEP_RESET_REQ: u32 = 0x44;
pub const PNS_PEP_RESET_RESP: u32 = 0x45;
pub const PNS_PEP_ENABLE_REQ: u32 = 0x46;
pub const PNS_PEP_ENABLE_RESP: u32 = 0x47;
pub const PNS_PEP_CTRL_REQ: u32 = 0x48;
pub const PNS_PEP_CTRL_RESP: u32 = 0x49;
pub const PNS_PEP_DISABLE_REQ: u32 = 0x4c;
pub const PNS_PEP_DISABLE_RESP: u32 = 0x4d;
pub const PNS_PEP_STATUS_IND: u32 = 0x60;
pub const PNS_PIPE_CREATED_IND: u32 = 0x61;
pub const PNS_PIPE_RESET_IND: u32 = 0x63;
pub const PNS_PIPE_ENABLED_IND: u32 = 0x64;
pub const PNS_PIPE_REDIRECTED_IND: u32 = 0x65;
pub const PNS_PIPE_DISABLED_IND: u32 = 0x66;

pub const PN_PIPE_INVALID_HANDLE: u8 = 0xff;
pub const PN_PEP_TYPE_COMMON: u8 = 0x00;

pub const PN_PEP_IND_FLOW_CONTROL: u32 = 0;
pub const PN_PEP_IND_ID_MCFC_GRANT_CREDITS: u32 = 1;

pub const PN_PIPE_NO_ERROR: u32 = 0;
pub const PN_PIPE_ERR_INVALID_PARAM: u32 = 1;
pub const PN_PIPE_ERR_INVALID_HANDLE: u32 = 2;
pub const PN_PIPE_ERR_INVALID_CTRL_ID: u32 = 3;
pub const PN_PIPE_ERR_NOT_ALLOWED: u32 = 4;
pub const PN_PIPE_ERR_PEP_IN_USE: u32 = 5;
pub const PN_PIPE_ERR_OVERLOAD: u32 = 6;
pub const PN_PIPE_ERR_DEV_DISCONNECTED: u32 = 7;
pub const PN_PIPE_ERR_TIMEOUT: u32 = 8;
pub const PN_PIPE_ERR_ALL_PIPES_IN_USE: u32 = 9;
pub const PN_PIPE_ERR_GENERAL: u32 = 10;
pub const PN_PIPE_ERR_NOT_SUPPORTED: u32 = 11;

pub const PN_PIPE_DISABLE: u32 = 0;
pub const PN_PIPE_ENABLE: u32 = 1;

pub const PN_PIPE_SB_CREATE_REQ_PEP_SUB_TYPE: u32 = 0;
pub const PN_PIPE_SB_CONNECT_REQ_PEP_SUB_TYPE: u32 = 1;
pub const PN_PIPE_SB_REDIRECT_REQ_PEP_SUB_TYPE: u32 = 2;
pub const PN_PIPE_SB_NEGOTIATED_FC: u32 = 3;
pub const PN_PIPE_SB_REQUIRED_FC_TX: u32 = 4;
pub const PN_PIPE_SB_PREFERRED_FC_RX: u32 = 5;
pub const PN_PIPE_SB_ALIGNED_DATA: u32 = 6;

pub const PN_NO_FLOW_CONTROL: u32 = 0;
pub const PN_LEGACY_FLOW_CONTROL: u32 = 1;
pub const PN_ONE_CREDIT_FLOW_CONTROL: u32 = 2;
pub const PN_MULTI_CREDIT_FLOW_CONTROL: u32 = 3;
pub const PN_MAX_FLOW_CONTROL: u32 = 4;

#[inline]
pub const fn pn_flow_safe(fc: u32) -> u32 {
    fc >> 1
}

pub const PEP_IND_EMPTY: u32 = 0;
pub const PEP_IND_BUSY: u32 = 1;
pub const PEP_IND_READY: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
