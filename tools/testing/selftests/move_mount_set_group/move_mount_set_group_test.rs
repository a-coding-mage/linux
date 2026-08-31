// SPDX-License-Identifier: GPL-2.0
// C source dependencies: sched.h, stdio.h, errno.h, string.h, sys/stat.h,
// sys/types.h, sys/mount.h, sys/wait.h, stdlib.h, unistd.h, fcntl.h,
// stdbool.h, stdarg.h, sys/syscall.h, and "kselftest_harness.h".

use libc::{
    c_char, c_int, c_long, c_void, gid_t, pid_t, size_t, ssize_t, uid_t, FILE, AT_FDCWD, EINTR,
    ENOENT, MNT_DETACH, MS_BIND, MS_NOATIME, MS_NODEV, MS_REC, O_CLOEXEC, O_NOCTTY, O_NOFOLLOW,
    O_RDONLY, O_WRONLY, SIGCHLD,
};
use std::ptr;

const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWUSER: c_int = 0x10000000;
const MS_SHARED: libc::c_ulong = 1 << 20;
const MS_PRIVATE: libc::c_ulong = 1 << 18;
const MOVE_MOUNT_SET_GROUP: c_int = 0x00000100;
const MOVE_MOUNT_F_EMPTY_PATH: c_int = 0x00000004;
const MOVE_MOUNT_T_EMPTY_PATH: c_int = 0x00000040;

const SET_GROUP_FROM: *const c_char = b"/tmp/move_mount_set_group_supported_from\0".as_ptr().cast();
const SET_GROUP_TO: *const c_char = b"/tmp/move_mount_set_group_supported_to\0".as_ptr().cast();
const SET_GROUP_A: *const c_char = b"/tmp/A\0".as_ptr().cast();
const __STACK_SIZE: usize = 8 * 1024 * 1024;

unsafe fn errno_location() -> *mut c_int {
    libc::__errno_location()
}

unsafe fn write_nointr(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    let mut ret: ssize_t;

    loop {
        ret = libc::write(fd, buf, count);
        if !(ret < 0 && *errno_location() == EINTR) {
            break;
        }
    }

    ret
}

unsafe fn write_file(path: *const c_char, buf: *const c_void, count: size_t) -> c_int {
    let fd: c_int;
    let ret: ssize_t;

    fd = libc::open(path, O_WRONLY | O_CLOEXEC | O_NOCTTY | O_NOFOLLOW);
    if fd < 0 {
        return -1;
    }

    ret = write_nointr(fd, buf, count);
    libc::close(fd);
    if ret < 0 || ret as size_t != count {
        return -1;
    }

    0
}

unsafe fn create_and_enter_userns() -> c_int {
    let uid: uid_t;
    let gid: gid_t;
    let mut map: [c_char; 100] = [0; 100];

    uid = libc::getuid();
    gid = libc::getgid();

    if libc::unshare(CLONE_NEWUSER) != 0 {
        return -1;
    }

    if write_file(
        b"/proc/self/setgroups\0".as_ptr().cast(),
        b"deny".as_ptr().cast(),
        b"deny".len(),
    ) != 0
        && *errno_location() != ENOENT
    {
        return -1;
    }

    libc::snprintf(
        map.as_mut_ptr(),
        map.len(),
        b"0 %d 1\0".as_ptr().cast(),
        uid,
    );
    if write_file(
        b"/proc/self/uid_map\0".as_ptr().cast(),
        map.as_ptr().cast(),
        libc::strlen(map.as_ptr()),
    ) != 0
    {
        return -1;
    }

    libc::snprintf(
        map.as_mut_ptr(),
        map.len(),
        b"0 %d 1\0".as_ptr().cast(),
        gid,
    );
    if write_file(
        b"/proc/self/gid_map\0".as_ptr().cast(),
        map.as_ptr().cast(),
        libc::strlen(map.as_ptr()),
    ) != 0
    {
        return -1;
    }

    if libc::setgid(0) != 0 {
        return -1;
    }

    if libc::setuid(0) != 0 {
        return -1;
    }

    0
}

unsafe fn prepare_unpriv_mountns() -> c_int {
    if create_and_enter_userns() != 0 {
        return -1;
    }

    if libc::unshare(CLONE_NEWNS) != 0 {
        return -1;
    }

    if libc::mount(
        ptr::null(),
        b"/\0".as_ptr().cast(),
        ptr::null(),
        MS_REC | MS_PRIVATE,
        ptr::null(),
    ) != 0
    {
        return -1;
    }

    0
}

unsafe fn get_field(src: *mut c_char, nfields: c_int) -> *mut c_char {
    let mut i: c_int;
    let mut p = src;

    i = 0;
    while i < nfields {
        while *p != 0 && *p != b' ' as c_char && *p != b'\t' as c_char {
            p = p.add(1);
        }

        if *p == 0 {
            break;
        }

        p = p.add(1);
        i += 1;
    }

    p
}

