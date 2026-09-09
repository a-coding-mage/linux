// SPDX-License-Identifier: GPL-2.0-only
/* Network filesystem read subrequest result collection, assessment and
 * retrying.
 */

const HIT_PENDING: u32 = 0x01;
const MADE_PROGRESS: u32 = 0x04;
const BUFFERED: u32 = 0x08;
const NEED_RETRY: u32 = 0x10;
const COPY_TO_CACHE: u32 = 0x40;
const ABANDON_SREQ: u32 = 0x80;

unsafe fn netfs_clear_unread(subreq: *mut netfs_io_subrequest) {
    netfs_reset_iter(subreq);
    WARN_ON_ONCE((*subreq).len - (*subreq).transferred != iov_iter_count(&(*subreq).io_iter));
    iov_iter_zero(iov_iter_count(&(*subreq).io_iter), &mut (*subreq).io_iter);
    if (*subreq).start + (*subreq).transferred >= (*(*subreq).rreq).i_size {
        __set_bit(NETFS_SREQ_HIT_EOF, &mut (*subreq).flags);
    }
}

unsafe fn netfs_unlock_read_folio(rreq: *mut netfs_io_request, folioq: *mut folio_queue, slot: i32) {
    let mut finfo: *mut netfs_folio;
    let folio = folioq_folio(folioq, slot);
    if unlikely(folio_pos(folio) < (*rreq).abandon_to) {
        trace_netfs_folio(folio, netfs_folio_trace_abandon);
        goto_just_unlock(rreq, folioq, slot, folio);
        return;
    }
    flush_dcache_folio(folio);
    folio_mark_uptodate(folio);
    if !test_bit(NETFS_RREQ_USE_PGPRIV2, &(*rreq).flags) {
        finfo = netfs_folio_info(folio);
        if !finfo.is_null() {
            trace_netfs_folio(folio, netfs_folio_trace_filled_gaps);
            if !(*finfo).netfs_group.is_null() { folio_change_private(folio, (*finfo).netfs_group); }
            else { folio_detach_private(folio); }
            kfree(finfo);
        }
        if test_bit(NETFS_RREQ_FOLIO_COPY_TO_CACHE, &(*rreq).flags) {
            if !WARN_ON_ONCE(!folio_get_private(folio).is_null()) {
                trace_netfs_folio(folio, netfs_folio_trace_copy_to_cache);
                folio_attach_private(folio, NETFS_FOLIO_COPY_TO_CACHE);
                folio_mark_dirty(folio);
            }
        } else { trace_netfs_folio(folio, netfs_folio_trace_read_done); }
        folioq_clear(folioq, slot);
    } else {
        // TODO: Use of PG_private_2 is deprecated.
        if test_bit(NETFS_RREQ_FOLIO_COPY_TO_CACHE, &(*rreq).flags) { netfs_pgpriv2_copy_to_cache(rreq, folio); }
    }
    goto_just_unlock(rreq, folioq, slot, folio);
}

unsafe fn goto_just_unlock(rreq: *mut netfs_io_request, folioq: *mut folio_queue, slot: i32, folio: *mut folio) {
    if folio == (*rreq).no_unlock_folio && test_bit(NETFS_RREQ_NO_UNLOCK_FOLIO, &(*rreq).flags) { _debug("no unlock"); }
    else { trace_netfs_folio(folio, netfs_folio_trace_read_unlock); folio_unlock(folio); }
    folioq_clear(folioq, slot);
}

