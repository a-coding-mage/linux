// SPDX-License-Identifier: GPL-2.0-only
/* Network filesystem high-level buffered write support. */

/* C kernel dependencies are supplied by the surrounding translation unit. */

unsafe fn netfs_grab_folio_for_write(mapping: *mut address_space, pos: loff_t,
                                     part: usize) -> *mut folio {
    let index: pgoff_t = pos / PAGE_SIZE as loff_t;
    let mut flags: fgf_t = FGP_WRITEBEGIN;
    if mapping_large_folio_support(mapping) {
        flags |= fgf_set_order((pos % PAGE_SIZE as loff_t) as usize + part);
    }
    __filemap_get_folio(mapping, index, flags, mapping_gfp_mask(mapping))
}

unsafe fn netfs_update_i_size(ctx: *mut netfs_inode, inode: *mut inode,
                              pos: loff_t, copied: usize) {
    let end = pos + copied as loff_t;
    if end <= i_size_read(inode) { return; }
    if (*(*ctx).ops).update_i_size.is_some() {
        ((*(*ctx).ops).update_i_size.unwrap())(inode, end);
        return;
    }
    spin_lock(&mut (*inode).i_lock);
    let old = i_size_read(inode);
    if end > old {
        i_size_write(inode, end);
        let gap = SECTOR_SIZE as loff_t - (old & (SECTOR_SIZE as loff_t - 1));
        if copied > gap as usize {
            let add = DIV_ROUND_UP(copied - gap as usize, SECTOR_SIZE as usize) as blkcnt_t;
            (*inode).i_blocks = core::cmp::min(DIV_ROUND_UP(end as usize, SECTOR_SIZE as usize) as blkcnt_t,
                                               (*inode).i_blocks + add);
        }
    }
    spin_unlock(&mut (*inode).i_lock);
}

