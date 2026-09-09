/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding UAPI and sem_types headers.

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_SYSVIPC")]
extern "C" {
    pub fn copy_semundo(clone_flags: u64, tsk: *mut task_struct) -> ::core::ffi::c_int;
    pub fn exit_sem(tsk: *mut task_struct);
}

#[cfg(not(feature = "CONFIG_SYSVIPC"))]
#[inline]
pub fn copy_semundo(_clone_flags: u64, _tsk: *mut task_struct) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_SYSVIPC"))]
#[inline]
pub fn exit_sem(_tsk: *mut task_struct) {
    return;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
