// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation
 * (C) Copyright Red Hat Inc. 2022
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
 *    Xin Long <lucien.xin@gmail.com>
 */

// Fair Capacity and Weighted Fair Queueing handling
// RFC 8260 section 3.5 and 3.6

unsafe fn sctp_sched_fc_unsched_all(stream: *mut sctp_stream);

unsafe fn sctp_sched_wfq_set(
    stream: *mut sctp_stream,
    sid: u16,
    weight: u16,
    _gfp: gfp_t,
) -> i32 {
    let soute = (*SCTP_SO(stream, sid)).ext;

    if weight == 0 {
        return -EINVAL;
    }

    (*soute).fc_weight = weight;
    0
}

unsafe fn sctp_sched_wfq_get(stream: *mut sctp_stream, sid: u16, value: *mut u16) -> i32 {
    let soute = (*SCTP_SO(stream, sid)).ext;

    *value = (*soute).fc_weight;
    0
}

unsafe fn sctp_sched_fc_set(
    _stream: *mut sctp_stream,
    _sid: u16,
    _weight: u16,
    _gfp: gfp_t,
) -> i32 {
    0
}

unsafe fn sctp_sched_fc_get(_stream: *mut sctp_stream, _sid: u16, _value: *mut u16) -> i32 {
    0
}

unsafe fn sctp_sched_fc_init(stream: *mut sctp_stream) -> i32 {
    INIT_LIST_HEAD(&mut (*stream).fc_list);
    0
}

unsafe fn sctp_sched_fc_init_sid(stream: *mut sctp_stream, sid: u16, _gfp: gfp_t) -> i32 {
    let soute = (*SCTP_SO(stream, sid)).ext;

    INIT_LIST_HEAD(&mut (*soute).fc_list);
    (*soute).fc_length = 0;
    (*soute).fc_weight = 1;
    0
}

unsafe fn sctp_sched_fc_free_sid(_stream: *mut sctp_stream, _sid: u16) {}

unsafe fn sctp_sched_fc_sched(stream: *mut sctp_stream, soute: *mut sctp_stream_out_ext) {
    let mut pos: *mut sctp_stream_out_ext;

    if !list_empty(&(*soute).fc_list) {
        return;
    }

    list_for_each_entry!(pos, &(*stream).fc_list, fc_list, {
        if ((*pos).fc_length as u64) * ((*soute).fc_weight as u64)
            >= ((*soute).fc_length as u64) * ((*pos).fc_weight as u64)
        {
            break;
        }
    });
    list_add_tail(&mut (*soute).fc_list, &mut (*pos).fc_list);
}

unsafe fn sctp_sched_fc_enqueue(q: *mut sctp_outq, msg: *mut sctp_datamsg) {
    let stream: *mut sctp_stream;
    let ch: *mut sctp_chunk;
    let sid: u16;

    ch = list_first_entry!(&(*msg).chunks, sctp_chunk, frag_list);
    sid = sctp_chunk_stream_no(ch);
    stream = &mut (*(*q).asoc).stream;
    sctp_sched_fc_sched(stream, (*SCTP_SO(stream, sid)).ext);
}

unsafe fn sctp_sched_fc_dequeue(q: *mut sctp_outq) -> *mut sctp_chunk {
    let stream = &mut (*(*q).asoc).stream;
    let soute: *mut sctp_stream_out_ext;
    let ch: *mut sctp_chunk;

    // Bail out quickly if queue is empty
    if list_empty(&(*q).out_chunk_list) {
        return core::ptr::null_mut();
    }

    // Find which chunk is next
    if !(*stream).out_curr.is_null() {
        soute = (*(*stream).out_curr).ext;
    } else {
        soute = list_entry!((*stream).fc_list.next, sctp_stream_out_ext, fc_list);
    }
    ch = list_entry!((*soute).outq.next, sctp_chunk, stream_list);

    sctp_sched_dequeue_common(q, ch);
    ch
}

