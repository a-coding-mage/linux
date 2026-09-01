// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and included Linux/selftest headers.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type bool_t = bool;
type pid_t = c_int;
type uid_t = c_uint;
type gid_t = c_uint;
type mode_t = c_uint;
type dev_t = c_ulong;

const PATH_MAX: usize = 4096;
const MAX_HANDLE_SZ: usize = 128;

const AT_FDCWD: c_int = -100;
const AT_EMPTY_PATH: c_int = 0x1000;
const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
const O_CREAT: c_int = 0o100;
const O_EXCL: c_int = 0o200;
const O_WRONLY: c_int = 0o1;
const O_RDONLY: c_int = 0;
const O_DIRECTORY: c_int = 0o200000;
const O_TMPFILE: c_int = 0o20200000;
const S_IFREG: mode_t = 0o100000;
const S_IFIFO: mode_t = 0o010000;
const CLONE_NEWNS: c_int = 0x00020000;
const MS_SLAVE: c_ulong = 1 << 19;
const MS_REC: c_ulong = 16384;
const OPEN_TREE_CLONE: c_uint = 1;
const OPEN_TREE_CLOEXEC: c_uint = 0o2000000;
const MOVE_MOUNT_F_EMPTY_PATH: c_uint = 0x00000004;
const FSCONFIG_SET_STRING: c_uint = 1;
const FSCONFIG_CMD_CREATE: c_uint = 6;
const FSCONFIG_SET_FD: c_uint = 5;
const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;

/*
 * An idmapping that maps the mount-visible id range [0, ID_RANGE) onto the
 * host/overlay-final id range [ID_HOST, ID_HOST + ID_RANGE).  Through such an
 * idmapped overlay mount, an overlay-final id of ID_HOST + n is reported as n,
 * and an id of n requested through the mount is stored as ID_HOST + n.
 */
const ID_NS: c_int = 0;
const ID_HOST: c_int = 10000;
const ID_RANGE: c_int = 10000;

/*
 * For the composition test the lower layer's on-disk ids live in a
 * separate range and are mapped by an idmapped lower layer onto the
 * overlay-final range [ID_HOST, ID_HOST + ID_RANGE).
 */
const LAYER_HOST: c_int = 20000;

// Fallback definitions from the C preprocessor when system headers lack them.
const MOUNT_ATTR_IDMAP: u64 = 0x00100000;
const __NR_mount_setattr: c_long = 442;

#[repr(C)]
struct mount_attr {
    attr_set: u64,
    attr_clr: u64,
    propagation: u64,
    userns_fd: u64,
}

#[repr(C)]
struct stat {
    st_dev: c_ulong,
    st_ino: c_ulong,
    st_nlink: c_ulong,
    st_mode: c_uint,
    st_uid: c_uint,
    st_gid: c_uint,
    __pad0: c_int,
    st_rdev: c_ulong,
    st_size: c_long,
    st_blksize: c_long,
    st_blocks: c_long,
    __glibc_reserved: [c_long; 6],
}

#[repr(C)]
struct file_handle {
    handle_bytes: c_uint,
    handle_type: c_int,
    f_handle: [u8; 0],
}

#[repr(C)]
union file_handle_storage {
    fh: core::mem::ManuallyDrop<file_handle>,
    buf: [c_char; size_of::<file_handle>() + MAX_HANDLE_SZ],
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut stderr: *mut c_void;

