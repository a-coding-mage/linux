// SPDX-License-Identifier: GPL-2.0-only

/*
 * Copyright 2021 Google LLC.
 */

// C dependencies translated as external declarations:
// <test_progs.h>, <cgroup_helpers.h>, <network_helpers.h>
// "cgroup_getset_retval_setsockopt.skel.h"
// "cgroup_getset_retval_getsockopt.skel.h"
// "cgroup_getset_retval_hooks.skel.h"

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const SOL_CUSTOM: c_int = 0xdeadbeefu32 as c_int;

const _SC_PAGESIZE: c_int = 30;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;
const AF_INET: c_int = 2;
const SOCK_DGRAM: c_int = 2;
const EUNATCH: c_int = 49;
const EISCONN: c_int = 106;
const EPERM: c_int = 1;
const EOPNOTSUPP: c_int = 95;

type socklen_t = c_uint;

static mut zero: c_int = 0;

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup_getset_retval_setsockopt_bss {
    pub page_size: c_long,
    pub invocations: c_int,
    pub assertion_error: bool,
    pub retval_value: c_int,
}

#[repr(C)]
pub struct cgroup_getset_retval_setsockopt_progs {
    pub set_eunatch: *mut bpf_program,
    pub get_retval: *mut bpf_program,
    pub set_eisconn: *mut bpf_program,
    pub legacy_eperm: *mut bpf_program,
}

#[repr(C)]
pub struct cgroup_getset_retval_setsockopt {
    pub bss: *mut cgroup_getset_retval_setsockopt_bss,
    pub progs: cgroup_getset_retval_setsockopt_progs,
}

#[repr(C)]
pub struct cgroup_getset_retval_getsockopt_bss {
    pub page_size: c_long,
    pub invocations: c_int,
    pub assertion_error: bool,
    pub retval_value: c_int,
    pub ctx_retval_value: c_int,
}

#[repr(C)]
pub struct cgroup_getset_retval_getsockopt_progs {
    pub get_retval: *mut bpf_program,
    pub set_eisconn: *mut bpf_program,
    pub clear_retval: *mut bpf_program,
}

#[repr(C)]
pub struct cgroup_getset_retval_getsockopt {
    pub bss: *mut cgroup_getset_retval_getsockopt_bss,
    pub progs: cgroup_getset_retval_getsockopt_progs,
}

#[repr(C)]
pub struct cgroup_getset_retval_hooks {
    pub obj: *mut bpf_object,
}

#[repr(C)]
struct exposed_hook {
    name: *const c_char,
    expected_err: c_int,
}

unsafe extern "C" {
    fn sysconf(name: c_int) -> c_long;
    fn setsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut socklen_t,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn __errno_location() -> *mut c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_FALSE(actual: bool, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: *const c_void, expected: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;

    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool) -> c_int;

    fn cgroup_getset_retval_setsockopt__open_and_load(
    ) -> *mut cgroup_getset_retval_setsockopt;
    fn cgroup_getset_retval_setsockopt__destroy(obj: *mut cgroup_getset_retval_setsockopt);

    fn cgroup_getset_retval_getsockopt__open_and_load(
    ) -> *mut cgroup_getset_retval_getsockopt;
    fn cgroup_getset_retval_getsockopt__destroy(obj: *mut cgroup_getset_retval_getsockopt);

    fn cgroup_getset_retval_hooks__open() -> *mut cgroup_getset_retval_hooks;
    fn cgroup_getset_retval_hooks__load(skel: *mut cgroup_getset_retval_hooks) -> c_int;
    fn cgroup_getset_retval_hooks__destroy(skel: *mut cgroup_getset_retval_hooks);

    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_void,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
}

