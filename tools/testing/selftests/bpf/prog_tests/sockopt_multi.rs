// SPDX-License-Identifier: GPL-2.0
// Translated from C source:
//   testing/selftests/bpf/prog_tests/sockopt_multi.c
//
// Original dependencies:
//   <test_progs.h>
//   "cgroup_helpers.h"
//   "sockopt_multi.skel.h"

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type socklen_t = c_uint;
type __u8 = u8;

const SOL_IP: c_int = 0;
const IP_TOS: c_int = 1;
const AF_INET: c_int = 2;
const SOCK_STREAM: c_int = 1;
const _SC_PAGESIZE: c_int = 30;

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockopt_multi_bss {
    pub page_size: c_long,
}

#[repr(C)]
pub struct sockopt_multi_progs {
    pub _getsockopt_child: *mut bpf_program,
    pub _getsockopt_parent: *mut bpf_program,
    pub _setsockopt: *mut bpf_program,
}

#[repr(C)]
pub struct sockopt_multi {
    pub progs: sockopt_multi_progs,
    pub bss: *mut sockopt_multi_bss,
}

unsafe extern "C" {
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
    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn sysconf(name: c_int) -> c_long;

    fn log_err(fmt: *const c_char, ...);
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;

    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn sockopt_multi__open_and_load() -> *mut sockopt_multi;
    fn sockopt_multi__destroy(obj: *mut sockopt_multi);
}

