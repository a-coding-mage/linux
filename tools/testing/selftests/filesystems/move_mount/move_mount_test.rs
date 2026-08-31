// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2026 Christian Brauner <brauner@kernel.org>

// C dependencies translated as external declarations/constants where they are
// referenced by this file: errno.h, fcntl.h, sched.h, stdio.h, string.h,
// sys/stat.h, sys/mount.h, unistd.h, sys/syscall.h, linux/stat.h,
// ../wrappers.h, ../utils.h, ../statmount/statmount.h,
// ../../kselftest_harness.h.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;

type uint64_t = u64;

const AT_FDCWD: c_int = -100;
const AT_EMPTY_PATH: c_int = 0x1000;
const AT_RECURSIVE: c_int = 0x8000;
const CLONE_NEWNS: c_int = 0x00020000;
const EINVAL: c_int = 22;
const FSCONFIG_CMD_CREATE: c_int = 6;
const FSOPEN_CLOEXEC: c_uint = 0x00000001;
const FSMOUNT_CLOEXEC: c_uint = 0x00000001;
const MNT_DETACH: c_int = 2;
const MOVE_MOUNT_F_EMPTY_PATH: c_uint = 0x00000004;

// #ifndef MOVE_MOUNT_BENEATH
const MOVE_MOUNT_BENEATH: c_uint = 0x00000200;
// #endif

const MS_PRIVATE: c_ulong = 1 << 18;
const MS_REC: c_ulong = 16384;
const OPEN_TREE_CLONE: c_uint = 1;
const OPEN_TREE_CLOEXEC: c_uint = 0o2000000;
const STATMOUNT_MNT_BASIC: c_uint = 0x00000001;
const STATX_MNT_ID_UNIQUE: c_uint = 0x00004000;

type c_uint = u32;

#[repr(C)]
struct statx {
    stx_mask: c_uint,
    _rest: [u8; 0],
    stx_mnt_id: uint64_t,
}

#[repr(C)]
struct statmount {
    mnt_parent_id: uint64_t,
}

#[repr(C)]
struct move_mount {
    orig_root_id: uint64_t,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn chdir(path: *const c_char) -> c_int;
    fn chroot(path: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fchdir(fd: c_int) -> c_int;
    fn mkdir(path: *const c_char, mode: c_uint) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn rmdir(path: *const c_char) -> c_int;
    fn statx(
        dirfd: c_int,
        pathname: *const c_char,
        flags: c_int,
        mask: c_uint,
        statxbuf: *mut statx,
    ) -> c_int;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;
    fn unshare(flags: c_int) -> c_int;

    fn get_unique_mnt_id(path: *const c_char) -> uint64_t;
    fn setup_userns() -> c_int;
    fn statmount(
        mnt_id: uint64_t,
        request_mask: uint64_t,
        flags: c_uint,
        mask: c_uint,
        buf: *mut statmount,
        bufsize: usize,
        spare: c_uint,
    ) -> c_int;
    fn sys_fsconfig(
        fd: c_int,
        cmd: c_uint,
        key: *const c_char,
        value: *const c_char,
        aux: c_int,
    ) -> c_int;
    fn sys_fsmount(fs_fd: c_int, flags: c_uint, attr_flags: c_uint) -> c_int;
    fn sys_fsopen(fs_name: *const c_char, flags: c_uint) -> c_int;
    fn sys_move_mount(
        from_dfd: c_int,
        from_pathname: *const c_char,
        to_dfd: c_int,
        to_pathname: *const c_char,
        flags: c_uint,
    ) -> c_int;
    fn sys_open_tree(dfd: c_int, filename: *const c_char, flags: c_uint) -> c_int;
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr $(,)?) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr $(,)?) => {
        assert_ne!($left, $right)
    };
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr $(,)?) => {
        assert!($left >= $right)
    };
}

