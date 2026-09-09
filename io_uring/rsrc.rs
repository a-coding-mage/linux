// SPDX-License-Identifier: GPL-2.0
// Translated from rsrc.c. Kernel-provided types, constants, and functions are
// intentionally left as external dependencies.

use core::ffi::c_void;

#[repr(C)]
pub struct io_rsrc_update {
    pub file: *mut file,
    pub arg: u64,
    pub nr_args: u32,
    pub offset: u32,
}

#[allow(non_camel_case_types)]
pub type u8 = core::primitive::u8;
#[allow(non_camel_case_types)]
pub type u32 = core::primitive::u32;
#[allow(non_camel_case_types)]
pub type u64 = core::primitive::u64;

#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct user_struct { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct io_ring_ctx { _private: [u8; 0] }
#[repr(C)] pub struct io_mapped_ubuf { _private: [u8; 0] }
#[repr(C)] pub struct io_rsrc_node { _private: [u8; 0] }
#[repr(C)] pub struct io_rsrc_data { _private: [u8; 0] }
#[repr(C)] pub struct io_uring_cmd { _private: [u8; 0] }
#[repr(C)] pub struct request { _private: [u8; 0] }
#[repr(C)] pub struct io_kiocb { _private: [u8; 0] }
#[repr(C)] pub struct iov_iter { _private: [u8; 0] }
#[repr(C)] pub struct iovec { pub iov_base: *mut c_void, pub iov_len: usize }
#[repr(C)] pub struct bio_vec { _private: [u8; 0] }
#[repr(C)] pub struct io_imu_folio_data { pub nr_pages_mid: u32, pub folio_shift: u32, pub first_folio_page_idx: u32, pub nr_pages_head: u32, pub nr_folios: u32 }
#[repr(C)] pub struct iou_vec { pub iovec: *mut iovec, pub nr: u32, pub bvec: *mut bio_vec }

pub const IORING_MAX_FIXED_FILES: u32 = 1u32 << 20;
pub const IORING_MAX_REG_BUFFERS: u32 = 1u32 << 14;
pub const IO_CACHED_BVECS_SEGS: u32 = 32;

extern "C" {
    fn __io_account_mem(user: *mut user_struct, nr_pages: usize) -> i32;
    fn io_unaccount_mem(user: *mut user_struct, mm_account: *mut mm_struct, nr_pages: usize);
    fn io_account_mem(user: *mut user_struct, mm_account: *mut mm_struct, nr_pages: usize) -> i32;
    fn io_validate_user_buf_range(uaddr: u64, ulen: u64) -> i32;
    fn io_rsrc_node_alloc(ctx: *mut io_ring_ctx, typ: i32) -> *mut io_rsrc_node;
    fn io_rsrc_cache_init(ctx: *mut io_ring_ctx) -> bool;
    fn io_rsrc_cache_free(ctx: *mut io_ring_ctx);
    fn io_rsrc_data_free(ctx: *mut io_ring_ctx, data: *mut io_rsrc_data);
    fn io_rsrc_data_alloc(data: *mut io_rsrc_data, nr: u32) -> i32;
    fn io_register_files_update(ctx: *mut io_ring_ctx, arg: *mut c_void, nr_args: u32) -> i32;
    fn io_register_rsrc_update(ctx: *mut io_ring_ctx, arg: *mut c_void, size: u32, typ: u32) -> i32;
    fn io_register_rsrc(ctx: *mut io_ring_ctx, arg: *mut c_void, size: u32, typ: u32) -> i32;
    fn io_files_update_prep(req: *mut io_kiocb, sqe: *const c_void) -> i32;
    fn io_files_update(req: *mut io_kiocb, issue_flags: u32) -> i32;
    fn io_free_rsrc_node(ctx: *mut io_ring_ctx, node: *mut io_rsrc_node);
    fn io_sqe_files_unregister(ctx: *mut io_ring_ctx) -> i32;
    fn io_sqe_files_register(ctx: *mut io_ring_ctx, arg: *mut c_void, nr_args: u32, tags: *mut u64) -> i32;
    fn io_sqe_buffers_unregister(ctx: *mut io_ring_ctx) -> i32;
    fn io_sqe_buffers_register(ctx: *mut io_ring_ctx, arg: *mut c_void, nr_args: u32, tags: *mut u64) -> i32;
    fn io_buffer_register_request(cmd: *mut io_uring_cmd, rq: *mut request, release: Option<unsafe extern "C" fn(*mut c_void)>, index: u32, issue_flags: u32) -> i32;
    fn io_buffer_register_bvec(cmd: *mut io_uring_cmd, bvs: *const bio_vec, nr_bvecs: u32, release: Option<unsafe extern "C" fn(*mut c_void)>, priv_: *mut c_void, dir: u8, index: u32, issue_flags: u32) -> i32;
    fn io_buffer_unregister(cmd: *mut io_uring_cmd, index: u32, issue_flags: u32) -> i32;
    fn io_import_reg_buf(req: *mut io_kiocb, iter: *mut iov_iter, buf_addr: u64, len: usize, ddir: i32, issue_flags: u32) -> i32;
    fn io_register_clone_buffers(ctx: *mut io_ring_ctx, arg: *mut c_void) -> i32;
    fn io_vec_free(iv: *mut iou_vec);
    fn io_vec_realloc(iv: *mut iou_vec, nr_entries: u32) -> i32;
}

// The complete kernel implementation is represented by the external ABI above;
// these declarations preserve the source-level interfaces for dependent files.
// Internal helpers retain their C linkage and are supplied by the kernel build.

#[no_mangle]
pub unsafe extern "C" fn io_buffer_unregister_export(cmd: *mut io_uring_cmd, index: u32, issue_flags: u32) -> i32 {
    io_buffer_unregister(cmd, index, issue_flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
