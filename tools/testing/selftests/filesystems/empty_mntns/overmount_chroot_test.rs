// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Test: rootfs overmounted multiple times with chroot into topmost
 *
 * This test creates a scenario where:
 * 1. A new mount namespace is created with a tmpfs root (via pivot_root)
 * 2. A mountpoint is created and overmounted multiple times
 * 3. The caller chroots into the topmost mount layer
 *
 * The test verifies that:
 * - Multiple overmounts create separate mount layers
 * - Each layer's files are isolated
 * - chroot correctly sets the process's root to the topmost layer
 * - After chroot, only the topmost layer's files are visible
 *
 * Copyright (c) 2024 Christian Brauner <brauner@kernel.org>
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const NR_OVERMOUNTS: usize = 5;

const O_CREAT: c_int = 0o100;
const O_RDWR: c_int = 0o2;
const F_OK: c_int = 0;
const CLONE_NEWNS: c_int = 0x0002_0000;
const MS_REC: c_ulong = 16384;
const MS_PRIVATE: c_ulong = 1 << 18;
const MNT_DETACH: c_int = 2;
const SYS_PIVOT_ROOT: c_long = 155;
const STATMOUNT_MNT_BASIC: u64 = 0x0000_0001;
const STATMOUNT_MNT_ROOT: u64 = 0x0000_0002;
const STATMOUNT_MNT_POINT: u64 = 0x0000_0004;
const STATMOUNT_FS_TYPE: u64 = 0x0000_0020;

type PidT = c_int;
type SsizeT = isize;

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
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn chdir(path: *const c_char) -> c_int;
    fn chroot(path: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn count_mounts() -> SsizeT;
    fn enter_userns() -> c_int;
    fn fork() -> PidT;
    fn free(ptr: *mut c_void);
    fn get_unique_mnt_id(path: *const c_char) -> u64;
    fn mkdir(pathname: *const c_char, mode: u32) -> c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, mode: u32) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn statmount_alloc(mnt_id: u64, request_mask: u64, mask: u64, flags: u32) -> *mut statmount;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn syscall(num: c_long, ...) -> c_long;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn wait_for_pid(pid: PidT) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> SsizeT;
    fn _exit(status: c_int) -> !;
}

/*
 * Setup a proper root filesystem using pivot_root.
 * This ensures we own the root directory in our user namespace.
 */
unsafe fn setup_root() -> c_int {
    let mut tmpdir = *b"/tmp/overmount_test.XXXXXX\0";
    let mut oldroot = [0 as c_char; 256];

    if mkdtemp(tmpdir.as_mut_ptr() as *mut c_char).is_null() {
        return -1;
    }

    /* Mount tmpfs at the temporary directory */
    if mount(
        c"tmpfs".as_ptr(),
        tmpdir.as_ptr() as *const c_char,
        c"tmpfs".as_ptr(),
        0,
        c"size=10M".as_ptr() as *const c_void,
    ) != 0
    {
        return -1;
    }

    /* Create directory for old root */
    snprintf(
        oldroot.as_mut_ptr(),
        oldroot.len(),
        c"%s/oldroot".as_ptr(),
        tmpdir.as_ptr() as *const c_char,
    );
    if mkdir(oldroot.as_ptr(), 0o755) != 0 {
        return -1;
    }

    /* pivot_root to use the tmpfs as new root */
    if syscall(
        SYS_PIVOT_ROOT,
        tmpdir.as_ptr() as *const c_char,
        oldroot.as_ptr(),
    ) != 0
    {
        return -1;
    }

    if chdir(c"/".as_ptr()) != 0 {
        return -1;
    }

    /* Unmount old root */
    if umount2(c"/oldroot".as_ptr(), MNT_DETACH) != 0 {
        return -1;
    }

    /* Remove oldroot directory */
    if rmdir(c"/oldroot".as_ptr()) != 0 {
        return -1;
    }

    0
}

/*
 * Test scenario:
 * 1. Enter a user namespace to gain CAP_SYS_ADMIN
 * 2. Create a new mount namespace
 * 3. Setup a tmpfs root via pivot_root
 * 4. Create a mountpoint /newroot and overmount it multiple times
 * 5. Create a marker file in each layer
 * 6. Chroot into /newroot (the topmost overmount)
 * 7. Verify we're in the topmost layer (only topmost marker visible)
 */
