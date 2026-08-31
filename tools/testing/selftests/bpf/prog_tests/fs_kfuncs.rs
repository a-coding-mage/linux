// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/* Dependencies from:
 * <stdlib.h>, <sys/types.h>, <sys/xattr.h>, <linux/fsverity.h>,
 * <unistd.h>, <test_progs.h>, and generated skeleton headers.
 */

static testfile: &[u8] = b"/tmp/test_progs_fs_kfuncs\0";

const O_CREAT: c_int = 0o100;
const O_RDONLY: c_int = 0;
const O_RDWR: c_int = 0o2;
const EOPNOTSUPP: c_int = 95;
const EINVAL: c_int = 22;
const FS_VERITY_HASH_ALG_SHA256: u32 = 1;
const FS_IOC_ENABLE_VERITY: c_ulong = 0x40806685;
const FS_IOC_MEASURE_VERITY: c_ulong = 0xc0046686;

#[cfg(not(any()))]
const SHA256_DIGEST_SIZE_FROM_HEADER: usize = 32;
const SHA256_DIGEST_SIZE: usize = 32;

type c_ulong = u64;
type bool_ = bool;

#[repr(C)]
struct fsverity_enable_arg {
    version: u32,
    hash_algorithm: u32,
    block_size: u32,
    salt_size: u32,
    salt_ptr: u64,
    sig_size: u32,
    __reserved1: u32,
    sig_ptr: u64,
    __reserved2: [u64; 11],
}

#[repr(C)]
struct fsverity_digest {
    digest_algorithm: u16,
    digest_size: u16,
    digest: [u8; 0],
}

#[repr(C)]
struct test_get_xattr {
    bss: *mut test_get_xattr_bss,
}

#[repr(C)]
struct test_get_xattr_bss {
    monitored_pid: c_int,
    found_xattr_from_file: c_int,
    found_xattr_from_dentry: c_int,
}

#[repr(C)]
struct test_set_remove_xattr {
    bss: *mut test_set_remove_xattr_bss,
    rodata: *mut test_set_remove_xattr_rodata,
    data: *mut test_set_remove_xattr_data,
}

#[repr(C)]
struct test_set_remove_xattr_bss {
    monitored_pid: c_int,
    set_security_bpf_bar_success: bool_,
    remove_security_bpf_bar_success: bool_,
    set_security_selinux_fail: bool_,
    remove_security_selinux_fail: bool_,
    locked_set_security_bpf_bar_success: bool_,
    locked_remove_security_bpf_bar_success: bool_,
    locked_set_security_selinux_fail: bool_,
    locked_remove_security_selinux_fail: bool_,
}

#[repr(C)]
struct test_set_remove_xattr_rodata {
    xattr_foo: *const c_char,
    xattr_bar: *const c_char,
}

#[repr(C)]
struct test_set_remove_xattr_data {
    value_bar: [c_char; 6],
}

#[repr(C)]
struct test_fsverity {
    bss: *mut test_fsverity_bss,
}

#[repr(C)]
struct test_fsverity_bss {
    monitored_pid: c_int,
    expected_digest: [u8; 4096],
    got_fsverity: c_int,
    digest_matches: c_int,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn remove(pathname: *const c_char) -> c_int;
    fn setxattr(
        path: *const c_char,
        name: *const c_char,
        value: *const c_void,
        size: usize,
        flags: c_int,
    ) -> c_int;
    fn getxattr(path: *const c_char, name: *const c_char, value: *mut c_void, size: usize) -> isize;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn getpid() -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;

    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool_;

    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool_;
    fn ASSERT_LT(actual: isize, expected: isize, name: *const c_char) -> bool_;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool_;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool_;
    fn ASSERT_EQ(actual: isize, expected: isize, name: *const c_char) -> bool_;
    fn ASSERT_TRUE(actual: bool_, name: *const c_char) -> bool_;

