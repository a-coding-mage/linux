// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Tests for empty mount namespace creation via UNSHARE_EMPTY_MNTNS
 *
 * Copyright (c) 2024 Christian Brauner <brauner@kernel.org>
 */

// C dependencies: fcntl.h, linux/mount.h, linux/stat.h, sched.h, stdio.h,
// string.h, sys/mount.h, sys/stat.h, sys/types.h, sys/wait.h, unistd.h,
// ../utils.h, ../wrappers.h, empty_mntns.h, kselftest_harness.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

type pid_t = c_int;
type uid_t = c_uint;
type gid_t = c_uint;
type ssize_t = isize;

const EINVAL: c_int = 22;
const EPERM: c_int = 1;
const EEXIST: c_int = 17;
const ENOENT: c_int = 2;

const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWUTS: c_int = 0x04000000;
const CLONE_NEWIPC: c_int = 0x08000000;
const CLONE_NEWUSER: c_int = 0x10000000;

const MS_BIND: c_ulong = 4096;
const MS_REC: c_ulong = 16384;
const MS_PRIVATE: c_ulong = 1 << 18;

const O_CREAT: c_int = 0o100;
const O_RDWR: c_int = 0o2;
const F_OK: c_int = 0;
const AT_FDCWD: c_int = -100;

const FSCONFIG_SET_STRING: c_uint = 1;
const FSCONFIG_CMD_CREATE: c_uint = 6;
const MOVE_MOUNT_F_EMPTY_PATH: c_uint = 0x00000004;

const STATMOUNT_MNT_BASIC: u64 = 0x00000001;
const STATMOUNT_MNT_ROOT: u64 = 0x00000002;
const STATMOUNT_MNT_POINT: u64 = 0x00000004;
const STATMOUNT_FS_TYPE: u64 = 0x00000020;
const LSMT_ROOT: u64 = !0u64;
const UNSHARE_EMPTY_MNTNS: c_int = 0x00000002;

#[repr(C)]
struct statmount {
    size: u32,
    __spare1: u32,
    mask: u64,
    sb_dev_major: u32,
    sb_dev_minor: u32,
    sb_magic: u64,
    sb_flags: u32,
    fs_type: u32,
    mnt_id: u64,
    mnt_parent_id: u64,
    mnt_id_old: u32,
    mnt_parent_id_old: u32,
    mnt_attr: u64,
    mnt_propagation: u64,
    mnt_peer_group: u64,
    mnt_master: u64,
    propagate_from: u64,
    mnt_root: u32,
    mnt_point: u32,
    __spare2: [u64; 50],
    str_: [c_char; 0],
}

unsafe extern "C" {
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn _exit(status: c_int) -> !;
    fn unshare(flags: c_int) -> c_int;
    fn getuid() -> uid_t;
    fn getgid() -> gid_t;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
    fn chdir(path: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fchdir(fd: c_int) -> c_int;
    fn chroot(path: *const c_char) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn sethostname(name: *const c_char, len: usize) -> c_int;
    fn gethostname(name: *mut c_char, len: usize) -> c_int;

    fn enter_userns() -> c_int;
    fn count_mounts() -> ssize_t;
    fn get_unique_mnt_id(path: *const c_char) -> u64;
    fn statmount_alloc(mnt_id: u64, request_mask: u64, mask: u64, flags: c_uint) -> *mut statmount;
    fn statmount(
        mnt_id: u64,
        request_mask: u64,
        attr_mask: u64,
        mask: u64,
        buf: *mut statmount,
        bufsize: usize,
        flags: c_uint,
    ) -> c_int;
    fn listmount(
        mnt_id: u64,
        request_mask: u64,
        attr_mask: u64,
        list: *mut u64,
        num: usize,
        flags: c_uint,
    ) -> ssize_t;
    fn sys_fsopen(fs_name: *const c_char, flags: c_uint) -> c_int;
    fn sys_fsconfig(
        fd: c_int,
        cmd: c_uint,
        key: *const c_char,
        value: *const c_char,
        aux: c_int,
    ) -> c_int;
    fn sys_fsmount(fs_fd: c_int, flags: c_uint, attr_flags: c_uint) -> c_int;
    fn sys_move_mount(
        from_dfd: c_int,
        from_pathname: *const c_char,
        to_dfd: c_int,
        to_pathname: *const c_char,
        flags: c_uint,
    ) -> c_int;

    static mut errno: c_int;
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn wait_for_pid(pid: pid_t) -> c_int;

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right);
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right);
    };
}

