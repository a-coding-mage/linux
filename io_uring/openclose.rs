// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct io_open {
    pub file: *mut file,
    pub dfd: i32,
    pub file_slot: u32,
    pub filename: delayed_filename,
    pub how: open_how,
    pub nofile: usize,
}

#[repr(C)]
pub struct io_close {
    pub file: *mut file,
    pub fd: i32,
    pub file_slot: u32,
}

#[repr(C)]
pub struct io_fixed_install {
    pub file: *mut file,
    pub o_flags: u32,
}

unsafe fn io_openat_force_async(open: *mut io_open) -> bool {
    (*open).how.flags & (O_TRUNC | O_CREAT | __O_TMPFILE) != 0
}

unsafe fn __io_openat_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let open = io_kiocb_to_cmd::<io_open>(req);
    let mut fname: *const u8;
    let ret: i32;
    if (*sqe).buf_index != 0 { return -EINVAL; }
    if (*req).flags & REQ_F_FIXED_FILE != 0 { return -EBADF; }
    if (*open).how.flags & O_PATH == 0 && force_o_largefile() {
        (*open).how.flags |= O_LARGEFILE;
    }
    (*open).dfd = READ_ONCE((*sqe).fd);
    fname = u64_to_user_ptr(READ_ONCE((*sqe).addr));
    ret = delayed_getname(&mut (*open).filename, fname);
    if ret != 0 { return ret; }
    (*req).flags |= REQ_F_NEED_CLEANUP;
    (*open).file_slot = READ_ONCE((*sqe).file_index);
    if (*open).file_slot != 0 && (*open).how.flags & O_CLOEXEC != 0 { return -EINVAL; }
    (*open).nofile = rlimit(RLIMIT_NOFILE);
    if io_openat_force_async(open) { (*req).flags |= REQ_F_FORCE_ASYNC; }
    0
}

pub unsafe fn io_openat_bpf_populate(bctx: *mut io_uring_bpf_ctx, req: *mut io_kiocb) {
    let open = io_kiocb_to_cmd::<io_open>(req);
    (*bctx).open.flags = (*open).how.flags;
    (*bctx).open.mode = (*open).how.mode;
    (*bctx).open.resolve = (*open).how.resolve;
}

pub unsafe fn io_openat_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let open = io_kiocb_to_cmd::<io_open>(req);
    (*open).how = build_open_how(READ_ONCE((*sqe).open_flags), READ_ONCE((*sqe).len));
    __io_openat_prep(req, sqe)
}

pub unsafe fn io_openat2_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let open = io_kiocb_to_cmd::<io_open>(req);
    let how: *mut open_how = u64_to_user_ptr(READ_ONCE((*sqe).addr2));
    let len = READ_ONCE((*sqe).len) as usize;
    if len < OPEN_HOW_SIZE_VER0 { return -EINVAL; }
    let ret = copy_struct_from_user(&mut (*open).how, core::mem::size_of::<open_how>(), how, len);
    if ret != 0 { return ret; }
    __io_openat_prep(req, sqe)
}

pub unsafe fn io_openat2(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let open = io_kiocb_to_cmd::<io_open>(req);
    let mut op: open_flags = core::mem::zeroed();
    let mut file: *mut file;
    let fixed = (*open).file_slot != 0;
    let mut name = filename_complete_delayed(&mut (*open).filename);
    let mut ret = build_open_flags(&(*open).how, &mut op);
    if ret != 0 { goto_err(req, ret); return IOU_COMPLETE; }
    let nonblock_set = op.open_flag & O_NONBLOCK != 0;
    let resolve_nonblock = (*open).how.resolve & RESOLVE_CACHED != 0;
    if issue_flags & IO_URING_F_NONBLOCK != 0 {
        WARN_ON_ONCE(io_openat_force_async(open));
        op.lookup_flags |= LOOKUP_CACHED;
        op.open_flag |= O_NONBLOCK;
    }
    if !fixed {
        ret = __get_unused_fd_flags((*open).how.flags, (*open).nofile);
        if ret < 0 { goto_err(req, ret); return IOU_COMPLETE; }
    }
    file = do_file_open((*open).dfd, name, &op);
    if IS_ERR(file) {
        if !fixed { put_unused_fd(ret); }
        ret = PTR_ERR(file);
        if ret == -EAGAIN && !resolve_nonblock && issue_flags & IO_URING_F_NONBLOCK != 0 {
            ret = putname_to_delayed(&mut (*open).filename, no_free_ptr(name));
            if ret == 0 { return -EAGAIN; }
        }
        goto_err(req, ret); return IOU_COMPLETE;
    }
    if issue_flags & IO_URING_F_NONBLOCK != 0 && !nonblock_set { (*file).f_flags &= !O_NONBLOCK; }
    if !fixed { fd_install(ret, file); }
    else { ret = io_fixed_fd_install(req, issue_flags, file, (*open).file_slot); }
    goto_err(req, ret); IOU_COMPLETE
}

