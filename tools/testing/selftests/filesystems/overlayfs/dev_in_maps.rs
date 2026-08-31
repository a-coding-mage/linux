// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/filesystems/overlayfs/dev_in_maps.c
// Original C dependencies: kselftest.h, log.h, ../wrappers.h, Linux mount/stat APIs.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const FSCONFIG_SET_STRING: c_uint = 1;
const FSCONFIG_CMD_CREATE: c_uint = 6;
const MOVE_MOUNT_F_EMPTY_PATH: c_uint = 0x00000004;

unsafe extern "C" {
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut libc::FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut libc::FILE) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mkdir(pathname: *const c_char, mode: libc::mode_t) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, mode: libc::mode_t) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: libc::size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: libc::off_t,
    ) -> *mut c_void;
    fn statx(
        dirfd: c_int,
        pathname: *const c_char,
        flags: c_int,
        mask: c_uint,
        statxbuf: *mut libc::statx,
    ) -> c_int;
    fn unshare(flags: c_int) -> c_int;

    fn sys_fsopen(fs_name: *const c_char, flags: c_uint) -> c_int;
    fn sys_fsconfig(
        fd: c_int,
        cmd: c_uint,
        key: *const c_char,
        value: *const c_void,
        aux: c_int,
    ) -> c_int;
    fn sys_fsmount(fsfd: c_int, flags: c_uint, attr_flags: c_uint) -> c_int;
    fn sys_move_mount(
        from_dfd: c_int,
        from_pathname: *const c_char,
        to_dfd: c_int,
        to_pathname: *const c_char,
        flags: c_uint,
    ) -> c_int;
    fn sys_mount(
        src: *const c_char,
        tgt: *const c_char,
        fstype: *const c_char,
        flags: c_ulong,
        data: *const c_void,
    ) -> c_int;

    fn pr_perror(format: *const c_char, ...) -> c_long;
    fn pr_err(format: *const c_char, ...) -> c_long;
    fn pr_fail(format: *const c_char, ...) -> c_int;

    fn ksft_test_result_pass(format: *const c_char, ...);
    fn ksft_test_result_skip(format: *const c_char, ...);
    fn ksft_set_plan(cnt: c_uint);
    fn ksft_exit_pass() -> !;
}

unsafe fn get_file_dev_and_inode(addr: *mut c_void, stx: *mut libc::statx) -> c_long {
    let mut buf = [0 as c_char; 4096];
    let mapf: *mut libc::FILE;

    mapf = fopen(c"/proc/self/maps".as_ptr(), c"r".as_ptr());
    if mapf.is_null() {
        return pr_perror(c"fopen(/proc/self/maps)".as_ptr());
    }

    while !fgets(buf.as_mut_ptr(), buf.len() as c_int, mapf).is_null() {
        let mut start: c_ulong = 0;
        let mut end: c_ulong = 0;
        let mut maj: u32 = 0;
        let mut min: u32 = 0;
        let mut ino: u64 = 0;

        if sscanf(
            buf.as_ptr(),
            c"%lx-%lx %*s %*s %x:%x %llu".as_ptr(),
            &mut start,
            &mut end,
            &mut maj,
            &mut min,
            &mut ino,
        ) != 5
        {
            return pr_perror(c"unable to parse: %s".as_ptr(), buf.as_ptr());
        }
        if start == addr as c_ulong {
            (*stx).stx_dev_major = maj;
            (*stx).stx_dev_minor = min;
            (*stx).stx_ino = ino;
            return 0;
        }
    }

    pr_err(c"unable to find the mapping".as_ptr())
}

