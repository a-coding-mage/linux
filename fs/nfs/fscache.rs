// SPDX-License-Identifier: GPL-2.0-or-later
/* NFS filesystem cache interface
 *
 * Copyright (C) 2008 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel headers and local headers from the C source are external Rust dependencies.

const NFS_MAX_KEY_LEN: usize = 1000;

unsafe fn nfs_append_int(key: *mut u8, len: *mut i32, x: u64) -> bool {
    if *len as usize > NFS_MAX_KEY_LEN {
        return false;
    }
    if x == 0 {
        *key.add(*len as usize) = b',';
        *len += 1;
    } else {
        // Equivalent to sprintf(key + *_len, ",%llx", x).
        *len += sprintf_hex(key.add(*len as usize), x);
    }
    true
}

/*
 * Get the per-client index cookie for an NFS client if the appropriate mount
 * flag was set
 * - We always try and get an index cookie for the client, but get filehandle
 *   cookies on a per-superblock basis, depending on the mount flags
 */
unsafe fn nfs_fscache_get_client_key(
    clp: *mut nfs_client,
    key: *mut u8,
    len: *mut i32,
) -> bool {
    let sin6 = &(*(clp)).cl_addr as *const _ as *const sockaddr_in6;
    let sin = &(*(clp)).cl_addr as *const _ as *const sockaddr_in;

    *len += snprintf_client_key(
        key.add(*len as usize),
        NFS_MAX_KEY_LEN - *len as usize,
        (*(*clp).rpc_ops).version,
        (*clp).cl_minorversion,
        (*clp).cl_addr.ss_family,
    );

    match (*clp).cl_addr.ss_family {
        AF_INET => {
            if !nfs_append_int(key, len, (*sin).sin_port as u64)
                || !nfs_append_int(key, len, (*sin).sin_addr.s_addr as u64)
            {
                return false;
            }
            true
        }
        AF_INET6 => {
            if !nfs_append_int(key, len, (*sin6).sin6_port as u64)
                || !nfs_append_int(key, len, (*sin6).sin6_addr.s6_addr32[0] as u64)
                || !nfs_append_int(key, len, (*sin6).sin6_addr.s6_addr32[1] as u64)
                || !nfs_append_int(key, len, (*sin6).sin6_addr.s6_addr32[2] as u64)
                || !nfs_append_int(key, len, (*sin6).sin6_addr.s6_addr32[3] as u64)
            {
                return false;
            }
            true
        }
        _ => {
            printk_warning((*clp).cl_addr.ss_family);
            false
        }
    }
}

/*
 * Get the cache cookie for an NFS superblock.
 *
 * The default uniquifier is just an empty string, but it may be overridden
 * either by the 'fsc=xxx' option to mount, or by inheriting it from the parent
 * superblock across an automount point of some nature.
 */
unsafe fn nfs_fscache_get_super_cookie(sb: *mut super_block, uniq: *const u8, ulen: i32) -> i32 {
    let mut vcookie: *mut fscache_volume;
    let nfss = NFS_SB(sb);
    let mut len: i32 = 3;
    let key = kmalloc(NFS_MAX_KEY_LEN + 24, GFP_KERNEL);

    if !uniq.is_null() {
        (*nfss).fscache_uniq = kmemdup_nul(uniq, ulen, GFP_KERNEL);
        if (*nfss).fscache_uniq.is_null() {
            return -ENOMEM;
        }
    }
    if key.is_null() {
        return -ENOMEM;
    }
    memcpy(key, b"nfs".as_ptr(), 3);
    if !nfs_fscache_get_client_key((*nfss).nfs_client, key, &mut len)
        || !nfs_append_int(key, &mut len, (*nfss).fsid.major as u64)
        || !nfs_append_int(key, &mut len, (*nfss).fsid.minor as u64)
        || !nfs_append_int(key, &mut len, ((*sb).s_flags & NFS_SB_MASK) as u64)
        || !nfs_append_int(key, &mut len, (*nfss).flags as u64)
        || !nfs_append_int(key, &mut len, (*nfss).rsize as u64)
        || !nfs_append_int(key, &mut len, (*nfss).wsize as u64)
        || !nfs_append_int(key, &mut len, (*nfss).acregmin as u64)
        || !nfs_append_int(key, &mut len, (*nfss).acregmax as u64)
        || !nfs_append_int(key, &mut len, (*nfss).acdirmin as u64)
        || !nfs_append_int(key, &mut len, (*nfss).acdirmax as u64)
        || !nfs_append_int(key, &mut len, (*(*nfss).client).cl_auth.au_flavor as u64)
    {
        kfree(key);
        return 0;
    }
    if ulen > 0 {
        if ulen as usize > NFS_MAX_KEY_LEN - len as usize {
            kfree(key);
            return 0;
        }
        *key.add(len as usize) = b',';
        len += 1;
        memcpy(key.add(len as usize), uniq, ulen as usize);
        len += ulen;
    }
    *key.add(len as usize) = 0;

    vcookie = fscache_acquire_volume(key, core::ptr::null_mut(), core::ptr::null(), 0);
    if IS_ERR(vcookie) {
        if vcookie != ERR_PTR(-EBUSY) {
            kfree(key);
            return PTR_ERR(vcookie);
        }
        pr_err_cache_volume_key_in_use(key);
        vcookie = core::ptr::null_mut();
    }
    (*nfss).fscache = vcookie;
    kfree(key);
    0
}

