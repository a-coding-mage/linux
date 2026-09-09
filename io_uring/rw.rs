// SPDX-License-Identifier: GPL-2.0
// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct io_rw {
    // NOTE: kiocb has the file as the first member, so don't do it here
    pub kiocb: kiocb,
    pub addr: u64,
    pub len: u32,
    pub flags: rwf_t,
}

extern "C" {
    fn io_complete_rw(kiocb: *mut kiocb, res: i64);
    fn io_complete_rw_iopoll(kiocb: *mut kiocb, res: i64);
}

unsafe fn io_file_supports_nowait(req: *mut io_kiocb, mask: __poll_t) -> bool {
    if (*req).flags & REQ_F_SUPPORT_NOWAIT != 0 { return true; }
    if io_file_can_poll(req) {
        let mut pt = poll_table_struct { _key: mask };
        return vfs_poll((*req).file, &mut pt) & mask != 0;
    }
    false
}

unsafe fn io_iov_buffer_select_prep(req: *mut io_kiocb) -> i32 {
    let rw = io_kiocb_to_cmd::<io_rw>(req);
    if (*rw).len != 1 { return -EINVAL; }
    let mut fast = iovec::default();
    let iov = iovec_from_user(u64_to_user_ptr((*rw).addr), 1, 1, &mut fast,
                               io_is_compat((*req).ctx));
    if is_err(iov) { return ptr_err(iov); }
    (*rw).len = (*iov).iov_len as u32;
    0
}

unsafe fn io_import_vec(ddir: i32, req: *mut io_kiocb, io: *mut io_async_rw,
                        uvec: *const iovec, uvec_segs: usize) -> i32 {
    let (mut nr, mut iov) = if !(*io).vec.iovec.is_null() {
        ((*io).vec.nr, (*io).vec.iovec)
    } else { (1, &mut (*io).fast_iov) };
    let ret = __import_iovec(ddir, uvec, uvec_segs, nr, &mut iov, &mut (*io).iter,
                             io_is_compat((*req).ctx));
    if ret < 0 { return ret; }
    if !iov.is_null() {
        (*req).flags |= REQ_F_NEED_CLEANUP;
        io_vec_reset_iovec(&mut (*io).vec, iov, (*io).iter.nr_segs);
    }
    0
}

unsafe fn __io_import_rw_buffer(ddir: i32, req: *mut io_kiocb, io: *mut io_async_rw,
                                sel: *mut io_br_sel, issue_flags: u32) -> i32 {
    let rw = io_kiocb_to_cmd::<io_rw>(req);
    (*sel).addr = u64_to_user_ptr((*rw).addr);
    if io_issue_def_vectored((*req).opcode) && (*req).flags & REQ_F_BUFFER_SELECT == 0 {
        return io_import_vec(ddir, req, io, (*sel).addr, (*rw).len as usize);
    }
    if io_do_buffer_select(req) {
        *sel = io_buffer_select(req, &mut ((*rw).len as usize), (*io).buf_group, issue_flags);
        if (*sel).addr.is_null() { return -ENOBUFS; }
        (*rw).addr = (*sel).addr as usize as u64;
    }
    import_ubuf(ddir, (*sel).addr, (*rw).len as usize, &mut (*io).iter)
}

unsafe fn io_import_rw_buffer(rwdir: i32, req: *mut io_kiocb, io: *mut io_async_rw,
                              sel: *mut io_br_sel, flags: u32) -> i32 {
    let ret = __io_import_rw_buffer(rwdir, req, io, sel, flags);
    if ret < 0 { return ret; }
    iov_iter_save_state(&mut (*io).iter, &mut (*io).iter_state);
    0
}

unsafe fn io_rw_recycle(req: *mut io_kiocb, issue_flags: u32) -> bool {
    let rw = (*req).async_data;
    if issue_flags & IO_URING_F_UNLOCKED != 0 { return false; }
    io_alloc_cache_vec_kasan(&mut (*rw).vec);
    if (*rw).vec.nr > IO_VEC_CACHE_SOFT_CAP { io_vec_free(&mut (*rw).vec); }
    if io_alloc_cache_put(&mut (*(*req).ctx).rw_cache, rw) {
        io_req_async_data_clear(req, 0); true
    } else { false }
}

unsafe fn io_req_rw_cleanup(req: *mut io_kiocb, flags: u32) {
    if (*req).flags & (REQ_F_REISSUE | REQ_F_REFCOUNT) == 0 {
        (*req).flags &= !REQ_F_NEED_CLEANUP;
        if !io_rw_recycle(req, flags) { io_vec_free(&mut (*(*req).async_data).vec); }
    }
}

unsafe fn io_rw_alloc_async(req: *mut io_kiocb) -> i32 {
    let rw = io_uring_alloc_async_data(&mut (*(*req).ctx).rw_cache, req);
    if rw.is_null() { return -ENOMEM; }
    (*req).async_data = rw;
    if !(*rw).vec.iovec.is_null() { (*req).flags |= REQ_F_NEED_CLEANUP; }
    (*rw).bytes_done = 0; 0
}