macro_rules! SKIP {
    (return, $msg:expr) => {
        return;
    };
}

unsafe fn cstr_ptr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn cstr_mut_ptr(bytes: &mut [u8]) -> *mut c_char {
    bytes.as_mut_ptr() as *mut c_char
}

unsafe fn statmount_str(sm: *mut statmount, offset: u32) -> *const c_char {
    ((*sm).str_.as_ptr() as *const c_char).add(offset as usize)
}

static mut _empty_mntns_fixture: () = ();

unsafe fn unshare_empty_mntns_supported() -> bool {
    let pid: pid_t;
    let mut status: c_int = 0;

    pid = fork();
    if pid < 0 {
        return false;
    }

    if pid == 0 {
        if enter_userns() != 0 {
            _exit(1);
        }

        if unshare(UNSHARE_EMPTY_MNTNS) != 0 && errno == EINVAL {
            _exit(1);
        }
        _exit(0);
    }

    if waitpid(pid, &mut status, 0) != pid {
        return false;
    }

    if !WIFEXITED(status) {
        return false;
    }

    WEXITSTATUS(status) == 0
}

// FIXTURE(empty_mntns) {};

unsafe fn empty_mntns_setup() {
    if !unshare_empty_mntns_supported() {
        SKIP!(return, "UNSHARE_EMPTY_MNTNS not supported");
    }
}

unsafe fn empty_mntns_teardown() {}

/* Verify unshare succeeds, produces exactly 1 mount, and root == cwd */
unsafe fn empty_mntns_basic() {
    let pid: pid_t;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let root_id: u64;
        let cwd_id: u64;

        if enter_userns() != 0 {
            _exit(1);
        }

        if unshare(UNSHARE_EMPTY_MNTNS) != 0 {
            _exit(2);
        }

        if count_mounts() != 1 {
            _exit(3);
        }

        root_id = get_unique_mnt_id(cstr_ptr(b"/\0"));
        cwd_id = get_unique_mnt_id(cstr_ptr(b".\0"));
        if root_id == 0 || cwd_id == 0 {
            _exit(4);
        }

        if root_id != cwd_id {
            _exit(5);
        }

        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);
}

/*
 * UNSHARE_EMPTY_MNTNS combined with CLONE_NEWUSER.
 *
 * The user namespace must be created first so /proc is still accessible
 * for writing uid_map/gid_map.  The empty mount namespace is created
 * afterwards.
 */
unsafe fn empty_mntns_with_clone_newuser() {
    let pid: pid_t;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let uid: uid_t = getuid();
        let gid: gid_t = getgid();
        let mut map: [c_char; 100] = [0; 100];

        if unshare(CLONE_NEWUSER) != 0 {
            _exit(1);
        }

        snprintf(map.as_mut_ptr(), map.len(), cstr_ptr(b"0 %d 1\0"), uid);
        if write_file(cstr_ptr(b"/proc/self/uid_map\0"), map.as_ptr()) != 0 {
            _exit(2);
        }

        if write_file(cstr_ptr(b"/proc/self/setgroups\0"), cstr_ptr(b"deny\0")) != 0 {
            _exit(3);
        }

        snprintf(map.as_mut_ptr(), map.len(), cstr_ptr(b"0 %d 1\0"), gid);
        if write_file(cstr_ptr(b"/proc/self/gid_map\0"), map.as_ptr()) != 0 {
            _exit(4);
        }

        if unshare(UNSHARE_EMPTY_MNTNS) != 0 {
            _exit(5);
        }

        if count_mounts() != 1 {
            _exit(6);
        }

        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);
}

unsafe extern "C" {
    fn write_file(path: *const c_char, buf: *const c_char) -> c_int;
}

