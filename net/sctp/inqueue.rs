// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation
 * Copyright (c) 1999-2000 Cisco, Inc.
 * Copyright (c) 1999-2001 Motorola, Inc.
 * Copyright (c) 2002 International Business Machines, Corp.
 *
 * This file is part of the SCTP kernel implementation
 *
 * These functions are the methods for accessing the SCTP inqueue.
 *
 * An SCTP inqueue is a queue into which you push SCTP packets
 * (which might be bundles or fragments of chunks) and out of which you
 * pop SCTP whole chunks.
 */

// Dependencies supplied by the SCTP and Linux kernel compatibility layers.

pub unsafe fn sctp_inq_init(queue: *mut sctp_inq) {
    INIT_LIST_HEAD(&mut (*queue).in_chunk_list);
    (*queue).in_progress = core::ptr::null_mut();

    /* Create a task for delivering data. */
    INIT_WORK(&mut (*queue).immediate, None);
}

#[inline]
unsafe fn sctp_inq_chunk_free(chunk: *mut sctp_chunk) {
    if !(*chunk).head_skb.is_null() {
        (*chunk).skb = (*chunk).head_skb;
    }
    sctp_chunk_free(chunk);
}

pub unsafe fn sctp_inq_free(queue: *mut sctp_inq) {
    let mut chunk: *mut sctp_chunk;
    let mut tmp: *mut sctp_chunk;

    /* Empty the queue. */
    list_for_each_entry_safe!(chunk, tmp, &mut (*queue).in_chunk_list, list, {
        list_del_init(&mut (*chunk).list);
        sctp_chunk_free(chunk);
    });

    /* If there is a packet which is currently being worked on, free it as well. */
    if !(*queue).in_progress.is_null() {
        sctp_inq_chunk_free((*queue).in_progress);
        (*queue).in_progress = core::ptr::null_mut();
    }
}

pub unsafe fn sctp_inq_push(q: *mut sctp_inq, chunk: *mut sctp_chunk) {
    /* Directly call the packet handling routine. */
    if (*(*chunk).rcvr).dead || (!(*chunk).transport.is_null() && (*(*chunk).transport).dead) {
        sctp_chunk_free(chunk);
        return;
    }

    list_add_tail(&mut (*chunk).list, &mut (*q).in_chunk_list);
    if !(*chunk).asoc.is_null() {
        (*(*chunk).asoc).stats.ipackets = (*(*chunk).asoc).stats.ipackets.wrapping_add(1);
    }
    ((*q).immediate.func.unwrap())(&mut (*q).immediate);
}

pub unsafe fn sctp_inq_peek(queue: *mut sctp_inq) -> *mut sctp_chunkhdr {
    let chunk = (*queue).in_progress;
    if (*chunk).singleton || (*chunk).end_of_packet || (*chunk).pdiscard {
        return core::ptr::null_mut();
    }
    (*chunk).chunk_end as *mut sctp_chunkhdr
}