unsafe fn null_endofword(mut word: *mut c_char) {
    while *word != 0 && *word != b' ' as c_char && *word != b'\t' as c_char {
        word = word.add(1);
    }
    *word = 0;
}

unsafe fn is_shared_mount(path: *const c_char) -> bool {
    let mut len: size_t = 0;
    let mut line: *mut c_char = ptr::null_mut();
    let mut f: *mut FILE = ptr::null_mut();

    f = libc::fopen(b"/proc/self/mountinfo\0".as_ptr().cast(), b"re\0".as_ptr().cast());
    if f.is_null() {
        return false;
    }

    while libc::getline(&mut line, &mut len, f) != -1 {
        let opts: *mut c_char;
        let target: *mut c_char;

        target = get_field(line, 4);
        if target.is_null() {
            continue;
        }

        opts = get_field(target, 2);
        if opts.is_null() {
            continue;
        }

        null_endofword(target);

        if libc::strcmp(target, path) != 0 {
            continue;
        }

        null_endofword(opts);
        if !libc::strstr(opts, b"shared:\0".as_ptr().cast()).is_null() {
            return true;
        }
    }

    libc::free(line.cast());
    libc::fclose(f);

    false
}

/* Attempt to de-conflict with the selftests tree. */
// C fallback: #ifndef SKIP
// #define SKIP(s, ...) XFAIL(s, ##__VA_ARGS__)
// #endif

unsafe fn move_mount_set_group_supported() -> bool {
    let ret: c_long;

    if libc::mount(
        b"testing\0".as_ptr().cast(),
        b"/tmp\0".as_ptr().cast(),
        b"tmpfs\0".as_ptr().cast(),
        MS_NOATIME | MS_NODEV,
        b"size=100000,mode=700\0".as_ptr().cast(),
    ) != 0
    {
        return true;
    }

    if libc::mount(ptr::null(), b"/tmp\0".as_ptr().cast(), ptr::null(), MS_PRIVATE, ptr::null())
        != 0
    {
        return true;
    }

    if libc::mkdir(SET_GROUP_FROM, 0o777) != 0 {
        return true;
    }

    if libc::mkdir(SET_GROUP_TO, 0o777) != 0 {
        return true;
    }

    if libc::mount(
        b"testing\0".as_ptr().cast(),
        SET_GROUP_FROM,
        b"tmpfs\0".as_ptr().cast(),
        MS_NOATIME | MS_NODEV,
        b"size=100000,mode=700\0".as_ptr().cast(),
    ) != 0
    {
        return true;
    }

    if libc::mount(SET_GROUP_FROM, SET_GROUP_TO, ptr::null(), MS_BIND, ptr::null()) != 0 {
        return true;
    }

    if libc::mount(ptr::null(), SET_GROUP_FROM, ptr::null(), MS_SHARED, ptr::null()) != 0 {
        return true;
    }

    ret = libc::syscall(
        libc::SYS_move_mount,
        AT_FDCWD,
        SET_GROUP_FROM,
        AT_FDCWD,
        SET_GROUP_TO,
        MOVE_MOUNT_SET_GROUP,
    );
    libc::umount2(b"/tmp\0".as_ptr().cast(), MNT_DETACH);

    ret >= 0
}

// FIXTURE(move_mount_set_group) {
// };
#[repr(C)]
struct move_mount_set_group {}

unsafe fn fixture_setup_move_mount_set_group() {
    let ret: bool;

    ASSERT_EQ!(prepare_unpriv_mountns(), 0);

    ret = move_mount_set_group_supported();
    ASSERT_GE!(ret as c_int, 0);
    if !ret {
        SKIP!(return, "move_mount(MOVE_MOUNT_SET_GROUP) is not supported");
    }

    libc::umount2(b"/tmp\0".as_ptr().cast(), MNT_DETACH);

    ASSERT_EQ!(
        libc::mount(
            b"testing\0".as_ptr().cast(),
            b"/tmp\0".as_ptr().cast(),
            b"tmpfs\0".as_ptr().cast(),
            MS_NOATIME | MS_NODEV,
            b"size=100000,mode=700\0".as_ptr().cast()
        ),
        0
    );

    ASSERT_EQ!(libc::mkdir(SET_GROUP_A, 0o777), 0);

    ASSERT_EQ!(
        libc::mount(
            b"testing\0".as_ptr().cast(),
            SET_GROUP_A,
            b"tmpfs\0".as_ptr().cast(),
            MS_NOATIME | MS_NODEV,
            b"size=100000,mode=700\0".as_ptr().cast()
        ),
        0
    );
}

