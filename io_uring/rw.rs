// SPDX-License-Identifier: GPL-2.0
//! io_uring read/write operations, translated from rw.c.
//!
//! Request state, completion, retries, multishot and polling are owned here.
//! The accompanying helper file exposes existing C header inline/macro APIs.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/io_uring_rw_generated.rs"));
}
use bindings::*;
use core::mem::{offset_of, size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, null_mut, read_volatile};
use kernel::ffi::{c_int, c_long, c_void};

// Match the C types of ki_flags and errno expressions rather than bindgen's
// unsigned macro spelling. Request flags retain their generated 64-bit type.
const EAGAIN: c_int = bindings::EAGAIN as c_int;
const EBADF: c_int = bindings::EBADF as c_int;
const EBADFD: c_int = bindings::EBADFD as c_int;
const EFAULT: c_int = bindings::EFAULT as c_int;
const EINTR: c_int = bindings::EINTR as c_int;
const EINVAL: c_int = bindings::EINVAL as c_int;
const EIOCBQUEUED: c_int = bindings::EIOCBQUEUED as c_int;
const ENOBUFS: c_int = bindings::ENOBUFS as c_int;
const ENOMEM: c_int = bindings::ENOMEM as c_int;
const EOPNOTSUPP: c_int = bindings::EOPNOTSUPP as c_int;
const ERESTARTNOHAND: c_int = bindings::ERESTARTNOHAND as c_int;
const ERESTARTNOINTR: c_int = bindings::ERESTARTNOINTR as c_int;
const ERESTARTSYS: c_int = bindings::ERESTARTSYS as c_int;
const ERESTART_RESTARTBLOCK: c_int = bindings::ERESTART_RESTARTBLOCK as c_int;
const IOCB_DIRECT: c_int = bindings::IOCB_DIRECT as c_int;
const IOCB_HAS_METADATA: c_int = bindings::IOCB_HAS_METADATA as c_int;
const IOCB_HIPRI: c_int = bindings::RUST_RW_IOCB_HIPRI as c_int;
const IOCB_NOWAIT: c_int = bindings::RUST_RW_IOCB_NOWAIT as c_int;
const IOCB_WAITQ: c_int = bindings::IOCB_WAITQ as c_int;
const IOCB_WRITE: c_int = bindings::IOCB_WRITE as c_int;

const REQ_F_APOLL_MULTISHOT: u64 = bindings::REQ_F_APOLL_MULTISHOT as u64;
const REQ_F_BL_NO_RECYCLE: u64 = bindings::REQ_F_BL_NO_RECYCLE as u64;
const REQ_F_BUFFERS_COMMIT: u64 = bindings::REQ_F_BUFFERS_COMMIT as u64;
const REQ_F_BUFFER_RING: u64 = bindings::REQ_F_BUFFER_RING as u64;
const REQ_F_BUFFER_SELECT: u64 = bindings::REQ_F_BUFFER_SELECT as u64;
const REQ_F_BUFFER_SELECTED: u64 = bindings::REQ_F_BUFFER_SELECTED as u64;
const REQ_F_BUF_NODE: u64 = bindings::REQ_F_BUF_NODE as u64;
const REQ_F_CUR_POS: u64 = bindings::REQ_F_CUR_POS as u64;
const REQ_F_FIXED_FILE: u64 = bindings::REQ_F_FIXED_FILE as u64;
const REQ_F_HAS_METADATA: u64 = bindings::REQ_F_HAS_METADATA as u64;
const REQ_F_IMPORT_BUFFER: u64 = bindings::REQ_F_IMPORT_BUFFER as u64;
const REQ_F_IOPOLL: u64 = bindings::REQ_F_IOPOLL as u64;
const REQ_F_IOPOLL_STATE: u64 = bindings::REQ_F_IOPOLL_STATE as u64;
const REQ_F_ISREG: u64 = bindings::REQ_F_ISREG as u64;
const REQ_F_NEED_CLEANUP: u64 = bindings::REQ_F_NEED_CLEANUP as u64;
const REQ_F_NOWAIT: u64 = bindings::REQ_F_NOWAIT as u64;
const REQ_F_REFCOUNT: u64 = bindings::REQ_F_REFCOUNT as u64;
const REQ_F_REISSUE: u64 = bindings::REQ_F_REISSUE as u64;
const REQ_F_SUPPORT_NOWAIT: u64 = bindings::REQ_F_SUPPORT_NOWAIT as u64;
const IO_URING_F_UNLOCKED: u32 = bindings::RUST_RW_IO_URING_F_UNLOCKED as u32;
const IO_URING_F_NONBLOCK: u32 = bindings::RUST_RW_IO_URING_F_NONBLOCK as u32;
const IO_URING_F_MULTISHOT: u32 = bindings::RUST_RW_IO_URING_F_MULTISHOT as u32;
const FMODE_STREAM: u32 = bindings::RUST_RW_FMODE_STREAM as u32;
const FMODE_HAS_METADATA: u32 = bindings::RUST_RW_FMODE_HAS_METADATA as u32;
const FMODE_READ: u32 = bindings::RUST_RW_FMODE_READ as u32;
const FMODE_WRITE: u32 = bindings::RUST_RW_FMODE_WRITE as u32;
const FOP_BUFFER_RASYNC: u32 = bindings::RUST_RW_FOP_BUFFER_RASYNC as u32;
const FOP_BUFFER_WASYNC: u32 = bindings::RUST_RW_FOP_BUFFER_WASYNC as u32;
const EPOLLIN: u32 = bindings::RUST_RW_EPOLLIN as u32;
const EPOLLOUT: u32 = bindings::RUST_RW_EPOLLOUT as u32;