unsafe fn sctp_sched_fc_dequeue_done(q: *mut sctp_outq, ch: *mut sctp_chunk) {
    let stream = &mut (*(*q).asoc).stream;
    let soute: *mut sctp_stream_out_ext;
    let mut pos: *mut sctp_stream_out_ext;
    let sid: u16;
    let mut i: u16;

    sid = sctp_chunk_stream_no(ch);
    soute = (*SCTP_SO(stream, sid)).ext;
    // reduce all fc_lengths by U32_MAX / 4 if the current fc_length overflows.
    if (*soute).fc_length > U32_MAX - (*(*ch).skb).len {
        i = 0;
        while i < (*stream).outcnt {
            pos = (*SCTP_SO(stream, i)).ext;
            if !pos.is_null() {
                if (*pos).fc_length <= (U32_MAX >> 2) {
                    (*pos).fc_length = 0;
                } else {
                    (*pos).fc_length -= U32_MAX >> 2;
                }
            }
            i += 1;
        }
    }
    (*soute).fc_length += (*(*ch).skb).len;

    if list_empty(&(*soute).outq) {
        list_del_init(&mut (*soute).fc_list);
        return;
    }

    pos = soute;
    list_for_each_entry_continue!(pos, &(*stream).fc_list, fc_list, {
        if ((*pos).fc_length as u64) * ((*soute).fc_weight as u64)
            >= ((*soute).fc_length as u64) * ((*pos).fc_weight as u64)
        {
            break;
        }
    });
    list_move_tail(&mut (*soute).fc_list, &mut (*pos).fc_list);
}

unsafe fn sctp_sched_fc_sched_all(stream: *mut sctp_stream) {
    let asoc = container_of!(stream, sctp_association, stream);
    let mut ch: *mut sctp_chunk;

    list_for_each_entry!(ch, &(*(*asoc).outqueue).out_chunk_list, list, {
        let sid = sctp_chunk_stream_no(ch);
        if !(*SCTP_SO(stream, sid)).ext.is_null() {
            sctp_sched_fc_sched(stream, (*SCTP_SO(stream, sid)).ext);
        }
    });
}

unsafe fn sctp_sched_fc_unsched_all(stream: *mut sctp_stream) {
    let mut soute: *mut sctp_stream_out_ext;
    let mut tmp: *mut sctp_stream_out_ext;

    list_for_each_entry_safe!(soute, tmp, &(*stream).fc_list, fc_list, {
        list_del_init(&mut (*soute).fc_list);
    });
}

static mut SCTP_SCHED_FC: sctp_sched_ops = sctp_sched_ops {
    set: sctp_sched_fc_set,
    get: sctp_sched_fc_get,
    init: sctp_sched_fc_init,
    init_sid: sctp_sched_fc_init_sid,
    free_sid: sctp_sched_fc_free_sid,
    enqueue: sctp_sched_fc_enqueue,
    dequeue: sctp_sched_fc_dequeue,
    dequeue_done: sctp_sched_fc_dequeue_done,
    sched_all: sctp_sched_fc_sched_all,
    unsched_all: sctp_sched_fc_unsched_all,
};

unsafe fn sctp_sched_ops_fc_init() {
    sctp_sched_ops_register(SCTP_SS_FC, &raw mut SCTP_SCHED_FC);
}

static mut SCTP_SCHED_WFQ: sctp_sched_ops = sctp_sched_ops {
    set: sctp_sched_wfq_set,
    get: sctp_sched_wfq_get,
    init: sctp_sched_fc_init,
    init_sid: sctp_sched_fc_init_sid,
    free_sid: sctp_sched_fc_free_sid,
    enqueue: sctp_sched_fc_enqueue,
    dequeue: sctp_sched_fc_dequeue,
    dequeue_done: sctp_sched_fc_dequeue_done,
    sched_all: sctp_sched_fc_sched_all,
    unsched_all: sctp_sched_fc_unsched_all,
};

unsafe fn sctp_sched_ops_wfq_init() {
    sctp_sched_ops_register(SCTP_SS_WFQ, &raw mut SCTP_SCHED_WFQ);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
