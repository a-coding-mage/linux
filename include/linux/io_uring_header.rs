/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by the corresponding Linux kernel headers:
// linux/sched.h, linux/xarray.h, and uapi/linux/io_uring.h.

#[cfg(CONFIG_IO_URING)]
extern "C" {
    pub fn __io_uring_cancel(cancel_all: bool);
    pub fn __io_uring_free(tsk: *mut task_struct);
    pub fn io_uring_unreg_ringfd();
    pub fn io_uring_get_opcode(opcode: u8) -> *const core::ffi::c_char;
    pub fn io_is_uring_fops(file: *mut file) -> bool;
    pub fn __io_uring_fork(tsk: *mut task_struct) -> i32;
}

// External kernel types and the current task pointer are supplied by
// linux/sched.h and related headers.
#[cfg(CONFIG_IO_URING)]
extern "C" {
    static mut current: *mut task_struct;
}

#[cfg(CONFIG_IO_URING)]
#[inline]
pub unsafe fn io_uring_files_cancel() {
    if !(*current).io_uring.is_null() {
        __io_uring_cancel(false);
    }
}

#[cfg(CONFIG_IO_URING)]
#[inline]
pub unsafe fn io_uring_task_cancel() {
    if !(*current).io_uring.is_null() {
        __io_uring_cancel(true);
    }
}

#[cfg(CONFIG_IO_URING)]
#[inline]
pub unsafe fn io_uring_free(tsk: *mut task_struct) {
    if !(*tsk).io_uring.is_null() || !(*tsk).io_uring_restrict.is_null() {
        __io_uring_free(tsk);
    }
}

#[cfg(CONFIG_IO_URING)]
#[inline]
pub unsafe fn io_uring_fork(tsk: *mut task_struct) -> i32 {
    if !(*tsk).io_uring_restrict.is_null() {
        return __io_uring_fork(tsk);
    }

    0
}

#[cfg(not(CONFIG_IO_URING))]
#[inline]
pub fn io_uring_task_cancel() {}

#[cfg(not(CONFIG_IO_URING))]
#[inline]
pub fn io_uring_files_cancel() {}

#[cfg(not(CONFIG_IO_URING))]
#[inline]
pub fn io_uring_free(_tsk: *mut task_struct) {}

#[cfg(not(CONFIG_IO_URING))]
#[inline]
pub fn io_uring_get_opcode(_opcode: u8) -> *const core::ffi::c_char {
    b"\0".as_ptr() as *const core::ffi::c_char
}

#[cfg(not(CONFIG_IO_URING))]
#[inline]
pub fn io_is_uring_fops(_file: *mut file) -> bool {
    false
}

#[cfg(not(CONFIG_IO_URING))]
#[inline]
pub fn io_uring_fork(_tsk: *mut task_struct) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
