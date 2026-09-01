// SPDX-License-Identifier: GPL-2.0

// Translated from:
// testing/selftests/bpf/prog_tests/cgroup_get_current_cgroup_id.c
//
// C dependencies:
// #include <sys/stat.h>
// #include <sys/sysmacros.h>
// #include "test_progs.h"
// #include "cgroup_helpers.h"
// #include "get_cgroup_id_kern.skel.h"

const TEST_CGROUP: *const ::std::os::raw::c_char =
    b"/test-bpf-get-cgroup-id/\0".as_ptr() as *const ::std::os::raw::c_char;

#[repr(C)]
pub struct timespec {
    pub tv_sec: libc::time_t,
    pub tv_nsec: libc::c_long,
}

#[repr(C)]
pub struct get_cgroup_id_kern__bss {
    pub expected_pid: libc::pid_t,
    pub cg_id: u64,
}

#[repr(C)]
pub struct get_cgroup_id_kern {
    pub bss: *mut get_cgroup_id_kern__bss,
}

extern "C" {
    fn cgroup_setup_and_join(path: *const ::std::os::raw::c_char) -> libc::c_int;
    fn get_cgroup_id_kern__open_and_load() -> *mut get_cgroup_id_kern;
    fn get_cgroup_id_kern__attach(skel: *mut get_cgroup_id_kern) -> libc::c_int;
    fn get_cgroup_id_kern__destroy(skel: *mut get_cgroup_id_kern);
    fn get_cgroup_id(path: *const ::std::os::raw::c_char) -> u64;
    fn cleanup_cgroup_environment();

    fn ASSERT_OK_FD(fd: libc::c_int, name: *const ::std::os::raw::c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const ::std::ffi::c_void, name: *const ::std::os::raw::c_char) -> bool;
    fn ASSERT_OK(res: libc::c_long, name: *const ::std::os::raw::c_char) -> bool;
    fn ASSERT_EQ(
        actual: u64,
        expected: u64,
        name: *const ::std::os::raw::c_char,
    ) -> bool;

    fn getpid() -> libc::pid_t;
    fn syscall(num: libc::c_long, ...) -> libc::c_long;
    fn close(fd: libc::c_int) -> libc::c_int;
}

pub unsafe fn test_cgroup_get_current_cgroup_id() {
    let mut skel: *mut get_cgroup_id_kern;
    let req: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 1,
    };
    let cgroup_fd: libc::c_int;
    let ucgid: u64;

    cgroup_fd = cgroup_setup_and_join(TEST_CGROUP);
    if !ASSERT_OK_FD(cgroup_fd, b"cgroup switch\0".as_ptr() as *const ::std::os::raw::c_char) {
        return;
    }

    skel = get_cgroup_id_kern__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const ::std::ffi::c_void,
        b"load program\0".as_ptr() as *const ::std::os::raw::c_char,
    ) {
        close(cgroup_fd);
        cleanup_cgroup_environment();
        return;
    }

    if !ASSERT_OK(
        get_cgroup_id_kern__attach(skel) as libc::c_long,
        b"attach bpf program\0".as_ptr() as *const ::std::os::raw::c_char,
    ) {
        get_cgroup_id_kern__destroy(skel);
        close(cgroup_fd);
        cleanup_cgroup_environment();
        return;
    }

    (*(*skel).bss).expected_pid = getpid();
    /* trigger the syscall on which is attached the tested prog */
    if !ASSERT_OK(
        syscall(libc::SYS_nanosleep as libc::c_long, &req as *const timespec, ::std::ptr::null::<libc::c_void>()),
        b"nanosleep\0".as_ptr() as *const ::std::os::raw::c_char,
    ) {
        get_cgroup_id_kern__destroy(skel);
        close(cgroup_fd);
        cleanup_cgroup_environment();
        return;
    }

    ucgid = get_cgroup_id(TEST_CGROUP);

    ASSERT_EQ((*(*skel).bss).cg_id, ucgid, b"compare cgroup ids\0".as_ptr() as *const ::std::os::raw::c_char);

    get_cgroup_id_kern__destroy(skel);
    close(cgroup_fd);
    cleanup_cgroup_environment();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
