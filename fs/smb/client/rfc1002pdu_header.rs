/* SPDX-License-Identifier: LGPL-2.1 */
/*
 *
 *   Protocol Data Unit definitions for RFC 1001/1002 support
 *
 *   Copyright (c) International Business Machines  Corp., 2004
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *
 */

/* NB: unlike smb/cifs packets, the RFC1002 structures are big endian */

/* RFC 1002 session packet types */
pub const RFC1002_SESSION_MESSAGE: u8 = 0x00;
pub const RFC1002_SESSION_REQUEST: u8 = 0x81;
pub const RFC1002_POSITIVE_SESSION_RESPONSE: u8 = 0x82;
pub const RFC1002_NEGATIVE_SESSION_RESPONSE: u8 = 0x83;
pub const RFC1002_RETARGET_SESSION_RESPONSE: u8 = 0x84;
pub const RFC1002_SESSION_KEEP_ALIVE: u8 = 0x85;

/* RFC 1002 flags (only one defined */
pub const RFC1002_LENGTH_EXTEND: u8 = 0x80; /* high order bit of length (ie +64K) */

#[repr(C, packed)]
pub struct Rfc1002SessionReq {
    pub called_len: u8,
    pub called_name: [u8; 32],
    pub scope1: u8, /* null */
    pub calling_len: u8,
    pub calling_name: [u8; 32],
    pub scope2: u8, /* null */
}

#[repr(C, packed)]
pub struct Rfc1002RetargetResp {
    pub retarget_ip_addr: u32,
    pub port: u16,
}

#[repr(C)]
pub union Rfc1002SessionPacketTrailer {
    pub session_req: Rfc1002SessionReq,
    pub retarget_resp: Rfc1002RetargetResp,
    pub neg_ses_resp_error_code: u8,
    /* POSITIVE_SESSION_RESPONSE packet does not include trailer.
     * SESSION_KEEP_ALIVE packet also does not include a trailer.
     * Trailer for the SESSION_MESSAGE packet is SMB/CIFS header */
}

#[repr(C, packed)]
pub struct Rfc1002SessionPacket {
    pub type_: u8,
    pub flags: u8,
    pub length: u16,
    pub trailer: Rfc1002SessionPacketTrailer,
}

/* Negative Session Response error codes */
pub const RFC1002_NOT_LISTENING_CALLED: u8 = 0x80; /* not listening on called name */
pub const RFC1002_NOT_LISTENING_CALLING: u8 = 0x81; /* not listening on calling name */
pub const RFC1002_NOT_PRESENT: u8 = 0x82; /* called name not present */
pub const RFC1002_INSUFFICIENT_RESOURCE: u8 = 0x83;
pub const RFC1002_UNSPECIFIED_ERROR: u8 = 0x8F;

/* RFC 1002 Datagram service packets are not defined here as they
 * are not needed for the network filesystem client unless we plan on
 * implementing broadcast resolution of the server ip address (from
 * server netbios name). Currently server names are resolved only via DNS
 * (tcp name) or ip address or an /etc/hosts equivalent mapping to ip address.*/

pub const DEFAULT_CIFS_CALLED_NAME: &[u8] = b"*SMBSERVER      \0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
