// SPDX-License-Identifier: GPL-2.0
/*
 * (C) 2001 Clemson University and The University of Chicago
 * Copyright 2018 Omnibond Systems, L.L.C.
 *
 * See COPYING in top-level directory.
 */

/* Linux VFS file operations. */

unsafe fn flush_racache(inode: *mut inode) -> c_int {
    let orangefs_inode = ORANGEFS_I(inode);
    let new_op = op_alloc(ORANGEFS_VFS_OP_RA_FLUSH);
    if new_op.is_null() { return -ENOMEM; }

    gossip_debug(GOSSIP_UTILS_DEBUG, "%s: %pU: Handle is %pU | fs_id %d\n", __func__,
        get_khandle_from_ino(inode), &(*orangefs_inode).refn.khandle,
        (*orangefs_inode).refn.fs_id);
    (*new_op).upcall.req.ra_cache_flush.refn = (*orangefs_inode).refn;
    let ret = service_operation(new_op, "orangefs_flush_racache", get_interruptible_flag(inode));
    gossip_debug(GOSSIP_UTILS_DEBUG, "%s: got return value of %d\n", __func__, ret);
    op_release(new_op);
    ret
}

/* Post and wait for the I/O upcall to finish. */
pub unsafe fn wait_for_direct_io(
    type_: ORANGEFS_io_type, inode: *mut inode, offset: *mut loff_t,
    iter: *mut iov_iter, total_size: size_t, readahead_size: loff_t,
    wr: *mut orangefs_write_range, index_return: *mut c_int, file: *mut file,
) -> ssize_t {
    let orangefs_inode = ORANGEFS_I(inode);
    let handle = &mut (*orangefs_inode).refn.khandle as *mut orangefs_khandle;
    let new_op = op_alloc(ORANGEFS_VFS_OP_FILE_IO);
    if new_op.is_null() { return -ENOMEM; }
    (*new_op).upcall.req.io.readahead_size = readahead_size;
    (*new_op).upcall.req.io.io_type = type_;
    (*new_op).upcall.req.io.refn = (*orangefs_inode).refn;
    let mut buffer_index: c_int;
    let mut ret: ssize_t;
    let mut copy_amount: size_t;
    let open_for_read: bool;
    let open_for_write: bool;

    'populate_shared_memory: loop {
        buffer_index = orangefs_bufmap_get();
        if buffer_index < 0 { ret = buffer_index as ssize_t; gossip_debug(GOSSIP_FILE_DEBUG, "%s: orangefs_bufmap_get failure (%zd)\n", __func__, ret); break; }
        gossip_debug(GOSSIP_FILE_DEBUG, "%s(%pU): GET op %p -> buffer_index %d\n", __func__, handle, new_op, buffer_index);
        (*new_op).uses_shared_memory = 1;
        (*new_op).upcall.req.io.buf_index = buffer_index;
        (*new_op).upcall.req.io.count = total_size;
        (*new_op).upcall.req.io.offset = *offset;
        if type_ == ORANGEFS_IO_WRITE && !wr.is_null() {
            (*new_op).upcall.uid = from_kuid(&init_user_ns, (*wr).uid);
            (*new_op).upcall.gid = from_kgid(&init_user_ns, (*wr).gid);
        }
        if !file.is_null() {
            open_for_write = (*file).f_mode & FMODE_WRITE != 0;
            open_for_read = (*file).f_mode & FMODE_READ != 0;
        } else { open_for_write = true; open_for_read = false; }
        if type_ == ORANGEFS_IO_WRITE && open_for_write { (*new_op).upcall.uid = 0; }
        if type_ == ORANGEFS_IO_READ && open_for_read { (*new_op).upcall.uid = 0; }
        gossip_debug(GOSSIP_FILE_DEBUG, "%s(%pU): offset: %llu total_size: %zd\n", __func__, handle, llu(*offset), total_size);
        if type_ == ORANGEFS_IO_WRITE && total_size != 0 {
            ret = orangefs_bufmap_copy_from_iovec(iter, buffer_index, total_size);
            if ret < 0 { gossip_err("%s: Failed to copy-in buffers. Please make sure that the pvfs2-client is running. %ld\n", __func__, ret); break; }
        }
        gossip_debug(GOSSIP_FILE_DEBUG, "%s(%pU): Calling post_io_request with tag (%llu)\n", __func__, handle, llu((*new_op).tag));
        ret = service_operation(new_op, if type_ == ORANGEFS_IO_WRITE { "file_write" } else { "file_read" }, get_interruptible_flag(inode));
        if ret == -EAGAIN && op_state_purged(new_op) {
            orangefs_bufmap_put(buffer_index);
            if type_ == ORANGEFS_IO_WRITE { iov_iter_revert(iter, total_size); }
            gossip_debug(GOSSIP_FILE_DEBUG, "%s:going to repopulate_shared_memory.\n", __func__);
            continue 'populate_shared_memory;
        }
        if ret < 0 {
            if ret == -EINTR {
                match (*new_op).op_state - OP_VFS_STATE_GIVEN_UP {
                    OP_VFS_STATE_WAITING => { ret = if *offset == 0 { -EINTR } else { 0 }; }
                    OP_VFS_STATE_INPROGR => { ret = if type_ == ORANGEFS_IO_READ { -EINTR } else { total_size as ssize_t }; }
                    _ => { gossip_err("%s: unexpected op state :%d:.\n", __func__, (*new_op).op_state); ret = 0; }
                }
                gossip_debug(GOSSIP_FILE_DEBUG, "%s: got EINTR, state:%d: %p\n", __func__, (*new_op).op_state, new_op);
            } else { gossip_err("%s: error in %s handle %pU, returning %zd\n", __func__, if type_ == ORANGEFS_IO_READ { "read from" } else { "write to" }, handle, ret); }
            if orangefs_cancel_op_in_progress(new_op) != 0 { return ret; }
            break;
        }
        if type_ == ORANGEFS_IO_READ && (*new_op).downcall.resp.io.amt_complete != 0 {
            copy_amount = (*new_op).downcall.resp.io.amt_complete;
            ret = orangefs_bufmap_copy_to_iovec(iter, buffer_index, copy_amount);
            if ret < 0 { gossip_err("%s: Failed to copy-out buffers. Please make sure that the pvfs2-client is running (%ld)\n", __func__, ret); break; }
        }
        ret = (*new_op).downcall.resp.io.amt_complete as ssize_t;
        break;
    }
    if buffer_index >= 0 { orangefs_bufmap_put(buffer_index); gossip_debug(GOSSIP_FILE_DEBUG, "%s(%pU): PUT buffer_index %d\n", __func__, handle, buffer_index); }
    op_release(new_op);
    ret
}

