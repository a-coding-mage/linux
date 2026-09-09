// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation
 * (C) Copyright IBM Corp. 2003, 2004
 *
 * This file is part of the SCTP kernel implementation
 *
 * This file contains the code relating the chunk abstraction.
 */

/* Dependencies are supplied by the surrounding SCTP implementation. */

/* This file is mostly in anticipation of future work, but initially
 * populate with fragment tracking for an outbound message.
 */

/* Initialize datamsg from memory. */
unsafe fn sctp_datamsg_init(msg: *mut sctp_datamsg) {
    refcount_set(&mut (*msg).refcnt, 1);
    (*msg).send_failed = 0;
    (*msg).send_error = 0;
    (*msg).can_delay = 1;
    (*msg).abandoned = 0;
    (*msg).expires_at = 0;
    INIT_LIST_HEAD(&mut (*msg).chunks);
}

/* Allocate and initialize datamsg. */
unsafe fn sctp_datamsg_new(gfp: gfp_t) -> *mut sctp_datamsg {
    let msg = kmalloc_obj::<sctp_datamsg>(gfp);
    if !msg.is_null() {
        sctp_datamsg_init(msg);
        SCTP_DBG_OBJCNT_INC(datamsg);
    }
    msg
}

pub unsafe fn sctp_datamsg_free(msg: *mut sctp_datamsg) {
    let mut chunk: *mut sctp_chunk;
    /* This doesn't have to be a _safe vairant because
     * sctp_chunk_free() only drops the refs.
     */
    list_for_each_entry!(chunk, &mut (*msg).chunks, frag_list, {
        sctp_chunk_free(chunk);
    });
    sctp_datamsg_put(msg);
}

/* Final destructruction of datamsg memory. */
unsafe fn sctp_datamsg_destroy(msg: *mut sctp_datamsg) {
    let mut asoc: *mut sctp_association = core::ptr::null_mut();
    let (mut pos, mut temp): (*mut list_head, *mut list_head);
    let mut chunk: *mut sctp_chunk;
    let mut ev: *mut sctp_ulpevent;
    let (mut error, mut sent): (i32, i32);

    /* Release all references. */
    list_for_each_safe!(pos, temp, &mut (*msg).chunks, {
        list_del_init(pos);
        chunk = list_entry!(pos, sctp_chunk, frag_list);
        if (*msg).send_failed == 0 {
            sctp_chunk_put(chunk);
            continue;
        }
        asoc = (*chunk).asoc;
        error = if (*msg).send_error != 0 { (*msg).send_error } else { (*asoc).outqueue.error };
        sent = if (*chunk).has_tsn != 0 { SCTP_DATA_SENT } else { SCTP_DATA_UNSENT };
        if sctp_ulpevent_type_enabled((*asoc).subscribe, SCTP_SEND_FAILED) {
            ev = sctp_ulpevent_make_send_failed(asoc, chunk, sent, error, GFP_ATOMIC);
            if !ev.is_null() { (*(*asoc).stream.si).enqueue_event(&mut (*asoc).ulpq, ev); }
        }
        if sctp_ulpevent_type_enabled((*asoc).subscribe, SCTP_SEND_FAILED_EVENT) {
            ev = sctp_ulpevent_make_send_failed_event(asoc, chunk, sent, error, GFP_ATOMIC);
            if !ev.is_null() { (*(*asoc).stream.si).enqueue_event(&mut (*asoc).ulpq, ev); }
        }
        sctp_chunk_put(chunk);
    });
    SCTP_DBG_OBJCNT_DEC(datamsg);
    kfree(msg);
}

/* Hold a reference. */
unsafe fn sctp_datamsg_hold(msg: *mut sctp_datamsg) { refcount_inc(&mut (*msg).refcnt); }

/* Release a reference. */
pub unsafe fn sctp_datamsg_put(msg: *mut sctp_datamsg) {
    if refcount_dec_and_test(&mut (*msg).refcnt) { sctp_datamsg_destroy(msg); }
}