#[inline] unsafe fn rw(req: *mut io_kiocb) -> *mut io_rw { req.cast() }
#[inline] unsafe fn file(req: *mut io_kiocb) -> *mut bindings::file { (*req).__bindgen_anon_1.file }
#[inline] unsafe fn io(req: *mut io_kiocb) -> *mut io_async_rw { (*req).async_data.cast() }
#[inline] unsafe fn data(io: *mut io_async_rw) -> *mut io_async_rw__bindgen_ty_1__bindgen_ty_1 {
    addr_of_mut!((*io).__bindgen_anon_1.__bindgen_anon_1)
}
#[inline] unsafe fn meta(io: *mut io_async_rw) -> *mut io_async_rw__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
    addr_of_mut!((*data(io)).__bindgen_anon_1.__bindgen_anon_1)
}
#[inline] unsafe fn iter_count(iter: *const iov_iter) -> usize {
    (*iter).__bindgen_anon_1.__bindgen_anon_1.count
}
#[inline] unsafe fn ctx_flags(ctx: *const io_ring_ctx) -> u32 { (*ctx).__bindgen_anon_1.flags }
#[inline] fn is_err<T>(p: *const T) -> bool { p as usize >= usize::MAX - 4094 }
#[inline] fn ptr_err<T>(p: *const T) -> c_int { p as isize as c_int }

unsafe fn io_file_supports_nowait(req: *mut io_kiocb, mask: __poll_t) -> bool {
    if (*req).flags & REQ_F_SUPPORT_NOWAIT != 0 { return true; }
    if rust_rw_file_can_poll(req) {
        let mut pt: poll_table_struct = zeroed();
        pt._key = mask;
        return rust_rw_vfs_poll(file(req), &mut pt) & mask != 0;
    }
    false
}

unsafe fn io_iov_buffer_select_prep(req: *mut io_kiocb) -> c_int {
    let rw = rw(req);
    if (*rw).len != 1 { return -EINVAL; }
    let mut fast: iovec = zeroed();
    let iov = iovec_from_user((*rw).addr as usize as *const iovec, 1, 1,
                              &mut fast, rust_rw_is_compat((*req).ctx));
    if is_err(iov) { return ptr_err(iov); }
    (*rw).len = (*iov).iov_len as u32;
    0
}

unsafe fn io_import_vec(ddir: c_int, req: *mut io_kiocb, io: *mut io_async_rw,
                       uvec: *const iovec, uvec_segs: usize) -> c_int {
    let d = data(io);
    let (nr, mut iov) = if !(*io).vec.__bindgen_anon_1.iovec.is_null() {
        ((*io).vec.nr, (*io).vec.__bindgen_anon_1.iovec)
    } else { (1, addr_of_mut!((*d).fast_iov)) };
    let ret = __import_iovec(ddir, uvec, uvec_segs as _, nr as _, &mut iov,
                             addr_of_mut!((*d).iter), rust_rw_is_compat((*req).ctx)) as c_int;
    if ret < 0 { return ret; }
    if !iov.is_null() {
        (*req).flags |= REQ_F_NEED_CLEANUP;
        rust_rw_vec_reset(addr_of_mut!((*io).vec), iov, (*d).iter.__bindgen_anon_2.nr_segs as _);
    }
    0
}

unsafe fn __io_import_rw_buffer(ddir: c_int, req: *mut io_kiocb, io: *mut io_async_rw,
                               sel: *mut io_br_sel, issue_flags: u32) -> c_int {
    let rw = rw(req);
    let mut len = (*rw).len as usize;
    (*sel).__bindgen_anon_1.addr = (*rw).addr as usize as *mut c_void;
    if rust_rw_vectored((*req).opcode as _) && (*req).flags & REQ_F_BUFFER_SELECT == 0 {
        return io_import_vec(ddir, req, io, (*sel).__bindgen_anon_1.addr.cast(), len);
    }
    if rust_rw_do_buffer_select(req) {
        *sel = io_buffer_select(req, &mut len, (*data(io)).buf_group, issue_flags);
        if (*sel).__bindgen_anon_1.addr.is_null() { return -ENOBUFS; }
        (*rw).addr = (*sel).__bindgen_anon_1.addr as usize as u64;
        (*rw).len = len as u32;
    }
    import_ubuf(ddir, (*sel).__bindgen_anon_1.addr, len, addr_of_mut!((*data(io)).iter))
}

unsafe fn io_import_rw_buffer(ddir: c_int, req: *mut io_kiocb, io: *mut io_async_rw,
                             sel: *mut io_br_sel, flags: u32) -> c_int {
    let ret = __io_import_rw_buffer(ddir, req, io, sel, flags);
    if ret < 0 { return ret; }
    let d = data(io);
    rust_rw_iter_save(addr_of_mut!((*d).iter), addr_of_mut!((*d).iter_state));
    0
}

unsafe fn io_rw_recycle(req: *mut io_kiocb, issue_flags: u32) -> bool {
    let io = io(req);
    if issue_flags & IO_URING_F_UNLOCKED != 0 { return false; }
    rust_rw_vec_kasan(addr_of_mut!((*io).vec));
    if (*io).vec.nr > IO_VEC_CACHE_SOFT_CAP { io_vec_free(addr_of_mut!((*io).vec)); }
    if rust_rw_cache_put(addr_of_mut!((*(*req).ctx).__bindgen_anon_2.rw_cache), io.cast()) {
        rust_rw_async_clear(req);
        return true;
    }
    false
}

unsafe fn io_req_rw_cleanup(req: *mut io_kiocb, flags: u32) {
    // io-wq may still touch the iterator after invoking completion. Keep its
    // allocation alive until normal request teardown, exactly as in rw.c.
    if (*req).flags & (REQ_F_REISSUE | REQ_F_REFCOUNT) == 0 {
        (*req).flags &= !REQ_F_NEED_CLEANUP;
        if !io_rw_recycle(req, flags) { io_vec_free(addr_of_mut!((*io(req)).vec)); }
    }
}

unsafe fn io_rw_alloc_async(req: *mut io_kiocb) -> c_int {
    let io = rust_rw_async_alloc(addr_of_mut!((*(*req).ctx).__bindgen_anon_2.rw_cache), req).cast::<io_async_rw>();
    if io.is_null() { return -ENOMEM; }
    if !(*io).vec.__bindgen_anon_1.iovec.is_null() { (*req).flags |= REQ_F_NEED_CLEANUP; }
    (*io).bytes_done = 0;
    0
}

