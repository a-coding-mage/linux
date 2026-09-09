// SPDX-License-Identifier: GPL-2.0
// Kernel and Ceph dependencies are supplied by the surrounding translation unit.

static mut lock_secret: u64 = 0;

#[inline]
unsafe fn secure_addr(addr: *mut core::ffi::c_void) -> u64 {
    let mut v = lock_secret ^ (addr as usize as u64);
    // Set the most significant bit, so that MDS knows the 'owner'
    // is sufficient to identify the owner of lock. (old code uses
    // both 'owner' and 'pid')
    v |= 1u64 << 63;
    v
}

pub unsafe fn ceph_flock_init() {
    get_random_bytes(
        &mut lock_secret as *mut u64 as *mut core::ffi::c_void,
        core::mem::size_of::<u64>(),
    );
}

unsafe fn ceph_fl_copy_lock(dst: *mut file_lock, _src: *mut file_lock) {
    let inode = file_inode((*dst).c.flc_file);
    atomic_inc(&mut (*ceph_inode(inode)).i_filelock_ref);
    (*dst).fl_u.ceph.inode = igrab(inode);
}

// Do not use the 'fl->fl_file' in release function, which is possibly already
// released by another thread.
unsafe fn ceph_fl_release_lock(fl: *mut file_lock) {
    let inode = (*fl).fl_u.ceph.inode;
    if inode.is_null() {
        return;
    }
    let ci = ceph_inode(inode);
    if atomic_dec_and_test(&mut (*ci).i_filelock_ref) {
        clear_bit(CEPH_I_ERROR_FILELOCK_BIT, &mut (*ci).i_ceph_flags);
    }
    (*fl).fl_u.ceph.inode = core::ptr::null_mut();
    iput(inode);
}

static ceph_fl_lock_ops: file_lock_operations = file_lock_operations {
    fl_copy_lock: Some(ceph_fl_copy_lock),
    fl_release_private: Some(ceph_fl_release_lock),
};

// Implement fcntl and flock locking functions.
unsafe fn ceph_lock_message(
    lock_type: u8,
    operation: u16,
    inode: *mut inode,
    cmd: i32,
    mut wait: u8,
    fl: *mut file_lock,
) -> i32 {
    let mdsc = ceph_sb_to_mdsc((*inode).i_sb);
    let cl = (*(*mdsc).fsc).client;
    let mut length: u64 = 0;
    let owner: u64;

    if operation == CEPH_MDS_OP_SETFILELOCK {
        (*fl).fl_ops = &ceph_fl_lock_ops;
        ((*(*fl).fl_ops).fl_copy_lock.unwrap())(fl, core::ptr::null_mut());
    }
    if operation != CEPH_MDS_OP_SETFILELOCK || cmd == CEPH_LOCK_UNLOCK {
        wait = 0;
    }

    let req = ceph_mdsc_create_request(mdsc, operation, USE_AUTH_MDS);
    if is_err(req) {
        return ptr_err(req);
    }
    (*req).r_inode = inode;
    ihold(inode);
    (*req).r_num_caps = 1;

    if LLONG_MAX == (*fl).fl_end {
        length = 0;
    } else {
        length = (*fl).fl_end - (*fl).fl_start + 1;
    }
    owner = secure_addr((*fl).c.flc_owner);
    doutc!(cl, "rule: %d, op: %d, owner: %llx, pid: %llu, start: %llu, length: %llu, wait: %d, type: %d\n",
        lock_type as i32, operation as i32, owner, (*fl).c.flc_pid as u64,
        (*fl).fl_start, length, wait, (*fl).c.flc_type);

    (*req).r_args.filelock_change.rule = lock_type;
    (*req).r_args.filelock_change.type_ = cmd;
    (*req).r_args.filelock_change.owner = cpu_to_le64(owner);
    (*req).r_args.filelock_change.pid = cpu_to_le64((*fl).c.flc_pid as u64);
    (*req).r_args.filelock_change.start = cpu_to_le64((*fl).fl_start);
    (*req).r_args.filelock_change.length = cpu_to_le64(length);
    (*req).r_args.filelock_change.wait = wait;

    let mut err = ceph_mdsc_submit_request(mdsc, inode, req);
    if err == 0 {
        err = ceph_mdsc_wait_request(
            mdsc, req,
            if wait != 0 { Some(ceph_lock_wait_for_completion) } else { None },
        );
    }
    if err == 0 && operation == CEPH_MDS_OP_GETFILELOCK {
        let reply = (*req).r_reply_info.filelock_reply;
        (*fl).c.flc_pid = -(le64_to_cpu((*reply).pid) as i64);
        (*fl).c.flc_type = if (*reply).type_ == CEPH_LOCK_SHARED { F_RDLCK }
            else if (*reply).type_ == CEPH_LOCK_EXCL { F_WRLCK } else { F_UNLCK };
        (*fl).fl_start = le64_to_cpu((*reply).start);
        length = le64_to_cpu((*reply).start) + le64_to_cpu((*reply).length);
        (*fl).fl_end = if length >= 1 { length - 1 } else { 0 };
    }
    ceph_mdsc_put_request(req);
    doutc!(cl, "rule: %d, op: %d, pid: %llu, start: %llu, length: %llu, wait: %d, type: %d, err code %d\n",
        lock_type as i32, operation as i32, (*fl).c.flc_pid as u64,
        (*fl).fl_start, length, wait, (*fl).c.flc_type, err);
    err
}

