// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2025 Miklos Szeredi <miklos@szeredi.hu>

// Translated from C. Original includes supplied Linux, libc, kselftest, statmount,
// and utility declarations.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct __kernel_fsid_t {
    pub val: [c_int; 2],
}

#[repr(C)]
pub struct fanotify {
    pub fan_fd: [c_int; NUM_FAN_FDS],
    pub buf: [c_char; 256],
    pub rem: c_uint,
    pub next: *mut c_void,
    pub root_mntpoint: [c_char; ROOT_MNTPOINT_TEMPL_LEN],
    pub orig_root: c_int,
    pub ns_fd: c_int,
    pub root_id: u64,
}

#[repr(C)]
pub struct fanotify_event_metadata {
    pub event_len: u32,
    pub vers: u8,
    pub reserved: u8,
    pub metadata_len: u16,
    pub mask: u64,
    pub fd: i32,
    pub pid: i32,
}

#[repr(C)]
pub struct fanotify_event_info_mnt {
    pub hdr: fanotify_event_info_header,
    pub mnt_id: u64,
}

#[repr(C)]
pub struct fanotify_event_info_header {
    pub info_type: u8,
    pub pad: u8,
    pub len: u16,
}

const ROOT_MNTPOINT_TEMPL: &[u8; ROOT_MNTPOINT_TEMPL_LEN] =
    b"/tmp/mount-notify_test_root.XXXXXX\0";
const ROOT_MNTPOINT_TEMPL_LEN: usize = 37;

static MARK_CMDS: [c_uint; NUM_FAN_FDS] = [
    FAN_MARK_ADD,
    FAN_MARK_REMOVE,
    FAN_MARK_FLUSH,
];

const NUM_FAN_FDS: usize = 3;

