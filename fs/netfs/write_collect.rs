// SPDX-License-Identifier: GPL-2.0-only
/* Network filesystem write subrequest result collection, assessment
 * and retrying.
 *
 * Copyright (C) 2024 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel dependencies supplied by the surrounding translation unit.

const HIT_PENDING: u32 = 0x01;
const NEED_REASSESS: u32 = 0x02;
const MADE_PROGRESS: u32 = 0x04;
const NEED_UNLOCK: u32 = 0x08;
const NEED_RETRY: u32 = 0x10;
const SAW_FAILURE: u32 = 0x20;

unsafe fn netfs_dump_request(rreq: *const netfs_io_request) {
    pr_err!("Request R={:08x} r={} fl={:x} or={:x} e={}\n", (*rreq).debug_id, refcount_read(&(*rreq).ref_), (*rreq).flags, (*rreq).origin, (*rreq).error);
    pr_err!("  st={:x} tsl={:x}/{:x}/{:x}\n", (*rreq).start, (*rreq).transferred, (*rreq).submitted, (*rreq).len);
    pr_err!("  cci={:x}/{:x}/{:x}\n", (*rreq).cleaned_to, (*rreq).collected_to, atomic64_read(&(*rreq).issued_to));
    pr_err!("  iw={:pSR}\n", (*(*rreq).netfs_ops).issue_write);
    for i in 0..NR_IO_STREAMS {
        let s = &(*rreq).io_streams[i];
        pr_err!("  str[{:x}] s={:x} e={} acnf={},{},{},{}\n", s.stream_nr, s.source, s.error, s.avail, s.active, s.need_retry, s.failed);
        pr_err!("  str[{:x}] ct={:x} t={:x}\n", s.stream_nr, s.collected_to, s.transferred);
        // list_for_each_entry(sreq, &s->subrequests, rreq_link)
        let mut pos = s.subrequests.next;
        while pos != &s.subrequests as *const _ as *mut _ {
            let sreq = container_of!(pos, netfs_io_subrequest, rreq_link);
            pr_err!("  sreq[{:x}:{:x}] sc={} s={:x} t={:x}/{:x} r={} f={:x}\n", (*sreq).stream_nr, (*sreq).debug_index, (*sreq).source, (*sreq).start, (*sreq).transferred, (*sreq).len, refcount_read(&(*sreq).ref_), (*sreq).flags);
            pos = (*pos).next;
        }
    }
}

pub unsafe fn netfs_folio_written_back(folio: *mut folio) -> i32 {
    let mut why = netfs_folio_trace_clear;
    let inode = folio_inode(folio);
    let ictx = netfs_inode(inode);
    let mut gcount = 0;
    let mut group = core::ptr::null_mut();

    let finfo = netfs_folio_info(folio);
    if !finfo.is_null() {
        let fend = folio_pos(folio) + (*finfo).dirty_offset + (*finfo).dirty_len;
        spin_lock(&(*ictx).inode.i_lock);
        if fend > (*ictx)._zero_point { netfs_write_zero_point(inode, fend); }
        spin_unlock(&(*ictx).inode.i_lock);
        folio_detach_private(folio);
        group = (*finfo).netfs_group;
        gcount += 1;
        kfree(finfo);
        why = netfs_folio_trace_clear_s;
    } else {
        group = netfs_folio_group(folio);
        if !group.is_null() {
            if group == NETFS_FOLIO_COPY_TO_CACHE { why = netfs_folio_trace_clear_cc; folio_detach_private(folio); }
            else {
                why = netfs_folio_trace_redirtied;
                if !folio_test_dirty(folio) { folio_detach_private(folio); gcount += 1; why = netfs_folio_trace_clear_g; }
            }
        }
    }
    trace_netfs_folio(folio, why);
    folio_end_writeback(folio);
    gcount
}

unsafe fn netfs_writeback_unlock_folios(wreq: *mut netfs_io_request, notes: *mut u32) {
    let mut folioq = (*wreq).buffer.tail;
    let collected_to = (*wreq).collected_to;
    let mut slot = (*wreq).buffer.first_tail_slot;
    if folioq.is_null() { if WARN_ON_ONCE(true) { pr_err!("[!] Writeback unlock found empty rolling buffer!\n"); netfs_dump_request(wreq); } return; }
    if (*wreq).origin == NETFS_PGPRIV2_COPY_TO_CACHE { if netfs_pgpriv2_unlock_copied_folios(wreq) { *notes |= MADE_PROGRESS; } return; }
    if slot >= folioq_nr_slots(folioq) { folioq = rolling_buffer_delete_spent(&mut (*wreq).buffer); if folioq.is_null() { return; } slot = 0; }
    loop {
        let folio = folioq_folio(folioq, slot);
        if WARN_ONCE(!folio_test_writeback(folio), "R={:08x}: folio {:x} is not under writeback\n", (*wreq).debug_id, (*folio).index) { trace_netfs_folio(folio, netfs_folio_trace_not_under_wback); }
        let fpos = folio_pos(folio); let fsize = folio_size(folio); let finfo = netfs_folio_info(folio);
        let flen = if !finfo.is_null() { (*finfo).dirty_offset + (*finfo).dirty_len } else { fsize };
        let fend = core::cmp::min(fpos + flen, (*wreq).i_size);
        trace_netfs_collect_folio(wreq, folio, fend, collected_to);
        if collected_to < fend { break; }
        (*wreq).nr_group_rel += netfs_folio_written_back(folio);
        (*wreq).cleaned_to = fpos + fsize; *notes |= MADE_PROGRESS;
        folioq_clear(folioq, slot); slot += 1;
        if slot >= folioq_nr_slots(folioq) { folioq = rolling_buffer_delete_spent(&mut (*wreq).buffer); if folioq.is_null() { break; } slot = 0; }
        if fpos + fsize >= collected_to { break; }
    }
    (*wreq).buffer.tail = folioq; (*wreq).buffer.first_tail_slot = slot;
}

unsafe fn netfs_collect_write_results(wreq: *mut netfs_io_request) {
    let mut notes: u32;
    _enter!("{:x}-{:x}", (*wreq).start, (*wreq).start + (*wreq).len);
    trace_netfs_collect(wreq); trace_netfs_rreq(wreq, netfs_rreq_trace_collect);
    'reassess_streams: loop {
        let issued_to = atomic64_read(&(*wreq).issued_to); smp_rmb!();
        let mut collected_to = ULLONG_MAX;
        notes = if (*wreq).origin == NETFS_WRITEBACK || (*wreq).origin == NETFS_WRITETHROUGH || (*wreq).origin == NETFS_PGPRIV2_COPY_TO_CACHE { NEED_UNLOCK } else { 0 };
        for s in 0..NR_IO_STREAMS { let stream = &mut (*wreq).io_streams[s]; if !smp_load_acquire(&stream.active) { continue; }
            let mut front = list_first_entry_or_null_acquire(&stream.subrequests);
            while !front.is_null() { trace_netfs_collect_sreq(wreq, front);
                if stream.collected_to < (*front).start { trace_netfs_collect_gap(wreq, stream, issued_to, 'F'); stream.collected_to = (*front).start; }
                if netfs_check_subreq_in_progress(front) { notes |= HIT_PENDING; break; } smp_rmb!();
                if stream.failed { stream.collected_to = (*front).start + (*front).len; notes |= MADE_PROGRESS | SAW_FAILURE; }
                else { if (*front).start + (*front).transferred > stream.collected_to { stream.collected_to = (*front).start + (*front).transferred; stream.transferred = stream.collected_to - (*wreq).start; stream.transferred_valid = true; notes |= MADE_PROGRESS; }
                    if test_bit(NETFS_SREQ_FAILED, &(*front).flags) { stream.failed = true; stream.error = (*front).error; if stream.source == NETFS_UPLOAD_TO_SERVER { mapping_set_error((*wreq).mapping, (*front).error); } notes |= NEED_REASSESS | SAW_FAILURE; break; }
                    if (*front).transferred < (*front).len { stream.need_retry = true; notes |= NEED_RETRY | MADE_PROGRESS; break; }
                }
                spin_lock(&(*wreq).lock); let remove = front; list_del_init(&mut (*front).rreq_link); front = list_first_entry_or_null(&stream.subrequests); spin_unlock(&(*wreq).lock);
                netfs_put_subrequest(remove, if notes & SAW_FAILURE != 0 { netfs_sreq_trace_put_cancel } else { netfs_sreq_trace_put_done });
            }
            if front.is_null() && issued_to > stream.collected_to { trace_netfs_collect_gap(wreq, stream, issued_to, 'E'); stream.collected_to = issued_to; }
            if stream.collected_to < collected_to { collected_to = stream.collected_to; }
        }
        if collected_to != ULLONG_MAX && collected_to > (*wreq).collected_to { (*wreq).collected_to = collected_to; }
        for s in 0..NR_IO_STREAMS { let stream = &(*wreq).io_streams[s]; if stream.active { trace_netfs_collect_stream(wreq, stream); } }
        trace_netfs_collect_state(wreq, (*wreq).collected_to, notes);
        if notes & NEED_UNLOCK != 0 { if (*wreq).cleaned_to < (*wreq).collected_to { netfs_writeback_unlock_folios(wreq, &mut notes); } } else { (*wreq).cleaned_to = (*wreq).collected_to; }
        if notes & NEED_RETRY != 0 { netfs_retry_writes(wreq); break; }
        if notes & MADE_PROGRESS != 0 { netfs_wake_rreq_flag(wreq, NETFS_RREQ_PAUSE, netfs_rreq_trace_unpause); continue; }
        if notes & NEED_REASSESS != 0 { continue; }
        break 'reassess_streams;
    }
    netfs_put_group_many((*wreq).group, (*wreq).nr_group_rel); (*wreq).nr_group_rel = 0; _leave!(" = {:x}", notes);
}

pub unsafe fn netfs_write_collection(wreq: *mut netfs_io_request) -> bool {
    let ictx = netfs_inode((*wreq).inode); netfs_collect_write_results(wreq);
    if !test_bit(NETFS_RREQ_ALL_QUEUED, &(*wreq).flags) { return false; } smp_rmb!();
    let mut transferred = LONG_MAX; let mut valid = false;
    for s in 0..NR_IO_STREAMS { let stream = &(*wreq).io_streams[s]; if !stream.active { continue; } if !list_empty(&stream.subrequests) { return false; } if stream.transferred_valid && stream.transferred < transferred { transferred = stream.transferred; valid = true; } }
    if valid { (*wreq).transferred = transferred; } trace_netfs_rreq(wreq, netfs_rreq_trace_write_done);
    if (*wreq).io_streams[1].active && (*wreq).io_streams[1].failed && (*(*ictx).ops).invalidate_cache.is_some() { (*(*ictx).ops).invalidate_cache.unwrap()(wreq); }
    netfs_wake_rreq_flag(wreq, NETFS_RREQ_IN_PROGRESS, netfs_rreq_trace_wake_ip);
    match (*wreq).origin { NETFS_WRITEBACK | NETFS_WRITEBACK_SINGLE | NETFS_WRITETHROUGH => netfs_wb_end(ictx), _ => {} }
    if !(*wreq).iocb.is_null() { let written = core::cmp::min((*wreq).transferred, (*wreq).len); (*(*wreq).iocb).ki_pos += written; if let Some(done) = (*(*wreq).iocb).ki_complete { done((*wreq).iocb, if (*wreq).error != 0 { (*wreq).error } else { written as _ }); } (*wreq).iocb = VFS_PTR_POISON; }
    netfs_clear_subrequests(wreq); true
}

pub unsafe fn netfs_write_collection_worker(work: *mut work_struct) {
    let rreq = container_of!(work, netfs_io_request, work); netfs_see_request(rreq, netfs_rreq_trace_see_work);
    if netfs_check_rreq_in_progress(rreq) { if netfs_write_collection(rreq) { netfs_put_request(rreq, netfs_rreq_trace_put_work_ip); } else { netfs_see_request(rreq, netfs_rreq_trace_see_work_complete); } }
}

pub unsafe fn netfs_write_subrequest_terminated(op: *mut core::ffi::c_void, transferred_or_error: isize) {
    let subreq = op as *mut netfs_io_subrequest; let wreq = (*subreq).rreq;
    match (*subreq).source { NETFS_UPLOAD_TO_SERVER => netfs_stat(&netfs_n_wh_upload_done), NETFS_WRITE_TO_CACHE => netfs_stat(&netfs_n_wh_write_done), _ => BUG!() }
    if IS_ERR_VALUE(transferred_or_error) { (*subreq).error = transferred_or_error; if !test_bit(NETFS_SREQ_NEED_RETRY, &(*subreq).flags) { set_bit(NETFS_SREQ_FAILED, &mut (*subreq).flags); trace_netfs_failure(wreq, subreq, transferred_or_error, netfs_fail_write); } match (*subreq).source { NETFS_WRITE_TO_CACHE => netfs_stat(&netfs_n_wh_write_failed), NETFS_UPLOAD_TO_SERVER => netfs_stat(&netfs_n_wh_upload_failed), _ => {} } trace_netfs_rreq(wreq, netfs_rreq_trace_set_pause); set_bit(NETFS_RREQ_PAUSE, &mut (*wreq).flags); }
    else { if WARN(transferred_or_error > (*subreq).len - (*subreq).transferred, "Subreq excess write") { transferred_or_error = (*subreq).len - (*subreq).transferred; } (*subreq).error = 0; (*subreq).transferred += transferred_or_error as usize; if (*subreq).transferred < (*subreq).len { set_bit(NETFS_SREQ_NEED_RETRY, &mut (*subreq).flags); } }
    trace_netfs_sreq(subreq, netfs_sreq_trace_terminated); netfs_subreq_clear_in_progress(subreq); netfs_put_subrequest(subreq, netfs_sreq_trace_put_terminated);
}

// EXPORT_SYMBOL(netfs_write_subrequest_terminated)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
