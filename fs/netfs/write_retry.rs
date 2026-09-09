// SPDX-License-Identifier: GPL-2.0-only
/* Network filesystem write retrying.
 *
 * Copyright (C) 2024 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel dependencies and "internal.h" are supplied by the surrounding translation.

/* Perform retries on the streams that need it. */
unsafe fn netfs_retry_write_stream(
    wreq: *mut netfs_io_request,
    stream: *mut netfs_io_stream,
) {
    let mut next: *mut list_head;

    _enter((*wreq).debug_id, (*stream).stream_nr);

    if list_empty(&mut (*stream).subrequests) {
        return;
    }

    if (*stream).source == NETFS_UPLOAD_TO_SERVER
        && (*wreq).netfs_ops.as_ref().unwrap().retry_request.is_some()
    {
        ((*wreq).netfs_ops.as_ref().unwrap().retry_request.unwrap())(wreq, stream);
    }

    if unlikely((*stream).failed) {
        return;
    }

    /* If there's no renegotiation to do, just resend each failed subreq. */
    if (*stream).prepare_write.is_none() {
        let mut subreq: *mut netfs_io_subrequest;
        list_for_each_entry!(subreq, &mut (*stream).subrequests, rreq_link, {
            if test_bit(NETFS_SREQ_FAILED, &(*subreq).flags) {
                break;
            }
            if __test_and_clear_bit(NETFS_SREQ_NEED_RETRY, &mut (*subreq).flags) {
                let mut source: iov_iter;
                netfs_reset_iter(subreq);
                source = (*subreq).io_iter;
                netfs_get_subrequest(subreq, netfs_sreq_trace_get_resubmit);
                netfs_reissue_write(stream, subreq, &mut source);
            }
        });
        return;
    }

    next = (*stream).subrequests.next;

    loop {
        let mut subreq: *mut netfs_io_subrequest = core::ptr::null_mut();
        let mut from: *mut netfs_io_subrequest;
        let mut to: *mut netfs_io_subrequest;
        let mut tmp: *mut netfs_io_subrequest;
        let mut source: iov_iter;
        let mut start: u64;
        let mut len: usize;
        let mut part: usize;
        let mut boundary = false;

        /* Go through the stream and find the next span of contiguous data that we then rejig and reissue. */
        from = list_entry!(next, netfs_io_subrequest, rreq_link);
        to = from;
        start = (*from).start + (*from).transferred;
        len = (*from).len - (*from).transferred;

        if test_bit(NETFS_SREQ_FAILED, &(*from).flags)
            || !test_bit(NETFS_SREQ_NEED_RETRY, &(*from).flags)
        {
            return;
        }

        loop {
            /* Read pointer to subreq before reading subreq state. */
            next = smp_load_acquire(&mut (*next).next);
            if next == &mut (*stream).subrequests {
                break;
            }
            subreq = list_entry!(next, netfs_io_subrequest, rreq_link);
            if (*subreq).start + (*subreq).transferred != start + len as u64
                || test_bit(NETFS_SREQ_BOUNDARY, &(*subreq).flags)
                || !test_bit(NETFS_SREQ_NEED_RETRY, &(*subreq).flags)
            {
                break;
            }
            to = subreq;
            len += (*to).len;
        }

        netfs_reset_iter(from);
        source = (*from).io_iter;
        source.count = len;

        subreq = from;
        list_for_each_entry_from!(subreq, &mut (*stream).subrequests, rreq_link, {
            if len == 0 { break; }
            (*subreq).start = start;
            (*subreq).len = len;
            __clear_bit(NETFS_SREQ_NEED_RETRY, &mut (*subreq).flags);
            trace_netfs_sreq(subreq, netfs_sreq_trace_retry);
            (*stream).sreq_max_len = len;
            ((*stream).prepare_write.unwrap())(subreq);
            part = umin(len, (*stream).sreq_max_len);
            if (*stream).sreq_max_segs != 0 {
                part = netfs_limit_iter(&mut source, 0, part, (*stream).sreq_max_segs);
            }
            (*subreq).len = part;
            (*subreq).transferred = 0;
            len -= part;
            start += part as u64;
            if len != 0 && subreq == to && __test_and_clear_bit(NETFS_SREQ_BOUNDARY, &mut (*to).flags) {
                boundary = true;
            }
            netfs_get_subrequest(subreq, netfs_sreq_trace_get_resubmit);
            netfs_reissue_write(stream, subreq, &mut source);
            if subreq == to { break; }
        });

        if len == 0 {
            if subreq == to { continue; }
            list_for_each_entry_safe_from!(subreq, tmp, &mut (*stream).subrequests, rreq_link, {
                trace_netfs_sreq(subreq, netfs_sreq_trace_discard);
                spin_lock(&mut (*wreq).lock);
                list_del(&mut (*subreq).rreq_link);
                spin_unlock(&mut (*wreq).lock);
                netfs_put_subrequest(subreq, netfs_sreq_trace_put_done);
                if subreq == to { break; }
            });
            continue;
        }

        do {
            subreq = netfs_alloc_subrequest(wreq);
            (*subreq).source = (*to).source;
            (*subreq).start = start;
            (*subreq).stream_nr = (*to).stream_nr;
            (*subreq).retry_count = 1;
            trace_netfs_sreq_ref((*wreq).debug_id, (*subreq).debug_index, refcount_read(&(*subreq).ref), netfs_sreq_trace_new);
            trace_netfs_sreq(subreq, netfs_sreq_trace_split);
            spin_lock(&mut (*wreq).lock);
            list_add(&mut (*subreq).rreq_link, &mut (*to).rreq_link);
            spin_unlock(&mut (*wreq).lock);
            to = subreq;
            trace_netfs_sreq(subreq, netfs_sreq_trace_retry);
            (*stream).sreq_max_len = len;
            (*stream).sreq_max_segs = INT_MAX;
            match (*stream).source {
                NETFS_UPLOAD_TO_SERVER => { netfs_stat(&mut netfs_n_wh_upload); (*stream).sreq_max_len = umin(len, (*wreq).wsize); }
                NETFS_WRITE_TO_CACHE => { netfs_stat(&mut netfs_n_wh_write); }
                _ => { WARN_ON_ONCE(true); }
            }
            ((*stream).prepare_write.unwrap())(subreq);
            part = umin(len, (*stream).sreq_max_len);
            (*subreq).len = (*subreq).transferred + part;
            len -= part;
            start += part as u64;
            if len == 0 && boundary { __set_bit(NETFS_SREQ_BOUNDARY, &mut (*to).flags); boundary = false; }
            netfs_reissue_write(stream, subreq, &mut source);
            if len == 0 { break; }
        } while len != 0;

        if list_is_head(next, &mut (*stream).subrequests) { break; }
    }
}

/* Perform retries on the streams that need it. */
pub unsafe fn netfs_retry_writes(wreq: *mut netfs_io_request) {
    netfs_stat(&mut netfs_n_wh_retry_write_req);
    set_bit(NETFS_RREQ_RETRYING, &mut (*wreq).flags);
    for s in 0..NR_IO_STREAMS {
        let stream = (*wreq).io_streams.as_mut_ptr().add(s);
        if (*stream).active { netfs_wait_for_in_progress_stream(wreq, stream); }
    }
    clear_bit(NETFS_RREQ_RETRYING, &mut (*wreq).flags);
    // TODO: Enc: Fetch changed partial pages
    // TODO: Enc: Reencrypt content if needed.
    // TODO: Enc: Wind back transferred point.
    // TODO: Enc: Mark cache pages for retry.
    for s in 0..NR_IO_STREAMS {
        let stream = (*wreq).io_streams.as_mut_ptr().add(s);
        if (*stream).need_retry {
            (*stream).need_retry = false;
            netfs_retry_write_stream(wreq, stream);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
