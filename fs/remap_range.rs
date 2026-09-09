// SPDX-License-Identifier: GPL-2.0-only
// Linux kernel dependencies supplied by the surrounding translation unit.

unsafe fn generic_remap_checks(file_in: *mut file, pos_in: loff_t,
    file_out: *mut file, pos_out: loff_t, req_count: *mut loff_t,
    remap_flags: c_uint) -> c_int {
    let inode_in = (*(*file_in).f_mapping).host;
    let inode_out = (*(*file_out).f_mapping).host;
    let mut count = *req_count as u64;
    let bcount: u64;
    let size_in: loff_t;
    let size_out: loff_t;
    let bs = (*(*inode_out).i_sb).s_blocksize as loff_t;

    if !IS_ALIGNED(pos_in, bs) || !IS_ALIGNED(pos_out, bs) { return -EINVAL; }
    if pos_in.wrapping_add(count as loff_t) < pos_in || pos_out.wrapping_add(count as loff_t) < pos_out { return -EINVAL; }
    size_in = i_size_read(inode_in);
    size_out = i_size_read(inode_out);
    if (remap_flags & REMAP_FILE_DEDUP) != 0 &&
       (pos_in >= size_in || pos_in + count as loff_t > size_in || pos_out >= size_out || pos_out + count as loff_t > size_out) { return -EINVAL; }
    if pos_in >= size_in { return -EINVAL; }
    count = core::cmp::min(count, (size_in - pos_in) as u64);
    let ret = generic_write_check_limits(file_out, pos_out, &mut count as *mut u64);
    if ret != 0 { return ret; }
    if pos_in + count as loff_t == size_in &&
       ((remap_flags & REMAP_FILE_DEDUP) == 0 || pos_out + count as loff_t == size_out) {
        bcount = (ALIGN(size_in, bs) - pos_in) as u64;
    } else {
        if !IS_ALIGNED(count as loff_t, bs) { count = ALIGN_DOWN(count as loff_t, bs) as u64; }
        bcount = count;
    }
    if inode_in == inode_out && pos_out + bcount as loff_t > pos_in && pos_out < pos_in + bcount as loff_t { return -EINVAL; }
    if *req_count as u64 != count && (remap_flags & REMAP_FILE_CAN_SHORTEN) == 0 { return -EINVAL; }
    *req_count = count as loff_t;
    0
}

#[no_mangle]
pub unsafe extern "C" fn remap_verify_area(file: *mut file, pos: loff_t, len: loff_t, write: bool) -> c_int {
    let mask = if write { MAY_WRITE } else { MAY_READ };
    let mut tmp: loff_t = 0;
    if unlikely(pos < 0 || len < 0) { return -EINVAL; }
    if unlikely(check_add_overflow(pos, len, &mut tmp)) { return -EINVAL; }
    let ret = security_file_permission(file, mask);
    if ret != 0 { return ret; }
    fsnotify_file_area_perm(file, mask, &mut (pos as loff_t), len)
}

unsafe fn generic_remap_check_len(inode_in: *mut inode, inode_out: *mut inode, pos_out: loff_t, len: *mut loff_t, remap_flags: c_uint) -> c_int {
    let blkmask = (i_blocksize(inode_in) - 1) as loff_t;
    let mut new_len = *len;
    if (*len & blkmask) == 0 { return 0; }
    if pos_out + *len < i_size_read(inode_out) { new_len &= !blkmask; }
    if new_len == *len { return 0; }
    if (remap_flags & REMAP_FILE_CAN_SHORTEN) != 0 { *len = new_len; return 0; }
    if (remap_flags & REMAP_FILE_DEDUP) != 0 { -EBADE } else { -EINVAL }
}

unsafe fn vfs_dedupe_get_folio(file: *mut file, pos: loff_t) -> *mut folio { read_mapping_folio((*file).f_mapping, pos >> PAGE_SHIFT, file) }

unsafe fn vfs_lock_two_folios(mut folio1: *mut folio, mut folio2: *mut folio) {
    if (*folio1).index > (*folio2).index { core::mem::swap(&mut folio1, &mut folio2); }
    folio_lock(folio1); if folio1 != folio2 { folio_lock(folio2); }
}
unsafe fn vfs_unlock_two_folios(folio1: *mut folio, folio2: *mut folio) { folio_unlock(folio1); if folio1 != folio2 { folio_unlock(folio2); } }