unsafe fn errno_value() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn test_setsockopt_set(cgroup_fd: c_int, sock_fd: c_int) {
    let obj: *mut cgroup_getset_retval_setsockopt;
    let mut link_set_eunatch: *mut bpf_link = ptr::null_mut();

    obj = unsafe { cgroup_getset_retval_setsockopt__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(obj as *const c_void, c"skel-load".as_ptr()) } {
        return;
    }

    unsafe { (*(*obj).bss).page_size = sysconf(_SC_PAGESIZE) };

    /* Attach setsockopt that sets EUNATCH, assert that
     * we actually get that error when we run setsockopt()
     */
    link_set_eunatch =
        unsafe { bpf_program__attach_cgroup((*obj).progs.set_eunatch, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_set_eunatch as *const c_void,
            c"cg-attach-set_eunatch".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    if !unsafe {
        ASSERT_ERR(
            setsockopt(
                sock_fd,
                SOL_SOCKET,
                SO_REUSEADDR,
                (&raw const zero).cast::<c_void>(),
                size_of::<c_int>() as socklen_t,
            ),
            c"setsockopt".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ(errno_value(), EUNATCH, c"setsockopt-errno".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    if !unsafe { ASSERT_EQ((*(*obj).bss).invocations, 1, c"invocations".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_FALSE((*(*obj).bss).assertion_error, c"assertion_error".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    unsafe { bpf_link__destroy(link_set_eunatch) };
    unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
}

unsafe fn test_setsockopt_set_and_get(cgroup_fd: c_int, sock_fd: c_int) {
    let obj: *mut cgroup_getset_retval_setsockopt;
    let mut link_set_eunatch: *mut bpf_link = ptr::null_mut();
    let mut link_get_retval: *mut bpf_link = ptr::null_mut();

    obj = unsafe { cgroup_getset_retval_setsockopt__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(obj as *const c_void, c"skel-load".as_ptr()) } {
        return;
    }

    unsafe { (*(*obj).bss).page_size = sysconf(_SC_PAGESIZE) };

    /* Attach setsockopt that sets EUNATCH, and one that gets the
     * previously set errno. Assert that we get the same errno back.
     */
    link_set_eunatch =
        unsafe { bpf_program__attach_cgroup((*obj).progs.set_eunatch, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_set_eunatch as *const c_void,
            c"cg-attach-set_eunatch".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    link_get_retval =
        unsafe { bpf_program__attach_cgroup((*obj).progs.get_retval, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_get_retval as *const c_void,
            c"cg-attach-get_retval".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    if !unsafe {
        ASSERT_ERR(
            setsockopt(
                sock_fd,
                SOL_SOCKET,
                SO_REUSEADDR,
                (&raw const zero).cast::<c_void>(),
                size_of::<c_int>() as socklen_t,
            ),
            c"setsockopt".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ(errno_value(), EUNATCH, c"setsockopt-errno".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    if !unsafe { ASSERT_EQ((*(*obj).bss).invocations, 2, c"invocations".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_FALSE((*(*obj).bss).assertion_error, c"assertion_error".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ((*(*obj).bss).retval_value, -EUNATCH, c"retval_value".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    unsafe { bpf_link__destroy(link_set_eunatch) };
    unsafe { bpf_link__destroy(link_get_retval) };
    unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
}

unsafe fn test_setsockopt_default_zero(cgroup_fd: c_int, sock_fd: c_int) {
    let obj: *mut cgroup_getset_retval_setsockopt;
    let mut link_get_retval: *mut bpf_link = ptr::null_mut();

    obj = unsafe { cgroup_getset_retval_setsockopt__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(obj as *const c_void, c"skel-load".as_ptr()) } {
        return;
    }

    unsafe { (*(*obj).bss).page_size = sysconf(_SC_PAGESIZE) };

    /* Attach setsockopt that gets the previously set errno.
     * Assert that, without anything setting one, we get 0.
     */
    link_get_retval =
        unsafe { bpf_program__attach_cgroup((*obj).progs.get_retval, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_get_retval as *const c_void,
            c"cg-attach-get_retval".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    if !unsafe {
        ASSERT_OK(
            setsockopt(
                sock_fd,
                SOL_SOCKET,
                SO_REUSEADDR,
                (&raw const zero).cast::<c_void>(),
                size_of::<c_int>() as socklen_t,
            ),
            c"setsockopt".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    if !unsafe { ASSERT_EQ((*(*obj).bss).invocations, 1, c"invocations".as_ptr()) } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_FALSE((*(*obj).bss).assertion_error, c"assertion_error".as_ptr()) } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ((*(*obj).bss).retval_value, 0, c"retval_value".as_ptr()) } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    unsafe { bpf_link__destroy(link_get_retval) };
    unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
}

unsafe fn test_setsockopt_default_zero_and_set(cgroup_fd: c_int, sock_fd: c_int) {
    let obj: *mut cgroup_getset_retval_setsockopt;
    let mut link_get_retval: *mut bpf_link = ptr::null_mut();
    let mut link_set_eunatch: *mut bpf_link = ptr::null_mut();

    obj = unsafe { cgroup_getset_retval_setsockopt__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(obj as *const c_void, c"skel-load".as_ptr()) } {
        return;
    }

    unsafe { (*(*obj).bss).page_size = sysconf(_SC_PAGESIZE) };

    /* Attach setsockopt that gets the previously set errno, and then
     * one that sets the errno to EUNATCH. Assert that the get does not
     * see EUNATCH set later, and does not prevent EUNATCH from being set.
     */
    link_get_retval =
        unsafe { bpf_program__attach_cgroup((*obj).progs.get_retval, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_get_retval as *const c_void,
            c"cg-attach-get_retval".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    link_set_eunatch =
        unsafe { bpf_program__attach_cgroup((*obj).progs.set_eunatch, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_set_eunatch as *const c_void,
            c"cg-attach-set_eunatch".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    if !unsafe {
        ASSERT_ERR(
            setsockopt(
                sock_fd,
                SOL_SOCKET,
                SO_REUSEADDR,
                (&raw const zero).cast::<c_void>(),
                size_of::<c_int>() as socklen_t,
            ),
            c"setsockopt".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ(errno_value(), EUNATCH, c"setsockopt-errno".as_ptr()) } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    if !unsafe { ASSERT_EQ((*(*obj).bss).invocations, 2, c"invocations".as_ptr()) } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_FALSE((*(*obj).bss).assertion_error, c"assertion_error".as_ptr()) } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ((*(*obj).bss).retval_value, 0, c"retval_value".as_ptr()) } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    unsafe { bpf_link__destroy(link_get_retval) };
    unsafe { bpf_link__destroy(link_set_eunatch) };
    unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
}

unsafe fn test_setsockopt_override(cgroup_fd: c_int, sock_fd: c_int) {
    let obj: *mut cgroup_getset_retval_setsockopt;
    let mut link_set_eunatch: *mut bpf_link = ptr::null_mut();
    let mut link_set_eisconn: *mut bpf_link = ptr::null_mut();
    let mut link_get_retval: *mut bpf_link = ptr::null_mut();

    obj = unsafe { cgroup_getset_retval_setsockopt__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(obj as *const c_void, c"skel-load".as_ptr()) } {
        return;
    }

    unsafe { (*(*obj).bss).page_size = sysconf(_SC_PAGESIZE) };

    /* Attach setsockopt that sets EUNATCH, then one that sets EISCONN,
     * and then one that gets the exported errno. Assert both the syscall
     * and the helper sees the last set errno.
     */
    link_set_eunatch =
        unsafe { bpf_program__attach_cgroup((*obj).progs.set_eunatch, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_set_eunatch as *const c_void,
            c"cg-attach-set_eunatch".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    link_set_eisconn =
        unsafe { bpf_program__attach_cgroup((*obj).progs.set_eisconn, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_set_eisconn as *const c_void,
            c"cg-attach-set_eisconn".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    link_get_retval =
        unsafe { bpf_program__attach_cgroup((*obj).progs.get_retval, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_get_retval as *const c_void,
            c"cg-attach-get_retval".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    if !unsafe {
        ASSERT_ERR(
            setsockopt(
                sock_fd,
                SOL_SOCKET,
                SO_REUSEADDR,
                (&raw const zero).cast::<c_void>(),
                size_of::<c_int>() as socklen_t,
            ),
            c"setsockopt".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ(errno_value(), EISCONN, c"setsockopt-errno".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    if !unsafe { ASSERT_EQ((*(*obj).bss).invocations, 3, c"invocations".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_FALSE((*(*obj).bss).assertion_error, c"assertion_error".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ((*(*obj).bss).retval_value, -EISCONN, c"retval_value".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    unsafe { bpf_link__destroy(link_set_eunatch) };
    unsafe { bpf_link__destroy(link_set_eisconn) };
    unsafe { bpf_link__destroy(link_get_retval) };
    unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
}

unsafe fn test_setsockopt_legacy_eperm(cgroup_fd: c_int, sock_fd: c_int) {
    let obj: *mut cgroup_getset_retval_setsockopt;
    let mut link_legacy_eperm: *mut bpf_link = ptr::null_mut();
    let mut link_get_retval: *mut bpf_link = ptr::null_mut();

    obj = unsafe { cgroup_getset_retval_setsockopt__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(obj as *const c_void, c"skel-load".as_ptr()) } {
        return;
    }

    unsafe { (*(*obj).bss).page_size = sysconf(_SC_PAGESIZE) };

    /* Attach setsockopt that return a reject without setting errno
     * (legacy reject), and one that gets the errno. Assert that for
     * backward compatibility the syscall result in EPERM, and this
     * is also visible to the helper.
     */
    link_legacy_eperm =
        unsafe { bpf_program__attach_cgroup((*obj).progs.legacy_eperm, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_legacy_eperm as *const c_void,
            c"cg-attach-legacy_eperm".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    link_get_retval =
        unsafe { bpf_program__attach_cgroup((*obj).progs.get_retval, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_get_retval as *const c_void,
            c"cg-attach-get_retval".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    if !unsafe {
        ASSERT_ERR(
            setsockopt(
                sock_fd,
                SOL_SOCKET,
                SO_REUSEADDR,
                (&raw const zero).cast::<c_void>(),
                size_of::<c_int>() as socklen_t,
            ),
            c"setsockopt".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ(errno_value(), EPERM, c"setsockopt-errno".as_ptr()) } {
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    if !unsafe { ASSERT_EQ((*(*obj).bss).invocations, 2, c"invocations".as_ptr()) } {
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_FALSE((*(*obj).bss).assertion_error, c"assertion_error".as_ptr()) } {
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ((*(*obj).bss).retval_value, -EPERM, c"retval_value".as_ptr()) } {
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    unsafe { bpf_link__destroy(link_legacy_eperm) };
    unsafe { bpf_link__destroy(link_get_retval) };
    unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
}

unsafe fn test_setsockopt_legacy_no_override(cgroup_fd: c_int, sock_fd: c_int) {
    let obj: *mut cgroup_getset_retval_setsockopt;
    let mut link_set_eunatch: *mut bpf_link = ptr::null_mut();
    let mut link_legacy_eperm: *mut bpf_link = ptr::null_mut();
    let mut link_get_retval: *mut bpf_link = ptr::null_mut();

    obj = unsafe { cgroup_getset_retval_setsockopt__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(obj as *const c_void, c"skel-load".as_ptr()) } {
        return;
    }

    unsafe { (*(*obj).bss).page_size = sysconf(_SC_PAGESIZE) };

    /* Attach setsockopt that sets EUNATCH, then one that return a reject
     * without setting errno, and then one that gets the exported errno.
     * Assert both the syscall and the helper's errno are unaffected by
     * the second prog (i.e. legacy rejects does not override the errno
     * to EPERM).
     */
    link_set_eunatch =
        unsafe { bpf_program__attach_cgroup((*obj).progs.set_eunatch, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_set_eunatch as *const c_void,
            c"cg-attach-set_eunatch".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    link_legacy_eperm =
        unsafe { bpf_program__attach_cgroup((*obj).progs.legacy_eperm, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_legacy_eperm as *const c_void,
            c"cg-attach-legacy_eperm".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    link_get_retval =
        unsafe { bpf_program__attach_cgroup((*obj).progs.get_retval, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_get_retval as *const c_void,
            c"cg-attach-get_retval".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    if !unsafe {
        ASSERT_ERR(
            setsockopt(
                sock_fd,
                SOL_SOCKET,
                SO_REUSEADDR,
                (&raw const zero).cast::<c_void>(),
                size_of::<c_int>() as socklen_t,
            ),
            c"setsockopt".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ(errno_value(), EUNATCH, c"setsockopt-errno".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    if !unsafe { ASSERT_EQ((*(*obj).bss).invocations, 3, c"invocations".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_FALSE((*(*obj).bss).assertion_error, c"assertion_error".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ((*(*obj).bss).retval_value, -EUNATCH, c"retval_value".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eunatch) };
        unsafe { bpf_link__destroy(link_legacy_eperm) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
        return;
    }

    unsafe { bpf_link__destroy(link_set_eunatch) };
    unsafe { bpf_link__destroy(link_legacy_eperm) };
    unsafe { bpf_link__destroy(link_get_retval) };
    unsafe { cgroup_getset_retval_setsockopt__destroy(obj) };
}

unsafe fn test_getsockopt_get(cgroup_fd: c_int, sock_fd: c_int) {
    let obj: *mut cgroup_getset_retval_getsockopt;
    let mut link_get_retval: *mut bpf_link = ptr::null_mut();
    let mut buf: c_int = 0;
    let mut optlen: socklen_t = size_of::<c_int>() as socklen_t;

    obj = unsafe { cgroup_getset_retval_getsockopt__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(obj as *const c_void, c"skel-load".as_ptr()) } {
        return;
    }

    unsafe { (*(*obj).bss).page_size = sysconf(_SC_PAGESIZE) };

    /* Attach getsockopt that gets previously set errno. Assert that the
     * error from kernel is in both ctx_retval_value and retval_value.
     */
    link_get_retval =
        unsafe { bpf_program__attach_cgroup((*obj).progs.get_retval, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_get_retval as *const c_void,
            c"cg-attach-get_retval".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }

    if !unsafe {
        ASSERT_ERR(
            getsockopt(
                sock_fd,
                SOL_CUSTOM,
                0,
                (&mut buf as *mut c_int).cast::<c_void>(),
                &mut optlen,
            ),
            c"getsockopt".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ(errno_value(), EOPNOTSUPP, c"getsockopt-errno".as_ptr()) } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }

    if !unsafe { ASSERT_EQ((*(*obj).bss).invocations, 1, c"invocations".as_ptr()) } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_FALSE((*(*obj).bss).assertion_error, c"assertion_error".as_ptr()) } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ((*(*obj).bss).retval_value, -EOPNOTSUPP, c"retval_value".as_ptr()) } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }
    if !unsafe {
        ASSERT_EQ(
            (*(*obj).bss).ctx_retval_value,
            -EOPNOTSUPP,
            c"ctx_retval_value".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }

    unsafe { bpf_link__destroy(link_get_retval) };
    unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
}

unsafe fn test_getsockopt_override(cgroup_fd: c_int, sock_fd: c_int) {
    let obj: *mut cgroup_getset_retval_getsockopt;
    let mut link_set_eisconn: *mut bpf_link = ptr::null_mut();
    let mut buf: c_int = 0;
    let mut optlen: socklen_t = size_of::<c_int>() as socklen_t;

    obj = unsafe { cgroup_getset_retval_getsockopt__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(obj as *const c_void, c"skel-load".as_ptr()) } {
        return;
    }

    unsafe { (*(*obj).bss).page_size = sysconf(_SC_PAGESIZE) };

    /* Attach getsockopt that sets retval to -EISCONN. Assert that this
     * overrides the value from kernel.
     */
    link_set_eisconn =
        unsafe { bpf_program__attach_cgroup((*obj).progs.set_eisconn, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_set_eisconn as *const c_void,
            c"cg-attach-set_eisconn".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }

    if !unsafe {
        ASSERT_ERR(
            getsockopt(
                sock_fd,
                SOL_CUSTOM,
                0,
                (&mut buf as *mut c_int).cast::<c_void>(),
                &mut optlen,
            ),
            c"getsockopt".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ(errno_value(), EISCONN, c"getsockopt-errno".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }

    if !unsafe { ASSERT_EQ((*(*obj).bss).invocations, 1, c"invocations".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_FALSE((*(*obj).bss).assertion_error, c"assertion_error".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }

    unsafe { bpf_link__destroy(link_set_eisconn) };
    unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
}

unsafe fn test_getsockopt_retval_sync(cgroup_fd: c_int, sock_fd: c_int) {
    let obj: *mut cgroup_getset_retval_getsockopt;
    let mut link_set_eisconn: *mut bpf_link = ptr::null_mut();
    let mut link_clear_retval: *mut bpf_link = ptr::null_mut();
    let mut link_get_retval: *mut bpf_link = ptr::null_mut();
    let mut buf: c_int = 0;
    let mut optlen: socklen_t = size_of::<c_int>() as socklen_t;

    obj = unsafe { cgroup_getset_retval_getsockopt__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(obj as *const c_void, c"skel-load".as_ptr()) } {
        return;
    }

    unsafe { (*(*obj).bss).page_size = sysconf(_SC_PAGESIZE) };

    /* Attach getsockopt that sets retval to -EISCONN, and one that clears
     * ctx retval. Assert that the clearing ctx retval is synced to helper
     * and clears any errors both from kernel and BPF..
     */
    link_set_eisconn =
        unsafe { bpf_program__attach_cgroup((*obj).progs.set_eisconn, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_set_eisconn as *const c_void,
            c"cg-attach-set_eisconn".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_clear_retval) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }
    link_clear_retval =
        unsafe { bpf_program__attach_cgroup((*obj).progs.clear_retval, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_clear_retval as *const c_void,
            c"cg-attach-clear_retval".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_clear_retval) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }
    link_get_retval =
        unsafe { bpf_program__attach_cgroup((*obj).progs.get_retval, cgroup_fd) };
    if !unsafe {
        ASSERT_OK_PTR(
            link_get_retval as *const c_void,
            c"cg-attach-get_retval".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_clear_retval) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }

    if !unsafe {
        ASSERT_OK(
            getsockopt(
                sock_fd,
                SOL_CUSTOM,
                0,
                (&mut buf as *mut c_int).cast::<c_void>(),
                &mut optlen,
            ),
            c"getsockopt".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_clear_retval) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }

    if !unsafe { ASSERT_EQ((*(*obj).bss).invocations, 3, c"invocations".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_clear_retval) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_FALSE((*(*obj).bss).assertion_error, c"assertion_error".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_clear_retval) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }
    if !unsafe { ASSERT_EQ((*(*obj).bss).retval_value, 0, c"retval_value".as_ptr()) } {
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_clear_retval) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }
    if !unsafe {
        ASSERT_EQ(
            (*(*obj).bss).ctx_retval_value,
            0,
            c"ctx_retval_value".as_ptr(),
        )
    } {
        unsafe { bpf_link__destroy(link_set_eisconn) };
        unsafe { bpf_link__destroy(link_clear_retval) };
        unsafe { bpf_link__destroy(link_get_retval) };
        unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
        return;
    }

    unsafe { bpf_link__destroy(link_set_eisconn) };
    unsafe { bpf_link__destroy(link_clear_retval) };
    unsafe { bpf_link__destroy(link_get_retval) };
    unsafe { cgroup_getset_retval_getsockopt__destroy(obj) };
}

/* Original C expands exposed_hooks[] from "cgroup_getset_retval_hooks.h":
 *
 * #define BPF_RETVAL_HOOK(NAME, SECTION, CTX, EXPECTED_ERR) \
 *      { .name = #NAME, .expected_err = EXPECTED_ERR, },
 * #include "cgroup_getset_retval_hooks.h"
 *
 * The isolated translation is not allowed to read that header, so the generated
 * entries remain an external dependency.
 */
unsafe extern "C" {
    static exposed_hooks: [exposed_hook; 0];
}

unsafe fn test_exposed_hooks(cgroup_fd: c_int, sock_fd: c_int) {
    let mut skel: *mut cgroup_getset_retval_hooks;
    let mut prog: *mut bpf_program;
    let mut err: c_int;
    let mut i: usize;

    let _ = cgroup_fd;
    let _ = sock_fd;

    i = 0;
    while i < exposed_hooks.len() {
        skel = unsafe { cgroup_getset_retval_hooks__open() };
        if !unsafe {
            ASSERT_OK_PTR(
                skel as *const c_void,
                c"cgroup_getset_retval_hooks__open".as_ptr(),
            )
        } {
            i += 1;
            continue;
        }

        prog = unsafe { bpf_object__find_program_by_name((*skel).obj, exposed_hooks[i].name) };
        if !unsafe {
            ASSERT_NEQ(
                prog as *const c_void,
                ptr::null(),
                c"bpf_object__find_program_by_name".as_ptr(),
            )
        } {
            unsafe { cgroup_getset_retval_hooks__destroy(skel) };
            i += 1;
            continue;
        }

        err = unsafe { bpf_program__set_autoload(prog, true) };
        if !unsafe { ASSERT_OK(err, c"bpf_program__set_autoload".as_ptr()) } {
            unsafe { cgroup_getset_retval_hooks__destroy(skel) };
            i += 1;
            continue;
        }

        err = unsafe { cgroup_getset_retval_hooks__load(skel) };
        unsafe { ASSERT_EQ(err, exposed_hooks[i].expected_err, c"expected_err".as_ptr()) };

        unsafe { cgroup_getset_retval_hooks__destroy(skel) };
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgroup_getset_retval() {
    let mut cgroup_fd: c_int = -1;
    let mut sock_fd: c_int = -1;

    cgroup_fd = unsafe { test__join_cgroup(c"/cgroup_getset_retval".as_ptr()) };
    if !unsafe { ASSERT_GE(cgroup_fd, 0, c"cg-create".as_ptr()) } {
        unsafe { close(cgroup_fd) };
        return;
    }

    sock_fd = unsafe { start_server(AF_INET, SOCK_DGRAM, ptr::null(), 0, 0) };
    if !unsafe { ASSERT_GE(sock_fd, 0, c"start-server".as_ptr()) } {
        unsafe { close(cgroup_fd) };
        return;
    }

    if unsafe { test__start_subtest(c"setsockopt-set".as_ptr()) } {
        unsafe { test_setsockopt_set(cgroup_fd, sock_fd) };
    }

    if unsafe { test__start_subtest(c"setsockopt-set_and_get".as_ptr()) } {
        unsafe { test_setsockopt_set_and_get(cgroup_fd, sock_fd) };
    }

    if unsafe { test__start_subtest(c"setsockopt-default_zero".as_ptr()) } {
        unsafe { test_setsockopt_default_zero(cgroup_fd, sock_fd) };
    }

    if unsafe { test__start_subtest(c"setsockopt-default_zero_and_set".as_ptr()) } {
        unsafe { test_setsockopt_default_zero_and_set(cgroup_fd, sock_fd) };
    }

    if unsafe { test__start_subtest(c"setsockopt-override".as_ptr()) } {
        unsafe { test_setsockopt_override(cgroup_fd, sock_fd) };
    }

    if unsafe { test__start_subtest(c"setsockopt-legacy_eperm".as_ptr()) } {
        unsafe { test_setsockopt_legacy_eperm(cgroup_fd, sock_fd) };
    }

    if unsafe { test__start_subtest(c"setsockopt-legacy_no_override".as_ptr()) } {
        unsafe { test_setsockopt_legacy_no_override(cgroup_fd, sock_fd) };
    }

    if unsafe { test__start_subtest(c"getsockopt-get".as_ptr()) } {
        unsafe { test_getsockopt_get(cgroup_fd, sock_fd) };
    }

    if unsafe { test__start_subtest(c"getsockopt-override".as_ptr()) } {
        unsafe { test_getsockopt_override(cgroup_fd, sock_fd) };
    }

    if unsafe { test__start_subtest(c"getsockopt-retval_sync".as_ptr()) } {
        unsafe { test_getsockopt_retval_sync(cgroup_fd, sock_fd) };
    }

    if unsafe { test__start_subtest(c"exposed_hooks".as_ptr()) } {
        unsafe { test_exposed_hooks(cgroup_fd, sock_fd) };
    }

    unsafe { close(cgroup_fd) };
}
