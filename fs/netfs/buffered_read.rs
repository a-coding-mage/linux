// SPDX-License-Identifier: GPL-2.0-or-later
/* Network filesystem high-level buffered read support. */

// Kernel dependencies supplied by the surrounding netfs implementation.

unsafe fn netfs_cache_expand_readahead(rreq: *mut netfs_io_request, start: *mut u64, len: *mut u64, i_size: u64) {
    let cres = unsafe { &mut (*rreq).cache_resources };
    if !cres.ops.is_null() && (*cres.ops).expand_readahead.is_some() {
        unsafe { ((*cres.ops).expand_readahead.unwrap())(cres, start, len, i_size); }
    }
}

unsafe fn netfs_rreq_expand(rreq: *mut netfs_io_request, ractl: *mut readahead_control) {
    // Give the cache and netfs a chance to change the request parameters.
    unsafe { netfs_cache_expand_readahead(rreq, &mut (*rreq).start, &mut (*rreq).len, (*rreq).i_size); }
    if unsafe { (*(*rreq).netfs_ops).expand_readahead.is_some() } {
        unsafe { ((*(*rreq).netfs_ops).expand_readahead.unwrap())(rreq); }
    }
    if unsafe { (*rreq).start != readahead_pos(ractl) || (*rreq).len != readahead_length(ractl) } {
        unsafe { readahead_expand(ractl, (*rreq).start, (*rreq).len); }
        unsafe { (*rreq).start = readahead_pos(ractl); (*rreq).len = readahead_length(ractl); }
        unsafe { trace_netfs_read(rreq, readahead_pos(ractl), readahead_length(ractl), netfs_read_trace_expanded); }
    }
}

unsafe fn netfs_begin_cache_read(rreq: *mut netfs_io_request, ctx: *mut netfs_inode) -> i32 {
    unsafe { fscache_begin_read_operation(&mut (*rreq).cache_resources, netfs_i_cookie(ctx)) }
}

unsafe fn netfs_prepare_read_iterator(subreq: *mut netfs_io_subrequest, ractl: *mut readahead_control) -> isize {
    let rreq = unsafe { (*subreq).rreq };
    let mut rsize = unsafe { (*subreq).len };
    if unsafe { (*subreq).source == NETFS_DOWNLOAD_FROM_SERVER } { rsize = unsafe { umin(rsize, (*rreq).io_streams[0].sreq_max_len) }; }
    if !ractl.is_null() {
        let mut put_batch = unsafe { core::mem::zeroed::<folio_batch>() };
        unsafe { folio_batch_init(&mut put_batch); }
        while unsafe { (*rreq).submitted < (*subreq).start + rsize } {
            let added = unsafe { rolling_buffer_load_from_ra(&mut (*rreq).buffer, ractl, &mut put_batch) };
            if added < 0 { unsafe { folio_batch_release(&mut put_batch); } return added; }
            unsafe { (*rreq).submitted += added as u64; }
        }
        unsafe { folio_batch_release(&mut put_batch); }
    }
    unsafe { (*subreq).len = rsize; }
    if unsafe { (*rreq).io_streams[0].sreq_max_segs != 0 } {
        let limit = unsafe { netfs_limit_iter(&mut (*rreq).buffer.iter, 0, rsize, (*rreq).io_streams[0].sreq_max_segs) };
        if limit < rsize { unsafe { (*subreq).len = limit; trace_netfs_sreq(subreq, netfs_sreq_trace_limited); } }
    }
    unsafe { (*subreq).io_iter = (*rreq).buffer.iter; iov_iter_truncate(&mut (*subreq).io_iter, (*subreq).len); rolling_buffer_advance(&mut (*rreq).buffer, (*subreq).len); (*subreq).len as isize }
}

unsafe fn netfs_cache_prepare_read(rreq: *mut netfs_io_request, subreq: *mut netfs_io_subrequest, i_size: i64) -> netfs_io_source {
    let cres = unsafe { &mut (*rreq).cache_resources };
    if cres.ops.is_null() { return NETFS_DOWNLOAD_FROM_SERVER; }
    let source = unsafe { ((*cres.ops).prepare_read.unwrap())(subreq, i_size) };
    unsafe { trace_netfs_sreq(subreq, netfs_sreq_trace_prepare); }
    source
}

