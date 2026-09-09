// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file contians vfs address (mmap) ops for 9P2000.
 *
 *  Copyright (C) 2005 by Eric Van Hensbergen <ericvh@gmail.com>
 *  Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
 */

// Linux kernel dependencies supplied by other translation units.

/*
 * Writeback calls this when it finds a folio that needs uploading.  This isn't
 * called if writeback only has copy-to-cache to deal with.
 */
unsafe fn v9fs_begin_writeback(wreq: *mut netfs_io_request) {
    let mut fid: *mut p9_fid;

    fid = v9fs_fid_find_inode((*wreq).inode, true, INVALID_UID, true);
    if fid.is_null() {
        WARN_ONCE!(true, "folio expected an open fid inode->i_ino=%llx\n", (*(*wreq).inode).i_ino);
        return;
    }

    (*wreq).wsize = (*(*fid).clnt).msize - P9_IOHDRSZ;
    if (*fid).iounit != 0 {
        (*wreq).wsize = min((*wreq).wsize, (*fid).iounit);
    }
    (*wreq).netfs_priv = fid as *mut core::ffi::c_void;
    (*wreq).io_streams[0].avail = true;
}

/*
 * Issue a subrequest to write to the server.
 */
unsafe fn v9fs_issue_write(subreq: *mut netfs_io_subrequest) {
    let fid = (*(*subreq).rreq).netfs_priv as *mut p9_fid;
    let mut err: i32 = 0;
    let len: i32;

    len = p9_client_write(fid, (*subreq).start, &mut (*subreq).io_iter, &mut err);
    if len > 0 {
        __set_bit(NETFS_SREQ_MADE_PROGRESS, &mut (*subreq).flags);
    }
    netfs_write_subrequest_terminated(subreq, if len != 0 { len } else { err });
}

/**
 * v9fs_issue_read - Issue a read from 9P
 * @subreq: The read to make
 */
unsafe fn v9fs_issue_read(subreq: *mut netfs_io_subrequest) {
    let rreq = (*subreq).rreq;
    let fid = (*rreq).netfs_priv as *mut p9_fid;
    let mut target: *mut i8;
    let pos = (*subreq).start + (*subreq).transferred;
    let mut total: i32 = 0;
    let mut err: i32 = 0;
    let len: i32;
    let n: usize;

    if S_ISLNK!((*(*rreq).inode).i_mode) {
        /* p9_client_readlink() must not be called for legacy protocols
         * 9p2000 or 9p2000.u.
         */
        BUG_ON!(!p9_is_proto_dotl((*fid).clnt));
        if WARN_ON_ONCE!(pos != 0) {
            /* reading a link at a non null offset should
             * not happen
             */
            err = -EIO;
            goto_fill_subreq!();
        }
        err = p9_client_readlink(fid, &mut target);
        if err != 0 {
            goto_fill_subreq!();
        }
        len = strlen(target);
        n = copy_to_iter(target, len, &mut (*subreq).io_iter);
        kfree(target);
        total = n as i32;
    } else {
        total = p9_client_read(fid, pos, &mut (*subreq).io_iter, &mut err);
    }

    goto_fill_subreq!();

    /* if we just extended the file size, any portion not in
     * cache won't be on server and is zeroes */
    if (*rreq).origin != NETFS_UNBUFFERED_READ && (*rreq).origin != NETFS_DIO_READ {
        __set_bit(NETFS_SREQ_CLEAR_TAIL, &mut (*subreq).flags);
    }
    if pos + total as u64 >= i_size_read((*rreq).inode) {
        __set_bit(NETFS_SREQ_HIT_EOF, &mut (*subreq).flags);
    }
    if err == 0 && total != 0 {
        (*subreq).transferred += total as u64;
        __set_bit(NETFS_SREQ_MADE_PROGRESS, &mut (*subreq).flags);
    }

    (*subreq).error = err;
    netfs_read_subreq_terminated(subreq);
}

/**
 * v9fs_init_request - Initialise a request
 * @rreq: The read request
 * @file: The file being read from
 */
unsafe fn v9fs_init_request(rreq: *mut netfs_io_request, file: *mut file) -> i32 {
    let mut fid: *mut p9_fid;
    let mut dentry: *mut dentry;
    let writing = (*rreq).origin == NETFS_READ_FOR_WRITE ||
        (*rreq).origin == NETFS_WRITETHROUGH ||
        (*rreq).origin == NETFS_UNBUFFERED_WRITE ||
        (*rreq).origin == NETFS_DIO_WRITE;

    if (*rreq).origin == NETFS_WRITEBACK {
        return 0; /* We don't get the write handle until we find we
                   * have actually dirty data and not just
                   * copy-to-cache data.
                   */
    }

    if !file.is_null() {
        fid = (*file).private_data as *mut p9_fid;
        if fid.is_null() {
            goto_no_fid!();
        }
        p9_fid_get(fid);
    } else if S_ISLNK!((*(*rreq).inode).i_mode) {
        dentry = d_find_any_alias((*rreq).inode);
        if dentry.is_null() {
            goto_no_fid!();
        }
        fid = v9fs_fid_lookup(dentry);
        dput(dentry);
        if IS_ERR!(fid) {
            goto_no_fid!();
        }
    } else {
        fid = v9fs_fid_find_inode((*rreq).inode, writing, INVALID_UID, true);
        if fid.is_null() {
            goto_no_fid!();
        }
    }

    (*rreq).wsize = (*(*fid).clnt).msize - P9_IOHDRSZ;
    if (*fid).iounit != 0 {
        (*rreq).wsize = min((*rreq).wsize, (*fid).iounit);
    }

    /* we might need to read from a fid that was opened write-only
     * for read-modify-write of page cache, use the writeback fid
     * for that */
    WARN_ON!((*rreq).origin == NETFS_READ_FOR_WRITE && ((*fid).mode & P9_ORDWR) == 0);
    (*rreq).netfs_priv = fid as *mut core::ffi::c_void;
    return 0;

    // C goto target retained as a local control-flow marker.
    goto_no_fid!();
    WARN_ONCE!(true, "folio expected an open fid inode->i_ino=%llx\n", (*(*rreq).inode).i_ino);
    -EINVAL
}

/**
 * v9fs_free_request - Cleanup request initialized by v9fs_init_rreq
 * @rreq: The I/O request to clean up
 */
unsafe fn v9fs_free_request(rreq: *mut netfs_io_request) {
    let fid = (*rreq).netfs_priv as *mut p9_fid;

    p9_fid_put(fid);
}

const v9fs_req_ops: netfs_request_ops = netfs_request_ops {
    init_request: Some(v9fs_init_request),
    free_request: Some(v9fs_free_request),
    issue_read: Some(v9fs_issue_read),
    begin_writeback: Some(v9fs_begin_writeback),
    issue_write: Some(v9fs_issue_write),
};

const v9fs_addr_operations: address_space_operations = address_space_operations {
    read_folio: Some(netfs_read_folio),
    readahead: Some(netfs_readahead),
    dirty_folio: Some(netfs_dirty_folio),
    release_folio: Some(netfs_release_folio),
    invalidate_folio: Some(netfs_invalidate_folio),
    direct_IO: Some(noop_direct_IO),
    writepages: Some(netfs_writepages),
    migrate_folio: Some(filemap_migrate_folio),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