unsafe fn io_meta_save_state(io: *mut io_async_rw) {
    let m = meta(io);
    (*m).meta_state.seed = (*m).meta.seed as u32;
    rust_rw_iter_save(addr_of_mut!((*m).meta.iter), addr_of_mut!((*m).meta_state.iter_meta));
}
unsafe fn io_meta_restore(io: *mut io_async_rw, kiocb: *mut kiocb) {
    if (*kiocb).ki_flags & IOCB_HAS_METADATA != 0 {
        let m = meta(io);
        (*m).meta.seed = (*m).meta_state.seed as u64;
        iov_iter_restore(addr_of_mut!((*m).meta.iter), addr_of_mut!((*m).meta_state.iter_meta));
    }
}

unsafe fn io_prep_rw_pi(req: *mut io_kiocb, ddir: c_int, attr_ptr: u64) -> c_int {
    let mut attr: io_uring_attr_pi = zeroed();
    if rust_rw_copy_from_user(addr_of_mut!(attr).cast(), attr_ptr as usize as *const c_void,
                             size_of::<io_uring_attr_pi>() as _) != 0 { return -EFAULT; }
    if attr.rsvd != 0 { return -EINVAL; }
    let io = io(req);
    let m = meta(io);
    (*m).meta.flags = attr.flags;
    (*m).meta.app_tag = attr.app_tag;
    (*m).meta.seed = attr.seed;
    let ret = import_ubuf(ddir, attr.addr as usize as *mut c_void, attr.len as _, addr_of_mut!((*m).meta.iter));
    if ret < 0 { return ret; }
    (*req).flags |= REQ_F_HAS_METADATA;
    io_meta_save_state(io);
    ret
}

unsafe fn __io_prep_rw(req: *mut io_kiocb, sqe: *const io_uring_sqe, ddir: c_int) -> c_int {
    let rw = rw(req);
    if io_rw_alloc_async(req) != 0 { return -ENOMEM; }
    let io = io(req);
    (*rw).kiocb.ki_pos = read_volatile(addr_of!((*sqe).__bindgen_anon_1.off)) as _;
    (*req).buf_index = read_volatile(addr_of!((*sqe).__bindgen_anon_4.buf_index));
    (*data(io)).buf_group = (*req).buf_index as _;
    let ioprio = read_volatile(addr_of!((*sqe).ioprio));
    if ioprio != 0 {
        let ret = rust_rw_ioprio_check_cap(ioprio as _);
        if ret != 0 { return ret; }
        (*rw).kiocb.ki_ioprio = ioprio;
    } else { (*rw).kiocb.ki_ioprio = rust_rw_current_ioprio(); }
    (*rw).kiocb.ki_flags = 0;
    (*rw).kiocb.ki_write_stream = read_volatile(addr_of!((*sqe).__bindgen_anon_5.__bindgen_anon_2.write_stream));
    (*rw).kiocb.ki_complete = if ctx_flags((*req).ctx) & IORING_SETUP_IOPOLL != 0 {
        Some(io_complete_rw_iopoll)
    } else { Some(io_complete_rw) };
    (*rw).addr = read_volatile(addr_of!((*sqe).__bindgen_anon_2.addr));
    (*rw).len = read_volatile(addr_of!((*sqe).len));
    (*rw).flags = read_volatile(addr_of!((*sqe).__bindgen_anon_3.rw_flags)) as _;
    let attrs = addr_of!((*sqe).__bindgen_anon_6).cast::<io_uring_sqe__bindgen_ty_6__bindgen_ty_2>();
    let mask = read_volatile(addr_of!((*attrs).attr_type_mask));
    if mask != 0 {
        if mask != IORING_RW_ATTR_FLAG_PI as u64 { return -EINVAL; }
        return io_prep_rw_pi(req, ddir, read_volatile(addr_of!((*attrs).attr_ptr)));
    }
    0
}

unsafe fn io_prep_rw(req: *mut io_kiocb, sqe: *const io_uring_sqe, ddir: c_int) -> c_int {
    let ret = __io_prep_rw(req, sqe, ddir);
    if ret != 0 { return ret; }
    if rust_rw_do_buffer_select(req) { return 0; }
    let mut sel: io_br_sel = zeroed();
    io_import_rw_buffer(ddir, req, io(req), &mut sel, 0)
}
unsafe fn io_prep_rwv(req: *mut io_kiocb, sqe: *const io_uring_sqe, ddir: c_int) -> c_int {
    let ret = io_prep_rw(req, sqe, ddir);
    if ret != 0 { return ret; }
    if (*req).flags & REQ_F_BUFFER_SELECT == 0 { return 0; }
    io_iov_buffer_select_prep(req)
}
#[no_mangle] pub unsafe extern "C" fn io_prep_read(r: *mut io_kiocb, s: *const io_uring_sqe) -> c_int { io_prep_rw(r, s, ITER_DEST as _) }
#[no_mangle] pub unsafe extern "C" fn io_prep_write(r: *mut io_kiocb, s: *const io_uring_sqe) -> c_int { io_prep_rw(r, s, ITER_SOURCE as _) }
#[no_mangle] pub unsafe extern "C" fn io_prep_readv(r: *mut io_kiocb, s: *const io_uring_sqe) -> c_int { io_prep_rwv(r, s, ITER_DEST as _) }
#[no_mangle] pub unsafe extern "C" fn io_prep_writev(r: *mut io_kiocb, s: *const io_uring_sqe) -> c_int { io_prep_rwv(r, s, ITER_SOURCE as _) }
#[no_mangle] pub unsafe extern "C" fn io_prep_read_fixed(r: *mut io_kiocb, s: *const io_uring_sqe) -> c_int { __io_prep_rw(r, s, ITER_DEST as _) }
#[no_mangle] pub unsafe extern "C" fn io_prep_write_fixed(r: *mut io_kiocb, s: *const io_uring_sqe) -> c_int { __io_prep_rw(r, s, ITER_SOURCE as _) }

