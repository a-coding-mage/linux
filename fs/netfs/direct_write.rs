// SPDX-License-Identifier: GPL-2.0-or-later
/* Unbuffered and direct write support.
 *
 * Copyright (C) 2023 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Linux dependencies are supplied by the surrounding translation unit. */

/* Perform the cleanup rituals after an unbuffered write is complete. */
unsafe fn netfs_unbuffered_write_done(wreq: *mut netfs_io_request) {
    let ictx = netfs_inode((*wreq).inode);

    _enter!("R=%x", (*wreq).debug_id);
    trace_netfs_rreq(wreq, netfs_rreq_trace_write_done);

    if (*wreq).error == 0 {
        netfs_update_i_size(ictx, &mut (*ictx).inode, (*wreq).start, (*wreq).transferred);
    }

    if (*wreq).origin == NETFS_DIO_WRITE && (*wreq).mapping.nrpages != 0 {
        /* mmap may have got underfoot and we may now have folios locally
         * covering the region we just wrote.  Attempt to discard the folios,
         * but leave in place any modified locally. */
        let first: pgoff_t = (*wreq).start >> PAGE_SHIFT;
        let last: pgoff_t = ((*wreq).start + (*wreq).transferred - 1) >> PAGE_SHIFT;
        invalidate_inode_pages2_range((*wreq).mapping, first, last);
    }

    if (*wreq).origin == NETFS_DIO_WRITE {
        inode_dio_end((*wreq).inode);
    }

    _debug!("finished");
    netfs_wake_rreq_flag(wreq, NETFS_RREQ_IN_PROGRESS, netfs_rreq_trace_wake_ip);

    if !(*wreq).iocb.is_null() {
        let written: size_t = umin((*wreq).transferred, (*wreq).len);
        (*(*wreq).iocb).ki_pos += written;
        if let Some(complete) = (*(*wreq).iocb).ki_complete {
            trace_netfs_rreq(wreq, netfs_rreq_trace_ki_complete);
            complete((*wreq).iocb, if (*wreq).error != 0 { (*wreq).error } else { written });
        }
        (*wreq).iocb = VFS_PTR_POISON;
    }

    netfs_clear_subrequests(wreq);
}

/* Collect the subrequest results of unbuffered write subrequests. */
unsafe fn netfs_unbuffered_write_collect(
    wreq: *mut netfs_io_request,
    stream: *mut netfs_io_stream,
    subreq: *mut netfs_io_subrequest,
) {
    trace_netfs_collect_sreq(wreq, subreq);
    spin_lock(&mut (*wreq).lock);
    list_del_init(&mut (*subreq).rreq_link);
    spin_unlock(&mut (*wreq).lock);

    (*wreq).transferred += (*subreq).transferred;
    iov_iter_advance(&mut (*wreq).buffer.iter, (*subreq).transferred);
    (*stream).collected_to = (*subreq).start + (*subreq).transferred;
    (*wreq).collected_to = (*stream).collected_to;
    netfs_put_subrequest(subreq, netfs_sreq_trace_put_done);
    trace_netfs_collect_stream(wreq, stream);
    trace_netfs_collect_state(wreq, (*wreq).collected_to, 0);
}

/* Write data to the server without going through the pagecache and without
 * writing it to the local cache. */
