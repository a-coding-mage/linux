// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation
 * (C) Copyright Red Hat Inc. 2017
 *
 * This file is part of the SCTP kernel implementation
 *
 * These functions manipulate sctp stream queue/scheduling.
 */

// C dependencies supplied by the surrounding SCTP implementation.
use core::ffi::c_int;

/* First Come First Serve (a.k.a. FIFO), RFC DRAFT ndata Section 3.1 */
unsafe fn sctp_sched_fcfs_set(_stream: *mut sctp_stream, _sid: u16,
                              _value: u16, _gfp: gfp_t) -> c_int { 0 }

unsafe fn sctp_sched_fcfs_get(_stream: *mut sctp_stream, _sid: u16,
                              value: *mut u16) -> c_int {
    *value = 0;
    0
}

unsafe fn sctp_sched_fcfs_init(_stream: *mut sctp_stream) -> c_int { 0 }
unsafe fn sctp_sched_fcfs_init_sid(_stream: *mut sctp_stream, _sid: u16,
                                   _gfp: gfp_t) -> c_int { 0 }
unsafe fn sctp_sched_fcfs_free_sid(_stream: *mut sctp_stream, _sid: u16) {}
unsafe fn sctp_sched_fcfs_enqueue(_q: *mut sctp_outq, _msg: *mut sctp_datamsg) {}

unsafe fn sctp_sched_fcfs_dequeue(q: *mut sctp_outq) -> *mut sctp_chunk {
    let stream = (*q).asoc.as_ref().unwrap().stream.as_ref();
    let mut ch: *mut sctp_chunk = core::ptr::null_mut();

    if list_empty(&(*q).out_chunk_list) {
        return ch;
    }

    if !stream.out_curr.is_null() {
        ch = list_entry((*stream.out_curr).ext.as_ref().unwrap().outq.next,
                        sctp_chunk, stream_list);
    } else {
        ch = list_entry((*q).out_chunk_list.next, sctp_chunk, list);
    }

    sctp_sched_dequeue_common(q, ch);
    ch
}

unsafe fn sctp_sched_fcfs_dequeue_done(_q: *mut sctp_outq,
                                       _chunk: *mut sctp_chunk) {}
unsafe fn sctp_sched_fcfs_sched_all(_stream: *mut sctp_stream) {}
unsafe fn sctp_sched_fcfs_unsched_all(_stream: *mut sctp_stream) {}

static SCTP_SCHED_FCFS: sctp_sched_ops = sctp_sched_ops {
    set: sctp_sched_fcfs_set,
    get: sctp_sched_fcfs_get,
    init: sctp_sched_fcfs_init,
    init_sid: sctp_sched_fcfs_init_sid,
    free_sid: sctp_sched_fcfs_free_sid,
    enqueue: sctp_sched_fcfs_enqueue,
    dequeue: sctp_sched_fcfs_dequeue,
    dequeue_done: sctp_sched_fcfs_dequeue_done,
    sched_all: sctp_sched_fcfs_sched_all,
    unsched_all: sctp_sched_fcfs_unsched_all,
};

unsafe fn sctp_sched_ops_fcfs_init() {
    sctp_sched_ops_register(SCTP_SS_FCFS, &SCTP_SCHED_FCFS);
}

static mut SCTP_SCHED_OPS: [*const sctp_sched_ops; SCTP_SS_MAX + 1] =
    [core::ptr::null(); SCTP_SS_MAX + 1];

pub unsafe fn sctp_sched_ops_register(sched: sctp_sched_type,
                                      sched_ops: *const sctp_sched_ops) {
    SCTP_SCHED_OPS[sched as usize] = sched_ops;
}

pub unsafe fn sctp_sched_ops_init() {
    sctp_sched_ops_fcfs_init();
    sctp_sched_ops_prio_init();
    sctp_sched_ops_rr_init();
    sctp_sched_ops_fc_init();
    sctp_sched_ops_wfq_init();
}

unsafe fn sctp_sched_free_sched(stream: *mut sctp_stream) {
    let sched = sctp_sched_ops_from_stream(stream);
    ((*sched).unsched_all)(stream);
    for i in 0..(*stream).outcnt {
        let soute = sctp_so(stream, i).ext;
        if soute.is_null() { continue; }
        ((*sched).free_sid)(stream, i);
        // C macro memset_after(soute, 0, outq): clear fields after outq.
        memset_after(soute, 0, outq);
    }
}

