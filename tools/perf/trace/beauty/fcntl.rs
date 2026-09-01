// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/fcntl.c
 *
 *  Copyright (C) 2017, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

// Dependencies from:
// #include "trace/beauty/beauty.h"
// #include <linux/kernel.h>
// #include <linux/fcntl.h>

use core::ffi::{c_char, c_int, c_ulong};

type size_t = usize;

// Provided by linux/fcntl.h or compatibility definitions in the original C file.
// The original file defines these from F_LINUX_SPECIFIC_BASE when missing.
const F_GET_RW_HINT: c_int = F_LINUX_SPECIFIC_BASE + 11;
const F_SET_RW_HINT: c_int = F_LINUX_SPECIFIC_BASE + 12;
const F_GET_FILE_RW_HINT: c_int = F_LINUX_SPECIFIC_BASE + 13;
const F_SET_FILE_RW_HINT: c_int = F_LINUX_SPECIFIC_BASE + 14;

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub show_string_prefix: bool,
    pub mask: c_ulong,
}

#[repr(C)]
pub struct strarray;

unsafe extern "C" {
    static strarray__fcntl_setlease: strarray;

    static F_LINUX_SPECIFIC_BASE: c_int;
    static F_GETFL: c_ulong;
    static F_GETFD: c_ulong;
    static F_DUPFD_CLOEXEC: c_ulong;
    static F_DUPFD: c_ulong;
    static F_GETOWN: c_ulong;
    static F_GETLEASE: c_ulong;
    static F_GET_SEALS: c_ulong;
    static F_GETSIG: c_ulong;
    static F_SETFD: c_int;
    static F_SETFL: c_int;
    static F_SETOWN: c_int;
    static F_SETLEASE: c_int;
    static F_SETLK: c_int;
    static F_SETLKW: c_int;
    static F_GETLK: c_int;
    static F_OFD_SETLK: c_int;
    static F_OFD_SETLKW: c_int;
    static F_OFD_GETLK: c_int;
    static F_GETOWN_EX: c_int;
    static F_SETOWN_EX: c_int;

    fn scnprintf(bf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> size_t;
    fn strarray__scnprintf(
        sa: *const strarray,
        bf: *mut c_char,
        size: size_t,
        fmt: *const c_char,
        show_prefix: bool,
        val: c_ulong,
    ) -> size_t;
    fn syscall_arg__set_ret_scnprintf(
        arg: *mut syscall_arg,
        scnprintf: unsafe extern "C" fn(*mut c_char, size_t, *mut syscall_arg) -> size_t,
    );
    fn syscall_arg__scnprintf_open_flags(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    fn syscall_arg__scnprintf_fd(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    fn syscall_arg__scnprintf_pid(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    fn syscall_arg__scnprintf_strarrays(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    fn syscall_arg__val(arg: *mut syscall_arg, idx: c_int) -> c_ulong;
    fn open__scnprintf_flags(
        val: c_ulong,
        bf: *mut c_char,
        size: size_t,
        show_prefix: bool,
    ) -> size_t;
    fn syscall_arg__scnprintf_hex(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
    fn syscall_arg__scnprintf_long(
        bf: *mut c_char,
        size: size_t,
        arg: *mut syscall_arg,
    ) -> size_t;
}

unsafe extern "C" fn fcntl__scnprintf_getfd(
    val: c_ulong,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
) -> size_t {
    unsafe {
        if val != 0 {
            scnprintf(bf, size, c"%s".as_ptr(), c"0".as_ptr())
        } else {
            scnprintf(
                bf,
                size,
                c"%s%s".as_ptr(),
                if show_prefix {
                    c"FD_".as_ptr()
                } else {
                    c"".as_ptr()
                },
                c"CLOEXEC".as_ptr(),
            )
        }
    }
}

unsafe extern "C" fn syscall_arg__scnprintf_fcntl_getfd(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    unsafe { fcntl__scnprintf_getfd((*arg).val, bf, size, (*arg).show_string_prefix) }
}

unsafe extern "C" fn fcntl__scnprintf_getlease(
    val: c_ulong,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
) -> size_t {
    // static const char *fcntl_setlease[] = { "RDLCK", "WRLCK", "UNLCK", };
    // static DEFINE_STRARRAY(fcntl_setlease, "F_");
    unsafe {
        strarray__scnprintf(
            &raw const strarray__fcntl_setlease,
            bf,
            size,
            c"%x".as_ptr(),
            show_prefix,
            val,
        )
    }
}

unsafe extern "C" fn syscall_arg__scnprintf_fcntl_getlease(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    unsafe { fcntl__scnprintf_getlease((*arg).val, bf, size, (*arg).show_string_prefix) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_fcntl_cmd(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    unsafe {
        if (*arg).val == F_GETFL {
            syscall_arg__set_ret_scnprintf(arg, syscall_arg__scnprintf_open_flags);
            (*arg).mask |= 1 << 2;
        } else if (*arg).val == F_GETFD {
            syscall_arg__set_ret_scnprintf(arg, syscall_arg__scnprintf_fcntl_getfd);
            (*arg).mask |= 1 << 2;
        } else if (*arg).val == F_DUPFD_CLOEXEC || (*arg).val == F_DUPFD {
            syscall_arg__set_ret_scnprintf(arg, syscall_arg__scnprintf_fd);
        } else if (*arg).val == F_GETOWN {
            syscall_arg__set_ret_scnprintf(arg, syscall_arg__scnprintf_pid);
            (*arg).mask |= 1 << 2;
        } else if (*arg).val == F_GETLEASE {
            syscall_arg__set_ret_scnprintf(arg, syscall_arg__scnprintf_fcntl_getlease);
            (*arg).mask |= 1 << 2;
        } else {
            /*
             * Some commands ignore the third fcntl argument, "arg", so mask it
             */
            if (*arg).val == F_GET_SEALS || (*arg).val == F_GETSIG {
                (*arg).mask |= 1 << 2;
            }
        }

        syscall_arg__scnprintf_strarrays(bf, size, arg)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_fcntl_arg(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    unsafe {
        let show_prefix: bool = (*arg).show_string_prefix;
        let cmd: c_int = syscall_arg__val(arg, 1) as c_int;

        if cmd == F_DUPFD as c_int {
            return syscall_arg__scnprintf_fd(bf, size, arg);
        }

        if cmd == F_SETFD {
            return fcntl__scnprintf_getfd((*arg).val, bf, size, show_prefix);
        }

        if cmd == F_SETFL {
            return open__scnprintf_flags((*arg).val, bf, size, show_prefix);
        }

        if cmd == F_SETOWN {
            return syscall_arg__scnprintf_pid(bf, size, arg);
        }

        if cmd == F_SETLEASE {
            return fcntl__scnprintf_getlease((*arg).val, bf, size, show_prefix);
        }
        /*
         * We still don't grab the contents of pointers on entry or exit,
         * so just print them as hex numbers
         */
        if cmd == F_SETLK
            || cmd == F_SETLKW
            || cmd == F_GETLK
            || cmd == F_OFD_SETLK
            || cmd == F_OFD_SETLKW
            || cmd == F_OFD_GETLK
            || cmd == F_GETOWN_EX
            || cmd == F_SETOWN_EX
            || cmd == F_GET_RW_HINT
            || cmd == F_SET_RW_HINT
            || cmd == F_GET_FILE_RW_HINT
            || cmd == F_SET_FILE_RW_HINT
        {
            return syscall_arg__scnprintf_hex(bf, size, arg);
        }

        syscall_arg__scnprintf_long(bf, size, arg)
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