pub unsafe fn io_openat(req: *mut io_kiocb, issue_flags: u32) -> i32 { io_openat2(req, issue_flags) }

pub unsafe fn io_open_cleanup(req: *mut io_kiocb) {
    let open = io_kiocb_to_cmd::<io_open>(req);
    dismiss_delayed_filename(&mut (*open).filename);
}

pub unsafe fn __io_close_fixed(ctx: *mut io_ring_ctx, issue_flags: u32, offset: u32) -> i32 {
    io_ring_submit_lock(ctx, issue_flags);
    let ret = io_fixed_fd_remove(ctx, offset);
    io_ring_submit_unlock(ctx, issue_flags);
    ret
}

pub unsafe fn io_close_fixed(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let close = io_kiocb_to_cmd::<io_close>(req);
    __io_close_fixed((*req).ctx, issue_flags, (*close).file_slot - 1)
}

pub unsafe fn io_close_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let close = io_kiocb_to_cmd::<io_close>(req);
    if (*sqe).off != 0 || (*sqe).addr != 0 || (*sqe).len != 0 || (*sqe).rw_flags != 0 || (*sqe).buf_index != 0 { return -EINVAL; }
    if (*req).flags & REQ_F_FIXED_FILE != 0 { return -EBADF; }
    (*close).fd = READ_ONCE((*sqe).fd);
    (*close).file_slot = READ_ONCE((*sqe).file_index);
    if (*close).file_slot != 0 && (*close).fd != 0 { return -EINVAL; }
    0
}

pub unsafe fn io_close(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let files = (*current()).files;
    let close = io_kiocb_to_cmd::<io_close>(req);
    let mut ret = -EBADF;
    if (*close).file_slot != 0 { ret = io_close_fixed(req, issue_flags); goto_close_err(req, ret); return IOU_COMPLETE; }
    spin_lock(&mut (*files).file_lock);
    let mut file = files_lookup_fd_locked(files, (*close).fd);
    if file.is_null() || io_is_uring_fops(file) { spin_unlock(&mut (*files).file_lock); goto_close_err(req, ret); return IOU_COMPLETE; }
    if !(*file).f_op.is_null() && !(*(*file).f_op).flush.is_none() && issue_flags & IO_URING_F_NONBLOCK != 0 { spin_unlock(&mut (*files).file_lock); return -EAGAIN; }
    file = file_close_fd_locked(files, (*close).fd);
    spin_unlock(&mut (*files).file_lock);
    if file.is_null() { goto_close_err(req, ret); return IOU_COMPLETE; }
    ret = filp_close(file, files);
    goto_close_err(req, ret); IOU_COMPLETE
}

pub unsafe fn io_install_fixed_fd_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    if (*sqe).off != 0 || (*sqe).addr != 0 || (*sqe).len != 0 || (*sqe).buf_index != 0 || (*sqe).splice_fd_in != 0 || (*sqe).addr3 != 0 { return -EINVAL; }
    if (*req).flags & REQ_F_FIXED_FILE == 0 { return -EBADF; }
    let flags = READ_ONCE((*sqe).install_fd_flags);
    if flags & !IORING_FIXED_FD_NO_CLOEXEC != 0 { return -EINVAL; }
    if (*req).flags & REQ_F_CREDS != 0 { return -EPERM; }
    let ifi = io_kiocb_to_cmd::<io_fixed_install>(req);
    (*ifi).o_flags = if flags & IORING_FIXED_FD_NO_CLOEXEC != 0 { 0 } else { O_CLOEXEC };
    0
}

pub unsafe fn io_install_fixed_fd(req: *mut io_kiocb, _issue_flags: u32) -> i32 {
    let ifi = io_kiocb_to_cmd::<io_fixed_install>(req);
    let ret = receive_fd((*req).file, core::ptr::null_mut(), (*ifi).o_flags);
    if ret < 0 { req_set_fail(req); }
    io_req_set_res(req, ret, 0); IOU_COMPLETE
}

#[repr(C)]
pub struct io_pipe { pub file: *mut file, pub fds: *mut i32, pub flags: i32, pub file_slot: i32, pub nofile: usize }