    fn syscall(num: c_long, ...) -> c_long;
    fn close(fd: c_int) -> c_int;
    fn mkdir(path: *const c_char, mode: mode_t) -> c_int;
    fn chown(path: *const c_char, owner: uid_t, group: gid_t) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn mknod(path: *const c_char, mode: mode_t, dev: dev_t) -> c_int;
    fn fstatat(dirfd: c_int, pathname: *const c_char, statbuf: *mut stat, flags: c_int) -> c_int;
    fn lstat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn mkdirat(dirfd: c_int, pathname: *const c_char, mode: mode_t) -> c_int;
    fn mknodat(dirfd: c_int, pathname: *const c_char, mode: mode_t, dev: dev_t) -> c_int;
    fn symlinkat(target: *const c_char, newdirfd: c_int, linkpath: *const c_char) -> c_int;
    fn fchownat(dirfd: c_int, pathname: *const c_char, owner: uid_t, group: gid_t, flags: c_int) -> c_int;
    fn linkat(olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char, flags: c_int) -> c_int;
    fn fork() -> pid_t;
    fn _exit(status: c_int) -> !;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn name_to_handle_at(dirfd: c_int, pathname: *const c_char, handle: *mut file_handle, mount_id: *mut c_int, flags: c_int) -> c_int;
    fn open_by_handle_at(mount_fd: c_int, handle: *mut file_handle, flags: c_int) -> c_int;

    fn sys_fsopen(fsname: *const c_char, flags: c_uint) -> c_int;
    fn sys_fsconfig(fd: c_int, cmd: c_uint, key: *const c_char, value: *const c_char, aux: c_int) -> c_int;
    fn sys_fsmount(fsfd: c_int, flags: c_uint, attr_flags: c_uint) -> c_int;
    fn sys_open_tree(dfd: c_int, filename: *const c_char, flags: c_uint) -> c_int;
    fn sys_mount(source: *const c_char, target: *const c_char, filesystemtype: *const c_char, mountflags: c_ulong, data: *const c_void) -> c_int;
    fn sys_move_mount(from_dfd: c_int, from_pathname: *const c_char, to_dfd: c_int, to_pathname: *const c_char, flags: c_uint) -> c_int;
    fn get_userns_fd(nsid: c_int, hostid: c_int, range: c_int) -> c_int;
    fn switch_userns(fd: c_int, uid: uid_t, gid: gid_t, drop_caps: bool_t) -> bool_t;
    fn wait_for_pid(pid: pid_t) -> c_int;
}

unsafe fn sys_mount_setattr(
    dfd: c_int,
    path: *const c_char,
    flags: c_uint,
    attr: *mut mount_attr,
    size: size_t,
) -> c_int {
    syscall(__NR_mount_setattr, dfd, path, flags, attr, size) as c_int
}

unsafe fn ovl_supported() -> bool {
    let fd = sys_fsopen(c"overlay".as_ptr(), 0);

    if fd < 0 {
        return false;
    }
    close(fd);
    true
}

/* base/{l,u,w} owned by ID_HOST so they map to ID_NS through the idmap. */
unsafe fn setup_layers(base: *const c_char) -> c_int {
    static sub: [*const c_char; 4] = [
        c"".as_ptr(),
        c"/l".as_ptr(),
        c"/u".as_ptr(),
        c"/w".as_ptr(),
    ];
    let mut path = [0 as c_char; PATH_MAX];

    for i in 0..sub.len() {
        snprintf(path.as_mut_ptr(), path.len(), c"%s%s".as_ptr(), base, sub[i]);
        if mkdir(path.as_ptr(), 0o755) != 0 && errno != 17 {
            return -1;
        }
        if i != 0 && chown(path.as_ptr(), ID_HOST as uid_t, ID_HOST as gid_t) != 0 {
            return -1;
        }
    }
    0
}