unsafe fn vfs_dedupe_file_range_compare(src: *mut file, mut srcoff: loff_t, dest: *mut file, mut dstoff: loff_t, mut len: loff_t, is_same: *mut bool) -> c_int {
    let mut same = true;
    while len != 0 {
        let cmp_len = core::cmp::min(core::cmp::min(PAGE_SIZE - offset_in_page(srcoff), PAGE_SIZE - offset_in_page(dstoff)), len);
        if cmp_len <= 0 { return -EINVAL; }
        let src_folio = vfs_dedupe_get_folio(src, srcoff); if IS_ERR(src_folio) { return PTR_ERR(src_folio); }
        let dst_folio = vfs_dedupe_get_folio(dest, dstoff); if IS_ERR(dst_folio) { folio_put(src_folio); return PTR_ERR(dst_folio); }
        vfs_lock_two_folios(src_folio, dst_folio);
        if !folio_test_uptodate(src_folio) || !folio_test_uptodate(dst_folio) || (*src_folio).mapping != (*src).f_mapping || (*dst_folio).mapping != (*dest).f_mapping { same = false; vfs_unlock_two_folios(src_folio, dst_folio); folio_put(dst_folio); folio_put(src_folio); break; }
        let src_addr = kmap_local_folio(src_folio, offset_in_folio(src_folio, srcoff));
        let dst_addr = kmap_local_folio(dst_folio, offset_in_folio(dst_folio, dstoff));
        flush_dcache_folio(src_folio); flush_dcache_folio(dst_folio);
        if memcmp(src_addr, dst_addr, cmp_len) != 0 { same = false; }
        kunmap_local(dst_addr); kunmap_local(src_addr); vfs_unlock_two_folios(src_folio, dst_folio); folio_put(dst_folio); folio_put(src_folio);
        if !same { break; } srcoff += cmp_len; dstoff += cmp_len; len -= cmp_len;
    }
    *is_same = same; 0
}

#[no_mangle]
pub unsafe extern "C" fn __generic_remap_file_range_prep(file_in: *mut file, pos_in: loff_t, file_out: *mut file, pos_out: loff_t, len: *mut loff_t, remap_flags: c_uint, dax_read_ops: *const iomap_ops) -> c_int {
    let inode_in = file_inode(file_in); let inode_out = file_inode(file_out); let same_inode = inode_in == inode_out;
    if IS_IMMUTABLE(inode_out) { return -EPERM; } if IS_SWAPFILE(inode_in) || IS_SWAPFILE(inode_out) { return -ETXTBSY; }
    if S_ISDIR((*inode_in).i_mode) || S_ISDIR((*inode_out).i_mode) { return -EISDIR; } if !S_ISREG((*inode_in).i_mode) || !S_ISREG((*inode_out).i_mode) { return -EINVAL; }
    if *len == 0 { let isize = i_size_read(inode_in); if remap_flags & REMAP_FILE_DEDUP != 0 || pos_in == isize { return 0; } if pos_in > isize { return -EINVAL; } *len = isize - pos_in; if *len == 0 { return 0; } }
    let mut ret = generic_remap_checks(file_in, pos_in, file_out, pos_out, len, remap_flags); if ret != 0 || *len == 0 { return ret; }
    inode_dio_wait(inode_in); if !same_inode { inode_dio_wait(inode_out); }
    ret = filemap_write_and_wait_range((*inode_in).i_mapping, pos_in, pos_in + *len - 1); if ret != 0 { return ret; }
    ret = filemap_write_and_wait_range((*inode_out).i_mapping, pos_out, pos_out + *len - 1); if ret != 0 { return ret; }
    if remap_flags & REMAP_FILE_DEDUP != 0 { let mut is_same = false; if !IS_DAX(inode_in) { ret = vfs_dedupe_file_range_compare(file_in, pos_in, file_out, pos_out, *len, &mut is_same); } else if !dax_read_ops.is_null() { ret = dax_dedupe_file_range_compare(inode_in, pos_in, inode_out, pos_out, *len, &mut is_same, dax_read_ops); } else { return -EINVAL; } if ret != 0 { return ret; } if !is_same { return -EBADE; } }
    ret = generic_remap_check_len(inode_in, inode_out, pos_out, len, remap_flags); if ret != 0 || *len == 0 { return ret; }
    if remap_flags & REMAP_FILE_DEDUP == 0 { ret = file_modified(file_out); } ret
}

#[no_mangle]
pub unsafe extern "C" fn generic_remap_file_range_prep(file_in: *mut file, pos_in: loff_t, file_out: *mut file, pos_out: loff_t, len: *mut loff_t, remap_flags: c_uint) -> c_int { __generic_remap_file_range_prep(file_in, pos_in, file_out, pos_out, len, remap_flags, core::ptr::null()) }