unsafe fn io_init_rw_fixed(req: *mut io_kiocb, flags: u32, ddir: c_int) -> c_int {
    let rw = rw(req);
    let io = io(req);
    if (*io).bytes_done != 0 { return 0; }
    let d = data(io);
    let ret = io_import_reg_buf(req, addr_of_mut!((*d).iter), (*rw).addr, (*rw).len as _, ddir, flags);
    rust_rw_iter_save(addr_of_mut!((*d).iter), addr_of_mut!((*d).iter_state));
    ret
}
unsafe fn io_rw_import_reg_vec(req: *mut io_kiocb, io: *mut io_async_rw, ddir: c_int, flags: u32) -> c_int {
    let d = data(io);
    let ret = io_import_reg_vec(ddir, addr_of_mut!((*d).iter), req, addr_of_mut!((*io).vec), (*rw(req)).len, flags);
    if ret != 0 { return ret; }
    rust_rw_iter_save(addr_of_mut!((*d).iter), addr_of_mut!((*d).iter_state));
    (*req).flags &= !REQ_F_IMPORT_BUFFER;
    0
}
unsafe fn io_prep_rwv_fixed(req: *mut io_kiocb, sqe: *const io_uring_sqe, ddir: c_int) -> c_int {
    let ret = __io_prep_rw(req, sqe, ddir);
    if ret != 0 { return ret; }
    let rw = rw(req);
    io_prep_reg_iovec(req, addr_of_mut!((*io(req)).vec), (*rw).addr as usize as *const iovec, (*rw).len as _)
}
#[no_mangle] pub unsafe extern "C" fn io_prep_readv_fixed(r: *mut io_kiocb, s: *const io_uring_sqe) -> c_int { io_prep_rwv_fixed(r, s, ITER_DEST as _) }
#[no_mangle] pub unsafe extern "C" fn io_prep_writev_fixed(r: *mut io_kiocb, s: *const io_uring_sqe) -> c_int { io_prep_rwv_fixed(r, s, ITER_SOURCE as _) }

#[no_mangle]
pub unsafe extern "C" fn io_read_mshot_prep(req: *mut io_kiocb, sqe: *const io_uring_sqe) -> c_int {
    if (*req).flags & REQ_F_BUFFER_SELECT == 0 { return -EINVAL; }
    let ret = __io_prep_rw(req, sqe, ITER_DEST as _);
    if ret != 0 { return ret; }
    if (*rw(req)).addr != 0 || (*rw(req)).len != 0 { return -EINVAL; }
    (*req).flags |= REQ_F_APOLL_MULTISHOT;
    0
}
#[no_mangle]
pub unsafe extern "C" fn io_readv_writev_cleanup(req: *mut io_kiocb) {
    rust_rw_assert_locked((*req).ctx);
    io_vec_free(addr_of_mut!((*io(req)).vec));
    io_rw_recycle(req, 0);
}

unsafe fn io_kiocb_update_pos(req: *mut io_kiocb) -> *mut loff_t {
    let rw = rw(req);
    if (*rw).kiocb.ki_pos != -1 { return addr_of_mut!((*rw).kiocb.ki_pos); }
    if (*file(req)).f_mode & FMODE_STREAM != 0 { (*rw).kiocb.ki_pos = 0; return null_mut(); }
    (*req).flags |= REQ_F_CUR_POS;
    (*rw).kiocb.ki_pos = (*file(req)).f_pos;
    addr_of_mut!((*rw).kiocb.ki_pos)
}

unsafe fn io_rw_should_reissue(req: *mut io_kiocb) -> bool {
    #[cfg(CONFIG_BLOCK)] {
        let mode = (*(*file(req)).f_inode).i_mode as u32 & S_IFMT;
        if mode != S_IFBLK && mode != S_IFREG { return false; }
        if (*req).flags & REQ_F_NOWAIT != 0 ||
            (rust_rw_worker() && (*req).flags & REQ_F_IOPOLL == 0) { return false; }
        if rust_rw_dying((*req).ctx) { return false; }
        let io = io(req);
        io_meta_restore(io, addr_of_mut!((*rw(req)).kiocb));
        let d = data(io);
        iov_iter_restore(addr_of_mut!((*d).iter), addr_of_mut!((*d).iter_state));
        true
    }
    #[cfg(not(CONFIG_BLOCK))] { let _ = req; false }
}
unsafe fn io_req_end_write(req: *mut io_kiocb) {
    if (*req).flags & REQ_F_ISREG != 0 { rust_rw_end_write(addr_of_mut!((*rw(req)).kiocb)); }
}
unsafe fn io_req_io_end(req: *mut io_kiocb) {
    if (*rw(req)).kiocb.ki_flags & IOCB_WRITE != 0 {
        io_req_end_write(req);
        rust_rw_notify_modify(file(req));
    } else { rust_rw_notify_access(file(req)); }
}
unsafe fn __io_complete_rw_common(req: *mut io_kiocb, res: c_long) {
    if res == (*req).cqe.res as c_long { return; }
    if (res == -EOPNOTSUPP as c_long || res == -EAGAIN as c_long) && io_rw_should_reissue(req) {
        (*req).flags |= REQ_F_REISSUE | REQ_F_BL_NO_RECYCLE;
    } else { rust_rw_set_fail(req); (*req).cqe.res = res as c_int; }
}
unsafe fn io_fixup_rw_res(req: *mut io_kiocb, mut res: c_long) -> c_int {
    let io = io(req);
    if rust_rw_has_async(req) && (*io).bytes_done > 0 {
        res = if res < 0 { (*io).bytes_done as c_long }
              else { (res as usize).wrapping_add((*io).bytes_done) as c_long };
    }
    res as c_int
}
#[no_mangle]
pub unsafe extern "C" fn io_req_rw_complete(tw_req: io_tw_req, tw: io_tw_token_t) {
    let req = tw_req.req;
    io_req_io_end(req);
    if (*req).flags & (REQ_F_BUFFER_SELECTED | REQ_F_BUFFER_RING) != 0 {
        (*req).cqe.__bindgen_anon_1.flags |= rust_rw_put_kbuf(req, (*req).cqe.res.max(0), null_mut());
    }
    io_req_rw_cleanup(req, 0);
    io_req_task_complete(tw_req, tw);
}
unsafe extern "C" fn io_complete_rw(kiocb: *mut kiocb, res: c_long) {
    // kiocb is the first field of the generated io_rw overlay and request.
    let req = kiocb.cast::<io_kiocb>();
    __io_complete_rw_common(req, res);
    rust_rw_set_res(req, io_fixup_rw_res(req, res), 0);
    (*req).__bindgen_anon_4.io_task_work.func = Some(io_req_rw_complete);
    rust_rw_task_work_add(req, IOU_F_TWQ_LAZY_WAKE);
}
unsafe extern "C" fn io_complete_rw_iopoll(kiocb: *mut kiocb, res: c_long) {
    let req = kiocb.cast::<io_kiocb>();
    let final_res = io_fixup_rw_res(req, res);
    if (*kiocb).ki_flags & IOCB_WRITE != 0 { io_req_end_write(req); }
    if res == -EAGAIN as c_long && io_rw_should_reissue(req) {
        (*req).flags |= REQ_F_REISSUE | REQ_F_BL_NO_RECYCLE;
    } else if final_res != (*req).cqe.res { (*req).cqe.res = final_res; }
    // Pairs with the acquire load in io_do_iopoll before consuming the CQE.
    rust_rw_store_complete(req);
}
fn io_fixup_restart_res(ret: c_long) -> c_long {
    if ret == -ERESTARTSYS as c_long || ret == -ERESTARTNOINTR as c_long ||
       ret == -ERESTARTNOHAND as c_long || ret == -ERESTART_RESTARTBLOCK as c_long {
        -EINTR as c_long
    } else { ret }
}
unsafe fn io_rw_done(req: *mut io_kiocb, mut ret: c_long) {
    if ret == -EIOCBQUEUED as c_long { return; }
    if ret < 0 { ret = io_fixup_restart_res(ret); }
    if (*req).flags & REQ_F_IOPOLL != 0 {
        io_complete_rw_iopoll(addr_of_mut!((*rw(req)).kiocb), ret);
    } else { io_complete_rw(addr_of_mut!((*rw(req)).kiocb), ret); }
}
unsafe fn kiocb_done(req: *mut io_kiocb, ret: c_long, sel: *mut io_br_sel, flags: u32) -> c_int {
    let final_ret = io_fixup_rw_res(req, ret);
    if ret >= 0 && (*req).flags & REQ_F_CUR_POS != 0 { (*file(req)).f_pos = (*rw(req)).kiocb.ki_pos; }
    if ret >= 0 && (*req).flags & REQ_F_IOPOLL == 0 {
        __io_complete_rw_common(req, ret);
        io_req_io_end(req);
        let cflags = if sel.is_null() { 0 } else { rust_rw_put_kbuf(req, ret as _, (*sel).buf_list) };
        rust_rw_set_res(req, final_ret, cflags);
        io_req_rw_cleanup(req, flags);
        IOU_COMPLETE
    } else { io_rw_done(req, ret); IOU_ISSUE_SKIP_COMPLETE }
}

