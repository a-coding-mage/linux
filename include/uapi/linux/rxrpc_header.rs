/* SPDX-License-Identifier: GPL-2.0-or-later WITH Linux-syscall-note */
/* Types and definitions for AF_RXRPC.
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use core::mem::ManuallyDrop;

/* External types supplied by the corresponding Linux headers. */
use linux_types::{__u8, __u16, __u32, __kernel_sa_family_t};
use linux_in::{sockaddr_in};
use linux_in6::{sockaddr_in6};

/*
 * RxRPC socket address
 */
#[repr(C)]
pub struct sockaddr_rxrpc {
    pub srx_family: __kernel_sa_family_t, /* address family */
    pub srx_service: __u16,               /* service desired */
    pub transport_type: __u16,            /* type of transport socket (SOCK_DGRAM) */
    pub transport_len: __u16,              /* length of transport address */
    pub transport: sockaddr_rxrpc_transport,
}

#[repr(C)]
pub union sockaddr_rxrpc_transport {
    pub family: __kernel_sa_family_t, /* transport address family */
    pub sin: ManuallyDrop<sockaddr_in>, /* IPv4 transport address */
    pub sin6: ManuallyDrop<sockaddr_in6>, /* IPv6 transport address */
}

/*
 * RxRPC socket options
 */
pub const RXRPC_SECURITY_KEY: i32 = 1; /* [clnt] set client security key */
pub const RXRPC_SECURITY_KEYRING: i32 = 2; /* [srvr] set ring of server security keys */
pub const RXRPC_EXCLUSIVE_CONNECTION: i32 = 3; /* Deprecated; use RXRPC_EXCLUSIVE_CALL instead */
pub const RXRPC_MIN_SECURITY_LEVEL: i32 = 4; /* minimum security level */
pub const RXRPC_UPGRADEABLE_SERVICE: i32 = 5; /* Upgrade service[0] -> service[1] */
pub const RXRPC_SUPPORTED_CMSG: i32 = 6; /* Get highest supported control message type */
pub const RXRPC_MANAGE_RESPONSE: i32 = 7; /* [clnt] Want to manage RESPONSE packets */

/*
 * RxRPC control messages
 * - If neither abort or accept are specified, the message is a data message.
 * - terminal messages mean that a user call ID tag can be recycled
 * - C/S/- indicate whether these are applicable to client, server or both
 * - s/r/- indicate whether these are applicable to sendmsg() and/or recvmsg()
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rxrpc_cmsg_type {
    RXRPC_USER_CALL_ID = 1,
    RXRPC_ABORT = 2,
    RXRPC_ACK = 3,
    RXRPC_NET_ERROR = 5,
    RXRPC_BUSY = 6,
    RXRPC_LOCAL_ERROR = 7,
    RXRPC_NEW_CALL = 8,
    RXRPC_EXCLUSIVE_CALL = 10,
    RXRPC_UPGRADE_SERVICE = 11,
    RXRPC_TX_LENGTH = 12,
    RXRPC_SET_CALL_TIMEOUT = 13,
    RXRPC_CHARGE_ACCEPT = 14,
    RXRPC_OOB_ID = 15,
    RXRPC_CHALLENGED = 16,
    RXRPC_RESPOND = 17,
    RXRPC_RESPONDED = 18,
    RXRPC_RESP_RXGK_APPDATA = 19,
    RXRPC__SUPPORTED = 20,
}

/* RxRPC security levels */
pub const RXRPC_SECURITY_PLAIN: i32 = 0;
pub const RXRPC_SECURITY_AUTH: i32 = 1;
pub const RXRPC_SECURITY_ENCRYPT: i32 = 2;

/* RxRPC security indices */
pub const RXRPC_SECURITY_NONE: i32 = 0;
pub const RXRPC_SECURITY_RXKAD: i32 = 2;
pub const RXRPC_SECURITY_RXGK: i32 = 4;
pub const RXRPC_SECURITY_RXK5: i32 = 5;
pub const RXRPC_SECURITY_YFS_RXGK: i32 = 6;

/* RxRPC-level abort codes */
pub const RX_CALL_DEAD: i32 = -1;
pub const RX_INVALID_OPERATION: i32 = -2;
pub const RX_CALL_TIMEOUT: i32 = -3;
pub const RX_EOF: i32 = -4;
pub const RX_PROTOCOL_ERROR: i32 = -5;
pub const RX_USER_ABORT: i32 = -6;
pub const RX_ADDRINUSE: i32 = -7;
pub const RX_DEBUGI_BADTYPE: i32 = -8;

/* (un)marshalling abort codes (rxgen) */
pub const RXGEN_CC_MARSHAL: i32 = -450;
pub const RXGEN_CC_UNMARSHAL: i32 = -451;
pub const RXGEN_SS_MARSHAL: i32 = -452;
pub const RXGEN_SS_UNMARSHAL: i32 = -453;
pub const RXGEN_DECODE: i32 = -454;
pub const RXGEN_OPCODE: i32 = -455;
pub const RXGEN_SS_XDRFREE: i32 = -456;
pub const RXGEN_CC_XDRFREE: i32 = -457;

/* Rx kerberos security abort codes */
pub const RXKADINCONSISTENCY: i32 = 19270400;
pub const RXKADPACKETSHORT: i32 = 19270401;
pub const RXKADLEVELFAIL: i32 = 19270402;
pub const RXKADTICKETLEN: i32 = 19270403;
pub const RXKADOUTOFSEQUENCE: i32 = 19270404;
pub const RXKADNOAUTH: i32 = 19270405;
pub const RXKADBADKEY: i32 = 19270406;
pub const RXKADBADTICKET: i32 = 19270407;
pub const RXKADUNKNOWNKEY: i32 = 19270408;
pub const RXKADEXPIRED: i32 = 19270409;
pub const RXKADSEALEDINCON: i32 = 19270410;
pub const RXKADDATALEN: i32 = 19270411;
pub const RXKADILLEGALLEVEL: i32 = 19270412;

/* RxGK GSSAPI security abort codes. */
/* Original standard abort codes (used by OpenAFS) are disabled in the source. */
/* Revised standard abort codes (used by YFS): */
pub const RXGK_INCONSISTENCY: i32 = 1233242880;
pub const RXGK_PACKETSHORT: i32 = 1233242881;
pub const RXGK_BADCHALLENGE: i32 = 1233242882;
pub const RXGK_SEALEDINCON: i32 = 1233242883;
pub const RXGK_NOTAUTH: i32 = 1233242884;
pub const RXGK_EXPIRED: i32 = 1233242885;
pub const RXGK_BADLEVEL: i32 = 1233242886;
pub const RXGK_BADKEYNO: i32 = 1233242887;
pub const RXGK_NOTRXGK: i32 = 1233242888;
pub const RXGK_UNSUPPORTED: i32 = 1233242889;
pub const RXGK_GSSERROR: i32 = 1233242890;

/*
 * Challenge information in the RXRPC_CHALLENGED control message.
 */
#[repr(C)]
pub struct rxrpc_challenge {
    pub service_id: __u16,
    pub security_index: __u8,
    pub pad: __u8,
    /* ... The security class gets to append extra information ... */
}

#[repr(C)]
pub struct rxgk_challenge {
    pub base: rxrpc_challenge,
    pub enctype: __u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