unsafe fn netfs_unbuffered_write(wreq: *mut netfs_io_request) -> c_int {
    let mut subreq: *mut netfs_io_subrequest = core::ptr::null_mut();
    let stream = &mut (*wreq).io_streams[0];
    let mut ret: c_int = 0;

    _enter!("%llx", (*wreq).len);
    if (*wreq).origin == NETFS_DIO_WRITE { inode_dio_begin((*wreq).inode); }
    stream.collected_to = (*wreq).start;

    loop {
        let mut retry = false;
        if subreq.is_null() {
            netfs_prepare_write(wreq, stream, (*wreq).start + (*wreq).transferred);
            subreq = stream.construct;
            stream.construct = core::ptr::null_mut();
        }
        if unlikely(test_bit(NETFS_SREQ_FAILED, &(*subreq).flags)) {
            netfs_write_subrequest_terminated(subreq, (*subreq).error);
            (*wreq).error = (*subreq).error;
            break;
        }
        iov_iter_truncate(&mut (*subreq).io_iter, (*wreq).len - (*wreq).transferred);
        if iov_iter_count(&(*subreq).io_iter) == 0 { break; }
        (*subreq).len = netfs_limit_iter(&(*subreq).io_iter, 0, stream.sreq_max_len, stream.sreq_max_segs);
        iov_iter_truncate(&mut (*subreq).io_iter, (*subreq).len);
        stream.submit_extendable_to = (*subreq).len;
        trace_netfs_sreq(subreq, netfs_sreq_trace_submit);
        (stream.issue_write)(subreq);
        netfs_wait_for_in_progress_stream(wreq, stream);

        if test_bit(NETFS_SREQ_NEED_RETRY, &(*subreq).flags) { retry = true; }
        else if test_bit(NETFS_SREQ_FAILED, &(*subreq).flags) {
            ret = (*subreq).error; (*wreq).error = ret;
            netfs_see_subrequest(subreq, netfs_sreq_trace_see_failed);
            subreq = core::ptr::null_mut(); break;
        }
        if !retry {
            netfs_unbuffered_write_collect(wreq, stream, subreq); subreq = core::ptr::null_mut();
            if (*wreq).transferred >= (*wreq).len { break; }
            if (*wreq).iocb.is_null() && signal_pending(current) {
                ret = if (*wreq).transferred != 0 { -EINTR } else { -ERESTARTSYS };
                trace_netfs_rreq(wreq, netfs_rreq_trace_intr); break;
            }
            continue;
        }

        (*subreq).error = -EAGAIN;
        trace_netfs_sreq(subreq, netfs_sreq_trace_retry);
        if (*subreq).transferred > 0 {
            iov_iter_advance(&mut (*wreq).buffer.iter, (*subreq).transferred);
            (*wreq).transferred += (*subreq).transferred;
        }
        if stream.source == NETFS_UPLOAD_TO_SERVER && (*wreq).netfs_ops).retry_request.is_some() {
            ((*wreq).netfs_ops).retry_request.unwrap()(wreq, stream);
        }
        __clear_bit(NETFS_SREQ_MADE_PROGRESS, &mut (*subreq).flags);
        __clear_bit(NETFS_SREQ_NEED_RETRY, &mut (*subreq).flags);
        __clear_bit(NETFS_SREQ_BOUNDARY, &mut (*subreq).flags);
        __clear_bit(NETFS_SREQ_FAILED, &mut (*subreq).flags);
        (*subreq).io_iter = (*wreq).buffer.iter;
        (*subreq).start = (*wreq).start + (*wreq).transferred;
        (*subreq).len = (*wreq).len - (*wreq).transferred;
        (*subreq).transferred = 0;
        (*subreq).retry_count += 1;
        stream.sreq_max_len = UINT_MAX; stream.sreq_max_segs = INT_MAX;
        netfs_get_subrequest(subreq, netfs_sreq_trace_get_resubmit);
        if let Some(prepare) = stream.prepare_write { prepare(subreq); }
        __set_bit(NETFS_SREQ_IN_PROGRESS, &mut (*subreq).flags);
        netfs_stat(&mut netfs_n_wh_retry_write_subreq);
    }
    netfs_unbuffered_write_done(wreq);
    _leave!(" = %d", ret);
    ret
}

unsafe fn netfs_unbuffered_write_async(work: *mut work_struct) {
    let wreq = container_of!(work, netfs_io_request, work);
    netfs_unbuffered_write(wreq);
    netfs_put_request(wreq, netfs_rreq_trace_put_complete);
}

/* Perform an unbuffered write where we may have to do an RMW operation on an
 * encrypted file.  This can also be used for direct I/O writes. */
