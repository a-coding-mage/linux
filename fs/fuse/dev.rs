// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust-facing translation of fuse/dev.c.
// Kernel-provided types, constants, globals, and helper functions are supplied
// by the surrounding FUSE Rust bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_int, c_long, c_uint, c_ulong, c_void};

// The Linux kernel interfaces used by this implementation are external
// dependencies.  Their declarations intentionally remain external here.
extern "C" {
    fn fuse_len_args(numargs: c_uint, args: *mut fuse_arg) -> c_uint;
}

#[repr(C)]
pub struct fuse_arg {
    pub size: c_uint,
    pub value: *mut c_void,
}

#[repr(C)]
pub struct fuse_chan {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fuse_req {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fuse_iqueue {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fuse_copy_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fuse_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fuse_pqueue {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fuse_args {
    pub opcode: c_uint,
    pub nodeid: u64,
    pub uid: c_uint,
    pub gid: c_uint,
    pub pid: c_uint,
    pub in_numargs: c_uint,
    pub in_args: *mut fuse_arg,
    pub out_numargs: c_uint,
    pub out_args: *mut fuse_arg,
    pub force: bool,
    pub abort_on_kill: bool,
    pub noreply: bool,
    pub out_argvar: bool,
    pub is_ext: bool,
    pub ext_idx: c_uint,
    pub end: Option<unsafe extern "C" fn(*mut fuse_args, c_int)>,
}

#[repr(C)]
pub struct fuse_chan_param {
    pub minor: c_uint,
    pub max_write: c_uint,
    pub max_pages: c_uint,
    pub io_uring_enabled: bool,
}

// C's exported entry points are retained as ABI declarations until the
// corresponding kernel binding types are available.
extern "C" {
    pub fn fuse_chan_set_initialized(fch: *mut fuse_chan, param: *mut fuse_chan_param);
    pub fn fuse_chan_release(fch: *mut fuse_chan);
    pub fn fuse_chan_free(fch: *mut fuse_chan);
    pub fn fuse_chan_new() -> *mut fuse_chan;
    pub fn fuse_dev_chan_new() -> *mut fuse_chan;
    pub fn fuse_dev_alloc() -> *mut fuse_dev;
    pub fn fuse_dev_alloc_install(fch: *mut fuse_chan) -> *mut fuse_dev;
    pub fn fuse_dev_put(fud: *mut fuse_dev);
    pub fn fuse_dev_is_installed(fud: *mut fuse_dev) -> bool;
    pub fn fuse_dev_verify(fud: *mut fuse_dev, fch: *mut fuse_chan) -> bool;
    pub fn fuse_chan_send(fch: *mut fuse_chan, args: *mut fuse_args) -> isize;
    pub fn fuse_chan_send_bg(fch: *mut fuse_chan, args: *mut fuse_args, gfp_flags: c_uint) -> c_int;
    pub fn fuse_chan_send_notify_reply(fch: *mut fuse_chan, args: *mut fuse_args, unique: u64) -> c_int;
    pub fn fuse_copy_init(cs: *mut fuse_copy_state, write: bool, iter: *mut c_void);
    pub fn fuse_copy_finish(cs: *mut fuse_copy_state);
    pub fn fuse_copy_one(cs: *mut fuse_copy_state, val: *mut c_void, size: c_uint) -> c_int;
    pub fn fuse_copy_args(cs: *mut fuse_copy_state, numargs: c_uint, argpages: c_uint,
                          args: *mut fuse_arg, zeroing: c_int) -> c_int;
    pub fn fuse_request_end(req: *mut fuse_req);
    pub fn fuse_chan_abort(fch: *mut fuse_chan, abort_with_err: bool);
    pub fn fuse_chan_wait_aborted(fch: *mut fuse_chan);
    pub fn fuse_dev_install(fud: *mut fuse_dev, fch: *mut fuse_chan);
    pub fn fuse_dev_release(inode: *mut c_void, file: *mut c_void) -> c_int;
    pub fn fuse_dev_init() -> c_int;
    pub fn fuse_dev_cleanup();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