pub unsafe fn sctp_inq_pop(queue: *mut sctp_inq) -> *mut sctp_chunk {
    let mut chunk = (*queue).in_progress;
    let mut ch: *mut sctp_chunkhdr = core::ptr::null_mut();

    if !chunk.is_null() {
        if (*chunk).singleton || (*chunk).end_of_packet || (*chunk).pdiscard {
            if (*chunk).head_skb == (*chunk).skb {
                (*chunk).skb = skb_shinfo((*chunk).skb).frag_list;
                ch = (*(*chunk).skb).data as *mut sctp_chunkhdr;
                (*chunk).singleton = 1;
                (*chunk).data_accepted = 0;
                (*chunk).pdiscard = 0;
                (*chunk).auth = 0;
                (*chunk).has_asconf = 0;
                (*chunk).end_of_packet = 0;
            } else if !(*(*chunk).skb).next.is_null() {
                (*chunk).skb = (*(*chunk).skb).next;
                ch = (*(*chunk).skb).data as *mut sctp_chunkhdr;
                (*chunk).singleton = 1;
                (*chunk).data_accepted = 0;
                (*chunk).pdiscard = 0;
                (*chunk).auth = 0;
                (*chunk).has_asconf = 0;
                (*chunk).end_of_packet = 0;
            } else {
                sctp_inq_chunk_free(chunk);
                (*queue).in_progress = core::ptr::null_mut();
                chunk = core::ptr::null_mut();
            }
        } else {
            ch = (*chunk).chunk_end as *mut sctp_chunkhdr;
            skb_pull((*chunk).skb, (*chunk).chunk_end.offset_from((*(*chunk).skb).data) as u32);
        }
    }

    if chunk.is_null() {
        loop {
            let entry = sctp_list_dequeue(&mut (*queue).in_chunk_list);
            if entry.is_null() { return core::ptr::null_mut(); }
            chunk = list_entry!(entry, sctp_chunk, list);
            if skb_is_gso((*chunk).skb) && skb_is_gso_sctp((*chunk).skb) {
                if !skb_shinfo((*chunk).skb).frag_list.is_null() { (*chunk).head_skb = (*chunk).skb; }
                if !(*chunk).head_skb.is_null() && (*(*chunk).skb).data_len == (*(*chunk).skb).len {
                    if !skb_shinfo((*chunk).skb).frag_list.is_null() {
                        (*chunk).skb = skb_shinfo((*chunk).skb).frag_list;
                    } else {
                        sctp_chunk_free(chunk);
                        continue;
                    }
                }
            }
            if !(*chunk).asoc.is_null() { sock_rps_save_rxhash((*(*chunk).asoc).base.sk, (*chunk).skb); }
            (*queue).in_progress = chunk;
            ch = (*(*chunk).skb).data as *mut sctp_chunkhdr;
            (*chunk).singleton = 1;
            (*chunk).data_accepted = 0;
            (*chunk).pdiscard = 0;
            (*chunk).auth = 0;
            (*chunk).has_asconf = 0;
            (*chunk).end_of_packet = 0;
            break;
        }
    }

    if !(*chunk).head_skb.is_null() {
        let cb = SCTP_INPUT_CB((*chunk).skb);
        let head_cb = SCTP_INPUT_CB((*chunk).head_skb);
        (*cb).chunk = (*head_cb).chunk;
        (*cb).af = (*head_cb).af;
        (*cb).encap_port = (*head_cb).encap_port;
    }

    (*chunk).chunk_hdr = ch;
    (*chunk).chunk_end = (ch as *mut u8).add(SCTP_PAD4(ntohs((*ch).length)) as usize);
    skb_pull((*chunk).skb, core::mem::size_of::<sctp_chunkhdr>() as u32);
    (*chunk).subh.v = core::ptr::null_mut();
    if (*chunk).chunk_end.add(core::mem::size_of::<sctp_chunkhdr>()) <= skb_tail_pointer((*chunk).skb) {
        (*chunk).singleton = 0;
    } else if (*chunk).chunk_end > skb_tail_pointer((*chunk).skb) {
        (*chunk).pdiscard = 1;
        (*chunk).chunk_end = skb_tail_pointer((*chunk).skb);
    } else {
        (*chunk).end_of_packet = 1;
    }
    pr_debug!(
        "+++sctp_inq_pop+++ chunk:%p[%s], length:%d, skb->len:%d\n",
        chunk,
        sctp_cname(SCTP_ST_CHUNK((*chunk).chunk_hdr).type),
        ntohs((*chunk).chunk_hdr).length,
        (*(*chunk).skb).len
    );
    chunk
}

pub unsafe fn sctp_inq_set_th_handler(q: *mut sctp_inq, callback: work_func_t) {
    INIT_WORK(&mut (*q).immediate, Some(callback));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
