/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SCTP kernel implementation
 * (C) Copyright IBM Corp. 2001, 2004
 * Copyright (c) 1999-2000 Cisco, Inc.
 * Copyright (c) 1999-2001 Motorola, Inc.
 * Copyright (c) 2001 Intel Corp.
 *
 * These are the definitions needed for the tsnmap type.  The tsnmap is used
 * to track out of order TSNs received.
 */

/* Dependency supplied by the SCTP constants header. */
pub type __u16 = u16;
pub type __u32 = u32;
pub type __be32 = u32;
pub type gfp_t = usize;

/* SCTP_MAX_DUP_TSNS is supplied by the SCTP constants dependency. */
extern "C" {
    pub fn htonl(hostlong: __u32) -> __be32;
}

#[repr(C)]
pub struct sctp_transport {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sctp_gap_ack_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sctp_tsnmap {
    /* This array counts the number of chunks with each TSN. */
    pub tsn_map: *mut usize,
    /* This is the TSN at tsn_map[0]. */
    pub base_tsn: __u32,
    /* The Cumulative TSN ACK Point. */
    pub cumulative_tsn_ack_point: __u32,
    /* This is the highest TSN we've marked. */
    pub max_tsn_seen: __u32,
    /* This is the minimum number of TSNs we can track. */
    pub len: __u16,
    /* Data chunks pending receipt. */
    pub pending_data: __u16,
    /* Record duplicate TSNs here. */
    pub num_dup_tsns: __u16,
    pub dup_tsns: [__be32; SCTP_MAX_DUP_TSNS],
}

#[repr(C)]
pub struct sctp_tsnmap_iter {
    pub start: __u32,
}

extern "C" {
    pub fn sctp_tsnmap_init(
        map: *mut sctp_tsnmap,
        len: __u16,
        initial_tsn: __u32,
        gfp: gfp_t,
    ) -> *mut sctp_tsnmap;

    pub fn sctp_tsnmap_free(map: *mut sctp_tsnmap);

    pub fn sctp_tsnmap_check(map: *const sctp_tsnmap, tsn: __u32) -> i32;

    pub fn sctp_tsnmap_mark(
        map: *mut sctp_tsnmap,
        tsn: __u32,
        trans: *mut sctp_transport,
    ) -> i32;

    pub fn sctp_tsnmap_skip(map: *mut sctp_tsnmap, tsn: __u32);

    pub fn sctp_tsnmap_num_gabs(
        map: *mut sctp_tsnmap,
        gabs: *mut sctp_gap_ack_block,
    ) -> __u16;

    pub fn sctp_tsnmap_pending(map: *mut sctp_tsnmap) -> __u16;

    pub fn sctp_tsnmap_renege(map: *mut sctp_tsnmap, tsn: __u32);
}

#[inline]
pub unsafe fn sctp_tsnmap_get_ctsn(map: *const sctp_tsnmap) -> __u32 {
    (*map).cumulative_tsn_ack_point
}

#[inline]
pub unsafe fn sctp_tsnmap_get_max_tsn_seen(map: *const sctp_tsnmap) -> __u32 {
    (*map).max_tsn_seen
}

#[inline]
pub unsafe fn sctp_tsnmap_num_dups(map: *mut sctp_tsnmap) -> __u16 {
    (*map).num_dup_tsns
}

#[inline]
pub unsafe fn sctp_tsnmap_get_dups(map: *mut sctp_tsnmap) -> *mut __be32 {
    (*map).num_dup_tsns = 0;
    (*map).dup_tsns.as_mut_ptr()
}

#[inline]
pub unsafe fn sctp_tsnmap_mark_dup(map: *mut sctp_tsnmap, tsn: __u32) {
    if (*map).num_dup_tsns < SCTP_MAX_DUP_TSNS {
        let index = (*map).num_dup_tsns as usize;
        (*map).dup_tsns[index] = htonl(tsn);
        (*map).num_dup_tsns += 1;
    }
}

#[inline]
pub unsafe fn sctp_tsnmap_has_gap(map: *const sctp_tsnmap) -> i32 {
    ((*map).cumulative_tsn_ack_point != (*map).max_tsn_seen) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
