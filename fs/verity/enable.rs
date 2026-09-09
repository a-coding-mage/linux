// SPDX-License-Identifier: GPL-2.0
/*
 * Ioctl to enable verity on a file
 *
 * Copyright 2019 Google LLC
 */

#[repr(C)]
struct block_buffer {
    filled: u32,
    is_root_hash: bool,
    data: *mut u8,
}

/* Hash a block, writing the result to the next level's pending block buffer. */
unsafe fn hash_one_block(params: *const merkle_tree_params, cur: *mut block_buffer) -> i32 {
    let next = cur.add(1);
    if unsafe { (*next).is_root_hash && (*next).filled != 0 } {
        if unsafe { WARN_ON_ONCE(true) } {
            return -EINVAL;
        }
    }
    unsafe {
        memset((*cur).data.add((*cur).filled as usize), 0,
               (*params).block_size as usize - (*cur).filled as usize);
        fsverity_hash_block(params, (*cur).data, (*next).data.add((*next).filled as usize));
        (*next).filled += (*params).digest_size;
        (*cur).filled = 0;
    }
    0
}

unsafe fn write_merkle_tree_block(file: *mut file, buf: *const u8, index: usize,
                                  params: *const merkle_tree_params) -> i32 {
    let inode = unsafe { file_inode(file) };
    let pos = (index as u64) << unsafe { (*params).log_blocksize };
    let err = unsafe {
        (*(*(*inode).i_sb).s_vop).write_merkle_tree_block(file, buf, pos,
                                                          (*params).block_size)
    };
    if err != 0 {
        unsafe { fsverity_err(inode, "Error %d writing Merkle tree block %lu", err, index); }
    }
    err
}

unsafe fn build_merkle_tree(filp: *mut file, params: *const merkle_tree_params,
                            root_hash: *mut u8) -> i32 {
    let inode = unsafe { file_inode(filp) };
    let data_size = unsafe { (*inode).i_size };
    let num_levels = unsafe { (*params).num_levels };
    let mut buffers = [block_buffer { filled: 0, is_root_hash: false, data: core::ptr::null_mut() }; 1 + FS_VERITY_MAX_LEVELS + 1];
    let base = unsafe { buffers.as_mut_ptr().add(1) };
    let mut level_offset = [0usize; FS_VERITY_MAX_LEVELS];
    let mut level: i32;
    let mut offset: u64;
    let mut err: i32;

    if data_size == 0 {
        unsafe { memset(root_hash, 0, (*params).digest_size as usize); }
        return 0;
    }
    level = -1;
    while level < num_levels {
        unsafe { (*base.offset(level as isize)).data = kzalloc((*params).block_size as usize, GFP_KERNEL); }
        if unsafe { (*base.offset(level as isize)).data.is_null() } { err = -ENOMEM; goto_out!(out, err); }
        level += 1;
    }
    unsafe { (*base.add(num_levels as usize)).data = root_hash; (*base.add(num_levels as usize)).is_root_hash = true; }
    unsafe { memcpy(level_offset.as_mut_ptr(), (*params).level_start, core::mem::size_of_val(&level_offset)); }

    offset = 0;
    while offset < data_size {
        let mut pos = offset as i64;
        let cur = unsafe { &mut *base.offset(-1) };
        cur.filled = core::cmp::min(unsafe { (*params).block_size as u64 }, data_size - offset) as u32;
        let bytes_read = unsafe { __kernel_read(filp, cur.data, cur.filled as usize, &mut pos) };
        if bytes_read < 0 { err = bytes_read as i32; unsafe { fsverity_err(inode, "Error %d reading file data", err); } goto_out!(out, err); }
        if bytes_read as u32 != cur.filled { err = -EINVAL; unsafe { fsverity_err(inode, "Short read of file data"); } goto_out!(out, err); }
        err = hash_one_block(params, base.offset(-1)); if err != 0 { goto_out!(out, err); }
        level = 0;
        while level < num_levels {
            let b = unsafe { &mut *base.offset(level as isize) };
            if b.filled + unsafe { (*params).digest_size } <= unsafe { (*params).block_size } { break; }
            err = hash_one_block(params, b); if err != 0 { goto_out!(out, err); }
            err = write_merkle_tree_block(filp, b.data, level_offset[level as usize], params); if err != 0 { goto_out!(out, err); }
            level_offset[level as usize] += 1;
            level += 1;
        }
        if unsafe { fatal_signal_pending(current) } { err = -EINTR; goto_out!(out, err); }
        unsafe { cond_resched(); }
        offset += unsafe { (*params).block_size as u64 };
    }
    level = 0;
    while level < num_levels {
        let b = unsafe { &mut *base.offset(level as isize) };
        if b.filled != 0 {
            err = hash_one_block(params, b); if err != 0 { goto_out!(out, err); }
            err = write_merkle_tree_block(filp, b.data, level_offset[level as usize], params); if err != 0 { goto_out!(out, err); }
        }
        level += 1;
    }
    if unsafe { (*base.add(num_levels as usize)).filled != (*params).digest_size } { err = -EINVAL; goto_out!(out, err); }
    err = 0;
out:
    level = -1;
    while level < num_levels { unsafe { kfree((*base.offset(level as isize)).data as *mut core::ffi::c_void); } level += 1; }
    err
}

