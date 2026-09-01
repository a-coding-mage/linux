// SPDX-License-Identifier: LGPL-2.1
/*
 * trace/beauty/socket.c
 *
 *  Copyright (C) 2018, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 */

use std::ffi::{c_char, c_int};

pub type size_t = usize;

#[repr(C)]
pub struct syscall_arg {
    pub val: c_int,
    pub show_string_prefix: bool,
}

#[repr(C)]
pub struct strarray {
    _private: [u8; 0],
}

unsafe extern "C" {
    /*
     * Dependencies from trace/beauty/beauty.h and
     * trace/beauty/generated/socket.c.
     */
    static strarray__socket_ipproto: strarray;
    static strarray__socket_level: strarray;

    fn strarray__scnprintf(
        sa: *const strarray,
        bf: *mut c_char,
        size: size_t,
        fmt: *const c_char,
        show_prefix: bool,
        val: c_int,
    ) -> size_t;
    fn syscall_arg__val(arg: *mut syscall_arg, idx: c_int) -> c_int;
    fn syscall_arg__scnprintf_int(bf: *mut c_char, size: size_t, arg: *mut syscall_arg) -> size_t;
    fn scnprintf(bf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> size_t;
}

const AF_INET: c_int = 2;
const AF_INET6: c_int = 10;

unsafe fn socket__scnprintf_ipproto(
    protocol: c_int,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
) -> size_t {
    /*
     * C source has:
     * static DEFINE_STRARRAY(socket_ipproto, "IPPROTO_");
     */
    unsafe {
        strarray__scnprintf(
            &strarray__socket_ipproto,
            bf,
            size,
            c"%d".as_ptr(),
            show_prefix,
            protocol,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_socket_protocol(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let domain: c_int = unsafe { syscall_arg__val(arg, 0) };

    if domain == AF_INET || domain == AF_INET6 {
        return unsafe {
            socket__scnprintf_ipproto((*arg).val, bf, size, (*arg).show_string_prefix)
        };
    }

    unsafe { syscall_arg__scnprintf_int(bf, size, arg) }
}

unsafe fn socket__scnprintf_level(
    level: c_int,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
) -> size_t {
    /*
     * C conditional:
     * #if defined(__alpha__) || defined(__hppa__) || defined(__mips__) || defined(__sparc__)
     *     const int sol_socket = 0xffff;
     * #else
     *     const int sol_socket = 1;
     * #endif
     */
    #[cfg(any(
        target_arch = "alpha",
        target_arch = "hppa",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "sparc",
        target_arch = "sparc64"
    ))]
    let sol_socket: c_int = 0xffff;
    #[cfg(not(any(
        target_arch = "alpha",
        target_arch = "hppa",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "sparc",
        target_arch = "sparc64"
    )))]
    let sol_socket: c_int = 1;

    if level == sol_socket {
        return unsafe {
            scnprintf(
                bf,
                size,
                c"%sSOCKET".as_ptr(),
                if show_prefix {
                    c"SOL_".as_ptr()
                } else {
                    c"".as_ptr()
                },
            )
        };
    }

    unsafe {
        strarray__scnprintf(
            &strarray__socket_level,
            bf,
            size,
            c"%d".as_ptr(),
            show_prefix,
            level,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_socket_level(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    unsafe { socket__scnprintf_level((*arg).val, bf, size, (*arg).show_string_prefix) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