pub unsafe fn netfs_perform_write(iocb: *mut kiocb, iter: *mut iov_iter,
                                  netfs_group: *mut netfs_group) -> ssize_t {
    let file = (*iocb).ki_filp;
    let inode = file_inode(file);
    let mapping = (*inode).i_mapping;
    let ctx = netfs_inode(inode);
    let mut wbc = writeback_control { sync_mode: WB_SYNC_NONE, for_sync: true,
        nr_to_write: LONG_MAX, range_start: (*iocb).ki_pos,
        range_end: (*iocb).ki_pos + (*iter).count };
    let mut wreq: *mut netfs_io_request = core::ptr::null_mut();
    let mut written: ssize_t = 0;
    let mut ret: ssize_t = 0;
    let mut pos = (*iocb).ki_pos;
    let max_chunk = mapping_max_folio_size(mapping);
    let mut maybe_trouble = false;
    let bdp_flags = if (*iocb).ki_flags & IOCB_NOWAIT != 0 { BDP_ASYNC } else { 0 };

    if (*iocb).ki_flags & (IOCB_DSYNC | IOCB_SYNC) != 0 {
        wbc_attach_fdatawrite_inode(&mut wbc, (*mapping).host);
        ret = filemap_write_and_wait_range(mapping, pos, pos + (*iter).count as loff_t);
        if ret < 0 { wbc_detach_inode(&mut wbc); return ret; }
        wreq = netfs_begin_writethrough(iocb, (*iter).count);
        if IS_ERR(wreq) { ret = PTR_ERR(wreq); wreq = core::ptr::null_mut(); wbc_detach_inode(&mut wbc); return ret; }
        if !is_sync_kiocb(iocb) { (*wreq).iocb = iocb; }
        netfs_stat(&mut netfs_n_wh_writethrough);
    } else { netfs_stat(&mut netfs_n_wh_buffered_write); }

    while iov_iter_count(iter) != 0 {
        let mut folio: *mut folio;
        let offset0 = (pos as usize) & (max_chunk - 1);
        let part0 = core::cmp::min(max_chunk - offset0, iov_iter_count(iter));
        ret = -EFAULT;
        if fault_in_iov_iter_readable(iter, part0) == part0 { break; }
        folio = netfs_grab_folio_for_write(mapping, pos, part0);
        if IS_ERR(folio) { ret = PTR_ERR(folio); break; }
        let flen = folio_size(folio);
        let fpos = folio_pos(folio);
        let offset = (pos - fpos) as usize;
        let part = core::cmp::min(flen - offset, part0);
        if !folio_get_private(folio).is_null() && folio_wait_writeback_killable(folio) != 0 {
            ret = if written != 0 { -EINTR } else { -ERESTARTSYS }; folio_unlock(folio); folio_put(folio); break;
        }
        if signal_pending(current) != 0 { ret = if written != 0 { -EINTR } else { -ERESTARTSYS }; folio_unlock(folio); folio_put(folio); break; }
        let finfo = netfs_folio_info(folio);
        let group = netfs_folio_group(folio);
        if group != netfs_group && group != NETFS_FOLIO_COPY_TO_CACHE && !group.is_null() {
            WARN_ON_ONCE(netfs_group.is_null());
            folio_unlock(folio); folio_put(folio);
            ret = filemap_write_and_wait_range(mapping, fpos, fpos + flen as loff_t - 1);
            if ret < 0 { break; } else { continue; }
        }
        let mut copied: usize;
        let trace: enum_netfs_folio_trace;
        if folio_test_uptodate(folio) {
            if mapping_writably_mapped(mapping) { flush_dcache_folio(folio); }
            copied = copy_folio_from_iter_atomic(folio, offset, part, iter);
            if copied == 0 { ret = -EFAULT; folio_unlock(folio); folio_put(folio); break; }
            trace = netfs_folio_is_uptodate;
        } else if fpos >= netfs_read_zero_point(inode) {
            folio_zero_segment(folio, 0, offset);
            copied = copy_folio_from_iter_atomic(folio, offset, part, iter);
            if copied == 0 { ret = -EFAULT; folio_unlock(folio); folio_put(folio); break; }
            folio_zero_segment(folio, offset + copied, flen);
            trace = if !finfo.is_null() { netfs_modify_and_clear_rm_finfo } else { netfs_modify_and_clear };
        } else if !maybe_trouble && offset == 0 && part >= flen {
            copied = copy_folio_from_iter_atomic(folio, offset, part, iter);
            if copied == part { trace = if !finfo.is_null() { netfs_whole_folio_modify_filled } else { netfs_whole_folio_modify }; }
            else { maybe_trouble = true; iov_iter_revert(iter, copied); folio_unlock(folio); folio_put(folio); continue; }
        } else if netfs_is_cache_maybe_enabled(ctx) {
            if !finfo.is_null() { folio_unlock(folio); folio_put(folio); continue; }
            ret = netfs_prefetch_for_write(file, folio, offset, part);
            if ret < 0 { folio_unlock(folio); folio_put(folio); break; }
            copied = copy_folio_from_iter_atomic(folio, offset, part, iter);
            if copied == 0 { ret = -EFAULT; folio_unlock(folio); folio_put(folio); break; }
            trace = netfs_just_prefetch;
        } else {
            copied = copy_folio_from_iter_atomic(folio, offset, part, iter);
            if copied == 0 { ret = -EFAULT; folio_unlock(folio); folio_put(folio); break; }
            trace = netfs_streaming_write;
        }
        folio_mark_uptodate(folio);
        if folio_get_private(folio) != netfs_group { if netfs_group.is_null() { folio_detach_private(folio); } else { folio_attach_private(folio, netfs_get_group(netfs_group)); } }
        trace_netfs_folio(folio, trace);
        flush_dcache_folio(folio);
        netfs_update_i_size(ctx, inode, pos, copied);
        pos += copied as loff_t; written += copied as ssize_t;
        if wreq.is_null() { folio_mark_dirty(folio); folio_unlock(folio); } else { netfs_advance_writethrough(wreq, &mut wbc, folio, copied, offset + copied == flen, core::ptr::null_mut()); }
        folio_put(folio);
        ret = balance_dirty_pages_ratelimited_flags(mapping, bdp_flags);
        if ret < 0 { break; }
        cond_resched();
    }
    if written != 0 { set_bit(NETFS_ICTX_MODIFIED_ATTR, &mut (*ctx).flags); if (*(*ctx).ops).post_modify.is_some() { ((*(*ctx).ops).post_modify.unwrap())(inode); } }
    if !wreq.is_null() { let r = netfs_end_writethrough(wreq, &mut wbc, core::ptr::null_mut()); wbc_detach_inode(&mut wbc); if r == -EIOCBQUEUED { return r; } if ret == 0 && r < 0 { ret = r; } }
    (*iocb).ki_pos += written as loff_t; if written != 0 { written } else { ret }
}

