// SPDX-License-Identifier: LGPL-2.1
// C dependencies: "trace/beauty/beauty.h", "util/machine.h", "util/thread.h"

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct syscall_arg {
    pub val: i64,
    pub trace: *mut trace,
}

#[repr(C)]
pub struct trace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn scnprintf(bf: *mut c_char, size: usize, fmt: *const c_char, ...) -> usize;
    fn trace__host(trace: *mut trace) -> *mut machine;
    fn machine__findnew_thread(machine: *mut machine, pid: c_int, tid: c_int) -> *mut thread;
    fn thread__comm_set(thread: *mut thread) -> bool;
    fn thread__set_comm_from_proc(thread: *mut thread);
    fn thread__comm_str(thread: *mut thread) -> *const c_char;
    fn thread__put(thread: *mut thread);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_pid(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let pid: c_int = unsafe { (*arg).val as c_int };
    let trace: *mut trace = unsafe { (*arg).trace };
    let mut printed: usize = unsafe { scnprintf(bf, size, c"%d".as_ptr(), pid) };
    let thread: *mut thread =
        unsafe { machine__findnew_thread(trace__host(trace), pid, pid) };

    if !thread.is_null() {
        if unsafe { !thread__comm_set(thread) } {
            unsafe { thread__set_comm_from_proc(thread) };
        }

        if unsafe { thread__comm_set(thread) } {
            printed += unsafe {
                scnprintf(
                    bf.add(printed),
                    size.wrapping_sub(printed),
                    c" (%s)".as_ptr(),
                    thread__comm_str(thread),
                )
            };
        }
        unsafe { thread__put(thread) };
    }

    printed
}