unsafe fn ovl_mount(base: *const c_char, nfs_export: bool) -> c_int {
    let mut lower = [0 as c_char; PATH_MAX];
    let mut upper = [0 as c_char; PATH_MAX];
    let mut work = [0 as c_char; PATH_MAX];
    let fsfd: c_int;
    let ovl: c_int;

    snprintf(lower.as_mut_ptr(), lower.len(), c"%s/l".as_ptr(), base);
    snprintf(upper.as_mut_ptr(), upper.len(), c"%s/u".as_ptr(), base);
    snprintf(work.as_mut_ptr(), work.len(), c"%s/w".as_ptr(), base);

    fsfd = sys_fsopen(c"overlay".as_ptr(), 0);
    if fsfd < 0 {
        return -1;
    }

    if sys_fsconfig(fsfd, FSCONFIG_SET_STRING, c"source".as_ptr(), c"test".as_ptr(), 0) != 0
        || sys_fsconfig(fsfd, FSCONFIG_SET_STRING, c"lowerdir".as_ptr(), lower.as_ptr(), 0) != 0
        || sys_fsconfig(fsfd, FSCONFIG_SET_STRING, c"upperdir".as_ptr(), upper.as_ptr(), 0) != 0
        || sys_fsconfig(fsfd, FSCONFIG_SET_STRING, c"workdir".as_ptr(), work.as_ptr(), 0) != 0
    {
        close(fsfd);
        return -1;
    }
    if nfs_export
        && (sys_fsconfig(fsfd, FSCONFIG_SET_STRING, c"index".as_ptr(), c"on".as_ptr(), 0) != 0
            || sys_fsconfig(fsfd, FSCONFIG_SET_STRING, c"nfs_export".as_ptr(), c"on".as_ptr(), 0) != 0)
    {
        close(fsfd);
        return -1;
    }
    if sys_fsconfig(fsfd, FSCONFIG_CMD_CREATE, ptr::null(), ptr::null(), 0) != 0 {
        close(fsfd);
        return -1;
    }

    ovl = sys_fsmount(fsfd, 0, 0);
    close(fsfd);
    ovl
}

/* Idmap the (still detached, not yet visible) overlay mount @mfd. */
unsafe fn ovl_idmap(mfd: c_int) -> c_int {
    let mut attr = mount_attr {
        attr_set: MOUNT_ATTR_IDMAP,
        attr_clr: 0,
        propagation: 0,
        userns_fd: 0,
    };
    let ret: c_int;
    let userns_fd: c_int;

    /*
     * get_userns_fd(fs_id, mount_id, range): a file whose filesystem id
     * is fs_id + n is shown through the idmapped mount as mount_id + n.
     * Here the overlay-final (fs side) range is [ID_HOST, ..) and the
     * caller-visible (mount side) range is [ID_NS, ..).
     */
    userns_fd = get_userns_fd(ID_HOST, ID_NS, ID_RANGE);
    if userns_fd < 0 {
        return -1;
    }

    attr.userns_fd = userns_fd as u64;
    ret = sys_mount_setattr(mfd, c"".as_ptr(), AT_EMPTY_PATH as c_uint, &mut attr, size_of::<mount_attr>());
    close(userns_fd);
    ret
}

/* Clone @path into a detached, idmapped mount usable as an overlay layer. */
unsafe fn idmapped_layer_fd(path: *const c_char, nsid: c_int, hostid: c_int, range: c_int) -> c_int {
    let mut attr = mount_attr {
        attr_set: MOUNT_ATTR_IDMAP,
        attr_clr: 0,
        propagation: 0,
        userns_fd: 0,
    };
    let fd_tree: c_int;
    let userns_fd: c_int;

    fd_tree = sys_open_tree(AT_FDCWD, path, OPEN_TREE_CLONE | OPEN_TREE_CLOEXEC);
    if fd_tree < 0 {
        return -1;
    }
    userns_fd = get_userns_fd(nsid, hostid, range);
    if userns_fd < 0 {
        close(fd_tree);
        return -1;
    }
    attr.userns_fd = userns_fd as u64;
    if sys_mount_setattr(fd_tree, c"".as_ptr(), AT_EMPTY_PATH as c_uint, &mut attr, size_of::<mount_attr>()) != 0 {
        close(userns_fd);
        close(fd_tree);
        return -1;
    }
    close(userns_fd);
    fd_tree
}