unsafe extern "C" {
    fn unshare(flags: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn chroot(path: *const c_char) -> c_int;
    fn chdir(path: *const c_char) -> c_int;
    fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
    fn get_unique_mnt_id(path: *const c_char) -> u64;
    fn fanotify_init(flags: c_uint, event_f_flags: c_uint) -> c_int;
    fn fanotify_mark(
        fanotify_fd: c_int,
        flags: c_uint,
        mask: u64,
        dirfd: c_int,
        pathname: *const c_char,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fchdir(fd: c_int) -> c_int;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn __errno_location() -> *mut c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn listmount(
        mnt_id: u64,
        param: u64,
        flags: u64,
        list: *mut u64,
        num: usize,
        spare: c_uint,
    ) -> isize;
    fn free(ptr: *mut c_void);
    fn umount(target: *const c_char) -> c_int;
    fn move_mount(
        from_dfd: c_int,
        from_pathname: *const c_char,
        to_dfd: c_int,
        to_pathname: *const c_char,
        flags: c_uint,
    ) -> c_int;
    fn fsopen(fs_name: *const c_char, flags: c_uint) -> c_int;
    fn fsconfig(
        fd: c_int,
        cmd: c_uint,
        key: *const c_char,
        value: *const c_char,
        aux: c_int,
    ) -> c_int;
    fn fsmount(fd: c_int, flags: c_uint, mount_attrs: c_uint) -> c_int;
    fn fork() -> c_int;
    fn wait(wstatus: *mut c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn syscall(num: c_long, ...) -> c_long;
}

type c_long = isize;

unsafe fn fanotify_setup(_metadata: *mut __test_metadata, self_: *mut fanotify) {
    let mut i: c_int;
    let mut ret: c_int;

    ASSERT_EQ!(unshare(CLONE_NEWNS), 0);

    (*self_).ns_fd = open(c"/proc/self/ns/mnt".as_ptr(), O_RDONLY);
    ASSERT_GE!((*self_).ns_fd, 0);

    ASSERT_EQ!(
        mount(c"".as_ptr(), c"/".as_ptr(), ptr::null(), MS_REC | MS_PRIVATE, ptr::null()),
        0
    );

    strcpy((*self_).root_mntpoint.as_mut_ptr(), ROOT_MNTPOINT_TEMPL.as_ptr().cast());
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
    while i < NUM_FAN_FDS as c_int {
        (*self_).fan_fd[i as usize] = fanotify_init(FAN_REPORT_MNT | FAN_NONBLOCK, 0);
        ASSERT_GE!((*self_).fan_fd[i as usize], 0);
        ret = fanotify_mark(
            (*self_).fan_fd[i as usize],
            FAN_MARK_ADD | FAN_MARK_MNTNS,
            FAN_MNT_ATTACH | FAN_MNT_DETACH,
            (*self_).ns_fd,
            ptr::null(),
        );
        ASSERT_EQ!(ret, 0);
        // On fd[0] we do an extra ADD that changes nothing.
        // On fd[1]/fd[2] we REMOVE/FLUSH which removes the mark.
        ret = fanotify_mark(
            (*self_).fan_fd[i as usize],
            MARK_CMDS[i as usize] | FAN_MARK_MNTNS,
            FAN_MNT_ATTACH | FAN_MNT_DETACH,
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
    while i < NUM_FAN_FDS as c_int {
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
    let mnt: *mut fanotify_event_info_mnt;
    let mut thislen: c_uint;

    if (*self_).rem == 0 {
        let len: isize;
        let mut i: c_int;
        let mut last_len: isize = 0;

        i = NUM_FAN_FDS as c_int - 1;
        while i >= 0 {
            last_len = read(
                (*self_).fan_fd[i as usize],
                (*self_).buf.as_mut_ptr().cast(),
                size_of_val(&(*self_).buf),
            );
            if i > 0 {
                // Groups 1,2 should get EAGAIN
                ASSERT_EQ!(last_len, -1);
                ASSERT_EQ!(*__errno_location(), EAGAIN);
            } else {
                // Group 0 should get events
                ASSERT_GT!(last_len, 0);
            }
            i -= 1;
        }

        (*self_).rem = last_len as c_uint;
        (*self_).next = (*self_).buf.as_mut_ptr().cast();
    }

    meta = (*self_).next.cast();
    ASSERT_TRUE!(FAN_EVENT_OK(meta, (*self_).rem));

    thislen = (*meta).event_len;
    (*self_).rem -= thislen;
    (*self_).next = (*self_).next.cast::<u8>().add(thislen as usize).cast();

    *mask = (*meta).mask;
    thislen -= size_of::<fanotify_event_metadata>() as c_uint;

    mnt = (meta.cast::<u8>())
        .add((*meta).event_len as usize - thislen as usize)
        .cast();

    ASSERT_EQ!(thislen as usize, size_of::<fanotify_event_info_mnt>());

    (*mnt).mnt_id
}

unsafe fn expect_notify_n(
    _metadata: *mut __test_metadata,
    self_: *mut fanotify,
    n: c_uint,
    mask: *mut u64,
    mnts: *mut u64,
) {
    let mut i: c_uint;

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
    n: c_uint,
    mnts: *mut u64,
) {
    let mut i: c_uint;

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
    let mut i: c_uint;
    let mut j: c_uint;

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

unsafe fn check_mounted(_metadata: *mut __test_metadata, mnts: *const u64, num: usize) {
    let ret: isize;
    let list: *mut u64;

    list = malloc((num + 1) * size_of::<u64>()).cast();
    ASSERT_NE!(list, ptr::null_mut());

    ret = listmount(LSMT_ROOT, 0, 0, list, num + 1, 0);
    ASSERT_EQ!(ret, num as isize);

    verify_mount_ids(_metadata, mnts, list, num);

    free(list.cast());
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

    mnts[1] = expect_notify_mask(_metadata, self_, FAN_MNT_ATTACH);
    ASSERT_NE!(mnts[0], mnts[1]);

    check_mounted(_metadata, mnts.as_ptr(), 2);

    // Cleanup
    let detach_id: u64;
    ret = umount(c"/".as_ptr());
    ASSERT_EQ!(ret, 0);

    detach_id = expect_notify_mask(_metadata, self_, FAN_MNT_DETACH);
    ASSERT_EQ!(detach_id, mnts[1]);

    check_mounted(_metadata, mnts.as_ptr(), 1);
}

unsafe fn fanotify_move(_metadata: *mut __test_metadata, self_: *mut fanotify) {
    let mut ret: c_int;
    let mut mnts: [u64; 2] = [(*self_).root_id, 0];
    let move_id: u64;

    ret = mount(c"/".as_ptr(), c"/a".as_ptr(), ptr::null(), MS_BIND, ptr::null());
    ASSERT_EQ!(ret, 0);

    mnts[1] = expect_notify_mask(_metadata, self_, FAN_MNT_ATTACH);
    ASSERT_NE!(mnts[0], mnts[1]);

    check_mounted(_metadata, mnts.as_ptr(), 2);

    ret = move_mount(AT_FDCWD, c"/a".as_ptr(), AT_FDCWD, c"/b".as_ptr(), 0);
    ASSERT_EQ!(ret, 0);

    move_id = expect_notify_mask(_metadata, self_, FAN_MNT_ATTACH | FAN_MNT_DETACH);
    ASSERT_EQ!(move_id, mnts[1]);

    // Cleanup
    ret = umount(c"/b".as_ptr());
    ASSERT_EQ!(ret, 0);

    check_mounted(_metadata, mnts.as_ptr(), 1);
}

unsafe fn fanotify_propagate(_metadata: *mut __test_metadata, self_: *mut fanotify) {
    const LOG2_NUM: c_uint = 4;
    const NUM: usize = 1 << LOG2_NUM;
    let mut mnts: [u64; NUM] = [0; NUM];

    setup_mount_tree(_metadata, LOG2_NUM as c_int);

    expect_notify_mask_n(
        _metadata,
        self_,
        FAN_MNT_ATTACH,
        NUM as c_uint - 1,
        mnts.as_mut_ptr().add(1),
    );

    mnts[0] = (*self_).root_id;
    check_mounted(_metadata, mnts.as_ptr(), NUM);

    // Cleanup
    let mut ret: c_int;
    let mut mnts2: [u64; NUM] = [0; NUM];
    ret = umount2(c"/".as_ptr(), MNT_DETACH);
    ASSERT_EQ!(ret, 0);

    ret = mount(c"".as_ptr(), c"/".as_ptr(), ptr::null(), MS_PRIVATE, ptr::null());
    ASSERT_EQ!(ret, 0);

    mnts2[0] = (*self_).root_id;
    expect_notify_mask_n(
        _metadata,
        self_,
        FAN_MNT_DETACH,
        NUM as c_uint - 1,
        mnts2.as_mut_ptr().add(1),
    );
    verify_mount_ids(_metadata, mnts.as_ptr(), mnts2.as_ptr(), NUM);

    check_mounted(_metadata, mnts.as_ptr(), 1);
}

unsafe fn fanotify_fsmount(_metadata: *mut __test_metadata, self_: *mut fanotify) {
    let mut ret: c_int;
    let fs: c_int;
    let mnt: c_int;
    let mut mnts: [u64; 2] = [(*self_).root_id, 0];

    fs = fsopen(c"tmpfs".as_ptr(), 0);
    ASSERT_GE!(fs, 0);

    ret = fsconfig(fs, FSCONFIG_CMD_CREATE, ptr::null(), ptr::null(), 0);
    ASSERT_EQ!(ret, 0);

    mnt = fsmount(fs, 0, 0);
    ASSERT_GE!(mnt, 0);

    close(fs);

    ret = move_mount(mnt, c"".as_ptr(), AT_FDCWD, c"/a".as_ptr(), MOVE_MOUNT_F_EMPTY_PATH);
    ASSERT_EQ!(ret, 0);

    close(mnt);

    mnts[1] = expect_notify_mask(_metadata, self_, FAN_MNT_ATTACH);
    ASSERT_NE!(mnts[0], mnts[1]);

    check_mounted(_metadata, mnts.as_ptr(), 2);

    // Cleanup
    let detach_id: u64;
    ret = umount(c"/a".as_ptr());
    ASSERT_EQ!(ret, 0);

    detach_id = expect_notify_mask(_metadata, self_, FAN_MNT_DETACH);
    ASSERT_EQ!(detach_id, mnts[1]);

    check_mounted(_metadata, mnts.as_ptr(), 1);
}

unsafe fn fanotify_reparent(_metadata: *mut __test_metadata, self_: *mut fanotify) {
    let mut mnts: [u64; 6] = [(*self_).root_id, 0, 0, 0, 0, 0];
    let mut dmnts: [u64; 3] = [0; 3];
    let mut masks: [u64; 3] = [0; 3];
    let mut i: c_uint;
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

    expect_notify_mask_n(_metadata, self_, FAN_MNT_ATTACH, 2, mnts.as_mut_ptr().add(1));

    check_mounted(_metadata, mnts.as_ptr(), 3);

    // Mount on a[3], which is propagated to b[4]
    ret = mount(c"/".as_ptr(), c"/a".as_ptr(), ptr::null(), MS_BIND, ptr::null());
    ASSERT_EQ!(ret, 0);

    expect_notify_mask_n(_metadata, self_, FAN_MNT_ATTACH, 2, mnts.as_mut_ptr().add(3));

    check_mounted(_metadata, mnts.as_ptr(), 5);

    // Mount on b[5], not propagated
    ret = mount(c"/".as_ptr(), c"/b".as_ptr(), ptr::null(), MS_BIND, ptr::null());
    ASSERT_EQ!(ret, 0);

    mnts[5] = expect_notify_mask(_metadata, self_, FAN_MNT_ATTACH);

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
            ASSERT_EQ!(masks[i as usize], FAN_MNT_ATTACH | FAN_MNT_DETACH);
        } else {
            ASSERT_EQ!(masks[i as usize], FAN_MNT_DETACH);
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

    expect_notify_mask_n(_metadata, self_, FAN_MNT_DETACH, 3, dmnts.as_mut_ptr());
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

    expect_notify_mask_n(_metadata, self_, FAN_MNT_ATTACH, 2, mnts.as_mut_ptr().add(1));

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

    expect_notify_mask_n(_metadata, self_, FAN_MNT_DETACH, 2, mnts.as_mut_ptr().add(1));
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

    mnts[2] = expect_notify_mask(_metadata, self_, FAN_MNT_ATTACH);

    ret = mkdir(c"/a/new".as_ptr(), 0o700);
    ASSERT_EQ!(ret, 0);

    ret = mkdir(c"/a/old".as_ptr(), 0o700);
    ASSERT_EQ!(ret, 0);

    ret = mount(c"/a".as_ptr(), c"/a/new".as_ptr(), ptr::null(), MS_BIND, ptr::null());
    ASSERT_EQ!(ret, 0);

    mnts[1] = expect_notify_mask(_metadata, self_, FAN_MNT_ATTACH);
    check_mounted(_metadata, mnts.as_ptr(), 3);

    ret = syscall(SYS_pivot_root, c"/a/new".as_ptr(), c"/a/new/old".as_ptr()) as c_int;
    ASSERT_EQ!(ret, 0);

    expect_notify_mask_n(
        _metadata,
        self_,
        FAN_MNT_ATTACH | FAN_MNT_DETACH,
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

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
