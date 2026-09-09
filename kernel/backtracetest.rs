// SPDX-License-Identifier: GPL-2.0-only
/*
 * Simple stack backtrace regression test module
 *
 * (C) Copyright 2008 Intel Corporation
 * Author: Arjan van de Ven <arjan@linux.intel.com>
 */

// Dependencies supplied by the kernel headers in the original source.

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

extern "C" {
    fn pr_info(format: *const core::ffi::c_char, ...);
    fn dump_stack();
    fn queue_work(workqueue: *mut core::ffi::c_void, work: *mut work_struct) -> bool;
    fn flush_work(work: *mut work_struct);
    fn stack_trace_save(entries: *mut usize, size: usize, skipnr: usize) -> u32;
    fn stack_trace_print(entries: *const usize, nr_entries: u32, spaces: usize);
    static mut system_bh_wq: *mut core::ffi::c_void;
}

unsafe extern "C" fn backtrace_test_bh_workfn(_work: *mut work_struct) {
    unsafe {
        dump_stack();
    }
}

// Equivalent of DECLARE_WORK(backtrace_bh_work, &backtrace_test_bh_workfn).
static mut backtrace_bh_work: work_struct = work_struct { _private: [] };

unsafe fn backtrace_test_normal() {
    unsafe {
        pr_info(b"Testing a backtrace from process context.\n\0".as_ptr() as *const _);
        pr_info(b"The following trace is a kernel self test and not a bug!\n\0".as_ptr() as *const _);

        dump_stack();
    }
}

unsafe fn backtrace_test_bh() {
    unsafe {
        pr_info(b"Testing a backtrace from BH context.\n\0".as_ptr() as *const _);
        pr_info(b"The following trace is a kernel self test and not a bug!\n\0".as_ptr() as *const _);

        queue_work(system_bh_wq, &raw mut backtrace_bh_work);
        flush_work(&raw mut backtrace_bh_work);
    }
}

// CONFIG_STACKTRACE controls which implementation is compiled by the kernel build.
#[cfg(feature = "CONFIG_STACKTRACE")]
unsafe fn backtrace_test_saved() {
    let mut entries: [usize; 8] = [0; 8];
    let nr_entries: u32;

    unsafe {
        pr_info(b"Testing a saved backtrace.\n\0".as_ptr() as *const _);
        pr_info(b"The following trace is a kernel self test and not a bug!\n\0".as_ptr() as *const _);

        nr_entries = stack_trace_save(entries.as_mut_ptr(), entries.len(), 0);
        stack_trace_print(entries.as_ptr(), nr_entries, 0);
    }
}

#[cfg(not(feature = "CONFIG_STACKTRACE"))]
unsafe fn backtrace_test_saved() {
    unsafe {
        pr_info(b"Saved backtrace test skipped.\n\0".as_ptr() as *const _);
    }
}

unsafe fn backtrace_regression_test() -> i32 {
    unsafe {
        pr_info(b"====[ backtrace testing ]===========\n\0".as_ptr() as *const _);

        backtrace_test_normal();
        backtrace_test_bh();
        backtrace_test_saved();

        pr_info(b"====[ end of backtrace testing ]====\n\0".as_ptr() as *const _);
    }
    0
}

unsafe fn exitf() {}

// Equivalent of module_init(backtrace_regression_test) and module_exit(exitf).
// MODULE_DESCRIPTION("Simple stack backtrace regression test module");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Arjan van de Ven <arjan@linux.intel.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