pub unsafe fn netfs_buffered_write_iter_locked(iocb: *mut kiocb, from: *mut iov_iter,
                                                group: *mut netfs_group) -> ssize_t {
    let file = (*iocb).ki_filp;
    trace_netfs_write_iter(iocb, from);
    let mut ret = file_remove_privs(file); if ret != 0 { return ret; }
    ret = file_update_time(file); if ret != 0 { return ret; }
    netfs_perform_write(iocb, from, group)
}

pub unsafe fn netfs_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t {
    let file = (*iocb).ki_filp; let inode = (*file).f_mapping.host; let ctx = netfs_inode(inode);
    if iov_iter_count(from) == 0 { return 0; }
    if (*iocb).ki_flags & IOCB_DIRECT != 0 || test_bit(NETFS_ICTX_UNBUFFERED, &(*ctx).flags) != 0 { return netfs_unbuffered_write_iter(iocb, from); }
    let mut ret = netfs_start_io_write(inode); if ret < 0 { return ret; }
    ret = generic_write_checks(iocb, from); if ret > 0 { ret = netfs_buffered_write_iter_locked(iocb, from, core::ptr::null_mut()); }
    netfs_end_io_write(inode); if ret > 0 { ret = generic_write_sync(iocb, ret); } ret
}

pub unsafe fn netfs_page_mkwrite(vmf: *mut vm_fault, group: *mut netfs_group) -> vm_fault_t {
    let folio = page_folio((*vmf).page); let file = (*(*vmf).vma).vm_file; let mapping = (*file).f_mapping;
    let inode = file_inode(file); let ctx = netfs_inode(inode); let mut ret = VM_FAULT_NOPAGE;
    sb_start_pagefault((*inode).i_sb);
    if folio_lock_killable(folio) < 0 { sb_end_pagefault((*inode).i_sb); return ret; }
    if (*folio).mapping != mapping || folio_wait_writeback_killable(folio) < 0 { folio_unlock(folio); sb_end_pagefault((*inode).i_sb); return ret; }
    if !folio_test_uptodate(folio) { ret = VM_FAULT_SIGBUS; folio_unlock(folio); sb_end_pagefault((*inode).i_sb); return ret; }
    let old = netfs_folio_group(folio);
    if !old.is_null() && old != group && old != NETFS_FOLIO_COPY_TO_CACHE { folio_unlock(folio); let e = filemap_fdatawrite_range(mapping, folio_pos(folio), folio_next_pos(folio)); ret = match e { 0 => VM_FAULT_RETRY, -ENOMEM => VM_FAULT_OOM, _ => VM_FAULT_SIGBUS }; sb_end_pagefault((*inode).i_sb); return ret; }
    trace_netfs_folio(folio, if folio_test_dirty(folio) { netfs_folio_trace_mkwrite_plus } else { netfs_folio_trace_mkwrite });
    let priv = folio_get_private(folio); if priv != group { if group.is_null() && priv == NETFS_FOLIO_COPY_TO_CACHE { folio_detach_private(folio); } else if !group.is_null() && priv == NETFS_FOLIO_COPY_TO_CACHE { folio_change_private(folio, netfs_get_group(group)); } else if !group.is_null() && priv.is_null() { folio_attach_private(folio, netfs_get_group(group)); } else { WARN_ON_ONCE(true); } }
    file_update_time(file); set_bit(NETFS_ICTX_MODIFIED_ATTR, &mut (*ctx).flags); if (*(*ctx).ops).post_modify.is_some() { ((*(*ctx).ops).post_modify.unwrap())(inode); }
    ret = VM_FAULT_LOCKED; sb_end_pagefault((*inode).i_sb); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