    fn test_get_xattr__open_and_load() -> *mut test_get_xattr;
    fn test_get_xattr__attach(skel: *mut test_get_xattr) -> c_int;
    fn test_get_xattr__destroy(skel: *mut test_get_xattr);

    fn test_set_remove_xattr__open_and_load() -> *mut test_set_remove_xattr;
    fn test_set_remove_xattr__attach(skel: *mut test_set_remove_xattr) -> c_int;
    fn test_set_remove_xattr__destroy(skel: *mut test_set_remove_xattr);

    fn test_fsverity__open_and_load() -> *mut test_fsverity;
    fn test_fsverity__attach(skel: *mut test_fsverity) -> c_int;
    fn test_fsverity__destroy(skel: *mut test_fsverity);
}

unsafe fn test_get_xattr(name: *const c_char, value: *const c_char, allow_access: bool_) {
    let mut skel: *mut test_get_xattr = ptr::null_mut();
    let mut fd: c_int = -1;
    let mut err: c_int;
    let mut v: [c_int; 32] = [0; 32];

    fd = open(testfile.as_ptr() as *const c_char, O_CREAT | O_RDONLY, 0o644);
    if !ASSERT_GE(fd, 0, c"create_file".as_ptr()) {
        return;
    }

    close(fd);
    fd = -1;

    err = setxattr(
        testfile.as_ptr() as *const c_char,
        name,
        value as *const c_void,
        strlen(value) + 1,
        0,
    );
    if err != 0 && errno == EOPNOTSUPP {
        printf(
            c"%s:SKIP:local fs doesn't support xattr (%d)\nTo run this test, make sure /tmp filesystem supports xattr.\n".as_ptr(),
            c"test_get_xattr".as_ptr(),
            errno,
        );
        test__skip();
        goto_out_get_xattr(&mut fd, skel);
        return;
    }

    if !ASSERT_OK(err, c"setxattr".as_ptr()) {
        goto_out_get_xattr(&mut fd, skel);
        return;
    }

    skel = test_get_xattr__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_get_xattr__open_and_load".as_ptr()) {
        goto_out_get_xattr(&mut fd, skel);
        return;
    }

    (*(*skel).bss).monitored_pid = getpid();
    err = test_get_xattr__attach(skel);

    if !ASSERT_OK(err, c"test_get_xattr__attach".as_ptr()) {
        goto_out_get_xattr(&mut fd, skel);
        return;
    }

    fd = open(testfile.as_ptr() as *const c_char, O_RDONLY, 0o644);

    if !ASSERT_GE(fd, 0, c"open_file".as_ptr()) {
        goto_out_get_xattr(&mut fd, skel);
        return;
    }

    /* Trigger security_inode_getxattr */
    err = getxattr(
        testfile.as_ptr() as *const c_char,
        name,
        v.as_mut_ptr() as *mut c_void,
        size_of_val(&v),
    ) as c_int;

    if allow_access {
        ASSERT_EQ(err as isize, -1, c"getxattr_return".as_ptr());
        ASSERT_EQ(errno as isize, EINVAL as isize, c"getxattr_errno".as_ptr());
        ASSERT_EQ(
            (*(*skel).bss).found_xattr_from_file as isize,
            1,
            c"found_xattr_from_file".as_ptr(),
        );
        ASSERT_EQ(
            (*(*skel).bss).found_xattr_from_dentry as isize,
            1,
            c"found_xattr_from_dentry".as_ptr(),
        );
    } else {
        ASSERT_EQ(
            err as isize,
            (strlen(value) + 1) as isize,
            c"getxattr_return".as_ptr(),
        );
        ASSERT_EQ(
            (*(*skel).bss).found_xattr_from_file as isize,
            0,
            c"found_xattr_from_file".as_ptr(),
        );
        ASSERT_EQ(
            (*(*skel).bss).found_xattr_from_dentry as isize,
            0,
            c"found_xattr_from_dentry".as_ptr(),
        );
    }

    goto_out_get_xattr(&mut fd, skel);
}

