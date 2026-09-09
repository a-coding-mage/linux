// SPDX-License-Identifier: GPL-2.0

// Translated from kbuf.h. The Linux io_uring types and flag constants are
// supplied by the surrounding crate.

use core::ffi::c_void;

pub const IOBL_BUF_RING: i32 = 1;
pub const IOBL_INC: i32 = 2;

#[repr(C)]
pub union io_buffer_list__bindgen_ty_1 {
    pub buf_list: crate::list_head,
    pub buf_ring: *mut crate::io_uring_buf_ring,
}

#[repr(C)]
pub struct io_buffer_list {
    pub buf_list_or_ring: io_buffer_list__bindgen_ty_1,
    pub nbufs: i32,
    pub bgid: u16,
    pub head: u16,
    pub mask: u16,
    pub flags: u16,
    pub min_left_sub_one: u32,
    pub region: crate::io_mapped_region,
}

#[repr(C)]
pub struct io_buffer {
    pub list: crate::list_head,
    pub addr: u64,
    pub len: u32,
    pub bid: u16,
    pub bgid: u16,
}

pub const KBUF_MODE_EXPAND: i32 = 1;
pub const KBUF_MODE_FREE: i32 = 2;

#[repr(C)]
pub struct buf_sel_arg {
    pub iovs: *mut crate::iovec,
    pub out_len: usize,
    pub max_len: usize,
    pub nr_iovs: u16,
    pub mode: u16,
    pub buf_group: u16,
    pub partial_map: u16,
}

extern "C" {
    pub fn io_buffer_select(
        req: *mut crate::io_kiocb,
        len: *mut usize,
        buf_group: u32,
        issue_flags: u32,
    ) -> crate::io_br_sel;
    pub fn io_buffers_select(
        req: *mut crate::io_kiocb,
        arg: *mut buf_sel_arg,
        sel: *mut crate::io_br_sel,
        issue_flags: u32,
    ) -> i32;
    pub fn io_buffers_peek(
        req: *mut crate::io_kiocb,
        arg: *mut buf_sel_arg,
        sel: *mut crate::io_br_sel,
    ) -> i32;
    pub fn io_destroy_buffers(ctx: *mut crate::io_ring_ctx);

    pub fn io_remove_buffers_prep(
        req: *mut crate::io_kiocb,
        sqe: *const crate::io_uring_sqe,
    ) -> i32;
    pub fn io_provide_buffers_prep(
        req: *mut crate::io_kiocb,
        sqe: *const crate::io_uring_sqe,
    ) -> i32;
    pub fn io_manage_buffers_legacy(req: *mut crate::io_kiocb, issue_flags: u32) -> i32;

    pub fn io_register_pbuf_ring(ctx: *mut crate::io_ring_ctx, arg: *mut c_void) -> i32;
    pub fn io_unregister_pbuf_ring(ctx: *mut crate::io_ring_ctx, arg: *mut c_void) -> i32;
    pub fn io_register_pbuf_status(ctx: *mut crate::io_ring_ctx, arg: *mut c_void) -> i32;

    pub fn io_kbuf_recycle_legacy(req: *mut crate::io_kiocb, issue_flags: u32) -> bool;
    pub fn io_kbuf_drop_legacy(req: *mut crate::io_kiocb);

    pub fn __io_put_kbufs(
        req: *mut crate::io_kiocb,
        bl: *mut io_buffer_list,
        len: i32,
        nbufs: i32,
    ) -> u32;
    pub fn io_kbuf_commit(
        req: *mut crate::io_kiocb,
        bl: *mut io_buffer_list,
        len: i32,
        nr: i32,
    ) -> bool;

    pub fn io_pbuf_get_region(
        ctx: *mut crate::io_ring_ctx,
        bgid: u32,
    ) -> *mut crate::io_mapped_region;
}

#[inline]
pub unsafe fn io_kbuf_recycle_ring(
    req: *mut crate::io_kiocb,
    bl: *mut io_buffer_list,
) -> bool {
    if !bl.is_null() {
        (*req).flags &= !(crate::REQ_F_BUFFER_RING | crate::REQ_F_BUFFERS_COMMIT);
        return true;
    }
    false
}

#[inline]
pub unsafe fn io_do_buffer_select(req: *mut crate::io_kiocb) -> bool {
    if (*req).flags & crate::REQ_F_BUFFER_SELECT == 0 {
        return false;
    }
    (*req).flags & (crate::REQ_F_BUFFER_SELECTED | crate::REQ_F_BUFFER_RING) == 0
}

#[inline]
pub unsafe fn io_kbuf_recycle(
    req: *mut crate::io_kiocb,
    bl: *mut io_buffer_list,
    issue_flags: u32,
) -> bool {
    if (*req).flags & crate::REQ_F_BL_NO_RECYCLE != 0 {
        return false;
    }
    if (*req).flags & crate::REQ_F_BUFFER_RING != 0 {
        return io_kbuf_recycle_ring(req, bl);
    }
    if (*req).flags & crate::REQ_F_BUFFER_SELECTED != 0 {
        return io_kbuf_recycle_legacy(req, issue_flags);
    }
    false
}

#[inline]
pub unsafe fn io_put_kbuf(
    req: *mut crate::io_kiocb,
    len: i32,
    bl: *mut io_buffer_list,
) -> u32 {
    if (*req).flags & (crate::REQ_F_BUFFER_RING | crate::REQ_F_BUFFER_SELECTED) == 0 {
        return 0;
    }
    __io_put_kbufs(req, bl, len, 1)
}

#[inline]
pub unsafe fn io_put_kbufs(
    req: *mut crate::io_kiocb,
    len: i32,
    bl: *mut io_buffer_list,
    nbufs: i32,
) -> u32 {
    if (*req).flags & (crate::REQ_F_BUFFER_RING | crate::REQ_F_BUFFER_SELECTED) == 0 {
        return 0;
    }
    __io_put_kbufs(req, bl, len, nbufs)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
