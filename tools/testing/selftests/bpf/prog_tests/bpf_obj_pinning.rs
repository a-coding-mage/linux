// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */
/* Translated from testing/selftests/bpf/prog_tests/bpf_obj_pinning.c. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const PATH_MAX: usize = 4096;
const BPF_MAP_TYPE_ARRAY: c_int = 2;
const BPF_F_PATH_FD: c_uint = 1 << 14;
const FSCONFIG_CMD_CREATE: c_uint = 6;
const O_PATH: c_int = 0o10000000;
const __NR_fsopen: c_long = 430;
const __NR_fsconfig: c_long = 431;
const __NR_fsmount: c_long = 432;
const __NR_move_mount: c_long = 429;

type c_long = i64;

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_obj_pin_opts {
    sz: usize,
    file_flags: c_uint,
    path_fd: c_int,
}

impl Default for bpf_obj_pin_opts {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
            file_flags: 0,
            path_fd: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_obj_get_opts {
    sz: usize,
    file_flags: c_uint,
    path_fd: c_int,
}

impl Default for bpf_obj_get_opts {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
            file_flags: 0,
            path_fd: 0,
        }
    }
}

extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn close(fd: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn chdir(path: *const c_char) -> c_int;
    fn getcwd(buf: *mut c_char, size: usize) -> *mut c_char;
    fn unlink(pathname: *const c_char) -> c_int;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;

    fn bpf_map_create(
        map_type: c_int,
        map_name: *const c_char,
        key_size: c_uint,
        value_size: c_uint,
        max_entries: c_uint,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_obj_pin_opts(
        bpf_fd: c_int,
        pathname: *const c_char,
        opts: *const bpf_obj_pin_opts,
    ) -> c_int;
    fn bpf_obj_get_opts(pathname: *const c_char, opts: *const bpf_obj_get_opts) -> c_int;
    fn bpf_obj_pin(bpf_fd: c_int, pathname: *const c_char) -> c_int;
    fn bpf_obj_get(pathname: *const c_char) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: c_ulong,
    ) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;

    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_char, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
}

#[inline]
unsafe fn sys_fsopen(fsname: *const c_char, flags: c_uint) -> c_int {
    syscall(__NR_fsopen, fsname, flags) as c_int
}

#[inline]
unsafe fn sys_fsconfig(
    fs_fd: c_int,
    cmd: c_uint,
    key: *const c_char,
    val: *const c_void,
    aux: c_int,
) -> c_int {
    syscall(__NR_fsconfig, fs_fd, cmd, key, val, aux) as c_int
}

#[inline]
unsafe fn sys_fsmount(fs_fd: c_int, flags: c_uint, ms_flags: c_uint) -> c_int {
    syscall(__NR_fsmount, fs_fd, flags, ms_flags) as c_int
}

#[allow(dead_code)]
#[inline]
unsafe fn sys_move_mount(
    from_dfd: c_int,
    from_path: *const c_char,
    to_dfd: c_int,
    to_path: *const c_char,
    ms_flags: c_uint,
) -> c_int {
    syscall(
        __NR_move_mount,
        from_dfd,
        from_path,
        to_dfd,
        to_path,
        ms_flags,
    ) as c_int
}

unsafe fn bpf_obj_pinning_detached() {
    let mut pin_opts = bpf_obj_pin_opts::default();
    let mut get_opts = bpf_obj_get_opts::default();
    let mut fs_fd: c_int = -1;
    let mut mnt_fd: c_int = -1;
    let mut map_fd: c_int = -1;
    let mut map_fd2: c_int = -1;
    let mut zero: c_int = 0;
    let mut src_value: c_int;
    let mut dst_value: c_int;
    let mut err: c_int;
    let map_name = b"fsmount_map\0".as_ptr() as *const c_char;

    /* A bunch of below UAPI calls are constructed based on reading:
     * https://brauner.io/2023/02/28/mounting-into-mount-namespaces.html
     */

    /* create VFS context */
    fs_fd = sys_fsopen(b"bpf\0".as_ptr() as *const c_char, 0);
    if !ASSERT_GE(fs_fd, 0, b"fs_fd\0".as_ptr() as *const c_char) {
        goto_cleanup_detached(fs_fd, mnt_fd, map_fd, map_fd2);
        return;
    }

    /* instantiate FS object */
    err = sys_fsconfig(
        fs_fd,
        FSCONFIG_CMD_CREATE,
        core::ptr::null(),
        core::ptr::null(),
        0,
    );
    if !ASSERT_OK(err, b"fs_create\0".as_ptr() as *const c_char) {
        goto_cleanup_detached(fs_fd, mnt_fd, map_fd, map_fd2);
        return;
    }

    /* create O_PATH fd for detached mount */
    mnt_fd = sys_fsmount(fs_fd, 0, 0);
    if !ASSERT_GE(mnt_fd, 0, b"mnt_fd\0".as_ptr() as *const c_char) {
        goto_cleanup_detached(fs_fd, mnt_fd, map_fd, map_fd2);
        return;
    }

    /* If we wanted to expose detached mount in the file system, we'd do
     * something like below. But the whole point is that we actually don't
     * even have to expose BPF FS in the file system to be able to work
     * (pin/get objects) with it.
     *
     * err = sys_move_mount(mnt_fd, "", -EBADF, mnt_path, MOVE_MOUNT_F_EMPTY_PATH);
     * if (!ASSERT_OK(err, "move_mount"))
     *	goto cleanup;
     */

    /* create BPF map to pin */
    map_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, map_name, 4, 4, 1, core::ptr::null());
    if !ASSERT_GE(map_fd, 0, b"map_fd\0".as_ptr() as *const c_char) {
        goto_cleanup_detached(fs_fd, mnt_fd, map_fd, map_fd2);
        return;
    }

    /* pin BPF map into detached BPF FS through mnt_fd */
    pin_opts.file_flags = BPF_F_PATH_FD;
    pin_opts.path_fd = mnt_fd;
    err = bpf_obj_pin_opts(map_fd, map_name, &pin_opts);
    if !ASSERT_OK(err, b"map_pin\0".as_ptr() as *const c_char) {
        goto_cleanup_detached(fs_fd, mnt_fd, map_fd, map_fd2);
        return;
    }

    /* get BPF map from detached BPF FS through mnt_fd */
    get_opts.file_flags = BPF_F_PATH_FD;
    get_opts.path_fd = mnt_fd;
    map_fd2 = bpf_obj_get_opts(map_name, &get_opts);
    if !ASSERT_GE(map_fd2, 0, b"map_get\0".as_ptr() as *const c_char) {
        goto_cleanup_detached(fs_fd, mnt_fd, map_fd, map_fd2);
        return;
    }

    /* update map through one FD */
    src_value = 0xcafebeefu32 as c_int;
    err = bpf_map_update_elem(
        map_fd,
        &zero as *const _ as *const c_void,
        &src_value as *const _ as *const c_void,
        0,
    );
    ASSERT_OK(err, b"map_update\0".as_ptr() as *const c_char);

    /* check values written/read through different FDs do match */
    dst_value = 0;
    err = bpf_map_lookup_elem(
        map_fd2,
        &zero as *const _ as *const c_void,
        &mut dst_value as *mut _ as *mut c_void,
    );
    ASSERT_OK(err, b"map_lookup\0".as_ptr() as *const c_char);
    ASSERT_EQ(
        dst_value,
        src_value,
        b"map_value_eq1\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ(
        dst_value,
        0xcafebeefu32 as c_int,
        b"map_value_eq2\0".as_ptr() as *const c_char,
    );

    goto_cleanup_detached(fs_fd, mnt_fd, map_fd, map_fd2);
}