unsafe fn goto_out_get_xattr(fd: &mut c_int, skel: *mut test_get_xattr) {
    close(*fd);
    test_get_xattr__destroy(skel);
    remove(testfile.as_ptr() as *const c_char);
}

/* xattr value we will set to security.bpf.foo */
static value_foo: &[u8] = b"hello\0";

unsafe fn read_and_validate_foo(skel: *mut test_set_remove_xattr) {
    let mut value_out: [c_char; 32] = [0; 32];
    let mut err: isize;

    err = getxattr(
        testfile.as_ptr() as *const c_char,
        (*(*skel).rodata).xattr_foo,
        value_out.as_mut_ptr() as *mut c_void,
        size_of_val(&value_out),
    );
    ASSERT_EQ(err, size_of_val(value_foo) as isize, c"getxattr size foo".as_ptr());
    ASSERT_EQ(
        strncmp(
            value_out.as_ptr(),
            value_foo.as_ptr() as *const c_char,
            size_of_val(value_foo),
        ) as isize,
        0,
        c"strncmp value_foo".as_ptr(),
    );
}

unsafe fn set_foo(skel: *mut test_set_remove_xattr) {
    ASSERT_OK(
        setxattr(
            testfile.as_ptr() as *const c_char,
            (*(*skel).rodata).xattr_foo,
            value_foo.as_ptr() as *const c_void,
            strlen(value_foo.as_ptr() as *const c_char) + 1,
            0,
        ),
        c"setxattr foo".as_ptr(),
    );
}

unsafe fn validate_bar_match(skel: *mut test_set_remove_xattr) {
    let mut value_out: [c_char; 32] = [0; 32];
    let mut err: isize;

    err = getxattr(
        testfile.as_ptr() as *const c_char,
        (*(*skel).rodata).xattr_bar,
        value_out.as_mut_ptr() as *mut c_void,
        size_of_val(&value_out),
    );
    ASSERT_EQ(
        err,
        size_of::<[c_char; 6]>() as isize,
        c"getxattr size bar".as_ptr(),
    );
    ASSERT_EQ(
        strncmp(
            value_out.as_ptr(),
            (*(*skel).data).value_bar.as_ptr(),
            size_of::<[c_char; 6]>(),
        ) as isize,
        0,
        c"strncmp value_bar".as_ptr(),
    );
}

unsafe fn validate_bar_removed(skel: *mut test_set_remove_xattr) {
    let mut value_out: [c_char; 32] = [0; 32];
    let mut err: isize;

    err = getxattr(
        testfile.as_ptr() as *const c_char,
        (*(*skel).rodata).xattr_bar,
        value_out.as_mut_ptr() as *mut c_void,
        size_of_val(&value_out),
    );
    ASSERT_LT(err, 0, c"getxattr size bar should fail".as_ptr());
}

