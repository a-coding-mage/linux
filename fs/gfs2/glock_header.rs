/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from glock.h. Dependencies are supplied by other translation units. */

pub const Opt_jid: ::core::ffi::c_int = 0;
pub const Opt_id: ::core::ffi::c_int = 1;
pub const Opt_first: ::core::ffi::c_int = 2;
pub const Opt_nodir: ::core::ffi::c_int = 3;
pub const Opt_err: ::core::ffi::c_int = 4;

pub const LM_TYPE_RESERVED: u32 = 0x00;
pub const LM_TYPE_NONDISK: u32 = 0x01;
pub const LM_TYPE_INODE: u32 = 0x02;
pub const LM_TYPE_RGRP: u32 = 0x03;
pub const LM_TYPE_META: u32 = 0x04;
pub const LM_TYPE_IOPEN: u32 = 0x05;
pub const LM_TYPE_FLOCK: u32 = 0x06;
pub const LM_TYPE_PLOCK: u32 = 0x07;
pub const LM_TYPE_QUOTA: u32 = 0x08;
pub const LM_TYPE_JOURNAL: u32 = 0x09;

pub const LM_ST_UNLOCKED: u32 = 0;
pub const LM_ST_EXCLUSIVE: u32 = 1;
pub const LM_ST_DEFERRED: u32 = 2;
pub const LM_ST_SHARED: u32 = 3;

pub const LM_FLAG_TRY: u32 = 0x0001;
pub const LM_FLAG_TRY_1CB: u32 = 0x0002;
pub const LM_FLAG_RECOVER: u32 = 0x0004;
pub const LM_FLAG_ANY: u32 = 0x0008;
pub const LM_FLAG_NODE_SCOPE: u32 = 0x0020;
pub const GL_ASYNC: u32 = 0x0040;
pub const GL_EXACT: u32 = 0x0080;
pub const GL_SKIP: u32 = 0x0100;
pub const GL_NOPID: u32 = 0x0200;
pub const GL_NOCACHE: u32 = 0x0400;
pub const GL_NOBLOCK: u32 = 0x0800;

pub const LM_OUT_ST_MASK: u32 = 0x00000003;
pub const LM_OUT_TRY_AGAIN: u32 = 0x00000020;
pub const LM_OUT_DEADLOCK: u32 = 0x00000010;
pub const LM_OUT_CANCELED: u32 = 0x00000008;
pub const LM_OUT_ERROR: u32 = 0x00000004;
pub const LM_RD_GAVEUP: u32 = 308;
pub const LM_RD_SUCCESS: u32 = 309;
pub const GLR_TRYFAILED: u32 = 13;

#[repr(C)]
pub struct lm_lockops {
    pub lm_proto_name: *const ::core::ffi::c_char,
    pub lm_mount: Option<unsafe extern "C" fn(*mut gfs2_sbd, *const ::core::ffi::c_char) -> ::core::ffi::c_int>,
    pub lm_first_done: Option<unsafe extern "C" fn(*mut gfs2_sbd)>,
    pub lm_recovery_result: Option<unsafe extern "C" fn(*mut gfs2_sbd, u32, u32)>,
    pub lm_unmount: Option<unsafe extern "C" fn(*mut gfs2_sbd, bool)>,
    pub lm_withdraw: Option<unsafe extern "C" fn(*mut gfs2_sbd)>,
    pub lm_put_lock: Option<unsafe extern "C" fn(*mut gfs2_glock)>,
    pub lm_lock: Option<unsafe extern "C" fn(*mut gfs2_glock, u32, u32) -> ::core::ffi::c_int>,
    pub lm_cancel: Option<unsafe extern "C" fn(*mut gfs2_glock)>,
    pub lm_tokens: *const match_table_t,
}

#[repr(C)]
pub struct gfs2_glock_aspace {
    pub glock: gfs2_glock,
    pub mapping: address_space,
}

extern "C" {
    pub static gfs2_dlm_ops: lm_lockops;
    pub fn __gfs2_holder_init(gl: *mut gfs2_glock, state: u32, flags: u16, gh: *mut gfs2_holder, ip: ::core::ffi::c_ulong);
    pub fn gfs2_glock_nq(gh: *mut gfs2_holder) -> ::core::ffi::c_int;
    pub fn gfs2_holder_uninit(gh: *mut gfs2_holder);
}

pub unsafe fn gfs2_holder_init(gl: *mut gfs2_glock, state: u32, flags: u16, gh: *mut gfs2_holder) {
    __gfs2_holder_init(gl, state, flags, gh, 0);
}

pub unsafe fn gfs2_glock_nq_init(gl: *mut gfs2_glock, state: u32, flags: u16, gh: *mut gfs2_holder) -> ::core::ffi::c_int {
    __gfs2_holder_init(gl, state, flags, gh, 0);
    let error = gfs2_glock_nq(gh);
    if error != 0 { gfs2_holder_uninit(gh); }
    error
}

extern "C" {
    pub fn gfs2_glock_get(sdp: *mut gfs2_sbd, number: u64, glops: *const gfs2_glock_operations, create: ::core::ffi::c_int, glp: *mut *mut gfs2_glock) -> ::core::ffi::c_int;
    pub fn gfs2_glock_hold(gl: *mut gfs2_glock) -> *mut gfs2_glock;
    pub fn gfs2_glock_put(gl: *mut gfs2_glock);
    pub fn gfs2_glock_put_async(gl: *mut gfs2_glock);
    pub fn gfs2_holder_reinit(state: u32, flags: u16, gh: *mut gfs2_holder);
    pub fn gfs2_holder_queued(gh: *mut gfs2_holder) -> bool;
}

