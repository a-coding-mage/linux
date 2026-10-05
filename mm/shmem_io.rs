// SPDX-License-Identifier: GPL-2.0-only
#[export_name = "rust_shmem_owner_shmem_write_begin"]
unsafe extern "C" fn shmem_write_begin(
    _iocb: *const kiocb,
    mapping: *mut address_space,
    pos: loff_t,
    len: c_uint,
    foliop: *mut *mut folio,
    _fsdata: *mut *mut c_void,
) -> c_int {
    let inode = (*mapping).host;
    let info = SHMEM_I(inode);
    let end = pos.wrapping_add(len as loff_t);
    if (*info).seals & (RUST_SHMEM_F_SEAL_WRITE | RUST_SHMEM_F_SEAL_FUTURE_WRITE) as c_uint != 0
        || ((*info).seals & RUST_SHMEM_F_SEAL_GROW as c_uint != 0 && end > (*inode).i_size)
        || ((*info).flags & RUST_SHMEM_SHMEM_F_MAPPING_FROZEN as c_ulong != 0
            && end > (*inode).i_size)
    {
        return -(RUST_SHMEM_EPERM as c_int);
    }
    let mut f = null_mut();
    let ret = shmem_get_folio(
        inode,
        (pos >> RUST_SHMEM_PAGE_SHIFT) as Pgoff,
        end,
        &mut f,
        SGP_WRITE,
    );
    if ret != 0 {
        return ret;
    }
    if folio_contain_hwpoisoned_page(f) {
        folio_unlock(f);
        folio_put(f);
        return -(RUST_SHMEM_EIO as c_int);
    }
    *foliop = f;
    0
}
#[export_name = "rust_shmem_owner_shmem_write_end"]
unsafe extern "C" fn shmem_write_end(
    _iocb: *const kiocb,
    mapping: *mut address_space,
    pos: loff_t,
    _len: c_uint,
    copied: c_uint,
    f: *mut folio,
    _fsdata: *mut c_void,
) -> c_int {
    let inode = (*mapping).host;
    let end = pos.wrapping_add(copied as loff_t);
    if end > (*inode).i_size {
        i_size_write(inode, end);
    }
    if !folio_test_uptodate(f) {
        if (copied as usize) < folio_size(f) {
            let from = rust_shmem_offset_in_folio(f, pos as c_ulong);
            folio_zero_segments(
                f,
                0,
                from,
                from.wrapping_add(copied as usize),
                folio_size(f),
            );
        }
        folio_mark_uptodate(f);
    }
    folio_mark_dirty(f);
    folio_unlock(f);
    folio_put(f);
    copied as c_int
}
#[export_name = "rust_shmem_owner_shmem_file_read_iter"]
unsafe extern "C" fn shmem_file_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    let file = (*iocb).ki_filp;
    let inode = file_inode(file);
    let mapping = (*inode).i_mapping;
    let mut retval: ssize_t = 0;
    let mut error = 0;
    loop {
        if (*iocb).ki_pos >= i_size_read(inode) {
            break;
        }
        let index = ((*iocb).ki_pos >> RUST_SHMEM_PAGE_SHIFT) as Pgoff;
        let mut f = null_mut();
        let mut page = null_mut();
        let mut fallback = false;
        error = shmem_get_folio(inode, index, 0, &mut f, SGP_READ);
        if error != 0 {
            if error == -(RUST_SHMEM_EINVAL as c_int) {
                error = 0;
            }
            break;
        }
        if !f.is_null() {
            folio_unlock(f);
            page = folio_file_page(f, index);
            if rust_shmem_PageHWPoison(page) {
                folio_put(f);
                error = -(RUST_SHMEM_EIO as c_int);
                break;
            }
            fallback = folio_test_large(f) && folio_test_has_hwpoisoned(f);
        }
        let size = i_size_read(inode);
        if (*iocb).ki_pos >= size {
            if !f.is_null() {
                folio_put(f);
            }
            break;
        }
        let end = core::cmp::min(size, (*iocb).ki_pos.wrapping_add((*to).count as loff_t));
        let fsize = if !f.is_null() && !fallback {
            folio_size(f)
        } else {
            RUST_SHMEM_PAGE_SIZE as usize
        };
        let offset = (*iocb).ki_pos as usize & (fsize - 1);
        let nr = core::cmp::min(end.wrapping_sub((*iocb).ki_pos) as usize, fsize - offset);
        let ret;
        if !f.is_null() {
            if rust_shmem_mapping_writably_mapped(mapping) {
                if !fallback {
                    flush_dcache_folio(f);
                } else {
                    flush_dcache_page(page);
                }
            }
            if offset == 0 {
                folio_mark_accessed(f);
            }
            ret = if !fallback {
                copy_folio_to_iter(f, offset, nr, to)
            } else {
                copy_page_to_iter(page, offset, nr, to)
            };
            folio_put(f);
        } else if user_backed_iter(to) {
            ret = copy_page_to_iter(rust_shmem_zero_page(), offset, nr, to);
        } else {
            ret = iov_iter_zero(nr, to);
        }
        retval = retval.wrapping_add(ret as ssize_t);
        (*iocb).ki_pos = (*iocb).ki_pos.wrapping_add(ret as loff_t);
        if iov_iter_count(to) == 0 {
            break;
        }
        if ret < nr {
            error = -(RUST_SHMEM_EFAULT as c_int);
            break;
        }
        rust_shmem_cond_resched();
    }
    file_accessed(file);
    if retval != 0 {
        retval
    } else {
        error as ssize_t
    }
}
#[export_name = "rust_shmem_owner_shmem_file_write_iter"]
unsafe extern "C" fn shmem_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t {
    let file = (*iocb).ki_filp;
    let inode = (*(*file).f_mapping).host;
    inode_lock(inode);
    let mut ret = generic_write_checks(iocb, from);
    if ret > 0 {
        ret = file_remove_privs(file) as ssize_t;
        if ret == 0 {
            ret = file_update_time(file) as ssize_t;
            if ret == 0 {
                ret = generic_perform_write(iocb, from);
            }
        }
    }
    inode_unlock(inode);
    ret
}
#[export_name = "rust_shmem_owner_zero_pipe_buf_get"]
unsafe extern "C" fn zero_pipe_buf_get(
    _pipe: *mut pipe_inode_info,
    _buf: *mut pipe_buffer,
) -> bool {
    true
}
#[export_name = "rust_shmem_owner_zero_pipe_buf_release"]
unsafe extern "C" fn zero_pipe_buf_release(_pipe: *mut pipe_inode_info, _buf: *mut pipe_buffer) {}
#[export_name = "rust_shmem_owner_zero_pipe_buf_try_steal"]
unsafe extern "C" fn zero_pipe_buf_try_steal(
    _pipe: *mut pipe_inode_info,
    _buf: *mut pipe_buffer,
) -> bool {
    false
}
unsafe fn splice_zeropage_into_pipe(pipe: *mut pipe_inode_info, pos: loff_t, size: usize) -> usize {
    let offset = pos as usize & !(RUST_SHMEM_PAGE_MASK as usize);
    let size = core::cmp::min(size, RUST_SHMEM_PAGE_SIZE as usize - offset);
    if !pipe_is_full(pipe) {
        let buf = pipe_head_buf(pipe);
        core::ptr::write(buf, zeroed());
        (*buf).ops = addr_of!(rust_shmem_data_zero_pipe_buf_ops);
        (*buf).page = rust_shmem_zero_page();
        (*buf).offset = offset as _;
        (*buf).len = size as _;
        rust_shmem_pipe_advance_head(pipe);
    }
    size
}
#[export_name = "rust_shmem_owner_shmem_file_splice_read"]
unsafe extern "C" fn shmem_file_splice_read(
    input: *mut file,
    pos: *mut loff_t,
    pipe: *mut pipe_inode_info,
    mut len: usize,
    _flags: c_uint,
) -> ssize_t {
    let inode = file_inode(input);
    let mapping = (*inode).i_mapping;
    let mut f = null_mut();
    let mut total: usize = 0;
    let mut error = 0;
    let pages = core::cmp::max(
        ((*pipe).max_usage as usize).wrapping_sub(pipe_buf_usage(pipe) as usize) as ssize_t,
        0,
    ) as usize;
    len = core::cmp::min(len, pages.wrapping_mul(RUST_SHMEM_PAGE_SIZE as usize));
    loop {
        if *pos >= i_size_read(inode) {
            break;
        }
        let index = (*pos >> RUST_SHMEM_PAGE_SHIFT) as Pgoff;
        let mut fallback = false;
        let mut page = null_mut();
        error = shmem_get_folio(inode, index, 0, &mut f, SGP_READ);
        if error != 0 {
            if error == -(RUST_SHMEM_EINVAL as c_int) {
                error = 0;
            }
            break;
        }
        if !f.is_null() {
            folio_unlock(f);
            page = folio_file_page(f, index);
            if rust_shmem_PageHWPoison(page) {
                error = -(RUST_SHMEM_EIO as c_int);
                break;
            }
            fallback = folio_test_large(f) && folio_test_has_hwpoisoned(f);
        }
        let isize = i_size_read(inode);
        if *pos >= isize {
            break;
        }
        let mut size = len;
        if fallback {
            size = core::cmp::min(
                size,
                RUST_SHMEM_PAGE_SIZE as usize - (*pos as usize & !(RUST_SHMEM_PAGE_MASK as usize)),
            );
        }
        let part = core::cmp::min(isize.wrapping_sub(*pos), size as loff_t) as usize;
        let n;
        if !f.is_null() {
            if rust_shmem_mapping_writably_mapped(mapping) {
                if !fallback {
                    flush_dcache_folio(f);
                } else {
                    flush_dcache_page(page);
                }
            }
            folio_mark_accessed(f);
            n = splice_folio_into_pipe(pipe, f, *pos, part);
            folio_put(f);
            f = null_mut();
        } else {
            n = splice_zeropage_into_pipe(pipe, *pos, part);
        }
        if n == 0 {
            break;
        }
        len = len.wrapping_sub(n);
        total = total.wrapping_add(n);
        *pos = (*pos).wrapping_add(n as loff_t);
        (*rust_shmem_file_prev_pos(input)) = *pos;
        if pipe_is_full(pipe) {
            break;
        }
        rust_shmem_cond_resched();
        if len == 0 {
            break;
        }
    }
    if !f.is_null() {
        folio_put(f);
    }
    file_accessed(input);
    if total != 0 {
        total as ssize_t
    } else {
        error as ssize_t
    }
}
#[export_name = "rust_shmem_owner_shmem_file_llseek"]
unsafe extern "C" fn shmem_file_llseek(
    file: *mut file,
    mut offset: loff_t,
    whence: c_int,
) -> loff_t {
    let mapping = (*file).f_mapping;
    let inode = (*mapping).host;
    if whence != RUST_SHMEM_SEEK_DATA as c_int && whence != RUST_SHMEM_SEEK_HOLE as c_int {
        return generic_file_llseek_size(
            file,
            offset,
            whence,
            RUST_SHMEM_MAX_LFS_FILESIZE as loff_t,
            i_size_read(inode),
        );
    }
    if offset < 0 {
        return -(RUST_SHMEM_ENXIO as loff_t);
    }
    inode_lock(inode);
    offset = mapping_seek_hole_data(mapping, offset, (*inode).i_size, whence);
    if offset >= 0 {
        offset = vfs_setpos(file, offset, RUST_SHMEM_MAX_LFS_FILESIZE as loff_t);
    }
    inode_unlock(inode);
    offset
}
#[export_name = "rust_shmem_owner_shmem_fallocate"]
unsafe extern "C" fn shmem_fallocate(
    file: *mut file,
    mode: c_int,
    offset: loff_t,
    len: loff_t,
) -> c_long {
    let inode = file_inode(file);
    let sb = SHMEM_SB((*inode).i_sb);
    let info = SHMEM_I(inode);
    if mode & !((RUST_SHMEM_FALLOC_FL_KEEP_SIZE | RUST_SHMEM_FALLOC_FL_PUNCH_HOLE) as c_int) != 0 {
        return -(RUST_SHMEM_EOPNOTSUPP as c_long);
    }
    inode_lock(inode);
    let mut error = 0;
    'operation: {
        if (*info).flags & RUST_SHMEM_SHMEM_F_MAPPING_FROZEN as c_ulong != 0 {
            error = -(RUST_SHMEM_EPERM as c_int);
            break 'operation;
        }
        if mode & RUST_SHMEM_FALLOC_FL_PUNCH_HOLE as c_int != 0 {
            if (*info).seals & (RUST_SHMEM_F_SEAL_WRITE | RUST_SHMEM_F_SEAL_FUTURE_WRITE) as c_uint
                != 0
            {
                error = -(RUST_SHMEM_EPERM as c_int);
                break 'operation;
            }
            let start = round_up_u64(offset as u64, RUST_SHMEM_PAGE_SIZE as u64) as loff_t;
            let end = round_down_u64(offset.wrapping_add(len) as u64, RUST_SHMEM_PAGE_SIZE as u64)
                .wrapping_sub(1) as loff_t;
            let mut queue = MaybeUninit::<wait_queue_head_t>::uninit();
            rust_shmem_init_falloc_waitq(queue.as_mut_ptr());
            let mut fa: shmem_falloc = zeroed();
            fa.waitq = queue.as_mut_ptr();
            fa.start = (start as u64 >> RUST_SHMEM_PAGE_SHIFT) as Pgoff;
            fa.next = (end.wrapping_add(1) >> RUST_SHMEM_PAGE_SHIFT) as Pgoff;
            spin_lock(addr_of_mut!((*inode).i_lock));
            rust_shmem_write_private(inode, addr_of_mut!(fa).cast());
            spin_unlock(addr_of_mut!((*inode).i_lock));
            if end as u64 > start as u64 {
                unmap_mapping_range(
                    (*file).f_mapping,
                    start,
                    end.wrapping_add(1).wrapping_sub(start),
                    0,
                );
            }
            shmem_truncate_range(
                inode,
                offset,
                offset.wrapping_add(len).wrapping_sub(1) as uoff_t,
            );
            spin_lock(addr_of_mut!((*inode).i_lock));
            rust_shmem_write_private(inode, null_mut());
            rust_shmem_wake_all(queue.as_mut_ptr());
            rust_shmem_warn_waitq(!rust_shmem_list_empty(addr_of!((*queue.as_ptr()).head)));
            spin_unlock(addr_of_mut!((*inode).i_lock));
            break 'operation;
        }
        let requested_end = offset.wrapping_add(len);
        error = inode_newsize_ok(inode, requested_end);
        if error != 0 {
            break 'operation;
        }
        if (*info).seals & RUST_SHMEM_F_SEAL_GROW as c_uint != 0 && requested_end > (*inode).i_size
        {
            error = -(RUST_SHMEM_EPERM as c_int);
            break 'operation;
        }
        let aligned_end = match requested_end.checked_add(RUST_SHMEM_PAGE_SIZE as loff_t - 1) {
            Some(v) => v,
            None => {
                error = -(RUST_SHMEM_EFBIG as c_int);
                break 'operation;
            }
        };
        let start = (offset >> RUST_SHMEM_PAGE_SHIFT) as Pgoff;
        let end = (aligned_end >> RUST_SHMEM_PAGE_SHIFT) as Pgoff;
        if (*sb).max_blocks != 0 && end.wrapping_sub(start) > (*sb).max_blocks {
            error = -(RUST_SHMEM_ENOSPC as c_int);
            break 'operation;
        }
        let mut fa: shmem_falloc = zeroed();
        fa.start = start;
        fa.next = start;
        spin_lock(addr_of_mut!((*inode).i_lock));
        rust_shmem_write_private(inode, addr_of_mut!(fa).cast());
        spin_unlock(addr_of_mut!((*inode).i_lock));
        let undo = (*info).fallocend;
        if (*info).fallocend < end {
            (*info).fallocend = end;
        }
        let mut index = start;
        while index < end {
            let mut f = null_mut();
            error = if rust_shmem_fatal_signal_pending() {
                -(RUST_SHMEM_EINTR as c_int)
            } else if fa.nr_unswapped > fa.nr_falloced {
                -(RUST_SHMEM_ENOMEM as c_int)
            } else {
                shmem_get_folio(inode, index, requested_end, &mut f, SGP_FALLOC)
            };
            if error != 0 {
                (*info).fallocend = undo;
                if index > start {
                    shmem_undo_range(
                        inode,
                        (start as loff_t) << RUST_SHMEM_PAGE_SHIFT,
                        ((index as uoff_t) << RUST_SHMEM_PAGE_SHIFT).wrapping_sub(1),
                        true,
                    );
                }
                break;
            }
            index = folio_next_index(f);
            if index == 0 {
                index = index.wrapping_sub(1);
            }
            if !folio_test_uptodate(f) {
                fa.nr_falloced = fa.nr_falloced.wrapping_add(index.wrapping_sub(fa.next));
            }
            fa.next = index;
            folio_mark_dirty(f);
            folio_unlock(f);
            folio_put(f);
            rust_shmem_cond_resched();
        }
        if error == 0
            && mode & RUST_SHMEM_FALLOC_FL_KEEP_SIZE as c_int == 0
            && requested_end > (*inode).i_size
        {
            i_size_write(inode, requested_end);
        }
        spin_lock(addr_of_mut!((*inode).i_lock));
        rust_shmem_write_private(inode, null_mut());
        spin_unlock(addr_of_mut!((*inode).i_lock));
    }
    if error == 0 {
        file_modified(file);
    }
    inode_unlock(inode);
    error as c_long
}
