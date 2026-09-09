// SPDX-License-Identifier: GPL-2.0-only
/* Network filesystem read subrequest retrying.
 *
 * Copyright (C) 2024 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel dependencies and symbols are supplied by the surrounding crate.

unsafe fn netfs_reissue_read(
    _rreq: *mut netfs_io_request,
    subreq: *mut netfs_io_subrequest,
) {
    (*subreq).error = 0;
    __clear_bit(NETFS_SREQ_MADE_PROGRESS, &mut (*subreq).flags);
    __set_bit(NETFS_SREQ_IN_PROGRESS, &mut (*subreq).flags);
    netfs_stat(&netfs_n_rh_retry_read_subreq);
    ((*(*subreq).rreq).netfs_ops.issue_read)(subreq);
}

/*
 * Go through the list of failed/short reads, retrying all retryable ones.  We
 * need to switch failed cache reads to network downloads.
 */
unsafe fn netfs_retry_read_subrequests(rreq: *mut netfs_io_request) {
    let stream = &mut (*rreq).io_streams[0];
    let mut next: *mut list_head;

    _enter!("R={:x}", (*rreq).debug_id);

    if list_empty(&stream.subrequests) { return; }
    if let Some(retry_request) = (*rreq).netfs_ops.retry_request {
        retry_request(rreq, core::ptr::null_mut());
    }

    if (*rreq).netfs_ops.prepare_read.is_none() && (*rreq).cache_resources.ops.is_none() {
        list_for_each_entry!(subreq, &stream.subrequests, rreq_link, {
            if test_bit(NETFS_SREQ_FAILED, &subreq.flags) { break; }
            if __test_and_clear_bit(NETFS_SREQ_NEED_RETRY, &mut subreq.flags) {
                __clear_bit(NETFS_SREQ_MADE_PROGRESS, &mut subreq.flags);
                subreq.retry_count += 1;
                netfs_reset_iter(subreq);
                netfs_get_subrequest(subreq, netfs_sreq_trace_get_resubmit);
                netfs_reissue_read(rreq, subreq);
            }
        });
        return;
    }

    next = stream.subrequests.next;
    loop {
        let mut subreq: *mut netfs_io_subrequest;
        let mut tmp: *mut netfs_io_subrequest;
        let mut source: iov_iter;
        let mut start: u64;
        let mut len: u64;
        let mut part: usize;
        let mut boundary = false;
        let mut subreq_superfluous = false;

        let from = list_entry!(next, netfs_io_subrequest, rreq_link);
        let mut to = from;
        start = (*from).start + (*from).transferred;
        len = (*from).len - (*from).transferred;

        _debug!("from R={:08x}[{:x}] s={:x} ctl={:x}/{:x}",
            (*rreq).debug_id, (*from).debug_index, (*from).start,
            (*from).transferred, (*from).len);

        if test_bit(NETFS_SREQ_FAILED, &(*from).flags) ||
           !test_bit(NETFS_SREQ_NEED_RETRY, &(*from).flags) {
            subreq = from;
            goto_abandon!(rreq, stream, subreq);
        }

        loop {
            next = smp_load_acquire(&mut (*next).next);
            if next == &mut stream.subrequests { break; }
            subreq = list_entry!(next, netfs_io_subrequest, rreq_link);
            if (*subreq).start + (*subreq).transferred != start + len ||
               test_bit(NETFS_SREQ_BOUNDARY, &(*subreq).flags) ||
               !test_bit(NETFS_SREQ_NEED_RETRY, &(*subreq).flags) { break; }
            to = subreq;
            len += (*to).len;
        }

        _debug!(" - range: {:x}-{:x} {:x}", start, start + len - 1, len);
        netfs_reset_iter(from);
        source = (*from).io_iter;
        source.count = len as usize;

        subreq = from;
        list_for_each_entry_from!(subreq, &stream.subrequests, rreq_link, {
            if len == 0 { subreq_superfluous = true; break; }
            (*subreq).source = NETFS_DOWNLOAD_FROM_SERVER;
            (*subreq).start = start - (*subreq).transferred;
            (*subreq).len = len + (*subreq).transferred;
            __clear_bit(NETFS_SREQ_NEED_RETRY, &mut (*subreq).flags);
            __clear_bit(NETFS_SREQ_MADE_PROGRESS, &mut (*subreq).flags);
            (*subreq).retry_count += 1;
            trace_netfs_sreq(subreq, netfs_sreq_trace_retry);
            stream.sreq_max_len = (*subreq).len;
            if let Some(prepare_read) = (*rreq).netfs_ops.prepare_read {
                if prepare_read(subreq) < 0 {
                    trace_netfs_sreq(subreq, netfs_sreq_trace_reprep_failed);
                    __set_bit(NETFS_SREQ_FAILED, &mut (*subreq).flags);
                    goto_abandon!(rreq, stream, subreq);
                }
            }
            part = umin(len, stream.sreq_max_len) as usize;
            if stream.sreq_max_segs != 0 { part = netfs_limit_iter(&mut source, 0, part, stream.sreq_max_segs); }
            (*subreq).len = (*subreq).transferred + part as u64;
            (*subreq).io_iter = source;
            iov_iter_truncate(&mut (*subreq).io_iter, part);
            iov_iter_advance(&mut source, part);
            len -= part as u64; start += part as u64;
            if len == 0 { if boundary { __set_bit(NETFS_SREQ_BOUNDARY, &mut (*subreq).flags); } }
            else { __clear_bit(NETFS_SREQ_BOUNDARY, &mut (*subreq).flags); }
            netfs_get_subrequest(subreq, netfs_sreq_trace_get_resubmit);
            netfs_reissue_read(rreq, subreq);
            if subreq == to { subreq_superfluous = false; break; }
        });

        if len == 0 {
            if !subreq_superfluous { continue; }
            list_for_each_entry_safe_from!(subreq, tmp, &stream.subrequests, rreq_link, {
                trace_netfs_sreq(subreq, netfs_sreq_trace_superfluous);
                spin_lock(&mut (*rreq).lock); list_del(&mut (*subreq).rreq_link); spin_unlock(&mut (*rreq).lock);
                netfs_put_subrequest(subreq, netfs_sreq_trace_put_done);
                if subreq == to { break; }
            });
            continue;
        }

        loop {
            subreq = netfs_alloc_subrequest(rreq);
            if subreq.is_null() { goto_abandon_after!(rreq, stream, to); }
            (*subreq).source = NETFS_DOWNLOAD_FROM_SERVER;
            (*subreq).start = start; (*subreq).len = len;
            (*subreq).stream_nr = stream.stream_nr; (*subreq).retry_count = 1;
            trace_netfs_sreq_ref((*rreq).debug_id, (*subreq).debug_index,
                refcount_read(&(*subreq).ref), netfs_sreq_trace_new);
            spin_lock(&mut (*rreq).lock); list_add(&mut (*subreq).rreq_link, &mut (*to).rreq_link); spin_unlock(&mut (*rreq).lock);
            to = subreq; trace_netfs_sreq(subreq, netfs_sreq_trace_retry);
            stream.sreq_max_len = umin(len, (*rreq).rsize); stream.sreq_max_segs = 0;
            netfs_stat(&netfs_n_rh_download);
            if ((*rreq).netfs_ops.prepare_read.unwrap())(subreq) < 0 {
                trace_netfs_sreq(subreq, netfs_sreq_trace_reprep_failed); __set_bit(NETFS_SREQ_FAILED, &mut (*subreq).flags);
                goto_abandon!(rreq, stream, subreq);
            }
            part = umin(len, stream.sreq_max_len) as usize;
            (*subreq).len = (*subreq).transferred + part as u64; (*subreq).io_iter = source;
            iov_iter_truncate(&mut (*subreq).io_iter, part); iov_iter_advance(&mut source, part);
            len -= part as u64; start += part as u64;
            if len == 0 && boundary { __set_bit(NETFS_SREQ_BOUNDARY, &mut (*to).flags); boundary = false; }
            netfs_reissue_read(rreq, subreq);
            if len == 0 { break; }
        }
    }
}

