// SPDX-License-Identifier: GPL-2.0
// C dependencies removed in Rust translation:
// errno.h, fcntl.h, limits.h, sched.h, stdio.h, unistd.h, sys/fsuid.h,
// sys/stat.h, sys/syscall.h, linux/mount.h, linux/types.h,
// kselftest_harness.h, wrappers.h, utils.h

/*
 * The test mount maps caller-visible ids [0, MAP_RANGE) onto the on-disk range
 * [MAP_HOST, MAP_HOST + MAP_RANGE).  An id outside [0, MAP_RANGE) therefore has
 * no mapping in the mount and is not representable in the filesystem.
 */
const MAP_HOST: libc::uid_t = 10000;
const MAP_RANGE: libc::uid_t = 10000;
const UNMAPPED: libc::uid_t = 50000;

// Fallback from the C source's #ifndef MOUNT_ATTR_IDMAP.
const MOUNT_ATTR_IDMAP: libc::c_ulonglong = 0x00100000;

// Fallback from the C source's #ifndef __NR_mount_setattr.
const __NR_mount_setattr: libc::c_long = 442;

#[repr(C)]
struct mount_attr {
    attr_set: libc::c_ulonglong,
    attr_clr: libc::c_ulonglong,
    propagation: libc::c_ulonglong,
    userns_fd: libc::c_ulonglong,
}