unsafe fn ceph_lock_wait_for_completion(
    mdsc: *mut ceph_mds_client,
    req: *mut ceph_mds_request,
) -> i32 {
    let cl = (*(*mdsc).fsc).client;
    let inode = (*req).r_inode;
    let mut lock_type: i32;
    bug_on!((*req).r_op != CEPH_MDS_OP_SETFILELOCK);
    if (*req).r_args.filelock_change.rule == CEPH_LOCK_FCNTL {
        lock_type = CEPH_LOCK_FCNTL_INTR;
    } else if (*req).r_args.filelock_change.rule == CEPH_LOCK_FLOCK {
        lock_type = CEPH_LOCK_FLOCK_INTR;
    } else { bug_on!(true); }
    bug_on!((*req).r_args.filelock_change.type_ == CEPH_LOCK_UNLOCK);

    let mut err = wait_for_completion_interruptible(&mut (*req).r_completion);
    if err == 0 { return 0; }
    doutc!(cl, "request %llu was interrupted\n", (*req).r_tid);
    mutex_lock(&mut (*mdsc).mutex);
    if test_bit(CEPH_MDS_R_GOT_RESULT, &(*req).r_req_flags) {
        err = 0;
    } else {
        mutex_lock(&mut (*req).r_fill_mutex);
        (*req).r_err = err;
        set_bit(CEPH_MDS_R_ABORTED, &mut (*req).r_req_flags);
        mutex_unlock(&mut (*req).r_fill_mutex);
        if (*req).r_session.is_null() { err = 0; }
    }
    mutex_unlock(&mut (*mdsc).mutex);
    if err == 0 { return 0; }
    let intr_req = ceph_mdsc_create_request(mdsc, CEPH_MDS_OP_SETFILELOCK, USE_AUTH_MDS);
    if is_err(intr_req) { return ptr_err(intr_req); }
    (*intr_req).r_inode = inode;
    ihold(inode);
    (*intr_req).r_num_caps = 1;
    (*intr_req).r_args.filelock_change = (*req).r_args.filelock_change;
    (*intr_req).r_args.filelock_change.rule = lock_type as u8;
    (*intr_req).r_args.filelock_change.type_ = CEPH_LOCK_UNLOCK;
    err = ceph_mdsc_do_request(mdsc, inode, intr_req);
    ceph_mdsc_put_request(intr_req);
    if err != 0 && err != -ERESTARTSYS { return err; }
    err = wait_for_completion_killable(&mut (*req).r_safe_completion);
    if err != 0 { return err; }
    0
}

unsafe fn try_unlock_file(file: *mut file, fl: *mut file_lock) -> i32 {
    let orig_flags = (*fl).c.flc_flags;
    (*fl).c.flc_flags |= FL_EXISTS;
    let mut err = locks_lock_file_wait(file, fl);
    (*fl).c.flc_flags = orig_flags;
    if err == -ENOENT {
        if orig_flags & FL_EXISTS == 0 { err = 0; }
        return err;
    }
    1
}