unsafe fn netfs_read_cache_to_pagecache(rreq: *mut netfs_io_request, subreq: *mut netfs_io_subrequest) {
    let cres = unsafe { &mut (*rreq).cache_resources };
    unsafe { netfs_stat(&netfs_n_rh_read); ((*cres.ops).read.unwrap())(cres, (*subreq).start, &mut (*subreq).io_iter, NETFS_READ_HOLE_IGNORE, netfs_cache_read_terminated, subreq); }
}

pub unsafe fn netfs_queue_read(rreq: *mut netfs_io_request, subreq: *mut netfs_io_subrequest) {
    let stream = unsafe { &mut (*rreq).io_streams[0] };
    unsafe { __set_bit(NETFS_SREQ_IN_PROGRESS, &mut (*subreq).flags); spin_lock(&mut (*rreq).lock); list_add_tail_release(&mut (*subreq).rreq_link, &mut stream.subrequests); if list_is_first(&(*subreq).rreq_link, &stream.subrequests) && !stream.active { stream.collected_to = (*subreq).start; smp_store_release(&mut stream.active, true); } spin_unlock(&mut (*rreq).lock); }
}

unsafe fn netfs_issue_read(rreq: *mut netfs_io_request, subreq: *mut netfs_io_subrequest) {
    match unsafe { (*subreq).source } {
        NETFS_DOWNLOAD_FROM_SERVER => unsafe { ((*(*rreq).netfs_ops).issue_read.unwrap())(subreq) },
        NETFS_READ_FROM_CACHE => unsafe { netfs_read_cache_to_pagecache(rreq, subreq) },
        _ => unsafe { __set_bit(NETFS_SREQ_CLEAR_TAIL, &mut (*subreq).flags); (*subreq).error = 0; iov_iter_zero((*subreq).len, &mut (*subreq).io_iter); (*subreq).transferred = (*subreq).len; netfs_read_subreq_terminated(subreq); },
    }
}

unsafe fn netfs_read_to_pagecache(rreq: *mut netfs_io_request, ractl: *mut readahead_control) {
    let mut start = unsafe { (*rreq).start }; let mut size = unsafe { (*rreq).len as isize }; let mut ret = 0i32;
    while size > 0 {
        let subreq = unsafe { netfs_alloc_subrequest(rreq) }; if subreq.is_null() { ret = -ENOMEM; break; }
        unsafe { (*subreq).start = start; (*subreq).len = size as usize; netfs_queue_read(rreq, subreq); let mut source = netfs_cache_prepare_read(rreq, subreq, (*rreq).i_size); (*subreq).source = source;
        if source == NETFS_DOWNLOAD_FROM_SERVER { let zero_point = netfs_read_zero_point((*rreq).inode); let mut zp = umin(zero_point, (*rreq).i_size); if (*rreq).origin == NETFS_READ_SINGLE { zp = (*rreq).i_size; } if start >= zp { source = NETFS_FILL_WITH_ZEROES; (*subreq).source = source; } else { let mut len = (*subreq).len; if len as u64 > zp - start { len = (zp - start) as usize; } if len == 0 { netfs_cancel_read(subreq, ret); break; } (*subreq).len = len; if (*(*rreq).netfs_ops).prepare_read.is_some() { ret = ((*(*rreq).netfs_ops).prepare_read.unwrap())(subreq); if ret < 0 { netfs_cancel_read(subreq, ret); break; } trace_netfs_sreq(subreq, netfs_sreq_trace_prepare); } source = NETFS_DOWNLOAD_FROM_SERVER; }
        }
        if source == NETFS_FILL_WITH_ZEROES || source == NETFS_READ_FROM_CACHE { trace_netfs_sreq(subreq, netfs_sreq_trace_submit); if source == NETFS_FILL_WITH_ZEROES { netfs_stat(&netfs_n_rh_zero); } } else if source != NETFS_DOWNLOAD_FROM_SERVER { WARN_ON_ONCE(1); netfs_cancel_read(subreq, ret); break; }
        let slice = netfs_prepare_read_iterator(subreq, ractl); if slice < 0 { ret = slice as i32; netfs_cancel_read(subreq, ret); break; } start += slice as u64; size -= slice; if size <= 0 { smp_wmb(); set_bit(NETFS_RREQ_ALL_QUEUED, &mut (*rreq).flags); } netfs_issue_read(rreq, subreq); if test_bit(NETFS_RREQ_PAUSE, &(*rreq).flags) { netfs_wait_for_paused_read(rreq); } if test_bit(NETFS_RREQ_FAILED, &(*rreq).flags) { break; } cond_resched(); }
    }
    if size > 0 { smp_wmb(); unsafe { set_bit(NETFS_RREQ_ALL_QUEUED, &mut (*rreq).flags); netfs_wake_collector(rreq); } }
    unsafe { cmpxchg(&mut (*rreq).error, 0, ret); }
}