unsafe fn goto_cleanup_detached(fs_fd: c_int, mnt_fd: c_int, map_fd: c_int, map_fd2: c_int) {
    if map_fd >= 0 {
        ASSERT_OK(close(map_fd), b"close_map_fd\0".as_ptr() as *const c_char);
    }
    if map_fd2 >= 0 {
        ASSERT_OK(close(map_fd2), b"close_map_fd2\0".as_ptr() as *const c_char);
    }
    if fs_fd >= 0 {
        ASSERT_OK(close(fs_fd), b"close_fs_fd\0".as_ptr() as *const c_char);
    }
    if mnt_fd >= 0 {
        ASSERT_OK(close(mnt_fd), b"close_mnt_fd\0".as_ptr() as *const c_char);
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum path_kind {
    PATH_STR_ABS,
    PATH_STR_REL,
    PATH_FD_REL,
}

unsafe fn validate_pin(
    map_fd: c_int,
    map_name: *const c_char,
    src_value: c_int,
    path_kind: path_kind,
) {
    let mut pin_opts = bpf_obj_pin_opts::default();
    let mut abs_path = [0 as c_char; PATH_MAX];
    let mut old_cwd = [0 as c_char; PATH_MAX];
    let mut pin_path: *const c_char = core::ptr::null();
    let mut zero: c_int = 0;
    let mut dst_value: c_int;
    let mut map_fd2: c_int;
    let mut err: c_int;

    snprintf(
        abs_path.as_mut_ptr(),
        abs_path.len(),
        b"/sys/fs/bpf/%s\0".as_ptr() as *const c_char,
        map_name,
    );
    old_cwd[0] = 0;

    match path_kind {
        path_kind::PATH_STR_ABS => {
            /* absolute path */
            pin_path = abs_path.as_ptr();
        }
        path_kind::PATH_STR_REL => {
            /* cwd + relative path */
            ASSERT_OK_PTR(
                getcwd(old_cwd.as_mut_ptr(), old_cwd.len()),
                b"getcwd\0".as_ptr() as *const c_char,
            );
            ASSERT_OK(
                chdir(b"/sys/fs/bpf\0".as_ptr() as *const c_char),
                b"chdir\0".as_ptr() as *const c_char,
            );
            pin_path = map_name;
        }
        path_kind::PATH_FD_REL => {
            /* dir fd + relative path */
            pin_opts.file_flags = BPF_F_PATH_FD;
            pin_opts.path_fd = open(b"/sys/fs/bpf\0".as_ptr() as *const c_char, O_PATH);
            ASSERT_GE(
                pin_opts.path_fd,
                0,
                b"path_fd\0".as_ptr() as *const c_char,
            );
            pin_path = map_name;
        }
    }

    /* pin BPF map using specified path definition */
    err = bpf_obj_pin_opts(map_fd, pin_path, &pin_opts);
    ASSERT_OK(err, b"obj_pin\0".as_ptr() as *const c_char);

    /* cleanup */
    if path_kind == path_kind::PATH_FD_REL && pin_opts.path_fd >= 0 {
        close(pin_opts.path_fd);
    }
    if old_cwd[0] != 0 {
        ASSERT_OK(
            chdir(old_cwd.as_ptr()),
            b"restore_cwd\0".as_ptr() as *const c_char,
        );
    }

    map_fd2 = bpf_obj_get(abs_path.as_ptr());
    if !ASSERT_GE(map_fd2, 0, b"map_get\0".as_ptr() as *const c_char) {
        goto_cleanup_validate(abs_path.as_ptr(), map_fd2);
        return;
    }

    /* update map through one FD */
    err = bpf_map_update_elem(
        map_fd,
        &zero as *const _ as *const c_void,
        &src_value as *const _ as *const c_void,
        0,
    );
    ASSERT_OK(err, b"map_update\0".as_ptr() as *const c_char);

    /* check values written/read through different FDs do match */
    dst_value = 0;
    err = bpf_map_lookup_elem(
        map_fd2,
        &zero as *const _ as *const c_void,
        &mut dst_value as *mut _ as *mut c_void,
    );
    ASSERT_OK(err, b"map_lookup\0".as_ptr() as *const c_char);
    ASSERT_EQ(
        dst_value,
        src_value,
        b"map_value_eq\0".as_ptr() as *const c_char,
    );
    goto_cleanup_validate(abs_path.as_ptr(), map_fd2);
}

unsafe fn validate_get(
    map_fd: c_int,
    map_name: *const c_char,
    src_value: c_int,
    path_kind: path_kind,
) {
    let mut get_opts = bpf_obj_get_opts::default();
    let mut abs_path = [0 as c_char; PATH_MAX];
    let mut old_cwd = [0 as c_char; PATH_MAX];
    let mut pin_path: *const c_char = core::ptr::null();
    let mut zero: c_int = 0;
    let mut dst_value: c_int;
    let mut map_fd2: c_int;
    let mut err: c_int;

    snprintf(
        abs_path.as_mut_ptr(),
        abs_path.len(),
        b"/sys/fs/bpf/%s\0".as_ptr() as *const c_char,
        map_name,
    );
    /* pin BPF map using specified path definition */
    err = bpf_obj_pin(map_fd, abs_path.as_ptr());
    if !ASSERT_OK(err, b"pin_map\0".as_ptr() as *const c_char) {
        return;
    }

    old_cwd[0] = 0;

    match path_kind {
        path_kind::PATH_STR_ABS => {
            /* absolute path */
            pin_path = abs_path.as_ptr();
        }
        path_kind::PATH_STR_REL => {
            /* cwd + relative path */
            ASSERT_OK_PTR(
                getcwd(old_cwd.as_mut_ptr(), old_cwd.len()),
                b"getcwd\0".as_ptr() as *const c_char,
            );
            ASSERT_OK(
                chdir(b"/sys/fs/bpf\0".as_ptr() as *const c_char),
                b"chdir\0".as_ptr() as *const c_char,
            );
            pin_path = map_name;
        }
        path_kind::PATH_FD_REL => {
            /* dir fd + relative path */
            get_opts.file_flags = BPF_F_PATH_FD;
            get_opts.path_fd = open(b"/sys/fs/bpf\0".as_ptr() as *const c_char, O_PATH);
            ASSERT_GE(
                get_opts.path_fd,
                0,
                b"path_fd\0".as_ptr() as *const c_char,
            );
            pin_path = map_name;
        }
    }

    map_fd2 = bpf_obj_get_opts(pin_path, &get_opts);
    if !ASSERT_GE(map_fd2, 0, b"map_get\0".as_ptr() as *const c_char) {
        goto_cleanup_validate(abs_path.as_ptr(), map_fd2);
        return;
    }

    /* cleanup */
    if path_kind == path_kind::PATH_FD_REL && get_opts.path_fd >= 0 {
        close(get_opts.path_fd);
    }
    if old_cwd[0] != 0 {
        ASSERT_OK(
            chdir(old_cwd.as_ptr()),
            b"restore_cwd\0".as_ptr() as *const c_char,
        );
    }

    /* update map through one FD */
    err = bpf_map_update_elem(
        map_fd,
        &zero as *const _ as *const c_void,
        &src_value as *const _ as *const c_void,
        0,
    );
    ASSERT_OK(err, b"map_update\0".as_ptr() as *const c_char);

    /* check values written/read through different FDs do match */
    dst_value = 0;
    err = bpf_map_lookup_elem(
        map_fd2,
        &zero as *const _ as *const c_void,
        &mut dst_value as *mut _ as *mut c_void,
    );
    ASSERT_OK(err, b"map_lookup\0".as_ptr() as *const c_char);
    ASSERT_EQ(
        dst_value,
        src_value,
        b"map_value_eq\0".as_ptr() as *const c_char,
    );
    goto_cleanup_validate(abs_path.as_ptr(), map_fd2);
}

unsafe fn goto_cleanup_validate(abs_path: *const c_char, map_fd2: c_int) {
    if map_fd2 >= 0 {
        ASSERT_OK(close(map_fd2), b"close_map_fd2\0".as_ptr() as *const c_char);
    }
    unlink(abs_path);
}

unsafe fn bpf_obj_pinning_mounted(path_kind: path_kind) {
    let map_name = b"mounted_map\0".as_ptr() as *const c_char;
    let map_fd: c_int;

    /* create BPF map to pin */
    map_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, map_name, 4, 4, 1, core::ptr::null());
    if !ASSERT_GE(map_fd, 0, b"map_fd\0".as_ptr() as *const c_char) {
        return;
    }

    validate_pin(map_fd, map_name, 100 + path_kind as c_int, path_kind);
    validate_get(map_fd, map_name, 200 + path_kind as c_int, path_kind);
    ASSERT_OK(close(map_fd), b"close_map_fd\0".as_ptr() as *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn test_bpf_obj_pinning() {
    if test__start_subtest(b"detached\0".as_ptr() as *const c_char) {
        bpf_obj_pinning_detached();
    }
    if test__start_subtest(b"mounted-str-abs\0".as_ptr() as *const c_char) {
        bpf_obj_pinning_mounted(path_kind::PATH_STR_ABS);
    }
    if test__start_subtest(b"mounted-str-rel\0".as_ptr() as *const c_char) {
        bpf_obj_pinning_mounted(path_kind::PATH_STR_REL);
    }
    if test__start_subtest(b"mounted-fd-rel\0".as_ptr() as *const c_char) {
        bpf_obj_pinning_mounted(path_kind::PATH_FD_REL);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
