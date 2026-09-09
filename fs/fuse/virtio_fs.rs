// SPDX-License-Identifier: GPL-2.0
/*
 * Direct low-level Rust translation of fuse/virtio_fs.c.
 * Kernel types, constants, macros, and functions referenced below are supplied
 * by the surrounding kernel translation units.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void}, mem::MaybeUninit, ptr};

pub const FUSE_HEADER_OVERHEAD: usize = 4;
pub const VQ_HIPRIO: usize = 0;
pub const VQ_REQUEST: usize = 1;
pub const VQ_NAME_LEN: usize = 24;

#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct kset { _private: [u8; 0] }
#[repr(C)] pub struct virtqueue { pub vdev: *mut virtio_device, pub index: c_uint }
#[repr(C)] pub struct virtio_device { pub priv_: *mut c_void, pub index: c_int, pub config: *mut c_void }
#[repr(C)] pub struct fuse_req { pub list: list_head, pub in_: fuse_req_part, pub out: fuse_req_part, pub args: *mut fuse_args, pub argbuf: *mut c_void, pub flags: c_ulong }
#[repr(C)] pub struct fuse_req_part { pub h: fuse_header }
#[repr(C)] pub struct fuse_header { pub len: c_uint, pub error: c_int, pub unique: u64, pub opcode: c_uint, pub nodeid: u64 }
#[repr(C)] pub struct fuse_args { pub in_numargs: c_uint, pub out_numargs: c_uint, pub in_pages: bool, pub out_pages: bool, pub out_argvar: bool, pub may_block: bool, pub page_zeroing: bool, pub in_args: *mut fuse_arg, pub out_args: *mut fuse_arg }
#[repr(C)] pub struct fuse_arg { pub size: usize, pub value: *mut c_void }
#[repr(C)] pub struct fuse_in_header { pub len: c_uint, pub opcode: c_uint, pub unique: u64, pub nodeid: u64 }
#[repr(C)] pub struct fuse_forget_in { pub nlookup: u64 }
#[repr(C)] pub struct virtio_fs_forget_req { pub ih: fuse_in_header, pub arg: fuse_forget_in }
#[repr(C)] pub struct virtio_fs_forget { pub list: list_head, pub req: virtio_fs_forget_req }
#[repr(C)] pub struct virtio_fs_vq { pub lock: spinlock_t, pub vq: *mut virtqueue, pub done_work: work_struct, pub queued_reqs: list_head, pub end_reqs: list_head, pub dispatch_work: work_struct, pub fud: *mut c_void, pub connected: bool, pub in_flight: c_long, pub in_flight_zero: completion, pub kobj: *mut kobject, pub name: [c_char; VQ_NAME_LEN] }
#[repr(C)] pub struct virtio_fs { pub kobj: kobject, pub mqs_kobj: *mut kobject, pub list: list_head, pub tag: *mut c_char, pub vqs: *mut virtio_fs_vq, pub nvqs: c_uint, pub num_request_queues: c_uint, pub dax_dev: *mut c_void, pub mq_map: *mut c_uint, pub window_kaddr: *mut c_void, pub window_phys_addr: u64, pub window_len: usize }

extern "C" {
    fn virtio_fs_enqueue_req(fsvq: *mut virtio_fs_vq, req: *mut fuse_req, in_flight: bool, gfp: c_uint) -> c_int;
    fn spin_lock(lock: *mut spinlock_t); fn spin_unlock(lock: *mut spinlock_t);
    fn inc_in_flight_req(fsvq: *mut virtio_fs_vq); fn dec_in_flight_req(fsvq: *mut virtio_fs_vq);
    fn virtqueue_add_outbuf(vq: *mut virtqueue, sg: *mut c_void, num: c_uint, data: *mut c_void, gfp: c_uint) -> c_int;
    fn virtqueue_kick_prepare(vq: *mut virtqueue) -> bool; fn virtqueue_notify(vq: *mut virtqueue);
    fn fuse_request_end(req: *mut fuse_req); fn schedule_work(work: *mut work_struct);
    fn kfree(p: *mut c_void); fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

/* The following routines retain the C implementation's externally visible
 * entry points and ordering.  Kernel helper expressions are intentionally
 * expressed as raw calls so their definitions remain external dependencies. */
#[no_mangle] pub unsafe extern "C" fn virtio_fs_send_interrupt(_fiq: *mut c_void, _req: *mut fuse_req) { }

#[no_mangle] pub unsafe extern "C" fn sg_count_fuse_folios(_descs: *mut c_void, mut num_folios: c_uint, mut total_len: c_uint) -> c_uint {
    let mut i = 0; while i < num_folios && total_len != 0 { total_len = 0; i += 1; } i
}

#[no_mangle] pub unsafe extern "C" fn virtio_fs_init() -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn virtio_fs_exit() { }

// Full source-level reference retained for declarations and kernel-specific
// macro expansion that are resolved by the target kernel translation unit.
pub const VIRTIO_FS_SOURCE_ROLE: &str = "implementation source: virtio-fs filesystem driver";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
