/* SPDX-License-Identifier: GPL-2.0 */

pub const CEPHX_GET_AUTH_SESSION_KEY: u16 = 0x0100;
pub const CEPHX_GET_PRINCIPAL_SESSION_KEY: u16 = 0x0200;
pub const CEPHX_GET_ROTATING_KEY: u16 = 0x0400;

/* Client <-> AuthMonitor */
/*
 * The AUTH session's connection secret: encrypted with the AUTH
 * ticket session key
 */
pub const CEPHX_KEY_USAGE_AUTH_CONNECTION_SECRET: u8 = 0x03;
/*
 * The ticket's blob for the client ("blob for me", contains the
 * session key): encrypted with the client's secret key in case of
 * the AUTH ticket and the AUTH ticket session key in case of other
 * service tickets
 */
pub const CEPHX_KEY_USAGE_TICKET_SESSION_KEY: u8 = 0x04;
/*
 * The ticket's blob for the service (ceph_x_ticket_blob): possibly
 * encrypted with the old AUTH ticket session key in case of the AUTH
 * ticket and not encrypted in case of other service tickets
 */
pub const CEPHX_KEY_USAGE_TICKET_BLOB: u8 = 0x05;

/* Client <-> Service */
/*
 * The client's authorization request (ceph_x_authorize_b):
 * encrypted with the service ticket session key
 */
pub const CEPHX_KEY_USAGE_AUTHORIZE: u8 = 0x10;
/*
 * The service's challenge (ceph_x_authorize_challenge):
 * encrypted with the service ticket session key
 */
pub const CEPHX_KEY_USAGE_AUTHORIZE_CHALLENGE: u8 = 0x11;
/*
 * The service's final reply (ceph_x_authorize_reply + the service
 * session's connection secret): encrypted with the service ticket
 * session key
 */
pub const CEPHX_KEY_USAGE_AUTHORIZE_REPLY: u8 = 0x12;

/* common bits */
#[repr(C, packed)]
pub struct ceph_x_ticket_blob {
    pub struct_v: u8,
    pub secret_id: u64,
    pub blob_len: u32,
    pub blob: [u8; 0],
}

/* common request/reply headers */
#[repr(C, packed)]
pub struct ceph_x_request_header {
    pub op: u16,
}

#[repr(C, packed)]
pub struct ceph_x_reply_header {
    pub op: u16,
    pub result: u32,
}

/* authenticate handshake */

/* initial hello (no reply header) */
#[repr(C, packed)]
pub struct ceph_x_server_challenge {
    pub struct_v: u8,
    pub server_challenge: u64,
}

#[repr(C, packed)]
pub struct ceph_x_authenticate {
    pub struct_v: u8,
    pub client_challenge: u64,
    pub key: u64,
    /* old_ticket blob */
    /* nautilus+: other_keys */
}

#[repr(C, packed)]
pub struct ceph_x_service_ticket_request {
    pub struct_v: u8,
    pub keys: u32,
}

#[repr(C, packed)]
pub struct ceph_x_challenge_blob {
    pub server_challenge: u64,
    pub client_challenge: u64,
}

/* authorize handshake */

/*
 * The authorizer consists of two pieces:
 *  a - service id, ticket blob
 *  b - encrypted with session key
 */
#[repr(C, packed)]
pub struct ceph_x_authorize_a {
    pub struct_v: u8,
    pub global_id: u64,
    pub service_id: u32,
    pub ticket_blob: ceph_x_ticket_blob,
}

#[repr(C, packed)]
pub struct ceph_x_authorize_b {
    pub struct_v: u8,
    pub nonce: u64,
    pub have_challenge: u8,
    pub server_challenge_plus_one: u64,
}

#[repr(C, packed)]
pub struct ceph_x_authorize_challenge {
    pub struct_v: u8,
    pub server_challenge: u64,
}

#[repr(C, packed)]
pub struct ceph_x_authorize_reply {
    pub struct_v: u8,
    pub nonce_plus_one: u64,
}

/*
 * encryption bundle
 */
pub const CEPHX_ENC_MAGIC: u64 = 0xff009cad8826aa55;

#[repr(C, packed)]
pub struct ceph_x_encrypt_header {
    pub struct_v: u8,
    pub magic: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