/* Retry reads. */
pub unsafe fn netfs_retry_reads(rreq: *mut netfs_io_request) {
    let stream = &mut (*rreq).io_streams[0];
    netfs_stat(&netfs_n_rh_retry_read_req);
    set_bit(NETFS_RREQ_RETRYING, &mut (*rreq).flags);
    netfs_wait_for_in_progress_stream(rreq, stream);
    clear_bit(NETFS_RREQ_RETRYING, &mut (*rreq).flags);
    trace_netfs_rreq(rreq, netfs_rreq_trace_resubmit);
    netfs_retry_read_subrequests(rreq);
}

/* Unlock any the pages that haven't been unlocked yet due to abandoned subrequests. */
pub unsafe fn netfs_unlock_abandoned_read_pages(rreq: *mut netfs_io_request) {
    let mut p = (*rreq).buffer.tail;
    while !p.is_null() {
        for slot in 0..folioq_count(p) {
            let folio = folioq_folio(p, slot);
            if !folio.is_null() && !folioq_is_marked2(p, slot) {
                if folio == (*rreq).no_unlock_folio && test_bit(NETFS_RREQ_NO_UNLOCK_FOLIO, &(*rreq).flags) { _debug!("no unlock"); }
                else { trace_netfs_folio(folio, netfs_folio_trace_abandon); folio_unlock(folio); }
            }
        }
        p = (*p).next;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