/* UNSHARE_EMPTY_MNTNS combined with other namespace flags */
unsafe fn empty_mntns_with_other_ns_flags() {
    let pid: pid_t;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        if enter_userns() != 0 {
            _exit(1);
        }

        if unshare(UNSHARE_EMPTY_MNTNS | CLONE_NEWUTS | CLONE_NEWIPC) != 0 {
            _exit(2);
        }

        if count_mounts() != 1 {
            _exit(3);
        }

        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);
}

/* EPERM without proper capabilities */
unsafe fn empty_mntns_eperm_without_caps() {
    let pid: pid_t;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        /* Skip if already root */
        if getuid() == 0 {
            _exit(0);
        }

        if unshare(UNSHARE_EMPTY_MNTNS) == 0 {
            _exit(1);
        }

        if errno != EPERM {
            _exit(2);
        }

        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);
}

/* Many source mounts still result in exactly 1 mount */
unsafe fn empty_mntns_many_source_mounts() {
    let pid: pid_t;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut tmpdir = *b"/tmp/empty_mntns_test.XXXXXX\0";
        let mut i: c_int;

        if enter_userns() != 0 {
            _exit(1);
        }

        if unshare(CLONE_NEWNS) != 0 {
            _exit(2);
        }

        if mount(core::ptr::null(), cstr_ptr(b"/\0"), core::ptr::null(), MS_REC | MS_PRIVATE, core::ptr::null()) != 0 {
            _exit(3);
        }

        if mkdtemp(cstr_mut_ptr(&mut tmpdir)) == core::ptr::null_mut() {
            _exit(4);
        }

        if mount(cstr_ptr(b"tmpfs\0"), cstr_mut_ptr(&mut tmpdir), cstr_ptr(b"tmpfs\0"), 0, cstr_ptr(b"size=1M\0") as *const c_void) != 0 {
            _exit(5);
        }

        i = 0;
        while i < 5 {
            let mut subdir: [c_char; 256] = [0; 256];

            snprintf(subdir.as_mut_ptr(), subdir.len(), cstr_ptr(b"%s/sub%d\0"), cstr_mut_ptr(&mut tmpdir), i);
            if mkdir(subdir.as_ptr(), 0o755) != 0 && errno != EEXIST {
                _exit(6);
            }
            if mount(subdir.as_ptr(), subdir.as_ptr(), core::ptr::null(), MS_BIND, core::ptr::null()) != 0 {
                _exit(7);
            }

            i += 1;
        }

        if count_mounts() < 5 {
            _exit(8);
        }

        if unshare(UNSHARE_EMPTY_MNTNS) != 0 {
            _exit(9);
        }

        if count_mounts() != 1 {
            _exit(10);
        }

        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);
}

/* CWD on a different mount gets reset to root */
unsafe fn empty_mntns_cwd_reset() {
    let pid: pid_t;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut tmpdir = *b"/tmp/empty_mntns_cwd.XXXXXX\0";
        let root_id: u64;
        let cwd_id: u64;
        let sm: *mut statmount;

        if enter_userns() != 0 {
            _exit(1);
        }

        if unshare(CLONE_NEWNS) != 0 {
            _exit(2);
        }

        if mount(core::ptr::null(), cstr_ptr(b"/\0"), core::ptr::null(), MS_REC | MS_PRIVATE, core::ptr::null()) != 0 {
            _exit(3);
        }

        if mkdtemp(cstr_mut_ptr(&mut tmpdir)) == core::ptr::null_mut() {
            _exit(4);
        }

        if mount(cstr_ptr(b"tmpfs\0"), cstr_mut_ptr(&mut tmpdir), cstr_ptr(b"tmpfs\0"), 0, cstr_ptr(b"size=1M\0") as *const c_void) != 0 {
            _exit(5);
        }

        if chdir(cstr_mut_ptr(&mut tmpdir)) != 0 {
            _exit(6);
        }

        if unshare(UNSHARE_EMPTY_MNTNS) != 0 {
            _exit(7);
        }

        root_id = get_unique_mnt_id(cstr_ptr(b"/\0"));
        cwd_id = get_unique_mnt_id(cstr_ptr(b".\0"));
        if root_id == 0 || cwd_id == 0 {
            _exit(8);
        }

        if root_id != cwd_id {
            _exit(9);
        }

        sm = statmount_alloc(root_id, 0, STATMOUNT_MNT_ROOT | STATMOUNT_MNT_POINT, 0);
        if sm.is_null() {
            _exit(10);
        }

        if strcmp(statmount_str(sm, (*sm).mnt_point), cstr_ptr(b"/\0")) != 0 {
            _exit(11);
        }

        free(sm as *mut c_void);
        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);
}

