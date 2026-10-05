// SPDX-License-Identifier: GPL-2.0-only
// Original mm/filemap.c:4108-4603.
unsafe fn do_read_cache_folio(
    mapping: *mut address_space,
    mut index: Pgoff,
    mut filler: Filler,
    file: *mut file,
    gfp: gfp_t,
) -> *mut folio {
    if filler.is_none() {
        filler = (*mapping_aops(mapping)).read_folio;
    }
    loop {
        let mut folio = filemap_get_folio(mapping, index);
        let mut fill = true;
        if IS_ERR(folio.cast()) {
            folio = filemap_alloc_folio_c4119(gfp, mapping_min_folio_order(mapping), null_mut());
            if folio.is_null() {
                return ERR_PTR(-(ENOMEM as c_long)).cast();
            }
            index = mapping_align_index(mapping, index);
            let err = filemap_add_folio(mapping, folio, index, gfp);
            if err != 0 {
                folio_put(folio);
                if err == -(EEXIST as c_int) {
                    continue;
                }
                return ERR_PTR(err as c_long).cast();
            }
        } else if folio_test_uptodate(folio) {
            fill = false;
        } else {
            if !folio_trylock(folio) {
                folio_put_wait_locked(folio, RUST_FILEMAP_TASK_UNINTERRUPTIBLE as c_int);
                continue;
            }
            if folio_mapping_field(folio).is_null() {
                folio_unlock(folio);
                folio_put(folio);
                continue;
            }
            if folio_test_uptodate(folio) {
                folio_unlock(folio);
                fill = false;
            }
        }
        if fill {
            let err = filemap_read_folio(file, filler, folio);
            if err != 0 {
                folio_put(folio);
                if err == RUST_FILEMAP_AOP_TRUNCATED_PAGE as c_int {
                    continue;
                }
                return ERR_PTR(err as c_long).cast();
            }
        }
        folio_mark_accessed(folio);
        return folio;
    }
}
#[no_mangle]
pub unsafe extern "C" fn read_cache_folio(
    mapping: *mut address_space,
    index: Pgoff,
    filler: Filler,
    file: *mut file,
) -> *mut folio {
    do_read_cache_folio(mapping, index, filler, file, mapping_gfp_mask(mapping))
}
#[no_mangle]
pub unsafe extern "C" fn mapping_read_folio_gfp(
    mapping: *mut address_space,
    index: Pgoff,
    gfp: gfp_t,
) -> *mut folio {
    do_read_cache_folio(mapping, index, None, null_mut(), gfp)
}
unsafe fn do_read_cache_page(
    mapping: *mut address_space,
    index: Pgoff,
    filler: Filler,
    file: *mut file,
    gfp: gfp_t,
) -> *mut page {
    let folio = do_read_cache_folio(mapping, index, filler, file, gfp);
    if IS_ERR(folio.cast()) {
        return folio_page0(folio);
    }
    folio_file_page(folio, index)
}
#[no_mangle]
pub unsafe extern "C" fn read_cache_page(
    mapping: *mut address_space,
    index: Pgoff,
    filler: Filler,
    file: *mut file,
) -> *mut page {
    do_read_cache_page(mapping, index, filler, file, mapping_gfp_mask(mapping))
}
#[no_mangle]
pub unsafe extern "C" fn read_cache_page_gfp(
    mapping: *mut address_space,
    index: Pgoff,
    gfp: gfp_t,
) -> *mut page {
    do_read_cache_page(mapping, index, None, null_mut(), gfp)
}
unsafe fn dio_warn_stale_pagecache(filp: *mut file) {
    let mut pathname = MaybeUninit::<[kernel::ffi::c_char; 128]>::uninit();
    errseq_set(mapping_wb_err(file_mapping(filp)), -(EIO as c_int));
    if rust_filemap_dio_ratelimit() {
        let mut path = file_path(filp, pathname.as_mut_ptr().cast(), 128);
        if IS_ERR(path.cast()) {
            path = kernel::str::as_char_ptr_in_const_context(c"(unknown)").cast_mut();
        }
        rust_filemap_dio_print(path);
    }
}
// Preserve C's loff_t + size_t usual arithmetic conversions before shifting.
// LP64 promotes to unsigned long long; ILP32 retains signed long long.
fn post_direct_write_last_index(pos: loff_t, count: usize) -> Pgoff {
    #[cfg(target_pointer_width = "64")]
    {
        ((pos as u64).wrapping_add(count as u64).wrapping_sub(1) >> PAGE_SHIFT) as Pgoff
    }
    #[cfg(target_pointer_width = "32")]
    {
        (pos.wrapping_add(count as loff_t).wrapping_sub(1) >> PAGE_SHIFT) as Pgoff
    }
}
#[no_mangle]
pub unsafe extern "C" fn kiocb_invalidate_post_direct_write(iocb: *mut kiocb, count: usize) {
    let mapping = file_mapping((*iocb).ki_filp);
    if (*mapping_nrpages(mapping)) != 0
        && invalidate_inode_pages2_range(
            mapping,
            ((*iocb).ki_pos >> PAGE_SHIFT) as Pgoff,
            post_direct_write_last_index((*iocb).ki_pos, count),
        ) != 0
    {
        dio_warn_stale_pagecache((*iocb).ki_filp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn generic_file_direct_write(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    let mapping = file_mapping((*iocb).ki_filp);
    let mut write_len = iov_iter_count(from);
    let written = kiocb_invalidate_pages(iocb, write_len) as isize;
    if written != 0 {
        return if written == -(EBUSY as isize) {
            0
        } else {
            written
        };
    }
    let written = (*mapping_aops(mapping)).direct_IO.unwrap_unchecked()(iocb, from);
    if written > 0 {
        let inode = mapping_host(mapping);
        let mut pos = (*iocb).ki_pos;
        kiocb_invalidate_post_direct_write(iocb, written as usize);
        pos = pos.wrapping_add(written as loff_t);
        write_len = write_len.wrapping_sub(written as usize);
        if pos > i_size_read(inode) && !inode_is_blk(inode) {
            i_size_write(inode, pos);
            mark_inode_dirty(inode);
        }
        (*iocb).ki_pos = pos;
    }
    if written != -(EIOCBQUEUED as isize) {
        iov_iter_revert(from, write_len.wrapping_sub(iov_iter_count(from)));
    }
    written
}
#[no_mangle]
pub unsafe extern "C" fn generic_perform_write(iocb: *mut kiocb, i: *mut iov_iter) -> isize {
    let file = (*iocb).ki_filp;
    let mut pos = (*iocb).ki_pos;
    let mapping = file_mapping(file);
    let a_ops = mapping_aops(mapping);
    let mut chunk = mapping_max_folio_size(mapping);
    let mut status: c_long;
    let mut written: isize = 0;
    'write: loop {
        let mut fsdata = null_mut();
        let mut bytes = iov_iter_count(i);
        loop {
            let mut offset = pos as usize & chunk.wrapping_sub(1);
            bytes = core::cmp::min(chunk.wrapping_sub(offset), bytes);
            balance_dirty_pages_ratelimited(mapping);
            if fatal_signal_pending(current()) != 0 {
                status = -(EINTR as c_long);
                break 'write;
            }
            let mut folio = MaybeUninit::<*mut folio>::uninit();
            status = (*a_ops).write_begin.unwrap_unchecked()(
                iocb,
                mapping,
                pos,
                bytes as c_uint,
                folio.as_mut_ptr(),
                &mut fsdata,
            ) as c_long;
            if status < 0 {
                break 'write;
            }
            let folio = folio.assume_init();
            offset = offset_in_folio(folio, pos);
            if bytes > folio_size(folio).wrapping_sub(offset) {
                bytes = folio_size(folio).wrapping_sub(offset);
            }
            if mapping_writably_mapped(mapping) != 0 {
                flush_dcache_folio(folio);
            }
            let copied = copy_folio_from_iter_atomic(folio, offset, bytes, i);
            flush_dcache_folio(folio);
            status = (*a_ops).write_end.unwrap_unchecked()(
                iocb,
                mapping,
                pos,
                bytes as c_uint,
                copied as c_uint,
                folio,
                fsdata,
            ) as c_long;
            if status as usize != copied {
                iov_iter_revert(i, copied.wrapping_sub(core::cmp::max(status, 0) as usize));
                if status < 0 {
                    break 'write;
                }
            }
            cond_resched();
            if status == 0 {
                if chunk > PAGE_SIZE as usize {
                    chunk /= 2;
                }
                if copied != 0 {
                    bytes = copied;
                    continue;
                }
                if fault_in_iov_iter_readable(i, bytes) == bytes {
                    status = -(EFAULT as c_long);
                    break 'write;
                }
            } else {
                pos = pos.wrapping_add(status as loff_t);
                written = written.wrapping_add(status as isize);
            }
            break;
        }
        if iov_iter_count(i) == 0 {
            break;
        }
    }
    if written == 0 {
        return status as isize;
    }
    (*iocb).ki_pos = (*iocb).ki_pos.wrapping_add(written as loff_t);
    written
}
#[no_mangle]
pub unsafe extern "C" fn __generic_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    let file = (*iocb).ki_filp;
    let mapping = file_mapping(file);
    let inode = mapping_host(mapping);
    let ret = file_remove_privs(file);
    if ret != 0 {
        return ret as isize;
    }
    let ret = file_update_time(file);
    if ret != 0 {
        return ret as isize;
    }
    if (*iocb).ki_flags & RUST_FILEMAP_IOCB_DIRECT != 0 {
        let ret = generic_file_direct_write(iocb, from);
        if ret < 0 || iov_iter_count(from) == 0 || IS_DAX(inode) {
            return ret;
        }
        let buffered = generic_perform_write(iocb, from);
        return direct_write_fallback(iocb, from, ret, buffered);
    }
    generic_perform_write(iocb, from)
}
#[no_mangle]
pub unsafe extern "C" fn generic_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    let file = (*iocb).ki_filp;
    let inode = mapping_host(file_mapping(file));
    inode_lock(inode);
    let mut ret = generic_write_checks(iocb, from);
    if ret > 0 {
        ret = __generic_file_write_iter(iocb, from);
    }
    inode_unlock(inode);
    if ret > 0 {
        ret = generic_write_sync(iocb, ret);
    }
    ret
}
#[no_mangle]
pub unsafe extern "C" fn filemap_release_folio(folio: *mut folio, gfp: gfp_t) -> bool {
    let mapping = folio_mapping_field(folio);
    rust_filemap_bug(!folio_test_locked(folio));
    if !folio_needs_release(folio) {
        return true;
    }
    if folio_test_writeback(folio) {
        return false;
    }
    if !mapping.is_null() {
        if let Some(release) = (*mapping_aops(mapping)).release_folio {
            return release(folio, gfp);
        }
    }
    try_to_free_buffers(folio)
}
#[no_mangle]
pub unsafe extern "C" fn filemap_invalidate_inode(
    inode: *mut inode,
    flush: bool,
    start: loff_t,
    end: loff_t,
) -> c_int {
    let mapping = inode_mapping(inode);
    let first = (start >> PAGE_SHIFT) as Pgoff;
    let last = (end >> PAGE_SHIFT) as Pgoff;
    let nr = if end == i64::MAX {
        c_ulong::MAX
    } else {
        last.wrapping_sub(first).wrapping_add(1)
    };
    if !mapping.is_null() && (*mapping_nrpages(mapping)) != 0 && end >= start {
        filemap_invalidate_lock(mapping);
        if (*mapping_nrpages(mapping)) != 0 {
            unmap_mapping_pages(mapping, first, nr, false);
            if flush {
                filemap_fdatawrite_range(mapping, start, end);
            }
            // C's signed/unsigned usual conversion follows PAGE_SIZE's native
            // unsigned-long width; the ranges reaching here are nonnegative.
            invalidate_inode_pages2_range(
                mapping,
                file_bytes_div_pages(start),
                file_bytes_div_pages(end),
            );
        }
        filemap_invalidate_unlock(mapping);
    }
    filemap_check_errors(mapping)
}