/* Overlay with a layer passed by fd (idmapped) plus a plain upper/work. */
unsafe fn ovl_mount_lower_fd(upper: *const c_char, work: *const c_char, fd_lower: c_int) -> c_int {
    let fsfd: c_int;
    let ovl: c_int;

    fsfd = sys_fsopen(c"overlay".as_ptr(), 0);
    if fsfd < 0 {
        return -1;
    }

    if sys_fsconfig(fsfd, FSCONFIG_SET_STRING, c"source".as_ptr(), c"test".as_ptr(), 0) != 0
        || sys_fsconfig(fsfd, FSCONFIG_SET_STRING, c"upperdir".as_ptr(), upper, 0) != 0
        || sys_fsconfig(fsfd, FSCONFIG_SET_STRING, c"workdir".as_ptr(), work, 0) != 0
        || sys_fsconfig(fsfd, FSCONFIG_SET_FD, c"lowerdir+".as_ptr(), ptr::null(), fd_lower) != 0
        || sys_fsconfig(fsfd, FSCONFIG_CMD_CREATE, ptr::null(), ptr::null(), 0) != 0
    {
        close(fsfd);
        return -1;
    }

    ovl = sys_fsmount(fsfd, 0, 0);
    close(fsfd);
    ovl
}

/*
 * Mount an overlay inside user namespace @u1 (so the overlay sb's s_user_ns is
 * not the initial namespace) and idmap that overlay mount with @u2.  Runs in a
 * child that joins @u1; returns 0 on success.
 */
unsafe fn userns_overlay_child(u1: c_int) -> c_int {
    let mut attr = mount_attr {
        attr_set: MOUNT_ATTR_IDMAP,
        attr_clr: 0,
        propagation: 0,
        userns_fd: 0,
    };
    let mut st: stat = core::mem::zeroed();
    let ovl: c_int;
    let u2: c_int;

    /* Become root in the overlay sb's user namespace u1. */
    if !switch_userns(u1, 0, 0, false) {
        fprintf(stderr, c"userns: switch_userns: %m\n".as_ptr());
        return -1;
    }
    if unshare(CLONE_NEWNS) != 0
        || sys_mount(ptr::null(), c"/".as_ptr(), ptr::null(), MS_SLAVE | MS_REC, ptr::null()) != 0
    {
        fprintf(stderr, c"userns: unshare/slave: %m\n".as_ptr());
        return -1;
    }
    if sys_mount(c"tmpfs".as_ptr(), c"/tmp".as_ptr(), c"tmpfs".as_ptr(), 0, ptr::null()) != 0 {
        fprintf(stderr, c"userns: mount tmpfs: %m\n".as_ptr());
        return -1;
    }
    if setup_layers(c"/tmp/ovl".as_ptr()) != 0 {
        fprintf(stderr, c"userns: setup_layers: %m\n".as_ptr());
        return -1;
    }
    if mknod(c"/tmp/ovl/l/file".as_ptr(), S_IFREG | 0o644, 0) != 0
        || chown(c"/tmp/ovl/l/file".as_ptr(), (ID_HOST + 5) as uid_t, (ID_HOST + 5) as gid_t) != 0
    {
        fprintf(stderr, c"userns: lower file: %m\n".as_ptr());
        return -1;
    }

    ovl = ovl_mount(c"/tmp/ovl".as_ptr(), false);
    if ovl < 0 {
        fprintf(stderr, c"userns: ovl_mount: %m\n".as_ptr());
        return -1;
    }

    /*
     * mount_setattr() requires CAP_SYS_ADMIN over the idmap user
     * namespace, so it must be a child of u1.  Create it now, from
     * inside u1.
     */
    u2 = get_userns_fd(ID_HOST, ID_NS, ID_RANGE);
    if u2 < 0 {
        fprintf(stderr, c"userns: get_userns_fd: %m\n".as_ptr());
        return -1;
    }
    attr.userns_fd = u2 as u64;
    if sys_mount_setattr(ovl, c"".as_ptr(), AT_EMPTY_PATH as c_uint, &mut attr, size_of::<mount_attr>()) != 0 {
        fprintf(stderr, c"userns: mount_setattr: %m\n".as_ptr());
        return -1;
    }
    close(u2);

    if fstatat(ovl, c"file".as_ptr(), &mut st, 0) != 0 {
        fprintf(stderr, c"userns: fstatat: %m\n".as_ptr());
        return -1;
    }
    if st.st_uid != (ID_NS + 5) as c_uint || st.st_gid != (ID_NS + 5) as c_uint {
        fprintf(
            stderr,
            c"userns: got %u:%u expected %u:%u\n".as_ptr(),
            st.st_uid,
            st.st_gid,
            ID_NS + 5,
            ID_NS + 5,
        );
        return -1;
    }
    0
}

