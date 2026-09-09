/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* AF_VSOCK sock_diag(7) interface for querying open sockets */

/* Request */
#[repr(C)]
pub struct vsock_diag_req {
    pub sdiag_family: u8,    /* must be AF_VSOCK */
    pub sdiag_protocol: u8,  /* must be 0 */
    pub pad: u16,            /* must be 0 */
    pub vdiag_states: u32,   /* query bitmap (e.g. 1 << TCP_LISTEN) */
    pub vdiag_ino: u32,      /* must be 0 (reserved) */
    pub vdiag_show: u32,     /* must be 0 (reserved) */
    pub vdiag_cookie: [u32; 2],
}

/* Response */
#[repr(C)]
pub struct vsock_diag_msg {
    pub vdiag_family: u8,    /* AF_VSOCK */
    pub vdiag_type: u8,      /* SOCK_STREAM or SOCK_DGRAM */
    pub vdiag_state: u8,     /* sk_state (e.g. TCP_LISTEN) */
    pub vdiag_shutdown: u8,  /* local RCV_SHUTDOWN | SEND_SHUTDOWN */
    pub vdiag_src_cid: u32,
    pub vdiag_src_port: u32,
    pub vdiag_dst_cid: u32,
    pub vdiag_dst_port: u32,
    pub vdiag_ino: u32,
    pub vdiag_cookie: [u32; 2],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