unsafe fn netfs_read_unlock_folios(rreq: *mut netfs_io_request, notes: *mut u32) {
    let mut folioq = (*rreq).buffer.tail;
    let collected_to = (*rreq).collected_to;
    let mut slot = (*rreq).buffer.first_tail_slot;
    if (*rreq).cleaned_to >= (*rreq).collected_to { return; }
    // TODO: Begin decryption
    if slot >= folioq_nr_slots(folioq) { folioq = rolling_buffer_delete_spent(&mut (*rreq).buffer); if folioq.is_null() { (*rreq).front_folio_order = 0; return; } slot = 0; }
    loop {
        if *notes & COPY_TO_CACHE != 0 { set_bit(NETFS_RREQ_FOLIO_COPY_TO_CACHE, &mut (*rreq).flags); }
        let folio = folioq_folio(folioq, slot);
        if WARN_ONCE(!folio_test_locked(folio), "R=%08x: folio %lx is not locked\n", (*rreq).debug_id, (*folio).index) { trace_netfs_folio(folio, netfs_folio_trace_not_locked); }
        let order = folioq_folio_order(folioq, slot); (*rreq).front_folio_order = order;
        let fsize = PAGE_SIZE << order; let fpos = folio_pos(folio); let fend = fpos + fsize;
        trace_netfs_collect_folio(rreq, folio, fend, collected_to);
        if collected_to < fend { break; }
        netfs_unlock_read_folio(rreq, folioq, slot); WRITE_ONCE((*rreq).cleaned_to, fpos + fsize); *notes |= MADE_PROGRESS;
        clear_bit(NETFS_RREQ_FOLIO_COPY_TO_CACHE, &mut (*rreq).flags); folioq_clear(folioq, slot); slot += 1;
        if slot >= folioq_nr_slots(folioq) { folioq = rolling_buffer_delete_spent(&mut (*rreq).buffer); if folioq.is_null() { break; } slot = 0; trace_netfs_folioq(folioq, netfs_trace_folioq_read_progress); }
        if fpos + fsize >= collected_to { break; }
    }
    (*rreq).buffer.tail = folioq; (*rreq).buffer.first_tail_slot = slot;
}

unsafe fn netfs_collect_read_results(rreq: *mut netfs_io_request) {
    let stream = &mut (*rreq).io_streams[0]; let mut notes: u32;
    _enter!("%llx-%llx", (*rreq).start, (*rreq).start + (*rreq).len); trace_netfs_rreq(rreq, netfs_rreq_trace_collect); trace_netfs_collect(rreq);
    'reassess: loop {
        notes = if (*rreq).origin == NETFS_READAHEAD || (*rreq).origin == NETFS_READPAGE || (*rreq).origin == NETFS_READ_FOR_WRITE { BUFFERED } else { 0 };
        let mut front = list_first_entry_or_null_acquire(&stream.subrequests);
        while !front.is_null() {
            trace_netfs_collect_sreq(rreq, front); _debug!("sreq [%x] %llx %zx/%zx", (*front).debug_index, (*front).start, (*front).transferred, (*front).len);
            if stream.collected_to < (*front).start { trace_netfs_collect_gap(rreq, stream, (*front).start, 'F'); stream.collected_to = (*front).start; }
            if netfs_check_subreq_in_progress(front) { notes |= HIT_PENDING; } smp_rmb(); let mut transferred = READ_ONCE((*front).transferred);
            if notes & BUFFERED != 0 { let fsize = PAGE_SIZE << (*rreq).front_folio_order; if notes & HIT_PENDING == 0 && (*front).error == 0 && transferred < (*front).len && (test_bit(NETFS_SREQ_HIT_EOF, &(*front).flags) || test_bit(NETFS_SREQ_CLEAR_TAIL, &(*front).flags)) { netfs_clear_unread(front); (*front).transferred = (*front).len; transferred = (*front).len; trace_netfs_sreq(front, netfs_sreq_trace_clear); } stream.collected_to = (*front).start + transferred; (*rreq).collected_to = stream.collected_to; if test_bit(NETFS_SREQ_COPY_TO_CACHE, &(*front).flags) { notes |= COPY_TO_CACHE; } if test_bit(NETFS_SREQ_FAILED, &(*front).flags) { (*rreq).abandon_to = (*front).start + (*front).len; (*front).transferred = (*front).len; transferred = (*front).len; trace_netfs_rreq(rreq, netfs_rreq_trace_set_abandon); } if (*front).start + transferred >= (*rreq).cleaned_to + fsize || test_bit(NETFS_SREQ_HIT_EOF, &(*front).flags) { netfs_read_unlock_folios(rreq, &mut notes); } } else { stream.collected_to = (*front).start + transferred; (*rreq).collected_to = stream.collected_to; }
            if notes & HIT_PENDING != 0 { break; }
            if test_bit(NETFS_SREQ_FAILED, &(*front).flags) { if !stream.failed { stream.error = (*front).error; (*rreq).error = (*front).error; set_bit(NETFS_RREQ_FAILED, &mut (*rreq).flags); stream.failed = true; } notes |= MADE_PROGRESS | ABANDON_SREQ; }
            else if test_bit(NETFS_SREQ_NEED_RETRY, &(*front).flags) { stream.need_retry = true; notes |= NEED_RETRY | MADE_PROGRESS; break; }
            else if test_bit(NETFS_RREQ_SHORT_TRANSFER, &(*rreq).flags) { notes |= MADE_PROGRESS; }
            else { if !stream.failed { stream.transferred += transferred; stream.transferred_valid = true; } if (*front).transferred < (*front).len { set_bit(NETFS_RREQ_SHORT_TRANSFER, &mut (*rreq).flags); } notes |= MADE_PROGRESS; }
            stream.source = (*front).source; spin_lock(&mut (*rreq).lock); let remove = front; list_del_init(&mut (*front).rreq_link); front = list_first_entry_or_null(&stream.subrequests); spin_unlock(&mut (*rreq).lock); netfs_put_subrequest(remove, if notes & ABANDON_SREQ != 0 { netfs_sreq_trace_put_abandon } else { netfs_sreq_trace_put_done });
        }
        trace_netfs_collect_stream(rreq, stream); trace_netfs_collect_state(rreq, (*rreq).collected_to, notes); if notes & BUFFERED == 0 { (*rreq).cleaned_to = (*rreq).collected_to; } if notes & NEED_RETRY != 0 { netfs_retry_reads(rreq); break; } if notes & MADE_PROGRESS != 0 { netfs_wake_rreq_flag(rreq, NETFS_RREQ_PAUSE, netfs_rreq_trace_unpause); continue 'reassess; } break;
    }
    _leave!(" = %x", notes);
}

