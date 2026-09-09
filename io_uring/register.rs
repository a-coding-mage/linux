// SPDX-License-Identifier: GPL-2.0
/* Code related to the io_uring_register() syscall. */

// Kernel and local header dependencies are supplied by the surrounding tree.

const IORING_MAX_RESTRICTIONS: usize = IORING_RESTRICTION_LAST as usize
    + IORING_REGISTER_LAST as usize + IORING_OP_LAST as usize;

unsafe fn io_probe(ctx: *mut io_ring_ctx, arg: *mut core::ffi::c_void, mut nr_args: u32) -> i32 {
    let mut p: *mut io_uring_probe;
    let size: usize;
    let mut i: i32 = 0;
    let mut ret: i32;
    if nr_args > IORING_OP_LAST { nr_args = IORING_OP_LAST; }
    size = struct_size_probe(nr_args);
    p = memdup_user(arg, size);
    if is_err(p) { return ptr_err(p); }
    ret = -EINVAL;
    if memchr_inv(p as *const _, 0, size).is_null() == false { goto_out!(out); }
    (*p).last_op = IORING_OP_LAST - 1;
    while i < nr_args as i32 {
        (*p).ops.add(i as usize).as_mut().unwrap().op = i as u8;
        if io_uring_op_supported(i as u32) { (*p).ops.add(i as usize).as_mut().unwrap().flags = IO_URING_OP_SUPPORTED; }
        i += 1;
    }
    (*p).ops_len = i as u8;
    ret = 0;
    if copy_to_user(arg, p as *const _, size) != 0 { ret = -EFAULT; }
out:
    kfree(p as *mut _);
    ret
}

pub unsafe fn io_unregister_personality(ctx: *mut io_ring_ctx, id: u32) -> i32 {
    let creds = xa_erase(&mut (*ctx).personalities, id);
    if !creds.is_null() { put_cred(creds); return 0; }
    -EINVAL
}

unsafe fn io_register_personality(ctx: *mut io_ring_ctx) -> i32 {
    let creds = get_current_cred();
    let mut id: u32 = 0;
    let ret = xa_alloc_cyclic(&mut (*ctx).personalities, &mut id, creds as *mut _, XA_LIMIT(0, USHRT_MAX), &mut (*ctx).pers_next, GFP_KERNEL);
    if ret < 0 { put_cred(creds); return ret; }
    id as i32
}

unsafe fn io_parse_restrictions(arg: *mut core::ffi::c_void, nr_args: u32, restrictions: *mut io_restriction) -> i32 {
    if arg.is_null() || nr_args as usize > IORING_MAX_RESTRICTIONS { return -EINVAL; }
    let size = array_size(nr_args as usize, core::mem::size_of::<io_uring_restriction>());
    if size == usize::MAX { return -EOVERFLOW; }
    let res = memdup_user(arg, size) as *mut io_uring_restriction;
    if is_err(res) { return ptr_err(res as *mut _); }
    let mut ret = -EINVAL;
    for i in 0..nr_args as usize {
        let r = &*res.add(i);
        match r.opcode {
            IORING_RESTRICTION_REGISTER_OP => { if r.register_op >= IORING_REGISTER_LAST { break; } set_bit(r.register_op, (*restrictions).register_op.as_mut_ptr()); (*restrictions).reg_registered = true; }
            IORING_RESTRICTION_SQE_OP => { if r.sqe_op >= IORING_OP_LAST { break; } set_bit(r.sqe_op, (*restrictions).sqe_op.as_mut_ptr()); (*restrictions).op_registered = true; }
            IORING_RESTRICTION_SQE_FLAGS_ALLOWED => { (*restrictions).sqe_flags_allowed = r.sqe_flags; (*restrictions).op_registered = true; }
            IORING_RESTRICTION_SQE_FLAGS_REQUIRED => { (*restrictions).sqe_flags_required = r.sqe_flags; (*restrictions).op_registered = true; }
            _ => break,
        }
        ret = (i + 1) as i32;
    }
    if ret == nr_args as i32 && nr_args == 0 { (*restrictions).op_registered = true; (*restrictions).reg_registered = true; }
    kfree(res as *mut _); ret
}

unsafe fn io_register_restrictions(ctx: *mut io_ring_ctx, arg: *mut core::ffi::c_void, nr_args: u32) -> i32 {
    if (*ctx).flags & IORING_SETUP_R_DISABLED == 0 { return -EBADFD; }
    if (*ctx).restrictions.op_registered || (*ctx).restrictions.reg_registered { return -EBUSY; }
    let ret = io_parse_restrictions(arg, nr_args, &mut (*ctx).restrictions);
    if ret < 0 { let bpf = (*ctx).restrictions.bpf_filters; let cowed = (*ctx).restrictions.bpf_filters_cow; core::ptr::write_bytes(&mut (*ctx).restrictions, 0, 1); (*ctx).restrictions.bpf_filters = bpf; (*ctx).restrictions.bpf_filters_cow = cowed; return ret; }
    if (*ctx).restrictions.op_registered { (*ctx).int_flags |= IO_RING_F_OP_RESTRICTED; }
    if (*ctx).restrictions.reg_registered { (*ctx).int_flags |= IO_RING_F_REG_RESTRICTED; }
    0
}