unsafe fn fixture_teardown_move_mount_set_group() {
    let ret: bool;

    ret = move_mount_set_group_supported();
    ASSERT_GE!(ret as c_int, 0);
    if !ret {
        SKIP!(return, "move_mount(MOVE_MOUNT_SET_GROUP) is not supported");
    }

    libc::umount2(b"/tmp\0".as_ptr().cast(), MNT_DETACH);
}

unsafe fn do_clone(
    fn_: unsafe extern "C" fn(*mut c_void) -> c_int,
    arg: *mut c_void,
    flags: c_int,
) -> pid_t {
    let stack: *mut c_void;

    stack = libc::malloc(__STACK_SIZE);
    if stack.is_null() {
        return -libc::ENOMEM;
    }

    // C used __clone2 on __ia64__; otherwise clone(fn, stack + __STACK_SIZE, flags | SIGCHLD, ...).
    libc::clone(
        fn_,
        (stack as *mut u8).add(__STACK_SIZE).cast(),
        flags | SIGCHLD,
        arg,
    )
}

unsafe fn wait_for_pid(pid: pid_t) -> c_int {
    let mut status: c_int = 0;
    let mut ret: c_int;

    loop {
        ret = libc::waitpid(pid, &mut status, 0);
        if ret == -1 {
            if *errno_location() == EINTR {
                continue;
            }

            return -1;
        }
        break;
    }

    if !libc::WIFEXITED(status) {
        return -1;
    }

    libc::WEXITSTATUS(status)
}

#[repr(C)]
struct child_args {
    unsfd: c_int,
    mntnsfd: c_int,
    shared: bool,
    mntfd: c_int,
}

unsafe extern "C" fn get_nestedns_mount_cb(data: *mut c_void) -> c_int {
    let ca: *mut child_args = data.cast();
    let mut ret: c_int;

    ret = prepare_unpriv_mountns();
    if ret != 0 {
        return 1;
    }

    if (*ca).shared {
        ret = libc::mount(ptr::null(), SET_GROUP_A, ptr::null(), MS_SHARED, ptr::null());
        if ret != 0 {
            return 1;
        }
    }

    ret = libc::open(b"/proc/self/ns/user\0".as_ptr().cast(), O_RDONLY);
    if ret < 0 {
        return 1;
    }
    (*ca).unsfd = ret;

    ret = libc::open(b"/proc/self/ns/mnt\0".as_ptr().cast(), O_RDONLY);
    if ret < 0 {
        return 1;
    }
    (*ca).mntnsfd = ret;

    ret = libc::open(SET_GROUP_A, O_RDONLY);
    if ret < 0 {
        return 1;
    }
    (*ca).mntfd = ret;

    0
}

unsafe fn test_move_mount_set_group_complex_sharing_copying() {
    let mut ca_from = child_args {
        unsfd: 0,
        mntnsfd: 0,
        shared: true,
        mntfd: 0,
    };
    let mut ca_to = child_args {
        unsfd: 0,
        mntnsfd: 0,
        shared: false,
        mntfd: 0,
    };
    let mut pid: pid_t;
    let ret: bool;

    ret = move_mount_set_group_supported();
    ASSERT_GE!(ret as c_int, 0);
    if !ret {
        SKIP!(return, "move_mount(MOVE_MOUNT_SET_GROUP) is not supported");
    }

    pid = do_clone(
        get_nestedns_mount_cb,
        (&mut ca_from as *mut child_args).cast(),
        libc::CLONE_VFORK | libc::CLONE_VM | libc::CLONE_FILES,
    );
    ASSERT_GT!(pid, 0);
    ASSERT_EQ!(wait_for_pid(pid), 0);

    pid = do_clone(
        get_nestedns_mount_cb,
        (&mut ca_to as *mut child_args).cast(),
        libc::CLONE_VFORK | libc::CLONE_VM | libc::CLONE_FILES,
    );
    ASSERT_GT!(pid, 0);
    ASSERT_EQ!(wait_for_pid(pid), 0);

    ASSERT_EQ!(
        libc::syscall(
            libc::SYS_move_mount,
            ca_from.mntfd,
            b"\0".as_ptr().cast::<c_char>(),
            ca_to.mntfd,
            b"\0".as_ptr().cast::<c_char>(),
            MOVE_MOUNT_SET_GROUP | MOVE_MOUNT_F_EMPTY_PATH | MOVE_MOUNT_T_EMPTY_PATH
        ),
        0
    );

    ASSERT_EQ!(libc::setns(ca_to.mntnsfd, CLONE_NEWNS), 0);
    ASSERT_EQ!(is_shared_mount(SET_GROUP_A) as c_int, 1);
}

// TEST_HARNESS_MAIN
