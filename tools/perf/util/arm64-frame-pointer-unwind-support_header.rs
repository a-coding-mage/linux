/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from C header guard:
 * __PERF_ARM_FRAME_POINTER_UNWIND_SUPPORT_H
 */

/* Dependency intent from C: #include <linux/types.h> */

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct record_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn add_leaf_frame_caller_opts_aarch64(opts: *mut record_opts);
    pub fn get_leaf_frame_caller_aarch64(
        sample: *mut perf_sample,
        thread: *mut thread,
        user_idx: ::std::os::raw::c_int,
    ) -> u64;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
