// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/fs/nfs/read.c
 *
 * Block I/O for NFS
 *
 * Partial copy of Linus' read cache modifications to fs/nfs/file.c
 * modified for async RPC by okir@monad.swb.de
 */

// C dependencies: linux/time.h, linux/kernel.h, linux/errno.h,
// linux/fcntl.h, linux/stat.h, linux/mm.h, linux/slab.h,
// linux/task_io_accounting_ops.h, linux/pagemap.h, linux/sunrpc/clnt.h,
// linux/nfs_fs.h, linux/nfs_page.h, linux/module.h, and local NFS headers.

// #define NFSDBG_FACILITY NFSDBG_PAGECACHE

extern "C" {
    static mut nfs_rdata_cachep: *mut kmem_cache;
    static nfs_async_read_completion_ops: nfs_pgio_completion_ops;
    static nfs_rw_read_ops: nfs_rw_ops;
}

unsafe fn nfs_readhdr_alloc() -> *mut nfs_pgio_header {
    let p = kmem_cache_zalloc(nfs_rdata_cachep, GFP_KERNEL) as *mut nfs_pgio_header;
    if !p.is_null() {
        (*p).rw_mode = FMODE_READ;
    }
    p
}

unsafe fn nfs_readhdr_free(rhdr: *mut nfs_pgio_header) {
    kfree((*rhdr).res.scratch);
    kmem_cache_free(nfs_rdata_cachep, rhdr as *mut _);
}