// The remaining exported lock entry points and encoding helpers retain the
// kernel's file-lock ABI and call the external kernel/Ceph operations directly.
pub unsafe fn ceph_lock(file: *mut file, cmd: i32, fl: *mut file_lock) -> i32 {
    let inode = file_inode(file);
    let ci = ceph_inode(inode);
    let mdsc = ceph_sb_to_mdsc((*inode).i_sb);
    let cl = ceph_inode_to_client(inode);
    if (*fl).c.flc_flags & FL_POSIX == 0 { return -ENOLCK; }
    if ceph_inode_is_shutdown(inode) { return -ESTALE; }
    doutc!(cl, "fl_owner: %p\n", (*fl).c.flc_owner);
    let mut op = if IS_GETLK(cmd) { CEPH_MDS_OP_GETFILELOCK } else { CEPH_MDS_OP_SETFILELOCK };
    let wait = if IS_SETLKW(cmd) { 1 } else { 0 };
    if test_bit(CEPH_I_ERROR_FILELOCK_BIT, &(*ci).i_ceph_flags) {
        if op == CEPH_MDS_OP_SETFILELOCK && lock_is_unlock(fl) { posix_lock_file(file, fl, core::ptr::null_mut()); }
        return -EIO;
    }
    if op == CEPH_MDS_OP_SETFILELOCK && !lock_is_unlock(fl) {
        let err = ceph_mdsc_wait_for_reset(mdsc); if err != 0 { return err; }
    }
    let lock_cmd = if lock_is_read(fl) { CEPH_LOCK_SHARED } else if lock_is_write(fl) { CEPH_LOCK_EXCL } else { CEPH_LOCK_UNLOCK };
    if op == CEPH_MDS_OP_SETFILELOCK && lock_is_unlock(fl) { let err = try_unlock_file(file, fl); if err <= 0 { return err; } }
    let mut err = ceph_lock_message(CEPH_LOCK_FCNTL, op, inode, lock_cmd, wait, fl);
    if err == 0 && op == CEPH_MDS_OP_SETFILELOCK && (*fl).c.flc_type != F_UNLCK {
        err = posix_lock_file(file, fl, core::ptr::null_mut());
        if err != 0 { ceph_lock_message(CEPH_LOCK_FCNTL, op, inode, CEPH_LOCK_UNLOCK, 0, fl); }
    }
    err
}

pub unsafe fn ceph_flock(file: *mut file, cmd: i32, fl: *mut file_lock) -> i32 {
    let inode = file_inode(file); let ci = ceph_inode(inode); let mdsc = ceph_sb_to_mdsc((*inode).i_sb);
    if (*fl).c.flc_flags & FL_FLOCK == 0 { return -ENOLCK; }
    if ceph_inode_is_shutdown(inode) { return -ESTALE; }
    if test_bit(CEPH_I_ERROR_FILELOCK_BIT, &(*ci).i_ceph_flags) { if lock_is_unlock(fl) { locks_lock_file_wait(file, fl); } return -EIO; }
    if !lock_is_unlock(fl) { let err = ceph_mdsc_wait_for_reset(mdsc); if err != 0 { return err; } }
    let wait = if IS_SETLKW(cmd) { 1 } else { 0 };
    let lock_cmd = if lock_is_read(fl) { CEPH_LOCK_SHARED } else if lock_is_write(fl) { CEPH_LOCK_EXCL } else { CEPH_LOCK_UNLOCK };
    if lock_is_unlock(fl) { let err = try_unlock_file(file, fl); if err <= 0 { return err; } }
    let mut err = ceph_lock_message(CEPH_LOCK_FLOCK, CEPH_MDS_OP_SETFILELOCK, inode, lock_cmd, wait, fl);
    if err == 0 && (*fl).c.flc_type != F_UNLCK { err = locks_lock_file_wait(file, fl); if err != 0 { ceph_lock_message(CEPH_LOCK_FLOCK, CEPH_MDS_OP_SETFILELOCK, inode, CEPH_LOCK_UNLOCK, 0, fl); } }
    err
}