/* release a per-superblock cookie */
unsafe fn nfs_fscache_release_super_cookie(sb: *mut super_block) {
    let nfss = NFS_SB(sb);
    fscache_relinquish_volume((*nfss).fscache, core::ptr::null(), false);
    (*nfss).fscache = core::ptr::null_mut();
    kfree((*nfss).fscache_uniq);
}

/* Initialise the per-inode cache cookie pointer for an NFS inode. */
unsafe fn nfs_fscache_init_inode(inode: *mut inode) {
    let mut auxdata: nfs_fscache_inode_auxdata = core::mem::zeroed();
    let nfss = NFS_SERVER(inode);
    let nfsi = NFS_I(inode);
    netfs_inode(inode).cache = core::ptr::null_mut();
    if (*nfss).fscache.is_null() || !S_ISREG((*inode).i_mode) { return; }
    nfs_fscache_update_auxdata(&mut auxdata, inode);
    netfs_inode(inode).cache = fscache_acquire_cookie(
        (*nfss).fscache, 0, (*nfsi).fh.data, (*nfsi).fh.size,
        &auxdata as *const _ as *const _, core::mem::size_of_val(&auxdata), i_size_read(inode));
    if !netfs_inode(inode).cache.is_null() { mapping_set_release_always((*inode).i_mapping); }
}

/* Release a per-inode cookie. */
unsafe fn nfs_fscache_clear_inode(inode: *mut inode) {
    fscache_relinquish_cookie(netfs_i_cookie(netfs_inode(inode)), false);
    netfs_inode(inode).cache = core::ptr::null_mut();
}

unsafe fn nfs_fscache_open_file(inode: *mut inode, _filp: *mut file) {
    let mut auxdata: nfs_fscache_inode_auxdata = core::mem::zeroed();
    let cookie = netfs_i_cookie(netfs_inode(inode));
    let open_for_write = inode_is_open_for_write(inode);
    if !fscache_cookie_valid(cookie) { return; }
    fscache_use_cookie(cookie, open_for_write);
    if open_for_write {
        nfs_fscache_update_auxdata(&mut auxdata, inode);
        fscache_invalidate(cookie, &auxdata, i_size_read(inode), FSCACHE_INVAL_DIO_WRITE);
    }
}

unsafe fn nfs_fscache_release_file(inode: *mut inode, _filp: *mut file) {
    let mut auxdata: nfs_fscache_inode_auxdata = core::mem::zeroed();
    let cookie = netfs_i_cookie(netfs_inode(inode));
    let mut i_size = i_size_read(inode);
    nfs_fscache_update_auxdata(&mut auxdata, inode);
    fscache_unuse_cookie(cookie, &auxdata, &mut i_size);
}

unsafe fn nfs_netfs_read_folio(file: *mut file, folio: *mut folio) -> i32 {
    if netfs_inode(folio_inode(folio)).cache.is_null() { return -ENOBUFS; }
    netfs_read_folio(file, folio)
}

unsafe fn nfs_netfs_readahead(ractl: *mut readahead_control) -> i32 {
    let inode = (*(*ractl).mapping).host;
    if netfs_inode(inode).cache.is_null() { return -ENOBUFS; }
    netfs_readahead(ractl);
    0
}

