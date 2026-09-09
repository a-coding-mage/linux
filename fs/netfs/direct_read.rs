// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct I/O support.
 *
 * Copyright (C) 2023 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel dependencies are supplied by the surrounding translation unit.

unsafe fn netfs_prepare_dio_read_iterator(subreq: *mut netfs_io_subrequest) {
    let rreq = (*subreq).rreq;
    let mut rsize: usize;

    rsize = umin((*subreq).len, (*rreq).io_streams[0].sreq_max_len);
    (*subreq).len = rsize;

    if unlikely((*rreq).io_streams[0].sreq_max_segs != 0) {
        let limit = netfs_limit_iter(
            &mut (*rreq).buffer.iter,
            0,
            rsize,
            (*rreq).io_streams[0].sreq_max_segs,
        );

        if limit < rsize {
            (*subreq).len = limit;
            trace_netfs_sreq(subreq, netfs_sreq_trace_limited);
        }
    }

    trace_netfs_sreq(subreq, netfs_sreq_trace_prepare);

    (*subreq).io_iter = (*rreq).buffer.iter;
    iov_iter_truncate(&mut (*subreq).io_iter, (*subreq).len);
    iov_iter_advance(&mut (*rreq).buffer.iter, (*subreq).len);
}

/*
 * Perform a read to a buffer from the server, slicing up the region to be read
 * according to the network rsize.
 */
unsafe fn netfs_dispatch_unbuffered_reads(rreq: *mut netfs_io_request) {
    let mut start: u64 = (*rreq).start;
    let mut size: isize = (*rreq).len as isize;
    let mut ret: i32;

    loop {
        let subreq: *mut netfs_io_subrequest;
        let slice: isize;

        subreq = netfs_alloc_subrequest(rreq);
        if subreq.is_null() {
            /* Stash the error in the request if there's not
             * already an error set.
             */
            cmpxchg(&mut (*rreq).error, 0, -ENOMEM);
            break;
        }

        (*subreq).source = NETFS_DOWNLOAD_FROM_SERVER;
        (*subreq).start = start;
        (*subreq).len = size as usize;

        netfs_queue_read(rreq, subreq);

        netfs_stat(&netfs_n_rh_download);
        if let Some(prepare_read) = (*rreq).netfs_ops.prepare_read {
            ret = prepare_read(subreq);
            if ret < 0 {
                netfs_cancel_read(subreq, ret);
                break;
            }
        }

        netfs_prepare_dio_read_iterator(subreq);
        slice = (*subreq).len as isize;
        size -= slice;
        start = start.wrapping_add(slice as u64);
        (*rreq).submitted += slice as usize;
        if size <= 0 {
            smp_wmb(); /* Write lists before ALL_QUEUED. */
            set_bit(NETFS_RREQ_ALL_QUEUED, &mut (*rreq).flags);
        }

        ((*rreq).netfs_ops.issue_read)(subreq);

        if test_bit(NETFS_RREQ_PAUSE, &(*rreq).flags) != 0 {
            netfs_wait_for_paused_read(rreq);
        }
        if test_bit(NETFS_RREQ_FAILED, &(*rreq).flags) != 0 {
            break;
        }
        cond_resched();
        if size <= 0 {
            break;
        }
    }

    if unlikely(size > 0) {
        smp_wmb(); /* Write lists before ALL_QUEUED. */
        set_bit(NETFS_RREQ_ALL_QUEUED, &mut (*rreq).flags);
        netfs_wake_collector(rreq);
    }
}

/*
 * Perform a read to an application buffer, bypassing the pagecache and the
 * local disk cache.
 */
unsafe fn netfs_unbuffered_read(rreq: *mut netfs_io_request, sync: bool) -> isize {
    let mut ret: isize;

    _enter!("R=%x %llx-%llx", (*rreq).debug_id, (*rreq).start,
            (*rreq).start + (*rreq).len as u64 - 1);

    if (*rreq).len == 0 {
        pr_err!("Zero-sized read [R=%x]", (*rreq).debug_id);
        netfs_put_request(rreq, netfs_rreq_trace_put_discard);
        return -EIO as isize;
    }

    // TODO: Use bounce buffer if requested

    inode_dio_begin((*rreq).inode);
    netfs_dispatch_unbuffered_reads(rreq);

    /* The collector will get run, even if we don't manage to submit any
     * subreqs, so we shouldn't call inode_dio_end() here.
     */

    if sync {
        ret = netfs_wait_for_read(rreq);
    } else {
        ret = -EIOCBQUEUED as isize;
    }

    _leave!(" = %zd", ret);
    ret
}