struct idmapped_overlay {
    base: [c_char; 64],
}

unsafe fn idmapped_overlay_setup(self_: *mut idmapped_overlay) {
    /* Private mount namespace so test mounts need no cleanup. */
    ASSERT_EQ!(unshare(CLONE_NEWNS), 0);
    ASSERT_EQ!(sys_mount(ptr::null(), c"/".as_ptr(), ptr::null(), MS_SLAVE | MS_REC, ptr::null()), 0);

    /* tmpfs for the layers so we can chown them to arbitrary ids. */
    ASSERT_EQ!(sys_mount(c"tmpfs".as_ptr(), c"/tmp".as_ptr(), c"tmpfs".as_ptr(), 0, ptr::null()), 0);

    snprintf((*self_).base.as_mut_ptr(), (*self_).base.len(), c"/tmp/ovl".as_ptr());
    ASSERT_EQ!(setup_layers((*self_).base.as_ptr()), 0);
}

unsafe fn idmapped_overlay_teardown(_self_: *mut idmapped_overlay) {}

/* A file owned by ID_HOST + 5 is reported as ID_NS + 5 through the idmap. */
unsafe fn idmapped_overlay_getattr(self_: *mut idmapped_overlay) {
    let mut path = [0 as c_char; PATH_MAX];
    let mut st: stat = core::mem::zeroed();
    let ovl: c_int;

    if !ovl_supported() {
        SKIP!(return, "overlayfs not supported");
    }

    snprintf(path.as_mut_ptr(), path.len(), c"%s/l/file".as_ptr(), (*self_).base.as_ptr());
    ASSERT_EQ!(mknod(path.as_ptr(), S_IFREG | 0o644, 0), 0);
    ASSERT_EQ!(chown(path.as_ptr(), (ID_HOST + 5) as uid_t, (ID_HOST + 5) as gid_t), 0);

    ovl = ovl_mount((*self_).base.as_ptr(), false);
    ASSERT_GE!(ovl, 0);
    ASSERT_EQ!(ovl_idmap(ovl), 0);

    ASSERT_EQ!(fstatat(ovl, c"file".as_ptr(), &mut st, 0), 0);
    EXPECT_EQ!(st.st_uid, (ID_NS + 5) as c_uint);
    EXPECT_EQ!(st.st_gid, (ID_NS + 5) as c_uint);

    EXPECT_EQ!(close(ovl), 0);
}

/*
 * Every creation path initializes the new owner through the mount idmap:
 * created as caller id ID_NS, stored on the upper layer as overlay-final
 * ID_HOST.  Covers ovl_create() (regular file), ovl_mkdir(), ovl_mknod()
 * and ovl_symlink() (which share ovl_create_object()), plus the separate
 * ovl_tmpfile() path.
 */