unsafe fn get_unique_mnt_id_fd(fd: c_int) -> uint64_t {
    let mut sx: statx = core::mem::zeroed();
    let ret: c_int;

    ret = statx(
        fd,
        c"".as_ptr(),
        AT_EMPTY_PATH,
        STATX_MNT_ID_UNIQUE,
        &mut sx,
    );
    if ret != 0 {
        return 0;
    }

    if (sx.stx_mask & STATX_MNT_ID_UNIQUE) == 0 {
        return 0;
    }

    sx.stx_mnt_id
}

/*
 * Create a locked overmount stack at /mnt_dir for testing MNT_LOCKED
 * transfer on non-rootfs mounts.
 *
 * Mounts tmpfs A at /mnt_dir, overmounts with tmpfs B, then enters a
 * new user+mount namespace where both become locked. Returns the exit
 * code to use on failure, or 0 on success.
 */
unsafe fn setup_locked_overmount() -> c_int {
    /* Isolate so mounts don't leak. */
    if unshare(CLONE_NEWNS) != 0 {
        return 1;
    }
    if mount(c"".as_ptr(), c"/".as_ptr(), core::ptr::null(), MS_REC | MS_PRIVATE, core::ptr::null()) != 0 {
        return 2;
    }

    /*
     * Create mounts while still in the initial user namespace so
     * they become locked after the subsequent user namespace
     * unshare.
     */
    rmdir(c"/mnt_dir".as_ptr());
    if mkdir(c"/mnt_dir".as_ptr(), 0o755) != 0 {
        return 3;
    }

    /* Mount tmpfs A */
    if mount(c"tmpfs".as_ptr(), c"/mnt_dir".as_ptr(), c"tmpfs".as_ptr(), 0, core::ptr::null()) != 0 {
        return 4;
    }

    /* Overmount with tmpfs B */
    if mount(c"tmpfs".as_ptr(), c"/mnt_dir".as_ptr(), c"tmpfs".as_ptr(), 0, core::ptr::null()) != 0 {
        return 5;
    }

    /*
     * Create user+mount namespace. Mounts A and B become locked
     * because they might be covering something that is not supposed
     * to be revealed.
     */
    if setup_userns() != 0 {
        return 6;
    }

    /* Sanity check: B must be locked */
    if umount2(c"/mnt_dir".as_ptr(), MNT_DETACH) == 0 || errno != EINVAL {
        return 7;
    }

    0
}

/*
 * Create a detached tmpfs mount and return its fd, or -1 on failure.
 */
unsafe fn create_detached_tmpfs() -> c_int {
    let fs_fd: c_int;
    let mnt_fd: c_int;

    fs_fd = sys_fsopen(c"tmpfs".as_ptr(), FSOPEN_CLOEXEC);
    if fs_fd < 0 {
        return -1;
    }

    if sys_fsconfig(fs_fd, FSCONFIG_CMD_CREATE, core::ptr::null(), core::ptr::null(), 0) != 0 {
        close(fs_fd);
        return -1;
    }

    mnt_fd = sys_fsmount(fs_fd, FSMOUNT_CLOEXEC, 0);
    close(fs_fd);
    mnt_fd
}

impl move_mount {
    unsafe fn setup(&mut self) {
        ASSERT_EQ!(unshare(CLONE_NEWNS), 0);

        ASSERT_EQ!(mount(c"".as_ptr(), c"/".as_ptr(), core::ptr::null(), MS_REC | MS_PRIVATE, core::ptr::null()), 0);

        self.orig_root_id = get_unique_mnt_id(c"/".as_ptr());
        ASSERT_NE!(self.orig_root_id, 0);
    }

    unsafe fn teardown(&mut self) {}
}

/*
 * Test successful MOVE_MOUNT_BENEATH on the rootfs.
 * Mount a clone beneath /, fchdir to the clone, chroot to switch root,
 * then detach the old root.
 */
