// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2025 Miklos Szeredi <miklos@szeredi.hu>

// C source used _GNU_SOURCE.

// Needed for linux/fanotify.h
#[repr(C)]
pub struct __kernel_fsid_t {
    pub val: [::std::os::raw::c_int; 2],
}

// Dependencies from C includes:
// fcntl.h, sched.h, stdio.h, string.h, sys/stat.h, sys/mount.h, unistd.h,
// sys/syscall.h, sys/fanotify.h, kselftest_harness.h,
// ../statmount/statmount.h, ../utils.h

use ::std::ffi::c_void;
use ::std::mem::size_of;
use ::std::os::raw::{c_char, c_int, c_ulong};
use ::std::ptr;

const root_mntpoint_templ: &[u8] = b"/tmp/mount-notify_test_root.XXXXXX\0";

static mark_types: [c_int; 3] = [
    FAN_MARK_FILESYSTEM,
    FAN_MARK_MOUNT,
    FAN_MARK_INODE,
];

static mark_cmds: [c_int; 3] = [
    FAN_MARK_ADD,
    FAN_MARK_REMOVE,
    FAN_MARK_FLUSH,
];

const NUM_FAN_FDS: usize = mark_cmds.len();

#[repr(C)]
pub struct fanotify {
    pub fan_fd: [c_int; NUM_FAN_FDS],
    pub buf: [c_char; 256],
    pub rem: ::std::os::raw::c_uint,
    pub next: *mut c_void,
    pub root_mntpoint: [c_char; root_mntpoint_templ.len()],
    pub orig_root: c_int,
    pub orig_ns_fd: c_int,
    pub ns_fd: c_int,
    pub root_id: u64,
}

unsafe fn fanotify_setup(_metadata: *mut __test_metadata, self_: *mut fanotify) {
    let mut i: c_int;
    let mut ret: c_int;

    (*self_).orig_ns_fd = open(c"/proc/self/ns/mnt".as_ptr(), O_RDONLY);
    ASSERT_GE!((*self_).orig_ns_fd, 0);

    ret = setup_userns();
    ASSERT_EQ!(ret, 0);

    (*self_).ns_fd = open(c"/proc/self/ns/mnt".as_ptr(), O_RDONLY);
    ASSERT_GE!((*self_).ns_fd, 0);

    strcpy((*self_).root_mntpoint.as_mut_ptr(), root_mntpoint_templ.as_ptr() as *const c_char);
    ASSERT_NE!(mkdtemp((*self_).root_mntpoint.as_mut_ptr()), ptr::null_mut());

    (*self_).orig_root = open(c"/".as_ptr(), O_PATH | O_CLOEXEC);
    ASSERT_GE!((*self_).orig_root, 0);

    ASSERT_EQ!(
        mount(
            c"tmpfs".as_ptr(),
            (*self_).root_mntpoint.as_ptr(),
            c"tmpfs".as_ptr(),
            0,
            ptr::null()
        ),
        0
    );

    ASSERT_EQ!(chroot((*self_).root_mntpoint.as_ptr()), 0);

    ASSERT_EQ!(chdir(c"/".as_ptr()), 0);

    ASSERT_EQ!(mkdir(c"a".as_ptr(), 0o700), 0);

    ASSERT_EQ!(mkdir(c"b".as_ptr(), 0o700), 0);

    (*self_).root_id = get_unique_mnt_id(c"/".as_ptr());
    ASSERT_NE!((*self_).root_id, 0);

    i = 0;
    while (i as usize) < NUM_FAN_FDS {
        let fan_fd = fanotify_init(FAN_REPORT_FID, 0);
        // Verify that watching tmpfs mounted inside userns is allowed
        ret = fanotify_mark(
            fan_fd,
            (FAN_MARK_ADD | mark_types[i as usize]) as c_ulong,
            FAN_OPEN as u64,
            AT_FDCWD,
            c"/".as_ptr(),
        );
        ASSERT_EQ!(ret, 0);
        // ...but watching entire orig root filesystem is not allowed
        ret = fanotify_mark(
            fan_fd,
            (FAN_MARK_ADD | FAN_MARK_FILESYSTEM) as c_ulong,
            FAN_OPEN as u64,
            (*self_).orig_root,
            c".".as_ptr(),
        );
        ASSERT_NE!(ret, 0);
        close(fan_fd);

        (*self_).fan_fd[i as usize] = fanotify_init(FAN_REPORT_MNT | FAN_NONBLOCK, 0);
        ASSERT_GE!((*self_).fan_fd[i as usize], 0);
        // Verify that watching mntns where group was created is allowed
        ret = fanotify_mark(
            (*self_).fan_fd[i as usize],
            (FAN_MARK_ADD | FAN_MARK_MNTNS) as c_ulong,
            (FAN_MNT_ATTACH | FAN_MNT_DETACH) as u64,
            (*self_).ns_fd,
            ptr::null(),
        );
        ASSERT_EQ!(ret, 0);
        // ...but watching orig mntns is not allowed
        ret = fanotify_mark(
            (*self_).fan_fd[i as usize],
            (FAN_MARK_ADD | FAN_MARK_MNTNS) as c_ulong,
            (FAN_MNT_ATTACH | FAN_MNT_DETACH) as u64,
            (*self_).orig_ns_fd,
            ptr::null(),
        );
        ASSERT_NE!(ret, 0);
        // On fd[0] we do an extra ADD that changes nothing.
        // On fd[1]/fd[2] we REMOVE/FLUSH which removes the mark.
        ret = fanotify_mark(
            (*self_).fan_fd[i as usize],
            (mark_cmds[i as usize] | FAN_MARK_MNTNS) as c_ulong,
            (FAN_MNT_ATTACH | FAN_MNT_DETACH) as u64,
            (*self_).ns_fd,
            ptr::null(),
        );
        ASSERT_EQ!(ret, 0);

        i += 1;
    }

    (*self_).rem = 0;
}