/* The remaining ioctl implementation is a direct translation of the C entry point. */
unsafe fn enable_verity(filp: *mut file, arg: *const fsverity_enable_arg) -> i32 {
    let inode = file_inode(filp);
    let vops = (*(*inode).i_sb).s_vop;
    let mut params = core::mem::zeroed::<merkle_tree_params>();
    let desc_size = struct_size_signature((*arg).sig_size);
    let desc = kzalloc(desc_size, GFP_KERNEL) as *mut fsverity_descriptor;
    if desc.is_null() { return -ENOMEM; }
    (*desc).version = 1; (*desc).hash_algorithm = (*arg).hash_algorithm;
    (*desc).log_blocksize = ilog2((*arg).block_size);
    let mut err = 0;
    if (*arg).salt_size != 0 && copy_from_user((*desc).salt, u64_to_user_ptr((*arg).salt_ptr), (*arg).salt_size) != 0 { err = -EFAULT; goto out; }
    (*desc).salt_size = (*arg).salt_size;
    if (*arg).sig_size != 0 && copy_from_user((*desc).signature, u64_to_user_ptr((*arg).sig_ptr), (*arg).sig_size) != 0 { err = -EFAULT; goto out; }
    (*desc).sig_size = cpu_to_le32((*arg).sig_size); (*desc).data_size = cpu_to_le64((*inode).i_size);
    err = fsverity_init_merkle_tree_params(&mut params, inode, (*arg).hash_algorithm, (*desc).log_blocksize, (*desc).salt, (*desc).salt_size); if err != 0 { goto out; }
    trace_fsverity_enable(inode, &params);
    inode_lock(inode); err = if IS_VERITY(inode) { -EEXIST } else { (*vops).begin_enable_verity(filp) }; inode_unlock(inode); if err != 0 { goto out; }
    err = build_merkle_tree(filp, &params, (*desc).root_hash); if err != 0 { goto rollback; }
    let vi = fsverity_create_info(inode, desc); if IS_ERR(vi) { err = PTR_ERR(vi); goto rollback; }
    trace_fsverity_tree_done(inode, vi, &params); err = fsverity_set_info(vi); if err != 0 { fsverity_free_info(vi); goto rollback; }
    inode_lock(inode); err = (*vops).end_enable_verity(filp, desc, desc_size, params.tree_size); inode_unlock(inode);
    if err != 0 { fsverity_remove_info(vi); } else if WARN_ON_ONCE(!IS_VERITY(inode)) { fsverity_remove_info(vi); err = -EINVAL; }
    goto out;
rollback:
    inode_lock(inode); (*vops).end_enable_verity(filp, core::ptr::null_mut(), 0, params.tree_size); inode_unlock(inode);
out:
    kfree(params.hashstate as *mut core::ffi::c_void); kfree(desc as *mut core::ffi::c_void); err
}

pub unsafe fn fsverity_ioctl_enable(filp: *mut file, uarg: *const core::ffi::c_void) -> i32 {
    let inode = file_inode(filp); let mut arg = core::mem::zeroed::<fsverity_enable_arg>();
    if copy_from_user(&mut arg, uarg, core::mem::size_of_val(&arg)) != 0 { return -EFAULT; }
    if arg.version != 1 || arg.__reserved1 != 0 || memchr_inv(arg.__reserved2.as_ptr(), 0, core::mem::size_of_val(&arg.__reserved2)).is_some() { return -EINVAL; }
    if !is_power_of_2(arg.block_size) { return -EINVAL; }
    if arg.salt_size > sizeof_field_salt() || arg.sig_size > FS_VERITY_MAX_SIGNATURE_SIZE { return -EMSGSIZE; }
    let mut err = file_permission(filp, MAY_WRITE); if err != 0 { return err; }
    if (*filp).f_mode & FMODE_READ == 0 { return -EBADF; }
    if IS_APPEND(inode) { return -EPERM; } if S_ISDIR((*inode).i_mode) { return -EISDIR; } if !S_ISREG((*inode).i_mode) { return -EINVAL; }
    err = mnt_want_write_file(filp); if err != 0 { return err; }
    err = deny_write_access(filp); if err != 0 { mnt_drop_write_file(filp); return err; }
    err = enable_verity(filp, &arg); allow_write_access(filp); mnt_drop_write_file(filp); err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