pub unsafe fn orangefs_revalidate_mapping(inode: *mut inode) -> c_int {
    let oi = ORANGEFS_I(inode); let mapping = (*inode).i_mapping; let bitlock = &mut (*oi).bitlock as *mut c_ulong; let ret;
    loop { ret = wait_on_bit(bitlock, 1, TASK_KILLABLE); if ret != 0 { return ret; } spin_lock(&mut (*inode).i_lock); if test_bit(1, bitlock) != 0 { spin_unlock(&mut (*inode).i_lock); continue; } if !time_before(jiffies, (*oi).mapping_time) { break; } spin_unlock(&mut (*inode).i_lock); return 0; }
    set_bit(1, bitlock); smp_wmb(); spin_unlock(&mut (*inode).i_lock); unmap_mapping_range(mapping, 0, 0, 0); let mut r = filemap_write_and_wait(mapping); if r == 0 { r = invalidate_inode_pages2(mapping); } (*oi).mapping_time = jiffies + orangefs_cache_timeout_msecs * HZ / 1000; clear_bit(1, bitlock); smp_mb__after_atomic(); wake_up_bit(bitlock, 1); r
}

unsafe fn orangefs_file_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t { orangefs_stats.reads += 1; let inode = file_inode((*iocb).ki_filp); down_read(&mut (*inode).i_rwsem); let ret = orangefs_revalidate_mapping(inode); let ret = if ret != 0 { ret as ssize_t } else { generic_file_read_iter(iocb, iter) }; up_read(&mut (*inode).i_rwsem); ret }
unsafe fn orangefs_file_splice_read(input: *mut file, ppos: *mut loff_t, pipe: *mut pipe_inode_info, len: size_t, flags: c_uint) -> ssize_t { let inode = file_inode(input); orangefs_stats.reads += 1; down_read(&mut (*inode).i_rwsem); let ret = orangefs_revalidate_mapping(inode); let ret = if ret != 0 { ret as ssize_t } else { filemap_splice_read(input, ppos, pipe, len, flags) }; up_read(&mut (*inode).i_rwsem); ret }
unsafe fn orangefs_file_write_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t { orangefs_stats.writes += 1; let inode = file_inode((*iocb).ki_filp); if (*iocb).ki_pos > i_size_read(inode) { let ret = orangefs_revalidate_mapping(inode); if ret != 0 { return ret as ssize_t; } } generic_file_write_iter(iocb, iter) }
unsafe fn orangefs_fault(vmf: *mut vm_fault) -> vm_fault_t { let file = (*(*vmf).vma).vm_file; let mut ret = orangefs_inode_getattr((*file).f_mapping.host, ORANGEFS_GETATTR_SIZE); if ret == -ESTALE { ret = -EIO; } if ret != 0 { gossip_err("%s: orangefs_inode_getattr failed, ret:%d:.\n", __func__, ret); return VM_FAULT_SIGBUS; } filemap_fault(vmf) }