#[no_mangle]
pub unsafe extern "C" fn vfs_clone_file_range(file_in: *mut file, pos_in: loff_t, file_out: *mut file, pos_out: loff_t, len: loff_t, remap_flags: c_uint) -> loff_t {
    if (*file_inode(file_in)).i_sb != (*file_inode(file_out)).i_sb { return -EXDEV as loff_t; }
    let mut ret = generic_file_rw_checks(file_in, file_out); if ret < 0 { return ret as loff_t; }
    if (*file_in).f_op.remap_file_range.is_none() { return -EOPNOTSUPP as loff_t; }
    ret = remap_verify_area(file_in, pos_in, len, false); if ret != 0 { return ret as loff_t; } ret = remap_verify_area(file_out, pos_out, len, true); if ret != 0 { return ret as loff_t; }
    file_start_write(file_out); ret = ((*file_in).f_op.remap_file_range.unwrap())(file_in, pos_in, file_out, pos_out, len, remap_flags); file_end_write(file_out); if ret < 0 { return ret as loff_t; }
    fsnotify_access(file_in); fsnotify_modify(file_out); ret as loff_t
}

unsafe fn may_dedupe_file(file: *mut file) -> bool {
    let idmap = file_mnt_idmap(file); let inode = file_inode(file);
    if capable(CAP_SYS_ADMIN) || (*file).f_mode & FMODE_WRITE != 0 { return true; }
    if vfsuid_eq_kuid(i_uid_into_vfsuid(idmap, inode), current_fsuid()) { return true; }
    inode_permission(idmap, inode, MAY_WRITE) == 0
}

#[no_mangle]
pub unsafe extern "C" fn vfs_dedupe_file_range_one(src_file: *mut file, src_pos: loff_t, dst_file: *mut file, dst_pos: loff_t, len: loff_t, remap_flags: c_uint) -> loff_t {
    let mut ret = remap_verify_area(src_file, src_pos, len, false); if ret != 0 { return ret as loff_t; }
    ret = remap_verify_area(dst_file, dst_pos, len, true); if ret != 0 { return ret as loff_t; }
    ret = mnt_want_write_file(dst_file); if ret != 0 { return ret as loff_t; }
    ret = -EPERM; if !may_dedupe_file(dst_file) { mnt_drop_write_file(dst_file); return ret as loff_t; }
    ret = -EXDEV; if (*file_inode(src_file)).i_sb != (*file_inode(dst_file)).i_sb { mnt_drop_write_file(dst_file); return ret as loff_t; }
    ret = -EISDIR; if S_ISDIR((*file_inode(dst_file)).i_mode) { mnt_drop_write_file(dst_file); return ret as loff_t; }
    ret = -EINVAL; if (*dst_file).f_op.remap_file_range.is_none() { mnt_drop_write_file(dst_file); return ret as loff_t; }
    if len == 0 { mnt_drop_write_file(dst_file); return 0; }
    ret = ((*dst_file).f_op.remap_file_range.unwrap())(src_file, src_pos, dst_file, dst_pos, len, remap_flags | REMAP_FILE_DEDUP); mnt_drop_write_file(dst_file); ret as loff_t
}

#[no_mangle]
pub unsafe extern "C" fn vfs_dedupe_file_range(file: *mut file, same: *mut file_dedupe_range) -> c_int {
    if (*file).f_mode & FMODE_READ == 0 || (*same).reserved1 != 0 || (*same).reserved2 != 0 { return -EINVAL; }
    let src = file_inode(file); let off = (*same).src_offset; let mut len = (*same).src_length; let count = (*same).dest_count;
    if S_ISDIR((*src).i_mode) { return -EISDIR; } if !S_ISREG((*src).i_mode) { return -EINVAL; } if (*file).f_op.remap_file_range.is_none() { return -EOPNOTSUPP; }
    let mut ret = remap_verify_area(file, off as loff_t, len as loff_t, false); if ret < 0 { return ret; } if off + len > i_size_read(src) as u64 { return -EINVAL; }
    len = core::cmp::min(len, 1u64 << 30);
    for i in 0..count { (*same).info[i as usize].bytes_deduped = 0; (*same).info[i as usize].status = FILE_DEDUPE_RANGE_SAME; }
    for i in 0..count { let info = &mut (*same).info[i as usize]; let dst_fd = CLASS_fd(info.dest_fd); if fd_empty(dst_fd) { info.status = -EBADF; continue; } if info.reserved != 0 { info.status = -EINVAL; continue; } let deduped = vfs_dedupe_file_range_one(file, off as loff_t, fd_file(dst_fd), info.dest_offset as loff_t, len as loff_t, REMAP_FILE_CAN_SHORTEN); if deduped == -EBADE as loff_t { info.status = FILE_DEDUPE_RANGE_DIFFERS; } else if deduped < 0 { info.status = deduped as _; } else { info.bytes_deduped = len; } if fatal_signal_pending(current()) { break; } }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