/* Assign a chunk to this datamsg. */
unsafe fn sctp_datamsg_assign(msg: *mut sctp_datamsg, chunk: *mut sctp_chunk) {
    sctp_datamsg_hold(msg);
    (*chunk).msg = msg;
}

/* A data chunk can have a maximum payload of (2^16 - 20).  Break
 * down any such message into smaller chunks.  Opportunistically, fragment
 * the chunks down to the current MTU constraints.  We may get refragmented
 * later if the PMTU changes, but it is _much better_ to fragment immediately
 * with a reasonable guess than always doing our fragmentation on the
 * soft-interrupt.
 */
pub unsafe fn sctp_datamsg_from_user(asoc: *mut sctp_association, sinfo: *mut sctp_sndrcvinfo, from: *mut iov_iter) -> *mut sctp_datamsg {
    let msg_len = iov_iter_count(from);
    let mut shkey: *mut sctp_shared_key = core::ptr::null_mut();
    let (mut pos, mut temp): (*mut list_head, *mut list_head);
    let (mut chunk, msg): (*mut sctp_chunk, *mut sctp_datamsg);
    let mut err: i32;
    msg = sctp_datamsg_new(GFP_KERNEL);
    if msg.is_null() { return ERR_PTR(-ENOMEM); }
    if (*asoc).peer.prsctp_capable != 0 && (*sinfo).sinfo_timetolive != 0 &&
       (SCTP_PR_TTL_ENABLED((*sinfo).sinfo_flags) || !SCTP_PR_POLICY((*sinfo).sinfo_flags)) {
        (*msg).expires_at = jiffies + msecs_to_jiffies((*sinfo).sinfo_timetolive);
    }
    let mut max_data = (*asoc).frag_point;
    if max_data == 0 {
        max_data = sctp_min_frag_point(sctp_sk((*asoc).base.sk), sctp_datachk_len(&(*asoc).stream));
        pr_warn_ratelimited!("%s: asoc:%p frag_point is zero, forcing max_data to default minimum (%zu)", __func__, asoc, max_data);
    }
    if sctp_auth_send_cid(SCTP_CID_DATA, asoc) {
        let hmac_desc = sctp_auth_asoc_get_hmac(asoc);
        if !hmac_desc.is_null() { max_data -= SCTP_PAD4(core::mem::size_of::<sctp_auth_chunk>() + (*hmac_desc).hmac_len); }
        if (*sinfo).sinfo_tsn != 0 && (*sinfo).sinfo_ssn != (*asoc).active_key_id {
            shkey = sctp_auth_get_shkey(asoc, (*sinfo).sinfo_ssn);
            if shkey.is_null() { err = -EINVAL; goto errout; }
        } else { shkey = (*asoc).shkey; }
    }
    let mut first_len = max_data;
    if timer_pending(&(*asoc).timers[SCTP_EVENT_TIMEOUT_SACK]) && (*asoc).outqueue.out_qlen == 0 &&
       list_empty(&(*asoc).outqueue.retransmit) && msg_len > max_data { first_len -= SCTP_PAD4(core::mem::size_of::<sctp_sack_chunk>()); }
    if (*asoc).state < SCTP_STATE_COOKIE_ECHOED { first_len -= SCTP_ARBITRARY_COOKIE_ECHO_LEN; }
    if msg_len >= first_len {
        (*msg).can_delay = 0;
        if msg_len > first_len { SCTP_INC_STATS((*asoc).base.net, SCTP_MIB_FRAGUSRMSGS); }
    } else { first_len = msg_len; }
    let mut remaining = msg_len;
    while remaining != 0 {
        let mut frag = SCTP_DATA_MIDDLE_FRAG;
        let mut len;
        if remaining == msg_len { frag |= SCTP_DATA_FIRST_FRAG; len = first_len; } else { len = max_data; }
        if len >= remaining { len = remaining; frag |= SCTP_DATA_LAST_FRAG; if (*sinfo).sinfo_flags & (SCTP_EOF | SCTP_SACK_IMMEDIATELY) != 0 { frag |= SCTP_DATA_SACK_IMM; } }
        chunk = (*(*asoc).stream.si).make_datafrag(asoc, sinfo, len, frag, GFP_KERNEL);
        if chunk.is_null() { err = -ENOMEM; goto errout; }
        err = sctp_user_addto_chunk(chunk, len, from);
        if err < 0 { sctp_chunk_free(chunk); goto errout; }
        (*chunk).shkey = shkey;
        __skb_pull((*chunk).skb, ((*chunk).chunk_hdr as *mut u8).offset_from((*chunk).skb as *mut u8));
        sctp_datamsg_assign(msg, chunk);
        list_add_tail(&mut (*chunk).frag_list, &mut (*msg).chunks);
        remaining -= len;
    }
    return msg;
errout:
    list_for_each_safe!(pos, temp, &mut (*msg).chunks, { list_del_init(pos); chunk = list_entry!(pos, sctp_chunk, frag_list); sctp_chunk_free(chunk); });
    sctp_datamsg_put(msg);
    return ERR_PTR(err);
}