unsafe fn idmapped_overlay_create(self_: *mut idmapped_overlay) {
    static names: [*const c_char; 4] = [
        c"reg".as_ptr(),
        c"dir".as_ptr(),
        c"fifo".as_ptr(),
        c"lnk".as_ptr(),
    ];
    let mut path = [0 as c_char; PATH_MAX];
    let mut st: stat = core::mem::zeroed();
    let ovl: c_int;
    let mut fd: c_int;

    if !ovl_supported() {
        SKIP!(return, "overlayfs not supported");
    }

    ovl = ovl_mount((*self_).base.as_ptr(), false);
    ASSERT_GE!(ovl, 0);
    ASSERT_EQ!(ovl_idmap(ovl), 0);

    /* One object per creation operation, all as caller id ID_NS. */
    fd = openat(ovl, c"reg".as_ptr(), O_CREAT | O_WRONLY | O_EXCL, 0o644);
    ASSERT_GE!(fd, 0);
    EXPECT_EQ!(close(fd), 0);
    ASSERT_EQ!(mkdirat(ovl, c"dir".as_ptr(), 0o755), 0);
    ASSERT_EQ!(mknodat(ovl, c"fifo".as_ptr(), S_IFIFO | 0o644, 0), 0);
    ASSERT_EQ!(symlinkat(c"target".as_ptr(), ovl, c"lnk".as_ptr()), 0);

    for i in 0..names.len() {
        /* Reported as ID_NS through the idmapped mount ... */
        ASSERT_EQ!(fstatat(ovl, names[i], &mut st, AT_SYMLINK_NOFOLLOW), 0);
        EXPECT_EQ!(st.st_uid, ID_NS as c_uint);
        EXPECT_EQ!(st.st_gid, ID_NS as c_uint);
        /* ... and stored as ID_HOST on the upper layer. */
        snprintf(path.as_mut_ptr(), path.len(), c"%s/u/%s".as_ptr(), (*self_).base.as_ptr(), names[i]);
        ASSERT_EQ!(lstat(path.as_ptr(), &mut st), 0);
        EXPECT_EQ!(st.st_uid, ID_HOST as c_uint);
        EXPECT_EQ!(st.st_gid, ID_HOST as c_uint);
    }

    /* O_TMPFILE goes through the separate ovl_tmpfile() path. */
    fd = openat(ovl, c".".as_ptr(), O_TMPFILE | O_WRONLY, 0o644);
    ASSERT_GE!(fd, 0);
    /* Inside the mount: caller id ID_NS. */
    ASSERT_EQ!(fstat(fd, &mut st), 0);
    EXPECT_EQ!(st.st_uid, ID_NS as c_uint);
    EXPECT_EQ!(st.st_gid, ID_NS as c_uint);
    /* Link it in so the upper backing file can be inspected too. */
    ASSERT_EQ!(linkat(fd, c"".as_ptr(), ovl, c"tmp".as_ptr(), AT_EMPTY_PATH), 0);
    EXPECT_EQ!(close(fd), 0);
    snprintf(path.as_mut_ptr(), path.len(), c"%s/u/tmp".as_ptr(), (*self_).base.as_ptr());
    ASSERT_EQ!(lstat(path.as_ptr(), &mut st), 0);
    EXPECT_EQ!(st.st_uid, ID_HOST as c_uint);
    EXPECT_EQ!(st.st_gid, ID_HOST as c_uint);

    EXPECT_EQ!(close(ovl), 0);
}

/* chown through the idmapped mount round-trips: ID_NS + 5 <-> ID_HOST + 5. */
unsafe fn idmapped_overlay_chown(self_: *mut idmapped_overlay) {
    let mut path = [0 as c_char; PATH_MAX];
    let mut st: stat = core::mem::zeroed();
    let ovl: c_int;
    let fd: c_int;

    if !ovl_supported() {
        SKIP!(return, "overlayfs not supported");
    }

    ovl = ovl_mount((*self_).base.as_ptr(), false);
    ASSERT_GE!(ovl, 0);
    ASSERT_EQ!(ovl_idmap(ovl), 0);

    fd = openat(ovl, c"f".as_ptr(), O_CREAT | O_WRONLY | O_EXCL, 0o644);
    ASSERT_GE!(fd, 0);
    EXPECT_EQ!(close(fd), 0);

    ASSERT_EQ!(fchownat(ovl, c"f".as_ptr(), (ID_NS + 5) as uid_t, (ID_NS + 5) as gid_t, 0), 0);

    ASSERT_EQ!(fstatat(ovl, c"f".as_ptr(), &mut st, 0), 0);
    EXPECT_EQ!(st.st_uid, (ID_NS + 5) as c_uint);
    EXPECT_EQ!(st.st_gid, (ID_NS + 5) as c_uint);

    snprintf(path.as_mut_ptr(), path.len(), c"%s/u/f".as_ptr(), (*self_).base.as_ptr());
    ASSERT_EQ!(stat(path.as_ptr(), &mut st), 0);
    EXPECT_EQ!(st.st_uid, (ID_HOST + 5) as c_uint);
    EXPECT_EQ!(st.st_gid, (ID_HOST + 5) as c_uint);

    EXPECT_EQ!(close(ovl), 0);
}

