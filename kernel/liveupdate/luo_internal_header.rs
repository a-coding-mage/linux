/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (c) 2025, Google LLC.
 * Pasha Tatashin <pasha.tatashin@soleen.com>
 */

/* Translated from the Linux kernel internal Live Update header. */

pub struct luo_ucmd {
    pub ubuffer: *mut core::ffi::c_void,
    pub user_size: u32,
    pub cmd: *mut core::ffi::c_void,
}

pub unsafe fn luo_ucmd_respond(
    ucmd: *mut luo_ucmd,
    kernel_cmd_size: usize,
) -> i32 {
    /*
     * Copy the minimum of what the user provided and what we actually
     * have.
     */
    if crate::copy_to_user(
        (*ucmd).ubuffer,
        (*ucmd).cmd,
        core::cmp::min((*ucmd).user_size as usize, kernel_cmd_size),
    ) != 0 {
        return -14; // -EFAULT
    }
    0
}

/*
 * Handles a deserialization failure: devices and memory is in unpredictable
 * state.
 *
 * Continuing the boot process after a failure is dangerous because it could
 * lead to leaks of private data.
 */
macro_rules! luo_restore_fail {
    ($fmt:expr $(, $arg:expr)*) => {
        panic!($fmt $(, $arg)*)
    };
}

/**
 * struct luo_file_set - A set of files that belong to the same sessions.
 * @files_list: An ordered list of files associated with this session, it is
 *              ordered by preservation time.
 * @block_set:  The set of serialization blocks.
 * @count:      A counter tracking the number of files currently stored in the
 *              @files_list for this session.
 */
#[repr(C)]
pub struct luo_file_set {
    pub files_list: crate::list_head,
    pub block_set: crate::kho_block_set,
    pub count: u64,
}

/**
 * struct luo_session - Represents an active or incoming Live Update session.
 * @name:       A unique name for this session, used for identification and
 *              retrieval.
 * @list:       A list_head member used to link this session into a global list
 *              of either outgoing (to be preserved) or incoming (restored from
 *              previous kernel) sessions.
 * @retrieved:  A boolean flag indicating whether this session has been
 *              retrieved by a consumer in the new kernel.
 * @file_set:   A set of files that belong to this session.
 * @mutex:      protects fields in the luo_session.
 */
#[repr(C)]
pub struct luo_session {
    pub name: [core::ffi::c_char; crate::LIVEUPDATE_SESSION_NAME_LENGTH],
    pub list: crate::list_head,
    pub retrieved: bool,
    pub file_set: luo_file_set,
    pub mutex: crate::mutex,
}

extern "C" {
    pub static mut luo_register_rwlock: crate::rw_semaphore;

    pub fn luo_session_create(name: *const core::ffi::c_char, filep: *mut *mut crate::file) -> i32;
    pub fn luo_session_retrieve(name: *const core::ffi::c_char, filep: *mut *mut crate::file) -> i32;
    pub fn luo_session_setup_outgoing(sessions_pa: *mut u64);
    pub fn luo_session_setup_incoming(sessions_pa: u64) -> i32;
    pub fn luo_session_serialize() -> i32;
    pub fn luo_session_deserialize() -> i32;

    pub fn luo_preserve_file(file_set: *mut luo_file_set, token: u64, fd: i32) -> i32;
    pub fn luo_file_unpreserve_files(file_set: *mut luo_file_set);
    pub fn luo_file_freeze(file_set: *mut luo_file_set, file_set_ser: *mut crate::luo_file_set_ser) -> i32;
    pub fn luo_file_unfreeze(file_set: *mut luo_file_set, file_set_ser: *mut crate::luo_file_set_ser);
    pub fn luo_retrieve_file(file_set: *mut luo_file_set, token: u64, filep: *mut *mut crate::file) -> i32;
    pub fn luo_file_finish(file_set: *mut luo_file_set) -> i32;
    pub fn luo_file_deserialize(file_set: *mut luo_file_set, file_set_ser: *mut crate::luo_file_set_ser) -> i32;
    pub fn luo_file_set_init(file_set: *mut luo_file_set);
    pub fn luo_file_set_destroy(file_set: *mut luo_file_set);

    pub fn luo_flb_file_preserve(fh: *mut crate::liveupdate_file_handler) -> i32;
    pub fn luo_flb_file_unpreserve(fh: *mut crate::liveupdate_file_handler);
    pub fn luo_flb_file_finish(fh: *mut crate::liveupdate_file_handler);
    pub fn luo_flb_unregister_all(fh: *mut crate::liveupdate_file_handler);
    pub fn luo_flb_setup_outgoing(flbs_pa: *mut u64) -> i32;
    pub fn luo_flb_setup_incoming(flbs_pa: u64);
    pub fn luo_flb_serialize();
}

#[cfg(feature = "CONFIG_LIVEUPDATE_TEST")]
extern "C" {
    pub fn liveupdate_test_register(fh: *mut crate::liveupdate_file_handler);
}

#[cfg(not(feature = "CONFIG_LIVEUPDATE_TEST"))]
pub unsafe fn liveupdate_test_register(_fh: *mut crate::liveupdate_file_handler) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
