// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and __SANE_USERSPACE_TYPES__ (Use ll64).
// Includes translated as external dependencies: fcntl.h, sched.h, stdio.h,
// string.h, sys/socket.h, sys/stat.h, sys/sysmacros.h, sys/mount.h, unistd.h,
// kselftest_harness.h, ../../pidfd/pidfd.h, log.h, ../utils.h, ../wrappers.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

type size_t = usize;
type pid_t = c_int;
type FILE = c_void;

#[repr(C)]
pub struct set_layers_via_fds {
    pidfd: c_int,
}

extern "C" {
    static mut errno: c_int;

    fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
    fn mkdirat(dirfd: c_int, pathname: *const c_char, mode: c_uint) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn dup3(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> isize;
    fn free(ptr: *mut c_void);
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn _exit(status: c_int) -> !;
    fn mknodat(dirfd: c_int, pathname: *const c_char, mode: c_uint, dev: u64) -> c_int;
    fn makedev(maj: c_uint, min: c_uint) -> u64;

    fn sys_pidfd_send_signal(pidfd: c_int, sig: c_int, info: *mut c_void, flags: c_uint) -> c_int;
    fn sys_mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: usize,
        data: *const c_void,
    ) -> c_int;
    fn sys_fsopen(fsname: *const c_char, flags: c_uint) -> c_int;
    fn sys_fsconfig(
        fd: c_int,
        cmd: c_uint,
        key: *const c_char,
        value: *const c_char,
        aux: c_int,
    ) -> c_int;
    fn sys_fsmount(fd: c_int, flags: c_uint, ms_flags: c_uint) -> c_int;
    fn sys_move_mount(
        from_dfd: c_int,
        from_pathname: *const c_char,
        to_dfd: c_int,
        to_pathname: *const c_char,
        flags: c_uint,
    ) -> c_int;
    fn sys_open_tree(dfd: c_int, filename: *const c_char, flags: c_uint) -> c_int;
    fn sys_waitid(idtype: c_int, id: pid_t, infop: *mut c_void, options: c_int) -> c_int;
    fn create_child(pidfd: *mut c_int, flags: c_int) -> pid_t;
    fn get_userns_fd(uid: c_uint, gid: c_uint, map_rootid: c_uint) -> c_int;
    fn switch_userns(fd: c_int, uid: c_uint, gid: c_uint, drop_caps: bool) -> bool;
    fn read_nointr(fd: c_int, buf: *mut c_void, count: size_t) -> isize;
    fn write_nointr(fd: c_int, buf: *const c_void, count: size_t) -> isize;
    fn wait_for_pid(pid: pid_t) -> c_int;
    fn cap_down(cap: c_int) -> bool;
}

extern "Rust" {
    fn TH_LOG(msg: *const c_char);
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {
        assert_ne!($left, $right)
    };
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right)
    };
}

const EBADF: c_int = 9;
const SIGKILL: c_int = 9;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const CLONE_NEWNS: c_int = 0x00020000;
const MS_REC: usize = 16384;
const MS_SLAVE: usize = 1 << 19;
const FSCONFIG_SET_FLAG: c_uint = 0;
const FSCONFIG_SET_STRING: c_uint = 1;
const FSCONFIG_SET_FD: c_uint = 5;
const FSCONFIG_CMD_CREATE: c_uint = 6;
const MOVE_MOUNT_F_EMPTY_PATH: c_uint = 0x00000004;
const OPEN_TREE_CLONE: c_uint = 1;
const OPEN_TREE_CLOEXEC: c_uint = O_CLOEXEC as c_uint;
const O_DIRECTORY: c_int = 0o200000;
const O_CLOEXEC: c_int = 0o2000000;
const O_PATH: c_int = 0o10000000;
const AF_LOCAL: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SOCK_CLOEXEC: c_int = O_CLOEXEC;
const P_PID: c_int = 1;
const WEXITED: c_int = 4;
const S_IFCHR: c_uint = 0o020000;
const EPERM: c_int = 1;
const CAP_MKNOD: c_int = 27;
const CAP_SYS_ADMIN: c_int = 21;

unsafe fn set_layers_via_fds_setup(self_: *mut set_layers_via_fds) {
    (*self_).pidfd = -EBADF;
    EXPECT_EQ!(mkdir(c"/set_layers_via_fds".as_ptr(), 0o755), 0);
    EXPECT_EQ!(mkdir(c"/set_layers_via_fds_tmpfs".as_ptr(), 0o755), 0);
}