unsafe fn beneath_rootfs_success(self_: &mut move_mount) {
    let fd_tree: c_int;
    let ret: c_int;
    let clone_id: uint64_t;
    let root_id: uint64_t;

    fd_tree = sys_open_tree(
        AT_FDCWD,
        c"/".as_ptr(),
        OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC,
    );
    ASSERT_GE!(fd_tree, 0);

    clone_id = get_unique_mnt_id_fd(fd_tree);
    ASSERT_NE!(clone_id, 0);
    ASSERT_NE!(clone_id, self_.orig_root_id);

    ASSERT_EQ!(fchdir(fd_tree), 0);

    ret = sys_move_mount(
        fd_tree,
        c"".as_ptr(),
        AT_FDCWD,
        c"/".as_ptr(),
        MOVE_MOUNT_F_EMPTY_PATH | MOVE_MOUNT_BENEATH,
    );
    ASSERT_EQ!(ret, 0);

    close(fd_tree);

    /* Switch root to the clone */
    ASSERT_EQ!(chroot(c".".as_ptr()), 0);

    /* Verify "/" is now the clone */
    root_id = get_unique_mnt_id(c"/".as_ptr());
    ASSERT_NE!(root_id, 0);
    ASSERT_EQ!(root_id, clone_id);

    /* Detach old root */
    ASSERT_EQ!(umount2(c".".as_ptr(), MNT_DETACH), 0);
}

/*
 * Test that after MOVE_MOUNT_BENEATH on the rootfs the old root is
 * stacked on top of the clone. Verify via statmount that the old
 * root's parent is the clone.
 */
unsafe fn beneath_rootfs_old_root_stacked(self_: &mut move_mount) {
    let fd_tree: c_int;
    let ret: c_int;
    let clone_id: uint64_t;
    let mut sm: statmount = core::mem::zeroed();

    fd_tree = sys_open_tree(
        AT_FDCWD,
        c"/".as_ptr(),
        OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC,
    );
    ASSERT_GE!(fd_tree, 0);

    clone_id = get_unique_mnt_id_fd(fd_tree);
    ASSERT_NE!(clone_id, 0);
    ASSERT_NE!(clone_id, self_.orig_root_id);

    ASSERT_EQ!(fchdir(fd_tree), 0);

    ret = sys_move_mount(
        fd_tree,
        c"".as_ptr(),
        AT_FDCWD,
        c"/".as_ptr(),
        MOVE_MOUNT_F_EMPTY_PATH | MOVE_MOUNT_BENEATH,
    );
    ASSERT_EQ!(ret, 0);

    close(fd_tree);

    ASSERT_EQ!(chroot(c".".as_ptr()), 0);

    /* Old root's parent should now be the clone */
    ASSERT_EQ!(
        statmount(
            self_.orig_root_id,
            0,
            0,
            STATMOUNT_MNT_BASIC,
            &mut sm,
            size_of::<statmount>(),
            0,
        ),
        0,
    );
    ASSERT_EQ!(sm.mnt_parent_id, clone_id);

    ASSERT_EQ!(umount2(c".".as_ptr(), MNT_DETACH), 0);
}

/*
 * Test that MOVE_MOUNT_BENEATH on rootfs fails when chroot'd into a
 * subdirectory of the same mount. The caller's fs->root.dentry doesn't
 * match mnt->mnt_root so the kernel rejects it.
 */
unsafe fn beneath_rootfs_in_chroot_fail(self_: &mut move_mount) {
    let fd_tree: c_int;
    let ret: c_int;
    let chroot_id: uint64_t;
    let clone_id: uint64_t;

    rmdir(c"/chroot_dir".as_ptr());
    ASSERT_EQ!(mkdir(c"/chroot_dir".as_ptr(), 0o755), 0);

    chroot_id = get_unique_mnt_id(c"/chroot_dir".as_ptr());
    ASSERT_NE!(chroot_id, 0);
    ASSERT_EQ!(self_.orig_root_id, chroot_id);

    ASSERT_EQ!(chdir(c"/chroot_dir".as_ptr()), 0);
    ASSERT_EQ!(chroot(c".".as_ptr()), 0);

    fd_tree = sys_open_tree(
        AT_FDCWD,
        c"/".as_ptr(),
        OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC,
    );
    ASSERT_GE!(fd_tree, 0);

    clone_id = get_unique_mnt_id_fd(fd_tree);
    ASSERT_NE!(clone_id, 0);
    ASSERT_NE!(clone_id, chroot_id);

    ASSERT_EQ!(fchdir(fd_tree), 0);

    /*
     * Should fail: fs->root.dentry (/chroot_dir) doesn't match
     * the mount's mnt_root (/).
     */
    ret = sys_move_mount(
        fd_tree,
        c"".as_ptr(),
        AT_FDCWD,
        c"/".as_ptr(),
        MOVE_MOUNT_F_EMPTY_PATH | MOVE_MOUNT_BENEATH,
    );
    ASSERT_EQ!(ret, -1);
    ASSERT_EQ!(errno, EINVAL);

    close(fd_tree);
}