unsafe fn fanotify_teardown(_metadata: *mut __test_metadata, self_: *mut fanotify) {
    let mut i: c_int;

    ASSERT_EQ!((*self_).rem, 0);
    i = 0;
    while (i as usize) < NUM_FAN_FDS {
        close((*self_).fan_fd[i as usize]);
        i += 1;
    }

    ASSERT_EQ!(fchdir((*self_).orig_root), 0);

    ASSERT_EQ!(chroot(c".".as_ptr()), 0);

    EXPECT_EQ!(umount2((*self_).root_mntpoint.as_ptr(), MNT_DETACH), 0);
    EXPECT_EQ!(chdir((*self_).root_mntpoint.as_ptr()), 0);
    EXPECT_EQ!(chdir(c"/".as_ptr()), 0);
    EXPECT_EQ!(rmdir((*self_).root_mntpoint.as_ptr()), 0);
}

unsafe fn expect_notify(
    _metadata: *mut __test_metadata,
    self_: *mut fanotify,
    mask: *mut u64,
) -> u64 {
    let mut meta: *mut fanotify_event_metadata;
    let mut mnt: *mut fanotify_event_info_mnt;
    let mut thislen: ::std::os::raw::c_uint;

    if (*self_).rem == 0 {
        let mut len: isize;
        let mut i: c_int;

        i = NUM_FAN_FDS as c_int - 1;
        while i >= 0 {
            len = read(
                (*self_).fan_fd[i as usize],
                (*self_).buf.as_mut_ptr() as *mut c_void,
                size_of_val(&(*self_).buf),
            );
            if i > 0 {
                // Groups 1,2 should get EAGAIN
                ASSERT_EQ!(len, -1);
                ASSERT_EQ!(errno, EAGAIN);
            } else {
                // Group 0 should get events
                ASSERT_GT!(len, 0);
            }
            i -= 1;
        }

        (*self_).rem = len as ::std::os::raw::c_uint;
        (*self_).next = (*self_).buf.as_mut_ptr() as *mut c_void;
    }

    meta = (*self_).next as *mut fanotify_event_metadata;
    ASSERT_TRUE!(FAN_EVENT_OK(meta, (*self_).rem));

    thislen = (*meta).event_len;
    (*self_).rem -= thislen;
    (*self_).next = ((*self_).next as *mut u8).add(thislen as usize) as *mut c_void;

    *mask = (*meta).mask;
    thislen -= size_of::<fanotify_event_metadata>() as ::std::os::raw::c_uint;

    mnt = (meta as *mut u8)
        .add((*meta).event_len as usize)
        .sub(thislen as usize) as *mut fanotify_event_info_mnt;

    ASSERT_EQ!(thislen as usize, size_of::<fanotify_event_info_mnt>());

    (*mnt).mnt_id
}

