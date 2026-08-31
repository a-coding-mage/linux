// SPDX-License-Identifier: GPL-2.0
// Dependencies from C includes:
// <api/fd/array.h>, <poll.h>, "util/debug.h", "tests/tests.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_short, c_void};

type FILE = c_void;

const POLLIN: c_short = 0x0001;
const POLLOUT: c_short = 0x0004;
const POLLERR: c_short = 0x0008;
const POLLHUP: c_short = 0x0010;

const TEST_FAIL: c_int = -1;
const fdarray_flag__default: c_int = 0;

#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}

#[repr(C)]
pub struct fdarray {
    pub entries: *mut pollfd,
    pub nr: c_int,
    pub nr_alloc: c_int,
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut verbose: c_int;
    static mut stderr: *mut FILE;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn pr_debug(format: *const c_char, ...);

    fn fdarray__new(nr_alloc: c_int, nr_autogrow: c_int) -> *mut fdarray;
    fn fdarray__delete(fda: *mut fdarray);
    fn fdarray__fprintf(fda: *mut fdarray, fp: *mut FILE) -> c_int;
    fn fdarray__filter(
        fda: *mut fdarray,
        revents: c_short,
        entry_destructor: Option<unsafe extern "C" fn(*mut pollfd, *mut c_void)>,
        arg: *mut c_void,
    ) -> c_int;
    fn fdarray__add(fda: *mut fdarray, fd: c_int, events: c_short, flags: c_int) -> c_int;
}

unsafe fn fdarray__init_revents(fda: *mut fdarray, revents: c_short) {
    let mut fd: c_int;

    unsafe {
        (*fda).nr = (*fda).nr_alloc;
    }

    fd = 0;
    while unsafe { fd < (*fda).nr } {
        unsafe {
            (*(*fda).entries.add(fd as usize)).fd = (*fda).nr - fd;
            (*(*fda).entries.add(fd as usize)).events = revents;
            (*(*fda).entries.add(fd as usize)).revents = revents;
        }
        fd += 1;
    }
}

unsafe fn fdarray__fprintf_prefix(
    fda: *mut fdarray,
    prefix: *const c_char,
    fp: *mut FILE,
) -> c_int {
    let mut printed: c_int = 0;

    if unsafe { verbose <= 0 } {
        return 0;
    }

    printed += unsafe { fprintf(fp, c"\n%s: ".as_ptr(), prefix) };
    printed + unsafe { fdarray__fprintf(fda, fp) }
}

macro_rules! goto_out {
    () => {
        return err;
    };
}

macro_rules! goto_out_delete_filter {
    ($fda:expr, $err:expr) => {{
        unsafe { fdarray__delete($fda) };
        return $err;
    }};
}

unsafe fn test__fdarray__filter(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let _ = test;
    let _ = subtest;

    let mut nr_fds: c_int;
    let mut err: c_int = TEST_FAIL;
    let fda: *mut fdarray = unsafe { fdarray__new(5, 5) };

    if fda.is_null() {
        unsafe { pr_debug(c"\nfdarray__new() failed!".as_ptr()) };
        goto_out!();
    }

    unsafe { fdarray__init_revents(fda, POLLIN) };
    nr_fds = unsafe { fdarray__filter(fda, POLLHUP, None, core::ptr::null_mut()) };
    if nr_fds != unsafe { (*fda).nr_alloc } {
        unsafe {
            pr_debug(
                c"\nfdarray__filter()=%d != %d shouldn't have filtered anything".as_ptr(),
                nr_fds,
                (*fda).nr_alloc,
            )
        };
        goto_out_delete_filter!(fda, err);
    }

    unsafe { fdarray__init_revents(fda, POLLHUP) };
    nr_fds = unsafe { fdarray__filter(fda, POLLHUP, None, core::ptr::null_mut()) };
    if nr_fds != 0 {
        unsafe {
            pr_debug(
                c"\nfdarray__filter()=%d != %d, should have filtered all fds".as_ptr(),
                nr_fds,
                (*fda).nr_alloc,
            )
        };
        goto_out_delete_filter!(fda, err);
    }

    unsafe { fdarray__init_revents(fda, POLLHUP) };
    unsafe {
        (*(*fda).entries.add(2)).revents = POLLIN;
    }

    unsafe { pr_debug(c"\nfiltering all but fda->entries[2]:".as_ptr()) };
    unsafe { fdarray__fprintf_prefix(fda, c"before".as_ptr(), stderr) };
    nr_fds = unsafe { fdarray__filter(fda, POLLHUP, None, core::ptr::null_mut()) };
    unsafe { fdarray__fprintf_prefix(fda, c" after".as_ptr(), stderr) };
    if nr_fds != 1 {
        unsafe {
            pr_debug(
                c"\nfdarray__filter()=%d != 1, should have left just one event".as_ptr(),
                nr_fds,
            )
        };
        goto_out_delete_filter!(fda, err);
    }

    unsafe { fdarray__init_revents(fda, POLLHUP) };
    unsafe {
        (*(*fda).entries.add(0)).revents = POLLIN;
        (*(*fda).entries.add(3)).revents = POLLIN;
    }

    unsafe { pr_debug(c"\nfiltering all but (fda->entries[0], fda->entries[3]):".as_ptr()) };
    unsafe { fdarray__fprintf_prefix(fda, c"before".as_ptr(), stderr) };
    nr_fds = unsafe { fdarray__filter(fda, POLLHUP, None, core::ptr::null_mut()) };
    unsafe { fdarray__fprintf_prefix(fda, c" after".as_ptr(), stderr) };
    if nr_fds != 2 {
        unsafe {
            pr_debug(
                c"\nfdarray__filter()=%d != 2, should have left just two events".as_ptr(),
                nr_fds,
            )
        };
        goto_out_delete_filter!(fda, err);
    }

    unsafe { pr_debug(c"\n".as_ptr()) };

    err = 0;
    unsafe { fdarray__delete(fda) };
    err
}

