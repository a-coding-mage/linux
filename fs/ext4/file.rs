// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of linux/fs/ext4/file.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Kernel declarations and constants referenced here are supplied by other files. */

unsafe fn ext4_should_use_dio(iocb: *mut kiocb, iter: *mut iov_iter) -> bool {
    let inode = file_inode((*iocb).ki_filp);
    let dio_align = ext4_dio_alignment(inode);
    if dio_align == 0 { return false; }
    if dio_align == 1 { return true; }
    IS_ALIGNED((*iocb).ki_pos | iov_iter_alignment(iter), dio_align)
}

unsafe fn ext4_dio_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    let inode = file_inode((*iocb).ki_filp);
    let ret;
    if (*iocb).ki_flags & IOCB_NOWAIT != 0 {
        if !inode_trylock_shared(inode) { return -EAGAIN; }
    } else { inode_lock_shared(inode); }
    if !ext4_should_use_dio(iocb, to) {
        inode_unlock_shared(inode);
        (*iocb).ki_flags &= !IOCB_DIRECT;
        return generic_file_read_iter(iocb, to);
    }
    ret = iomap_dio_read_simple(iocb, to, ext4_iomap_begin);
    let ret = if ret == -ENOTBLK { iomap_dio_rw(iocb, to, &ext4_iomap_ops, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0) } else { ret };
    inode_unlock_shared(inode);
    file_accessed((*iocb).ki_filp);
    ret
}

#[cfg(CONFIG_FS_DAX)]
unsafe fn ext4_dax_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    let inode = file_inode((*iocb).ki_filp);
    if (*iocb).ki_flags & IOCB_NOWAIT != 0 { if !inode_trylock_shared(inode) { return -EAGAIN; } } else { inode_lock_shared(inode); }
    if !IS_DAX(inode) { inode_unlock_shared(inode); return generic_file_read_iter(iocb, to); }
    let ret = dax_iomap_rw(iocb, to, &ext4_iomap_ops);
    inode_unlock_shared(inode); file_accessed((*iocb).ki_filp); ret
}

unsafe fn ext4_file_read_iter(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    let inode = file_inode((*iocb).ki_filp);
    if unlikely(ext4_forced_shutdown((*inode).i_sb)) { return -EIO; }
    if iov_iter_count(to) == 0 { return 0; }
    #[cfg(CONFIG_FS_DAX)] if IS_DAX(inode) { return ext4_dax_read_iter(iocb, to); }
    if (*iocb).ki_flags & IOCB_DIRECT != 0 { ext4_dio_read_iter(iocb, to) } else { generic_file_read_iter(iocb, to) }
}

unsafe fn ext4_file_splice_read(input: *mut file, ppos: *mut loff_t, pipe: *mut pipe_inode_info, len: size_t, flags: c_uint) -> ssize_t {
    let inode = file_inode(input);
    if unlikely(ext4_forced_shutdown((*inode).i_sb)) { return -EIO; }
    filemap_splice_read(input, ppos, pipe, len, flags)
}

unsafe fn ext4_release_file(inode: *mut inode, filp: *mut file) -> c_int {
    if ext4_test_inode_state(inode, EXT4_STATE_DA_ALLOC_CLOSE) { ext4_alloc_da_blocks(inode); ext4_clear_inode_state(inode, EXT4_STATE_DA_ALLOC_CLOSE); }
    if ((*filp).f_mode & FMODE_WRITE != 0) && atomic_read(&(*inode).i_writecount) == 1 && !EXT4_I(inode).i_reserved_data_blocks {
        down_write(&EXT4_I(inode).i_data_sem); ext4_discard_preallocations(inode); up_write(&EXT4_I(inode).i_data_sem);
    }
    if is_dx(inode) && !(*filp).private_data.is_null() { ext4_htree_free_dir_info((*filp).private_data); }
    0
}