unsafe fn expect_notify_n(
    _metadata: *mut __test_metadata,
    self_: *mut fanotify,
    n: ::std::os::raw::c_uint,
    mask: *mut u64,
    mnts: *mut u64,
) {
    let mut i: ::std::os::raw::c_uint;

    i = 0;
    while i < n {
        *mnts.add(i as usize) = expect_notify(_metadata, self_, mask.add(i as usize));
        i += 1;
    }
}

unsafe fn expect_notify_mask(
    _metadata: *mut __test_metadata,
    self_: *mut fanotify,
    expect_mask: u64,
) -> u64 {
    let mut mntid: u64;
    let mut mask: u64 = 0;

    mntid = expect_notify(_metadata, self_, &mut mask);
    ASSERT_EQ!(expect_mask, mask);

    mntid
}

unsafe fn expect_notify_mask_n(
    _metadata: *mut __test_metadata,
    self_: *mut fanotify,
    mask: u64,
    n: ::std::os::raw::c_uint,
    mnts: *mut u64,
) {
    let mut i: ::std::os::raw::c_uint;

    i = 0;
    while i < n {
        *mnts.add(i as usize) = expect_notify_mask(_metadata, self_, mask);
        i += 1;
    }
}

unsafe fn verify_mount_ids(
    _metadata: *mut __test_metadata,
    list1: *const u64,
    list2: *const u64,
    num: usize,
) {
    let mut i: ::std::os::raw::c_uint;
    let mut j: ::std::os::raw::c_uint;

    // Check that neither list has any duplicates
    i = 0;
    while (i as usize) < num {
        j = 0;
        while (j as usize) < num {
            if i != j {
                ASSERT_NE!(*list1.add(i as usize), *list1.add(j as usize));
                ASSERT_NE!(*list2.add(i as usize), *list2.add(j as usize));
            }
            j += 1;
        }
        i += 1;
    }
    // Check that all list1 memebers can be found in list2. Together with
    // the above it means that the list1 and list2 represent the same sets.
    i = 0;
    while (i as usize) < num {
        j = 0;
        while (j as usize) < num {
            if *list1.add(i as usize) == *list2.add(j as usize) {
                break;
            }
            j += 1;
        }
        ASSERT_NE!(j as usize, num);
        i += 1;
    }
}

unsafe fn check_mounted(
    _metadata: *mut __test_metadata,
    mnts: *const u64,
    num: usize,
) {
    let mut ret: isize;
    let mut list: *mut u64;

    list = malloc((num + 1) * size_of::<u64>()) as *mut u64;
    ASSERT_NE!(list, ptr::null_mut());

    ret = listmount(LSMT_ROOT, 0, 0, list, num + 1, 0);
    ASSERT_EQ!(ret, num as isize);

    verify_mount_ids(_metadata, mnts, list, num);

    free(list as *mut c_void);
}

unsafe fn setup_mount_tree(_metadata: *mut __test_metadata, log2_num: c_int) {
    let mut ret: c_int;
    let mut i: c_int;

    ret = mount(c"".as_ptr(), c"/".as_ptr(), ptr::null(), MS_SHARED, ptr::null());
    ASSERT_EQ!(ret, 0);

    i = 0;
    while i < log2_num {
        ret = mount(c"/".as_ptr(), c"/".as_ptr(), ptr::null(), MS_BIND, ptr::null());
        ASSERT_EQ!(ret, 0);
        i += 1;
    }
}

