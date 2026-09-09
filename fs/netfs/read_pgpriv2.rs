// SPDX-License-Identifier: GPL-2.0-only
/* Read with PG_private_2 [DEPRECATED].
 *
 * Copyright (C) 2024 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel dependencies are supplied by other translation units.

/*
 * [DEPRECATED] Copy a folio to the cache with PG_private_2 set.
 */
unsafe fn netfs_pgpriv2_copy_folio(creq: *mut netfs_io_request, folio: *mut folio) {
    let cache = &mut (*creq).io_streams[1];
    let fsize: usize = folio_size(folio);
    let mut flen = fsize;
    let fpos: loff_t = folio_pos(folio);
    let i_size: loff_t;
    let mut to_eof = false;

    _enter("");

    /* netfs_perform_write() may shift i_size around the page or from out
     * of the page to beyond it, but cannot move i_size into or through the
     * page since we have it locked.
     */
    i_size = i_size_read((*creq).inode);

    if fpos >= i_size {
        /* mmap beyond eof. */
        _debug("beyond eof");
        folio_end_private_2(folio);
        return;
    }

    if fpos + fsize as loff_t > (*creq).i_size {
        (*creq).i_size = i_size;
    }

    if flen as loff_t > i_size - fpos {
        flen = (i_size - fpos) as usize;
        to_eof = true;
    } else if flen as loff_t == i_size - fpos {
        to_eof = true;
    }

    _debug("folio %zx %zx", flen, fsize);

    trace_netfs_folio(folio, netfs_folio_trace_store_copy);

    /* Attach the folio to the rolling buffer. */
    if rolling_buffer_append(&mut (*creq).buffer, folio, 0, (*creq).gfp) < 0 {
        folio_end_private_2(folio);
        clear_bit(NETFS_RREQ_FOLIO_COPY_TO_CACHE, &mut (*creq).flags);
        return;
    }

    cache.submit_extendable_to = fsize;
    cache.submit_off = 0;
    cache.submit_len = flen;

    /* Attach the folio to one or more subrequests.  For a big folio, we
     * could end up with thousands of subrequests if the wsize is small -
     * but we might need to wait during the creation of subrequests for
     * network resources (eg. SMB credits).
     */
    loop {
        let part: ssize_t;

        (*creq).buffer.iter.iov_offset = cache.submit_off;

        atomic64_set(&(*creq).issued_to, fpos + cache.submit_off as loff_t);
        cache.submit_extendable_to = fsize - cache.submit_off;
        part = netfs_advance_write(
            creq, cache, fpos + cache.submit_off as loff_t, cache.submit_len, to_eof,
        );
        cache.submit_off += part as usize;
        if part > cache.submit_len as ssize_t {
            cache.submit_len = 0;
        } else {
            cache.submit_len -= part as usize;
        }
        if cache.submit_len <= 0 {
            break;
        }
    }

    (*creq).buffer.iter.iov_offset = 0;
    rolling_buffer_advance(&mut (*creq).buffer, fsize);
    atomic64_set(&(*creq).issued_to, fpos + fsize as loff_t);

    if flen < fsize {
        netfs_issue_write(creq, cache);
    }
}

/* [DEPRECATED] Set up copying to the cache. */
unsafe fn netfs_pgpriv2_begin_copy_to_cache(
    rreq: *mut netfs_io_request,
    folio: *mut folio,
) -> *mut netfs_io_request {
    let creq: *mut netfs_io_request;

    if !fscache_resources_valid(&(*rreq).cache_resources) {
        (*rreq).copy_to_cache = ERR_PTR(-ENOBUFS);
        clear_bit(NETFS_RREQ_FOLIO_COPY_TO_CACHE, &mut (*rreq).flags);
        return ERR_PTR(-ENOBUFS);
    }

    creq = netfs_create_write_req(
        (*rreq).mapping,
        core::ptr::null_mut(),
        folio_pos(folio),
        NETFS_PGPRIV2_COPY_TO_CACHE,
    );
    if IS_ERR(creq) {
        (*rreq).copy_to_cache = ERR_PTR(-ENOBUFS);
        clear_bit(NETFS_RREQ_FOLIO_COPY_TO_CACHE, &mut (*rreq).flags);
        return ERR_PTR(-ENOBUFS);
    }

    if !(*creq).io_streams[1].avail {
        netfs_put_failed_request(creq);
        (*rreq).copy_to_cache = ERR_PTR(-ENOBUFS);
        clear_bit(NETFS_RREQ_FOLIO_COPY_TO_CACHE, &mut (*rreq).flags);
        return ERR_PTR(-ENOBUFS);
    }

    __set_bit(NETFS_RREQ_OFFLOAD_COLLECTION, &mut (*creq).flags);
    trace_netfs_copy2cache(rreq, creq);
    trace_netfs_write(creq, netfs_write_trace_copy_to_cache);
    netfs_stat(&mut netfs_n_wh_copy_to_cache);
    (*rreq).copy_to_cache = creq;
    return creq;
}

