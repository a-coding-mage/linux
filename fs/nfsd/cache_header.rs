/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Request reply cache. This was heavily inspired by the
 * implementation in 4.3BSD/4.4BSD.
 *
 * Copyright (C) 1995, 1996 Olaf Kirch <okir@monad.swb.de>
 */

// Dependency declarations supplied by the surrounding kernel translation.

/*
 * Representation of a reply cache entry.
 *
 * Note that we use a sockaddr_in6 to hold the address instead of the more
 * typical sockaddr_storage. This is for space reasons, since sockaddr_storage
 * is much larger than a sockaddr_in6.
 */
#[repr(C)]
pub struct nfsd_cacherep {
    pub c_key: nfsd_cacherep_key,
    pub c_node: rb_node,
    pub c_lru: list_head,
    pub c_state: u8, // unused, inprog, done
    pub c_type: u8,  // status, buffer
    // C bit-field c_secure : 1; /* req came from port < 1024 */
    pub c_secure: u8,
    pub c_timestamp: c_ulong,
    pub c_u: nfsd_cacherep_union,
}

#[repr(C)]
pub struct nfsd_cacherep_key {
    /* Keep often-read xid, csum in the same cache line: */
    pub k_xid: __be32,
    pub k_csum: __wsum,
    pub k_proc: u32,
    pub k_prot: u32,
    pub k_vers: u32,
    pub k_len: c_uint,
    pub k_addr: sockaddr_in6,
}

#[repr(C)]
pub union nfsd_cacherep_union {
    pub u_vec: kvec,
    pub u_status: __be32,
}

// #define c_replvec c_u.u_vec
// #define c_replstat c_u.u_status

pub const RC_UNUSED: i32 = 0;
pub const RC_INPROG: i32 = 1;
pub const RC_DONE: i32 = 2;

pub const RC_DROPIT: i32 = 0;
pub const RC_REPLY: i32 = 1;
pub const RC_DOIT: i32 = 2;

pub const RC_NOCACHE: i32 = 0;
pub const RC_REPLSTAT: i32 = 1;
pub const RC_REPLBUFF: i32 = 2;

/* Cache entry expiration is expressed in kernel ticks. */
pub const RC_EXPIRE: c_ulong = 120 * HZ;

/* Checksum this amount of the request */
pub const RC_CSUMLEN: u32 = 256u32;

extern "C" {
    pub fn nfsd_drc_slab_create() -> c_int;
    pub fn nfsd_drc_slab_free();
    pub fn nfsd_reply_cache_init(nn: *mut nfsd_net) -> c_int;
    pub fn nfsd_reply_cache_shutdown(nn: *mut nfsd_net);
    pub fn nfsd_cache_lookup(
        rqstp: *mut svc_rqst,
        start: c_uint,
        len: c_uint,
        cacherep: *mut *mut nfsd_cacherep,
    ) -> c_int;
    pub fn nfsd_cache_update(
        rqstp: *mut svc_rqst,
        rp: *mut nfsd_cacherep,
        cachetype: c_int,
        statp: *mut __be32,
    );
    pub fn nfsd_reply_cache_stats_show(m: *mut seq_file, v: *mut c_void) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