pub unsafe fn netfs_read_collection(rreq: *mut netfs_io_request) -> bool { let stream = &mut (*rreq).io_streams[0]; netfs_collect_read_results(rreq); if !test_bit(NETFS_RREQ_ALL_QUEUED, &(*rreq).flags) { return false; } smp_rmb(); if !list_empty(&stream.subrequests) { return false; } (*rreq).transferred = stream.transferred; trace_netfs_rreq(rreq, netfs_rreq_trace_complete); match (*rreq).origin { NETFS_UNBUFFERED_READ | NETFS_DIO_READ | NETFS_READ_GAPS => netfs_rreq_assess_dio(rreq), NETFS_READ_SINGLE => netfs_rreq_assess_single(rreq), _ => {} } task_io_account_read((*rreq).transferred); netfs_wake_rreq_flag(rreq, NETFS_RREQ_IN_PROGRESS, netfs_rreq_trace_wake_ip); trace_netfs_rreq(rreq, netfs_rreq_trace_done); netfs_clear_subrequests(rreq); netfs_unlock_abandoned_read_pages(rreq); if unlikely((*rreq).copy_to_cache) { netfs_pgpriv2_end_copy_to_cache(rreq); } true }

unsafe fn netfs_rreq_assess_dio(rreq: *mut netfs_io_request) { if (*rreq).origin == NETFS_UNBUFFERED_READ || (*rreq).origin == NETFS_DIO_READ { for i in 0..(*rreq).direct_bv_count { flush_dcache_page((*rreq).direct_bv[i].bv_page); set_page_dirty((*rreq).direct_bv[i].bv_page); } } if !(*rreq).iocb.is_null() { (*(*rreq).iocb).ki_pos += (*rreq).transferred; if let Some(done) = (*(*rreq).iocb).ki_complete { done((*rreq).iocb, if (*rreq).error != 0 { (*rreq).error as isize } else { (*rreq).transferred as isize }); } } if let Some(done) = (*rreq).netfs_ops.done { done(rreq); } if (*rreq).origin == NETFS_UNBUFFERED_READ || (*rreq).origin == NETFS_DIO_READ { inode_dio_end((*rreq).inode); } }

