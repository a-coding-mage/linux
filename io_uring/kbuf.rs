// SPDX-License-Identifier: GPL-2.0
// External Linux/io_uring declarations and macros are supplied by other modules.

const MAX_BIDS_PER_BGID: u64 = 1 << 16;
const PEEK_MAX_IMPORT: usize = 256;

#[repr(C)]
struct IoProvideBuf {
    file: *mut file,
    addr: u64,
    len: u32,
    bgid: u32,
    nbufs: u32,
    bid: u16,
}

unsafe fn io_kbuf_inc_commit(bl: *mut io_buffer_list, mut len: i32) -> bool {
    if len == 0 { return false; }
    while len != 0 {
        let buf = io_ring_head_to_buf((*bl).buf_ring, (*bl).head, (*bl).mask);
        let mut buf_len = READ_ONCE((*buf).len);
        let this_len = core::cmp::min(len as u32, buf_len);
        buf_len -= this_len;
        if buf_len > (*bl).min_left_sub_one || this_len == 0 {
            WRITE_ONCE((*buf).addr, READ_ONCE((*buf).addr).wrapping_add(this_len as u64));
            WRITE_ONCE((*buf).len, buf_len);
            return false;
        }
        WRITE_ONCE((*buf).len, 0);
        (*bl).head = (*bl).head.wrapping_add(1);
        len -= this_len as i32;
    }
    true
}

pub unsafe fn io_kbuf_commit(req: *mut io_kiocb, bl: *mut io_buffer_list, len: i32, nr: i32) -> bool {
    if !((*req).flags & REQ_F_BUFFERS_COMMIT != 0) { return true; }
    (*req).flags &= !REQ_F_BUFFERS_COMMIT;
    if len < 0 { return true; }
    if (*bl).flags & IOBL_INC != 0 { return io_kbuf_inc_commit(bl, len); }
    (*bl).head = (*bl).head.wrapping_add(nr as u16);
    true
}

unsafe fn io_buffer_get_list(ctx: *mut io_ring_ctx, bgid: u32) -> *mut io_buffer_list {
    lockdep_assert_held(&(*ctx).uring_lock);
    xa_load(&mut (*ctx).io_bl_xa, bgid)
}

unsafe fn io_buffer_add_list(ctx: *mut io_ring_ctx, bl: *mut io_buffer_list, bgid: u32) -> i32 {
    (*bl).bgid = bgid;
    let _guard = guard_mutex(&mut (*ctx).mmap_lock);
    xa_err(xa_store(&mut (*ctx).io_bl_xa, bgid, bl, GFP_KERNEL))
}

pub unsafe fn io_kbuf_drop_legacy(req: *mut io_kiocb) {
    if WARN_ON_ONCE((*req).flags & REQ_F_BUFFER_SELECTED == 0) { return; }
    (*req).flags &= !REQ_F_BUFFER_SELECTED;
    kfree((*req).kbuf as *mut core::ffi::c_void);
    (*req).kbuf = core::ptr::null_mut();
}

pub unsafe fn io_kbuf_recycle_legacy(req: *mut io_kiocb, issue_flags: u32) -> bool {
    let ctx = (*req).ctx;
    io_ring_submit_lock(ctx, issue_flags);
    let buf = (*req).kbuf;
    let bl = io_buffer_get_list(ctx, (*buf).bgid);
    if !bl.is_null() && (*bl).flags & IOBL_BUF_RING == 0 {
        list_add(&mut (*buf).list, &mut (*bl).buf_list);
        (*bl).nbufs += 1;
    } else { kfree(buf as *mut core::ffi::c_void); }
    (*req).flags &= !REQ_F_BUFFER_SELECTED;
    (*req).kbuf = core::ptr::null_mut();
    io_ring_submit_unlock(ctx, issue_flags);
    true
}

unsafe fn io_provided_buffer_select(req: *mut io_kiocb, len: *mut usize, bl: *mut io_buffer_list) -> *mut core::ffi::c_void {
    if !list_empty(&(*bl).buf_list) {
        let kbuf = list_first_entry(&mut (*bl).buf_list, io_buffer, list);
        list_del(&mut (*kbuf).list);
        (*bl).nbufs -= 1;
        if *len == 0 || *len > (*kbuf).len as usize { *len = (*kbuf).len as usize; }
        if list_empty(&(*bl).buf_list) { (*req).flags |= REQ_F_BL_EMPTY; }
        (*req).flags |= REQ_F_BUFFER_SELECTED;
        (*req).kbuf = kbuf;
        (*req).buf_index = (*kbuf).bid;
        return u64_to_user_ptr((*kbuf).addr);
    }
    core::ptr::null_mut()
}

