// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation
 * (C) Copyright Red Hat Inc. 2017
 *
 * This file is part of the SCTP kernel implementation
 *
 * These functions manipulate sctp stream queue/scheduling.
 *
 * Please send any bug reports or fixes you make to the
 * email addresched(es):
 *    lksctp developers <linux-sctp@vger.kernel.org>
 *
 * Written or modified by:
 *    Marcelo Ricardo Leitner <marcelo.leitner@gmail.com>
 */

/* Priority handling
 * RFC DRAFT ndata section 3.2
 */
unsafe fn sctp_sched_rr_unsched_all(stream: *mut sctp_stream);

unsafe fn sctp_sched_rr_next_stream(stream: *mut sctp_stream) {
    let mut pos = (*(*stream).rr_next).rr_list.next;
    if pos == &mut (*stream).rr_list as *mut list_head {
        pos = (*pos).next;
    }
    (*stream).rr_next = list_entry(pos, sctp_stream_out_ext, rr_list);
}

unsafe fn sctp_sched_rr_unsched(stream: *mut sctp_stream, soute: *mut sctp_stream_out_ext) {
    if (*stream).rr_next == soute {
        sctp_sched_rr_next_stream(stream);
    }

    list_del_init(&mut (*soute).rr_list);

    if list_empty(&(*stream).rr_list) {
        (*stream).rr_next = core::ptr::null_mut();
    }
}

unsafe fn sctp_sched_rr_sched(stream: *mut sctp_stream, soute: *mut sctp_stream_out_ext) {
    if !list_empty(&(*soute).rr_list) {
        return;
    }

    list_add_tail(&mut (*soute).rr_list, &mut (*stream).rr_list);

    if (*stream).rr_next.is_null() {
        (*stream).rr_next = soute;
    }
}

unsafe fn sctp_sched_rr_set(_stream: *mut sctp_stream, _sid: u16, _prio: u16, _gfp: gfp_t) -> i32 {
    0
}

unsafe fn sctp_sched_rr_get(_stream: *mut sctp_stream, _sid: u16, _value: *mut u16) -> i32 {
    0
}

unsafe fn sctp_sched_rr_init(stream: *mut sctp_stream) -> i32 {
    INIT_LIST_HEAD(&mut (*stream).rr_list);
    (*stream).rr_next = core::ptr::null_mut();
    0
}

unsafe fn sctp_sched_rr_init_sid(stream: *mut sctp_stream, sid: u16, _gfp: gfp_t) -> i32 {
    INIT_LIST_HEAD(&mut (*SCTP_SO(stream, sid)).ext.rr_list);
    0
}

unsafe fn sctp_sched_rr_free_sid(_stream: *mut sctp_stream, _sid: u16) {}

unsafe fn sctp_sched_rr_enqueue(q: *mut sctp_outq, msg: *mut sctp_datamsg) {
    let ch = list_first_entry(&(*msg).chunks, sctp_chunk, frag_list);
    let sid = sctp_chunk_stream_no(ch);
    let stream = &mut (*(*q).asoc).stream as *mut sctp_stream;
    sctp_sched_rr_sched(stream, (*SCTP_SO(stream, sid)).ext);
}

unsafe fn sctp_sched_rr_dequeue(q: *mut sctp_outq) -> *mut sctp_chunk {
    let stream = &mut (*(*q).asoc).stream as *mut sctp_stream;
    let soute: *mut sctp_stream_out_ext;
    let mut ch: *mut sctp_chunk = core::ptr::null_mut();

    if list_empty(&(*q).out_chunk_list) {
        return ch;
    }

    if !(*stream).out_curr.is_null() {
        soute = (*(*stream).out_curr).ext;
    } else {
        soute = (*stream).rr_next;
    }
    ch = list_entry((*soute).outq.next, sctp_chunk, stream_list);
    sctp_sched_dequeue_common(q, ch);
    ch
}

unsafe fn sctp_sched_rr_dequeue_done(q: *mut sctp_outq, ch: *mut sctp_chunk) {
    let sid = sctp_chunk_stream_no(ch);
    let stream = &mut (*(*q).asoc).stream as *mut sctp_stream;
    let soute = (*SCTP_SO(stream, sid)).ext;
    sctp_sched_rr_next_stream(stream);
    if list_empty(&(*soute).outq) {
        sctp_sched_rr_unsched(stream, soute);
    }
}

unsafe fn sctp_sched_rr_sched_all(stream: *mut sctp_stream) {
    let asoc = container_of(stream, sctp_association, stream);
    let mut ch = (*asoc).outqueue.out_chunk_list.next;
    while ch != &mut (*asoc).outqueue.out_chunk_list as *mut list_head {
        let chunk = list_entry(ch, sctp_chunk, list);
        let sid = sctp_chunk_stream_no(chunk);
        let soute = (*SCTP_SO(stream, sid)).ext;
        if !soute.is_null() {
            sctp_sched_rr_sched(stream, soute);
        }
        ch = (*ch).next;
    }
}

unsafe fn sctp_sched_rr_unsched_all(stream: *mut sctp_stream) {
    let mut pos = (*stream).rr_list.next;
    while pos != &mut (*stream).rr_list as *mut list_head {
        let next = (*pos).next;
        let soute = list_entry(pos, sctp_stream_out_ext, rr_list);
        sctp_sched_rr_unsched(stream, soute);
        pos = next;
    }
}

static mut SCTP_SCHED_RR: sctp_sched_ops = sctp_sched_ops {
    set: Some(sctp_sched_rr_set),
    get: Some(sctp_sched_rr_get),
    init: Some(sctp_sched_rr_init),
    init_sid: Some(sctp_sched_rr_init_sid),
    free_sid: Some(sctp_sched_rr_free_sid),
    enqueue: Some(sctp_sched_rr_enqueue),
    dequeue: Some(sctp_sched_rr_dequeue),
    dequeue_done: Some(sctp_sched_rr_dequeue_done),
    sched_all: Some(sctp_sched_rr_sched_all),
    unsched_all: Some(sctp_sched_rr_unsched_all),
};

pub unsafe fn sctp_sched_ops_rr_init() {
    sctp_sched_ops_register(SCTP_SS_RR, &raw mut SCTP_SCHED_RR);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