/*
 * Composition: an idmapped lower layer underneath an idmapped overlay mount.
 * An on-disk id is mapped by the layer idmap into the overlay-final range and
 * then by the mount idmap into the caller's range:
 *
 *   on-disk LAYER_HOST+7  --layer-->  ID_HOST+7  --mount-->  ID_NS+7
 */
unsafe fn idmapped_overlay_composition(self_: *mut idmapped_overlay) {
    let mut lower = [0 as c_char; PATH_MAX];
    let mut upper = [0 as c_char; PATH_MAX];
    let mut work = [0 as c_char; PATH_MAX];
    let mut path = [0 as c_char; PATH_MAX];
    let mut st: stat = core::mem::zeroed();
    let ovl: c_int;
    let fd_lower: c_int;

    if !ovl_supported() {
        SKIP!(return, "overlayfs not supported");
    }

    snprintf(lower.as_mut_ptr(), lower.len(), c"%s/l".as_ptr(), (*self_).base.as_ptr());
    snprintf(upper.as_mut_ptr(), upper.len(), c"%s/u".as_ptr(), (*self_).base.as_ptr());
    snprintf(work.as_mut_ptr(), work.len(), c"%s/w".as_ptr(), (*self_).base.as_ptr());

    /* Put the lower layer's ids in the on-disk [LAYER_HOST, ..) range. */
    ASSERT_EQ!(chown(lower.as_ptr(), LAYER_HOST as uid_t, LAYER_HOST as gid_t), 0);
    snprintf(path.as_mut_ptr(), path.len(), c"%s/l/file".as_ptr(), (*self_).base.as_ptr());
    ASSERT_EQ!(mknod(path.as_ptr(), S_IFREG | 0o644, 0), 0);
    ASSERT_EQ!(chown(path.as_ptr(), (LAYER_HOST + 7) as uid_t, (LAYER_HOST + 7) as gid_t), 0);

    /* Idmapped lower: on-disk LAYER_HOST <-> overlay-final ID_HOST. */
    fd_lower = idmapped_layer_fd(lower.as_ptr(), LAYER_HOST, ID_HOST, ID_RANGE);
    ASSERT_GE!(fd_lower, 0);

    ovl = ovl_mount_lower_fd(upper.as_ptr(), work.as_ptr(), fd_lower);
    ASSERT_GE!(ovl, 0);
    EXPECT_EQ!(close(fd_lower), 0);

    /* Idmap the overlay mount: overlay-final ID_HOST <-> caller ID_NS. */
    ASSERT_EQ!(ovl_idmap(ovl), 0);

    ASSERT_EQ!(fstatat(ovl, c"file".as_ptr(), &mut st, 0), 0);
    EXPECT_EQ!(st.st_uid, (ID_NS + 7) as c_uint);
    EXPECT_EQ!(st.st_gid, (ID_NS + 7) as c_uint);

    EXPECT_EQ!(close(ovl), 0);
}