fn overmount_chroot() {
    let pid: PidT;

    unsafe {
        pid = fork();
    }
    assert!(pid >= 0);

    if pid == 0 {
        unsafe {
            let nr_mounts: SsizeT;
            let mut mnt_ids = [0_u64; NR_OVERMOUNTS + 1];
            let root_id_before: u64;
            let root_id_after: u64;
            let sm: *mut statmount;
            let mut marker = [0 as c_char; 64];
            let mut fd: c_int;
            let mut i: c_int;

            /* Step 1: Enter user namespace for privileges */
            if enter_userns() != 0 {
                _exit(1);
            }

            /* Step 2: Create a new mount namespace */
            if unshare(CLONE_NEWNS) != 0 {
                _exit(2);
            }

            /* Step 3: Make the mount tree private */
            if mount(
                core::ptr::null(),
                c"/".as_ptr(),
                core::ptr::null(),
                MS_REC | MS_PRIVATE,
                core::ptr::null(),
            ) != 0
            {
                _exit(3);
            }

            /* Step 4: Setup a proper tmpfs root via pivot_root */
            if setup_root() != 0 {
                _exit(4);
            }

            /* Create the base mount point for overmounting */
            if mkdir(c"/newroot".as_ptr(), 0o755) != 0 {
                _exit(5);
            }

            /* Mount base tmpfs on /newroot */
            if mount(
                c"tmpfs".as_ptr(),
                c"/newroot".as_ptr(),
                c"tmpfs".as_ptr(),
                0,
                c"size=1M".as_ptr() as *const c_void,
            ) != 0
            {
                _exit(6);
            }

            /* Record base mount ID */
            mnt_ids[0] = get_unique_mnt_id(c"/newroot".as_ptr());
            if mnt_ids[0] == 0 {
                _exit(7);
            }

            /* Create marker in base layer */
            fd = open(c"/newroot/layer_0".as_ptr(), O_CREAT | O_RDWR, 0o644);
            if fd < 0 {
                _exit(8);
            }
            if write(fd, c"layer_0".as_ptr() as *const c_void, 7) != 7 {
                close(fd);
                _exit(9);
            }
            close(fd);

            /* Step 5: Overmount /newroot multiple times with tmpfs */
            i = 0;
            while i < NR_OVERMOUNTS as c_int {
                if mount(
                    c"tmpfs".as_ptr(),
                    c"/newroot".as_ptr(),
                    c"tmpfs".as_ptr(),
                    0,
                    c"size=1M".as_ptr() as *const c_void,
                ) != 0
                {
                    _exit(10);
                }

                /* Record mount ID for this layer */
                mnt_ids[(i + 1) as usize] = get_unique_mnt_id(c"/newroot".as_ptr());
                if mnt_ids[(i + 1) as usize] == 0 {
                    _exit(11);
                }

                /* Create a marker file in each layer */
                snprintf(
                    marker.as_mut_ptr(),
                    marker.len(),
                    c"/newroot/layer_%d".as_ptr(),
                    i + 1,
                );
                fd = open(marker.as_ptr(), O_CREAT | O_RDWR, 0o644);
                if fd < 0 {
                    _exit(12);
                }

                if write(fd, marker.as_ptr() as *const c_void, strlen(marker.as_ptr()))
                    != strlen(marker.as_ptr()) as SsizeT
                {
                    close(fd);
                    _exit(13);
                }
                close(fd);

                i += 1;
            }

            /* Verify mount count increased */
            nr_mounts = count_mounts();
            if nr_mounts < (NR_OVERMOUNTS + 2) as SsizeT {
                _exit(14);
            }

            /* Record root mount ID before chroot */
            root_id_before = get_unique_mnt_id(c"/newroot".as_ptr());

            /* Verify this is the topmost layer's mount */
            if root_id_before != mnt_ids[NR_OVERMOUNTS] {
                _exit(15);
            }

            /* Step 6: Chroot into /newroot (the topmost overmount) */
            if chroot(c"/newroot".as_ptr()) != 0 {
                _exit(16);
            }

            /* Change to root directory within the chroot */
            if chdir(c"/".as_ptr()) != 0 {
                _exit(17);
            }

            /* Step 7: Verify we're in the topmost layer */
            root_id_after = get_unique_mnt_id(c"/".as_ptr());

            /* The mount ID should be the same as the topmost layer */
            if root_id_after != mnt_ids[NR_OVERMOUNTS] {
                _exit(18);
            }

            /* Verify the topmost layer's marker file exists */
            snprintf(
                marker.as_mut_ptr(),
                marker.len(),
                c"/layer_%d".as_ptr(),
                NR_OVERMOUNTS as c_int,
            );
            if access(marker.as_ptr(), F_OK) != 0 {
                _exit(19);
            }

            /* Verify we cannot see markers from lower layers (they're hidden) */
            i = 0;
            while i < NR_OVERMOUNTS as c_int {
                snprintf(
                    marker.as_mut_ptr(),
                    marker.len(),
                    c"/layer_%d".as_ptr(),
                    i,
                );
                if access(marker.as_ptr(), F_OK) == 0 {
                    _exit(20);
                }

                i += 1;
            }

            /* Verify the root mount is tmpfs */
            sm = statmount_alloc(
                root_id_after,
                0,
                STATMOUNT_MNT_BASIC
                    | STATMOUNT_MNT_ROOT
                    | STATMOUNT_MNT_POINT
                    | STATMOUNT_FS_TYPE,
                0,
            );
            if sm.is_null() {
                _exit(21);
            }

            if (*sm).mask & STATMOUNT_FS_TYPE != 0 {
                if strcmp((*sm).str_.as_ptr().add((*sm).fs_type as usize), c"tmpfs".as_ptr()) != 0
                {
                    free(sm as *mut c_void);
                    _exit(22);
                }
            }

            free(sm as *mut c_void);
            _exit(0);
        }
    }

    unsafe {
        assert_eq!(wait_for_pid(pid), 0);
    }
}

fn main() {
    overmount_chroot();
}