/* Verify statmount properties of the root mount */
unsafe fn empty_mntns_mount_properties() {
    let pid: pid_t;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let sm: *mut statmount;
        let root_id: u64;

        if enter_userns() != 0 {
            _exit(1);
        }

        if unshare(UNSHARE_EMPTY_MNTNS) != 0 {
            _exit(2);
        }

        root_id = get_unique_mnt_id(cstr_ptr(b"/\0"));
        if root_id == 0 {
            _exit(3);
        }

        sm = statmount_alloc(
            root_id,
            0,
            STATMOUNT_MNT_BASIC | STATMOUNT_MNT_ROOT | STATMOUNT_MNT_POINT | STATMOUNT_FS_TYPE,
            0,
        );
        if sm.is_null() {
            _exit(4);
        }

        if ((*sm).mask & STATMOUNT_MNT_POINT) == 0 {
            _exit(5);
        }

        if strcmp(statmount_str(sm, (*sm).mnt_point), cstr_ptr(b"/\0")) != 0 {
            _exit(6);
        }

        if ((*sm).mask & STATMOUNT_MNT_BASIC) == 0 {
            _exit(7);
        }

        if (*sm).mnt_id != root_id {
            _exit(8);
        }

        free(sm as *mut c_void);
        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);
}

/* Consecutive UNSHARE_EMPTY_MNTNS calls produce new namespaces */
unsafe fn empty_mntns_repeated_unshare() {
    let pid: pid_t;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let first_root_id: u64;
        let second_root_id: u64;

        if enter_userns() != 0 {
            _exit(1);
        }

        if unshare(UNSHARE_EMPTY_MNTNS) != 0 {
            _exit(2);
        }

        if count_mounts() != 1 {
            _exit(3);
        }

        first_root_id = get_unique_mnt_id(cstr_ptr(b"/\0"));

        if unshare(UNSHARE_EMPTY_MNTNS) != 0 {
            _exit(4);
        }

        if count_mounts() != 1 {
            _exit(5);
        }

        second_root_id = get_unique_mnt_id(cstr_ptr(b"/\0"));

        if first_root_id == second_root_id {
            _exit(6);
        }

        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);
}

/* Root mount's parent is itself */
unsafe fn empty_mntns_root_is_own_parent() {
    let pid: pid_t;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut sm: statmount = core::mem::zeroed();
        let root_id: u64;

        if enter_userns() != 0 {
            _exit(1);
        }

        if unshare(UNSHARE_EMPTY_MNTNS) != 0 {
            _exit(2);
        }

        root_id = get_unique_mnt_id(cstr_ptr(b"/\0"));
        if root_id == 0 {
            _exit(3);
        }

        if statmount(root_id, 0, 0, STATMOUNT_MNT_BASIC, &mut sm, core::mem::size_of::<statmount>(), 0) < 0 {
            _exit(4);
        }

        if (sm.mask & STATMOUNT_MNT_BASIC) == 0 {
            _exit(5);
        }

        if sm.mnt_parent_id != sm.mnt_id {
            _exit(6);
        }

        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);
}

/* Listmount returns only the root mount */
unsafe fn empty_mntns_listmount_single_entry() {
    let pid: pid_t;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut list: [u64; 16] = [0; 16];
        let nr_mounts: ssize_t;
        let root_id: u64;

        if enter_userns() != 0 {
            _exit(1);
        }

        if unshare(UNSHARE_EMPTY_MNTNS) != 0 {
            _exit(2);
        }

        nr_mounts = listmount(LSMT_ROOT, 0, 0, list.as_mut_ptr(), 16, 0);
        if nr_mounts != 1 {
            _exit(3);
        }

        root_id = get_unique_mnt_id(cstr_ptr(b"/\0"));
        if root_id == 0 {
            _exit(4);
        }

        if list[0] != root_id {
            _exit(5);
        }

        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);
}