static mut nfs_netfs_debug_id: atomic_t = atomic_t { counter: 0 };

unsafe fn nfs_netfs_init_request(rreq: *mut netfs_io_request, file: *mut file) -> i32 {
    if file.is_null() {
        if WARN_ON_ONCE((*rreq).origin != NETFS_PGPRIV2_COPY_TO_CACHE) { return -EIO; }
        return 0;
    }
    (*rreq).netfs_priv = get_nfs_open_context(nfs_file_open_context(file));
    (*rreq).debug_id = atomic_inc_return(&mut nfs_netfs_debug_id);
    // [DEPRECATED] Use PG_private_2 to mark folio being written to the cache.
    __set_bit(NETFS_RREQ_USE_PGPRIV2, &mut (*rreq).flags);
    (*rreq).io_streams[0].sreq_max_len = (*NFS_SB((*rreq).inode).as_ref()).rsize;
    0
}

unsafe fn nfs_netfs_free_request(rreq: *mut netfs_io_request) {
    if !(*rreq).netfs_priv.is_null() { put_nfs_open_context((*rreq).netfs_priv); }
}

unsafe fn nfs_netfs_alloc(sreq: *mut netfs_io_subrequest) -> *mut nfs_netfs_io_data {
    let netfs = kzalloc_obj::<nfs_netfs_io_data>(GFP_KERNEL_ACCOUNT);
    if netfs.is_null() { return core::ptr::null_mut(); }
    (*netfs).sreq = sreq;
    refcount_set(&mut (*netfs).refcount, 1);
    netfs
}

unsafe fn nfs_netfs_issue_read(sreq: *mut netfs_io_subrequest) {
    let rreq = (*sreq).rreq;
    let inode = (*rreq).inode;
    let ctx = (*rreq).netfs_priv;
    let start = ((*sreq).start + (*sreq).transferred) >> PAGE_SHIFT;
    let last = ((*sreq).start + (*sreq).len - (*sreq).transferred - 1) >> PAGE_SHIFT;
    let mut pgio: nfs_pageio_descriptor = core::mem::zeroed();
    nfs_pageio_init_read(&mut pgio, inode, false, &nfs_async_read_completion_ops);
    let netfs = nfs_netfs_alloc(sreq);
    if netfs.is_null() { (*sreq).error = -ENOMEM; return netfs_read_subreq_terminated(sreq); }
    pgio.pg_netfs = netfs;
    let mut idx: usize = 0;
    let mut page: *mut page = core::ptr::null_mut();
    xa_for_each_range(&(*rreq).mapping.i_pages, &mut idx, &mut page, start, last, |page| {
        nfs_read_add_folio(&mut pgio, ctx, page_folio(page))
    });
    nfs_pageio_complete_read(&mut pgio);
    nfs_netfs_put(netfs);
}

unsafe fn nfs_netfs_initiate_read(hdr: *mut nfs_pgio_header) {
    let netfs = (*hdr).netfs;
    if !netfs.is_null() { nfs_netfs_get(netfs); }
}

unsafe fn nfs_netfs_folio_unlock(folio: *mut folio) -> i32 {
    let inode = (*(*folio).mapping).host;
    if !netfs_inode(inode).cache.is_null() { return 0; }
    1
}

unsafe fn nfs_netfs_read_completion(hdr: *mut nfs_pgio_header) {
    let netfs = (*hdr).netfs;
    if netfs.is_null() { return; }
    let sreq = (*netfs).sreq;
    if test_bit(NFS_IOHDR_EOF, &(*hdr).flags)
        && (*(*sreq).rreq).origin != NETFS_UNBUFFERED_READ
        && (*(*sreq).rreq).origin != NETFS_DIO_READ
    { __set_bit(NETFS_SREQ_CLEAR_TAIL, &mut (*sreq).flags); }
    if (*hdr).error { (*netfs).error = (*hdr).error; }
    else { atomic64_add((*hdr).res.count, &mut (*netfs).transferred); }
    nfs_netfs_put(netfs);
    (*hdr).netfs = core::ptr::null_mut();
}

const nfs_netfs_ops: netfs_request_ops = netfs_request_ops {
    init_request: Some(nfs_netfs_init_request),
    free_request: Some(nfs_netfs_free_request),
    issue_read: Some(nfs_netfs_issue_read),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