unsafe fn test_set_remove_xattr() {
    let mut skel: *mut test_set_remove_xattr = ptr::null_mut();
    let mut fd: c_int = -1;
    let mut err: c_int;

    fd = open(testfile.as_ptr() as *const c_char, O_CREAT | O_RDONLY, 0o644);
    if !ASSERT_GE(fd, 0, c"create_file".as_ptr()) {
        return;
    }

    close(fd);
    fd = -1;

    skel = test_set_remove_xattr__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        c"test_set_remove_xattr__open_and_load".as_ptr(),
    ) {
        return;
    }

    /* Set security.bpf.foo to "hello" */
    err = setxattr(
        testfile.as_ptr() as *const c_char,
        (*(*skel).rodata).xattr_foo,
        value_foo.as_ptr() as *const c_void,
        strlen(value_foo.as_ptr() as *const c_char) + 1,
        0,
    );
    if err != 0 && errno == EOPNOTSUPP {
        printf(
            c"%s:SKIP:local fs doesn't support xattr (%d)\nTo run this test, make sure /tmp filesystem supports xattr.\n".as_ptr(),
            c"test_set_remove_xattr".as_ptr(),
            errno,
        );
        test__skip();
        goto_out_set_remove_xattr(&mut fd, skel);
        return;
    }

    if !ASSERT_OK(err, c"setxattr".as_ptr()) {
        goto_out_set_remove_xattr(&mut fd, skel);
        return;
    }

    (*(*skel).bss).monitored_pid = getpid();
    err = test_set_remove_xattr__attach(skel);
    if !ASSERT_OK(err, c"test_set_remove_xattr__attach".as_ptr()) {
        goto_out_set_remove_xattr(&mut fd, skel);
        return;
    }

    /* First, test not _locked version of the kfuncs with getxattr. */

    /* Read security.bpf.foo and trigger test_inode_getxattr. This
     * bpf program will set security.bpf.bar to "world".
     */
    read_and_validate_foo(skel);
    validate_bar_match(skel);

    /* Read security.bpf.foo and trigger test_inode_getxattr again.
     * This will remove xattr security.bpf.bar.
     */
    read_and_validate_foo(skel);
    validate_bar_removed(skel);

    ASSERT_TRUE(
        (*(*skel).bss).set_security_bpf_bar_success,
        c"set_security_bpf_bar_success".as_ptr(),
    );
    ASSERT_TRUE(
        (*(*skel).bss).remove_security_bpf_bar_success,
        c"remove_security_bpf_bar_success".as_ptr(),
    );
    ASSERT_TRUE(
        (*(*skel).bss).set_security_selinux_fail,
        c"set_security_selinux_fail".as_ptr(),
    );
    ASSERT_TRUE(
        (*(*skel).bss).remove_security_selinux_fail,
        c"remove_security_selinux_fail".as_ptr(),
    );

    /* Second, test _locked version of the kfuncs, with setxattr */

    /* Set security.bpf.foo and trigger test_inode_setxattr. This
     * bpf program will set security.bpf.bar to "world".
     */
    set_foo(skel);
    validate_bar_match(skel);

    /* Set security.bpf.foo and trigger test_inode_setxattr again.
     * This will remove xattr security.bpf.bar.
     */
    set_foo(skel);
    validate_bar_removed(skel);

    ASSERT_TRUE(
        (*(*skel).bss).locked_set_security_bpf_bar_success,
        c"locked_set_security_bpf_bar_success".as_ptr(),
    );
    ASSERT_TRUE(
        (*(*skel).bss).locked_remove_security_bpf_bar_success,
        c"locked_remove_security_bpf_bar_success".as_ptr(),
    );
    ASSERT_TRUE(
        (*(*skel).bss).locked_set_security_selinux_fail,
        c"locked_set_security_selinux_fail".as_ptr(),
    );
    ASSERT_TRUE(
        (*(*skel).bss).locked_remove_security_selinux_fail,
        c"locked_remove_security_selinux_fail".as_ptr(),
    );

    goto_out_set_remove_xattr(&mut fd, skel);
}

unsafe fn goto_out_set_remove_xattr(fd: &mut c_int, skel: *mut test_set_remove_xattr) {
    close(*fd);
    test_set_remove_xattr__destroy(skel);
    remove(testfile.as_ptr() as *const c_char);
}

