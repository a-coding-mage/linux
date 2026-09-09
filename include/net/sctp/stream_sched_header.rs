/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SCTP kernel implementation
 * (C) Copyright Red Hat Inc. 2017
 *
 * These are definitions used by the stream schedulers, defined in RFC
 * draft ndata (https://tools.ietf.org/html/draft-ietf-tsvwg-sctp-ndata-11)
 *
 * Please send any bug reports or fixes you make to the
 * email addresses:
 *    lksctp developers <linux-sctp@vger.kernel.org>
 *
 * Written or modified by:
 *   Marcelo Ricardo Leitner <marcelo.leitner@gmail.com>
 */

#[repr(C)]
pub struct sctp_sched_ops {
    /* Property handling for a given stream */
    pub set: Option<unsafe extern "C" fn(
        stream: *mut sctp_stream,
        sid: __u16,
        value: __u16,
        gfp: gfp_t,
    ) -> ::core::ffi::c_int>,
    pub get: Option<unsafe extern "C" fn(
        stream: *mut sctp_stream,
        sid: __u16,
        value: *mut __u16,
    ) -> ::core::ffi::c_int>,

    /* Init the specific scheduler */
    pub init: Option<unsafe extern "C" fn(stream: *mut sctp_stream) -> ::core::ffi::c_int>,
    /* Init a stream */
    pub init_sid: Option<unsafe extern "C" fn(
        stream: *mut sctp_stream,
        sid: __u16,
        gfp: gfp_t,
    ) -> ::core::ffi::c_int>,
    /* free a stream */
    pub free_sid: Option<unsafe extern "C" fn(stream: *mut sctp_stream, sid: __u16)>,

    /* Enqueue a chunk */
    pub enqueue: Option<unsafe extern "C" fn(q: *mut sctp_outq, msg: *mut sctp_datamsg)>,
    /* Dequeue a chunk */
    pub dequeue: Option<unsafe extern "C" fn(q: *mut sctp_outq) -> *mut sctp_chunk>,
    /* Called only if the chunk fit the packet */
    pub dequeue_done: Option<unsafe extern "C" fn(q: *mut sctp_outq, chunk: *mut sctp_chunk)>,
    /* Schedule all chunks already enqueued */
    pub sched_all: Option<unsafe extern "C" fn(stream: *mut sctp_stream)>,
    /* Unschedule all chunks already enqueued */
    pub unsched_all: Option<unsafe extern "C" fn(stream: *mut sctp_stream)>,
}

extern "C" {
    pub fn sctp_sched_set_sched(
        asoc: *mut sctp_association,
        sched: sctp_sched_type,
    ) -> ::core::ffi::c_int;
    pub fn sctp_sched_get_sched(asoc: *mut sctp_association) -> ::core::ffi::c_int;
    pub fn sctp_sched_set_value(
        asoc: *mut sctp_association,
        sid: __u16,
        value: __u16,
        gfp: gfp_t,
    ) -> ::core::ffi::c_int;
    pub fn sctp_sched_get_value(
        asoc: *mut sctp_association,
        sid: __u16,
        value: *mut __u16,
    ) -> ::core::ffi::c_int;
    pub fn sctp_sched_dequeue_done(q: *mut sctp_outq, ch: *mut sctp_chunk);

    pub fn sctp_sched_dequeue_common(q: *mut sctp_outq, ch: *mut sctp_chunk);
    pub fn sctp_sched_init_sid(
        stream: *mut sctp_stream,
        sid: __u16,
        gfp: gfp_t,
    ) -> ::core::ffi::c_int;
    pub fn sctp_sched_ops_from_stream(stream: *mut sctp_stream) -> *const sctp_sched_ops;

    pub fn sctp_sched_ops_register(
        sched: sctp_sched_type,
        sched_ops: *const sctp_sched_ops,
    );
    pub fn sctp_sched_ops_prio_init();
    pub fn sctp_sched_ops_rr_init();
    pub fn sctp_sched_ops_fc_init();
    pub fn sctp_sched_ops_wfq_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
