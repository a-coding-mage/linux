// SPDX-License-Identifier: GPL-2.0-only
// Original mm/filemap.c:346-796.
#[no_mangle]
pub unsafe extern "C" fn filemap_check_errors(mapping: *mut address_space) -> c_int {
    let flags = mapping_flags(mapping);
    let mut ret = 0;
    if test_bit(AS_ENOSPC as c_ulong, flags) && test_and_clear_bit(AS_ENOSPC as c_ulong, flags) {
        ret = -(ENOSPC as c_int);
    }
    if test_bit(AS_EIO as c_ulong, flags) && test_and_clear_bit(AS_EIO as c_ulong, flags) {
        ret = -(EIO as c_int);
    }
    ret
}
unsafe fn filemap_check_and_keep_errors(mapping: *mut address_space) -> c_int {
    if test_bit(AS_EIO as c_ulong, mapping_flags(mapping)) {
        return -(EIO as c_int);
    }
    if test_bit(AS_ENOSPC as c_ulong, mapping_flags(mapping)) {
        return -(ENOSPC as c_int);
    }
    0
}
unsafe fn filemap_writeback(
    mapping: *mut address_space,
    start: loff_t,
    end: loff_t,
    sync_mode: writeback_sync_modes,
    nr_to_write: *mut c_long,
) -> c_int {
    // C's designated initializer zeroes every field not listed here.
    let mut wbc: writeback_control = zeroed();
    wbc.sync_mode = sync_mode;
    wbc.nr_to_write = if nr_to_write.is_null() {
        c_long::MAX
    } else {
        *nr_to_write
    };
    wbc.range_start = start;
    wbc.range_end = end;
    if !mapping_can_writeback(mapping) || !mapping_tagged(mapping, RUST_FILEMAP_PAGECACHE_TAG_DIRTY)
    {
        return 0;
    }
    wbc_attach_fdatawrite_inode(&mut wbc, mapping_host(mapping));
    let ret = do_writepages(mapping, &mut wbc);
    wbc_detach_inode(&mut wbc);
    if ret == 0 && !nr_to_write.is_null() {
        *nr_to_write = wbc.nr_to_write;
    }
    ret
}
#[no_mangle]
pub unsafe extern "C" fn filemap_fdatawrite_range(
    mapping: *mut address_space,
    start: loff_t,
    end: loff_t,
) -> c_int {
    filemap_writeback(mapping, start, end, WB_SYNC_ALL, null_mut())
}
#[no_mangle]
pub unsafe extern "C" fn filemap_fdatawrite(mapping: *mut address_space) -> c_int {
    filemap_fdatawrite_range(mapping, 0, i64::MAX)
}
#[no_mangle]
pub unsafe extern "C" fn filemap_flush_range(
    mapping: *mut address_space,
    start: loff_t,
    end: loff_t,
) -> c_int {
    filemap_writeback(mapping, start, end, WB_SYNC_NONE, null_mut())
}
#[no_mangle]
pub unsafe extern "C" fn filemap_flush(mapping: *mut address_space) -> c_int {
    filemap_flush_range(mapping, 0, i64::MAX)
}
#[no_mangle]
pub unsafe extern "C" fn filemap_flush_nr(
    mapping: *mut address_space,
    nr_to_write: *mut c_long,
) -> c_int {
    filemap_writeback(mapping, 0, i64::MAX, WB_SYNC_NONE, nr_to_write)
}
#[no_mangle]
pub unsafe extern "C" fn filemap_range_has_page(
    mapping: *mut address_space,
    start_byte: loff_t,
    end_byte: loff_t,
) -> bool {
    let mut xas = rust_filemap_xa_state(
        mapping_i_pages(mapping),
        (start_byte >> PAGE_SHIFT) as Pgoff,
    );
    let max = (end_byte >> PAGE_SHIFT) as Pgoff;
    if end_byte < start_byte {
        return false;
    }
    rcu_read_lock();
    let folio = loop {
        let folio = xas_find(&mut xas, max);
        if xas_retry(&mut xas, folio) || xa_is_value(folio) {
            continue;
        }
        break folio;
    };
    rcu_read_unlock();
    !folio.is_null()
}
unsafe fn __filemap_fdatawait_range(
    mapping: *mut address_space,
    start_byte: loff_t,
    end_byte: loff_t,
) {
    let mut index = (start_byte >> PAGE_SHIFT) as Pgoff;
    let end = (end_byte >> PAGE_SHIFT) as Pgoff;
    let mut fbatch = MaybeUninit::<folio_batch>::uninit();
    folio_batch_init(fbatch.as_mut_ptr());
    let fbatch = fbatch.as_mut_ptr();
    while index <= end {
        let nr_folios = filemap_get_folios_tag(
            mapping,
            &mut index,
            end,
            RUST_FILEMAP_PAGECACHE_TAG_WRITEBACK,
            fbatch,
        );
        if nr_folios == 0 {
            break;
        }
        for i in 0..nr_folios as usize {
            folio_wait_writeback((*fbatch).folios[i]);
        }
        folio_batch_release(fbatch);
        cond_resched();
    }
}
#[no_mangle]
pub unsafe extern "C" fn filemap_fdatawait_range(
    mapping: *mut address_space,
    start_byte: loff_t,
    end_byte: loff_t,
) -> c_int {
    __filemap_fdatawait_range(mapping, start_byte, end_byte);
    filemap_check_errors(mapping)
}
#[no_mangle]
pub unsafe extern "C" fn filemap_fdatawait_range_keep_errors(
    mapping: *mut address_space,
    start_byte: loff_t,
    end_byte: loff_t,
) -> c_int {
    __filemap_fdatawait_range(mapping, start_byte, end_byte);
    filemap_check_and_keep_errors(mapping)
}
#[no_mangle]
pub unsafe extern "C" fn file_fdatawait_range(
    file: *mut file,
    start_byte: loff_t,
    end_byte: loff_t,
) -> c_int {
    __filemap_fdatawait_range(file_mapping(file), start_byte, end_byte);
    file_check_and_advance_wb_err(file)
}
#[no_mangle]
pub unsafe extern "C" fn filemap_fdatawait_keep_errors(mapping: *mut address_space) -> c_int {
    __filemap_fdatawait_range(mapping, 0, i64::MAX);
    filemap_check_and_keep_errors(mapping)
}
unsafe fn mapping_needs_writeback(mapping: *mut address_space) -> bool {
    (*mapping_nrpages(mapping)) != 0
}
#[no_mangle]
pub unsafe extern "C" fn filemap_range_has_writeback(
    mapping: *mut address_space,
    start_byte: loff_t,
    end_byte: loff_t,
) -> bool {
    let mut xas = rust_filemap_xa_state(
        mapping_i_pages(mapping),
        (start_byte >> PAGE_SHIFT) as Pgoff,
    );
    let max = (end_byte >> PAGE_SHIFT) as Pgoff;
    if end_byte < start_byte {
        return false;
    }
    rcu_read_lock();
    let mut folio = xas_find(&mut xas, max) as *mut folio;
    while !folio.is_null() {
        if !xas_retry(&mut xas, folio.cast())
            && !xa_is_value(folio.cast())
            && (folio_test_dirty(folio) || folio_test_locked(folio) || folio_test_writeback(folio))
        {
            break;
        }
        folio = xas_next_entry(&mut xas, max) as *mut folio;
    }
    rcu_read_unlock();
    !folio.is_null()
}
#[no_mangle]
pub unsafe extern "C" fn filemap_write_and_wait_range(
    mapping: *mut address_space,
    lstart: loff_t,
    lend: loff_t,
) -> c_int {
    let mut err = 0;
    if lend < lstart {
        return 0;
    }
    if mapping_needs_writeback(mapping) {
        err = filemap_fdatawrite_range(mapping, lstart, lend);
        if err != -(EIO as c_int) {
            __filemap_fdatawait_range(mapping, lstart, lend);
        }
    }
    let err2 = filemap_check_errors(mapping);
    if err == 0 {
        err = err2;
    }
    err
}
#[no_mangle]
pub unsafe extern "C" fn __filemap_set_wb_err(mapping: *mut address_space, err: c_int) {
    let eseq = errseq_set(mapping_wb_err(mapping), err);
    trace_filemap_set_wb_err(mapping, eseq);
}
#[no_mangle]
pub unsafe extern "C" fn file_check_and_advance_wb_err(file: *mut file) -> c_int {
    let mut err = 0;
    let mut old = rust_filemap_file_wb_err_read_once(file);
    let mapping = file_mapping(file);
    if errseq_check(mapping_wb_err(mapping), old) != 0 {
        spin_lock(file_lock_ptr(file));
        old = *file_wb_err_ptr(file);
        err = errseq_check_and_advance(mapping_wb_err(mapping), file_wb_err_ptr(file));
        trace_file_check_and_advance_wb_err(file, old);
        spin_unlock(file_lock_ptr(file));
    }
    clear_bit(AS_EIO as c_ulong, mapping_flags(mapping));
    clear_bit(AS_ENOSPC as c_ulong, mapping_flags(mapping));
    err
}
#[no_mangle]
pub unsafe extern "C" fn file_write_and_wait_range(
    file: *mut file,
    lstart: loff_t,
    lend: loff_t,
) -> c_int {
    let mut err = 0;
    let mapping = file_mapping(file);
    if lend < lstart {
        return 0;
    }
    if mapping_needs_writeback(mapping) {
        err = filemap_fdatawrite_range(mapping, lstart, lend);
        if err != -(EIO as c_int) {
            __filemap_fdatawait_range(mapping, lstart, lend);
        }
    }
    let err2 = file_check_and_advance_wb_err(file);
    if err == 0 {
        err = err2;
    }
    err
}