/*
 * Test that MOVE_MOUNT_BENEATH on rootfs succeeds when chroot'd into a
 * separate tmpfs mount. The caller's root dentry matches the mount's
 * mnt_root since it's a dedicated mount.
 */
unsafe fn beneath_rootfs_in_chroot_success(_self: &mut move_mount) {
    let fd_tree: c_int;
    let ret: c_int;
    let chroot_id: uint64_t;
    let clone_id: uint64_t;
    let root_id: uint64_t;
    let mut sm: statmount = core::mem::zeroed();

    rmdir(c"/chroot_dir".as_ptr());
    ASSERT_EQ!(mkdir(c"/chroot_dir".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mount(c"tmpfs".as_ptr(), c"/chroot_dir".as_ptr(), c"tmpfs".as_ptr(), 0, core::ptr::null()), 0);

    chroot_id = get_unique_mnt_id(c"/chroot_dir".as_ptr());
    ASSERT_NE!(chroot_id, 0);

    ASSERT_EQ!(chdir(c"/chroot_dir".as_ptr()), 0);
    ASSERT_EQ!(chroot(c".".as_ptr()), 0);

    ASSERT_EQ!(get_unique_mnt_id(c"/".as_ptr()), chroot_id);

    fd_tree = sys_open_tree(
        AT_FDCWD,
        c"/".as_ptr(),
        OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC,
    );
    ASSERT_GE!(fd_tree, 0);

    clone_id = get_unique_mnt_id_fd(fd_tree);
    ASSERT_NE!(clone_id, 0);
    ASSERT_NE!(clone_id, chroot_id);

    ASSERT_EQ!(fchdir(fd_tree), 0);

    ret = sys_move_mount(
        fd_tree,
        c"".as_ptr(),
        AT_FDCWD,
        c"/".as_ptr(),
        MOVE_MOUNT_F_EMPTY_PATH | MOVE_MOUNT_BENEATH,
    );
    ASSERT_EQ!(ret, 0);

    close(fd_tree);

    ASSERT_EQ!(chroot(c".".as_ptr()), 0);

    root_id = get_unique_mnt_id(c"/".as_ptr());
    ASSERT_NE!(root_id, 0);
    ASSERT_EQ!(root_id, clone_id);

    ASSERT_EQ!(
        statmount(
            chroot_id,
            0,
            0,
            STATMOUNT_MNT_BASIC,
            &mut sm,
            size_of::<statmount>(),
            0,
        ),
        0,
    );
    ASSERT_EQ!(sm.mnt_parent_id, clone_id);

    ASSERT_EQ!(umount2(c".".as_ptr(), MNT_DETACH), 0);
}

/*
 * Test MNT_LOCKED transfer when mounting beneath rootfs in a user+mount
 * namespace. After mount-beneath the new root gets MNT_LOCKED and the
 * old root has MNT_LOCKED cleared so it can be unmounted.
 */
unsafe fn beneath_rootfs_locked_transfer(_self: &mut move_mount) {
    let fd_tree: c_int;
    let ret: c_int;
    let clone_id: uint64_t;
    let mut root_id: uint64_t;

    ASSERT_EQ!(setup_userns(), 0);

    ASSERT_EQ!(mount(c"".as_ptr(), c"/".as_ptr(), core::ptr::null(), MS_REC | MS_PRIVATE, core::ptr::null()), 0);

    fd_tree = sys_open_tree(
        AT_FDCWD,
        c"/".as_ptr(),
        OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC | AT_RECURSIVE as c_uint,
    );
    ASSERT_GE!(fd_tree, 0);

    clone_id = get_unique_mnt_id_fd(fd_tree);
    ASSERT_NE!(clone_id, 0);

    ASSERT_EQ!(fchdir(fd_tree), 0);

    ret = sys_move_mount(
        fd_tree,
        c"".as_ptr(),
        AT_FDCWD,
        c"/".as_ptr(),
        MOVE_MOUNT_F_EMPTY_PATH | MOVE_MOUNT_BENEATH,
    );
    ASSERT_EQ!(ret, 0);

    close(fd_tree);

    ASSERT_EQ!(chroot(c".".as_ptr()), 0);

    root_id = get_unique_mnt_id(c"/".as_ptr());
    ASSERT_EQ!(root_id, clone_id);

    /*
     * The old root should be unmountable (MNT_LOCKED was
     * transferred to the clone). If MNT_LOCKED wasn't
     * cleared, this would fail with EINVAL.
     */
    ASSERT_EQ!(umount2(c".".as_ptr(), MNT_DETACH), 0);

    /* Verify "/" is still the clone after detaching old root */
    root_id = get_unique_mnt_id(c"/".as_ptr());
    ASSERT_EQ!(root_id, clone_id);
}

/*
 * Test containment invariant: after mount-beneath rootfs in a user+mount
 * namespace, the new root must be MNT_LOCKED. The lock transfer from the
 * old root preserves containment -- the process cannot unmount the new root
 * to escape the namespace.
 */
unsafe fn beneath_rootfs_locked_containment(_self: &mut move_mount) {
    let fd_tree: c_int;
    let ret: c_int;
    let clone_id: uint64_t;
    let mut root_id: uint64_t;

    ASSERT_EQ!(setup_userns(), 0);

    ASSERT_EQ!(mount(c"".as_ptr(), c"/".as_ptr(), core::ptr::null(), MS_REC | MS_PRIVATE, core::ptr::null()), 0);

    /* Sanity: rootfs must be locked in the new userns */
    ASSERT_EQ!(umount2(c"/".as_ptr(), MNT_DETACH), -1);
    ASSERT_EQ!(errno, EINVAL);

    fd_tree = sys_open_tree(
        AT_FDCWD,
        c"/".as_ptr(),
        OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC | AT_RECURSIVE as c_uint,
    );
    ASSERT_GE!(fd_tree, 0);

    clone_id = get_unique_mnt_id_fd(fd_tree);
    ASSERT_NE!(clone_id, 0);

    ASSERT_EQ!(fchdir(fd_tree), 0);

    ret = sys_move_mount(
        fd_tree,
        c"".as_ptr(),
        AT_FDCWD,
        c"/".as_ptr(),
        MOVE_MOUNT_F_EMPTY_PATH | MOVE_MOUNT_BENEATH,
    );
    ASSERT_EQ!(ret, 0);

    close(fd_tree);

    ASSERT_EQ!(chroot(c".".as_ptr()), 0);

    root_id = get_unique_mnt_id(c"/".as_ptr());
    ASSERT_EQ!(root_id, clone_id);

    /* Detach old root (MNT_LOCKED was cleared from it) */
    ASSERT_EQ!(umount2(c".".as_ptr(), MNT_DETACH), 0);

    /* Verify "/" is still the clone after detaching old root */
    root_id = get_unique_mnt_id(c"/".as_ptr());
    ASSERT_EQ!(root_id, clone_id);

    /*
     * The new root must be locked (MNT_LOCKED was transferred
     * from the old root). Attempting to unmount it must fail
     * with EINVAL, preserving the containment invariant.
     */
    ASSERT_EQ!(umount2(c"/".as_ptr(), MNT_DETACH), -1);
    ASSERT_EQ!(errno, EINVAL);
}

/*
 * Test MNT_LOCKED transfer when mounting beneath a non-rootfs locked mount.
 * Mounts created before unshare(CLONE_NEWUSER | CLONE_NEWNS) become locked
 * in the new namespace. Mount-beneath transfers the lock from the displaced
 * mount to the new mount, so the displaced mount can be unmounted.
 */
unsafe fn beneath_non_rootfs_locked_transfer(_self: &mut move_mount) {
    let mnt_fd: c_int;
    let ret: c_int;
    let mnt_new_id: uint64_t;
    let mnt_visible_id: uint64_t;

    ASSERT_EQ!(setup_locked_overmount(), 0);

    mnt_fd = create_detached_tmpfs();
    ASSERT_GE!(mnt_fd, 0);

    mnt_new_id = get_unique_mnt_id_fd(mnt_fd);
    ASSERT_NE!(mnt_new_id, 0);

    /* Move mount beneath B (which is locked) */
    ret = sys_move_mount(
        mnt_fd,
        c"".as_ptr(),
        AT_FDCWD,
        c"/mnt_dir".as_ptr(),
        MOVE_MOUNT_F_EMPTY_PATH | MOVE_MOUNT_BENEATH,
    );
    ASSERT_EQ!(ret, 0);

    close(mnt_fd);

    /*
     * B should now be unmountable (MNT_LOCKED was transferred
     * to the new mount beneath it). If MNT_LOCKED wasn't
     * cleared from B, this would fail with EINVAL.
     */
    ASSERT_EQ!(umount2(c"/mnt_dir".as_ptr(), MNT_DETACH), 0);

    /* Verify the new mount is now visible */
    mnt_visible_id = get_unique_mnt_id(c"/mnt_dir".as_ptr());
    ASSERT_EQ!(mnt_visible_id, mnt_new_id);
}

/*
 * Test MNT_LOCKED containment when mounting beneath a non-rootfs mount
 * that was locked during unshare(CLONE_NEWUSER | CLONE_NEWNS).
 * Mounts created before unshare become locked in the new namespace.
 * Mount-beneath transfers the lock, preserving containment: the new
 * mount cannot be unmounted, but the displaced mount can.
 */
unsafe fn beneath_non_rootfs_locked_containment(_self: &mut move_mount) {
    let mnt_fd: c_int;
    let ret: c_int;
    let mnt_new_id: uint64_t;
    let mnt_visible_id: uint64_t;

    ASSERT_EQ!(setup_locked_overmount(), 0);

    mnt_fd = create_detached_tmpfs();
    ASSERT_GE!(mnt_fd, 0);

    mnt_new_id = get_unique_mnt_id_fd(mnt_fd);
    ASSERT_NE!(mnt_new_id, 0);

    /*
     * Move new tmpfs beneath B at /mnt_dir.
     * Stack becomes: A -> new -> B
     * Lock transfers from B to new.
     */
    ret = sys_move_mount(
        mnt_fd,
        c"".as_ptr(),
        AT_FDCWD,
        c"/mnt_dir".as_ptr(),
        MOVE_MOUNT_F_EMPTY_PATH | MOVE_MOUNT_BENEATH,
    );
    ASSERT_EQ!(ret, 0);

    close(mnt_fd);

    /*
     * B lost MNT_LOCKED -- unmounting it must succeed.
     * This reveals the new mount at /mnt_dir.
     */
    ASSERT_EQ!(umount2(c"/mnt_dir".as_ptr(), MNT_DETACH), 0);

    /* Verify the new mount is now visible */
    mnt_visible_id = get_unique_mnt_id(c"/mnt_dir".as_ptr());
    ASSERT_EQ!(mnt_visible_id, mnt_new_id);

    /*
     * The new mount gained MNT_LOCKED -- unmounting it must
     * fail with EINVAL, preserving the containment invariant.
     */
    ASSERT_EQ!(umount2(c"/mnt_dir".as_ptr(), MNT_DETACH), -1);
    ASSERT_EQ!(errno, EINVAL);
}

fn main() {}