unsafe fn fanotify_bind(_metadata: *mut __test_metadata, self_: *mut fanotify) {
    let mut ret: c_int;
    let mut mnts: [u64; 2] = [(*self_).root_id, 0];

    ret = mount(c"/".as_ptr(), c"/".as_ptr(), ptr::null(), MS_BIND, ptr::null());
    ASSERT_EQ!(ret, 0);

    mnts[1] = expect_notify_mask(_metadata, self_, FAN_MNT_ATTACH as u64);
    ASSERT_NE!(mnts[0], mnts[1]);

    check_mounted(_metadata, mnts.as_ptr(), 2);

    // Cleanup
    let mut detach_id: u64;
    ret = umount(c"/".as_ptr());
    ASSERT_EQ!(ret, 0);

    detach_id = expect_notify_mask(_metadata, self_, FAN_MNT_DETACH as u64);
    ASSERT_EQ!(detach_id, mnts[1]);

    check_mounted(_metadata, mnts.as_ptr(), 1);
}

unsafe fn fanotify_move(_metadata: *mut __test_metadata, self_: *mut fanotify) {
    let mut ret: c_int;
    let mut mnts: [u64; 2] = [(*self_).root_id, 0];
    let mut move_id: u64;

    ret = mount(c"/".as_ptr(), c"/a".as_ptr(), ptr::null(), MS_BIND, ptr::null());
    ASSERT_EQ!(ret, 0);

    mnts[1] = expect_notify_mask(_metadata, self_, FAN_MNT_ATTACH as u64);
    ASSERT_NE!(mnts[0], mnts[1]);

    check_mounted(_metadata, mnts.as_ptr(), 2);

    ret = move_mount(AT_FDCWD, c"/a".as_ptr(), AT_FDCWD, c"/b".as_ptr(), 0);
    ASSERT_EQ!(ret, 0);

    move_id = expect_notify_mask(_metadata, self_, (FAN_MNT_ATTACH | FAN_MNT_DETACH) as u64);
    ASSERT_EQ!(move_id, mnts[1]);

    // Cleanup
    ret = umount(c"/b".as_ptr());
    ASSERT_EQ!(ret, 0);

    check_mounted(_metadata, mnts.as_ptr(), 1);
}

unsafe fn fanotify_propagate(_metadata: *mut __test_metadata, self_: *mut fanotify) {
    const log2_num: ::std::os::raw::c_uint = 4;
    const num: ::std::os::raw::c_uint = 1 << log2_num;
    let mut mnts: [u64; num as usize] = [0; num as usize];

    setup_mount_tree(_metadata, log2_num as c_int);

    expect_notify_mask_n(
        _metadata,
        self_,
        FAN_MNT_ATTACH as u64,
        num - 1,
        mnts.as_mut_ptr().add(1),
    );

    mnts[0] = (*self_).root_id;
    check_mounted(_metadata, mnts.as_ptr(), num as usize);

    // Cleanup
    let mut ret: c_int;
    let mut mnts2: [u64; num as usize] = [0; num as usize];
    ret = umount2(c"/".as_ptr(), MNT_DETACH);
    ASSERT_EQ!(ret, 0);

    ret = mount(c"".as_ptr(), c"/".as_ptr(), ptr::null(), MS_PRIVATE, ptr::null());
    ASSERT_EQ!(ret, 0);

    mnts2[0] = (*self_).root_id;
    expect_notify_mask_n(
        _metadata,
        self_,
        FAN_MNT_DETACH as u64,
        num - 1,
        mnts2.as_mut_ptr().add(1),
    );
    verify_mount_ids(_metadata, mnts.as_ptr(), mnts2.as_ptr(), num as usize);

    check_mounted(_metadata, mnts.as_ptr(), 1);
}

unsafe fn fanotify_fsmount(_metadata: *mut __test_metadata, self_: *mut fanotify) {
    let mut ret: c_int;
    let mut fs: c_int;
    let mut mnt: c_int;
    let mut mnts: [u64; 2] = [(*self_).root_id, 0];

    fs = fsopen(c"tmpfs".as_ptr(), 0);
    ASSERT_GE!(fs, 0);

    ret = fsconfig(fs, FSCONFIG_CMD_CREATE, 0, 0, 0);
    ASSERT_EQ!(ret, 0);

    mnt = fsmount(fs, 0, 0);
    ASSERT_GE!(mnt, 0);

    close(fs);

    ret = move_mount(mnt, c"".as_ptr(), AT_FDCWD, c"/a".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH);
    ASSERT_EQ!(ret, 0);

    close(mnt);

    mnts[1] = expect_notify_mask(_metadata, self_, FAN_MNT_ATTACH as u64);
    ASSERT_NE!(mnts[0], mnts[1]);

    check_mounted(_metadata, mnts.as_ptr(), 2);

    // Cleanup
    let mut detach_id: u64;
    ret = umount(c"/a".as_ptr());
    ASSERT_EQ!(ret, 0);

    detach_id = expect_notify_mask(_metadata, self_, FAN_MNT_DETACH as u64);
    ASSERT_EQ!(detach_id, mnts[1]);

    check_mounted(_metadata, mnts.as_ptr(), 1);
}