pub unsafe fn io_pipe_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> i32 {
    let p = io_kiocb_to_cmd::<io_pipe>(req);
    if (*sqe).fd != 0 || (*sqe).off != 0 || (*sqe).addr3 != 0 { return -EINVAL; }
    (*p).fds = u64_to_user_ptr(READ_ONCE((*sqe).addr));
    (*p).flags = READ_ONCE((*sqe).pipe_flags);
    if (*p).flags & !(O_CLOEXEC | O_NONBLOCK | O_DIRECT | O_NOTIFICATION_PIPE) != 0 { return -EINVAL; }
    (*p).file_slot = READ_ONCE((*sqe).file_index) as i32;
    (*p).nofile = rlimit(RLIMIT_NOFILE); 0
}

unsafe fn io_pipe_fixed(req: *mut io_kiocb, files: &mut [*mut file; 2], issue_flags: u32) -> i32 {
    let p = io_kiocb_to_cmd::<io_pipe>(req);
    let ctx = (*req).ctx;
    let mut fds = [-1i32, -1i32];
    let mut slot = (*p).file_slot;
    if (*p).flags & O_CLOEXEC != 0 { return -EINVAL; }
    let alloc_slot = slot == IORING_FILE_INDEX_ALLOC as i32;
    io_ring_submit_lock(ctx, issue_flags);
    let mut ret = __io_fixed_fd_install(ctx, files[0], slot as u32);
    if ret < 0 { io_ring_submit_unlock(ctx, issue_flags); return ret; }
    fds[0] = if alloc_slot { ret } else { slot - 1 }; files[0] = core::ptr::null_mut();
    if !alloc_slot { slot += 1; }
    ret = __io_fixed_fd_install(ctx, files[1], slot as u32);
    if ret < 0 { goto_pipe_err(ctx, issue_flags, &fds); return ret; }
    fds[1] = if alloc_slot { ret } else { slot - 1 }; files[1] = core::ptr::null_mut();
    io_ring_submit_unlock(ctx, issue_flags);
    if copy_to_user((*p).fds, fds.as_ptr(), core::mem::size_of_val(&fds)) == 0 { return 0; }
    io_ring_submit_lock(ctx, issue_flags); goto_pipe_err(ctx, issue_flags, &fds); -EFAULT
}

unsafe fn goto_pipe_err(ctx: *mut io_ring_ctx, issue_flags: u32, fds: &[i32; 2]) {
    if fds[0] != -1 { io_fixed_fd_remove(ctx, fds[0] as u32); }
    if fds[1] != -1 { io_fixed_fd_remove(ctx, fds[1] as u32); }
    io_ring_submit_unlock(ctx, issue_flags);
}

unsafe fn io_pipe_fd(req: *mut io_kiocb, files: &mut [*mut file; 2]) -> i32 {
    let p = io_kiocb_to_cmd::<io_pipe>(req);
    let mut fds = [-1i32, -1i32];
    let mut ret = __get_unused_fd_flags((*p).flags as u64, (*p).nofile);
    if ret < 0 { goto_pipe_fd_err(&fds); return ret; }
    fds[0] = ret;
    ret = __get_unused_fd_flags((*p).flags as u64, (*p).nofile);
    if ret < 0 { goto_pipe_fd_err(&fds); return ret; }
    fds[1] = ret;
    if copy_to_user((*p).fds, fds.as_ptr(), core::mem::size_of_val(&fds)) == 0 {
        fd_install(fds[0], files[0]); fd_install(fds[1], files[1]); return 0;
    }
    goto_pipe_fd_err(&fds); -EFAULT
}

unsafe fn goto_pipe_fd_err(fds: &[i32; 2]) {
    if fds[0] != -1 { put_unused_fd(fds[0]); }
    if fds[1] != -1 { put_unused_fd(fds[1]); }
}

pub unsafe fn io_pipe(req: *mut io_kiocb, issue_flags: u32) -> i32 {
    let p = io_kiocb_to_cmd::<io_pipe>(req);
    let mut files = [core::ptr::null_mut(); 2];
    let mut ret = create_pipe_files(files.as_mut_ptr(), (*p).flags);
    if ret != 0 { return ret; }
    if (*p).file_slot != 0 { ret = io_pipe_fixed(req, &mut files, issue_flags); }
    else { ret = io_pipe_fd(req, &mut files); }
    io_req_set_res(req, ret, 0);
    if ret == 0 { return IOU_COMPLETE; }
    req_set_fail(req);
    if !files[0].is_null() { fput(files[0]); }
    if !files[1].is_null() { fput(files[1]); }
    ret
}

unsafe fn goto_err(req: *mut io_kiocb, ret: i32) {
    (*req).flags &= !REQ_F_NEED_CLEANUP;
    if ret < 0 { req_set_fail(req); }
    io_req_set_res(req, ret, 0);
}

unsafe fn goto_close_err(req: *mut io_kiocb, ret: i32) {
    if ret < 0 { req_set_fail(req); }
    io_req_set_res(req, ret, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