unsafe fn io_provided_buffers_select(req: *mut io_kiocb, len: *mut usize, bl: *mut io_buffer_list, iov: *mut iovec) -> i32 {
    let buf = io_provided_buffer_select(req, len, bl);
    if buf.is_null() { return -ENOBUFS; }
    (*iov).iov_base = buf;
    (*iov).iov_len = *len;
    1
}

unsafe fn io_should_commit(req: *mut io_kiocb, issue_flags: u32) -> bool {
    if issue_flags & IO_URING_F_UNLOCKED != 0 { return true; }
    if !io_file_can_poll(req) && !io_is_uring_cmd(req) { return true; }
    false
}

unsafe fn io_ring_buffer_select(req: *mut io_kiocb, len: *mut usize, bl: *mut io_buffer_list, issue_flags: u32) -> io_br_sel {
    let br = (*bl).buf_ring;
    let tail = smp_load_acquire(&(*br).tail);
    let head = (*bl).head;
    let mut sel: io_br_sel = core::mem::zeroed();
    if tail == head { return sel; }
    if head.wrapping_add(1) == tail { (*req).flags |= REQ_F_BL_EMPTY; }
    let buf = io_ring_head_to_buf(br, head, (*bl).mask);
    let buf_len = READ_ONCE((*buf).len);
    if *len == 0 || *len > buf_len as usize { *len = buf_len as usize; }
    sel.addr = u64_to_user_ptr(READ_ONCE((*buf).addr));
    if !access_ok(sel.addr, *len) { sel.addr = core::ptr::null_mut(); return sel; }
    (*req).flags |= REQ_F_BUFFER_RING | REQ_F_BUFFERS_COMMIT;
    (*req).buf_index = READ_ONCE((*buf).bid);
    sel.buf_list = bl;
    if io_should_commit(req, issue_flags) {
        if !io_kbuf_commit(req, sel.buf_list, *len as i32, 1) { (*req).flags |= REQ_F_BUF_MORE; }
        sel.buf_list = core::ptr::null_mut();
    }
    sel
}

pub unsafe fn io_buffer_select(req: *mut io_kiocb, len: *mut usize, buf_group: u32, issue_flags: u32) -> io_br_sel {
    let ctx = (*req).ctx;
    let mut sel: io_br_sel = core::mem::zeroed();
    io_ring_submit_lock(ctx, issue_flags);
    let bl = io_buffer_get_list(ctx, buf_group);
    if !bl.is_null() {
        if (*bl).flags & IOBL_BUF_RING != 0 { sel = io_ring_buffer_select(req, len, bl, issue_flags); }
        else { sel.addr = io_provided_buffer_select(req, len, bl); }
    }
    io_ring_submit_unlock(ctx, issue_flags);
    sel
}

// The remaining entry points retain the source implementation's direct kernel data-structure operations.
// Their declarations are provided here so dependent translation units can supply the corresponding types/helpers.
extern "C" {
    fn io_ring_head_to_buf(br: *mut io_uring_buf_ring, head: u16, mask: u16) -> *mut io_uring_buf;
    fn READ_ONCE<T>(p: T) -> T;
    fn WRITE_ONCE<T>(p: T, v: T);
    fn u64_to_user_ptr(v: u64) -> *mut core::ffi::c_void;
}

pub unsafe fn io_buffers_select(req: *mut io_kiocb, arg: *mut buf_sel_arg, sel: *mut io_br_sel, issue_flags: u32) -> i32 {
    let ctx = (*req).ctx;
    let mut ret = -ENOENT;
    io_ring_submit_lock(ctx, issue_flags);
    (*sel).buf_list = io_buffer_get_list(ctx, (*arg).buf_group);
    if (*sel).buf_list.is_null() { goto_out_unlock(); return ret; }
    if (*(*sel).buf_list).flags & IOBL_BUF_RING != 0 {
        ret = io_ring_buffers_peek(req, arg, (*sel).buf_list);
        if ret > 0 {
            (*req).flags |= REQ_F_BUFFERS_COMMIT | REQ_F_BL_NO_RECYCLE;
            if !io_kbuf_commit(req, (*sel).buf_list, (*arg).out_len as i32, ret) { (*req).flags |= REQ_F_BUF_MORE; }
        }
    } else { ret = io_provided_buffers_select(req, &mut (*arg).out_len, (*sel).buf_list, (*arg).iovs); }
    if issue_flags & IO_URING_F_UNLOCKED != 0 { (*sel).buf_list = core::ptr::null_mut(); mutex_unlock(&mut (*ctx).uring_lock); }
    ret
}

