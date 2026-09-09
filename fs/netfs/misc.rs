// SPDX-License-Identifier: GPL-2.0-only
/* Miscellaneous routines.
 *
 * Copyright (C) 2023 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel/netfs translation unit.

pub unsafe fn netfs_alloc_folioq_buffer(
    mapping: *mut address_space,
    buffer: *mut *mut folio_queue,
    cur_size: *mut usize,
    mut size: isize,
    gfp: gfp_t,
) -> i32 {
    let mut tail = *buffer;
    let mut p: *mut folio_queue;

    size = round_up(size as usize, PAGE_SIZE) as isize;
    if *cur_size >= size as usize { return 0; }
    if !tail.is_null() {
        while !(*tail).next.is_null() { tail = (*tail).next; }
    }
    loop {
        let mut order: i32 = 0;
        let slot: i32;
        if tail.is_null() || folioq_full(tail) {
            p = netfs_folioq_alloc(0, GFP_NOFS, netfs_trace_folioq_alloc_buffer);
            if p.is_null() { return -ENOMEM; }
            if !tail.is_null() { (*tail).next = p; (*p).prev = tail; }
            else { *buffer = p; }
            tail = p;
        }
        if size as usize - *cur_size > PAGE_SIZE {
            order = umin(ilog2(size as usize - *cur_size) - PAGE_SHIFT, MAX_PAGECACHE_ORDER);
        }
        let mut folio = folio_alloc(gfp, order);
        if folio.is_null() && order > 0 { folio = folio_alloc(gfp, 0); }
        if folio.is_null() { return -ENOMEM; }
        (*folio).mapping = mapping;
        (*folio).index = *cur_size / PAGE_SIZE;
        trace_netfs_folio(folio, netfs_folio_trace_alloc_buffer);
        slot = folioq_append_mark(tail, folio);
        *cur_size += folioq_folio_size(tail, slot);
        if *cur_size >= size as usize { break; }
    }
    0
}

pub unsafe fn netfs_free_folioq_buffer(mut fq: *mut folio_queue) {
    let mut fbatch: folio_batch = core::mem::zeroed();
    folio_batch_init(&mut fbatch);
    while !fq.is_null() {
        for slot in 0..folioq_count(fq) {
            let folio = folioq_folio(fq, slot);
            if folio.is_null() || !folioq_is_marked(fq, slot) { continue; }
            trace_netfs_folio(folio, netfs_folio_trace_put);
            if folio_batch_add(&mut fbatch, folio) { folio_batch_release(&mut fbatch); }
        }
        netfs_stat_d(&mut netfs_n_folioq);
        let next = (*fq).next;
        kfree(fq as *mut core::ffi::c_void);
        fq = next;
    }
    folio_batch_release(&mut fbatch);
}

pub unsafe fn netfs_reset_iter(subreq: *mut netfs_io_subrequest) {
    let io_iter = &mut (*subreq).io_iter;
    let remain = (*subreq).len - (*subreq).transferred;
    if io_iter.count > remain { iov_iter_advance(io_iter, io_iter.count - remain); }
    else if io_iter.count < remain { iov_iter_revert(io_iter, remain - io_iter.count); }
    iov_iter_truncate(io_iter, remain);
}

pub unsafe fn netfs_dirty_folio(mapping: *mut address_space, folio: *mut folio) -> bool {
    let inode = (*mapping).host;
    let ictx = netfs_inode(inode);
    let cookie = netfs_i_cookie(ictx);
    let mut need_use = false;
    _enter("");
    if !filemap_dirty_folio(mapping, folio) { return false; }
    if !fscache_cookie_valid(cookie) { return true; }
    if inode_state_read_once(inode) & I_PINNING_NETFS_WB == 0 {
        spin_lock(&mut (*inode).i_lock);
        if inode_state_read(inode) & I_PINNING_NETFS_WB == 0 {
            inode_state_set(inode, I_PINNING_NETFS_WB); need_use = true;
        }
        spin_unlock(&mut (*inode).i_lock);
        if need_use { fscache_use_cookie(cookie, true); }
    }
    true
}

pub unsafe fn netfs_unpin_writeback(inode: *mut inode, wbc: *mut writeback_control) -> i32 {
    let cookie = netfs_i_cookie(netfs_inode(inode));
    if (*wbc).unpinned_netfs_wb { fscache_unuse_cookie(cookie, core::ptr::null(), core::ptr::null_mut()); }
    0
}

pub unsafe fn netfs_clear_inode_writeback(inode: *mut inode, aux: *const core::ffi::c_void) {
    let cookie = netfs_i_cookie(netfs_inode(inode));
    if inode_state_read_once(inode) & I_PINNING_NETFS_WB != 0 {
        let mut i_size = i_size_read(inode);
        fscache_unuse_cookie(cookie, aux, &mut i_size);
    }
}

pub unsafe fn netfs_invalidate_folio(folio: *mut folio, offset: usize, length: usize) {
    let mut finfo: *mut netfs_folio;
    let inode = folio_inode(folio);
    let ctx = netfs_inode(inode);
    let flen = folio_size(folio);
    _enter("{%lx},%zx,%zx", (*folio).index, offset, length);
    if offset == 0 && length == flen {
        let (mut i_size, mut remote_i_size, mut zero_point) = (0, 0, 0);
        let fpos = folio_pos(folio);
        netfs_read_sizes(inode, &mut i_size, &mut remote_i_size, &mut zero_point);
        let mut end = umin(fpos + flen, i_size);
        if fpos < i_size && end > zero_point {
            spin_lock(&mut (*inode).i_lock);
            end = umin(fpos + flen, (*inode).i_size);
            if fpos < i_size && end > (*ctx)._zero_point { netfs_write_zero_point(inode, end); }
            spin_unlock(&mut (*inode).i_lock);
        }
    }
    folio_wait_private_2(folio);
    if !folio_test_private(folio) { return; }
    finfo = netfs_folio_info(folio);
    if offset == 0 && length >= flen { goto erase_completely; }
    if !finfo.is_null() {
        let fstart = (*finfo).dirty_offset; let fend = fstart + (*finfo).dirty_len; let iend = offset + length;
        if offset >= fend || iend <= fstart { return; }
        if offset <= fstart {
            if iend >= fend { goto erase_completely; }
            (*finfo).dirty_len = fend - iend; (*finfo).dirty_offset = iend;
            trace_netfs_folio(folio, netfs_folio_trace_invalidate_front); return;
        }
        if iend >= fend { (*finfo).dirty_len = offset - fstart; trace_netfs_folio(folio, netfs_folio_trace_invalidate_tail); return; }
        trace_netfs_folio(folio, netfs_folio_trace_invalidate_middle);
    }
    return;
erase_completely:
    netfs_put_group(netfs_folio_group(folio)); folio_detach_private(folio); folio_clear_uptodate(folio);
    folio_cancel_dirty(folio); kfree(finfo as *mut core::ffi::c_void);
    trace_netfs_folio(folio, netfs_folio_trace_invalidate_all);
}

pub unsafe fn netfs_release_folio(folio: *mut folio, gfp: gfp_t) -> bool {
    let inode = folio_inode(folio); let ctx = netfs_inode(inode);
    let (mut i_size, mut remote_i_size, mut zero_point) = (0, 0, 0);
    if folio_test_dirty(folio) { return false; }
    netfs_read_sizes(inode, &mut i_size, &mut remote_i_size, &mut zero_point);
    let mut end = folio_next_pos(folio);
    if end > zero_point { spin_lock(&mut (*inode).i_lock); end = umin(end, (*ctx)._remote_i_size); if end > (*ctx)._zero_point { netfs_write_zero_point(inode, end); } spin_unlock(&mut (*inode).i_lock); }
    if folio_test_private(folio) { return false; }
    if unlikely(folio_test_private_2(folio)) { if current_is_kswapd() || gfp & __GFP_FS == 0 { return false; } folio_wait_private_2(folio); }
    fscache_note_page_release(netfs_i_cookie(ctx)); true
}

pub unsafe fn netfs_wake_collector(rreq: *mut netfs_io_request) {
    if test_bit(NETFS_RREQ_OFFLOAD_COLLECTION, &(*rreq).flags) && !test_bit(NETFS_RREQ_RETRYING, &(*rreq).flags) { queue_work(system_dfl_wq, &mut (*rreq).work); }
    else { trace_netfs_rreq(rreq, netfs_rreq_trace_wake_queue); wake_up(&mut (*rreq).waitq); }
}

pub unsafe fn netfs_subreq_clear_in_progress(subreq: *mut netfs_io_subrequest) {
    let rreq = (*subreq).rreq; let stream = &mut (*rreq).io_streams[(*subreq).stream_nr as usize];
    clear_bit_unlock(NETFS_SREQ_IN_PROGRESS, &mut (*subreq).flags); smp_mb__after_atomic();
    if list_is_first(&(*subreq).rreq_link, &stream.subrequests) || test_bit(NETFS_RREQ_RETRYING, &(*rreq).flags) { netfs_wake_collector(rreq); }
}

pub unsafe fn netfs_wait_for_in_progress_stream(rreq: *mut netfs_io_request, stream: *mut netfs_io_stream) {
    let mut myself: wait_queue_entry = core::mem::zeroed();
    let mut subreq = list_first_entry_or_null(&(*stream).subrequests, netfs_io_subrequest, rreq_link);
    while !subreq.is_null() {
        smp_rmb();
        if netfs_check_subreq_in_progress(subreq) { trace_netfs_rreq(rreq, netfs_rreq_trace_wait_quiesce); loop { prepare_to_wait(&mut (*rreq).waitq, &mut myself, TASK_UNINTERRUPTIBLE); if !netfs_check_subreq_in_progress(subreq) { break; } trace_netfs_sreq(subreq, netfs_sreq_trace_wait_for); schedule(); } }
        subreq = list_next_entry_or_null(subreq, rreq_link);
    }
    trace_netfs_rreq(rreq, netfs_rreq_trace_waited_quiesce); finish_wait(&mut (*rreq).waitq, &mut myself);
}

unsafe fn netfs_collect_in_app(rreq: *mut netfs_io_request, collector: unsafe fn(*mut netfs_io_request) -> bool) -> i32 {
    let (mut need_collect, mut inactive, mut done) = (false, true, true);
    if !netfs_check_rreq_in_progress(rreq) { trace_netfs_rreq(rreq, netfs_rreq_trace_recollect); return 1; }
    for i in 0..NR_IO_STREAMS {
        let stream = &mut (*rreq).io_streams[i as usize];
        if !stream.active { continue; }
        inactive = false; trace_netfs_collect_stream(rreq, stream);
        let subreq = list_first_entry_or_null(&stream.subrequests, netfs_io_subrequest, rreq_link);
        if !subreq.is_null() && (!netfs_check_subreq_in_progress(subreq) || test_bit(NETFS_SREQ_MADE_PROGRESS, &(*subreq).flags)) { need_collect = true; break; }
        if !subreq.is_null() || !test_bit(NETFS_RREQ_ALL_QUEUED, &(*rreq).flags) { done = false; }
    }
    if !need_collect && !inactive && !done { return 0; }
    __set_current_state(TASK_RUNNING);
    if collector(rreq) { netfs_put_request(rreq, netfs_rreq_trace_put_work_ip); return 1; }
    if inactive { WARN(true, "Failed to collect inactive req R=%08x\n", (*rreq).debug_id); cond_resched(); }
    2
}

unsafe fn netfs_wait_for_in_progress(rreq: *mut netfs_io_request, collector: unsafe fn(*mut netfs_io_request) -> bool) -> isize {
    let mut myself: wait_queue_entry = core::mem::zeroed();
    loop {
        prepare_to_wait(&mut (*rreq).waitq, &mut myself, TASK_UNINTERRUPTIBLE);
        if !test_bit(NETFS_RREQ_OFFLOAD_COLLECTION, &(*rreq).flags) {
            match netfs_collect_in_app(rreq, collector) {
                0 => (), 1 => break, 2 => { if !netfs_check_rreq_in_progress(rreq) { break; } cond_resched(); continue; }, _ => unreachable!()
            }
        }
        if !netfs_check_rreq_in_progress(rreq) { break; }
        trace_netfs_rreq(rreq, netfs_rreq_trace_wait_ip); schedule();
    }
    trace_netfs_rreq(rreq, netfs_rreq_trace_waited_ip); finish_wait(&mut (*rreq).waitq, &mut myself);
    let mut ret = (*rreq).error;
    if ret == 0 { ret = (*rreq).transferred; match (*rreq).origin { NETFS_DIO_READ | NETFS_DIO_WRITE | NETFS_READ_SINGLE | NETFS_UNBUFFERED_READ | NETFS_UNBUFFERED_WRITE => (), _ => if (*rreq).submitted < (*rreq).len { trace_netfs_failure(rreq, core::ptr::null_mut(), ret, netfs_fail_short_read); ret = -EIO; } } }
    ret
}

unsafe fn netfs_wait_for_pause(rreq: *mut netfs_io_request, collector: unsafe fn(*mut netfs_io_request) -> bool) {
    let mut myself: wait_queue_entry = core::mem::zeroed();
    loop {
        trace_netfs_rreq(rreq, netfs_rreq_trace_wait_pause); prepare_to_wait(&mut (*rreq).waitq, &mut myself, TASK_UNINTERRUPTIBLE);
        if !test_bit(NETFS_RREQ_OFFLOAD_COLLECTION, &(*rreq).flags) { match netfs_collect_in_app(rreq, collector) { 0 => (), 1 => break, 2 => { if !netfs_check_rreq_in_progress(rreq) || !test_bit(NETFS_RREQ_PAUSE, &(*rreq).flags) { break; } cond_resched(); continue; }, _ => unreachable!() } }
        if !netfs_check_rreq_in_progress(rreq) || !test_bit(NETFS_RREQ_PAUSE, &(*rreq).flags) { break; }
        schedule();
    }
    trace_netfs_rreq(rreq, netfs_rreq_trace_waited_pause); finish_wait(&mut (*rreq).waitq, &mut myself);
}

pub unsafe fn netfs_wait_for_read(rreq: *mut netfs_io_request) -> isize { netfs_wait_for_in_progress(rreq, netfs_read_collection) }
pub unsafe fn netfs_wait_for_write(rreq: *mut netfs_io_request) -> isize { netfs_wait_for_in_progress(rreq, netfs_write_collection) }
pub unsafe fn netfs_wait_for_paused_read(rreq: *mut netfs_io_request) { netfs_wait_for_pause(rreq, netfs_read_collection); }
pub unsafe fn netfs_wait_for_paused_write(rreq: *mut netfs_io_request) { netfs_wait_for_pause(rreq, netfs_write_collection); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