unsafe extern "C" {
    fn sys_open_tree(
        dfd: libc::c_int,
        filename: *const libc::c_char,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn sys_mount(
        src: *const libc::c_char,
        target: *const libc::c_char,
        fstype: *const libc::c_char,
        flags: libc::c_ulong,
        data: *const libc::c_void,
    ) -> libc::c_int;
    fn get_userns_fd(
        map_host: libc::uid_t,
        map_ns: libc::uid_t,
        map_range: libc::uid_t,
    ) -> libc::c_int;
}

#[inline]
unsafe fn sys_mount_setattr(
    dfd: libc::c_int,
    path: *const libc::c_char,
    flags: libc::c_uint,
    attr: *mut mount_attr,
    size: libc::size_t,
) -> libc::c_int {
    unsafe { libc::syscall(__NR_mount_setattr, dfd, path, flags, attr, size) as libc::c_int }
}

/*
 * Clone @path into a detached mount idmapped so that caller-visible ids
 * [0, MAP_RANGE) map onto the on-disk ids [MAP_HOST, MAP_HOST + MAP_RANGE).
 * Returns the mount fd, or -1 if idmapped mounts are not available.
 */
unsafe fn idmapped_clone(path: *const libc::c_char) -> libc::c_int {
    let mut attr = mount_attr {
        attr_set: MOUNT_ATTR_IDMAP,
        attr_clr: 0,
        propagation: 0,
        userns_fd: 0,
    };
    let fd_tree: libc::c_int;
    let userns_fd: libc::c_int;
    let ret: libc::c_int;

    fd_tree = unsafe {
        sys_open_tree(
            libc::AT_FDCWD,
            path,
            (libc::OPEN_TREE_CLONE | libc::OPEN_TREE_CLOEXEC) as libc::c_uint,
        )
    };
    if fd_tree < 0 {
        return -1;
    }

    userns_fd = unsafe { get_userns_fd(MAP_HOST, 0, MAP_RANGE) };
    if userns_fd < 0 {
        unsafe {
            libc::close(fd_tree);
        }
        return -1;
    }

    attr.userns_fd = userns_fd as libc::c_ulonglong;
    ret = unsafe {
        sys_mount_setattr(
            fd_tree,
            c"".as_ptr(),
            libc::AT_EMPTY_PATH as libc::c_uint,
            &mut attr,
            core::mem::size_of::<mount_attr>() as libc::size_t,
        )
    };
    unsafe {
        libc::close(userns_fd);
    }
    if ret != 0 {
        unsafe {
            libc::close(fd_tree);
        }
        return -1;
    }

    fd_tree
}

#[repr(C)]
struct idmapped_tmpfile {
    dir: [libc::c_char; 64], /* non-idmapped path to the layer directory */
}

unsafe fn idmapped_tmpfile_setup(self_: *mut idmapped_tmpfile) {
    /*
     * Private mount namespace so test mounts need no cleanup.
     */
    unsafe {
        ASSERT_EQ!(libc::unshare(libc::CLONE_NEWNS), 0);
        ASSERT_EQ!(
            sys_mount(
                core::ptr::null(),
                c"/".as_ptr(),
                core::ptr::null(),
                (libc::MS_SLAVE | libc::MS_REC) as libc::c_ulong,
                core::ptr::null(),
            ),
            0,
        );
        ASSERT_EQ!(
            sys_mount(
                c"tmpfs".as_ptr(),
                c"/tmp".as_ptr(),
                c"tmpfs".as_ptr(),
                0,
                core::ptr::null(),
            ),
            0,
        );

        libc::snprintf(
            (*self_).dir.as_mut_ptr(),
            (*self_).dir.len(),
            c"/tmp/d".as_ptr(),
        );
        ASSERT_EQ!(libc::mkdir((*self_).dir.as_ptr(), 0o777), 0);
        /*
         * World-writable so an unmapped caller still passes permission().
         */
        ASSERT_EQ!(libc::chmod((*self_).dir.as_ptr(), 0o777), 0);
    }
}

unsafe fn idmapped_tmpfile_teardown(_self: *mut idmapped_tmpfile) {}

/*
 * A caller whose fsuid/fsgid have no mapping in the idmapped mount must not be
 * able to create an O_TMPFILE.  Without the check in vfs_tmpfile() the inode
 * would be created owned by (uid_t)-1 and could then be linked into the
 * namespace.
 */
unsafe fn idmapped_tmpfile_unmapped_caller_is_refused(self_: *mut idmapped_tmpfile) {
    let mfd: libc::c_int;
    let fd: libc::c_int;

    mfd = unsafe { idmapped_clone((*self_).dir.as_ptr()) };
    if mfd < 0 {
        SKIP!(return, "idmapped mounts not supported");
    }

    /*
     * Become a caller outside the mount's [0, MAP_RANGE) range.
     */
    unsafe {
        libc::setfsgid(UNMAPPED);
        libc::setfsuid(UNMAPPED);
        ASSERT_EQ!(libc::setfsuid(-1i32 as libc::uid_t), UNMAPPED);
    }

    fd = unsafe { libc::openat(mfd, c".".as_ptr(), libc::O_TMPFILE | libc::O_WRONLY, 0o644) };
    ASSERT_LT!(fd, 0);
    unsafe {
        EXPECT_EQ!(*libc::__errno_location(), libc::EOVERFLOW);
    }
    if fd >= 0 {
        unsafe {
            libc::close(fd);
        }
    }

    unsafe {
        EXPECT_EQ!(libc::close(mfd), 0);
    }
}

/*
 * A mapped caller can create an O_TMPFILE and link it into the namespace; the
 * ownership round-trips through the mount idmap.  This is what makes refusing
 * the unmapped case above necessary in the first place.
 */
unsafe fn idmapped_tmpfile_mapped_caller_creates_and_links(self_: *mut idmapped_tmpfile) {
    let mut path: [libc::c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];
    let mut st: libc::stat = unsafe { core::mem::zeroed() };
    let mfd: libc::c_int;
    let fd: libc::c_int;

    mfd = unsafe { idmapped_clone((*self_).dir.as_ptr()) };
    if mfd < 0 {
        SKIP!(return, "idmapped mounts not supported");
    }

    /*
     * Caller is uid/gid 0, which maps to MAP_HOST through the mount.
     */
    fd = unsafe { libc::openat(mfd, c".".as_ptr(), libc::O_TMPFILE | libc::O_RDWR, 0o600) };
    ASSERT_GE!(fd, 0);

    unsafe {
        ASSERT_EQ!(libc::fstat(fd, &mut st), 0);
    }
    EXPECT_EQ!(st.st_uid, 0);
    EXPECT_EQ!(st.st_gid, 0);

    /*
     * The tmpfile is linkable: splice it into the directory.
     */
    unsafe {
        ASSERT_EQ!(
            libc::linkat(fd, c"".as_ptr(), mfd, c"linked".as_ptr(), libc::AT_EMPTY_PATH),
            0,
        );
        EXPECT_EQ!(libc::close(fd), 0);
    }

    unsafe {
        ASSERT_EQ!(libc::fstatat(mfd, c"linked".as_ptr(), &mut st, 0), 0);
    }
    EXPECT_EQ!(st.st_uid, 0);
    EXPECT_EQ!(st.st_gid, 0);

    /*
     * On the underlying, non-idmapped tmpfs it is stored as MAP_HOST.
     */
    unsafe {
        libc::snprintf(
            path.as_mut_ptr(),
            path.len(),
            c"%s/linked".as_ptr(),
            (*self_).dir.as_ptr(),
        );
        ASSERT_EQ!(libc::stat(path.as_ptr(), &mut st), 0);
    }
    EXPECT_EQ!(st.st_uid, MAP_HOST);
    EXPECT_EQ!(st.st_gid, MAP_HOST);

    unsafe {
        EXPECT_EQ!(libc::close(mfd), 0);
    }
}

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