unsafe fn fanotify_reparent(_metadata: *mut __test_metadata, self_: *mut fanotify) {
    let mut mnts: [u64; 6] = [(*self_).root_id, 0, 0, 0, 0, 0];
    let mut dmnts: [u64; 3] = [0; 3];
    let mut masks: [u64; 3] = [0; 3];
    let mut i: ::std::os::raw::c_uint;
    let mut ret: c_int;

    // Create setup with a[1] -> b[2] propagation
    ret = mount(c"/".as_ptr(), c"/a".as_ptr(), ptr::null(), MS_BIND, ptr::null());
    ASSERT_EQ!(ret, 0);

    ret = mount(c"".as_ptr(), c"/a".as_ptr(), ptr::null(), MS_SHARED, ptr::null());
    ASSERT_EQ!(ret, 0);

    ret = mount(c"/a".as_ptr(), c"/b".as_ptr(), ptr::null(), MS_BIND, ptr::null());
    ASSERT_EQ!(ret, 0);

    ret = mount(c"".as_ptr(), c"/b".as_ptr(), ptr::null(), MS_SLAVE, ptr::null());
    ASSERT_EQ!(ret, 0);

    expect_notify_mask_n(_metadata, self_, FAN_MNT_ATTACH as u64, 2, mnts.as_mut_ptr().add(1));

    check_mounted(_metadata, mnts.as_ptr(), 3);

    // Mount on a[3], which is propagated to b[4]
    ret = mount(c"/".as_ptr(), c"/a".as_ptr(), ptr::null(), MS_BIND, ptr::null());
    ASSERT_EQ!(ret, 0);

    expect_notify_mask_n(_metadata, self_, FAN_MNT_ATTACH as u64, 2, mnts.as_mut_ptr().add(3));

    check_mounted(_metadata, mnts.as_ptr(), 5);

    // Mount on b[5], not propagated
    ret = mount(c"/".as_ptr(), c"/b".as_ptr(), ptr::null(), MS_BIND, ptr::null());
    ASSERT_EQ!(ret, 0);

    mnts[5] = expect_notify_mask(_metadata, self_, FAN_MNT_ATTACH as u64);

    check_mounted(_metadata, mnts.as_ptr(), 6);

    // Umount a[3], which is propagated to b[4], but not b[5]
    // This will result in b[5] "falling" on b[2]
    ret = umount(c"/a".as_ptr());
    ASSERT_EQ!(ret, 0);

    expect_notify_n(_metadata, self_, 3, masks.as_mut_ptr(), dmnts.as_mut_ptr());
    verify_mount_ids(_metadata, mnts.as_ptr().add(3), dmnts.as_ptr(), 3);

    i = 0;
    while i < 3 {
        if dmnts[i as usize] == mnts[5] {
            ASSERT_EQ!(masks[i as usize], (FAN_MNT_ATTACH | FAN_MNT_DETACH) as u64);
        } else {
            ASSERT_EQ!(masks[i as usize], FAN_MNT_DETACH as u64);
        }
        i += 1;
    }

    mnts[3] = mnts[5];
    check_mounted(_metadata, mnts.as_ptr(), 4);

    // Cleanup
    ret = umount(c"/b".as_ptr());
    ASSERT_EQ!(ret, 0);

    ret = umount(c"/a".as_ptr());
    ASSERT_EQ!(ret, 0);

    ret = umount(c"/b".as_ptr());
    ASSERT_EQ!(ret, 0);

    expect_notify_mask_n(_metadata, self_, FAN_MNT_DETACH as u64, 3, dmnts.as_mut_ptr());
    verify_mount_ids(_metadata, mnts.as_ptr().add(1), dmnts.as_ptr(), 3);

    check_mounted(_metadata, mnts.as_ptr(), 1);
}

