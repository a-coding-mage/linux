// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation
 * (C) Copyright Red Hat Inc. 2017
 *
 * This file is part of the SCTP kernel implementation
 *
 * These functions manipulate sctp stream queue/scheduling.
 */

// C dependencies: linux/list.h, net/sctp/sctp.h, net/sctp/sm.h,
// and net/sctp/stream_sched.h provide the referenced types, macros, and functions.

/* Priority handling
 * RFC DRAFT ndata section 3.4
 */

unsafe fn sctp_sched_prio_unsched_all(stream: *mut sctp_stream);

unsafe fn sctp_sched_prio_head_get(p: *mut sctp_stream_priorities) -> *mut sctp_stream_priorities {
    (*p).users += 1;
    p
}

unsafe fn sctp_sched_prio_head_put(p: *mut sctp_stream_priorities) {
    if !p.is_null() {
        (*p).users -= 1;
        if (*p).users == 0 {
            kfree(p);
        }
    }
}

unsafe fn sctp_sched_prio_new_head(
    _stream: *mut sctp_stream,
    prio: i32,
    gfp: gfp_t,
) -> *mut sctp_stream_priorities {
    let p: *mut sctp_stream_priorities = kmalloc_obj(gfp);
    if p.is_null() {
        return core::ptr::null_mut();
    }

    INIT_LIST_HEAD(&mut (*p).prio_sched);
    INIT_LIST_HEAD(&mut (*p).active);
    (*p).next = core::ptr::null_mut();
    (*p).prio = prio;
    (*p).users = 1;
    p
}

unsafe fn sctp_sched_prio_get_head(
    stream: *mut sctp_stream,
    prio: i32,
    gfp: gfp_t,
) -> *mut sctp_stream_priorities {
    let mut p: *mut sctp_stream_priorities;
    let mut i: i32;

    list_for_each_entry!(p, &mut (*stream).prio_list, prio_sched) {
        if (*p).prio == prio {
            return sctp_sched_prio_head_get(p);
        }
        if (*p).prio > prio {
            break;
        }
    }

    i = 0;
    while i < (*stream).outcnt {
        if (*SCTP_SO(stream, i)).ext.is_null() {
            i += 1;
            continue;
        }
        p = (*SCTP_SO(stream, i)).ext.prio_head;
        if p.is_null() {
            break;
        }
        if (*p).prio == prio {
            return sctp_sched_prio_head_get(p);
        }
        i += 1;
    }
    sctp_sched_prio_new_head(stream, prio, gfp)
}

unsafe fn sctp_sched_prio_next_stream(p: *mut sctp_stream_priorities) {
    let mut pos = (*(*p).next).prio_list.next;
    if pos == &mut (*p).active as *mut list_head {
        pos = (*pos).next;
    }
    (*p).next = list_entry!(pos, sctp_stream_out_ext, prio_list);
}

unsafe fn sctp_sched_prio_unsched(soute: *mut sctp_stream_out_ext) -> bool {
    let mut scheduled = false;
    if !list_empty(&(*soute).prio_list) {
        let prio_head = (*soute).prio_head;
        scheduled = true;
        if (*prio_head).next == soute {
            sctp_sched_prio_next_stream(prio_head);
        }
        list_del_init(&mut (*soute).prio_list);
        if list_empty(&(*prio_head).active) {
            list_del_init(&mut (*prio_head).prio_sched);
            (*prio_head).next = core::ptr::null_mut();
        }
    }
    scheduled
}

unsafe fn sctp_sched_prio_sched(stream: *mut sctp_stream, soute: *mut sctp_stream_out_ext) {
    let prio_head = (*soute).prio_head;
    if !list_empty(&(*soute).prio_list) {
        return;
    }
    if !(*prio_head).next.is_null() {
        list_add(&mut (*soute).prio_list, (*(*prio_head).next).prio_list.prev);
        return;
    }
    list_add(&mut (*soute).prio_list, &mut (*prio_head).active);
    (*prio_head).next = soute;
    let mut prio: *mut sctp_stream_priorities;
    list_for_each_entry!(prio, &mut (*stream).prio_list, prio_sched) {
        if (*prio).prio > (*prio_head).prio {
            list_add(&mut (*prio_head).prio_sched, (*prio).prio_sched.prev);
            return;
        }
    }
    list_add_tail(&mut (*prio_head).prio_sched, &mut (*stream).prio_list);
}

unsafe fn sctp_sched_prio_set(stream: *mut sctp_stream, sid: u16, prio: u16, gfp: gfp_t) -> i32 {
    let sout = SCTP_SO(stream, sid);
    let soute = (*sout).ext;
    let old = (*soute).prio_head;
    if !old.is_null() && (*old).prio == prio as i32 { return 0; }
    let prio_head = sctp_sched_prio_get_head(stream, prio as i32, gfp);
    if prio_head.is_null() { return -12; }
    let reschedule = sctp_sched_prio_unsched(soute);
    (*soute).prio_head = prio_head;
    if reschedule { sctp_sched_prio_sched(stream, soute); }
    sctp_sched_prio_head_put(old);
    0
}

