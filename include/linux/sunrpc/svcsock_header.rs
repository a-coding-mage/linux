/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/linux/sunrpc/svcsock.h
 *
 * RPC server socket I/O.
 *
 * Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
 */

// Dependencies supplied by the surrounding Linux SUNRPC translation.

/*
 * RPC server socket.
 */
#[repr(C)]
pub struct svc_sock {
    pub sk_xprt: svc_xprt,
    pub sk_sock: *mut socket, // berkeley socket layer
    pub sk_sk: *mut sock, // INET layer

    /* We keep the old state_change and data_ready CB's here */
    pub sk_ostate: Option<unsafe extern "C" fn(*mut sock)>,
    pub sk_odata: Option<unsafe extern "C" fn(*mut sock)>,
    pub sk_owspace: Option<unsafe extern "C" fn(*mut sock)>,

    /* For sends (protected by xpt_mutex) */
    pub sk_bvec: *mut bio_vec,

    /* private TCP part */
    /* On-the-wire fragment header: */
    pub sk_marker: __be32,
    /* As we receive a record, this includes the length received so
     * far (including the fragment header): */
    pub sk_tcplen: u32,
    /* Total length of the data (not including fragment headers)
     * received so far in the fragments making up this rpc: */
    pub sk_datalen: u32,

    pub sk_frag_cache: page_frag_cache,
    pub sk_handshake_done: completion,

    /* received data */
    pub sk_maxpages: usize,
    // C flexible array member: __counted_by(sk_maxpages)
    pub sk_pages: [*mut page; 0],
}

pub unsafe inline fn svc_sock_reclen(svsk: *mut svc_sock) -> u32 {
    be32_to_cpu((*svsk).sk_marker) & RPC_FRAGMENT_SIZE_MASK
}

pub unsafe inline fn svc_sock_final_rec(svsk: *mut svc_sock) -> u32 {
    be32_to_cpu((*svsk).sk_marker) & RPC_LAST_STREAM_FRAGMENT
}

/*
 * Function prototypes.
 */
extern "C" {
    pub fn svc_recv(rqstp: *mut svc_rqst, timeo: i64) -> i32;
    pub fn svc_send(rqstp: *mut svc_rqst);
    pub fn svc_addsock(
        serv: *mut svc_serv,
        net: *mut net,
        fd: i32,
        name_return: *mut u8,
        len: usize,
        cred: *const cred,
    ) -> i32;
    pub fn svc_init_xprt_sock();
    pub fn svc_cleanup_xprt_sock();
}

/* svc_makesock socket characteristics */
pub const SVC_SOCK_DEFAULTS: u32 = 0U;
pub const SVC_SOCK_ANONYMOUS: u32 = 1U << 0; // don't register with pmap
pub const SVC_SOCK_TEMPORARY: u32 = 1U << 1; // flag socket as temporary

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
