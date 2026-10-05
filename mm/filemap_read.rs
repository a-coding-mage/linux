// SPDX-License-Identifier: GPL-2.0-only
// Original mm/filemap.c:2455-3272.
unsafe fn shrink_readahead_size_eio(ra: *mut file_ra_state) {
    (*ra).ra_pages /= 4;
}
unsafe fn filemap_get_read_batch(
    mapping: *mut address_space,
    index: Pgoff,
    max: Pgoff,
    fbatch: *mut folio_batch,
) {
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), index);
    if index > max {
        return;
    }
    rcu_read_lock();
    let mut folio = xas_load(&mut xas) as *mut folio;
    while !folio.is_null() {
        'entry: {
            if xas_retry(&mut xas, folio.cast()) {
                break 'entry;
            }
            if xa_is_value(folio.cast()) || xa_is_sibling(folio.cast()) {
                break;
            }
            if folio_try_get(folio) {
                if folio.cast() == xas_reload(&mut xas) {
                    if folio_batch_add(fbatch, folio) == 0
                        || !folio_test_uptodate(folio)
                        || folio_test_readahead(folio)
                    {
                        break;
                    }
                    xas_advance(&mut xas, folio_next_index(folio).wrapping_sub(1));
                    if xas.xa_index >= max {
                        break;
                    }
                    break 'entry;
                }
                folio_put(folio);
            }
            xas_reset(&mut xas);
        }
        folio = xas_next(&mut xas) as *mut folio;
    }
    rcu_read_unlock();
}
unsafe fn filemap_read_folio(file: *mut file, filler: Filler, folio: *mut folio) -> c_int {
    let workingset = folio_test_workingset(folio);
    let mut pflags = MaybeUninit::<c_ulong>::uninit();
    if workingset {
        psi_memstall_enter(pflags.as_mut_ptr());
    }
    let error = filler.unwrap_unchecked()(file, folio);
    if workingset {
        psi_memstall_leave(pflags.as_mut_ptr());
    }
    if error != 0 {
        return error;
    }
    let error = folio_wait_locked_killable(folio);
    if error != 0 {
        return error;
    }
    if folio_test_uptodate(folio) {
        return 0;
    }
    if !file.is_null() {
        shrink_readahead_size_eio(file_ra(file));
    }
    -(EIO as c_int)
}
unsafe fn filemap_range_uptodate(
    mapping: *mut address_space,
    mut pos: loff_t,
    mut count: usize,
    folio: *mut folio,
    need_uptodate: bool,
) -> bool {
    if folio_test_uptodate(folio) {
        return true;
    }
    if need_uptodate {
        return false;
    }
    let Some(partially_uptodate) = (*mapping_aops(mapping)).is_partially_uptodate else {
        return false;
    };
    if inode_blkbits(mapping_host(mapping)) as c_uint >= folio_shift(folio) {
        return false;
    }
    if folio_pos(folio) > pos {
        count = count.wrapping_sub(folio_pos(folio).wrapping_sub(pos) as usize);
        pos = 0;
    } else {
        pos = pos.wrapping_sub(folio_pos(folio));
    }
    if pos == 0 && count >= folio_size(folio) {
        return false;
    }
    partially_uptodate(folio, pos as usize, count)
}
unsafe fn filemap_update_page(
    iocb: *mut kiocb,
    mapping: *mut address_space,
    count: usize,
    folio: *mut folio,
    need_uptodate: bool,
) -> c_int {
    if (*iocb).ki_flags & RUST_FILEMAP_IOCB_NOWAIT != 0 {
        if filemap_invalidate_trylock_shared(mapping) == 0 {
            return -(EAGAIN as c_int);
        }
    } else {
        filemap_invalidate_lock_shared(mapping);
    }
    let error;
    'mapping: {
        if !folio_trylock(folio) {
            if (*iocb).ki_flags & (RUST_FILEMAP_IOCB_NOWAIT | RUST_FILEMAP_IOCB_NOIO) != 0 {
                error = -(EAGAIN as c_int);
                break 'mapping;
            }
            if (*iocb).ki_flags & RUST_FILEMAP_IOCB_WAITQ == 0 {
                filemap_invalidate_unlock_shared(mapping);
                folio_put_wait_locked(folio, RUST_FILEMAP_TASK_KILLABLE as c_int);
                return RUST_FILEMAP_AOP_TRUNCATED_PAGE as c_int;
            }
            let ret = __folio_lock_async(folio, (*iocb).ki_waitq);
            if ret != 0 {
                error = ret;
                break 'mapping;
            }
        }
        if folio_mapping_field(folio).is_null() {
            error = RUST_FILEMAP_AOP_TRUNCATED_PAGE as c_int;
        } else if filemap_range_uptodate(mapping, (*iocb).ki_pos, count, folio, need_uptodate) {
            error = 0;
        } else if (*iocb).ki_flags
            & (RUST_FILEMAP_IOCB_NOIO | RUST_FILEMAP_IOCB_NOWAIT | RUST_FILEMAP_IOCB_WAITQ)
            != 0
        {
            error = -(EAGAIN as c_int);
        } else {
            // read_folio owns unlocking, including the filesystem's error path.
            error = filemap_read_folio((*iocb).ki_filp, (*mapping_aops(mapping)).read_folio, folio);
            break 'mapping;
        }
        folio_unlock(folio);
    }
    filemap_invalidate_unlock_shared(mapping);
    if error == RUST_FILEMAP_AOP_TRUNCATED_PAGE as c_int {
        folio_put(folio);
    }
    error
}
unsafe fn filemap_create_folio(iocb: *mut kiocb, fbatch: *mut folio_batch) -> c_int {
    let mapping = file_mapping((*iocb).ki_filp);
    let min_order = mapping_min_folio_order(mapping);
    if (*iocb).ki_flags & (RUST_FILEMAP_IOCB_NOWAIT | RUST_FILEMAP_IOCB_WAITQ) != 0 {
        return -(EAGAIN as c_int);
    }
    let folio = filemap_alloc_folio_c2630(mapping_gfp_mask(mapping), min_order, null_mut());
    if folio.is_null() {
        return -(ENOMEM as c_int);
    }
    if (*iocb).ki_flags & RUST_FILEMAP_IOCB_DONTCACHE != 0 {
        __folio_set_dropbehind(folio);
    }
    filemap_invalidate_lock_shared(mapping);
    let index = (((*iocb).ki_pos >> (PAGE_SHIFT + min_order)) << min_order) as Pgoff;
    let mut error = filemap_add_folio(
        mapping,
        folio,
        index,
        mapping_gfp_constraint(mapping, RUST_FILEMAP_GFP_KERNEL),
    );
    if error == -(EEXIST as c_int) {
        error = RUST_FILEMAP_AOP_TRUNCATED_PAGE as c_int;
    }
    if error == 0 {
        error = filemap_read_folio((*iocb).ki_filp, (*mapping_aops(mapping)).read_folio, folio);
    }
    filemap_invalidate_unlock_shared(mapping);
    if error == 0 {
        folio_batch_add(fbatch, folio);
    } else {
        folio_put(folio);
    }
    error
}
unsafe fn filemap_readahead(
    iocb: *mut kiocb,
    file: *mut file,
    mapping: *mut address_space,
    folio: *mut folio,
    last_index: Pgoff,
) -> c_int {
    let mut ractl = rust_filemap_readahead_init(file, file_ra(file), mapping, folio_index(folio));
    if (*iocb).ki_flags & RUST_FILEMAP_IOCB_NOIO != 0 {
        return -(EAGAIN as c_int);
    }
    if (*iocb).ki_flags & RUST_FILEMAP_IOCB_DONTCACHE != 0 {
        readahead_set_dropbehind(&mut ractl);
    }
    page_cache_async_ra(
        &mut ractl,
        folio,
        last_index.wrapping_sub(folio_index(folio)),
    );
    0
}
unsafe fn filemap_get_pages(
    iocb: *mut kiocb,
    count: usize,
    fbatch: *mut folio_batch,
    need_uptodate: bool,
) -> c_int {
    let filp = (*iocb).ki_filp;
    let mapping = file_mapping(filp);
    let index = ((*iocb).ki_pos >> PAGE_SHIFT) as Pgoff;
    let align = mapping_min_folio_nrbytes(mapping) as u64;
    let last_index = ((((*iocb).ki_pos as u64)
        .wrapping_add(count as u64)
        .wrapping_sub(1)
        | align.wrapping_sub(1))
    .wrapping_add(1)
        >> PAGE_SHIFT) as Pgoff;
    loop {
        if fatal_signal_pending(current()) != 0 {
            return -(EINTR as c_int);
        }
        filemap_get_read_batch(mapping, index, last_index.wrapping_sub(1), fbatch);
        if folio_batch_count(fbatch) == 0 {
            let mut ractl = rust_filemap_readahead_init(filp, file_ra(filp), mapping, index);
            if (*iocb).ki_flags & RUST_FILEMAP_IOCB_NOIO != 0 {
                return -(EAGAIN as c_int);
            }
            let mut flags = 0;
            if (*iocb).ki_flags & RUST_FILEMAP_IOCB_NOWAIT != 0 {
                flags = memalloc_noio_save();
            }
            if (*iocb).ki_flags & RUST_FILEMAP_IOCB_DONTCACHE != 0 {
                readahead_set_dropbehind(&mut ractl);
            }
            page_cache_sync_ra(&mut ractl, last_index.wrapping_sub(index));
            if (*iocb).ki_flags & RUST_FILEMAP_IOCB_NOWAIT != 0 {
                memalloc_noio_restore(flags);
            }
            filemap_get_read_batch(mapping, index, last_index.wrapping_sub(1), fbatch);
        }
        if folio_batch_count(fbatch) == 0 {
            let err = filemap_create_folio(iocb, fbatch);
            if err == RUST_FILEMAP_AOP_TRUNCATED_PAGE as c_int {
                continue;
            }
            return err;
        }
        let folio = (*fbatch).folios[folio_batch_count(fbatch) as usize - 1];
        let mut err = 0;
        if folio_test_readahead(folio) {
            err = filemap_readahead(iocb, filp, mapping, folio, last_index);
        }
        if err == 0 && !folio_test_uptodate(folio) {
            err = if folio_batch_count(fbatch) > 1 {
                -(EAGAIN as c_int)
            } else {
                filemap_update_page(iocb, mapping, count, folio, need_uptodate)
            };
        }
        if err == 0 {
            trace_mm_filemap_get_pages(mapping, index, last_index.wrapping_sub(1));
            return 0;
        }
        if err < 0 {
            folio_put(folio);
        }
        (*fbatch).nr = (*fbatch).nr.wrapping_sub(1);
        if (*fbatch).nr != 0 {
            return 0;
        }
        if err != RUST_FILEMAP_AOP_TRUNCATED_PAGE as c_int {
            return err;
        }
    }
}
unsafe fn pos_same_folio(pos1: loff_t, pos2: loff_t, folio: *mut folio) -> bool {
    let shift = folio_shift(folio);
    pos1 >> shift == pos2 >> shift
}
unsafe fn filemap_end_dropbehind_read(folio: *mut folio) {
    if !folio_test_dropbehind(folio) || folio_test_writeback(folio) || folio_test_dirty(folio) {
        return;
    }
    if folio_trylock(folio) {
        filemap_end_dropbehind(folio);
        folio_unlock(folio);
    }
}
#[no_mangle]
pub unsafe extern "C" fn filemap_read(
    iocb: *mut kiocb,
    iter: *mut iov_iter,
    mut already_read: isize,
) -> isize {
    let filp = (*iocb).ki_filp;
    let ra = file_ra(filp);
    let mapping = file_mapping(filp);
    let inode = mapping_host(mapping);
    let mut error: c_int = 0;
    let mut last_pos = (*ra).prev_pos;
    if (*iocb).ki_pos < 0 {
        return -(EINVAL as isize);
    }
    if (*iocb).ki_pos >= inode_maxbytes(inode) || iov_iter_count(iter) == 0 {
        return 0;
    }
    iov_iter_truncate(
        iter,
        inode_maxbytes(inode).wrapping_sub((*iocb).ki_pos) as u64,
    );
    let mut batch = MaybeUninit::<folio_batch>::uninit();
    let batch = batch.as_mut_ptr();
    folio_batch_init(batch);
    loop {
        cond_resched();
        if (*iocb).ki_flags & RUST_FILEMAP_IOCB_WAITQ != 0 && already_read != 0 {
            (*iocb).ki_flags |= RUST_FILEMAP_IOCB_NOWAIT;
        }
        if (*iocb).ki_pos >= i_size_read(inode) {
            break;
        }
        error = filemap_get_pages(iocb, iov_iter_count(iter), batch, false);
        if error < 0 {
            break;
        }
        let isize = i_size_read(inode);
        if (*iocb).ki_pos < isize {
            let end_offset = core::cmp::min(
                isize,
                (*iocb).ki_pos.wrapping_add(iov_iter_count(iter) as loff_t),
            );
            let writably_mapped = mapping_writably_mapped(mapping) != 0;
            if !pos_same_folio((*iocb).ki_pos, last_pos.wrapping_sub(1), (*batch).folios[0]) {
                folio_mark_accessed((*batch).folios[0]);
            }
            for i in 0..folio_batch_count(batch) as usize {
                let folio = (*batch).folios[i];
                let fsize = folio_size(folio);
                let offset = (*iocb).ki_pos as usize & fsize.wrapping_sub(1);
                let bytes = core::cmp::min(
                    end_offset.wrapping_sub((*iocb).ki_pos),
                    fsize.wrapping_sub(offset) as loff_t,
                ) as usize;
                if end_offset < folio_pos(folio) {
                    break;
                }
                if i > 0 {
                    folio_mark_accessed(folio);
                }
                if writably_mapped {
                    flush_dcache_folio(folio);
                }
                let copied = copy_folio_to_iter(folio, offset, bytes, iter);
                already_read = already_read.wrapping_add(copied as isize);
                (*iocb).ki_pos = (*iocb).ki_pos.wrapping_add(copied as loff_t);
                last_pos = (*iocb).ki_pos;
                if copied < bytes {
                    error = -(EFAULT as c_int);
                    break;
                }
            }
        }
        for i in 0..folio_batch_count(batch) as usize {
            let folio = (*batch).folios[i];
            filemap_end_dropbehind_read(folio);
            folio_put(folio);
        }
        folio_batch_init(batch);
        if iov_iter_count(iter) == 0 || (*iocb).ki_pos >= isize || error != 0 {
            break;
        }
    }
    file_accessed(filp);
    (*ra).prev_pos = last_pos;
    if already_read != 0 {
        already_read
    } else {
        error as isize
    }
}
#[no_mangle]
pub unsafe extern "C" fn kiocb_write_and_wait(iocb: *mut kiocb, count: usize) -> c_int {
    let mapping = file_mapping((*iocb).ki_filp);
    let pos = (*iocb).ki_pos;
    let end = pos.wrapping_add(count as loff_t).wrapping_sub(1);
    if (*iocb).ki_flags & RUST_FILEMAP_IOCB_NOWAIT != 0 {
        return if filemap_range_needs_writeback(mapping, pos, end) {
            -(EAGAIN as c_int)
        } else {
            0
        };
    }
    filemap_write_and_wait_range(mapping, pos, end)
}
#[no_mangle]
pub unsafe extern "C" fn filemap_invalidate_pages(
    mapping: *mut address_space,
    pos: loff_t,
    end: loff_t,
    nowait: bool,
) -> c_int {
    if nowait {
        if filemap_range_has_page(mapping, pos, end) {
            return -(EAGAIN as c_int);
        }
    } else {
        let ret = filemap_write_and_wait_range(mapping, pos, end);
        if ret != 0 {
            return ret;
        }
    }
    invalidate_inode_pages2_range(
        mapping,
        (pos >> PAGE_SHIFT) as Pgoff,
        (end >> PAGE_SHIFT) as Pgoff,
    )
}
#[no_mangle]
pub unsafe extern "C" fn kiocb_invalidate_pages(iocb: *mut kiocb, count: usize) -> c_int {
    filemap_invalidate_pages(
        file_mapping((*iocb).ki_filp),
        (*iocb).ki_pos,
        (*iocb).ki_pos.wrapping_add(count as loff_t).wrapping_sub(1),
        (*iocb).ki_flags & RUST_FILEMAP_IOCB_NOWAIT != 0,
    )
}
#[no_mangle]
pub unsafe extern "C" fn generic_file_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize {
    let mut count = iov_iter_count(iter);
    let mut retval: isize = 0;
    if count == 0 {
        return 0;
    }
    if (*iocb).ki_flags & RUST_FILEMAP_IOCB_DIRECT != 0 {
        let file = (*iocb).ki_filp;
        let mapping = file_mapping(file);
        let inode = mapping_host(mapping);
        retval = kiocb_write_and_wait(iocb, count) as isize;
        if retval < 0 {
            return retval;
        }
        file_accessed(file);
        retval = (*mapping_aops(mapping)).direct_IO.unwrap_unchecked()(iocb, iter);
        if retval >= 0 {
            (*iocb).ki_pos = (*iocb).ki_pos.wrapping_add(retval as loff_t);
            count = count.wrapping_sub(retval as usize);
        }
        if retval != -(EIOCBQUEUED as isize) {
            iov_iter_revert(iter, count.wrapping_sub(iov_iter_count(iter)));
        }
        if retval < 0 || count == 0 || IS_DAX(inode) || (*iocb).ki_pos >= i_size_read(inode) {
            return retval;
        }
    }
    filemap_read(iocb, iter, retval)
}
#[no_mangle]
pub unsafe extern "C" fn splice_folio_into_pipe(
    pipe: *mut pipe_inode_info,
    folio: *mut folio,
    fpos: loff_t,
    mut size: usize,
) -> usize {
    let mut spliced: usize = 0;
    let mut offset = offset_in_folio(folio, fpos);
    let mut page = folio_page(folio, (offset / PAGE_SIZE as usize) as c_ulong);
    size = core::cmp::min(size, folio_size(folio).wrapping_sub(offset));
    offset %= PAGE_SIZE as usize;
    while spliced < size && !pipe_is_full(pipe) {
        let buf = pipe_head_buf(pipe);
        let part = core::cmp::min(
            (PAGE_SIZE as usize).wrapping_sub(offset),
            size.wrapping_sub(spliced),
        );
        *buf = zeroed();
        (*buf).ops = addr_of!(page_cache_pipe_buf_ops);
        (*buf).page = page;
        (*buf).offset = offset as c_uint;
        (*buf).len = part as c_uint;
        folio_get(folio);
        rust_filemap_pipe_increment_head(pipe);
        page = page_nth(page, 1);
        spliced = spliced.wrapping_add(part);
        offset = 0;
    }
    spliced
}
#[no_mangle]
pub unsafe extern "C" fn filemap_splice_read(
    input: *mut file,
    ppos: *mut loff_t,
    pipe: *mut pipe_inode_info,
    mut len: usize,
    _flags: c_uint,
) -> isize {
    let mut batch = MaybeUninit::<folio_batch>::uninit();
    let batch = batch.as_mut_ptr();
    let mut iocb = MaybeUninit::<kiocb>::uninit();
    let iocb = iocb.as_mut_ptr();
    let mut total_spliced: usize = 0;
    let mut error = 0;
    if *ppos >= inode_maxbytes(mapping_host(file_mapping(input))) {
        return 0;
    }
    init_sync_kiocb(iocb, input);
    (*iocb).ki_pos = *ppos;
    let used = pipe_buf_usage(pipe) as usize;
    let npages = core::cmp::max(
        (pipe_max_usage(pipe) as usize).wrapping_sub(used) as isize,
        0,
    ) as usize;
    len = core::cmp::min(len, npages.wrapping_mul(PAGE_SIZE as usize));
    folio_batch_init(batch);
    'out: loop {
        cond_resched();
        if *ppos >= i_size_read(mapping_host(file_mapping(input))) {
            break;
        }
        (*iocb).ki_pos = *ppos;
        error = filemap_get_pages(iocb, len, batch, true);
        if error < 0 {
            break;
        }
        let isize = i_size_read(mapping_host(file_mapping(input)));
        if *ppos >= isize {
            break;
        }
        let end_offset = core::cmp::min(isize, (*ppos).wrapping_add(len as loff_t));
        let writably_mapped = mapping_writably_mapped(file_mapping(input)) != 0;
        for i in 0..folio_batch_count(batch) as usize {
            let folio = (*batch).folios[i];
            if folio_pos(folio) >= end_offset {
                break 'out;
            }
            folio_mark_accessed(folio);
            if writably_mapped {
                flush_dcache_folio(folio);
            }
            let n = core::cmp::min(len as loff_t, isize.wrapping_sub(*ppos)) as usize;
            let n = splice_folio_into_pipe(pipe, folio, *ppos, n);
            if n == 0 {
                break 'out;
            }
            len = len.wrapping_sub(n);
            total_spliced = total_spliced.wrapping_add(n);
            *ppos = (*ppos).wrapping_add(n as loff_t);
            (*file_ra(input)).prev_pos = *ppos;
            if pipe_is_full(pipe) {
                break 'out;
            }
        }
        folio_batch_release(batch);
        if len == 0 {
            break;
        }
    }
    folio_batch_release(batch);
    file_accessed(input);
    if total_spliced != 0 {
        total_spliced as isize
    } else {
        error as isize
    }
}
unsafe fn folio_seek_hole_data(
    xas: *mut xa_state,
    mapping: *mut address_space,
    folio: *mut folio,
    mut start: loff_t,
    end: loff_t,
    seek_data: bool,
) -> loff_t {
    let ops = mapping_aops(mapping);
    let bsz = i_blocksize(mapping_host(mapping)) as usize;
    if xa_is_value(folio.cast()) || folio_test_uptodate(folio) {
        return if seek_data { start } else { end };
    }
    let Some(partially_uptodate) = (*ops).is_partially_uptodate else {
        return if seek_data { end } else { start };
    };
    xas_pause(xas);
    rcu_read_unlock();
    folio_lock(folio);
    if folio_mapping_field(folio) == mapping {
        let mut offset = offset_in_folio(folio, start) & !bsz.wrapping_sub(1);
        loop {
            if partially_uptodate(folio, offset, bsz) == seek_data {
                break;
            }
            start =
                ((start as u64).wrapping_add(bsz as u64) & !(bsz as u64).wrapping_sub(1)) as loff_t;
            offset = offset.wrapping_add(bsz);
            if offset >= folio_size(folio) {
                break;
            }
        }
    }
    folio_unlock(folio);
    rcu_read_lock();
    start
}
unsafe fn seek_folio_size(xas: *mut xa_state, folio: *mut folio) -> usize {
    if xa_is_value(folio.cast()) {
        (PAGE_SIZE as usize).wrapping_shl(xas_get_order(xas) as c_uint)
    } else {
        folio_size(folio)
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapping_seek_hole_data(
    mapping: *mut address_space,
    mut start: loff_t,
    end: loff_t,
    whence: c_int,
) -> loff_t {
    let mut xas = rust_filemap_xa_state(mapping_i_pages(mapping), (start >> PAGE_SHIFT) as Pgoff);
    let max = (end.wrapping_sub(1) >> PAGE_SHIFT) as Pgoff;
    let seek_data = whence == RUST_FILEMAP_SEEK_DATA as c_int;
    if end <= start {
        return -(ENXIO as loff_t);
    }
    rcu_read_lock();
    let mut folio;
    'unlock: {
        loop {
            folio = find_get_entry(&mut xas, max, RUST_FILEMAP_XA_PRESENT);
            if folio.is_null() {
                break;
            }
            let mut pos = (xas.xa_index as u64).wrapping_shl(PAGE_SHIFT) as loff_t;
            if start < pos {
                if !seek_data {
                    break 'unlock;
                }
                start = pos;
            }
            let seek_size = seek_folio_size(&mut xas, folio);
            let next = (((pos as u64).wrapping_add(1).wrapping_sub(1))
                | (seek_size as u64).wrapping_sub(1))
            .wrapping_add(1);
            pos = if next > end as u64 {
                end
            } else {
                next as loff_t
            };
            start = folio_seek_hole_data(&mut xas, mapping, folio, start, pos, seek_data);
            if start < pos {
                break 'unlock;
            }
            if start >= end {
                break;
            }
            if seek_size > PAGE_SIZE as usize {
                xas_set(&mut xas, (pos >> PAGE_SHIFT) as Pgoff);
            }
            if !xa_is_value(folio.cast()) {
                folio_put(folio);
            }
        }
        if seek_data {
            start = -(ENXIO as loff_t);
        }
    }
    rcu_read_unlock();
    if !folio.is_null() && !xa_is_value(folio.cast()) {
        folio_put(folio);
    }
    core::cmp::min(start, end)
}