unsafe fn loop_rw_iter(ddir: c_int, rw: *mut io_rw, iter: *mut iov_iter) -> c_long {
    let req = rw.cast::<io_kiocb>();
    let kiocb = addr_of_mut!((*rw).kiocb);
    let file = (*kiocb).ki_filp;
    if (*kiocb).ki_flags & IOCB_HIPRI != 0 { return -EOPNOTSUPP as _; }
    if (*kiocb).ki_flags & IOCB_NOWAIT != 0 && (*file).f_flags & O_NONBLOCK == 0 { return -EAGAIN as _; }
    if (*req).flags & REQ_F_BUF_NODE != 0 &&
       (*(*(*req).__bindgen_anon_2.buf_node).__bindgen_anon_1.buf).flags as u32 & IO_REGBUF_F_KBUF != 0 {
        return -EFAULT as _;
    }
    let ppos = if (*file).f_mode & FMODE_STREAM != 0 { null_mut() } else { addr_of_mut!((*kiocb).ki_pos) };
    let mut ret: c_long = 0;
    while iter_count(iter) != 0 {
        let (addr, len) = if rust_rw_iter_ubuf(iter) {
            // This is a userspace address; the legacy callback checks access.
            // Do not impose Rust allocation bounds while calculating it.
            ((*iter).__bindgen_anon_1.__bindgen_anon_1.__bindgen_anon_1.ubuf.cast::<u8>().wrapping_add((*iter).iov_offset).cast(), iter_count(iter))
        } else if !rust_rw_iter_bvec(iter) {
            (rust_rw_iter_iov_addr(iter), rust_rw_iter_iov_len(iter))
        } else { ((*rw).addr as usize as *mut c_void, (*rw).len as usize) };
        let nr = if ddir == READ as c_int {
            ((*(*file).f_op).read.unwrap_unchecked())(file, addr.cast(), len, ppos)
        } else { ((*(*file).f_op).write.unwrap_unchecked())(file, addr.cast(), len, ppos) };
        if nr < 0 { if ret == 0 { ret = nr; } break; }
        ret = ret.wrapping_add(nr);
        if !rust_rw_iter_bvec(iter) { iov_iter_advance(iter, nr as _); }
        else {
            (*rw).addr = (*rw).addr.wrapping_add(nr as u64);
            (*rw).len = (*rw).len.wrapping_sub(nr as u32);
            if (*rw).len == 0 { break; }
        }
        if nr as usize != len { break; }
    }
    ret
}
unsafe extern "C" fn io_async_buf_func(wait: *mut wait_queue_entry, _mode: u32, _sync: c_int, arg: *mut c_void) -> c_int {
    let req = (*wait).private.cast::<io_kiocb>();
    let wpq = wait.cast::<u8>().sub(offset_of!(wait_page_queue, wait)).cast();
    if !rust_rw_wake_page_match(wpq, arg.cast()) { return 0; }
    (*rw(req)).kiocb.ki_flags &= !IOCB_WAITQ;
    rust_rw_list_del_init(addr_of_mut!((*wait).entry));
    io_req_task_queue(req);
    1
}
unsafe fn io_rw_should_retry(req: *mut io_kiocb) -> bool {
    let kiocb = addr_of_mut!((*rw(req)).kiocb);
    if (*req).flags & (REQ_F_NOWAIT | REQ_F_HAS_METADATA) != 0 { return false; }
    if (*kiocb).ki_flags & (IOCB_DIRECT | IOCB_HIPRI) != 0 { return false; }
    if rust_rw_file_can_poll(req) || (*(*file(req)).f_op).fop_flags & FOP_BUFFER_RASYNC == 0 { return false; }
    let wait = addr_of_mut!((*data(io(req))).__bindgen_anon_1.wpq);
    (*wait).wait.func = Some(io_async_buf_func);
    (*wait).wait.private = req.cast();
    (*wait).wait.flags = 0;
    rust_rw_list_init(addr_of_mut!((*wait).wait.entry));
    (*kiocb).ki_flags |= IOCB_WAITQ;
    (*kiocb).ki_flags &= !IOCB_NOWAIT;
    (*kiocb).ki_waitq = wait;
    true
}
unsafe fn io_iter_do_read(rw: *mut io_rw, iter: *mut iov_iter) -> c_int {
    let file = (*rw).kiocb.ki_filp;
    if let Some(read) = (*(*file).f_op).read_iter { read(addr_of_mut!((*rw).kiocb), iter) as c_int }
    else if (*(*file).f_op).read.is_some() { loop_rw_iter(READ as _, rw, iter) as c_int }
    else { -EINVAL }
}
unsafe fn need_complete_io(req: *mut io_kiocb) -> bool {
    (*req).flags & REQ_F_ISREG != 0 || (*(*file(req)).f_inode).i_mode as u32 & S_IFMT == S_IFBLK
}