unsafe fn sctp_sched_prio_get(stream: *mut sctp_stream, sid: u16, value: *mut u16) -> i32 {
    *value = (*SCTP_SO(stream, sid)).ext.prio_head.prio as u16;
    0
}

unsafe fn sctp_sched_prio_init(stream: *mut sctp_stream) -> i32 {
    INIT_LIST_HEAD(&mut (*stream).prio_list); 0
}

unsafe fn sctp_sched_prio_init_sid(stream: *mut sctp_stream, sid: u16, gfp: gfp_t) -> i32 {
    INIT_LIST_HEAD(&mut (*SCTP_SO(stream, sid)).ext.prio_list);
    sctp_sched_prio_set(stream, sid, 0, gfp)
}

unsafe fn sctp_sched_prio_free_sid(stream: *mut sctp_stream, sid: u16) {
    sctp_sched_prio_head_put((*SCTP_SO(stream, sid)).ext.prio_head);
    (*SCTP_SO(stream, sid)).ext.prio_head = core::ptr::null_mut();
}

unsafe fn sctp_sched_prio_enqueue(q: *mut sctp_outq, msg: *mut sctp_datamsg) {
    let ch = list_first_entry!(&mut (*msg).chunks, sctp_chunk, frag_list);
    let sid = sctp_chunk_stream_no(ch);
    let stream = &mut (*(*q).asoc).stream;
    sctp_sched_prio_sched(stream, (*SCTP_SO(stream, sid)).ext);
}

unsafe fn sctp_sched_prio_dequeue(q: *mut sctp_outq) -> *mut sctp_chunk {
    let stream = &mut (*(*q).asoc).stream;
    let mut ch: *mut sctp_chunk = core::ptr::null_mut();
    if list_empty(&(*q).out_chunk_list) { return ch; }
    let soute;
    if !(*stream).out_curr.is_null() {
        soute = (*(*stream).out_curr).ext;
    } else {
        let prio = list_entry!((*stream).prio_list.next, sctp_stream_priorities, prio_sched);
        soute = (*prio).next;
    }
    ch = list_entry!((*soute).outq.next, sctp_chunk, stream_list);
    sctp_sched_dequeue_common(q, ch);
    ch
}

unsafe fn sctp_sched_prio_dequeue_done(q: *mut sctp_outq, ch: *mut sctp_chunk) {
    let sid = sctp_chunk_stream_no(ch);
    let soute = (*SCTP_SO(&mut (*(*q).asoc).stream, sid)).ext;
    let prio = (*soute).prio_head;
    sctp_sched_prio_next_stream(prio);
    if list_empty(&(*soute).outq) { sctp_sched_prio_unsched(soute); }
}

unsafe fn sctp_sched_prio_sched_all(stream: *mut sctp_stream) {
    let asoc = container_of!(stream, sctp_association, stream);
    let mut ch: *mut sctp_chunk;
    list_for_each_entry!(ch, &mut (*asoc).outqueue.out_chunk_list, list) {
        let sid = sctp_chunk_stream_no(ch);
        let sout = SCTP_SO(stream, sid);
        if !(*sout).ext.is_null() { sctp_sched_prio_sched(stream, (*sout).ext); }
    }
}

unsafe fn sctp_sched_prio_unsched_all(stream: *mut sctp_stream) {
    let mut p: *mut sctp_stream_priorities;
    let mut tmp: *mut sctp_stream_priorities;
    let mut soute: *mut sctp_stream_out_ext;
    let mut souttmp: *mut sctp_stream_out_ext;
    list_for_each_entry_safe!(p, tmp, &mut (*stream).prio_list, prio_sched) {
        list_for_each_entry_safe!(soute, souttmp, &mut (*p).active, prio_list) {
            sctp_sched_prio_unsched(soute);
        }
    }
}

static mut SCTP_SCHED_PRIO: sctp_sched_ops = sctp_sched_ops {
    set: Some(sctp_sched_prio_set), get: Some(sctp_sched_prio_get),
    init: Some(sctp_sched_prio_init), init_sid: Some(sctp_sched_prio_init_sid),
    free_sid: Some(sctp_sched_prio_free_sid), enqueue: Some(sctp_sched_prio_enqueue),
    dequeue: Some(sctp_sched_prio_dequeue), dequeue_done: Some(sctp_sched_prio_dequeue_done),
    sched_all: Some(sctp_sched_prio_sched_all), unsched_all: Some(sctp_sched_prio_unsched_all),
};

pub unsafe fn sctp_sched_ops_prio_init() {
    sctp_sched_ops_register(SCTP_SS_PRIO, &raw const SCTP_SCHED_PRIO);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