unsafe fn set_layers_via_fds_teardown(self_: *mut set_layers_via_fds) {
    if (*self_).pidfd >= 0 {
        EXPECT_EQ!(sys_pidfd_send_signal((*self_).pidfd, SIGKILL, core::ptr::null_mut(), 0), 0);
        EXPECT_EQ!(close((*self_).pidfd), 0);
    }
    umount2(c"/set_layers_via_fds".as_ptr(), 0);
    EXPECT_EQ!(rmdir(c"/set_layers_via_fds".as_ptr()), 0);

    umount2(c"/set_layers_via_fds_tmpfs".as_ptr(), 0);
    EXPECT_EQ!(rmdir(c"/set_layers_via_fds_tmpfs".as_ptr()), 0);
}

extern "C" {
    fn umount2(target: *const c_char, flags: c_int) -> c_int;
}

unsafe fn test_set_layers_via_fds_set_layers_via_fds(_self: *mut set_layers_via_fds) {
    let mut fd_context: c_int;
    let mut fd_tmpfs: c_int;
    let fd_overlay: c_int;
    let mut layer_fds: [c_int; 9] = [-EBADF; 9];
    let mut layers_found: [bool; 9] = [false; 9];
    let mut len: size_t = 0;
    let mut line: *mut c_char = core::ptr::null_mut();
    let f_mountinfo: *mut FILE;

    ASSERT_EQ!(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ!(sys_mount(core::ptr::null(), c"/".as_ptr(), core::ptr::null(), MS_SLAVE | MS_REC, core::ptr::null()), 0);

    fd_context = sys_fsopen(c"tmpfs".as_ptr(), 0);
    ASSERT_GE!(fd_context, 0);

    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE!(fd_tmpfs, 0);
    ASSERT_EQ!(close(fd_context), 0);

    ASSERT_EQ!(mkdirat(fd_tmpfs, c"w".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"u".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l1".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l2".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l3".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l4".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"d1".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"d2".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"d3".as_ptr(), 0o755), 0);

    layer_fds[0] = openat(fd_tmpfs, c"w".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[0], 0);
    layer_fds[1] = openat(fd_tmpfs, c"u".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[1], 0);
    layer_fds[2] = openat(fd_tmpfs, c"l1".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[2], 0);
    layer_fds[3] = openat(fd_tmpfs, c"l2".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[3], 0);
    layer_fds[4] = openat(fd_tmpfs, c"l3".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[4], 0);
    layer_fds[5] = openat(fd_tmpfs, c"l4".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[5], 0);
    layer_fds[6] = openat(fd_tmpfs, c"d1".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[6], 0);
    layer_fds[7] = openat(fd_tmpfs, c"d2".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[7], 0);
    layer_fds[8] = openat(fd_tmpfs, c"d3".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[8], 0);

    ASSERT_EQ!(sys_move_mount(fd_tmpfs, c"".as_ptr(), -EBADF, c"/tmp".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ!(close(fd_tmpfs), 0);

    fd_context = sys_fsopen(c"overlay".as_ptr(), 0);
    ASSERT_GE!(fd_context, 0);

    ASSERT_NE!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir".as_ptr(), core::ptr::null(), layer_fds[2]), 0);

    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"workdir".as_ptr(), core::ptr::null(), layer_fds[0]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"upperdir".as_ptr(), core::ptr::null(), layer_fds[1]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[2]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[3]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[4]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[5]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"datadir+".as_ptr(), core::ptr::null(), layer_fds[6]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"datadir+".as_ptr(), core::ptr::null(), layer_fds[7]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"datadir+".as_ptr(), core::ptr::null(), layer_fds[8]), 0);

    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_STRING, c"metacopy".as_ptr(), c"on".as_ptr(), 0), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);

    fd_overlay = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE!(fd_overlay, 0);
    ASSERT_EQ!(sys_move_mount(fd_overlay, c"".as_ptr(), -EBADF, c"/set_layers_via_fds".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH), 0);

    f_mountinfo = fopen(c"/proc/self/mountinfo".as_ptr(), c"r".as_ptr());
    ASSERT_NE!(f_mountinfo, core::ptr::null_mut());

    while getline(&mut line, &mut len, f_mountinfo) != -1 {
        let haystack = line;
        if !strstr(haystack, c"workdir=/tmp/w".as_ptr()).is_null() { layers_found[0] = true; }
        if !strstr(haystack, c"upperdir=/tmp/u".as_ptr()).is_null() { layers_found[1] = true; }
        if !strstr(haystack, c"lowerdir+=/tmp/l1".as_ptr()).is_null() { layers_found[2] = true; }
        if !strstr(haystack, c"lowerdir+=/tmp/l2".as_ptr()).is_null() { layers_found[3] = true; }
        if !strstr(haystack, c"lowerdir+=/tmp/l3".as_ptr()).is_null() { layers_found[4] = true; }
        if !strstr(haystack, c"lowerdir+=/tmp/l4".as_ptr()).is_null() { layers_found[5] = true; }
        if !strstr(haystack, c"datadir+=/tmp/d1".as_ptr()).is_null() { layers_found[6] = true; }
        if !strstr(haystack, c"datadir+=/tmp/d2".as_ptr()).is_null() { layers_found[7] = true; }
        if !strstr(haystack, c"datadir+=/tmp/d3".as_ptr()).is_null() { layers_found[8] = true; }
    }
    free(line as *mut c_void);

    for i in 0..layer_fds.len() {
        ASSERT_EQ!(layers_found[i], true);
        ASSERT_EQ!(close(layer_fds[i]), 0);
    }

    ASSERT_EQ!(close(fd_context), 0);
    ASSERT_EQ!(close(fd_overlay), 0);
    ASSERT_EQ!(fclose(f_mountinfo), 0);
}