/* Check whether this message has expired. */
pub unsafe fn sctp_chunk_abandoned(chunk: *mut sctp_chunk) -> i32 {
    if (*(*chunk).asoc).peer.prsctp_capable == 0 { return 0; }
    if (*(*chunk).msg).abandoned != 0 { return 1; }
    if (*chunk).has_tsn == 0 && ((*(*chunk).chunk_hdr).flags & SCTP_DATA_FIRST_FRAG) == 0 { return 0; }
    if SCTP_PR_TTL_ENABLED((*chunk).sinfo.sinfo_flags) && time_after(jiffies, (*(*chunk).msg).expires_at) {
        let streamout = SCTP_SO(&(*(*chunk).asoc).stream, (*chunk).sinfo.sinfo_stream);
        if (*chunk).sent_count != 0 {
            (*(*chunk).asoc).abandoned_sent[SCTP_PR_INDEX(TTL)] += 1;
            (*(*streamout).ext).abandoned_sent[SCTP_PR_INDEX(TTL)] += 1;
        } else {
            (*(*chunk).asoc).abandoned_unsent[SCTP_PR_INDEX(TTL)] += 1;
            (*(*streamout).ext).abandoned_unsent[SCTP_PR_INDEX(TTL)] += 1;
        }
        (*(*chunk).msg).abandoned = 1; return 1;
    } else if SCTP_PR_RTX_ENABLED((*chunk).sinfo.sinfo_flags) && (*chunk).sent_count > (*chunk).sinfo.sinfo_timetolive {
        let streamout = SCTP_SO(&(*(*chunk).asoc).stream, (*chunk).sinfo.sinfo_stream);
        (*(*chunk).asoc).abandoned_sent[SCTP_PR_INDEX(RTX)] += 1;
        (*(*streamout).ext).abandoned_sent[SCTP_PR_INDEX(RTX)] += 1;
        (*(*chunk).msg).abandoned = 1; return 1;
    } else if !SCTP_PR_POLICY((*chunk).sinfo.sinfo_flags) && (*(*chunk).msg).expires_at != 0 && time_after(jiffies, (*(*chunk).msg).expires_at) {
        (*(*chunk).msg).abandoned = 1; return 1;
    }
    /* PRIO policy is processed by sendmsg, not here */
    0
}

/* This chunk (and consequently entire message) has failed in its sending. */
pub unsafe fn sctp_chunk_fail(chunk: *mut sctp_chunk, error: i32) {
    (*(*chunk).msg).send_failed = 1;
    (*(*chunk).msg).send_error = error;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