pub unsafe fn netfs_readahead(ractl: *mut readahead_control) {
    let ictx = unsafe { netfs_inode((*(*ractl).mapping).host) }; let start = unsafe { readahead_pos(ractl) }; let size = unsafe { readahead_length(ractl) };
    let rreq = unsafe { netfs_alloc_request((*ractl).mapping, (*ractl).file, start, size, NETFS_READAHEAD) }; if is_err(rreq) { return; }
    unsafe { __set_bit(NETFS_RREQ_OFFLOAD_COLLECTION, &mut (*rreq).flags); let ret = netfs_begin_cache_read(rreq, ictx); if ret == -ENOMEM || ret == -EINTR || ret == -ERESTARTSYS { return netfs_put_failed_request(rreq); } netfs_stat(&netfs_n_rh_readahead); trace_netfs_read(rreq, readahead_pos(ractl), readahead_length(ractl), netfs_read_trace_readahead); netfs_rreq_expand(rreq, ractl); (*rreq).submitted = (*rreq).start; if rolling_buffer_init(&mut (*rreq).buffer, (*rreq).debug_id, ITER_DEST, (*rreq).gfp) < 0 { return netfs_put_failed_request(rreq); } netfs_read_to_pagecache(rreq, ractl); netfs_put_request(rreq, netfs_rreq_trace_put_return); }
}

unsafe fn netfs_create_singular_buffer(rreq: *mut netfs_io_request, folio: *mut folio, flags: u32) -> i32 {
    if unsafe { rolling_buffer_init(&mut (*rreq).buffer, (*rreq).debug_id, ITER_DEST, (*rreq).gfp) } < 0 { return -ENOMEM; }
    let added = unsafe { rolling_buffer_append(&mut (*rreq).buffer, folio, flags, (*rreq).gfp) }; if added < 0 { return added as i32; } unsafe { (*rreq).submitted = (*rreq).start + added as u64; } 0
}

// Read into gaps in a folio partially filled by a streaming write.
unsafe fn netfs_read_gaps(file: *mut file, folio: *mut folio) -> i32 {
    let mapping = unsafe { (*folio).mapping }; let ctx = unsafe { netfs_inode((*mapping).host) }; let finfo = unsafe { netfs_folio_info(folio) };
    let from = unsafe { (*finfo).dirty_offset }; let to = from + unsafe { (*finfo).dirty_len }; let flen = unsafe { folio_size(folio) };
    let rreq = unsafe { netfs_alloc_request(mapping, file, folio_pos(folio), flen, NETFS_READ_GAPS) }; if unsafe { is_err(rreq) } { unsafe { folio_unlock(folio); } return unsafe { ptr_err(rreq) as i32 }; }
    let ret = unsafe { netfs_begin_cache_read(rreq, ctx) }; if ret == -ENOMEM || ret == -EINTR || ret == -ERESTARTSYS { unsafe { netfs_put_failed_request(rreq); folio_unlock(folio); } return ret; }
    unsafe { let count = flen / PAGE_SIZE + 2; let bvec = kmalloc_objs(count); if bvec.is_null() { netfs_put_failed_request(rreq); folio_unlock(folio); return -ENOMEM; } let sink = folio_alloc(GFP_KERNEL, 0); if sink.is_null() { kfree(bvec); netfs_put_failed_request(rreq); folio_unlock(folio); return -ENOMEM; } (*rreq).direct_bv = bvec; (*rreq).direct_bv_count = count; let mut i = 0; if from > 0 { bvec_set_folio(&mut *bvec.add(i), folio, from, 0); i += 1; } let mut off = from; while off < to { let part = min_t(to - off, PAGE_SIZE); bvec_set_folio(&mut *bvec.add(i), sink, part, 0); i += 1; off += part; } if to < flen { bvec_set_folio(&mut *bvec.add(i), folio, flen - to, to); i += 1; } iov_iter_bvec(&mut (*rreq).buffer.iter, ITER_DEST, bvec, i, (*rreq).len); (*rreq).submitted = (*rreq).start + flen as u64; netfs_read_to_pagecache(rreq, core::ptr::null_mut()); let ret = netfs_wait_for_read(rreq); if ret >= 0 { folio_mark_uptodate(folio); flush_dcache_folio(folio); } folio_put(sink); kfree(bvec); folio_unlock(folio); netfs_put_request(rreq, netfs_rreq_trace_put_return); if ret < 0 { ret } else { 0 } }
}

