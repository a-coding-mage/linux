// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

/* Dependency intent from C includes:
 * errno.h, fcntl.h, sys/stat.h, string.h, unistd.h, sys/socket.h,
 * test_progs.h, cgroup_helpers.h, read_cgroupfs_xattr.skel.h,
 * cgroup_read_xattr.skel.h.
 */

use core::ffi::{c_char, c_int, c_void};

const CGROUP_FS_PARENT: &[u8] = b"foo/\0";
const CGROUP_FS_CHILD: &[u8] = b"foo/bar/\0";
const TMP_FILE: &[u8] = b"/tmp/selftests_cgroup_xattr\0";

static xattr_value_a: &[u8] = b"bpf_selftest_value_a\0";
static xattr_value_b: &[u8] = b"bpf_selftest_value_b\0";
static xattr_name: &[u8] = b"user.bpf_test\0";

const O_RDONLY: c_int = 0;
const O_CREAT: c_int = 0o100;

#[repr(C)]
pub struct read_cgroupfs_xattr_bss {
    pub target_pid: c_int,
    pub found_value_a: bool,
    pub found_value_b: bool,
}

#[repr(C)]
pub struct read_cgroupfs_xattr {
    pub bss: *mut read_cgroupfs_xattr_bss,
}

unsafe extern "C" {
    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn set_cgroup_xattr(
        path: *const c_char,
        name: *const c_char,
        value: *const c_char,
    ) -> c_int;
    fn sys_gettid() -> c_int;

    fn read_cgroupfs_xattr__open_and_load() -> *mut read_cgroupfs_xattr;
    fn read_cgroupfs_xattr__attach(skel: *mut read_cgroupfs_xattr) -> c_int;
    fn read_cgroupfs_xattr__destroy(skel: *mut read_cgroupfs_xattr);

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;

    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_TRUE(condition: bool, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn RUN_TESTS_cgroup_read_xattr();
}

unsafe fn test_read_cgroup_xattr() {
    let mut tmp_fd: c_int;
    let mut parent_cgroup_fd: c_int = -1;
    let mut child_cgroup_fd: c_int = -1;
    let mut skel: *mut read_cgroupfs_xattr = core::ptr::null_mut();

    parent_cgroup_fd = test__join_cgroup(CGROUP_FS_PARENT.as_ptr() as *const c_char);
    if !ASSERT_OK_FD(parent_cgroup_fd, c"create parent cgroup".as_ptr()) {
        return;
    }
    if !ASSERT_OK(
        set_cgroup_xattr(
            CGROUP_FS_PARENT.as_ptr() as *const c_char,
            xattr_name.as_ptr() as *const c_char,
            xattr_value_a.as_ptr() as *const c_char,
        ),
        c"set parent xattr".as_ptr(),
    ) {
        close(child_cgroup_fd);
        close(parent_cgroup_fd);
        read_cgroupfs_xattr__destroy(skel);
        unlink(TMP_FILE.as_ptr() as *const c_char);
        return;
    }

    child_cgroup_fd = test__join_cgroup(CGROUP_FS_CHILD.as_ptr() as *const c_char);
    if !ASSERT_OK_FD(child_cgroup_fd, c"create child cgroup".as_ptr()) {
        close(child_cgroup_fd);
        close(parent_cgroup_fd);
        read_cgroupfs_xattr__destroy(skel);
        unlink(TMP_FILE.as_ptr() as *const c_char);
        return;
    }
    if !ASSERT_OK(
        set_cgroup_xattr(
            CGROUP_FS_CHILD.as_ptr() as *const c_char,
            xattr_name.as_ptr() as *const c_char,
            xattr_value_b.as_ptr() as *const c_char,
        ),
        c"set child xattr".as_ptr(),
    ) {
        close(child_cgroup_fd);
        close(parent_cgroup_fd);
        read_cgroupfs_xattr__destroy(skel);
        unlink(TMP_FILE.as_ptr() as *const c_char);
        return;
    }

    skel = read_cgroupfs_xattr__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        c"read_cgroupfs_xattr__open_and_load".as_ptr(),
    ) {
        close(child_cgroup_fd);
        close(parent_cgroup_fd);
        read_cgroupfs_xattr__destroy(skel);
        unlink(TMP_FILE.as_ptr() as *const c_char);
        return;
    }

    (*(*skel).bss).target_pid = sys_gettid();

    if !ASSERT_OK(
        read_cgroupfs_xattr__attach(skel),
        c"read_cgroupfs_xattr__attach".as_ptr(),
    ) {
        close(child_cgroup_fd);
        close(parent_cgroup_fd);
        read_cgroupfs_xattr__destroy(skel);
        unlink(TMP_FILE.as_ptr() as *const c_char);
        return;
    }

    tmp_fd = open(TMP_FILE.as_ptr() as *const c_char, O_RDONLY | O_CREAT);
    ASSERT_OK_FD(tmp_fd, c"open tmp file".as_ptr());
    close(tmp_fd);

    ASSERT_TRUE((*(*skel).bss).found_value_a, c"found_value_a".as_ptr());
    ASSERT_TRUE((*(*skel).bss).found_value_b, c"found_value_b".as_ptr());

    close(child_cgroup_fd);
    close(parent_cgroup_fd);
    read_cgroupfs_xattr__destroy(skel);
    unlink(TMP_FILE.as_ptr() as *const c_char);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgroup_xattr() {
    /* C macro invocation: RUN_TESTS(cgroup_read_xattr); */
    RUN_TESTS_cgroup_read_xattr();

    if test__start_subtest(c"read_cgroupfs_xattr".as_ptr()) {
        test_read_cgroup_xattr();
    }
}