unsafe fn nfs_return_empty_folio(folio: *mut folio) -> c_int {
    folio_zero_segment(folio, 0, folio_size(folio));
    folio_mark_uptodate(folio);
    if nfs_netfs_folio_unlock(folio) {
        folio_unlock(folio);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn nfs_pageio_init_read(
    pgio: *mut nfs_pageio_descriptor,
    inode: *mut inode,
    force_mds: bool,
    compl_ops: *const nfs_pgio_completion_ops,
) {
    let server = NFS_SERVER(inode);
    let mut pg_ops = &nfs_pgio_rw_ops as *const nfs_pageio_ops;

    // CONFIG_NFS_V4 build-time conditional preserved from the C source.
    #[cfg(feature = "CONFIG_NFS_V4")]
    {
        if !(*server).pnfs_curr_ld.is_null() && !force_mds {
            pg_ops = (*(*server).pnfs_curr_ld).pg_read_ops;
        }
    }
    nfs_pageio_init(
        pgio, inode, pg_ops, compl_ops, &nfs_rw_read_ops,
        (*server).rsize, 0,
    );
}

#[no_mangle]
pub unsafe extern "C" fn nfs_pageio_complete_read(pgio: *mut nfs_pageio_descriptor) {
    let pgm: *mut nfs_pgio_mirror;
    let npages: c_ulong;

    nfs_pageio_complete(pgio);
    WARN_ON_ONCE((*pgio).pg_mirror_count != 1);
    pgm = &mut (*pgio).pg_mirrors[0];
    (*NFS_I((*pgio).pg_inode)).read_io += (*pgm).pg_bytes_written;
    npages = ((*pgm).pg_bytes_written + PAGE_SIZE - 1) >> PAGE_SHIFT;
    nfs_add_stats((*pgio).pg_inode, NFSIOS_READPAGES, npages);
}

#[no_mangle]
pub unsafe extern "C" fn nfs_pageio_reset_read_mds(pgio: *mut nfs_pageio_descriptor) {
    if !(*pgio).pg_ops.is_null() && !(*(*pgio).pg_ops).pg_cleanup.is_none() {
        ((*(*pgio).pg_ops).pg_cleanup.unwrap())(pgio);
    }
    (*pgio).pg_ops = &nfs_pgio_rw_ops;
    WARN_ON_ONCE((*pgio).pg_mirror_count != 1);
    let mirror = &mut (*pgio).pg_mirrors[0];
    (*mirror).pg_bsize = (*NFS_SERVER((*pgio).pg_inode)).rsize;
}

#[no_mangle]
pub unsafe extern "C" fn nfs_read_alloc_scratch(hdr: *mut nfs_pgio_header, size: usize) -> bool {
    WARN_ON(!(*hdr).res.scratch.is_null());
    (*hdr).res.scratch = kmalloc(size, GFP_KERNEL);
    !(*hdr).res.scratch.is_null()
}

unsafe fn nfs_readpage_release(req: *mut nfs_page, error: c_int) {
    let folio = nfs_page_to_folio(req);
    if nfs_page_group_sync_on_bit(req, PG_UNLOCKPAGE) && nfs_netfs_folio_unlock(folio) {
        folio_unlock(folio);
    }
    nfs_release_request(req);
}

unsafe fn nfs_page_group_set_uptodate(req: *mut nfs_page) {
    let mut uptodate = false;
    nfs_page_group_lock(req);
    if !test_bit(PG_READ_FAILED, &mut (*(*req).wb_head).wb_flags)
        && nfs_page_group_sync_on_bit_locked(req, PG_UPTODATE) {
        uptodate = true;
    }
    nfs_page_group_unlock(req);
    if uptodate { folio_mark_uptodate(nfs_page_to_folio(req)); }
}

unsafe fn nfs_page_group_mark_read_failed(req: *mut nfs_page) {
    nfs_page_group_lock(req);
    set_bit(PG_READ_FAILED, &mut (*(*req).wb_head).wb_flags);
    let mut tmp = req;
    loop {
        clear_bit(PG_UPTODATE, &mut (*tmp).wb_flags);
        tmp = (*tmp).wb_this_page;
        if tmp == req { break; }
    }
    nfs_page_group_unlock(req);
}

unsafe fn nfs_read_completion(hdr: *mut nfs_pgio_header) {
    let mut bytes: c_ulong = 0;
    if test_bit(NFS_IOHDR_REDO, &mut (*hdr).flags) { return (*hdr).release.unwrap()(hdr); }
    while !list_empty(&(*hdr).pages) {
        let req = nfs_list_entry((*hdr).pages.next);
        let folio = nfs_page_to_folio(req);
        let mut start = (*req).wb_pgbase;
        let end = (*req).wb_pgbase + (*req).wb_bytes;
        if test_bit(NFS_IOHDR_EOF, &mut (*hdr).flags) {
            if bytes > (*hdr).good_bytes {
                folio_zero_segment(folio, start, end);
            } else if (*hdr).good_bytes - bytes < (*req).wb_bytes {
                start += (*hdr).good_bytes - bytes;
                WARN_ON(start < (*req).wb_pgbase);
                folio_zero_segment(folio, start, end);
            }
        }
        let mut error = 0;
        bytes += (*req).wb_bytes;
        if test_bit(NFS_IOHDR_ERROR, &mut (*hdr).flags) {
            if bytes <= (*hdr).good_bytes { nfs_page_group_set_uptodate(req); }
            else {
                nfs_page_group_mark_read_failed(req);
                error = (*hdr).error;
                xchg(&mut (*nfs_req_openctx(req)).error, error);
            }
        } else { nfs_page_group_set_uptodate(req); }
        nfs_list_remove_request(req);
        nfs_readpage_release(req, error);
    }
    nfs_netfs_read_completion(hdr);
    (*hdr).release.unwrap()(hdr);
}

unsafe fn nfs_initiate_read(hdr: *mut nfs_pgio_header, msg: *mut rpc_message,
    rpc_ops: *const nfs_rpc_ops, _task_setup_data: *mut rpc_task_setup, _how: c_int) {
    ((*rpc_ops).read_setup.unwrap())(hdr, msg);
    nfs_netfs_initiate_read(hdr);
    trace_nfs_initiate_read(hdr);
}

unsafe fn nfs_async_read_error(head: *mut list_head, error: c_int) {
    while !list_empty(head) {
        let req = nfs_list_entry((*head).next);
        nfs_list_remove_request(req);
        nfs_readpage_release(req, error);
    }
}

#[no_mangle]
pub static nfs_async_read_completion_ops: nfs_pgio_completion_ops = nfs_pgio_completion_ops {
    error_cleanup: Some(nfs_async_read_error), completion: Some(nfs_read_completion),
};

// This is the callback from RPC telling us whether a reply was received or an error occurred.
unsafe fn nfs_readpage_done(task: *mut rpc_task, hdr: *mut nfs_pgio_header, inode: *mut inode) -> c_int {
    let status = (NFS_PROTO(inode).read_done.unwrap())(task, hdr);
    if status != 0 { return status; }
    nfs_add_stats(inode, NFSIOS_SERVERREADBYTES, (*hdr).res.count);
    trace_nfs_readpage_done(task, hdr);
    if (*task).tk_status == -ESTALE {
        nfs_set_inode_stale(inode);
        nfs_mark_for_revalidate(inode);
    }
    0
}

unsafe fn nfs_readpage_retry(task: *mut rpc_task, hdr: *mut nfs_pgio_header) {
    let argp = &mut (*hdr).args;
    let resp = &mut (*hdr).res;
    nfs_inc_stats((*hdr).inode, NFSIOS_SHORTREAD);
    trace_nfs_readpage_short(task, hdr);
    if resp.count == 0 { nfs_set_pgio_error(hdr, -EIO, argp.offset); return; }
    if (*task).tk_ops.is_null() { (*hdr).pnfs_error = -EAGAIN; return; }
    (*hdr).mds_offset += resp.count;
    argp.offset += resp.count;
    argp.pgbase += resp.count;
    argp.count -= resp.count;
    resp.count = 0;
    resp.eof = 0;
    rpc_restart_call_prepare(task);
}

unsafe fn nfs_readpage_result(task: *mut rpc_task, hdr: *mut nfs_pgio_header) {
    if (*hdr).res.eof {
        let pos = (*hdr).args.offset + (*hdr).res.count;
        let new = (pos - (*hdr).io_start) as c_uint;
        if (*hdr).good_bytes > new {
            (*hdr).good_bytes = new;
            set_bit(NFS_IOHDR_EOF, &mut (*hdr).flags);
            clear_bit(NFS_IOHDR_ERROR, &mut (*hdr).flags);
        }
    } else if (*hdr).res.count < (*hdr).args.count { nfs_readpage_retry(task, hdr); }
}

#[no_mangle]
pub unsafe extern "C" fn nfs_read_add_folio(pgio: *mut nfs_pageio_descriptor,
    ctx: *mut nfs_open_context, folio: *mut folio) -> c_int {
    let inode = (*(*folio).mapping).host;
    let server = NFS_SERVER(inode);
    let fsize = folio_size(folio);
    let rsize = (*server).rsize;
    let len = nfs_folio_length(folio);
    if len == 0 { return nfs_return_empty_folio(folio); }
    let aligned_len = min_t(ALIGN(len, rsize), fsize);
    let new = nfs_page_create_from_folio(ctx, folio, 0, aligned_len);
    if IS_ERR(new) {
        let error = PTR_ERR(new);
        if nfs_netfs_folio_unlock(folio) { folio_unlock(folio); }
        return error;
    }
    if len < fsize { folio_zero_segment(folio, len, fsize); }
    if !nfs_pageio_add_request(pgio, new) {
        nfs_list_remove_request(new);
        let error = (*pgio).pg_error;
        nfs_readpage_release(new, error);
        return error;
    }
    0
}

// Actually read a folio over the wire.
unsafe fn nfs_do_read_folio(file: *mut file, folio: *mut folio) -> c_int {
    let inode = file_inode(file);
    let mut pgio = core::mem::zeroed::<nfs_pageio_descriptor>();
    let ctx = get_nfs_open_context(nfs_file_open_context(file));
    xchg(&mut (*ctx).error, 0);
    nfs_pageio_init_read(&mut pgio, inode, false, &nfs_async_read_completion_ops);
    let mut ret = nfs_read_add_folio(&mut pgio, ctx, folio);
    if ret != 0 { put_nfs_open_context(ctx); return ret; }
    nfs_pageio_complete_read(&mut pgio);
    nfs_update_delegated_atime(inode);
    if pgio.pg_error < 0 { ret = pgio.pg_error; put_nfs_open_context(ctx); return ret; }
    ret = folio_wait_locked_killable(folio);
    if !folio_test_uptodate(folio) && ret == 0 { ret = xchg(&mut (*ctx).error, 0); }
    put_nfs_open_context(ctx);
    ret
}

// Synchronously read a folio.
#[no_mangle]
pub unsafe extern "C" fn nfs_read_folio(file: *mut file, folio: *mut folio) -> c_int {
    let inode = file_inode(file);
    let pos = folio_pos(folio);
    let len = folio_size(folio);
    trace_nfs_aop_readpage(inode, pos, len);
    nfs_inc_stats(inode, NFSIOS_VFSREADPAGE);
    task_io_account_read(len);
    let mut ret = nfs_wb_folio(inode, folio);
    if ret != 0 || folio_test_uptodate(folio) { folio_unlock(folio); }
    else {
        ret = -ESTALE;
        if !NFS_STALE(inode) {
            ret = nfs_netfs_read_folio(file, folio);
            if ret != 0 { ret = nfs_do_read_folio(file, folio); }
        }
    }
    trace_nfs_aop_readpage_done(inode, pos, len, ret);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn nfs_readahead(ractl: *mut readahead_control) {
    let mut pgio = core::mem::zeroed::<nfs_pageio_descriptor>();
    let nr_pages = readahead_count(ractl);
    let file = (*ractl).file;
    let inode = (*(*ractl).mapping).host;
    trace_nfs_aop_readahead(inode, readahead_pos(ractl), nr_pages);
    nfs_inc_stats(inode, NFSIOS_VFSREADPAGES);
    task_io_account_read(readahead_length(ractl));
    if NFS_STALE(inode) { trace_nfs_aop_readahead_done(inode, nr_pages, -ESTALE); return; }
    if nfs_netfs_readahead(ractl) == 0 { trace_nfs_aop_readahead_done(inode, nr_pages, 0); return; }
    let ctx = if file.is_null() {
        let c = nfs_find_open_context(inode, core::ptr::null_mut(), FMODE_READ);
        if c.is_null() { trace_nfs_aop_readahead_done(inode, nr_pages, -EBADF); return; }
        c
    } else { get_nfs_open_context(nfs_file_open_context(file)) };
    nfs_pageio_init_read(&mut pgio, inode, false, &nfs_async_read_completion_ops);
    loop {
        let folio = readahead_folio(ractl);
        if folio.is_null() { break; }
        if nfs_read_add_folio(&mut pgio, ctx, folio) != 0 { break; }
    }
    nfs_pageio_complete_read(&mut pgio);
    nfs_update_delegated_atime(inode);
    put_nfs_open_context(ctx);
    trace_nfs_aop_readahead_done(inode, nr_pages, 0);
}

#[no_mangle]
pub unsafe extern "C" fn nfs_init_readpagecache() -> c_int {
    nfs_rdata_cachep = kmem_cache_create("nfs_read_data\0".as_ptr() as *const c_char,
        core::mem::size_of::<nfs_pgio_header>(), 0, SLAB_HWCACHE_ALIGN, None);
    if nfs_rdata_cachep.is_null() { return -ENOMEM; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn nfs_destroy_readpagecache() {
    kmem_cache_destroy(nfs_rdata_cachep);
}

static nfs_rw_read_ops: nfs_rw_ops = nfs_rw_ops {
    rw_alloc_header: Some(nfs_readhdr_alloc),
    rw_free_header: Some(nfs_readhdr_free),
    rw_done: Some(nfs_readpage_done),
    rw_result: Some(nfs_readpage_result),
    rw_initiate: Some(nfs_initiate_read),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