unsafe fn test__fdarray__add(
    test: *mut test_suite,
    subtest: c_int,
) -> c_int {
    let _ = test;
    let _ = subtest;

    let mut err: c_int = TEST_FAIL;
    let fda: *mut fdarray = unsafe { fdarray__new(2, 2) };

    if fda.is_null() {
        unsafe { pr_debug(c"\nfdarray__new() failed!".as_ptr()) };
        return err;
    }

    macro_rules! FDA_CHECK {
        ($_idx:expr, $_fd:expr, $_revents:expr) => {{
            if unsafe { (*(*fda).entries.add($_idx as usize)).fd != $_fd } {
                unsafe {
                    pr_debug(
                        c"\n%d: fda->entries[%d](%d) != %d!".as_ptr(),
                        line!() as c_int,
                        $_idx,
                        (*(*fda).entries.add(1)).fd,
                        $_fd,
                    )
                };
                unsafe { fdarray__delete(fda) };
                return err;
            }
            if unsafe { (*(*fda).entries.add($_idx as usize)).events != $_revents } {
                unsafe {
                    pr_debug(
                        c"\n%d: fda->entries[%d].revents(%d) != %d!".as_ptr(),
                        line!() as c_int,
                        $_idx,
                        (*(*fda).entries.add($_idx as usize)).fd,
                        $_revents,
                    )
                };
                unsafe { fdarray__delete(fda) };
                return err;
            }
        }};
    }

    macro_rules! FDA_ADD {
        ($_idx:expr, $_fd:expr, $_revents:expr, $_nr:expr) => {{
            if unsafe { fdarray__add(fda, $_fd, $_revents, fdarray_flag__default) } < 0 {
                unsafe {
                    pr_debug(
                        c"\n%d: fdarray__add(fda, %d, %d) failed!".as_ptr(),
                        line!() as c_int,
                        $_fd,
                        $_revents,
                    )
                };
                unsafe { fdarray__delete(fda) };
                return err;
            }
            if unsafe { (*fda).nr != $_nr } {
                unsafe {
                    pr_debug(
                        c"\n%d: fdarray__add(fda, %d, %d)=%d != %d".as_ptr(),
                        line!() as c_int,
                        $_fd,
                        $_revents,
                        (*fda).nr,
                        $_nr,
                    )
                };
                unsafe { fdarray__delete(fda) };
                return err;
            }
            FDA_CHECK!($_idx, $_fd, $_revents);
        }};
    }

    FDA_ADD!(0, 1, POLLIN, 1);
    FDA_ADD!(1, 2, POLLERR, 2);

    unsafe { fdarray__fprintf_prefix(fda, c"before growing array".as_ptr(), stderr) };

    FDA_ADD!(2, 35, POLLHUP, 3);

    if unsafe { (*fda).entries.is_null() } {
        unsafe {
            pr_debug(
                c"\nfdarray__add(fda, 35, POLLHUP) should have allocated fda->pollfd!".as_ptr(),
            )
        };
        unsafe { fdarray__delete(fda) };
        return err;
    }

    unsafe { fdarray__fprintf_prefix(fda, c"after 3rd add".as_ptr(), stderr) };

    FDA_ADD!(3, 88, POLLIN | POLLOUT, 4);

    unsafe { fdarray__fprintf_prefix(fda, c"after 4th add".as_ptr(), stderr) };

    FDA_CHECK!(0, 1, POLLIN);
    FDA_CHECK!(1, 2, POLLERR);
    FDA_CHECK!(2, 35, POLLHUP);
    FDA_CHECK!(3, 88, POLLIN | POLLOUT);

    unsafe { pr_debug(c"\n".as_ptr()) };

    err = 0;
    unsafe { fdarray__delete(fda) };
    err
}

// C DEFINE_SUITE registrations:
// DEFINE_SUITE("Filter fds with revents mask in a fdarray", fdarray__filter);
// DEFINE_SUITE("Add fd to a fdarray, making it autogrow", fdarray__add);