/* [DEPRECATED] Mark page as requiring copy-to-cache using PG_private_2 and add
 * it to the copy write request.
 */
pub unsafe fn netfs_pgpriv2_copy_to_cache(
    rreq: *mut netfs_io_request,
    folio: *mut folio,
) {
    let mut creq = (*rreq).copy_to_cache;

    if creq.is_null() {
        creq = netfs_pgpriv2_begin_copy_to_cache(rreq, folio);
    }
    if IS_ERR(creq) {
        return;
    }

    trace_netfs_folio(folio, netfs_folio_trace_copy_to_cache);
    folio_start_private_2(folio);
    netfs_pgpriv2_copy_folio(creq, folio);
}

/* [DEPRECATED] End writing to the cache, flushing out any outstanding writes. */
pub unsafe fn netfs_pgpriv2_end_copy_to_cache(rreq: *mut netfs_io_request) {
    let creq = (*rreq).copy_to_cache;

    if IS_ERR_OR_NULL(creq) {
        return;
    }

    netfs_issue_write(creq, &mut (*creq).io_streams[1]);
    smp_wmb(); /* Write lists before ALL_QUEUED. */
    set_bit(NETFS_RREQ_ALL_QUEUED, &mut (*creq).flags);
    trace_netfs_rreq(rreq, netfs_rreq_trace_end_copy_to_cache);
    if list_empty_careful(&(*creq).io_streams[1].subrequests) {
        netfs_wake_collector(creq);
    }

    netfs_put_request(creq, netfs_rreq_trace_put_return);
    (*rreq).copy_to_cache = core::ptr::null_mut();
}

/* [DEPRECATED] Remove the PG_private_2 mark from any folios we've finished
 * copying.
 */
pub unsafe fn netfs_pgpriv2_unlock_copied_folios(
    creq: *mut netfs_io_request,
) -> bool {
    let mut folioq = (*creq).buffer.tail;
    let collected_to = (*creq).collected_to;
    let mut slot = (*creq).buffer.first_tail_slot;
    let mut made_progress = false;

    if slot >= folioq_nr_slots(folioq) {
        folioq = rolling_buffer_delete_spent(&mut (*creq).buffer);
        slot = 0;
    }

    loop {
        let folio = folioq_folio(folioq, slot);
        if WARN_ONCE(
            !folio_test_private_2(folio),
            "R=%08x: folio %lx is not marked private_2\n",
            (*creq).debug_id,
            (*folio).index,
        ) {
            trace_netfs_folio(folio, netfs_folio_trace_not_under_wback);
        }

        let fpos = folio_pos(folio);
        let fsize = folio_size(folio);
        let flen = fsize;
        let fend = core::cmp::min(fpos + flen as loff_t, (*creq).i_size);

        trace_netfs_collect_folio(creq, folio, fend, collected_to);

        /* Unlock any folio we've transferred all of. */
        if collected_to < fend {
            break;
        }

        trace_netfs_folio(folio, netfs_folio_trace_end_copy);
        folio_end_private_2(folio);
        (*creq).cleaned_to = fpos + fsize as loff_t;
        made_progress = true;

        /* Clean up the head folioq.  If we clear an entire folioq, then
         * we can get rid of it provided it's not also the tail folioq
         * being filled by the issuer.
         */
        folioq_clear(folioq, slot);
        slot += 1;
        if slot >= folioq_nr_slots(folioq) {
            folioq = rolling_buffer_delete_spent(&mut (*creq).buffer);
            if folioq.is_null() {
                (*creq).buffer.first_tail_slot = slot;
                return made_progress;
            }
            slot = 0;
        }

        if fpos + fsize as loff_t >= collected_to {
            break;
        }
    }

    (*creq).buffer.tail = folioq;
    (*creq).buffer.first_tail_slot = slot;
    made_progress
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