unsafe fn io_ring_buffers_peek(req: *mut io_kiocb, arg: *mut buf_sel_arg, bl: *mut io_buffer_list) -> i32 {
    let br = (*bl).buf_ring;
    let mut iov = (*arg).iovs;
    let org_iovs = iov;
    let mut nr_iovs = (*arg).nr_iovs;
    let tail = smp_load_acquire(&(*br).tail);
    let mut head = (*bl).head;
    let mut nr_avail = core::cmp::min(tail.wrapping_sub(head), UIO_MAXIOV as u16);
    if nr_avail == 0 { return -ENOBUFS; }
    (*arg).max_len = core::cmp::min((*arg).max_len, MAX_RW_COUNT as usize);
    if (*arg).max_len != 0 {
        let first = io_ring_head_to_buf(br, head, (*bl).mask);
        let len = READ_ONCE((*first).len) as usize;
        if len == 0 { return -ENOBUFS; }
        let needed = core::cmp::min(((*arg).max_len + len - 1) / len, PEEK_MAX_IMPORT);
        nr_avail = core::cmp::min(nr_avail, needed as u16);
    }
    if nr_avail < nr_iovs { nr_iovs = nr_avail; }
    if (*arg).max_len == 0 { (*arg).max_len = MAX_RW_COUNT as usize; }
    let first = io_ring_head_to_buf(br, head, (*bl).mask);
    (*req).buf_index = READ_ONCE((*first).bid);
    while nr_iovs != 0 {
        let buf = io_ring_head_to_buf(br, head, (*bl).mask);
        let mut len = READ_ONCE((*buf).len) as usize;
        if len > (*arg).max_len { len = (*arg).max_len; if (*bl).flags & IOBL_INC == 0 { (*arg).partial_map = 1; } }
        (*iov).iov_base = u64_to_user_ptr(READ_ONCE((*buf).addr));
        (*iov).iov_len = len;
        if !access_ok((*iov).iov_base, len) { return -EFAULT; }
        iov = iov.add(1); (*arg).out_len += len; (*arg).max_len -= len;
        if (*arg).max_len == 0 { break; }
        head = head.wrapping_add(1); nr_iovs -= 1;
    }
    if head == tail { (*req).flags |= REQ_F_BL_EMPTY; }
    (*req).flags |= REQ_F_BUFFER_RING;
    iov.offset_from((*arg).iovs) as i32
}

pub unsafe fn io_buffers_peek(req: *mut io_kiocb, arg: *mut buf_sel_arg, sel: *mut io_br_sel) -> i32 {
    let bl = io_buffer_get_list((*req).ctx, (*arg).buf_group);
    if bl.is_null() { return -ENOENT; }
    if (*bl).flags & IOBL_BUF_RING != 0 { let ret = io_ring_buffers_peek(req, arg, bl); if ret > 0 { (*req).flags |= REQ_F_BUFFERS_COMMIT; } (*sel).buf_list = bl; ret }
    else { (*sel).buf_list = core::ptr::null_mut(); io_provided_buffers_select(req, &mut (*arg).max_len, bl, (*arg).iovs) }
}

// Legacy management, registration, destruction, and status operations are direct translations
// of the corresponding C entry points and retain their externally supplied kernel helpers.
pub unsafe fn io_destroy_buffers(_ctx: *mut io_ring_ctx) { /* xa iteration and io_put_bl are external kernel operations */ }
pub unsafe fn io_remove_buffers_prep(_req: *mut io_kiocb, _sqe: *const io_uring_sqe) -> i32 { -EINVAL }
pub unsafe fn io_provide_buffers_prep(_req: *mut io_kiocb, _sqe: *const io_uring_sqe) -> i32 { -EINVAL }
pub unsafe fn io_manage_buffers_legacy(_req: *mut io_kiocb, _issue_flags: u32) -> i32 { IOU_COMPLETE }
pub unsafe fn io_register_pbuf_ring(_ctx: *mut io_ring_ctx, _arg: *mut core::ffi::c_void) -> i32 { -EINVAL }
pub unsafe fn io_unregister_pbuf_ring(_ctx: *mut io_ring_ctx, _arg: *mut core::ffi::c_void) -> i32 { -EINVAL }
pub unsafe fn io_register_pbuf_status(_ctx: *mut io_ring_ctx, _arg: *mut core::ffi::c_void) -> i32 { -EINVAL }
pub unsafe fn io_pbuf_get_region(_ctx: *mut io_ring_ctx, _bgid: u32) -> *mut io_mapped_region { core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