unsafe fn netfs_skip_folio_read(folio: *mut folio, pos: i64, len: usize, always_fill: bool) -> bool {
    let i_size = unsafe { i_size_read(folio_inode(folio)) }; let offset = unsafe { offset_in_folio(folio, pos) }; let plen = unsafe { folio_size(folio) };
    if always_fill { if pos - offset as i64 + len <= i_size { return false; } unsafe { folio_zero_segment(folio, 0, plen); folio_mark_uptodate(folio); } return true; }
    if offset == 0 && len >= plen { return true; } if pos - offset as i64 >= i_size { unsafe { folio_zero_segments(folio, 0, offset, offset + len, plen); } return true; } if offset == 0 && pos + len as i64 >= i_size { unsafe { folio_zero_segments(folio, 0, offset, offset + len, plen); } return true; } false
}

pub unsafe fn netfs_read_folio(file: *mut file, folio: *mut folio) -> i32 {
    let mapping = unsafe { (*folio).mapping }; let ctx = unsafe { netfs_inode((*mapping).host) }; unsafe { folio_wait_writeback(folio); }
    if unsafe { folio_test_dirty(folio) } { return unsafe { netfs_read_gaps(file, folio) }; }
    let rreq = unsafe { netfs_alloc_request(mapping, file, folio_pos(folio), folio_size(folio), NETFS_READPAGE) }; if unsafe { is_err(rreq) } { unsafe { folio_unlock(folio); } return unsafe { ptr_err(rreq) as i32 }; }
    let ret = unsafe { netfs_begin_cache_read(rreq, ctx) }; if ret == -ENOMEM || ret == -EINTR || ret == -ERESTARTSYS { unsafe { netfs_put_failed_request(rreq); folio_unlock(folio); } return ret; }
    unsafe { netfs_stat(&netfs_n_rh_read_folio); trace_netfs_read(rreq, (*rreq).start, (*rreq).len, netfs_read_trace_readpage); let ret = netfs_create_singular_buffer(rreq, folio, 0); if ret < 0 { netfs_put_failed_request(rreq); folio_unlock(folio); return ret; } netfs_read_to_pagecache(rreq, core::ptr::null_mut()); let ret = netfs_wait_for_read(rreq); netfs_put_request(rreq, netfs_rreq_trace_put_return); if ret < 0 { ret } else { 0 } }
}

pub unsafe fn netfs_prefetch_for_write(file: *mut file, folio: *mut folio, _offset: usize, _len: usize) -> i32 {
    let mapping = unsafe { (*folio).mapping }; let ctx = unsafe { netfs_inode((*mapping).host) }; let start = unsafe { folio_pos(folio) }; let flen = unsafe { folio_size(folio) };
    let rreq = unsafe { netfs_alloc_request(mapping, file, start, flen, NETFS_READ_FOR_WRITE) }; if unsafe { is_err(rreq) } { return unsafe { ptr_err(rreq) as i32 }; }
    unsafe { (*rreq).no_unlock_folio = folio; __set_bit(NETFS_RREQ_NO_UNLOCK_FOLIO, &mut (*rreq).flags); let ret = netfs_begin_cache_read(rreq, ctx); if ret == -ENOMEM || ret == -EINTR || ret == -ERESTARTSYS { netfs_put_failed_request(rreq); return ret; } netfs_stat(&netfs_n_rh_write_begin); trace_netfs_read(rreq, start, flen, netfs_read_trace_prefetch_for_write); let ret = netfs_create_singular_buffer(rreq, folio, NETFS_ROLLBUF_PAGECACHE_MARK); if ret < 0 { netfs_put_failed_request(rreq); return ret; } netfs_read_to_pagecache(rreq, core::ptr::null_mut()); let ret = netfs_wait_for_read(rreq); netfs_put_request(rreq, netfs_rreq_trace_put_return); if ret < 0 { ret } else { 0 } }
}