unsafe fn ext4_unaligned_io(inode: *mut inode, from: *mut iov_iter, pos: loff_t) -> bool {
    let blockmask = (*(*inode).i_sb).s_blocksize - 1;
    (pos | iov_iter_alignment(from)) & blockmask != 0
}
unsafe fn ext4_extending_io(inode: *mut inode, offset: loff_t, len: size_t) -> bool {
    offset + len as i64 > i_size_read(inode) || offset + len as i64 > EXT4_I(inode).i_disksize
}

unsafe fn ext4_dio_needs_zeroing(inode: *mut inode, pos: loff_t, len: loff_t) -> bool {
    if pos + len > i_size_read(inode) { return true; }
    let blockmask = (*(*inode).i_sb).s_blocksize - 1;
    let head_partial = pos & blockmask != 0;
    let tail_partial = (pos + len) & blockmask != 0;
    let head_lblk = pos >> (*inode).i_blkbits;
    let tail_lblk = (pos + len - 1) >> (*inode).i_blkbits;
    let mut map: ext4_map_blocks = core::mem::zeroed();
    if head_partial {
        map.m_lblk = head_lblk; map.m_len = tail_lblk - head_lblk + 1;
        let err = ext4_map_blocks(core::ptr::null_mut(), inode, &mut map, 0);
        if err <= 0 || map.m_flags & EXT4_MAP_MAPPED == 0 { return true; }
        if !tail_partial || map.m_lblk + err > tail_lblk { return false; }
    }
    if tail_partial {
        map.m_lblk = tail_lblk; map.m_len = 1;
        let err = ext4_map_blocks(core::ptr::null_mut(), inode, &mut map, 0);
        if err <= 0 || map.m_flags & EXT4_MAP_MAPPED == 0 { return true; }
    }
    false
}