unsafe fn test_set_layers_via_fds_set_500_layers_via_fds(_self: *mut set_layers_via_fds) {
    let mut fd_context: c_int;
    let mut fd_tmpfs: c_int;
    let fd_overlay: c_int;
    let fd_work: c_int;
    let fd_upper: c_int;
    let fd_lower: c_int;
    let mut layer_fds: [c_int; 500] = [-EBADF; 500];

    ASSERT_EQ!(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ!(sys_mount(core::ptr::null(), c"/".as_ptr(), core::ptr::null(), MS_SLAVE | MS_REC, core::ptr::null()), 0);

    fd_context = sys_fsopen(c"tmpfs".as_ptr(), 0);
    ASSERT_GE!(fd_context, 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE!(fd_tmpfs, 0);
    ASSERT_EQ!(close(fd_context), 0);

    for i in 0..layer_fds.len() {
        let mut path: [c_char; 100] = [0; 100];
        sprintf(path.as_mut_ptr(), c"l%d".as_ptr(), i as c_int);
        ASSERT_EQ!(mkdirat(fd_tmpfs, path.as_ptr(), 0o755), 0);
        layer_fds[i] = openat(fd_tmpfs, path.as_ptr(), O_DIRECTORY);
        ASSERT_GE!(layer_fds[i], 0);
    }

    ASSERT_EQ!(mkdirat(fd_tmpfs, c"w".as_ptr(), 0o755), 0);
    fd_work = openat(fd_tmpfs, c"w".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(fd_work, 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"u".as_ptr(), 0o755), 0);
    fd_upper = openat(fd_tmpfs, c"u".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(fd_upper, 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l501".as_ptr(), 0o755), 0);
    fd_lower = openat(fd_tmpfs, c"l501".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(fd_lower, 0);

    ASSERT_EQ!(sys_move_mount(fd_tmpfs, c"".as_ptr(), -EBADF, c"/tmp".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ!(close(fd_tmpfs), 0);

    fd_context = sys_fsopen(c"overlay".as_ptr(), 0);
    ASSERT_GE!(fd_context, 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"workdir".as_ptr(), core::ptr::null(), fd_work), 0);
    ASSERT_EQ!(close(fd_work), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"upperdir".as_ptr(), core::ptr::null(), fd_upper), 0);
    ASSERT_EQ!(close(fd_upper), 0);

    for i in 0..layer_fds.len() {
        ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[i]), 0);
        ASSERT_EQ!(close(layer_fds[i]), 0);
    }

    ASSERT_NE!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), fd_lower), 0);
    ASSERT_EQ!(close(fd_lower), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);
    fd_overlay = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE!(fd_overlay, 0);
    ASSERT_EQ!(close(fd_context), 0);
    ASSERT_EQ!(close(fd_overlay), 0);
}

