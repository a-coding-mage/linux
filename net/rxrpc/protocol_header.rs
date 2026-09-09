/* SPDX-License-Identifier: GPL-2.0-or-later */
/* packet.h: Rx packet layout and definitions
 *
 * Copyright (C) 2002, 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

pub type rxrpc_seq_t = u32; /* Rx message sequence number */
pub type rxrpc_serial_t = u32; /* Rx message serial number */
pub type rxrpc_seq_net_t = __be32; /* on-the-wire Rx message sequence number */
pub type rxrpc_serial_net_t = __be32; /* on-the-wire Rx message serial number */

/* on-the-wire Rx packet header; all multibyte fields are in network byte order */
#[repr(C, packed)]
pub struct rxrpc_wire_header {
    pub epoch: __be32,
    pub cid: __be32,
    pub callNumber: __be32,
    pub seq: __be32,
    pub serial: __be32,
    pub type_: u8,
    pub flags: u8,
    pub userStatus: u8,
    pub securityIndex: u8,
    pub _rsvd_cksum: rxrpc_wire_header__bindgen_ty_1,
    pub serviceId: __be16,
}

pub const RXRPC_RANDOM_EPOCH: u32 = 0x80000000;
pub const RXRPC_MAXCALLS: u32 = 4;
pub const RXRPC_CHANNELMASK: u32 = RXRPC_MAXCALLS - 1;
pub const RXRPC_CIDMASK: u32 = !RXRPC_CHANNELMASK;
pub const RXRPC_CIDSHIFT: u32 = 2;
pub const RXRPC_CID_INC: u32 = 1 << RXRPC_CIDSHIFT;

pub const RXRPC_PACKET_TYPE_DATA: u8 = 1;
pub const RXRPC_PACKET_TYPE_ACK: u8 = 2;
pub const RXRPC_PACKET_TYPE_BUSY: u8 = 3;
pub const RXRPC_PACKET_TYPE_ABORT: u8 = 4;
pub const RXRPC_PACKET_TYPE_ACKALL: u8 = 5;
pub const RXRPC_PACKET_TYPE_CHALLENGE: u8 = 6;
pub const RXRPC_PACKET_TYPE_RESPONSE: u8 = 7;
pub const RXRPC_PACKET_TYPE_DEBUG: u8 = 8;
pub const RXRPC_PACKET_TYPE_PARAMS: u8 = 9;
pub const RXRPC_PACKET_TYPE_10: u8 = 10;
pub const RXRPC_PACKET_TYPE_11: u8 = 11;
pub const RXRPC_PACKET_TYPE_VERSION: u8 = 13;

pub const RXRPC_CLIENT_INITIATED: u8 = 0x01;
pub const RXRPC_REQUEST_ACK: u8 = 0x02;
pub const RXRPC_LAST_PACKET: u8 = 0x04;
pub const RXRPC_MORE_PACKETS: u8 = 0x08;
pub const RXRPC_JUMBO_PACKET: u8 = 0x20;
pub const RXRPC_SLOW_START_OK: u8 = 0x20;
pub const RXRPC_USERSTATUS_SERVICE_UPGRADE: u8 = 0x01;

#[repr(C)]
pub union rxrpc_wire_header__bindgen_ty_1 {
    pub _rsvd: __be16,
    pub cksum: __be16,
}

#[repr(C, packed)]
pub struct rxrpc_jumbo_header {
    pub flags: u8,
    pub pad: u8,
    pub _rsvd_cksum: rxrpc_jumbo_header__bindgen_ty_1,
}

#[repr(C)]
pub union rxrpc_jumbo_header__bindgen_ty_1 {
    pub _rsvd: __be16,
    pub cksum: __be16,
}

pub const RXRPC_JUMBO_DATALEN: usize = 1412;
pub const RXRPC_JUMBO_SUBPKTLEN: usize = RXRPC_JUMBO_DATALEN + core::mem::size_of::<rxrpc_jumbo_header>();
pub const RXRPC_MAX_NR_JUMBO: usize = 46;

/* Size of a jumbo packet with N subpackets, excluding UDP+IP */
pub const fn RXRPC_JUMBO(n: usize) -> usize {
    core::mem::size_of::<rxrpc_wire_header>() + RXRPC_JUMBO_DATALEN
        + (n - 1) * RXRPC_JUMBO_SUBPKTLEN
}

/* on-the-wire Rx ACK packet data payload; all multibyte fields are in network byte order */
#[repr(C, packed)]
pub struct rxrpc_ackpacket {
    pub bufferSpace: __be16,
    pub maxSkew: __be16,
    pub firstPacket: __be32,
    pub previousPacket: __be32,
    pub serial: __be32,
    pub reason: u8,
    pub nAcks: u8,
    pub acks: [u8; 0],
}

pub const RXRPC_ACK_REQUESTED: u8 = 1;
pub const RXRPC_ACK_DUPLICATE: u8 = 2;
pub const RXRPC_ACK_OUT_OF_SEQUENCE: u8 = 3;
pub const RXRPC_ACK_EXCEEDS_WINDOW: u8 = 4;
pub const RXRPC_ACK_NOSPACE: u8 = 5;
pub const RXRPC_ACK_PING: u8 = 6;
pub const RXRPC_ACK_PING_RESPONSE: u8 = 7;
pub const RXRPC_ACK_DELAY: u8 = 8;
pub const RXRPC_ACK_IDLE: u8 = 9;
pub const RXRPC_ACK__INVALID: u8 = 10;
pub const RXRPC_MAXACKS: u8 = 255;
pub const RXRPC_ACK_TYPE_NACK: u8 = 0;
pub const RXRPC_ACK_TYPE_ACK: u8 = 1;

#[repr(C)]
pub struct rxrpc_acktrailer {
    pub maxMTU: __be32,
    pub ifMTU: __be32,
    pub rwind: __be32,
    pub jumbo_max: __be32,
}

#[repr(C, packed)]
pub struct rxkad_challenge {
    pub version: __be32,
    pub nonce: __be32,
    pub min_level: __be32,
    pub __padding: __be32,
}

#[repr(C, packed)]
pub struct rxkad_response {
    pub version: __be32,
    pub __pad: __be32,
    pub encrypted: rxkad_response_encrypted,
    pub kvno: __be32,
    pub ticket_len: __be32,
}

#[repr(C)]
pub struct rxkad_response_encrypted {
    pub epoch: __be32,
    pub cid: __be32,
    pub checksum: __be32,
    pub securityIndex: __be32,
    pub call_id: [__be32; 4],
    pub inc_nonce: __be32,
    pub level: __be32,
}

#[repr(C, packed)]
pub struct rxgk_header {
    pub epoch: __be32,
    pub cid: __be32,
    pub call_number: __be32,
    pub seq: __be32,
    pub sec_index: __be32,
    pub data_len: __be32,
}

#[repr(C, packed)]
pub struct rxgk_response {
    pub start_time: __be64,
    pub token_len: __be32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
