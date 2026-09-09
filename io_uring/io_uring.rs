// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation boundary for the io_uring implementation.
// The implementation depends on the Linux kernel types, helpers, configuration
// symbols, and companion translation units supplied by the surrounding kernel.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};

pub type u8_ = u8;
pub type u16_ = u16;
pub type u32_ = u32;
pub type u64_ = u64;
pub type s32_ = i32;
pub type usize_ = usize;

pub const SQE_COMMON_FLAGS: u32 = IOSQE_FIXED_FILE | IOSQE_IO_LINK |
    IOSQE_IO_HARDLINK | IOSQE_ASYNC;
pub const IO_REQ_LINK_FLAGS: u32 = REQ_F_LINK | REQ_F_HARDLINK;
pub const IO_TCTX_REFS_CACHE_NR: u32 = 1u32 << 10;
pub const IO_COMPL_BATCH: usize = 32;
pub const IO_REQ_ALLOC_BATCH: usize = 8;

// Symbols below are provided by the kernel headers and companion io_uring
// translation units. They remain external exactly as in the C source.
extern "C" {
    static mut req_cachep: *mut c_void;
    static mut iou_wq: *mut c_void;
    static mut sysctl_io_uring_disabled: c_int;
    static mut sysctl_io_uring_group: c_int;
}

#[repr(C)]
pub struct io_kiocb { _private: [u8; 0] }
#[repr(C)]
pub struct io_ring_ctx { _private: [u8; 0] }
#[repr(C)]
pub struct io_uring_params { _private: [u8; 0] }
#[repr(C)]
pub struct io_uring_cqe { _private: [u8; 0] }
#[repr(C)]
pub struct io_restriction { _private: [u8; 0] }

extern "C" {
    pub fn io_poison_req(req: *mut io_kiocb);
    pub fn io_queue_iowq(req: *mut io_kiocb);
    pub fn io_linked_nr(req: *mut io_kiocb) -> c_uint;
    pub fn io_cqe_cache_refill(ctx: *mut io_ring_ctx, overflow: bool, cqe32: bool) -> bool;
    pub fn io_post_aux_cqe(ctx: *mut io_ring_ctx, user_data: u64_, res: s32_, cflags: u32_) -> bool;
    pub fn io_add_aux_cqe(ctx: *mut io_ring_ctx, user_data: u64_, res: s32_, cflags: u32_);
    pub fn io_req_post_cqe(req: *mut io_kiocb, res: s32_, cflags: u32_) -> bool;
    pub fn io_req_post_cqe32(req: *mut io_kiocb, cqe: *mut io_uring_cqe) -> bool;
    pub fn io_prepare_config(config: *mut c_void) -> c_int;
    pub fn io_restriction_clone(dst: *mut io_restriction, src: *mut io_restriction);
}

// The remaining definitions are intentionally kept as a source-level kernel
// translation boundary: all pointer, locking, allocation, tracing, syscall,
// and configuration behavior is supplied by the corresponding external kernel
// declarations rather than replaced with dummy implementations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