/* An idmapped overlay mount whose sb lives inside a user namespace. */
unsafe fn idmapped_overlay_userns(_self_: *mut idmapped_overlay) {
    let u1: c_int;
    let pid: pid_t;

    if !ovl_supported() {
        SKIP!(return, "overlayfs not supported");
    }

    /* u1 backs the overlay sb: identity-mapped, but not the init ns. */
    u1 = get_userns_fd(0, 0, 65536);
    if u1 < 0 {
        SKIP!(return, "user namespaces not available");
    }

    pid = fork();
    ASSERT_GE!(pid, 0);
    if pid == 0 {
        let ret = userns_overlay_child(u1);

        _exit(if ret != 0 { EXIT_FAILURE } else { EXIT_SUCCESS });
    }
    EXPECT_EQ!(wait_for_pid(pid), 0);

    EXPECT_EQ!(close(u1), 0);
}

/*
 * An nfs_export overlay can be idmapped, and decodable file handles round-trip
 * through the idmapped mount with correctly mapped ownership.  Overlay file
 * handles encode object identity, not ownership, so the mount idmap does not
 * affect them; it only maps the owner reported once a handle is reopened.
 */
unsafe fn idmapped_overlay_nfs_export_handles(self_: *mut idmapped_overlay) {
    let mut path = [0 as c_char; PATH_MAX];
    let mut mnt = [0 as c_char; 128];
    let mut fhu = file_handle_storage {
        buf: [0 as c_char; size_of::<file_handle>() + MAX_HANDLE_SZ],
    };
    let fh: *mut file_handle = &mut fhu.fh as *mut core::mem::ManuallyDrop<file_handle> as *mut file_handle;
    let mut st: stat = core::mem::zeroed();
    let ovl: c_int;
    let mfd: c_int;
    let fd: c_int;
    let mut mount_id: c_int = 0;

    if !ovl_supported() {
        SKIP!(return, "overlayfs not supported");
    }

    snprintf(path.as_mut_ptr(), path.len(), c"%s/l/file".as_ptr(), (*self_).base.as_ptr());
    ASSERT_EQ!(mknod(path.as_ptr(), S_IFREG | 0o644, 0), 0);
    ASSERT_EQ!(chown(path.as_ptr(), (ID_HOST + 7) as uid_t, (ID_HOST + 7) as gid_t), 0);

    /* nfs_export=on gives decodable overlay file handles. */
    ovl = ovl_mount((*self_).base.as_ptr(), true);
    if ovl < 0 {
        SKIP!(return, "overlayfs nfs_export not supported");
    }
    ASSERT_EQ!(ovl_idmap(ovl), 0);

    /* Attach the idmapped mount so handles can be resolved against it. */
    snprintf(mnt.as_mut_ptr(), mnt.len(), c"%s/mnt".as_ptr(), (*self_).base.as_ptr());
    ASSERT_EQ!(mkdir(mnt.as_ptr(), 0o755), 0);
    ASSERT_EQ!(
        sys_move_mount(ovl, c"".as_ptr(), AT_FDCWD, mnt.as_ptr(), MOVE_MOUNT_F_EMPTY_PATH),
        0
    );

    snprintf(path.as_mut_ptr(), path.len(), c"%s/file".as_ptr(), mnt.as_ptr());
    (*fh).handle_bytes = MAX_HANDLE_SZ as c_uint;
    ASSERT_EQ!(name_to_handle_at(AT_FDCWD, path.as_ptr(), fh, &mut mount_id, 0), 0);

    mfd = open(mnt.as_ptr(), O_RDONLY | O_DIRECTORY);
    ASSERT_GE!(mfd, 0);
    fd = open_by_handle_at(mfd, fh, O_RDONLY);
    EXPECT_EQ!(close(mfd), 0);
    ASSERT_GE!(fd, 0);

    ASSERT_EQ!(fstat(fd, &mut st), 0);
    EXPECT_EQ!(st.st_uid, (ID_NS + 7) as c_uint);
    EXPECT_EQ!(st.st_gid, (ID_NS + 7) as c_uint);

    EXPECT_EQ!(close(fd), 0);
    EXPECT_EQ!(close(ovl), 0);
}

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
