// SPDX-License-Identifier: GPL-2.0-or-later
/* Single, monolithic object support (e.g. AFS directory).
 *
 * Copyright (C) 2024 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/**
 * netfs_single_mark_inode_dirty - Mark a single, monolithic object inode dirty
 * @inode: The inode to mark
 *
 * Mark an inode that contains a single, monolithic object as dirty so that its
 * writepages op will get called.  If set, the SINGLE_NO_UPLOAD flag indicates
 * that the object will only be written to the cache and not uploaded (e.g. AFS
 * directory contents).
 */
pub unsafe extern "C" fn netfs_single_mark_inode_dirty(inode: *mut inode) {
    let ictx = netfs_inode(inode);
    let cache_only = test_bit(NETFS_ICTX_SINGLE_NO_UPLOAD, &(*ictx).flags);
    let caching = fscache_cookie_enabled(netfs_i_cookie(netfs_inode(inode)));

    if cache_only && !caching {
        return;
    }

    mark_inode_dirty(inode);

    if caching && !(inode_state_read_once(inode) & I_PINNING_NETFS_WB != 0) {
        let mut need_use = false;

        spin_lock(&mut (*inode).i_lock);
        if inode_state_read(inode) & I_PINNING_NETFS_WB == 0 {
            inode_state_set(inode, I_PINNING_NETFS_WB);
            need_use = true;
        }
        spin_unlock(&mut (*inode).i_lock);

        if need_use {
            fscache_use_cookie(netfs_i_cookie(ictx), true);
        }
    }
}

unsafe fn netfs_single_begin_cache_read(
    rreq: *mut netfs_io_request,
    ctx: *mut netfs_inode,
) -> c_int {
    fscache_begin_read_operation(&mut (*rreq).cache_resources, netfs_i_cookie(ctx))
}

unsafe fn netfs_single_cache_prepare_read(
    rreq: *mut netfs_io_request,
    subreq: *mut netfs_io_subrequest,
) {
    let cres = &mut (*rreq).cache_resources;

    if cres.ops.is_null() {
        (*subreq).source = NETFS_DOWNLOAD_FROM_SERVER;
        return;
    }
    (*subreq).source = ((*cres).ops).as_ref().unwrap().prepare_read(subreq, (*rreq).i_size);
    trace_netfs_sreq(subreq, netfs_sreq_trace_prepare);
}

unsafe fn netfs_single_read_cache(
    rreq: *mut netfs_io_request,
    subreq: *mut netfs_io_subrequest,
) {
    let cres = &mut (*rreq).cache_resources;

    _enter!("R={:08x}[{:x}]", (*rreq).debug_id, (*subreq).debug_index);
    netfs_stat(&mut netfs_n_rh_read);
    ((*cres).ops).as_ref().unwrap().read(
        cres,
        (*subreq).start,
        &mut (*subreq).io_iter,
        NETFS_READ_HOLE_FAIL,
        netfs_cache_read_terminated,
        subreq,
    );
}

/*
 * Perform a read to a buffer from the cache or the server.  Only a single
 * subreq is permitted as the object must be fetched in a single transaction.
 */
unsafe fn netfs_single_dispatch_read(rreq: *mut netfs_io_request) -> c_int {
    let subreq = netfs_alloc_subrequest(rreq);
    if subreq.is_null() {
        return -ENOMEM;
    }

    (*subreq).source = NETFS_SOURCE_UNKNOWN;
    (*subreq).start = 0;
    (*subreq).len = (*rreq).len;
    (*subreq).io_iter = (*rreq).buffer.iter;

    netfs_queue_read(rreq, subreq);

    netfs_single_cache_prepare_read(rreq, subreq);
    match (*subreq).source {
        NETFS_DOWNLOAD_FROM_SERVER => {
            netfs_stat(&mut netfs_n_rh_download);
            if let Some(prepare_read) = (*rreq).netfs_ops.prepare_read {
                let ret = prepare_read(subreq);
                if ret < 0 {
                    return netfs_single_dispatch_read_cancel(rreq, subreq, ret);
                }
            }

            smp_wmb!(); /* Write lists before ALL_QUEUED. */
            set_bit(NETFS_RREQ_ALL_QUEUED, &mut (*rreq).flags);
            ((*rreq).netfs_ops.issue_read)(subreq);
            (*rreq).submitted += (*subreq).len;
            0
        }
        NETFS_READ_FROM_CACHE => {
            smp_wmb!(); /* Write lists before ALL_QUEUED. */
            set_bit(NETFS_RREQ_ALL_QUEUED, &mut (*rreq).flags);
            trace_netfs_sreq(subreq, netfs_sreq_trace_submit);
            netfs_single_read_cache(rreq, subreq);
            (*rreq).submitted += (*subreq).len;
            0
        }
        _ => {
            pr_warn!("Unexpected single-read source {}\n", (*subreq).source);
            WARN_ON_ONCE!(true);
            netfs_single_dispatch_read_cancel(rreq, subreq, -EIO)
        }
    }
}

unsafe fn netfs_single_dispatch_read_cancel(
    rreq: *mut netfs_io_request,
    subreq: *mut netfs_io_subrequest,
    ret: c_int,
) -> c_int {
    netfs_cancel_read(subreq, ret);
    smp_wmb!(); /* Write lists before ALL_QUEUED. */
    set_bit(NETFS_RREQ_ALL_QUEUED, &mut (*rreq).flags);
    netfs_wake_collector(rreq);
    ret
}

/**
 * netfs_read_single - Synchronously read a single blob of pages.
 * @inode: The inode to read from.
 * @file: The file we're using to read or NULL.
 * @iter: The buffer we're reading into.
 *
 * Fulfil a read request for a single monolithic object by drawing data from
 * the cache if possible, or the netfs if not.  The buffer may be larger than
 * the file content; unused beyond the EOF will be zero-filled.  The content
 * will be read with a single I/O request (though this may be retried).
 *
 * The calling netfs must initialise a netfs context contiguous to the vfs
 * inode before calling this.
 *
 * This is usable whether or not caching is enabled.  If caching is enabled,
 * the data will be stored as a single object into the cache.
 */
pub unsafe extern "C" fn netfs_read_single(
    inode: *mut inode,
    file: *mut file,
    iter: *mut iov_iter,
) -> ssize_t {
    let rreq = netfs_alloc_request((*inode).i_mapping, file, 0, iov_iter_count(iter), NETFS_READ_SINGLE);
    if is_err(rreq) {
        return ptr_err(rreq);
    }

    let ictx = netfs_inode(inode);
    let ret = netfs_single_begin_cache_read(rreq, ictx);
    if ret == -ENOMEM || ret == -EINTR || ret == -ERESTARTSYS {
        netfs_put_failed_request(rreq);
        return ret as ssize_t;
    }

    netfs_stat(&mut netfs_n_rh_read_single);
    trace_netfs_read(rreq, 0, (*rreq).len, netfs_read_trace_read_single);

    (*rreq).buffer.iter = *iter;
    netfs_single_dispatch_read(rreq);

    let ret = netfs_wait_for_read(rreq);
    netfs_put_request(rreq, netfs_rreq_trace_put_return);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