/*
 * Mount tmpfs over nullfs root to build a writable filesystem from scratch.
 * This exercises the intended usage pattern: create an empty mount namespace
 * (which has a nullfs root), then mount a real filesystem over it.
 *
 * Because resolving "/" returns the process root directly (via nd_jump_root)
 * without following overmounts, we use the new mount API (fsopen/fsmount)
 * to obtain a mount fd, then fchdir + chroot to enter the new filesystem.
 */
unsafe fn empty_mntns_overmount_tmpfs() {
    let pid: pid_t;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut sm: *mut statmount;
        let mut root_id: u64;
        let cwd_id: u64;
        let mut fd: c_int;
        let fsfd: c_int;
        let mntfd: c_int;

        if enter_userns() != 0 {
            _exit(1);
        }

        if unshare(UNSHARE_EMPTY_MNTNS) != 0 {
            _exit(2);
        }

        if count_mounts() != 1 {
            _exit(3);
        }

        root_id = get_unique_mnt_id(cstr_ptr(b"/\0"));
        if root_id == 0 {
            _exit(4);
        }

        /* Verify root is nullfs */
        sm = statmount_alloc(root_id, 0, STATMOUNT_FS_TYPE, 0);
        if sm.is_null() {
            _exit(5);
        }

        if ((*sm).mask & STATMOUNT_FS_TYPE) == 0 {
            _exit(6);
        }

        if strcmp(statmount_str(sm, (*sm).fs_type), cstr_ptr(b"nullfs\0")) != 0 {
            _exit(7);
        }

        free(sm as *mut c_void);

        cwd_id = get_unique_mnt_id(cstr_ptr(b".\0"));
        if cwd_id == 0 || root_id != cwd_id {
            _exit(8);
        }

        /*
         * nullfs root is immutable.  open(O_CREAT) returns ENOENT
         * because empty_dir_lookup() returns -ENOENT before the
         * IS_IMMUTABLE permission check in may_o_create() is reached.
         */
        fd = open(cstr_ptr(b"/test\0"), O_CREAT | O_RDWR, 0o644);
        if fd >= 0 {
            close(fd);
            _exit(9);
        }
        if errno != ENOENT {
            _exit(10);
        }

        /*
         * Use the new mount API to create tmpfs and get a mount fd.
         * We need the fd because after attaching the tmpfs on top of
         * "/", path resolution of "/" still returns the process root
         * (nullfs) without following the overmount.  The mount fd
         * lets us fchdir + chroot into the tmpfs.
         */
        fsfd = sys_fsopen(cstr_ptr(b"tmpfs\0"), 0);
        if fsfd < 0 {
            _exit(11);
        }

        if sys_fsconfig(fsfd, FSCONFIG_SET_STRING, cstr_ptr(b"size\0"), cstr_ptr(b"1M\0"), 0) != 0 {
            close(fsfd);
            _exit(12);
        }

        if sys_fsconfig(fsfd, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0) != 0 {
            close(fsfd);
            _exit(13);
        }

        mntfd = sys_fsmount(fsfd, 0, 0);
        close(fsfd);
        if mntfd < 0 {
            _exit(14);
        }

        if sys_move_mount(mntfd, cstr_ptr(b"\0"), AT_FDCWD, cstr_ptr(b"/\0"), MOVE_MOUNT_F_EMPTY_PATH) != 0 {
            close(mntfd);
            _exit(15);
        }

        if count_mounts() != 2 {
            close(mntfd);
            _exit(16);
        }

        /* Enter the tmpfs via the mount fd */
        if fchdir(mntfd) != 0 {
            close(mntfd);
            _exit(17);
        }

        if chroot(cstr_ptr(b".\0")) != 0 {
            close(mntfd);
            _exit(18);
        }

        close(mntfd);

        /* Verify "/" now resolves to tmpfs */
        root_id = get_unique_mnt_id(cstr_ptr(b"/\0"));
        if root_id == 0 {
            _exit(19);
        }

        sm = statmount_alloc(root_id, 0, STATMOUNT_FS_TYPE, 0);
        if sm.is_null() {
            _exit(20);
        }

        if ((*sm).mask & STATMOUNT_FS_TYPE) == 0 {
            _exit(21);
        }

        if strcmp(statmount_str(sm, (*sm).fs_type), cstr_ptr(b"tmpfs\0")) != 0 {
            _exit(22);
        }

        free(sm as *mut c_void);

        /* Verify tmpfs is writable */
        fd = open(cstr_ptr(b"/testfile\0"), O_CREAT | O_RDWR, 0o644);
        if fd < 0 {
            _exit(23);
        }

        if write(fd, cstr_ptr(b"test\0") as *const c_void, 4) != 4 {
            close(fd);
            _exit(24);
        }

        close(fd);

        if access(cstr_ptr(b"/testfile\0"), F_OK) != 0 {
            _exit(25);
        }

        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);
}