pub unsafe fn netfs_unbuffered_write_iter_locked(
    iocb: *mut kiocb, iter: *mut iov_iter, netfs_group: *mut netfs_group,
) -> ssize_t {
    let _ = netfs_group;
    let start = (*iocb).ki_pos;
    let _end = start + iov_iter_count(&*iter);
    let len = iov_iter_count(&*iter);
    let async_ = !is_sync_kiocb(iocb);
    let mut ret: ssize_t;
    let mut n: ssize_t;
    _enter!("");
    // TODO: Allocate/use a bounce buffer when encryption, compression, or block expansion changes the source data.
    _debug!("uw %llx-%llx", start, _end);
    let wreq = netfs_create_write_req((*(*iocb).ki_filp).f_mapping, (*iocb).ki_filp, start,
        if (*iocb).ki_flags & IOCB_DIRECT != 0 { NETFS_DIO_WRITE } else { NETFS_UNBUFFERED_WRITE });
    if IS_ERR(wreq) { return PTR_ERR(wreq); }
    (*wreq).io_streams[0].avail = true;
    trace_netfs_write(wreq, if (*iocb).ki_flags & IOCB_DIRECT != 0 { netfs_write_trace_dio_write } else { netfs_write_trace_unbuffered_write });
    if user_backed_iter(iter) {
        n = netfs_extract_user_iter(iter, len, &mut (*wreq).buffer.iter, 0);
        if n < 0 { ret = n; netfs_put_failed_request(wreq); return ret; }
        (*wreq).direct_bv = (*wreq).buffer.iter.bvec as *mut bio_vec;
        (*wreq).direct_bv_count = n;
        (*wreq).direct_bv_unpin = iov_iter_extract_will_pin(iter);
    } else { (*wreq).buffer.iter = *iter; }
    (*wreq).len = iov_iter_count(&(*wreq).buffer.iter);
    __set_bit(NETFS_RREQ_USE_IO_ITER, &mut (*wreq).flags);
    // TODO: Copy data into the bounce buffer and encrypt it.
    __set_bit(NETFS_RREQ_UPLOAD_TO_SERVER, &mut (*wreq).flags);
    if async_ {
        INIT_WORK!(&mut (*wreq).work, netfs_unbuffered_write_async);
        (*wreq).iocb = iocb; queue_work(system_dfl_wq, &mut (*wreq).work); ret = -EIOCBQUEUED;
    } else {
        ret = netfs_unbuffered_write(wreq);
        if ret >= 0 { (*iocb).ki_pos += (*wreq).transferred; ret = if (*wreq).transferred != 0 { (*wreq).transferred } else { (*wreq).error }; }
        netfs_put_request(wreq, netfs_rreq_trace_put_complete);
    }
    netfs_put_request(wreq, netfs_rreq_trace_put_return); ret
}

pub unsafe fn netfs_unbuffered_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t {
    let file = (*iocb).ki_filp; let mapping = (*file).f_mapping; let inode = (*mapping).host;
    let ictx = netfs_inode(inode); let pos = (*iocb).ki_pos; let mut ret: ssize_t;
    let mut end = pos + iov_iter_count(&*from) - 1;
    _enter!("%llx,%zx,%llx", pos, iov_iter_count(&*from), i_size_read(inode));
    if iov_iter_count(&*from) == 0 { return 0; }
    trace_netfs_write_iter(iocb, from); netfs_stat(&mut netfs_n_wh_dio_write);
    ret = netfs_start_io_direct(inode); if ret < 0 { return ret; }
    ret = generic_write_checks(iocb, from); if ret <= 0 { netfs_end_io_direct(inode); return ret; }
    ret = file_remove_privs(file); if ret < 0 { netfs_end_io_direct(inode); return ret; }
    ret = file_update_time(file); if ret < 0 { netfs_end_io_direct(inode); return ret; }
    if (*iocb).ki_flags & IOCB_NOWAIT != 0 { ret = -EAGAIN; if filemap_range_has_page(mapping, pos, end) && filemap_invalidate_inode(inode, true, pos, end) { netfs_end_io_direct(inode); return ret; } }
    else { ret = filemap_write_and_wait_range(mapping, pos, end); if ret < 0 { netfs_end_io_direct(inode); return ret; } }
    ret = filemap_invalidate_inode(inode, true, pos, end); if ret < 0 { netfs_end_io_direct(inode); return ret; }
    end = (*iocb).ki_pos + iov_iter_count(&*from); spin_lock(&mut (*inode).i_lock); if end > (*ictx)._zero_point { netfs_write_zero_point(inode, end); } spin_unlock(&mut (*inode).i_lock);
    fscache_invalidate(netfs_i_cookie(ictx), core::ptr::null_mut(), i_size_read(inode), FSCACHE_INVAL_DIO_WRITE);
    ret = netfs_unbuffered_write_iter_locked(iocb, from, core::ptr::null_mut()); netfs_end_io_direct(inode); ret
}

// EXPORT_SYMBOL(netfs_unbuffered_write_iter_locked);
// EXPORT_SYMBOL(netfs_unbuffered_write_iter);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