/* Remaining declarations retain the source header's external interfaces. */
extern "C" {
    pub fn gfs2_glock_cb(gl: *mut gfs2_glock, state: u32);
    pub fn gfs2_glock_complete(gl: *mut gfs2_glock, ret: ::core::ffi::c_int);
    pub fn gfs2_queue_try_to_evict(gl: *mut gfs2_glock) -> bool;
    pub fn gfs2_queue_verify_delete(gl: *mut gfs2_glock, later: bool) -> bool;
    pub fn gfs2_cancel_delete_work(gl: *mut gfs2_glock);
    pub fn gfs2_flush_delete_work(sdp: *mut gfs2_sbd);
    pub fn gfs2_wait_glocks(sdp: *mut gfs2_sbd);
    pub fn gfs2_withdraw_glocks(sdp: *mut gfs2_sbd);
    pub fn gfs2_glock_thaw(sdp: *mut gfs2_sbd);
    pub fn gfs2_glock_free(gl: *mut gfs2_glock);
    pub fn gfs2_glock_free_later(gl: *mut gfs2_glock);
    pub fn gfs2_glock_init() -> ::core::ffi::c_int;
    pub fn gfs2_glock_exit();
    pub fn gfs2_create_debugfs_file(sdp: *mut gfs2_sbd);
    pub fn gfs2_delete_debugfs_file(sdp: *mut gfs2_sbd);
    pub fn gfs2_register_debugfs();
    pub fn gfs2_unregister_debugfs();
    pub fn glock_set_object(gl: *mut gfs2_glock, object: *mut ::core::ffi::c_void);
    pub fn glock_clear_object(gl: *mut gfs2_glock, object: *mut ::core::ffi::c_void);
    pub fn gfs2_inode_remember_delete(gl: *mut gfs2_glock, generation: u64);
    pub fn gfs2_inode_already_deleted(gl: *mut gfs2_glock, generation: u64) -> bool;
    pub fn gfs2_glock_poll(gh: *mut gfs2_holder) -> ::core::ffi::c_int;
    pub fn gfs2_instantiate(gh: *mut gfs2_holder) -> ::core::ffi::c_int;
    pub fn gfs2_glock_holder_ready(gh: *mut gfs2_holder) -> ::core::ffi::c_int;
    pub fn gfs2_glock_wait(gh: *mut gfs2_holder) -> ::core::ffi::c_int;
    pub fn gfs2_glock_async_wait(num_gh: u32, ghs: *mut gfs2_holder, retries: u32) -> ::core::ffi::c_int;
    pub fn gfs2_glock_dq(gh: *mut gfs2_holder);
    pub fn gfs2_glock_dq_wait(gh: *mut gfs2_holder);
    pub fn gfs2_glock_dq_uninit(gh: *mut gfs2_holder);
    pub fn gfs2_glock_nq_num(sdp: *mut gfs2_sbd, number: u64, glops: *const gfs2_glock_operations, state: u32, flags: u16, gh: *mut gfs2_holder) -> ::core::ffi::c_int;
    pub fn gfs2_glock_nq_m(num_gh: u32, ghs: *mut gfs2_holder) -> ::core::ffi::c_int;
    pub fn gfs2_glock_dq_m(num_gh: u32, ghs: *mut gfs2_holder);
    pub fn gfs2_print_dbg(seq: *mut seq_file, fmt: *const ::core::ffi::c_char, ...);
}

/* Opaque types and kernel helpers are defined by the translated dependencies. */
extern "C" {
    pub type gfs2_sbd;
    pub type gfs2_glock;
    pub type gfs2_holder;
    pub type gfs2_glock_operations;
    pub type address_space;
    pub type match_table_t;
    pub type seq_file;
}

pub const GL_GLOCK_MAX_HOLD: ::core::ffi::c_long = 0; // (long)(HZ / 5), supplied by the kernel build.
pub const GL_GLOCK_DFT_HOLD: ::core::ffi::c_long = 0; // (long)(HZ / 5), supplied by the kernel build.
pub const GL_GLOCK_MIN_HOLD: ::core::ffi::c_long = 0; // (long)(HZ / 100), supplied by the kernel build.
pub const GL_GLOCK_HOLD_INCR: ::core::ffi::c_long = 0; // (long)(HZ / 20), supplied by the kernel build.
pub const GL_GLOCK_HOLD_DECR: ::core::ffi::c_long = 0; // (long)(HZ / 40), supplied by the kernel build.

/* Inline helpers preserve their C interfaces; their kernel list/bit primitives
 * and structure layouts are provided by the dependent translation units. */
extern "C" {
    pub fn gfs2_glock_is_locked_by_me(gl: *mut gfs2_glock) -> *mut gfs2_holder;
    pub fn gfs2_glock2aspace(gl: *mut gfs2_glock) -> *mut address_space;
    pub fn gfs2_holder_mark_uninitialized(gh: *mut gfs2_holder);
    pub fn gfs2_holder_initialized(gh: *mut gfs2_holder) -> bool;
    pub fn gfs2_glock_assert_warn(gl: *mut gfs2_glock, condition: bool);
    pub fn gfs2_glock_assert_withdraw(gl: *mut gfs2_glock, condition: bool);
    pub fn gfs2_dump_glock(seq: *mut seq_file, gl: *mut gfs2_glock, fsid: bool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