/*
 * Tests below do not require UNSHARE_EMPTY_MNTNS support.
 */

/* Invalid unshare flags return EINVAL */
unsafe fn invalid_flags() {
    let pid: pid_t;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        if enter_userns() != 0 {
            _exit(1);
        }

        if unshare(0x80000000u32 as c_int) == 0 {
            _exit(2);
        }

        if errno != EINVAL {
            _exit(3);
        }

        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);
}

/* Regular CLONE_NEWNS still copies the full mount tree */
unsafe fn clone_newns_full_copy() {
    let pid: pid_t;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let nr_mounts_before: ssize_t;
        let nr_mounts_after: ssize_t;
        let mut tmpdir = *b"/tmp/empty_mntns_regr.XXXXXX\0";
        let mut i: c_int;

        if enter_userns() != 0 {
            _exit(1);
        }

        if unshare(CLONE_NEWNS) != 0 {
            _exit(2);
        }

        if mount(core::ptr::null(), cstr_ptr(b"/\0"), core::ptr::null(), MS_REC | MS_PRIVATE, core::ptr::null()) != 0 {
            _exit(3);
        }

        if mkdtemp(cstr_mut_ptr(&mut tmpdir)) == core::ptr::null_mut() {
            _exit(4);
        }

        if mount(cstr_ptr(b"tmpfs\0"), cstr_mut_ptr(&mut tmpdir), cstr_ptr(b"tmpfs\0"), 0, cstr_ptr(b"size=1M\0") as *const c_void) != 0 {
            _exit(5);
        }

        i = 0;
        while i < 3 {
            let mut subdir: [c_char; 256] = [0; 256];

            snprintf(subdir.as_mut_ptr(), subdir.len(), cstr_ptr(b"%s/sub%d\0"), cstr_mut_ptr(&mut tmpdir), i);
            if mkdir(subdir.as_ptr(), 0o755) != 0 && errno != EEXIST {
                _exit(6);
            }
            if mount(subdir.as_ptr(), subdir.as_ptr(), core::ptr::null(), MS_BIND, core::ptr::null()) != 0 {
                _exit(7);
            }

            i += 1;
        }

        nr_mounts_before = count_mounts();
        if nr_mounts_before < 3 {
            _exit(8);
        }

        if unshare(CLONE_NEWNS) != 0 {
            _exit(9);
        }

        nr_mounts_after = count_mounts();
        if nr_mounts_after < nr_mounts_before {
            _exit(10);
        }

        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);
}

/* Other namespace unshares are unaffected */
unsafe fn other_ns_unaffected() {
    let pid: pid_t;

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut hostname: [c_char; 256] = [0; 256];

        if enter_userns() != 0 {
            _exit(1);
        }

        if unshare(CLONE_NEWUTS) != 0 {
            _exit(2);
        }

        if sethostname(cstr_ptr(b"test-empty-mntns\0"), 16) != 0 {
            _exit(3);
        }

        if gethostname(hostname.as_mut_ptr(), hostname.len()) != 0 {
            _exit(4);
        }

        if strcmp(hostname.as_ptr(), cstr_ptr(b"test-empty-mntns\0")) != 0 {
            _exit(5);
        }

        _exit(0);
    }

    ASSERT_EQ!(wait_for_pid(pid), 0);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
