/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SCTP kernel implementation
 * (C) Copyright IBM Corp. 2001, 2004
 * Copyright (c) 1999-2000 Cisco, Inc.
 * Copyright (c) 1999-2001 Motorola, Inc.
 * Copyright (c) 2001 Intel Corp.
 * Copyright (c) 2001 Nokia, Inc.
 * Copyright (c) 2001 La Monte H.P. Yarroll
 *
 * These are the definitions needed for the sctp_ulpq type.  The
 * sctp_ulpq is the interface between the Upper Layer Protocol, or ULP,
 * and the core SCTP state machine.  This is the component which handles
 * reassembly and ordering.
 *
 * Please send any bug reports or fixes you make to the
 * email addresses:
 *    lksctp developers <linux-sctp@vger.kernel.org>
 *
 * Written or modified by:
 *   Jon Grimm             <jgrimm@us.ibm.com>
 *   La Monte H.P. Yarroll <piggy@acm.org>
 *   Sridhar Samudrala     <sri@us.ibm.com>
 */

/* A structure to carry information to the ULP (e.g. Sockets API) */
#[repr(C)]
pub struct sctp_ulpq {
    pub pd_mode: i8,
    pub asoc: *mut sctp_association,
    pub reasm: sk_buff_head,
    pub reasm_uo: sk_buff_head,
    pub lobby: sk_buff_head,
}

/* External types supplied by other translation units. */
#[repr(C)]
pub struct sctp_association {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sk_buff_head {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sctp_chunk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}
pub type gfp_t = u32;
pub type __u16 = u16;
pub type __u32 = u32;

/* Prototypes. */
extern "C" {
    pub fn sctp_ulpq_init(ulpq: *mut sctp_ulpq, asoc: *mut sctp_association);
    pub fn sctp_ulpq_flush(ulpq: *mut sctp_ulpq);
    pub fn sctp_ulpq_free(ulpq: *mut sctp_ulpq);

    /* Add a new DATA chunk for processing. */
    pub fn sctp_ulpq_tail_data(
        ulpq: *mut sctp_ulpq,
        chunk: *mut sctp_chunk,
        gfp: gfp_t,
    ) -> i32;

    /* Add a new event for propagation to the ULP. */
    pub fn sctp_ulpq_tail_event(
        ulpq: *mut sctp_ulpq,
        skb_list: *mut sk_buff_head,
    ) -> i32;

    /* Renege previously received chunks.  */
    pub fn sctp_ulpq_renege(
        ulpq: *mut sctp_ulpq,
        chunk: *mut sctp_chunk,
        gfp: gfp_t,
    );

    /* Perform partial delivery. */
    pub fn sctp_ulpq_partial_delivery(ulpq: *mut sctp_ulpq, gfp: gfp_t);

    /* Abort the partial delivery. */
    pub fn sctp_ulpq_abort_pd(ulpq: *mut sctp_ulpq, gfp: gfp_t);

    /* Clear the partial data delivery condition on this socket. */
    pub fn sctp_clear_pd(sk: *mut sock, asoc: *mut sctp_association) -> i32;

    /* Skip over an SSN. */
    pub fn sctp_ulpq_skip(ulpq: *mut sctp_ulpq, sid: __u16, ssn: __u16);

    pub fn sctp_ulpq_reasm_flushtsn(ulpq: *mut sctp_ulpq, tsn: __u32);

    pub fn sctp_ulpq_renege_list(
        ulpq: *mut sctp_ulpq,
        list: *mut sk_buff_head,
        needed: __u16,
    ) -> __u16;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