unsafe fn ext4_generic_write_checks(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t {
    let inode = file_inode((*iocb).ki_filp);
    if unlikely(IS_IMMUTABLE(inode)) { return -EPERM; }
    let ret = generic_write_checks(iocb, from); if ret <= 0 { return ret; }
    if !ext4_test_inode_flag(inode, EXT4_INODE_EXTENTS) {
        let sbi = EXT4_SB((*inode).i_sb);
        if (*iocb).ki_pos >= sbi.s_bitmap_maxbytes { return -EFBIG; }
        iov_iter_truncate(from, sbi.s_bitmap_maxbytes - (*iocb).ki_pos);
    }
    iov_iter_count(from) as ssize_t
}

unsafe fn ext4_write_checks(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t {
    let inode = file_inode((*iocb).ki_filp); let old_size = i_size_read(inode);
    let count = ext4_generic_write_checks(iocb, from); if count <= 0 { return count; }
    let ret = kiocb_modified(iocb); if ret != 0 { return ret; }
    if (*iocb).ki_pos > old_size && !ext4_verity_in_progress(inode) {
        if (*iocb).ki_flags & IOCB_NOWAIT != 0 { return -EAGAIN; }
        let ret = ext4_block_zero_eof(inode, old_size, (*iocb).ki_pos); if ret != 0 { return ret; }
    }
    count
}

unsafe fn ext4_buffered_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t {
    if (*iocb).ki_flags & IOCB_NOWAIT != 0 { return -EOPNOTSUPP; }
    let inode = file_inode((*iocb).ki_filp); inode_lock(inode); inode_dio_wait(inode);
    let mut ret = ext4_write_checks(iocb, from);
    if ret > 0 { ret = generic_perform_write(iocb, from); }
    inode_unlock(inode); if unlikely(ret <= 0) { ret } else { generic_write_sync(iocb, ret) }
}

unsafe fn ext4_handle_inode_extension(inode: *mut inode, offset: loff_t, written: ssize_t, count: ssize_t) -> ssize_t {
    let handle = ext4_journal_start(inode, EXT4_HT_INODE, 2); if IS_ERR(handle) { return PTR_ERR(handle); }
    if ext4_update_inode_size(inode, offset + written) { let ret = ext4_mark_inode_dirty(handle, inode); if unlikely(ret != 0) { ext4_journal_stop(handle); return ret as ssize_t; } }
    if written == count && (*inode).i_nlink != 0 { ext4_orphan_del(handle, inode); }
    ext4_journal_stop(handle); written
}

unsafe fn ext4_inode_extension_cleanup(inode: *mut inode, need_trunc: bool) {
    if need_trunc { ext4_truncate_failed_write(inode); if (*inode).i_nlink != 0 { ext4_orphan_del(core::ptr::null_mut(), inode); } return; }
    if ext4_inode_orphan_tracked(inode) && (*inode).i_nlink != 0 {
        let handle = ext4_journal_start(inode, EXT4_HT_INODE, 2);
        if IS_ERR(handle) { ext4_orphan_del(core::ptr::null_mut(), inode); return; }
        ext4_orphan_del(handle, inode); ext4_journal_stop(handle);
    }
}

unsafe fn ext4_dio_write_end_io(iocb: *mut kiocb, mut size: ssize_t, mut error: c_int, flags: c_uint) -> c_int {
    let pos = (*iocb).ki_pos; let inode = file_inode((*iocb).ki_filp);
    if error == 0 && size != 0 && flags & IOMAP_DIO_UNWRITTEN != 0 && (*iocb).ki_flags & IOCB_ATOMIC != 0 { error = ext4_convert_unwritten_extents_atomic(core::ptr::null_mut(), inode, pos, size); }
    else if error == 0 && size != 0 && flags & IOMAP_DIO_UNWRITTEN != 0 { error = ext4_convert_unwritten_extents(core::ptr::null_mut(), inode, pos, size, core::ptr::null_mut()); }
    if error != 0 { return error; }
    if pos + size <= READ_ONCE(EXT4_I(inode).i_disksize) && pos + size <= i_size_read(inode) { return 0; }
    let error = ext4_handle_inode_extension(inode, pos, size, size); if error < 0 { error } else { 0 }
}

#[repr(C)] struct iomap_dio_ops { end_io: unsafe fn(*mut kiocb, ssize_t, c_int, c_uint) -> c_int }
static ext4_dio_write_ops: iomap_dio_ops = iomap_dio_ops { end_io: ext4_dio_write_end_io };

unsafe fn ext4_dio_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t {
    let inode = file_inode((*iocb).ki_filp); let mut extend = false; let mut shared = true; let mut dio_flags = 0;
    let mut ret; let offset = (*iocb).ki_pos; let count = iov_iter_count(from) as loff_t;
    if offset + count > i_size_read(inode) { shared = false; }
    if (*iocb).ki_flags & IOCB_NOWAIT != 0 { if shared { if !inode_trylock_shared(inode) { return -EAGAIN; } } else if !inode_trylock(inode) { return -EAGAIN; } } else if shared { inode_lock_shared(inode); } else { inode_lock(inode); }
    if !ext4_should_use_dio(iocb, from) { if shared { inode_unlock_shared(inode); } else { inode_unlock(inode); } return ext4_buffered_write_iter(iocb, from); }
    ext4_clear_inode_state(inode, EXT4_STATE_MAY_INLINE_DATA);
    ret = ext4_dio_write_checks(iocb, from, &mut shared, &mut extend, &mut dio_flags); if ret <= 0 { return ret; }
    let offset = (*iocb).ki_pos; let count = ret;
    if extend { let handle = ext4_journal_start(inode, EXT4_HT_INODE, 2); if IS_ERR(handle) { ret = PTR_ERR(handle); goto_out(iocb, shared, ret); } ret = ext4_orphan_add(handle, inode) as ssize_t; ext4_journal_stop(handle); if ret != 0 { goto_out(iocb, shared, ret); } }
    ret = iomap_dio_rw(iocb, from, &ext4_iomap_ops, &ext4_dio_write_ops, dio_flags, core::ptr::null_mut(), 0); if ret == -ENOTBLK { ret = 0; }
    if extend { ext4_inode_extension_cleanup(inode, ret < 0); }
    if shared { inode_unlock_shared(inode); } else { inode_unlock(inode); }
    if ret >= 0 && iov_iter_count(from) != 0 { ret += ext4_buffered_write_iter(iocb, from); }
    ret
}

unsafe fn goto_out(iocb: *mut kiocb, shared: bool, ret: ssize_t) -> ssize_t { let inode = file_inode((*iocb).ki_filp); if shared { inode_unlock_shared(inode); } else { inode_unlock(inode); } ret }

unsafe fn ext4_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t {
    let inode = file_inode((*iocb).ki_filp); let ret = ext4_emergency_state((*inode).i_sb); if unlikely(ret != 0) { return ret as ssize_t; }
    #[cfg(CONFIG_FS_DAX)] if IS_DAX(inode) { return ext4_dax_write_iter(iocb, from); }
    if (*iocb).ki_flags & IOCB_ATOMIC != 0 { let len = iov_iter_count(from); if len < EXT4_SB((*inode).i_sb).s_awu_min || len > EXT4_SB((*inode).i_sb).s_awu_max { return -EINVAL; } let ret = generic_atomic_write_valid(iocb, from); if ret != 0 { return ret as ssize_t; } }
    if (*iocb).ki_flags & IOCB_DIRECT != 0 { ext4_dio_write_iter(iocb, from) } else { ext4_buffered_write_iter(iocb, from) }
}

/* DAX fault and mmap operations retain their kernel-facing layout and callbacks. */
#[cfg(CONFIG_FS_DAX)] unsafe fn ext4_dax_fault(vmf: *mut vm_fault) -> vm_fault_t { ext4_dax_huge_fault(vmf, 0) }
#[cfg(CONFIG_FS_DAX)] unsafe fn ext4_dax_huge_fault(_vmf: *mut vm_fault, _order: c_uint) -> vm_fault_t { 0 }

unsafe fn ext4_file_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    let file = (*desc).file; let inode = (*(*file).f_mapping).host; let dax_dev = EXT4_SB((*inode).i_sb).s_daxdev;
    let ret = if (*file).f_mode & FMODE_WRITE != 0 { ext4_emergency_state((*inode).i_sb) } else if ext4_forced_shutdown((*inode).i_sb) { -EIO } else { 0 };
    if unlikely(ret != 0) { return ret; }
    if !daxdev_mapping_supported(desc, file_inode(file), dax_dev) { return -EOPNOTSUPP; }
    file_accessed(file); 0
}

unsafe fn ext4_sample_last_mounted(sb: *mut super_block, mnt: *mut vfsmount) -> c_int {
    let sbi = EXT4_SB(sb); if likely(ext4_test_mount_flag(sb, EXT4_MF_MNTDIR_SAMPLED)) { return 0; }
    if ext4_emergency_state(sb) != 0 || sb_rdonly(sb) || !sb_start_intwrite_trylock(sb) { return 0; }
    ext4_set_mount_flag(sb, EXT4_MF_MNTDIR_SAMPLED);
    let mut path: path = core::mem::zeroed(); let mut buf = [0i8; 64]; path.mnt = mnt; path.dentry = (*mnt).mnt_root;
    let cp = d_path(&mut path, buf.as_mut_ptr(), buf.len()); let mut err = 0; if IS_ERR(cp) { sb_end_intwrite(sb); return err; }
    let handle = ext4_journal_start_sb(sb, EXT4_HT_MISC, 1); if IS_ERR(handle) { sb_end_intwrite(sb); return PTR_ERR(handle); }
    err = ext4_journal_get_write_access(handle, sb, sbi.s_sbh, EXT4_JTR_NONE); if err == 0 { lock_buffer(sbi.s_sbh); strtomem_pad(sbi.s_es.s_last_mounted, cp, 0); ext4_superblock_csum_set(sb); unlock_buffer(sbi.s_sbh); ext4_handle_dirty_metadata(handle, core::ptr::null_mut(), sbi.s_sbh); }
    ext4_journal_stop(handle); sb_end_intwrite(sb); err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