/**
 * netfs_unbuffered_read_iter_locked - Perform an unbuffered or direct I/O read
 * @iocb: The I/O control descriptor describing the read
 * @iter: The output buffer (also specifies read length)
 *
 * Perform an unbuffered I/O or direct I/O from the file in @iocb to the
 * output buffer.  No use is made of the pagecache.
 *
 * The caller must hold any appropriate locks.
 */
pub unsafe fn netfs_unbuffered_read_iter_locked(
    iocb: *mut kiocb,
    iter: *mut iov_iter,
) -> isize {
    let rreq: *mut netfs_io_request;
    let mut ret: isize;
    let orig_count = iov_iter_count(iter);
    let sync = is_sync_kiocb(iocb);

    _enter!("");

    if orig_count == 0 {
        return 0; /* Don't update atime */
    }

    ret = kiocb_write_and_wait(iocb, orig_count);
    if ret < 0 {
        return ret;
    }
    file_accessed((*iocb).ki_filp);

    rreq = netfs_alloc_request(
        (*(*iocb).ki_filp).f_mapping,
        (*iocb).ki_filp,
        (*iocb).ki_pos,
        orig_count,
        if (*iocb).ki_flags & IOCB_DIRECT != 0 {
            NETFS_DIO_READ
        } else {
            NETFS_UNBUFFERED_READ
        },
    );
    if is_err(rreq) {
        return ptr_err(rreq);
    }

    netfs_stat(&netfs_n_rh_dio_read);
    trace_netfs_read(rreq, (*rreq).start, (*rreq).len, netfs_read_trace_dio_read);

    if user_backed_iter(iter) {
        ret = netfs_extract_user_iter(iter, (*rreq).len, &mut (*rreq).buffer.iter, 0);
        if ret < 0 {
            netfs_put_failed_request(rreq);
            return ret;
        }
        (*rreq).direct_bv = (*rreq).buffer.iter.bvec as *mut bio_vec;
        (*rreq).direct_bv_count = ret;
        (*rreq).direct_bv_unpin = iov_iter_extract_will_pin(iter);
        (*rreq).len = iov_iter_count(&mut (*rreq).buffer.iter);
    } else {
        (*rreq).buffer.iter = *iter;
        (*rreq).len = orig_count;
        (*rreq).direct_bv_unpin = false;
        iov_iter_advance(iter, orig_count);
    }

    // TODO: Set up bounce buffer if needed

    if !sync {
        (*rreq).iocb = iocb;
        __set_bit(NETFS_RREQ_OFFLOAD_COLLECTION, &mut (*rreq).flags);
    }

    ret = netfs_unbuffered_read(rreq, sync);
    if ret < 0 {
        netfs_put_request(rreq, netfs_rreq_trace_put_return);
        return ret;
    }
    if sync {
        // TODO: Copy from bounce buffer
        (*iocb).ki_pos += (*rreq).transferred as u64;
        ret = (*rreq).transferred as isize;
    }

    netfs_put_request(rreq, netfs_rreq_trace_put_return);
    ret
}

/**
 * netfs_unbuffered_read_iter - Perform an unbuffered or direct I/O read
 * @iocb: The I/O control descriptor describing the read
 * @iter: The output buffer (also specifies read length)
 *
 * Perform an unbuffered I/O or direct I/O from the file in @iocb to the
 * output buffer.  No use is made of the pagecache.
 */
pub unsafe fn netfs_unbuffered_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize {
    let inode = file_inode((*iocb).ki_filp);
    let mut ret: isize;

    if (*iter).count == 0 {
        return 0; /* Don't update atime */
    }

    ret = netfs_start_io_direct(inode);
    if ret == 0 {
        ret = netfs_unbuffered_read_iter_locked(iocb, iter);
        netfs_end_io_direct(inode);
    }
    ret
}

// EXPORT_SYMBOL(netfs_unbuffered_read_iter_locked);
// EXPORT_SYMBOL(netfs_unbuffered_read_iter);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
