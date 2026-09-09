// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_ring_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_task {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_tctx_node {
    pub ctx_node: list_head,
    pub task: *mut task_struct,
    pub ctx: *mut io_ring_ctx,
}

unsafe extern "C" {
    pub fn io_uring_alloc_task_context(
        task: *mut task_struct,
        ctx: *mut io_ring_ctx,
    ) -> *mut io_uring_task;
    pub fn io_uring_del_tctx_node(index: ::core::ffi::c_ulong);
    pub fn __io_uring_add_tctx_node(ctx: *mut io_ring_ctx) -> ::core::ffi::c_int;
    pub fn __io_uring_add_tctx_node_from_submit(ctx: *mut io_ring_ctx) -> ::core::ffi::c_int;
    pub fn io_uring_clean_tctx(tctx: *mut io_uring_task);
    pub fn io_uring_free_tctx(tsk: *mut task_struct);

    pub fn io_uring_unreg_ringfd();
    pub fn io_ringfd_register(
        ctx: *mut io_ring_ctx,
        arg: *mut ::core::ffi::c_void,
        nr_args: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn io_ringfd_unregister(
        ctx: *mut io_ring_ctx,
        arg: *mut ::core::ffi::c_void,
        nr_args: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub static mut current: *mut task_struct;
}

#[repr(C)]
struct task_struct_io_uring_view {
    _prefix: [u8; 0],
    io_uring: *mut io_uring_task,
}

#[inline]
pub unsafe fn io_uring_add_tctx_node(ctx: *mut io_ring_ctx) -> ::core::ffi::c_int {
    let tctx = (*(current as *mut task_struct_io_uring_view)).io_uring;

    if !tctx.is_null() && (*(tctx as *mut io_uring_task_last_view)).last == ctx {
        return 0;
    }

    __io_uring_add_tctx_node_from_submit(ctx)
}

#[repr(C)]
struct io_uring_task_last_view {
    last: *mut io_ring_ctx,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