unsafe fn fanotify_rmdir(_metadata: *mut __test_metadata, self_: *mut fanotify) {
    let mut mnts: [u64; 3] = [(*self_).root_id, 0, 0];
    let mut ret: c_int;

    ret = mount(c"/".as_ptr(), c"/a".as_ptr(), ptr::null(), MS_BIND, ptr::null());
    ASSERT_EQ!(ret, 0);

    ret = mount(c"/".as_ptr(), c"/a/b".as_ptr(), ptr::null(), MS_BIND, ptr::null());
    ASSERT_EQ!(ret, 0);

    expect_notify_mask_n(_metadata, self_, FAN_MNT_ATTACH as u64, 2, mnts.as_mut_ptr().add(1));

    check_mounted(_metadata, mnts.as_ptr(), 3);

    ret = chdir(c"/a".as_ptr());
    ASSERT_EQ!(ret, 0);

    ret = fork();
    ASSERT_GE!(ret, 0);

    if ret == 0 {
        chdir(c"/".as_ptr());
        unshare(CLONE_NEWNS);
        mount(c"".as_ptr(), c"/".as_ptr(), ptr::null(), MS_REC | MS_PRIVATE, ptr::null());
        umount2(c"/a".as_ptr(), MNT_DETACH);
        // This triggers a detach in the other namespace
        rmdir(c"/a".as_ptr());
        exit(0);
    }
    wait(ptr::null_mut());

    expect_notify_mask_n(_metadata, self_, FAN_MNT_DETACH as u64, 2, mnts.as_mut_ptr().add(1));
    check_mounted(_metadata, mnts.as_ptr(), 1);

    // Cleanup
    ret = chdir(c"/".as_ptr());
    ASSERT_EQ!(ret, 0);
}

unsafe fn fanotify_pivot_root(_metadata: *mut __test_metadata, self_: *mut fanotify) {
    let mut mnts: [u64; 3] = [(*self_).root_id, 0, 0];
    let mut mnts2: [u64; 3] = [0; 3];
    let mut ret: c_int;

    ret = mount(c"tmpfs".as_ptr(), c"/a".as_ptr(), c"tmpfs".as_ptr(), 0, ptr::null());
    ASSERT_EQ!(ret, 0);

    mnts[2] = expect_notify_mask(_metadata, self_, FAN_MNT_ATTACH as u64);

    ret = mkdir(c"/a/new".as_ptr(), 0o700);
    ASSERT_EQ!(ret, 0);

    ret = mkdir(c"/a/old".as_ptr(), 0o700);
    ASSERT_EQ!(ret, 0);

    ret = mount(c"/a".as_ptr(), c"/a/new".as_ptr(), ptr::null(), MS_BIND, ptr::null());
    ASSERT_EQ!(ret, 0);

    mnts[1] = expect_notify_mask(_metadata, self_, FAN_MNT_ATTACH as u64);
    check_mounted(_metadata, mnts.as_ptr(), 3);

    ret = syscall(SYS_pivot_root, c"/a/new".as_ptr(), c"/a/new/old".as_ptr()) as c_int;
    ASSERT_EQ!(ret, 0);

    expect_notify_mask_n(
        _metadata,
        self_,
        (FAN_MNT_ATTACH | FAN_MNT_DETACH) as u64,
        2,
        mnts2.as_mut_ptr(),
    );
    verify_mount_ids(_metadata, mnts.as_ptr(), mnts2.as_ptr(), 2);
    check_mounted(_metadata, mnts.as_ptr(), 3);

    // Cleanup
    ret = syscall(SYS_pivot_root, c"/old".as_ptr(), c"/old/a/new".as_ptr()) as c_int;
    ASSERT_EQ!(ret, 0);

    ret = umount(c"/a/new".as_ptr());
    ASSERT_EQ!(ret, 0);

    ret = umount(c"/a".as_ptr());
    ASSERT_EQ!(ret, 0);

    check_mounted(_metadata, mnts.as_ptr(), 1);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