unsafe fn test_fsverity() {
    let mut arg: fsverity_enable_arg = core::mem::zeroed();
    let mut skel: *mut test_fsverity = ptr::null_mut();
    let mut d: *mut fsverity_digest;
    let mut fd: c_int;
    let mut err: c_int;
    let mut buffer: [c_char; 4096] = [0; 4096];

    fd = open(testfile.as_ptr() as *const c_char, O_CREAT | O_RDWR, 0o644);
    if !ASSERT_GE(fd, 0, c"create_file".as_ptr()) {
        return;
    }

    /* Write random buffer, so the file is not empty */
    err = write(fd, buffer.as_ptr() as *const c_void, 4096) as c_int;
    if !ASSERT_EQ(err as isize, 4096, c"write_file".as_ptr()) {
        goto_out_fsverity(fd, skel);
        return;
    }
    close(fd);

    /* Reopen read-only, otherwise FS_IOC_ENABLE_VERITY will fail */
    fd = open(testfile.as_ptr() as *const c_char, O_RDONLY, 0o644);
    if !ASSERT_GE(fd, 0, c"open_file1".as_ptr()) {
        return;
    }

    /* Enable fsverity for the file.
     * If the file system doesn't support verity, this will fail. Skip
     * the test in such case.
     */
    arg.version = 1;
    arg.hash_algorithm = FS_VERITY_HASH_ALG_SHA256;
    arg.block_size = 4096;
    err = ioctl(fd, FS_IOC_ENABLE_VERITY, &mut arg as *mut fsverity_enable_arg);
    if err != 0 {
        printf(
            c"%s:SKIP:local fs doesn't support fsverity (%d)\nTo run this test, try enable CONFIG_FS_VERITY and enable FSVerity for the filesystem.\n".as_ptr(),
            c"test_fsverity".as_ptr(),
            errno,
        );
        test__skip();
        goto_out_fsverity(fd, skel);
        return;
    }

    skel = test_fsverity__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_fsverity__open_and_load".as_ptr()) {
        goto_out_fsverity(fd, skel);
        return;
    }

    /* Get fsverity_digest from ioctl */
    d = (*(*skel).bss).expected_digest.as_mut_ptr() as *mut fsverity_digest;
    (*d).digest_algorithm = FS_VERITY_HASH_ALG_SHA256 as u16;
    (*d).digest_size = SHA256_DIGEST_SIZE as u16;
    err = ioctl(
        fd,
        FS_IOC_MEASURE_VERITY,
        (*(*skel).bss).expected_digest.as_mut_ptr(),
    );
    if !ASSERT_OK(err, c"ioctl_FS_IOC_MEASURE_VERITY".as_ptr()) {
        goto_out_fsverity(fd, skel);
        return;
    }

    (*(*skel).bss).monitored_pid = getpid();
    err = test_fsverity__attach(skel);
    if !ASSERT_OK(err, c"test_fsverity__attach".as_ptr()) {
        goto_out_fsverity(fd, skel);
        return;
    }

    /* Reopen the file to trigger the program */
    close(fd);
    fd = open(testfile.as_ptr() as *const c_char, O_RDONLY);
    if !ASSERT_GE(fd, 0, c"open_file2".as_ptr()) {
        goto_out_fsverity(fd, skel);
        return;
    }

    ASSERT_EQ((*(*skel).bss).got_fsverity as isize, 1, c"got_fsverity".as_ptr());
    ASSERT_EQ(
        (*(*skel).bss).digest_matches as isize,
        1,
        c"digest_matches".as_ptr(),
    );

    goto_out_fsverity(fd, skel);
}

unsafe fn goto_out_fsverity(fd: c_int, skel: *mut test_fsverity) {
    close(fd);
    test_fsverity__destroy(skel);
    remove(testfile.as_ptr() as *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn test_fs_kfuncs() {
    /* Matches xattr_names in progs/test_get_xattr.c */
    if test__start_subtest(c"user_xattr".as_ptr()) {
        test_get_xattr(c"user.kfuncs".as_ptr(), c"hello".as_ptr(), true);
    }

    if test__start_subtest(c"security_bpf_xattr".as_ptr()) {
        test_get_xattr(c"security.bpf.xxx".as_ptr(), c"hello".as_ptr(), true);
    }

    if test__start_subtest(c"security_bpf_xattr_error".as_ptr()) {
        test_get_xattr(c"security.bpf".as_ptr(), c"hello".as_ptr(), false);
    }

    if test__start_subtest(c"security_selinux_xattr_error".as_ptr()) {
        test_get_xattr(c"security.selinux".as_ptr(), c"hello".as_ptr(), false);
    }

    if test__start_subtest(c"set_remove_xattr".as_ptr()) {
        test_set_remove_xattr();
    }

    if test__start_subtest(c"fsverity".as_ptr()) {
        test_fsverity();
    }
}