unsafe fn ovl_mount() -> c_int {
    let tmpfs: c_int;
    let mut fsfd: c_int;
    let ovl: c_int;

    fsfd = sys_fsopen(c"tmpfs".as_ptr(), 0);
    if fsfd == -1 {
        return pr_perror(c"fsopen(tmpfs)".as_ptr()) as c_int;
    }

    if sys_fsconfig(
        fsfd,
        FSCONFIG_CMD_CREATE,
        core::ptr::null(),
        core::ptr::null(),
        0,
    ) == -1
    {
        return pr_perror(c"FSCONFIG_CMD_CREATE".as_ptr()) as c_int;
    }

    tmpfs = sys_fsmount(fsfd, 0, 0);
    if tmpfs == -1 {
        return pr_perror(c"fsmount".as_ptr()) as c_int;
    }

    close(fsfd);

    /* overlayfs can't be constructed on top of a detached mount. */
    if sys_move_mount(
        tmpfs,
        c"".as_ptr(),
        libc::AT_FDCWD,
        c"/tmp".as_ptr(),
        MOVE_MOUNT_F_EMPTY_PATH,
    ) != 0
    {
        return pr_perror(c"move_mount".as_ptr()) as c_int;
    }
    close(tmpfs);

    if mkdir(c"/tmp/w".as_ptr(), 0o755) == -1
        || mkdir(c"/tmp/u".as_ptr(), 0o755) == -1
        || mkdir(c"/tmp/l".as_ptr(), 0o755) == -1
    {
        return pr_perror(c"mkdir".as_ptr()) as c_int;
    }

    fsfd = sys_fsopen(c"overlay".as_ptr(), 0);
    if fsfd == -1 {
        return pr_perror(c"fsopen(overlay)".as_ptr()) as c_int;
    }
    if sys_fsconfig(
        fsfd,
        FSCONFIG_SET_STRING,
        c"source".as_ptr(),
        c"test".as_ptr() as *const c_void,
        0,
    ) == -1
        || sys_fsconfig(
            fsfd,
            FSCONFIG_SET_STRING,
            c"lowerdir".as_ptr(),
            c"/tmp/l".as_ptr() as *const c_void,
            0,
        ) == -1
        || sys_fsconfig(
            fsfd,
            FSCONFIG_SET_STRING,
            c"upperdir".as_ptr(),
            c"/tmp/u".as_ptr() as *const c_void,
            0,
        ) == -1
        || sys_fsconfig(
            fsfd,
            FSCONFIG_SET_STRING,
            c"workdir".as_ptr(),
            c"/tmp/w".as_ptr() as *const c_void,
            0,
        ) == -1
    {
        return pr_perror(c"fsconfig".as_ptr()) as c_int;
    }
    if sys_fsconfig(
        fsfd,
        FSCONFIG_CMD_CREATE,
        core::ptr::null(),
        core::ptr::null(),
        0,
    ) == -1
    {
        return pr_perror(c"fsconfig".as_ptr()) as c_int;
    }
    ovl = sys_fsmount(fsfd, 0, 0);
    if ovl == -1 {
        return pr_perror(c"fsmount".as_ptr()) as c_int;
    }

    ovl
}

/*
 * Check that the file device and inode shown in /proc/pid/maps match values
 * returned by stat(2).
 */
unsafe fn test() -> c_int {
    let mut stx: libc::statx = core::mem::zeroed();
    let mut mstx: libc::statx = core::mem::zeroed();
    let ovl: c_int;
    let fd: c_int;
    let addr: *mut c_void;

    ovl = ovl_mount();
    if ovl == -1 {
        return -1;
    }

    fd = openat(
        ovl,
        c"test".as_ptr(),
        libc::O_RDWR | libc::O_CREAT,
        0o644,
    );
    if fd == -1 {
        return pr_perror(c"openat".as_ptr()) as c_int;
    }

    addr = mmap(
        core::ptr::null_mut(),
        4096,
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_FILE | libc::MAP_SHARED,
        fd,
        0,
    );
    if addr == libc::MAP_FAILED {
        return pr_perror(c"mmap".as_ptr()) as c_int;
    }

    if get_file_dev_and_inode(addr, &mut mstx) != 0 {
        return -1;
    }
    if statx(
        fd,
        c"".as_ptr(),
        libc::AT_EMPTY_PATH | libc::AT_STATX_SYNC_AS_STAT,
        libc::STATX_INO,
        &mut stx,
    ) != 0
    {
        return pr_perror(c"statx".as_ptr()) as c_int;
    }

    if stx.stx_dev_major != mstx.stx_dev_major
        || stx.stx_dev_minor != mstx.stx_dev_minor
        || stx.stx_ino != mstx.stx_ino
    {
        return pr_fail(
            c"unmatched dev:ino %x:%x:%llx (expected %x:%x:%llx)\n".as_ptr(),
            mstx.stx_dev_major,
            mstx.stx_dev_minor,
            mstx.stx_ino,
            stx.stx_dev_major,
            stx.stx_dev_minor,
            stx.stx_ino,
        );
    }

    ksft_test_result_pass(c"devices are matched\n".as_ptr());
    0
}

unsafe fn main_impl(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let fsfd: c_int;

    fsfd = sys_fsopen(c"overlay".as_ptr(), 0);
    if fsfd == -1 {
        ksft_test_result_skip(c"unable to create overlay mount\n".as_ptr());
        return 1;
    }
    close(fsfd);

    /* Create a new mount namespace to not care about cleaning test mounts. */
    if unshare(libc::CLONE_NEWNS) == -1 {
        ksft_test_result_skip(c"unable to create a new mount namespace\n".as_ptr());
        return 1;
    }
    if sys_mount(
        core::ptr::null(),
        c"/".as_ptr(),
        core::ptr::null(),
        (libc::MS_SLAVE | libc::MS_REC) as c_ulong,
        core::ptr::null(),
    ) == -1
    {
        pr_perror(c"mount".as_ptr());
        return 1;
    }

    ksft_set_plan(1);

    if test() != 0 {
        return 1;
    }

    ksft_exit_pass();
}

fn main() {
    let code = unsafe { main_impl(0, core::ptr::null_mut()) };
    std::process::exit(code);
}