unsafe fn io_register_enable_rings(ctx: *mut io_ring_ctx) -> i32 {
    if (*ctx).flags & IORING_SETUP_R_DISABLED == 0 { return -EBADFD; }
    if (*ctx).flags & IORING_SETUP_SINGLE_ISSUER != 0 { (*ctx).submitter_task = get_task_struct(current); if wq_has_sleeper(&(*ctx).poll_wq) { io_activate_pollwq(ctx); } }
    smp_store_release(&mut (*ctx).flags, (*ctx).flags & !IORING_SETUP_R_DISABLED);
    if !(*ctx).sq_data.is_null() && wq_has_sleeper(&(*(*ctx).sq_data).wait) { wake_up(&mut (*(*ctx).sq_data).wait); }
    0
}

// The remaining registration helpers retain the kernel ABI and dispatch structure.
// Their implementations are expressed with raw pointers and external kernel symbols.
unsafe fn __io_uring_register(ctx: *mut io_ring_ctx, mut opcode: u32, arg: *mut core::ffi::c_void, nr_args: u32) -> i32 {
    if percpu_ref_is_dying(&(*ctx).refs) { return -ENXIO; }
    if !(*ctx).submitter_task.is_null() && (*ctx).submitter_task != current { return -EEXIST; }
    if (*ctx).int_flags & IO_RING_F_REG_RESTRICTED != 0 && (*ctx).flags & IORING_SETUP_R_DISABLED == 0 { opcode = array_index_nospec(opcode, IORING_REGISTER_LAST); if !test_bit(opcode, (*ctx).restrictions.register_op.as_ptr()) { return -EACCES; } }
    match opcode {
        IORING_REGISTER_BUFFERS => if arg.is_null() { -EFAULT } else { io_sqe_buffers_register(ctx, arg, nr_args, core::ptr::null_mut()) },
        IORING_UNREGISTER_BUFFERS => if !arg.is_null() || nr_args != 0 { -EINVAL } else { io_sqe_buffers_unregister(ctx) },
        IORING_REGISTER_FILES => if arg.is_null() { -EFAULT } else { io_sqe_files_register(ctx, arg, nr_args, core::ptr::null_mut()) },
        IORING_UNREGISTER_FILES => if !arg.is_null() || nr_args != 0 { -EINVAL } else { io_sqe_files_unregister(ctx) },
        IORING_REGISTER_FILES_UPDATE => io_register_files_update(ctx, arg, nr_args),
        IORING_REGISTER_EVENTFD => if nr_args != 1 { -EINVAL } else { io_eventfd_register(ctx, arg, 0) },
        IORING_REGISTER_EVENTFD_ASYNC => if nr_args != 1 { -EINVAL } else { io_eventfd_register(ctx, arg, 1) },
        IORING_UNREGISTER_EVENTFD => if !arg.is_null() || nr_args != 0 { -EINVAL } else { io_eventfd_unregister(ctx) },
        IORING_REGISTER_PROBE => if arg.is_null() || nr_args > 256 { -EINVAL } else { io_probe(ctx, arg, nr_args) },
        IORING_REGISTER_PERSONALITY => if !arg.is_null() || nr_args != 0 { -EINVAL } else { io_register_personality(ctx) },
        IORING_UNREGISTER_PERSONALITY => if !arg.is_null() { -EINVAL } else { io_unregister_personality(ctx, nr_args) },
        IORING_REGISTER_ENABLE_RINGS => if !arg.is_null() || nr_args != 0 { -EINVAL } else { io_register_enable_rings(ctx) },
        IORING_REGISTER_RESTRICTIONS => io_register_restrictions(ctx, arg, nr_args),
        IORING_REGISTER_FILES2 => io_register_rsrc(ctx, arg, nr_args, IORING_RSRC_FILE),
        IORING_REGISTER_FILES_UPDATE2 => io_register_rsrc_update(ctx, arg, nr_args, IORING_RSRC_FILE),
        IORING_REGISTER_BUFFERS2 => io_register_rsrc(ctx, arg, nr_args, IORING_RSRC_BUFFER),
        IORING_REGISTER_BUFFERS_UPDATE => io_register_rsrc_update(ctx, arg, nr_args, IORING_RSRC_BUFFER),
        IORING_REGISTER_IOWQ_AFF => if arg.is_null() || nr_args == 0 { -EINVAL } else { io_register_iowq_aff(ctx, arg, nr_args) },
        IORING_UNREGISTER_IOWQ_AFF => if !arg.is_null() || nr_args != 0 { -EINVAL } else { io_unregister_iowq_aff(ctx) },
        IORING_REGISTER_IOWQ_MAX_WORKERS => if arg.is_null() || nr_args != 2 { -EINVAL } else { io_register_iowq_max_workers(ctx, arg) },
        IORING_REGISTER_RING_FDS => io_ringfd_register(ctx, arg, nr_args),
        IORING_UNREGISTER_RING_FDS => io_ringfd_unregister(ctx, arg, nr_args),
        IORING_REGISTER_PBUF_RING => if arg.is_null() || nr_args != 1 { -EINVAL } else { io_register_pbuf_ring(ctx, arg) },
        IORING_UNREGISTER_PBUF_RING => if arg.is_null() || nr_args != 1 { -EINVAL } else { io_unregister_pbuf_ring(ctx, arg) },
        IORING_REGISTER_SYNC_CANCEL => if arg.is_null() || nr_args != 1 { -EINVAL } else { io_sync_cancel(ctx, arg) },
        IORING_REGISTER_FILE_ALLOC_RANGE => if arg.is_null() || nr_args != 0 { -EINVAL } else { io_register_file_alloc_range(ctx, arg) },
        IORING_REGISTER_PBUF_STATUS => if arg.is_null() || nr_args != 1 { -EINVAL } else { io_register_pbuf_status(ctx, arg) },
        IORING_REGISTER_NAPI => if arg.is_null() || nr_args != 1 { -EINVAL } else { io_register_napi(ctx, arg) },
        IORING_UNREGISTER_NAPI => if nr_args != 1 { -EINVAL } else { io_unregister_napi(ctx, arg) },
        IORING_REGISTER_CLOCK => if arg.is_null() || nr_args != 0 { -EINVAL } else { io_register_clock(ctx, arg) },
        IORING_REGISTER_CLONE_BUFFERS => if arg.is_null() || nr_args != 1 { -EINVAL } else { io_register_clone_buffers(ctx, arg) },
        IORING_REGISTER_ZCRX_IFQ => if arg.is_null() || nr_args != 1 { -EINVAL } else { io_register_zcrx(ctx, arg) },
        IORING_REGISTER_RESIZE_RINGS => if arg.is_null() || nr_args != 1 { -EINVAL } else { io_register_resize_rings(ctx, arg) },
        IORING_REGISTER_MEM_REGION => if arg.is_null() || nr_args != 1 { -EINVAL } else { io_register_mem_region(ctx, arg) },
        IORING_REGISTER_QUERY => io_query(arg, nr_args),
        IORING_REGISTER_ZCRX_CTRL => io_zcrx_ctrl(ctx, arg, nr_args),
        IORING_REGISTER_BPF_FILTER => if nr_args != 1 { -EINVAL } else { io_register_bpf_filter(&mut (*ctx).restrictions, arg) },
        _ => -EINVAL,
    }
}