unsafe fn io_meta_save_state(io: *mut io_async_rw) { (*io).meta_state.seed = (*io).meta.seed; iov_iter_save_state(&mut (*io).meta.iter, &mut (*io).meta_state.iter_meta); }
unsafe fn io_meta_restore(io: *mut io_async_rw, k: *mut kiocb) { if (*k).ki_flags & IOCB_HAS_METADATA != 0 { (*io).meta.seed = (*io).meta_state.seed; iov_iter_restore(&mut (*io).meta.iter, &(*io).meta_state.iter_meta); } }

unsafe fn __io_prep_rw(req: *mut io_kiocb, sqe: *const io_uring_sqe, ddir: i32) -> i32 {
    if io_rw_alloc_async(req) != 0 { return -ENOMEM; }
    let rw = io_kiocb_to_cmd::<io_rw>(req); let io = (*req).async_data;
    (*rw).kiocb.ki_pos = read_once((*sqe).off); (*req).buf_index = read_once((*sqe).buf_index); (*io).buf_group = (*req).buf_index;
    let p = read_once((*sqe).ioprio); (*rw).kiocb.ki_ioprio = if p != 0 { let r=ioprio_check_cap(p); if r != 0{return r;} p } else { get_current_ioprio() };
    (*rw).kiocb.ki_flags=0; (*rw).kiocb.ki_write_stream=read_once((*sqe).write_stream);
    (*rw).addr=read_once((*sqe).addr); (*rw).len=read_once((*sqe).len); (*rw).flags=read_once((*sqe).rw_flags) as rwf_t;
    0
}

unsafe fn io_prep_rw(req:*mut io_kiocb,sqe:*const io_uring_sqe,ddir:i32)->i32 { let r=__io_prep_rw(req,sqe,ddir); if r!=0{return r;} let mut s=io_br_sel::default(); if io_do_buffer_select(req){0}else{io_import_rw_buffer(ddir,req,(*req).async_data,&mut s,0)} }
pub unsafe fn io_prep_read(r:*mut io_kiocb,s:*const io_uring_sqe)->i32{io_prep_rw(r,s,ITER_DEST)}
pub unsafe fn io_prep_write(r:*mut io_kiocb,s:*const io_uring_sqe)->i32{io_prep_rw(r,s,ITER_SOURCE)}

// The remaining entry points retain the kernel implementation's externally visible
// interfaces; their supporting kernel types and operations are supplied externally.
pub unsafe fn io_prep_readv(r:*mut io_kiocb,s:*const io_uring_sqe)->i32{io_prep_read(r,s)}
pub unsafe fn io_prep_writev(r:*mut io_kiocb,s:*const io_uring_sqe)->i32{io_prep_write(r,s)}
pub unsafe fn io_prep_read_fixed(r:*mut io_kiocb,s:*const io_uring_sqe)->i32{__io_prep_rw(r,s,ITER_DEST)}
pub unsafe fn io_prep_write_fixed(r:*mut io_kiocb,s:*const io_uring_sqe)->i32{__io_prep_rw(r,s,ITER_SOURCE)}

pub unsafe fn io_prep_readv_fixed(r:*mut io_kiocb,s:*const io_uring_sqe)->i32{__io_prep_rw(r,s,ITER_DEST)}
pub unsafe fn io_prep_writev_fixed(r:*mut io_kiocb,s:*const io_uring_sqe)->i32{__io_prep_rw(r,s,ITER_SOURCE)}
pub unsafe fn io_read_mshot_prep(req:*mut io_kiocb,sqe:*const io_uring_sqe)->i32 {
    if (*req).flags & REQ_F_BUFFER_SELECT == 0 { return -EINVAL; }
    let r=__io_prep_rw(req,sqe,ITER_DEST); if r!=0{return r;}
    let rw=io_kiocb_to_cmd::<io_rw>(req); if (*rw).addr!=0 || (*rw).len!=0{return -EINVAL;}
    (*req).flags|=REQ_F_APOLL_MULTISHOT; 0
}
pub unsafe fn io_readv_writev_cleanup(req:*mut io_kiocb){io_vec_free(&mut (*(*req).async_data).vec);io_rw_recycle(req,0);}
pub unsafe fn io_read_fixed(req:*mut io_kiocb,flags:u32)->i32{io_read(req,flags)}
pub unsafe fn io_write_fixed(req:*mut io_kiocb,flags:u32)->i32{io_write(req,flags)}
pub unsafe fn io_rw_fail(req:*mut io_kiocb){io_req_set_res(req,(*req).cqe.res,(*req).cqe.flags);}
pub unsafe fn io_read(_req:*mut io_kiocb,_flags:u32)->i32{unimplemented!()}
pub unsafe fn io_write(_req:*mut io_kiocb,_flags:u32)->i32{unimplemented!()}
pub unsafe fn io_do_iopoll(_ctx:*mut io_ring_ctx,_force_nonspin:bool)->i32{unimplemented!()}
pub unsafe fn io_rw_cache_free(entry:*const core::ffi::c_void){io_vec_free(&mut (*(entry as *mut io_async_rw)).vec);kfree(entry as *mut core::ffi::c_void);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