unsafe fn sctp_sched_set_sched(asoc: *mut sctp_association,
                               sched: sctp_sched_type) -> c_int {
    let old = (*asoc).outqueue.sched;
    let mut msg: *mut sctp_datamsg = core::ptr::null_mut();
    let mut ret: c_int = 0;

    if sched as usize > SCTP_SS_MAX { return -EINVAL; }
    let n = SCTP_SCHED_OPS[sched as usize];
    if old == n { return ret; }
    if !old.is_null() { sctp_sched_free_sched(&mut (*asoc).stream); }

    (*asoc).outqueue.sched = n;
    ((*n).init)(&mut (*asoc).stream);
    for i in 0..(*asoc).stream.outcnt {
        if sctp_so(&mut (*asoc).stream, i).ext.is_null() { continue; }
        ret = ((*n).init_sid)(&mut (*asoc).stream, i, GFP_ATOMIC);
        if ret != 0 { sctp_sched_free_sched(&mut (*asoc).stream); (*asoc).outqueue.sched = &SCTP_SCHED_FCFS; return ret; }
    }

    // C list_for_each_entry over out_chunk_list, enqueueing each new datamsg.
    let mut ch = list_first_entry(&(*asoc).outqueue.out_chunk_list, sctp_chunk, list);
    while !ch.is_null() {
        if (*ch).msg != msg { msg = (*ch).msg; ((*n).enqueue)(&mut (*asoc).outqueue, msg); }
        ch = list_next_entry(ch, list);
    }
    ret
}

unsafe fn sctp_sched_get_sched(asoc: *mut sctp_association) -> c_int {
    for i in 0..=SCTP_SS_MAX {
        if (*asoc).outqueue.sched == SCTP_SCHED_OPS[i] { return i as c_int; }
    }
    0
}

pub unsafe fn sctp_sched_set_value(asoc: *mut sctp_association, sid: u16,
                                   value: u16, gfp: gfp_t) -> c_int {
    if sid as usize >= (*asoc).stream.outcnt { return -EINVAL; }
    if sctp_so(&mut (*asoc).stream, sid as usize).ext.is_null() {
        let ret = sctp_stream_init_ext(&mut (*asoc).stream, sid);
        if ret != 0 { return ret; }
    }
    ((*(*asoc).outqueue.sched).set)(&mut (*asoc).stream, sid, value, gfp)
}

pub unsafe fn sctp_sched_get_value(asoc: *mut sctp_association, sid: u16,
                                   value: *mut u16) -> c_int {
    if sid as usize >= (*asoc).stream.outcnt { return -EINVAL; }
    if sctp_so(&mut (*asoc).stream, sid as usize).ext.is_null() { return 0; }
    ((*(*asoc).outqueue.sched).get)(&mut (*asoc).stream, sid, value)
}

pub unsafe fn sctp_sched_dequeue_done(q: *mut sctp_outq, ch: *mut sctp_chunk) {
    if !list_is_last(&(*ch).frag_list, &(*ch).msg.chunks) &&
       !(*q).asoc.as_ref().unwrap().peer.intl_capable {
        let sid = sctp_chunk_stream_no(ch);
        (*q).asoc.as_mut().unwrap().stream.out_curr = sctp_so(&mut (*q).asoc.as_mut().unwrap().stream, sid as usize);
        return;
    }
    (*q).asoc.as_mut().unwrap().stream.out_curr = core::ptr::null_mut();
    ((*q).sched.as_ref().unwrap().dequeue_done)(q, ch);
}

pub unsafe fn sctp_sched_dequeue_common(q: *mut sctp_outq, ch: *mut sctp_chunk) {
    list_del_init(&mut (*ch).list);
    list_del_init(&mut (*ch).stream_list);
    (*q).out_qlen -= (*ch).skb.as_ref().unwrap().len;
}

unsafe fn sctp_sched_init_sid(stream: *mut sctp_stream, sid: u16, gfp: gfp_t) -> c_int {
    let sched = sctp_sched_ops_from_stream(stream);
    let ext = sctp_so(stream, sid as usize).ext;
    init_list_head(&mut (*ext).outq);
    ((*sched).init_sid)(stream, sid, gfp)
}

unsafe fn sctp_sched_ops_from_stream(stream: *mut sctp_stream) -> *const sctp_sched_ops {
    let asoc = container_of_stream(stream);
    (*asoc).outqueue.sched
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