unsafe fn io_uring_register_blind(opcode: u32, arg: *mut core::ffi::c_void, nr_args: u32) -> i32 {
    match opcode { IORING_REGISTER_SEND_MSG_RING => io_uring_register_send_msg_ring(arg, nr_args), IORING_REGISTER_QUERY => io_query(arg, nr_args), IORING_REGISTER_RESTRICTIONS => io_register_restrictions_task(arg, nr_args), IORING_REGISTER_BPF_FILTER => io_register_bpf_filter_task(arg, nr_args), _ => -EINVAL }
}

pub unsafe fn io_uring_register(fd: u32, mut opcode: u32, arg: *mut core::ffi::c_void, nr_args: u32) -> i64 {
    let use_registered_ring = opcode & IORING_REGISTER_USE_REGISTERED_RING != 0;
    opcode &= !IORING_REGISTER_USE_REGISTERED_RING;
    if opcode >= IORING_REGISTER_LAST { return -EINVAL as i64; }
    if fd == u32::MAX { return io_uring_register_blind(opcode, arg, nr_args) as i64; }
    let file = io_uring_ctx_get_file(fd, use_registered_ring);
    if is_err(file) { return ptr_err(file) as i64; }
    let ctx = (*file).private_data as *mut io_ring_ctx;
    mutex_lock(&mut (*ctx).uring_lock);
    let ret = __io_uring_register(ctx, opcode, arg, nr_args);
    trace_io_uring_register(ctx, opcode, (*ctx).file_table.data.nr, (*ctx).buf_table.nr, ret);
    mutex_unlock(&mut (*ctx).uring_lock);
    if !use_registered_ring { fput(file); }
    ret as i64
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