unsafe fn test_set_layers_via_fds_set_override_creds(_self: *mut set_layers_via_fds) {
    let mut fd_context: c_int;
    let mut fd_tmpfs: c_int;
    let fd_overlay: c_int;
    let mut layer_fds: [c_int; 4] = [-EBADF; 4];
    let mut pid: pid_t;
    let mut pidfd: c_int = 0;

    ASSERT_EQ!(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ!(sys_mount(core::ptr::null(), c"/".as_ptr(), core::ptr::null(), MS_SLAVE | MS_REC, core::ptr::null()), 0);

    fd_context = sys_fsopen(c"tmpfs".as_ptr(), 0);
    ASSERT_GE!(fd_context, 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE!(fd_tmpfs, 0);
    ASSERT_EQ!(close(fd_context), 0);

    ASSERT_EQ!(mkdirat(fd_tmpfs, c"w".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"u".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l1".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l2".as_ptr(), 0o755), 0);

    layer_fds[0] = openat(fd_tmpfs, c"w".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[0], 0);
    layer_fds[1] = openat(fd_tmpfs, c"u".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[1], 0);
    layer_fds[2] = openat(fd_tmpfs, c"l1".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[2], 0);
    layer_fds[3] = openat(fd_tmpfs, c"l2".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[3], 0);

    ASSERT_EQ!(sys_move_mount(fd_tmpfs, c"".as_ptr(), -EBADF, c"/tmp".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ!(close(fd_tmpfs), 0);

    fd_context = sys_fsopen(c"overlay".as_ptr(), 0);
    ASSERT_GE!(fd_context, 0);
    ASSERT_NE!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir".as_ptr(), core::ptr::null(), layer_fds[2]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"workdir".as_ptr(), core::ptr::null(), layer_fds[0]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"upperdir".as_ptr(), core::ptr::null(), layer_fds[1]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[2]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[3]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_STRING, c"metacopy".as_ptr(), c"on".as_ptr(), 0), 0);

    pid = create_child(&mut pidfd, 0);
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        if sys_fsconfig(fd_context, FSCONFIG_SET_FLAG, c"override_creds".as_ptr(), core::ptr::null(), 0) != 0 {
            TH_LOG(c"sys_fsconfig should have succeeded".as_ptr());
            _exit(EXIT_FAILURE);
        }
        _exit(EXIT_SUCCESS);
    }
    ASSERT_GE!(sys_waitid(P_PID, pid, core::ptr::null_mut(), WEXITED), 0);
    ASSERT_GE!(close(pidfd), 0);

    pid = create_child(&mut pidfd, 0);
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        if sys_fsconfig(fd_context, FSCONFIG_SET_FLAG, c"nooverride_creds".as_ptr(), core::ptr::null(), 0) != 0 {
            TH_LOG(c"sys_fsconfig should have succeeded".as_ptr());
            _exit(EXIT_FAILURE);
        }
        _exit(EXIT_SUCCESS);
    }
    ASSERT_GE!(sys_waitid(P_PID, pid, core::ptr::null_mut(), WEXITED), 0);
    ASSERT_GE!(close(pidfd), 0);

    pid = create_child(&mut pidfd, 0);
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        if sys_fsconfig(fd_context, FSCONFIG_SET_FLAG, c"override_creds".as_ptr(), core::ptr::null(), 0) != 0 {
            TH_LOG(c"sys_fsconfig should have succeeded".as_ptr());
            _exit(EXIT_FAILURE);
        }
        _exit(EXIT_SUCCESS);
    }
    ASSERT_GE!(sys_waitid(P_PID, pid, core::ptr::null_mut(), WEXITED), 0);
    ASSERT_GE!(close(pidfd), 0);

    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);
    fd_overlay = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE!(fd_overlay, 0);
    ASSERT_EQ!(sys_move_mount(fd_overlay, c"".as_ptr(), -EBADF, c"/set_layers_via_fds".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ!(close(fd_context), 0);
    ASSERT_EQ!(close(fd_overlay), 0);
}

unsafe fn test_set_layers_via_fds_set_override_creds_invalid(self_: *mut set_layers_via_fds) {
    let mut fd_context: c_int;
    let mut fd_tmpfs: c_int;
    let fd_overlay: c_int;
    let ret: c_int;
    let mut layer_fds: [c_int; 4] = [-EBADF; 4];
    let pid: pid_t;
    let fd_userns1: c_int;
    let fd_userns2: c_int;
    let mut ipc_sockets: [c_int; 2] = [0; 2];
    let mut c: c_char = 0;
    const predictable_fd_context_nr: c_uint = 123;

    fd_userns1 = get_userns_fd(0, 0, 10000);
    ASSERT_GE!(fd_userns1, 0);
    fd_userns2 = get_userns_fd(0, 1234, 10000);
    ASSERT_GE!(fd_userns2, 0);

    ret = socketpair(AF_LOCAL, SOCK_STREAM | SOCK_CLOEXEC, 0, ipc_sockets.as_mut_ptr());
    ASSERT_GE!(ret, 0);

    pid = create_child(&mut (*self_).pidfd, 0);
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        if close(ipc_sockets[0]) != 0 {
            TH_LOG(c"close should have succeeded".as_ptr());
            _exit(EXIT_FAILURE);
        }
        if !switch_userns(fd_userns2, 0, 0, false) {
            TH_LOG(c"switch_userns should have succeeded".as_ptr());
            _exit(EXIT_FAILURE);
        }
        if read_nointr(ipc_sockets[1], &mut c as *mut _ as *mut c_void, 1) != 1 {
            TH_LOG(c"read_nointr should have succeeded".as_ptr());
            _exit(EXIT_FAILURE);
        }
        if close(ipc_sockets[1]) != 0 {
            TH_LOG(c"close should have succeeded".as_ptr());
            _exit(EXIT_FAILURE);
        }
        if sys_fsconfig(predictable_fd_context_nr as c_int, FSCONFIG_SET_FLAG, c"override_creds".as_ptr(), core::ptr::null(), 0) == 0 {
            TH_LOG(c"sys_fsconfig should have failed".as_ptr());
            _exit(EXIT_FAILURE);
        }
        _exit(EXIT_SUCCESS);
    }

    ASSERT_EQ!(close(ipc_sockets[1]), 0);
    ASSERT_EQ!(switch_userns(fd_userns1, 0, 0, false), true);
    ASSERT_EQ!(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ!(sys_mount(core::ptr::null(), c"/".as_ptr(), core::ptr::null(), MS_SLAVE | MS_REC, core::ptr::null()), 0);

    fd_context = sys_fsopen(c"tmpfs".as_ptr(), 0);
    ASSERT_GE!(fd_context, 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE!(fd_tmpfs, 0);
    ASSERT_EQ!(close(fd_context), 0);

    ASSERT_EQ!(mkdirat(fd_tmpfs, c"w".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"u".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l1".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l2".as_ptr(), 0o755), 0);
    layer_fds[0] = openat(fd_tmpfs, c"w".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[0], 0);
    layer_fds[1] = openat(fd_tmpfs, c"u".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[1], 0);
    layer_fds[2] = openat(fd_tmpfs, c"l1".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[2], 0);
    layer_fds[3] = openat(fd_tmpfs, c"l2".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[3], 0);

    ASSERT_EQ!(sys_move_mount(fd_tmpfs, c"".as_ptr(), -EBADF, c"/tmp".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ!(close(fd_tmpfs), 0);

    fd_context = sys_fsopen(c"overlay".as_ptr(), 0);
    ASSERT_GE!(fd_context, 0);
    ASSERT_EQ!(dup3(fd_context, predictable_fd_context_nr as c_int, 0), predictable_fd_context_nr as c_int);
    ASSERT_EQ!(close(fd_context), 0);
    fd_context = predictable_fd_context_nr as c_int;
    ASSERT_EQ!(write_nointr(ipc_sockets[0], c"1".as_ptr() as *const c_void, 1), 1);
    ASSERT_EQ!(close(ipc_sockets[0]), 0);

    ASSERT_EQ!(wait_for_pid(pid), 0);
    ASSERT_EQ!(close((*self_).pidfd), 0);
    (*self_).pidfd = -EBADF;

    ASSERT_NE!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir".as_ptr(), core::ptr::null(), layer_fds[2]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"workdir".as_ptr(), core::ptr::null(), layer_fds[0]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"upperdir".as_ptr(), core::ptr::null(), layer_fds[1]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[2]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[3]), 0);

    for i in 0..layer_fds.len() {
        ASSERT_EQ!(close(layer_fds[i]), 0);
    }

    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FLAG, c"userxattr".as_ptr(), core::ptr::null(), 0), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);
    fd_overlay = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE!(fd_overlay, 0);
    ASSERT_EQ!(sys_move_mount(fd_overlay, c"".as_ptr(), -EBADF, c"/set_layers_via_fds".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ!(close(fd_context), 0);
    ASSERT_EQ!(close(fd_overlay), 0);
    ASSERT_EQ!(close(fd_userns1), 0);
    ASSERT_EQ!(close(fd_userns2), 0);
}

unsafe fn test_set_layers_via_fds_set_override_creds_nomknod(_self: *mut set_layers_via_fds) {
    let mut fd_context: c_int;
    let mut fd_tmpfs: c_int;
    let fd_overlay: c_int;
    let mut layer_fds: [c_int; 4] = [-EBADF; 4];
    let pid: pid_t;
    let mut pidfd: c_int = 0;

    ASSERT_EQ!(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ!(sys_mount(core::ptr::null(), c"/".as_ptr(), core::ptr::null(), MS_SLAVE | MS_REC, core::ptr::null()), 0);

    fd_context = sys_fsopen(c"tmpfs".as_ptr(), 0);
    ASSERT_GE!(fd_context, 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE!(fd_tmpfs, 0);
    ASSERT_EQ!(close(fd_context), 0);

    ASSERT_EQ!(mkdirat(fd_tmpfs, c"w".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"u".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l1".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l2".as_ptr(), 0o755), 0);
    layer_fds[0] = openat(fd_tmpfs, c"w".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[0], 0);
    layer_fds[1] = openat(fd_tmpfs, c"u".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[1], 0);
    layer_fds[2] = openat(fd_tmpfs, c"l1".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[2], 0);
    layer_fds[3] = openat(fd_tmpfs, c"l2".as_ptr(), O_DIRECTORY);
    ASSERT_GE!(layer_fds[3], 0);

    ASSERT_EQ!(sys_move_mount(fd_tmpfs, c"".as_ptr(), -EBADF, c"/tmp".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ!(close(fd_tmpfs), 0);

    fd_context = sys_fsopen(c"overlay".as_ptr(), 0);
    ASSERT_GE!(fd_context, 0);
    ASSERT_NE!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir".as_ptr(), core::ptr::null(), layer_fds[2]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"workdir".as_ptr(), core::ptr::null(), layer_fds[0]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"upperdir".as_ptr(), core::ptr::null(), layer_fds[1]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[2]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[3]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FLAG, c"userxattr".as_ptr(), core::ptr::null(), 0), 0);

    pid = create_child(&mut pidfd, 0);
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        if !cap_down(CAP_MKNOD) {
            _exit(EXIT_FAILURE);
        }
        if !cap_down(CAP_SYS_ADMIN) {
            _exit(EXIT_FAILURE);
        }
        if sys_fsconfig(fd_context, FSCONFIG_SET_FLAG, c"override_creds".as_ptr(), core::ptr::null(), 0) != 0 {
            _exit(EXIT_FAILURE);
        }
        _exit(EXIT_SUCCESS);
    }
    ASSERT_EQ!(sys_waitid(P_PID, pid, core::ptr::null_mut(), WEXITED), 0);
    ASSERT_GE!(close(pidfd), 0);

    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);
    fd_overlay = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE!(fd_overlay, 0);
    ASSERT_EQ!(sys_move_mount(fd_overlay, c"".as_ptr(), -EBADF, c"/set_layers_via_fds".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ!(mknodat(fd_overlay, c"dev-zero".as_ptr(), S_IFCHR | 0o644, makedev(1, 5)), -1);
    ASSERT_EQ!(errno, EPERM);
    ASSERT_EQ!(close(fd_context), 0);
    ASSERT_EQ!(close(fd_overlay), 0);
}

unsafe fn test_set_layers_via_fds_set_500_layers_via_opath_fds(_self: *mut set_layers_via_fds) {
    let mut fd_context: c_int;
    let mut fd_tmpfs: c_int;
    let fd_overlay: c_int;
    let fd_work: c_int;
    let fd_upper: c_int;
    let fd_lower: c_int;
    let mut layer_fds: [c_int; 500] = [-EBADF; 500];

    ASSERT_EQ!(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ!(sys_mount(core::ptr::null(), c"/".as_ptr(), core::ptr::null(), MS_SLAVE | MS_REC, core::ptr::null()), 0);
    fd_context = sys_fsopen(c"tmpfs".as_ptr(), 0);
    ASSERT_GE!(fd_context, 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE!(fd_tmpfs, 0);
    ASSERT_EQ!(close(fd_context), 0);

    for i in 0..layer_fds.len() {
        let mut path: [c_char; 100] = [0; 100];
        sprintf(path.as_mut_ptr(), c"l%d".as_ptr(), i as c_int);
        ASSERT_EQ!(mkdirat(fd_tmpfs, path.as_ptr(), 0o755), 0);
        layer_fds[i] = openat(fd_tmpfs, path.as_ptr(), O_DIRECTORY | O_PATH);
        ASSERT_GE!(layer_fds[i], 0);
    }

    ASSERT_EQ!(mkdirat(fd_tmpfs, c"w".as_ptr(), 0o755), 0);
    fd_work = openat(fd_tmpfs, c"w".as_ptr(), O_DIRECTORY | O_PATH);
    ASSERT_GE!(fd_work, 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"u".as_ptr(), 0o755), 0);
    fd_upper = openat(fd_tmpfs, c"u".as_ptr(), O_DIRECTORY | O_PATH);
    ASSERT_GE!(fd_upper, 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l501".as_ptr(), 0o755), 0);
    fd_lower = openat(fd_tmpfs, c"l501".as_ptr(), O_DIRECTORY | O_PATH);
    ASSERT_GE!(fd_lower, 0);

    ASSERT_EQ!(sys_move_mount(fd_tmpfs, c"".as_ptr(), -EBADF, c"/tmp".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH), 0);
    ASSERT_EQ!(close(fd_tmpfs), 0);
    fd_context = sys_fsopen(c"overlay".as_ptr(), 0);
    ASSERT_GE!(fd_context, 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"workdir".as_ptr(), core::ptr::null(), fd_work), 0);
    ASSERT_EQ!(close(fd_work), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"upperdir".as_ptr(), core::ptr::null(), fd_upper), 0);
    ASSERT_EQ!(close(fd_upper), 0);

    for i in 0..layer_fds.len() {
        ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[i]), 0);
        ASSERT_EQ!(close(layer_fds[i]), 0);
    }

    ASSERT_NE!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), fd_lower), 0);
    ASSERT_EQ!(close(fd_lower), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);
    fd_overlay = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE!(fd_overlay, 0);
    ASSERT_EQ!(close(fd_context), 0);
    ASSERT_EQ!(close(fd_overlay), 0);
}

unsafe fn test_set_layers_via_fds_set_layers_via_detached_mount_fds(_self: *mut set_layers_via_fds) {
    let mut fd_context: c_int;
    let fd_tmpfs: c_int;
    let fd_overlay: c_int;
    let fd_tmp: c_int;
    let mut layer_fds: [c_int; 9] = [-EBADF; 9];
    let mut layers_found: [bool; 9] = [false; 9];
    let mut len: size_t = 0;
    let mut line: *mut c_char = core::ptr::null_mut();
    let f_mountinfo: *mut FILE;

    ASSERT_EQ!(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ!(sys_mount(core::ptr::null(), c"/".as_ptr(), core::ptr::null(), MS_SLAVE | MS_REC, core::ptr::null()), 0);
    fd_context = sys_fsopen(c"tmpfs".as_ptr(), 0);
    ASSERT_GE!(fd_context, 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);
    fd_tmpfs = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE!(fd_tmpfs, 0);
    ASSERT_EQ!(close(fd_context), 0);

    ASSERT_EQ!(mkdirat(fd_tmpfs, c"u".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"u/upper".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"u/work".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l1".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l2".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l3".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"l4".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"d1".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"d2".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mkdirat(fd_tmpfs, c"d3".as_ptr(), 0o755), 0);

    ASSERT_EQ!(sys_move_mount(fd_tmpfs, c"".as_ptr(), -EBADF, c"/set_layers_via_fds_tmpfs".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH), 0);
    fd_tmp = sys_open_tree(fd_tmpfs, c"u".as_ptr(), OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE!(fd_tmp, 0);

    layer_fds[0] = openat(fd_tmp, c"upper".as_ptr(), O_CLOEXEC | O_DIRECTORY | O_PATH);
    ASSERT_GE!(layer_fds[0], 0);
    layer_fds[1] = openat(fd_tmp, c"work".as_ptr(), O_CLOEXEC | O_DIRECTORY | O_PATH);
    ASSERT_GE!(layer_fds[1], 0);
    layer_fds[2] = sys_open_tree(fd_tmpfs, c"l1".as_ptr(), OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE!(layer_fds[2], 0);
    layer_fds[3] = sys_open_tree(fd_tmpfs, c"l2".as_ptr(), OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE!(layer_fds[3], 0);
    layer_fds[4] = sys_open_tree(fd_tmpfs, c"l3".as_ptr(), OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE!(layer_fds[4], 0);
    layer_fds[5] = sys_open_tree(fd_tmpfs, c"l4".as_ptr(), OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE!(layer_fds[5], 0);
    layer_fds[6] = sys_open_tree(fd_tmpfs, c"d1".as_ptr(), OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE!(layer_fds[6], 0);
    layer_fds[7] = sys_open_tree(fd_tmpfs, c"d2".as_ptr(), OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE!(layer_fds[7], 0);
    layer_fds[8] = sys_open_tree(fd_tmpfs, c"d3".as_ptr(), OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    ASSERT_GE!(layer_fds[8], 0);

    ASSERT_EQ!(close(fd_tmpfs), 0);
    fd_context = sys_fsopen(c"overlay".as_ptr(), 0);
    ASSERT_GE!(fd_context, 0);
    ASSERT_NE!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir".as_ptr(), core::ptr::null(), layer_fds[2]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"upperdir".as_ptr(), core::ptr::null(), layer_fds[0]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"workdir".as_ptr(), core::ptr::null(), layer_fds[1]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[2]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[3]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[4]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), core::ptr::null(), layer_fds[5]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"datadir+".as_ptr(), core::ptr::null(), layer_fds[6]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"datadir+".as_ptr(), core::ptr::null(), layer_fds[7]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_FD, c"datadir+".as_ptr(), core::ptr::null(), layer_fds[8]), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_SET_STRING, c"metacopy".as_ptr(), c"on".as_ptr(), 0), 0);
    ASSERT_EQ!(sys_fsconfig(fd_context, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0), 0);
    fd_overlay = sys_fsmount(fd_context, 0, 0);
    ASSERT_GE!(fd_overlay, 0);
    ASSERT_EQ!(sys_move_mount(fd_overlay, c"".as_ptr(), -EBADF, c"/set_layers_via_fds".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH), 0);

    f_mountinfo = fopen(c"/proc/self/mountinfo".as_ptr(), c"r".as_ptr());
    ASSERT_NE!(f_mountinfo, core::ptr::null_mut());

    while getline(&mut line, &mut len, f_mountinfo) != -1 {
        let haystack = line;
        if !strstr(haystack, c"workdir=/tmp/w".as_ptr()).is_null() { layers_found[0] = true; }
        if !strstr(haystack, c"upperdir=/tmp/u".as_ptr()).is_null() { layers_found[1] = true; }
        if !strstr(haystack, c"lowerdir+=/tmp/l1".as_ptr()).is_null() { layers_found[2] = true; }
        if !strstr(haystack, c"lowerdir+=/tmp/l2".as_ptr()).is_null() { layers_found[3] = true; }
        if !strstr(haystack, c"lowerdir+=/tmp/l3".as_ptr()).is_null() { layers_found[4] = true; }
        if !strstr(haystack, c"lowerdir+=/tmp/l4".as_ptr()).is_null() { layers_found[5] = true; }
        if !strstr(haystack, c"datadir+=/tmp/d1".as_ptr()).is_null() { layers_found[6] = true; }
        if !strstr(haystack, c"datadir+=/tmp/d2".as_ptr()).is_null() { layers_found[7] = true; }
        if !strstr(haystack, c"datadir+=/tmp/d3".as_ptr()).is_null() { layers_found[8] = true; }
    }
    free(line as *mut c_void);

    for i in 0..layer_fds.len() {
        ASSERT_EQ!(layers_found[i], true);
        ASSERT_EQ!(close(layer_fds[i]), 0);
    }

    ASSERT_EQ!(close(fd_context), 0);
    ASSERT_EQ!(close(fd_overlay), 0);
    ASSERT_EQ!(fclose(f_mountinfo), 0);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