static orangefs_file_vm_ops: vm_operations_struct = vm_operations_struct { fault: Some(orangefs_fault), map_pages: Some(filemap_map_pages), page_mkwrite: Some(orangefs_page_mkwrite) };

unsafe fn orangefs_file_mmap_prepare(desc: *mut vm_area_desc) -> c_int { let file = (*desc).file; let ret = orangefs_revalidate_mapping(file_inode(file)); if ret != 0 { return ret; } gossip_debug(GOSSIP_FILE_DEBUG, "orangefs_file_mmap: called on %pD\n", file); vma_desc_set_flags(desc, VMA_SEQ_READ_BIT); vma_desc_clear_flags(desc, VMA_RAND_READ_BIT); file_accessed(file); (*desc).vm_ops = &orangefs_file_vm_ops; 0 }

unsafe fn orangefs_file_release(inode: *mut inode, file: *mut file) -> c_int { gossip_debug(GOSSIP_FILE_DEBUG, "orangefs_file_release: called on %pD\n", file); if (*file).f_mapping.nrpages != 0 && orangefs_features & ORANGEFS_FEATURE_READAHEAD != 0 { gossip_debug(GOSSIP_INODE_DEBUG, "calling flush_racache on %pU\n", get_khandle_from_ino(inode)); flush_racache(inode); gossip_debug(GOSSIP_INODE_DEBUG, "flush_racache finished\n"); } 0 }

unsafe fn orangefs_fsync(file: *mut file, start: loff_t, end: loff_t, _datasync: c_int) -> c_int { let inode = file_inode(file); let mut ret = filemap_write_and_wait_range((*inode).i_mapping, start, end); if ret < 0 { return ret; } let op = op_alloc(ORANGEFS_VFS_OP_FSYNC); if op.is_null() { return -ENOMEM; } (*op).upcall.req.fsync.refn = (*ORANGEFS_I(inode)).refn; ret = service_operation(op, "orangefs_fsync", get_interruptible_flag(inode)); gossip_debug(GOSSIP_FILE_DEBUG, "orangefs_fsync got return value of %d\n", ret); op_release(op); ret }

unsafe fn orangefs_file_llseek(file: *mut file, offset: loff_t, origin: c_int) -> loff_t { let inode = file_inode(file); if origin == SEEK_END { let mut ret = orangefs_inode_getattr((*file).f_mapping.host, ORANGEFS_GETATTR_SIZE); if ret == -ESTALE { ret = -EIO; } if ret != 0 { return ret as loff_t; } } gossip_debug(GOSSIP_FILE_DEBUG, "orangefs_file_llseek: offset is %ld | origin is %d | inode size is %lu\n", offset, origin, i_size_read(inode)); generic_file_llseek(file, offset, origin) }
unsafe fn orangefs_lock(filp: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int { let mut rc = -EINVAL; if (*ORANGEFS_SB((*file_inode(filp)).i_sb)).flags & ORANGEFS_OPT_LOCAL_LOCK != 0 { if cmd == F_GETLK { rc = 0; posix_test_lock(filp, fl); } else { rc = posix_lock_file(filp, fl, core::ptr::null_mut()); } } rc }
unsafe fn orangefs_flush(file: *mut file, _id: fl_owner_t) -> c_int { let r = filemap_write_and_wait_range((*file).f_mapping, 0, LLONG_MAX); if r > 0 { 0 } else { r } }

pub static orangefs_file_operations: file_operations = file_operations {
    llseek: Some(orangefs_file_llseek), read_iter: Some(orangefs_file_read_iter), write_iter: Some(orangefs_file_write_iter), lock: Some(orangefs_lock), mmap_prepare: Some(orangefs_file_mmap_prepare), open: Some(generic_file_open), splice_read: Some(orangefs_file_splice_read), splice_write: Some(iter_file_splice_write), flush: Some(orangefs_flush), release: Some(orangefs_file_release), fsync: Some(orangefs_fsync), setlease: Some(generic_setlease),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