// Lock-list traversal and pagelist encoding are kept as direct pointer-based
// translations; the lock context/list macros are provided by the kernel ABI.
pub unsafe fn ceph_count_locks(inode: *mut inode, fcntl_count: *mut i32, flock_count: *mut i32) {
    let cl = ceph_inode_to_client(inode); *fcntl_count = 0; *flock_count = 0;
    let ctx = locks_inode_context(inode);
    if !ctx.is_null() { spin_lock(&mut (*ctx).flc_lock); for_each_file_lock!(_lock, &(*ctx).flc_posix, { *fcntl_count += 1; }); for_each_file_lock!(_lock, &(*ctx).flc_flock, { *flock_count += 1; }); spin_unlock(&mut (*ctx).flc_lock); }
    doutc!(cl, "counted %d flock locks and %d fcntl locks\n", *flock_count, *fcntl_count);
}

unsafe fn lock_to_ceph_filelock(inode: *mut inode, lock: *mut file_lock, cephlock: *mut ceph_filelock) -> i32 {
    let cl = ceph_inode_to_client(inode); (*cephlock).start = cpu_to_le64((*lock).fl_start); (*cephlock).length = cpu_to_le64((*lock).fl_end - (*lock).fl_start + 1); (*cephlock).client = cpu_to_le64(0); (*cephlock).pid = cpu_to_le64((*lock).c.flc_pid as u64); (*cephlock).owner = cpu_to_le64(secure_addr((*lock).c.flc_owner));
    (*cephlock).type_ = match (*lock).c.flc_type { F_RDLCK => CEPH_LOCK_SHARED, F_WRLCK => CEPH_LOCK_EXCL, F_UNLCK => CEPH_LOCK_UNLOCK, _ => { doutc!(cl, "Have unknown lock type %d\n", (*lock).c.flc_type); return -EINVAL; } }; 0
}

pub unsafe fn ceph_encode_locks_to_buffer(inode: *mut inode, flocks: *mut ceph_filelock, num_fcntl_locks: i32, num_flock_locks: i32) -> i32 {
    let ctx = locks_inode_context(inode); if ctx.is_null() { return 0; }
    let mut seen_fcntl = 0; let mut seen_flock = 0; let mut l = 0; let mut err = 0;
    spin_lock(&mut (*ctx).flc_lock);
    for_each_file_lock!(_lock, &(*ctx).flc_posix, {
        seen_fcntl += 1;
        if seen_fcntl > num_fcntl_locks { err = -ENOSPC; }
        else { err = lock_to_ceph_filelock(inode, _lock, flocks.add(l as usize)); l += 1; }
    });
    if err == 0 { for_each_file_lock!(_lock, &(*ctx).flc_flock, {
        seen_flock += 1;
        if seen_flock > num_flock_locks { err = -ENOSPC; }
        else { err = lock_to_ceph_filelock(inode, _lock, flocks.add(l as usize)); l += 1; }
    }); }
    spin_unlock(&mut (*ctx).flc_lock); err
}

pub unsafe fn ceph_locks_to_pagelist(flocks: *mut ceph_filelock, pagelist: *mut ceph_pagelist, num_fcntl_locks: i32, num_flock_locks: i32) -> i32 {
    let mut nlocks = cpu_to_le32(num_fcntl_locks); let mut err = ceph_pagelist_append(pagelist, &mut nlocks as *mut _ as *mut core::ffi::c_void, core::mem::size_of::<u32>()); if err != 0 { return err; }
    if num_fcntl_locks > 0 { err = ceph_pagelist_append(pagelist, flocks as *mut core::ffi::c_void, num_fcntl_locks as usize * core::mem::size_of::<ceph_filelock>()); if err != 0 { return err; } }
    nlocks = cpu_to_le32(num_flock_locks); err = ceph_pagelist_append(pagelist, &mut nlocks as *mut _ as *mut core::ffi::c_void, core::mem::size_of::<u32>()); if err != 0 { return err; }
    if num_flock_locks > 0 { err = ceph_pagelist_append(pagelist, flocks.add(num_fcntl_locks as usize) as *mut core::ffi::c_void, num_flock_locks as usize * core::mem::size_of::<ceph_filelock>()); } err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