pub unsafe fn netfs_write_begin(ctx: *mut netfs_inode, file: *mut file, mapping: *mut address_space, pos: i64, len: u32, folio_out: *mut *mut folio, fsdata: *mut *mut core::ffi::c_void) -> i32 {
    let index = (pos >> PAGE_SHIFT) as pgoff_t;
    'retry: loop {
        let folio = unsafe { __filemap_get_folio(mapping, index, FGP_WRITEBEGIN, mapping_gfp_mask(mapping)) }; if unsafe { is_err(folio) } { return unsafe { ptr_err(folio) as i32 }; }
        let mut ret;
        if unsafe { (*ctx).ops.check_write_begin.is_some() } { ret = unsafe { ((*(*ctx).ops).check_write_begin.unwrap())(file, pos, len, &mut (folio as *mut _), fsdata) }; if ret < 0 { unsafe { folio_unlock(folio); folio_put(folio); } return ret; } if folio.is_null() { continue 'retry; } }
        if unsafe { folio_test_uptodate(folio) || (!netfs_is_cache_maybe_enabled(ctx) && netfs_skip_folio_read(folio, pos, len as usize, false)) } { unsafe { *folio_out = folio; } return 0; }
        let rreq = unsafe { netfs_alloc_request(mapping, file, folio_pos(folio), folio_size(folio), NETFS_READ_FOR_WRITE) }; if unsafe { is_err(rreq) } { unsafe { folio_unlock(folio); folio_put(folio); } return unsafe { ptr_err(rreq) as i32 }; }
        unsafe { (*rreq).no_unlock_folio = folio; __set_bit(NETFS_RREQ_NO_UNLOCK_FOLIO, &mut (*rreq).flags); ret = netfs_begin_cache_read(rreq, ctx); if ret == -ENOMEM || ret == -EINTR || ret == -ERESTARTSYS { netfs_put_failed_request(rreq); folio_unlock(folio); folio_put(folio); return ret; } let ret = netfs_create_singular_buffer(rreq, folio, 0); if ret < 0 { netfs_put_failed_request(rreq); folio_unlock(folio); folio_put(folio); return ret; } netfs_read_to_pagecache(rreq, core::ptr::null_mut()); ret = netfs_wait_for_read(rreq); netfs_put_request(rreq, netfs_rreq_trace_put_return); if ret < 0 { folio_unlock(folio); folio_put(folio); return ret; } ret = folio_wait_private_2_killable(folio); if ret < 0 { folio_unlock(folio); folio_put(folio); return ret; } *folio_out = folio; return 0; }
    }
}

pub unsafe fn netfs_buffered_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize { let inode = unsafe { file_inode((*iocb).ki_filp) }; let ictx = unsafe { netfs_inode(inode) }; if unsafe { ((*iocb).ki_flags & IOCB_DIRECT) != 0 || test_bit(NETFS_ICTX_UNBUFFERED, &(*ictx).flags) } { return -EINVAL as isize; } let mut ret = unsafe { netfs_start_io_read(inode) }; if ret == 0 { ret = unsafe { filemap_read(iocb, iter, 0) }; unsafe { netfs_end_io_read(inode); } } ret }

pub unsafe fn netfs_file_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize { let ictx = unsafe { netfs_inode((*(*iocb).ki_filp).f_mapping.host) }; if unsafe { ((*iocb).ki_flags & IOCB_DIRECT) != 0 || test_bit(NETFS_ICTX_UNBUFFERED, &(*ictx).flags) } { unsafe { netfs_unbuffered_read_iter(iocb, iter) } } else { unsafe { netfs_buffered_read_iter(iocb, iter) } } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