unsafe fn run_getsockopt_test(
    obj: *mut sockopt_multi,
    cg_parent: c_int,
    cg_child: c_int,
    sock_fd: c_int,
) -> c_int {
    let mut link_parent: *mut bpf_link = core::ptr::null_mut();
    let mut link_child: *mut bpf_link = core::ptr::null_mut();
    let mut optlen: socklen_t;
    let mut buf: __u8;
    let mut err: c_int;

    /* Set IP_TOS to the expected value (0x80). */

    buf = 0x80;
    err = setsockopt(sock_fd, SOL_IP, IP_TOS, &buf as *const __u8 as *const c_void, 1);
    if err < 0 {
        log_err(c"Failed to call setsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    buf = 0x00;
    optlen = 1;
    err = getsockopt(
        sock_fd,
        SOL_IP,
        IP_TOS,
        &mut buf as *mut __u8 as *mut c_void,
        &mut optlen,
    );
    if err != 0 {
        log_err(c"Failed to call getsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    if buf != 0x80 {
        log_err(c"Unexpected getsockopt 0x%x != 0x80 without BPF".as_ptr(), buf as c_int);
        err = -1;
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    /* Attach child program and make sure it returns new value:
     * - kernel:      -> 0x80
     * - child:  0x80 -> 0x90
     */

    link_child = bpf_program__attach_cgroup((*obj).progs._getsockopt_child, cg_child);
    if !ASSERT_OK_PTR(link_child as *mut c_void, c"cg-attach-getsockopt_child".as_ptr()) {
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    buf = 0x00;
    optlen = 1;
    err = getsockopt(
        sock_fd,
        SOL_IP,
        IP_TOS,
        &mut buf as *mut __u8 as *mut c_void,
        &mut optlen,
    );
    if err != 0 {
        log_err(c"Failed to call getsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    if buf != 0x90 {
        log_err(c"Unexpected getsockopt 0x%x != 0x90".as_ptr(), buf as c_int);
        err = -1;
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    /* Attach parent program and make sure it returns new value:
     * - kernel:      -> 0x80
     * - child:  0x80 -> 0x90
     * - parent: 0x90 -> 0xA0
     */

    link_parent = bpf_program__attach_cgroup((*obj).progs._getsockopt_parent, cg_parent);
    if !ASSERT_OK_PTR(link_parent as *mut c_void, c"cg-attach-getsockopt_parent".as_ptr()) {
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    buf = 0x00;
    optlen = 1;
    err = getsockopt(
        sock_fd,
        SOL_IP,
        IP_TOS,
        &mut buf as *mut __u8 as *mut c_void,
        &mut optlen,
    );
    if err != 0 {
        log_err(c"Failed to call getsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    if buf != 0xA0 {
        log_err(c"Unexpected getsockopt 0x%x != 0xA0".as_ptr(), buf as c_int);
        err = -1;
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    /* Setting unexpected initial sockopt should return EPERM:
     * - kernel: -> 0x40
     * - child:  unexpected 0x40, EPERM
     * - parent: unexpected 0x40, EPERM
     */

    buf = 0x40;
    err = setsockopt(sock_fd, SOL_IP, IP_TOS, &buf as *const __u8 as *const c_void, 1);
    if err < 0 {
        log_err(c"Failed to call setsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    buf = 0x00;
    optlen = 1;
    err = getsockopt(
        sock_fd,
        SOL_IP,
        IP_TOS,
        &mut buf as *mut __u8 as *mut c_void,
        &mut optlen,
    );
    if err == 0 {
        log_err(c"Unexpected success from getsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    /* Detach child program and make sure we still get EPERM:
     * - kernel: -> 0x40
     * - parent: unexpected 0x40, EPERM
     */

    bpf_link__destroy(link_child);
    link_child = core::ptr::null_mut();

    buf = 0x00;
    optlen = 1;
    err = getsockopt(
        sock_fd,
        SOL_IP,
        IP_TOS,
        &mut buf as *mut __u8 as *mut c_void,
        &mut optlen,
    );
    if err == 0 {
        log_err(c"Unexpected success from getsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    /* Set initial value to the one the parent program expects:
     * - kernel:      -> 0x90
     * - parent: 0x90 -> 0xA0
     */

    buf = 0x90;
    err = setsockopt(sock_fd, SOL_IP, IP_TOS, &buf as *const __u8 as *const c_void, 1);
    if err < 0 {
        log_err(c"Failed to call setsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    buf = 0x00;
    optlen = 1;
    err = getsockopt(
        sock_fd,
        SOL_IP,
        IP_TOS,
        &mut buf as *mut __u8 as *mut c_void,
        &mut optlen,
    );
    if err != 0 {
        log_err(c"Failed to call getsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    if buf != 0xA0 {
        log_err(c"Unexpected getsockopt 0x%x != 0xA0".as_ptr(), buf as c_int);
        err = -1;
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    bpf_link__destroy(link_child);
    bpf_link__destroy(link_parent);

    err
}

unsafe fn run_setsockopt_test(
    obj: *mut sockopt_multi,
    cg_parent: c_int,
    cg_child: c_int,
    sock_fd: c_int,
) -> c_int {
    let mut link_parent: *mut bpf_link = core::ptr::null_mut();
    let mut link_child: *mut bpf_link = core::ptr::null_mut();
    let mut optlen: socklen_t;
    let mut buf: __u8;
    let mut err: c_int;

    /* Set IP_TOS to the expected value (0x80). */

    buf = 0x80;
    err = setsockopt(sock_fd, SOL_IP, IP_TOS, &buf as *const __u8 as *const c_void, 1);
    if err < 0 {
        log_err(c"Failed to call setsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    buf = 0x00;
    optlen = 1;
    err = getsockopt(
        sock_fd,
        SOL_IP,
        IP_TOS,
        &mut buf as *mut __u8 as *mut c_void,
        &mut optlen,
    );
    if err != 0 {
        log_err(c"Failed to call getsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    if buf != 0x80 {
        log_err(c"Unexpected getsockopt 0x%x != 0x80 without BPF".as_ptr(), buf as c_int);
        err = -1;
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    /* Attach child program and make sure it adds 0x10. */

    link_child = bpf_program__attach_cgroup((*obj).progs._setsockopt, cg_child);
    if !ASSERT_OK_PTR(link_child as *mut c_void, c"cg-attach-setsockopt_child".as_ptr()) {
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    buf = 0x80;
    err = setsockopt(sock_fd, SOL_IP, IP_TOS, &buf as *const __u8 as *const c_void, 1);
    if err < 0 {
        log_err(c"Failed to call setsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    buf = 0x00;
    optlen = 1;
    err = getsockopt(
        sock_fd,
        SOL_IP,
        IP_TOS,
        &mut buf as *mut __u8 as *mut c_void,
        &mut optlen,
    );
    if err != 0 {
        log_err(c"Failed to call getsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    if buf != 0x80 + 0x10 {
        log_err(c"Unexpected getsockopt 0x%x != 0x80 + 0x10".as_ptr(), buf as c_int);
        err = -1;
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    /* Attach parent program and make sure it adds another 0x10. */

    link_parent = bpf_program__attach_cgroup((*obj).progs._setsockopt, cg_parent);
    if !ASSERT_OK_PTR(link_parent as *mut c_void, c"cg-attach-setsockopt_parent".as_ptr()) {
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    buf = 0x80;
    err = setsockopt(sock_fd, SOL_IP, IP_TOS, &buf as *const __u8 as *const c_void, 1);
    if err < 0 {
        log_err(c"Failed to call setsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    buf = 0x00;
    optlen = 1;
    err = getsockopt(
        sock_fd,
        SOL_IP,
        IP_TOS,
        &mut buf as *mut __u8 as *mut c_void,
        &mut optlen,
    );
    if err != 0 {
        log_err(c"Failed to call getsockopt(IP_TOS)".as_ptr());
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    if buf != 0x80 + 2 * 0x10 {
        log_err(c"Unexpected getsockopt 0x%x != 0x80 + 2 * 0x10".as_ptr(), buf as c_int);
        err = -1;
        bpf_link__destroy(link_child);
        bpf_link__destroy(link_parent);
        return err;
    }

    bpf_link__destroy(link_child);
    bpf_link__destroy(link_parent);

    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_sockopt_multi() {
    let mut cg_parent: c_int = -1;
    let mut cg_child: c_int = -1;
    let mut obj: *mut sockopt_multi = core::ptr::null_mut();
    let mut sock_fd: c_int = -1;

    cg_parent = test__join_cgroup(c"/parent".as_ptr());
    if !ASSERT_GE(cg_parent, 0, c"join_cgroup /parent".as_ptr()) {
        close(sock_fd);
        sockopt_multi__destroy(obj);
        close(cg_child);
        close(cg_parent);
        return;
    }

    cg_child = test__join_cgroup(c"/parent/child".as_ptr());
    if !ASSERT_GE(cg_child, 0, c"join_cgroup /parent/child".as_ptr()) {
        close(sock_fd);
        sockopt_multi__destroy(obj);
        close(cg_child);
        close(cg_parent);
        return;
    }

    obj = sockopt_multi__open_and_load();
    if !ASSERT_OK_PTR(obj as *mut c_void, c"skel-load".as_ptr()) {
        close(sock_fd);
        sockopt_multi__destroy(obj);
        close(cg_child);
        close(cg_parent);
        return;
    }

    (*(*obj).bss).page_size = sysconf(_SC_PAGESIZE);

    sock_fd = socket(AF_INET, SOCK_STREAM, 0);
    if !ASSERT_GE(sock_fd, 0, c"socket".as_ptr()) {
        close(sock_fd);
        sockopt_multi__destroy(obj);
        close(cg_child);
        close(cg_parent);
        return;
    }

    ASSERT_OK(
        run_getsockopt_test(obj, cg_parent, cg_child, sock_fd),
        c"getsockopt_test".as_ptr(),
    );
    ASSERT_OK(
        run_setsockopt_test(obj, cg_parent, cg_child, sock_fd),
        c"setsockopt_test".as_ptr(),
    );

    close(sock_fd);
    sockopt_multi__destroy(obj);
    close(cg_child);
    close(cg_parent);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