unsafe fn netfs_rreq_assess_single(rreq: *mut netfs_io_request) { let stream = &mut (*rreq).io_streams[0]; if (*rreq).error == 0 && stream.source == NETFS_DOWNLOAD_FROM_SERVER && fscache_resources_valid(&(*rreq).cache_resources) { netfs_single_mark_inode_dirty((*rreq).inode); } if !(*rreq).iocb.is_null() { (*(*rreq).iocb).ki_pos += (*rreq).transferred; if let Some(done) = (*(*rreq).iocb).ki_complete { done((*rreq).iocb, if (*rreq).error != 0 { (*rreq).error as isize } else { (*rreq).transferred as isize }); } } if let Some(done) = (*rreq).netfs_ops.done { done(rreq); } }

pub unsafe fn netfs_read_collection_worker(work: *mut work_struct) { let rreq = container_of!(work, netfs_io_request, work); netfs_see_request(rreq, netfs_rreq_trace_see_work); if netfs_check_rreq_in_progress(rreq) { if netfs_read_collection(rreq) { netfs_put_request(rreq, netfs_rreq_trace_put_work_ip); } else { netfs_see_request(rreq, netfs_rreq_trace_see_work_complete); } } }

pub unsafe fn netfs_read_subreq_progress(subreq: *mut netfs_io_subrequest) { let rreq = (*subreq).rreq; let stream = &mut (*rreq).io_streams[0]; let fsize = PAGE_SIZE << (*rreq).front_folio_order; trace_netfs_sreq(subreq, netfs_sreq_trace_progress); if (*subreq).start + (*subreq).transferred > (*rreq).cleaned_to + fsize && ((*rreq).origin == NETFS_READAHEAD || (*rreq).origin == NETFS_READPAGE || (*rreq).origin == NETFS_READ_FOR_WRITE) && list_is_first(&(*subreq).rreq_link, &stream.subrequests) { __set_bit(NETFS_SREQ_MADE_PROGRESS, &mut (*subreq).flags); netfs_wake_collector(rreq); } }

pub unsafe fn netfs_read_subreq_terminated(subreq: *mut netfs_io_subrequest) { let rreq = (*subreq).rreq; if (*subreq).error == 0 && (*subreq).transferred < (*subreq).len { if test_bit(NETFS_SREQ_HIT_EOF, &(*subreq).flags) {} else if test_bit(NETFS_SREQ_CLEAR_TAIL, &(*subreq).flags) {} else if test_bit(NETFS_SREQ_NEED_RETRY, &(*subreq).flags) {} else if test_bit(NETFS_SREQ_MADE_PROGRESS, &(*subreq).flags) { __set_bit(NETFS_SREQ_NEED_RETRY, &mut (*subreq).flags); } else { __set_bit(NETFS_SREQ_FAILED, &mut (*subreq).flags); (*subreq).error = -ENODATA; } } if test_bit(NETFS_SREQ_NEED_RETRY, &(*subreq).flags) { set_bit(NETFS_RREQ_PAUSE, &mut (*rreq).flags); } else if (*subreq).error < 0 { if (*subreq).source == NETFS_READ_FROM_CACHE { __set_bit(NETFS_SREQ_NEED_RETRY, &mut (*subreq).flags); } else { __set_bit(NETFS_SREQ_FAILED, &mut (*subreq).flags); } set_bit(NETFS_RREQ_PAUSE, &mut (*rreq).flags); } netfs_subreq_clear_in_progress(subreq); netfs_put_subrequest(subreq, netfs_sreq_trace_put_terminated); }

pub unsafe fn netfs_cancel_read(subreq: *mut netfs_io_subrequest, error: i32) { (*subreq).error = error; __set_bit(NETFS_SREQ_FAILED, &mut (*subreq).flags); netfs_read_subreq_terminated(subreq); }
pub unsafe fn netfs_cache_read_terminated(priv_: *mut core::ffi::c_void, transferred_or_error: isize) { let subreq = priv_ as *mut netfs_io_subrequest; if transferred_or_error > 0 { (*subreq).error = 0; (*subreq).transferred += transferred_or_error as usize; __set_bit(NETFS_SREQ_MADE_PROGRESS, &mut (*subreq).flags); } else { (*subreq).error = transferred_or_error as i32; } netfs_read_subreq_terminated(subreq); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