unsafe fn io_rw_init_file(req: *mut io_kiocb, mode: fmode_t, rw_type: c_int) -> c_int {
    let rw = rw(req);
    let kiocb = addr_of_mut!((*rw).kiocb);
    let ctx = (*req).ctx;
    let file = file(req);
    if (*file).f_mode & mode == 0 { return -EBADF; }
    if (*req).flags & REQ_F_FIXED_FILE == 0 { (*req).flags |= io_file_get_flags(file); }
    (*kiocb).ki_flags = (*file).f_iocb_flags as _;
    let ret = rust_rw_set_flags(kiocb, (*rw).flags, rw_type);
    if ret != 0 { return ret; }
    if (*kiocb).ki_flags & IOCB_NOWAIT != 0 ||
       ((*file).f_flags & O_NONBLOCK != 0 && (*req).flags & REQ_F_SUPPORT_NOWAIT == 0) {
        (*req).flags |= REQ_F_NOWAIT;
    }
    if ctx_flags(ctx) & IORING_SETUP_IOPOLL != 0 {
        if (*kiocb).ki_flags & IOCB_DIRECT == 0 || (*(*file).f_op).iopoll.is_none() { return -EOPNOTSUPP; }
        (*req).flags |= REQ_F_IOPOLL;
        (*kiocb).private = null_mut();
        (*kiocb).ki_flags |= IOCB_HIPRI;
        (*req).iopoll_completed = 0;
        if ctx_flags(ctx) & IORING_SETUP_HYBRID_IOPOLL != 0 {
            (*req).flags &= !REQ_F_IOPOLL_STATE;
            (*req).__bindgen_anon_4.iopoll_start = rust_rw_ktime_get_ns();
        }
    } else if (*kiocb).ki_flags & IOCB_HIPRI != 0 { return -EINVAL; }
    if (*req).flags & REQ_F_HAS_METADATA != 0 {
        if (*file).f_mode & FMODE_HAS_METADATA == 0 { return -EINVAL; }
        if (*file).f_flags & O_DIRECT == 0 { return -EOPNOTSUPP; }
        (*kiocb).ki_flags |= IOCB_HAS_METADATA;
        (*kiocb).private = addr_of_mut!((*meta(io(req))).meta).cast();
    }
    0
}

unsafe fn __io_read(req: *mut io_kiocb, sel: *mut io_br_sel, flags: u32) -> c_int {
    let force_nonblock = flags & IO_URING_F_NONBLOCK != 0;
    let rw = rw(req);
    let io = io(req);
    let d = data(io);
    let kiocb = addr_of_mut!((*rw).kiocb);
    if (*req).flags & REQ_F_IMPORT_BUFFER != 0 {
        let ret = io_rw_import_reg_vec(req, io, ITER_DEST as _, flags);
        if ret != 0 { return ret; }
    } else if rust_rw_do_buffer_select(req) {
        let ret = io_import_rw_buffer(ITER_DEST as _, req, io, sel, flags);
        if ret < 0 { return ret; }
    }
    let ret = io_rw_init_file(req, FMODE_READ, READ as _);
    if ret != 0 { return ret; }
    (*req).cqe.res = iter_count(addr_of!((*d).iter)) as _;
    if force_nonblock {
        if !io_file_supports_nowait(req, EPOLLIN) { return -EAGAIN; }
        (*kiocb).ki_flags |= IOCB_NOWAIT;
    } else { (*kiocb).ki_flags &= !IOCB_NOWAIT; }
    let ppos = io_kiocb_update_pos(req);
    let ret = rw_verify_area(READ as _, file(req), ppos, (*req).cqe.res as _);
    if ret != 0 { return ret; }
    let mut ret = io_iter_do_read(rw, addr_of_mut!((*d).iter));
    if ret == -EOPNOTSUPP && force_nonblock { ret = -EAGAIN; }
    if ret == -EAGAIN {
        if rust_rw_file_can_poll(req) { return ret; }
        if !force_nonblock && (*req).flags & REQ_F_IOPOLL == 0 { return ret; }
        if (*req).flags & REQ_F_NOWAIT != 0 { return ret; }
        ret = 0;
    } else if ret == -EIOCBQUEUED { return IOU_ISSUE_SKIP_COMPLETE; }
    else if ret == (*req).cqe.res || ret <= 0 || !force_nonblock ||
            (*req).flags & REQ_F_NOWAIT != 0 || !need_complete_io(req) || flags & IO_URING_F_MULTISHOT != 0 {
        return ret;
    }
    iov_iter_restore(addr_of_mut!((*d).iter), addr_of_mut!((*d).iter_state));
    io_meta_restore(io, kiocb);
    loop {
        iov_iter_advance(addr_of_mut!((*d).iter), ret as _);
        if iter_count(addr_of!((*d).iter)) == 0 { break; }
        (*io).bytes_done = (*io).bytes_done.wrapping_add(ret as usize);
        rust_rw_iter_save(addr_of_mut!((*d).iter), addr_of_mut!((*d).iter_state));
        if !io_rw_should_retry(req) { (*kiocb).ki_flags &= !IOCB_WAITQ; return -EAGAIN; }
        (*req).cqe.res = iter_count(addr_of!((*d).iter)) as _;
        ret = io_iter_do_read(rw, addr_of_mut!((*d).iter));
        if ret == -EIOCBQUEUED { return IOU_ISSUE_SKIP_COMPLETE; }
        (*kiocb).ki_flags &= !IOCB_WAITQ;
        iov_iter_restore(addr_of_mut!((*d).iter), addr_of_mut!((*d).iter_state));
        if ret <= 0 { break; }
    }
    ret
}
#[no_mangle]
pub unsafe extern "C" fn io_read(req: *mut io_kiocb, flags: u32) -> c_int {
    let mut sel: io_br_sel = zeroed();
    let ret = __io_read(req, &mut sel, flags);
    if ret >= 0 { return kiocb_done(req, ret as _, &mut sel, flags); }
    if (*req).flags & REQ_F_BUFFERS_COMMIT != 0 { rust_rw_recycle_kbuf(req, sel.buf_list, flags); }
    io_fixup_restart_res(ret as _) as c_int
}
#[no_mangle]
pub unsafe extern "C" fn io_read_mshot(req: *mut io_kiocb, flags: u32) -> c_int {
    let rw = rw(req);
    let mut sel: io_br_sel = zeroed();
    let mut cflags = 0;
    if !rust_rw_file_can_poll(req) { return -EBADFD; }
    (*rw).kiocb.ki_complete = None;
    let mut ret = __io_read(req, &mut sel, flags);
    if ret == -EAGAIN {
        if rust_rw_recycle_kbuf(req, sel.buf_list, flags) { (*rw).len = 0; }
        return IOU_RETRY;
    } else if ret <= 0 {
        rust_rw_recycle_kbuf(req, sel.buf_list, flags);
        if ret < 0 { ret = io_fixup_restart_res(ret as _) as _; rust_rw_set_fail(req); }
    } else if (*req).flags & REQ_F_APOLL_MULTISHOT == 0 {
        cflags = rust_rw_put_kbuf(req, ret, sel.buf_list);
    } else {
        cflags = rust_rw_put_kbuf(req, ret, sel.buf_list);
        (*rw).len = 0;
        if io_req_post_cqe(req, ret, cflags | IORING_CQE_F_MORE) {
            if flags & IO_URING_F_MULTISHOT != 0 { rust_rw_poll_multishot_retry(req); }
            return IOU_RETRY;
        }
    }
    rust_rw_set_res(req, ret, cflags);
    io_req_rw_cleanup(req, flags);
    IOU_COMPLETE
}

