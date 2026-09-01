// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024 Christian Brauner <brauner@kernel.org>

// C dependencies: _GNU_SOURCE, fcntl.h, limits.h, sched.h, stdio.h, string.h,
// linux/fs.h, sys/ioctl.h, sys/stat.h, sys/mount.h, unistd.h, pidfd.h,
// kselftest_harness.h, ../filesystems/wrappers.h.

use core::mem::MaybeUninit;

#[repr(C)]
struct pidfd_bind_mount {
    template: [libc::c_char; libc::PATH_MAX as usize],
    fd_tmp: libc::c_int,
    pidfd: libc::c_int,
    st1: libc::stat,
    st2: libc::stat,
    gen1: u32,
    gen2: u32,
    must_unmount: bool,
}

unsafe extern "C" {
    fn sys_pidfd_open(pid: libc::pid_t, flags: libc::c_uint) -> libc::c_int;
    fn sys_open_tree(
        dfd: libc::c_int,
        filename: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn move_mount(
        from_dfd: libc::c_int,
        from_pathname: *const libc::c_char,
        to_dfd: libc::c_int,
        to_pathname: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
}

const OPEN_TREE_CLONE: libc::c_uint = 1;
const OPEN_TREE_CLOEXEC: libc::c_uint = libc::O_CLOEXEC as libc::c_uint;
const MOVE_MOUNT_F_EMPTY_PATH: libc::c_uint = 0x00000004;
const MOVE_MOUNT_T_EMPTY_PATH: libc::c_uint = 0x00000040;

// From linux/fs.h; kept as the ioctl request used by FS_IOC_GETVERSION.
const FS_IOC_GETVERSION: libc::c_ulong = 0x80087601;

unsafe fn fixture_setup_pidfd_bind_mount(self_: *mut pidfd_bind_mount) {
    unsafe {
        (*self_).fd_tmp = -libc::EBADF;
        (*self_).must_unmount = false;
        assert_eq!(libc::unshare(libc::CLONE_NEWNS), 0);
        assert!(
            libc::snprintf(
                (*self_).template.as_mut_ptr(),
                libc::PATH_MAX as usize,
                c"%s".as_ptr(),
                c"/tmp/pidfd_bind_mount_XXXXXX".as_ptr(),
            ) <= libc::PATH_MAX
        );
        (*self_).fd_tmp = libc::mkstemp((*self_).template.as_mut_ptr());
        assert!((*self_).fd_tmp >= 0);
        (*self_).pidfd = sys_pidfd_open(libc::getpid(), 0);
        assert!((*self_).pidfd >= 0);
        assert!(libc::fstat((*self_).pidfd, &mut (*self_).st1) >= 0);
        assert_eq!(
            libc::ioctl(
                (*self_).pidfd,
                FS_IOC_GETVERSION,
                &mut (*self_).gen1 as *mut u32,
            ),
            0
        );
    }
}

unsafe fn fixture_teardown_pidfd_bind_mount(self_: *mut pidfd_bind_mount) {
    unsafe {
        assert_eq!(libc::close((*self_).fd_tmp), 0);
        if (*self_).must_unmount {
            assert_eq!(libc::umount2((*self_).template.as_ptr(), 0), 0);
        }
        assert_eq!(libc::unlink((*self_).template.as_ptr()), 0);
    }
}

/*
 * Test that a detached mount can be created for a pidfd and then
 * attached to the filesystem hierarchy.
 */
unsafe fn test_pidfd_bind_mount_bind_mount(self_: *mut pidfd_bind_mount) {
    unsafe {
        let fd_tree: libc::c_int;

        fd_tree = sys_open_tree(
            (*self_).pidfd,
            c"".as_ptr(),
            OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC | libc::AT_EMPTY_PATH as libc::c_uint,
        );
        assert!(fd_tree >= 0);

        assert_eq!(
            move_mount(
                fd_tree,
                c"".as_ptr(),
                (*self_).fd_tmp,
                c"".as_ptr(),
                MOVE_MOUNT_F_EMPTY_PATH | MOVE_MOUNT_T_EMPTY_PATH,
            ),
            0
        );
        (*self_).must_unmount = true;

        assert_eq!(libc::close(fd_tree), 0);
    }
}

/* Test that a pidfd can be reopened through procfs. */
unsafe fn test_pidfd_bind_mount_reopen(self_: *mut pidfd_bind_mount) {
    unsafe {
        let pidfd: libc::c_int;
        let mut proc_path: [libc::c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];

        libc::sprintf(
            proc_path.as_mut_ptr(),
            c"/proc/self/fd/%d".as_ptr(),
            (*self_).pidfd,
        );
        pidfd = libc::open(
            proc_path.as_ptr(),
            libc::O_RDONLY | libc::O_NOCTTY | libc::O_CLOEXEC,
        );
        assert!(pidfd >= 0);

        assert!(libc::fstat((*self_).pidfd, &mut (*self_).st2) >= 0);
        assert_eq!(
            libc::ioctl(
                (*self_).pidfd,
                FS_IOC_GETVERSION,
                &mut (*self_).gen2 as *mut u32,
            ),
            0
        );

        assert!((*self_).st1.st_dev == (*self_).st2.st_dev && (*self_).st1.st_ino == (*self_).st2.st_ino);
        assert!((*self_).gen1 == (*self_).gen2);

        assert_eq!(libc::close(pidfd), 0);
    }
}

/*
 * Test that a detached mount can be created for a pidfd and then
 * attached to the filesystem hierarchy and reopened.
 */
unsafe fn test_pidfd_bind_mount_bind_mount_reopen(self_: *mut pidfd_bind_mount) {
    unsafe {
        let fd_tree: libc::c_int;
        let fd_pidfd_mnt: libc::c_int;

        fd_tree = sys_open_tree(
            (*self_).pidfd,
            c"".as_ptr(),
            OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC | libc::AT_EMPTY_PATH as libc::c_uint,
        );
        assert!(fd_tree >= 0);

        assert_eq!(
            move_mount(
                fd_tree,
                c"".as_ptr(),
                (*self_).fd_tmp,
                c"".as_ptr(),
                MOVE_MOUNT_F_EMPTY_PATH | MOVE_MOUNT_T_EMPTY_PATH,
            ),
            0
        );
        (*self_).must_unmount = true;

        fd_pidfd_mnt = libc::openat(
            -libc::EBADF,
            (*self_).template.as_ptr(),
            libc::O_RDONLY | libc::O_NOCTTY | libc::O_CLOEXEC,
        );
        assert!(fd_pidfd_mnt >= 0);

        assert!(libc::fstat(fd_tree, &mut (*self_).st2) >= 0);
        assert_eq!(
            libc::ioctl(
                fd_pidfd_mnt,
                FS_IOC_GETVERSION,
                &mut (*self_).gen2 as *mut u32,
            ),
            0
        );

        assert!((*self_).st1.st_dev == (*self_).st2.st_dev && (*self_).st1.st_ino == (*self_).st2.st_ino);
        assert!((*self_).gen1 == (*self_).gen2);

        assert_eq!(libc::close(fd_tree), 0);
        assert_eq!(libc::close(fd_pidfd_mnt), 0);
    }
}

// TEST_HARNESS_MAIN
fn main() {
    let _ = MaybeUninit::<pidfd_bind_mount>::uninit();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