unsafe fn io_kiocb_start_write(req: *mut io_kiocb, kiocb: *mut kiocb) -> bool {
    if (*req).flags & REQ_F_ISREG == 0 { return true; }
    if (*kiocb).ki_flags & IOCB_NOWAIT == 0 { rust_rw_start_write(kiocb); return true; }
    let sb = (*(*(*kiocb).ki_filp).f_inode).i_sb;
    let ret = rust_rw_sb_start_write_trylock(sb);
    if ret { rust_rw_sb_writers_release(sb); }
    ret
}
unsafe fn io_write_ret_eagain(req: *mut io_kiocb, io: *mut io_async_rw, kiocb: *mut kiocb) -> c_int {
    let d = data(io);
    iov_iter_restore(addr_of_mut!((*d).iter), addr_of_mut!((*d).iter_state));
    io_meta_restore(io, kiocb);
    if (*kiocb).ki_flags & IOCB_WRITE != 0 { io_req_end_write(req); }
    -EAGAIN
}
#[no_mangle]
pub unsafe extern "C" fn io_write(req: *mut io_kiocb, flags: u32) -> c_int {
    let force_nonblock = flags & IO_URING_F_NONBLOCK != 0;
    let rw = rw(req);
    let io = io(req);
    let d = data(io);
    let kiocb = addr_of_mut!((*rw).kiocb);
    if (*req).flags & REQ_F_IMPORT_BUFFER != 0 {
        let ret = io_rw_import_reg_vec(req, io, ITER_SOURCE as _, flags);
        if ret != 0 { return ret; }
    }
    let ret = io_rw_init_file(req, FMODE_WRITE, WRITE as _);
    if ret != 0 { return ret; }
    (*req).cqe.res = iter_count(addr_of!((*d).iter)) as _;
    if force_nonblock {
        if !io_file_supports_nowait(req, EPOLLOUT) { return io_write_ret_eagain(req, io, kiocb); }
        if (*kiocb).ki_flags & IOCB_DIRECT == 0 &&
           (*(*file(req)).f_op).fop_flags & FOP_BUFFER_WASYNC == 0 && (*req).flags & REQ_F_ISREG != 0 {
            return io_write_ret_eagain(req, io, kiocb);
        }
        (*kiocb).ki_flags |= IOCB_NOWAIT;
    } else { (*kiocb).ki_flags &= !IOCB_NOWAIT; }
    let ppos = io_kiocb_update_pos(req);
    let ret = rw_verify_area(WRITE as _, file(req), ppos, (*req).cqe.res as _);
    if ret != 0 { return ret; }
    if !io_kiocb_start_write(req, kiocb) { return -EAGAIN; }
    (*kiocb).ki_flags |= IOCB_WRITE;
    let fops = (*file(req)).f_op;
    let mut ret = if let Some(write) = (*fops).write_iter { write(kiocb, addr_of_mut!((*d).iter)) }
        else if (*fops).write.is_some() { loop_rw_iter(WRITE as _, rw, addr_of_mut!((*d).iter)) }
        else { -EINVAL as c_long };
    if ret == -EOPNOTSUPP as c_long && (*kiocb).ki_flags & IOCB_NOWAIT != 0 { ret = -EAGAIN as _; }
    if ret == -EAGAIN as c_long && (*req).flags & REQ_F_NOWAIT != 0 {
        return kiocb_done(req, ret, null_mut(), flags);
    }
    if !force_nonblock || ret != -EAGAIN as c_long {
        if ret == -EAGAIN as c_long && (*req).flags & REQ_F_IOPOLL != 0 {
            return io_write_ret_eagain(req, io, kiocb);
        }
        if ret != (*req).cqe.res as c_long && ret >= 0 && need_complete_io(req) {
            rust_rw_trace_short_write((*req).ctx, (*kiocb).ki_pos.wrapping_sub(ret as _), (*req).cqe.res as _, ret as _);
            rust_rw_iter_save(addr_of_mut!((*d).iter), addr_of_mut!((*d).iter_state));
            (*io).bytes_done = (*io).bytes_done.wrapping_add(ret as usize);
            if (*kiocb).ki_flags & IOCB_WRITE != 0 { io_req_end_write(req); }
            return -EAGAIN;
        }
        kiocb_done(req, ret, null_mut(), flags)
    } else { io_write_ret_eagain(req, io, kiocb) }
}
#[no_mangle]
pub unsafe extern "C" fn io_read_fixed(req: *mut io_kiocb, flags: u32) -> c_int {
    let ret = io_init_rw_fixed(req, flags, ITER_DEST as _);
    if ret != 0 { return ret; }
    io_read(req, flags)
}
#[no_mangle]
pub unsafe extern "C" fn io_write_fixed(req: *mut io_kiocb, flags: u32) -> c_int {
    let ret = io_init_rw_fixed(req, flags, ITER_SOURCE as _);
    if ret != 0 { return ret; }
    io_write(req, flags)
}
#[no_mangle]
pub unsafe extern "C" fn io_rw_fail(req: *mut io_kiocb) {
    let res = io_fixup_rw_res(req, (*req).cqe.res as _);
    rust_rw_set_res(req, res, (*req).cqe.__bindgen_anon_1.flags);
}
unsafe fn io_uring_classic_poll(req: *mut io_kiocb, iob: *mut io_comp_batch, flags: u32) -> c_int {
    let fops = (*file(req)).f_op;
    if rust_rw_is_uring_cmd(req) {
        ((*fops).uring_cmd_iopoll.unwrap_unchecked())(req.cast::<io_uring_cmd>(), iob, flags)
    } else { ((*fops).iopoll.unwrap_unchecked())(addr_of_mut!((*rw(req)).kiocb), iob, flags) }
}
unsafe fn io_hybrid_iopoll_delay(ctx: *mut io_ring_ctx, req: *mut io_kiocb) -> u64 {
    if (*req).flags & REQ_F_IOPOLL_STATE != 0 { return 0; }
    if (*ctx).__bindgen_anon_2.hybrid_poll_time == i64::MAX as u64 { return 0; }
    let sleep_time = (*ctx).__bindgen_anon_2.hybrid_poll_time / 2;
    (*req).flags |= REQ_F_IOPOLL_STATE;
    let mut timer: hrtimer_sleeper = zeroed();
    rust_rw_timer_setup(&mut timer);
    rust_rw_timer_expires(&mut timer, sleep_time);
    rust_rw_set_interruptible();
    rust_rw_timer_start(&mut timer);
    if !timer.task.is_null() { io_schedule(); }
    hrtimer_cancel(&mut timer.timer);
    rust_rw_set_running();
    rust_rw_timer_destroy(&mut timer);
    sleep_time
}
unsafe fn io_uring_hybrid_poll(req: *mut io_kiocb, iob: *mut io_comp_batch, flags: u32) -> c_int {
    let ctx = (*req).ctx;
    let start = read_volatile(addr_of!((*req).__bindgen_anon_4.iopoll_start));
    let sleep = io_hybrid_iopoll_delay(ctx, req);
    let ret = io_uring_classic_poll(req, iob, flags);
    let runtime = rust_rw_ktime_get_ns().wrapping_sub(start).wrapping_sub(sleep);
    if (*ctx).__bindgen_anon_2.hybrid_poll_time > runtime { (*ctx).__bindgen_anon_2.hybrid_poll_time = runtime; }
    ret
}
#[no_mangle]
pub unsafe extern "C" fn io_do_iopoll(ctx: *mut io_ring_ctx, force_nonspin: bool) -> c_int {
    let mut flags = 0;
    let mut iob: io_comp_batch = zeroed();
    iob.poll_ctx = ctx.cast();
    if (*ctx).__bindgen_anon_2.poll_multi_queue || force_nonspin { flags |= BLK_POLL_ONESHOT; }
    let head = addr_of_mut!((*ctx).__bindgen_anon_2.iopoll_list);
    let mut node = (*head).next;
    while node != head {
        let req = node.cast::<u8>().sub(offset_of!(io_kiocb, __bindgen_anon_5)).cast::<io_kiocb>();
        if read_volatile(addr_of!((*req).iopoll_completed)) != 0 { break; }
        let ret = if ctx_flags(ctx) & IORING_SETUP_HYBRID_IOPOLL != 0 {
            io_uring_hybrid_poll(req, &mut iob, flags)
        } else { io_uring_classic_poll(req, &mut iob, flags) };
        if ret < 0 { return ret; }
        if ret != 0 { flags |= BLK_POLL_ONESHOT; }
        if !rust_rw_rq_list_empty(&mut iob) || read_volatile(addr_of!((*req).iopoll_completed)) != 0 { break; }
        node = (*node).next;
    }
    if !rust_rw_rq_list_empty(&mut iob) { (iob.complete.unwrap_unchecked())(&mut iob); }
    let mut nr_events: c_int = 0;
    node = (*head).next;
    while node != head {
        let req = node.cast::<u8>().sub(offset_of!(io_kiocb, __bindgen_anon_5)).cast::<io_kiocb>();
        let next = (*node).next;
        if rust_rw_load_complete(req) {
            rust_rw_list_del(node);
            rust_rw_wq_add_tail(req, ctx);
            nr_events = nr_events.wrapping_add(1);
            (*req).cqe.__bindgen_anon_1.flags = rust_rw_put_kbuf(req, (*req).cqe.res.max(0), null_mut());
            if !rust_rw_is_uring_cmd(req) { io_req_rw_cleanup(req, 0); }
        }
        node = next;
    }
    if nr_events != 0 { __io_submit_flush_completions(ctx); }
    nr_events
}
#[no_mangle]
pub unsafe extern "C" fn io_rw_cache_free(entry: *const c_void) {
    let io = entry as *mut io_async_rw;
    io_vec_free(addr_of_mut!((*io).vec));
    kfree(entry);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
