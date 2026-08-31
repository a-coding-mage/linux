// SPDX-License-Identifier: GPL-2.0-or-later

// Translated from statmount_test.c. C includes are represented by the extern
// declarations and constants below; definitions from statmount.h and
// kselftest.h are expected to be supplied by the surrounding build.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type uint32_t = u32;
type uint64_t = u64;
type uid_t = u32;
type gid_t = u32;
type FILE = c_void;

const O_WRONLY: c_int = 1;
const O_PATH: c_int = 0o10000000;
const AT_FDCWD: c_int = -100;
const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWUSER: c_int = 0x10000000;
const CLONE_NEWPID: c_int = 0x20000000;
const MS_RDONLY: c_ulong = 1;
const MS_NOSUID: c_ulong = 2;
const MS_NODEV: c_ulong = 4;
const MS_NOEXEC: c_ulong = 8;
const MS_SYNCHRONOUS: c_ulong = 16;
const MS_REMOUNT: c_ulong = 32;
const MS_MANDLOCK: c_ulong = 64;
const MS_DIRSYNC: c_ulong = 128;
const MS_NOSYMFOLLOW: c_ulong = 256;
const MS_NOATIME: c_ulong = 1024;
const MS_NODIRATIME: c_ulong = 2048;
const MS_BIND: c_ulong = 4096;
const MS_MOVE: c_ulong = 8192;
const MS_REC: c_ulong = 16384;
const MS_VERBOSE: c_ulong = 32768;
const MS_SILENT: c_ulong = 32768;
const MS_POSIXACL: c_ulong = 1 << 16;
const MS_UNBINDABLE: c_ulong = 1 << 17;
const MS_PRIVATE: c_ulong = 1 << 18;
const MS_SLAVE: c_ulong = 1 << 19;
const MS_SHARED: c_ulong = 1 << 20;
const MS_RELATIME: c_ulong = 1 << 21;
const MS_KERNMOUNT: c_ulong = 1 << 22;
const MS_I_VERSION: c_ulong = 1 << 23;
const MS_STRICTATIME: c_ulong = 1 << 24;
const MS_LAZYTIME: c_ulong = 1 << 25;
const MNT_DETACH: c_int = 2;
const PATH_MAX: usize = 4096;
const ENOSYS: c_int = 38;
const EOVERFLOW: c_int = 75;

const STATX_MNT_ID: uint32_t = 0x00001000;
const STATX_MNT_ID_UNIQUE: uint32_t = 0x00004000;

const STATMOUNT_SB_BASIC: uint64_t = 0x00000001;
const STATMOUNT_MNT_BASIC: uint64_t = 0x00000002;
const STATMOUNT_PROPAGATE_FROM: uint64_t = 0x00000004;
const STATMOUNT_MNT_ROOT: uint64_t = 0x00000008;
const STATMOUNT_MNT_POINT: uint64_t = 0x00000010;
const STATMOUNT_FS_TYPE: uint64_t = 0x00000020;
const STATMOUNT_MNT_NS_ID: uint64_t = 0x00000040;
const STATMOUNT_MNT_OPTS: uint64_t = 0x00000080;
const LSMT_ROOT: uint64_t = 0xffffffffffffffff;

#[repr(C)]
struct statx {
    stx_mask: uint32_t,
    stx_blksize: uint32_t,
    stx_attributes: uint64_t,
    stx_nlink: uint32_t,
    stx_uid: uint32_t,
    stx_gid: uint32_t,
    stx_mode: u16,
    __spare0: [u16; 1],
    stx_ino: uint64_t,
    stx_size: uint64_t,
    stx_blocks: uint64_t,
    stx_attributes_mask: uint64_t,
    __spare_timestamp_space: [uint64_t; 8],
    stx_rdev_major: uint32_t,
    stx_rdev_minor: uint32_t,
    stx_dev_major: uint32_t,
    stx_dev_minor: uint32_t,
    stx_mnt_id: uint64_t,
    __spare2: uint64_t,
    __spare3: [uint64_t; 12],
}

#[repr(C)]
struct statfs {
    f_type: c_long,
    f_bsize: c_long,
    f_blocks: u64,
    f_bfree: u64,
    f_bavail: u64,
    f_files: u64,
    f_ffree: u64,
    f_fsid: [c_int; 2],
    f_namelen: c_long,
    f_frsize: c_long,
    f_flags: c_long,
    f_spare: [c_long; 4],
}

#[repr(C)]
struct statmount {
    size: uint32_t,
    __spare1: uint32_t,
    mask: uint64_t,
    sb_dev_major: uint32_t,
    sb_dev_minor: uint32_t,
    sb_magic: uint64_t,
    mnt_id: uint64_t,
    mnt_parent_id: uint64_t,
    mnt_id_old: uint32_t,
    mnt_parent_id_old: uint32_t,
    mnt_attr: uint64_t,
    mnt_propagation: uint64_t,
    mnt_peer_group: uint64_t,
    mnt_master: uint64_t,
    propagate_from: uint64_t,
    mnt_root: uint32_t,
    mnt_point: uint32_t,
    fs_type: uint32_t,
    mnt_ns_id: uint64_t,
    mnt_opts: uint32_t,
    __spare2: [uint64_t; 50],
    str_: [c_char; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn strlen(s: *const c_char) -> size_t;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn statx(dirfd: c_int, pathname: *const c_char, flags: c_int, mask: c_uint, statxbuf: *mut statx) -> c_int;
    fn fchdir(fd: c_int) -> c_int;
    fn chroot(path: *const c_char) -> c_int;
    fn chdir(path: *const c_char) -> c_int;
    fn umount2(target: *const c_char, flags: c_int) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn getuid() -> uid_t;
    fn getgid() -> gid_t;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn mount(source: *const c_char, target: *const c_char, filesystemtype: *const c_char, mountflags: c_ulong, data: *const c_void) -> c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn atexit(function: unsafe extern "C" fn()) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn atoi(nptr: *const c_char) -> c_int;
    fn statfs(path: *const c_char, buf: *mut statfs) -> c_int;
    fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;

    fn listmount(mnt_id: uint64_t, param: uint64_t, last_mnt_id: uint64_t, list: *mut uint64_t, num: size_t, flags: c_uint) -> ssize_t;
    fn statmount(mnt_id: uint64_t, param: uint64_t, flags: uint64_t, mask: uint64_t, buf: *mut statmount, bufsize: size_t, reserved: c_uint) -> c_int;
    fn statmount_alloc(mnt_id: uint64_t, param: uint64_t, mask: uint64_t, flags: c_uint) -> *mut statmount;
    fn statmount_alloc_by_fd(fd: c_int, mask: uint64_t) -> *mut statmount;

    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_perror(msg: *const c_char);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_set_plan(plan: c_uint);
    fn ksft_get_fail_cnt() -> c_int;
    fn ksft_get_error_cnt() -> c_int;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

static KNOWN_FS: &[*const c_char] = &[
    c"9p".as_ptr(), c"adfs".as_ptr(), c"affs".as_ptr(), c"afs".as_ptr(),
    c"aio".as_ptr(), c"anon_inodefs".as_ptr(), c"apparmorfs".as_ptr(),
    c"autofs".as_ptr(), c"bcachefs".as_ptr(), c"bdev".as_ptr(),
    c"befs".as_ptr(), c"bfs".as_ptr(), c"binder".as_ptr(),
    c"binfmt_misc".as_ptr(), c"bpf".as_ptr(), c"btrfs".as_ptr(),
    c"btrfs_test_fs".as_ptr(), c"ceph".as_ptr(), c"cgroup".as_ptr(),
    c"cgroup2".as_ptr(), c"cifs".as_ptr(), c"coda".as_ptr(),
    c"configfs".as_ptr(), c"cpuset".as_ptr(), c"cramfs".as_ptr(),
    c"cxl".as_ptr(), c"dax".as_ptr(), c"debugfs".as_ptr(),
    c"devpts".as_ptr(), c"devtmpfs".as_ptr(), c"dmabuf".as_ptr(),
    c"drm".as_ptr(), c"ecryptfs".as_ptr(), c"efivarfs".as_ptr(),
    c"efs".as_ptr(), c"erofs".as_ptr(), c"exfat".as_ptr(),
    c"ext2".as_ptr(), c"ext3".as_ptr(), c"ext4".as_ptr(),
    c"f2fs".as_ptr(), c"functionfs".as_ptr(), c"fuse".as_ptr(),
    c"fuseblk".as_ptr(), c"fusectl".as_ptr(), c"gadgetfs".as_ptr(),
    c"gfs2".as_ptr(), c"gfs2meta".as_ptr(), c"hfs".as_ptr(),
    c"hfsplus".as_ptr(), c"hostfs".as_ptr(), c"hpfs".as_ptr(),
    c"hugetlbfs".as_ptr(), c"ibmasmfs".as_ptr(), c"iomem".as_ptr(),
    c"ipathfs".as_ptr(), c"iso9660".as_ptr(), c"jffs2".as_ptr(),
    c"jfs".as_ptr(), c"minix".as_ptr(), c"mqueue".as_ptr(),
    c"msdos".as_ptr(), c"nfs".as_ptr(), c"nfs4".as_ptr(),
    c"nfsd".as_ptr(), c"nilfs2".as_ptr(), c"nsfs".as_ptr(),
    c"ntfs".as_ptr(), c"ntfs3".as_ptr(), c"ocfs2".as_ptr(),
    c"ocfs2_dlmfs".as_ptr(), c"omfs".as_ptr(), c"openpromfs".as_ptr(),
    c"overlay".as_ptr(), c"pipefs".as_ptr(), c"proc".as_ptr(),
    c"pstore".as_ptr(), c"pvfs2".as_ptr(), c"qnx4".as_ptr(),
    c"qnx6".as_ptr(), c"ramfs".as_ptr(), c"resctrl".as_ptr(),
    c"romfs".as_ptr(), c"rootfs".as_ptr(), c"rpc_pipefs".as_ptr(),
    c"s390_hypfs".as_ptr(), c"secretmem".as_ptr(), c"securityfs".as_ptr(),
    c"selinuxfs".as_ptr(), c"smackfs".as_ptr(), c"smb3".as_ptr(),
    c"sockfs".as_ptr(), c"spufs".as_ptr(), c"squashfs".as_ptr(),
    c"sysfs".as_ptr(), c"sysv".as_ptr(), c"tmpfs".as_ptr(),
    c"tracefs".as_ptr(), c"ubifs".as_ptr(), c"udf".as_ptr(),
    c"ufs".as_ptr(), c"v7".as_ptr(), c"vboxsf".as_ptr(),
    c"vfat".as_ptr(), c"virtiofs".as_ptr(), c"vxfs".as_ptr(),
    c"xenfs".as_ptr(), c"xfs".as_ptr(), c"zonefs".as_ptr(), ptr::null(),
];

static mut root_mntpoint: [c_char; 36] = *b"/tmp/statmount_test_root.XXXXXX\0";
static mut orig_root: c_int = 0;
static mut root_id: uint64_t = 0;
static mut parent_id: uint64_t = 0;
static mut old_root_id: uint32_t = 0;
static mut old_parent_id: uint32_t = 0;
static mut f_mountinfo: *mut FILE = ptr::null_mut();

unsafe fn sm_str(sm: *mut statmount, off: uint32_t) -> *mut c_char {
    ((*sm).str_.as_mut_ptr()).add(off as usize)
}

unsafe fn write_file(path: *const c_char, val: *const c_char) {
    let fd = open(path, O_WRONLY);
    let len = strlen(val);
    let ret: c_int;

    if fd == -1 {
        ksft_exit_fail_msg(c"opening %s for write: %s\n".as_ptr(), path, strerror(errno));
    }

    ret = write(fd, val as *const c_void, len) as c_int;
    if ret == -1 {
        ksft_exit_fail_msg(c"writing to %s: %s\n".as_ptr(), path, strerror(errno));
    }
    if ret as size_t != len {
        ksft_exit_fail_msg(c"short write to %s\n".as_ptr(), path);
    }

    let ret = close(fd);
    if ret == -1 {
        ksft_exit_fail_msg(c"closing %s\n".as_ptr(), path);
    }
}

unsafe fn get_mnt_id(name: *const c_char, path: *const c_char, mask: uint64_t) -> uint64_t {
    let mut sx: statx = core::mem::zeroed();
    let ret = statx(AT_FDCWD, path, 0, mask as c_uint, &mut sx);
    if ret == -1 {
        ksft_exit_fail_msg(
            c"retrieving %s mount ID for %s: %s\n".as_ptr(),
            if mask & STATX_MNT_ID_UNIQUE as uint64_t != 0 { c"unique".as_ptr() } else { c"old".as_ptr() },
            name,
            strerror(errno),
        );
    }
    if sx.stx_mask & mask as uint32_t == 0 {
        ksft_exit_fail_msg(
            c"no %s mount ID available for %s\n".as_ptr(),
            if mask & STATX_MNT_ID_UNIQUE as uint64_t != 0 { c"unique".as_ptr() } else { c"old".as_ptr() },
            name,
        );
    }

    sx.stx_mnt_id
}

unsafe extern "C" fn cleanup_namespace() {
    let ret: c_int;

    if !f_mountinfo.is_null() {
        fclose(f_mountinfo);
    }

    ret = fchdir(orig_root);
    if ret == -1 {
        ksft_perror(c"fchdir to original root".as_ptr());
    }

    let ret = chroot(c".".as_ptr());
    if ret == -1 {
        ksft_perror(c"chroot to original root".as_ptr());
    }

    umount2(root_mntpoint.as_ptr(), MNT_DETACH);
    rmdir(root_mntpoint.as_ptr());
}

unsafe fn setup_namespace() {
    let ret: c_int;
    let mut buf = [0 as c_char; 32];
    let uid = getuid();
    let gid = getgid();

    ret = unshare(CLONE_NEWNS | CLONE_NEWUSER | CLONE_NEWPID);
    if ret == -1 {
        ksft_exit_fail_msg(c"unsharing mountns and userns: %s\n".as_ptr(), strerror(errno));
    }

    sprintf(buf.as_mut_ptr(), c"0 %d 1".as_ptr(), uid);
    write_file(c"/proc/self/uid_map".as_ptr(), buf.as_ptr());
    write_file(c"/proc/self/setgroups".as_ptr(), c"deny".as_ptr());
    sprintf(buf.as_mut_ptr(), c"0 %d 1".as_ptr(), gid);
    write_file(c"/proc/self/gid_map".as_ptr(), buf.as_ptr());

    f_mountinfo = fopen(c"/proc/self/mountinfo".as_ptr(), c"re".as_ptr());
    if f_mountinfo.is_null() {
        ksft_exit_fail_msg(c"failed to open mountinfo: %s\n".as_ptr(), strerror(errno));
    }

    ret = mount(c"".as_ptr(), c"/".as_ptr(), ptr::null(), MS_REC | MS_PRIVATE, ptr::null());
    if ret == -1 {
        ksft_exit_fail_msg(c"making mount tree private: %s\n".as_ptr(), strerror(errno));
    }

    if mkdtemp(root_mntpoint.as_mut_ptr()).is_null() {
        ksft_exit_fail_msg(c"creating temporary directory %s: %s\n".as_ptr(), root_mntpoint.as_ptr(), strerror(errno));
    }

    old_parent_id = get_mnt_id(c"parent".as_ptr(), root_mntpoint.as_ptr(), STATX_MNT_ID as uint64_t) as uint32_t;
    parent_id = get_mnt_id(c"parent".as_ptr(), root_mntpoint.as_ptr(), STATX_MNT_ID_UNIQUE as uint64_t);

    orig_root = open(c"/".as_ptr(), O_PATH);
    if orig_root == -1 {
        ksft_exit_fail_msg(c"opening root directory: %s".as_ptr(), strerror(errno));
    }

    atexit(cleanup_namespace);

    ret = mount(root_mntpoint.as_ptr(), root_mntpoint.as_ptr(), ptr::null(), MS_BIND, ptr::null());
    if ret == -1 {
        ksft_exit_fail_msg(c"mounting temp root %s: %s\n".as_ptr(), root_mntpoint.as_ptr(), strerror(errno));
    }

    ret = chroot(root_mntpoint.as_ptr());
    if ret == -1 {
        ksft_exit_fail_msg(c"chroot to temp root %s: %s\n".as_ptr(), root_mntpoint.as_ptr(), strerror(errno));
    }

    ret = chdir(c"/".as_ptr());
    if ret == -1 {
        ksft_exit_fail_msg(c"chdir to root: %s\n".as_ptr(), strerror(errno));
    }

    old_root_id = get_mnt_id(c"root".as_ptr(), c"/".as_ptr(), STATX_MNT_ID as uint64_t) as uint32_t;
    root_id = get_mnt_id(c"root".as_ptr(), c"/".as_ptr(), STATX_MNT_ID_UNIQUE as uint64_t);
}

unsafe fn setup_mount_tree(log2_num: c_int) -> c_int {
    let mut ret = mount(c"".as_ptr(), c"/".as_ptr(), ptr::null(), MS_REC | MS_SHARED, ptr::null());
    if ret == -1 {
        ksft_test_result_fail(c"making mount tree shared: %s\n".as_ptr(), strerror(errno));
        return -1;
    }

    let mut i = 0;
    while i < log2_num {
        ret = mount(c"/".as_ptr(), c"/".as_ptr(), ptr::null(), MS_BIND, ptr::null());
        if ret == -1 {
            ksft_test_result_fail(c"mounting submount %s: %s\n".as_ptr(), root_mntpoint.as_ptr(), strerror(errno));
            return -1;
        }
        i += 1;
    }
    0
}

unsafe fn test_listmount_empty_root() {
    let size: c_uint = 32;
    let mut list = [0 as uint64_t; 32];

    let res = listmount(LSMT_ROOT, 0, 0, list.as_mut_ptr(), size as size_t, 0);
    if res == -1 {
        ksft_test_result_fail(c"listmount: %s\n".as_ptr(), strerror(errno));
        return;
    }
    if res != 1 {
        ksft_test_result_fail(c"listmount result is %zi != 1\n".as_ptr(), res);
        return;
    }

    if list[0] != root_id {
        ksft_test_result_fail(c"listmount ID doesn't match 0x%llx != 0x%llx\n".as_ptr(), list[0] as u64, root_id as u64);
        return;
    }

    ksft_test_result_pass(c"listmount empty root\n".as_ptr());
}

unsafe fn test_statmount_zero_mask() {
    let mut sm: statmount = core::mem::zeroed();
    let ret = statmount(root_id, 0, 0, 0, &mut sm, size_of::<statmount>(), 0);
    if ret == -1 {
        ksft_test_result_fail(c"statmount zero mask: %s\n".as_ptr(), strerror(errno));
        return;
    }
    if sm.size as usize != size_of::<statmount>() {
        ksft_test_result_fail(c"unexpected size: %u != %u\n".as_ptr(), sm.size, size_of::<statmount>() as uint32_t);
        return;
    }
    if sm.mask != 0 {
        ksft_test_result_fail(c"unexpected mask: 0x%llx != 0x0\n".as_ptr(), sm.mask as u64);
        return;
    }

    ksft_test_result_pass(c"statmount zero mask\n".as_ptr());
}

unsafe fn test_statmount_mnt_basic() {
    let mut sm: statmount = core::mem::zeroed();
    let mask = STATMOUNT_MNT_BASIC;

    let ret = statmount(root_id, 0, 0, mask, &mut sm, size_of::<statmount>(), 0);
    if ret == -1 {
        ksft_test_result_fail(c"statmount mnt basic: %s\n".as_ptr(), strerror(errno));
        return;
    }
    if sm.size as usize != size_of::<statmount>() {
        ksft_test_result_fail(c"unexpected size: %u != %u\n".as_ptr(), sm.size, size_of::<statmount>() as uint32_t);
        return;
    }
    if sm.mask != mask {
        ksft_test_result_skip(c"statmount mnt basic unavailable\n".as_ptr());
        return;
    }

    if sm.mnt_id != root_id {
        ksft_test_result_fail(c"unexpected root ID: 0x%llx != 0x%llx\n".as_ptr(), sm.mnt_id as u64, root_id as u64);
        return;
    }
    if sm.mnt_id_old != old_root_id {
        ksft_test_result_fail(c"unexpected old root ID: %u != %u\n".as_ptr(), sm.mnt_id_old, old_root_id);
        return;
    }
    if sm.mnt_parent_id != parent_id {
        ksft_test_result_fail(c"unexpected parent ID: 0x%llx != 0x%llx\n".as_ptr(), sm.mnt_parent_id as u64, parent_id as u64);
        return;
    }
    if sm.mnt_parent_id_old != old_parent_id {
        ksft_test_result_fail(c"unexpected old parent ID: %u != %u\n".as_ptr(), sm.mnt_parent_id_old, old_parent_id);
        return;
    }
    if sm.mnt_propagation != MS_PRIVATE as uint64_t {
        ksft_test_result_fail(c"unexpected propagation: 0x%llx\n".as_ptr(), sm.mnt_propagation as u64);
        return;
    }

    ksft_test_result_pass(c"statmount mnt basic\n".as_ptr());
}

unsafe fn test_statmount_sb_basic() {
    let mut sm: statmount = core::mem::zeroed();
    let mask = STATMOUNT_SB_BASIC;
    let mut sx: statx = core::mem::zeroed();
    let mut sf: statfs = core::mem::zeroed();

    let mut ret = statmount(root_id, 0, 0, mask, &mut sm, size_of::<statmount>(), 0);
    if ret == -1 {
        ksft_test_result_fail(c"statmount sb basic: %s\n".as_ptr(), strerror(errno));
        return;
    }
    if sm.size as usize != size_of::<statmount>() {
        ksft_test_result_fail(c"unexpected size: %u != %u\n".as_ptr(), sm.size, size_of::<statmount>() as uint32_t);
        return;
    }
    if sm.mask != mask {
        ksft_test_result_skip(c"statmount sb basic unavailable\n".as_ptr());
        return;
    }

    ret = statx(AT_FDCWD, c"/".as_ptr(), 0, 0, &mut sx);
    if ret == -1 {
        ksft_test_result_fail(c"stat root failed: %s\n".as_ptr(), strerror(errno));
        return;
    }

    if sm.sb_dev_major != sx.stx_dev_major || sm.sb_dev_minor != sx.stx_dev_minor {
        ksft_test_result_fail(c"unexpected sb dev %u:%u != %u:%u\n".as_ptr(), sm.sb_dev_major, sm.sb_dev_minor, sx.stx_dev_major, sx.stx_dev_minor);
        return;
    }

    ret = statfs(c"/".as_ptr(), &mut sf);
    if ret == -1 {
        ksft_test_result_fail(c"statfs root failed: %s\n".as_ptr(), strerror(errno));
        return;
    }

    if sm.sb_magic != sf.f_type as uint64_t {
        ksft_test_result_fail(c"unexpected sb magic: 0x%llx != 0x%lx\n".as_ptr(), sm.sb_magic as u64, sf.f_type);
        return;
    }

    ksft_test_result_pass(c"statmount sb basic\n".as_ptr());
}

unsafe fn test_statmount_mnt_point() {
    let sm = statmount_alloc(root_id, 0, STATMOUNT_MNT_POINT, 0);
    if sm.is_null() {
        ksft_test_result_fail(c"statmount mount point: %s\n".as_ptr(), strerror(errno));
        return;
    }

    if (*sm).mask & STATMOUNT_MNT_POINT == 0 {
        ksft_test_result_fail(c"missing STATMOUNT_MNT_POINT in mask\n".as_ptr());
        return;
    }
    if strcmp(sm_str(sm, (*sm).mnt_point), c"/".as_ptr()) != 0 {
        ksft_test_result_fail(c"unexpected mount point: '%s' != '/'\n".as_ptr(), sm_str(sm, (*sm).mnt_point));
        free(sm as *mut c_void);
        return;
    }
    ksft_test_result_pass(c"statmount mount point\n".as_ptr());
    free(sm as *mut c_void);
}

unsafe fn test_statmount_mnt_root() {
    let last_dir = strrchr(root_mntpoint.as_ptr(), '/' as c_int);
    assert!(!last_dir.is_null());
    let last_dir = last_dir.add(1);

    let sm = statmount_alloc(root_id, 0, STATMOUNT_MNT_ROOT, 0);
    if sm.is_null() {
        ksft_test_result_fail(c"statmount mount root: %s\n".as_ptr(), strerror(errno));
        return;
    }
    if (*sm).mask & STATMOUNT_MNT_ROOT == 0 {
        ksft_test_result_fail(c"missing STATMOUNT_MNT_ROOT in mask\n".as_ptr());
        return;
    }
    let mnt_root = sm_str(sm, (*sm).mnt_root);
    let mut last_root = strrchr(mnt_root, '/' as c_int);
    if !last_root.is_null() {
        last_root = last_root.add(1);
    } else {
        last_root = mnt_root;
    }

    if strcmp(last_dir, last_root) != 0 {
        ksft_test_result_fail(c"unexpected mount root last component: '%s' != '%s'\n".as_ptr(), last_root, last_dir);
        free(sm as *mut c_void);
        return;
    }
    ksft_test_result_pass(c"statmount mount root\n".as_ptr());
    free(sm as *mut c_void);
}

unsafe fn test_statmount_fs_type() {
    let sm = statmount_alloc(root_id, 0, STATMOUNT_FS_TYPE, 0);
    if sm.is_null() {
        ksft_test_result_fail(c"statmount fs type: %s\n".as_ptr(), strerror(errno));
        return;
    }
    if (*sm).mask & STATMOUNT_FS_TYPE == 0 {
        ksft_test_result_fail(c"missing STATMOUNT_FS_TYPE in mask\n".as_ptr());
        return;
    }
    let fs_type = sm_str(sm, (*sm).fs_type);
    let mut s = KNOWN_FS.as_ptr();
    while !s.is_null() && !(*s).is_null() {
        if strcmp(fs_type, *s) == 0 {
            break;
        }
        s = s.add(1);
    }
    if s.is_null() || (*s).is_null() {
        ksft_print_msg(c"unknown filesystem type: %s\n".as_ptr(), fs_type);
    }

    ksft_test_result_pass(c"statmount fs type\n".as_ptr());
    free(sm as *mut c_void);
}

unsafe fn test_statmount_mnt_opts() {
    let sm = statmount_alloc(root_id, 0, STATMOUNT_MNT_BASIC | STATMOUNT_MNT_OPTS, 0);
    let mut statmount_opts: *const c_char;
    let mut line: *mut c_char = ptr::null_mut();
    let mut len: size_t = 0;

    if sm.is_null() {
        ksft_test_result_fail(c"statmount mnt opts: %s\n".as_ptr(), strerror(errno));
        return;
    }

    if (*sm).mask & STATMOUNT_MNT_BASIC == 0 {
        ksft_test_result_fail(c"missing STATMOUNT_MNT_BASIC in mask\n".as_ptr());
        return;
    }

    while getline(&mut line, &mut len, f_mountinfo) != -1 {
        let mut i: c_int;
        let mut p: *mut c_char;
        let mut p2: *mut c_char;
        let old_mnt_id = atoi(line) as c_uint;
        if old_mnt_id != (*sm).mnt_id_old {
            continue;
        }

        p = line;
        i = 0;
        while !p.is_null() && i < 5 {
            p = strchr(p.add(1), ' ' as c_int);
            i += 1;
        }
        if p.is_null() {
            continue;
        }

        p2 = strchr(p.add(1), ' ' as c_int);
        if p2.is_null() {
            continue;
        }
        *p2 = 0;
        p = strchr(p2.add(1), '-' as c_int);
        if p.is_null() {
            continue;
        }
        p = p.add(1);
        i = 0;
        while !p.is_null() && i < 2 {
            p = strchr(p.add(1), ' ' as c_int);
            i += 1;
        }
        if p.is_null() {
            continue;
        }
        p = p.add(1);

        /* skip generic superblock options */
        if strncmp(p, c"ro".as_ptr(), 2) == 0 {
            p = p.add(2);
        } else if strncmp(p, c"rw".as_ptr(), 2) == 0 {
            p = p.add(2);
        }
        if *p == ',' as c_char {
            p = p.add(1);
        }
        if strncmp(p, c"sync".as_ptr(), 4) == 0 {
            p = p.add(4);
        }
        if *p == ',' as c_char {
            p = p.add(1);
        }
        if strncmp(p, c"dirsync".as_ptr(), 7) == 0 {
            p = p.add(7);
        }
        if *p == ',' as c_char {
            p = p.add(1);
        }
        if strncmp(p, c"lazytime".as_ptr(), 8) == 0 {
            p = p.add(8);
        }
        if *p == ',' as c_char {
            p = p.add(1);
        }
        p2 = strrchr(p, '\n' as c_int);
        if !p2.is_null() {
            *p2 = 0;
        }

        if (*sm).mask & STATMOUNT_MNT_OPTS != 0 {
            statmount_opts = sm_str(sm, (*sm).mnt_opts);
        } else {
            statmount_opts = c"".as_ptr();
        }
        if strcmp(statmount_opts, p) != 0 {
            ksft_test_result_fail(c"unexpected mount options: '%s' != '%s'\n".as_ptr(), statmount_opts, p);
        } else {
            ksft_test_result_pass(c"statmount mount options\n".as_ptr());
        }
        free(sm as *mut c_void);
        free(line as *mut c_void);
        return;
    }

    ksft_test_result_fail(c"didn't find mount entry\n".as_ptr());
    free(sm as *mut c_void);
    free(line as *mut c_void);
}

unsafe fn test_statmount_string(mask: uint64_t, off: size_t, name: *const c_char) {
    let sm = statmount_alloc(root_id, 0, mask, 0);
    let mut len: size_t;
    let shortsize: size_t;
    let exactsize: size_t;
    let start: uint32_t;
    let mut i: uint32_t;
    let mut ret: c_int;

    if sm.is_null() {
        ksft_test_result_fail(c"statmount %s: %s\n".as_ptr(), name, strerror(errno));
        free(sm as *mut c_void);
        return;
    }
    if (*sm).size as usize  < size_of::<statmount>() {
        ksft_test_result_fail(c"unexpected size: %u < %u\n".as_ptr(), (*sm).size, size_of::<statmount>() as uint32_t);
        free(sm as *mut c_void);
        return;
    }
    if (*sm).mask != mask {
        ksft_test_result_skip(c"statmount %s unavailable\n".as_ptr(), name);
        free(sm as *mut c_void);
        return;
    }
    len = (*sm).size as size_t - size_of::<statmount>();
    start = *((sm as *mut uint32_t).add(off));

    i = start;
    loop {
        if i as size_t >= len {
            ksft_test_result_fail(c"string out of bounds\n".as_ptr());
            free(sm as *mut c_void);
            return;
        }
        if *sm_str(sm, i) == 0 {
            break;
        }
        i = i.wrapping_add(1);
    }
    exactsize = (*sm).size as size_t;
    shortsize = size_of::<statmount>() + i as size_t;

    ret = statmount(root_id, 0, 0, mask, sm, exactsize, 0);
    if ret == -1 {
        ksft_test_result_fail(c"statmount exact size: %s\n".as_ptr(), strerror(errno));
        free(sm as *mut c_void);
        return;
    }
    errno = 0;
    ret = statmount(root_id, 0, 0, mask, sm, shortsize, 0);
    if ret != -1 || errno != EOVERFLOW {
        ksft_test_result_fail(c"should have failed with EOVERFLOW: %s\n".as_ptr(), strerror(errno));
        free(sm as *mut c_void);
        return;
    }

    ksft_test_result_pass(c"statmount string %s\n".as_ptr(), name);
    free(sm as *mut c_void);
}

unsafe fn test_listmount_tree() {
    let log2_num: c_uint = 4;
    let step: c_uint = 3;
    let size: c_uint = (1 << log2_num) + step + 1;
    let mut num: size_t;
    let expect: size_t = 1 << log2_num;
    let mut list = [0 as uint64_t; 20];
    let mut list2 = [0 as uint64_t; 20];
    let mut i: size_t;

    let mut res = setup_mount_tree(log2_num as c_int) as ssize_t;
    if res == -1 {
        return;
    }

    res = listmount(LSMT_ROOT, 0, 0, list.as_mut_ptr(), size as size_t, 0);
    num = res as size_t;
    if res == -1 {
        ksft_test_result_fail(c"listmount: %s\n".as_ptr(), strerror(errno));
        return;
    }
    if num != expect {
        ksft_test_result_fail(c"listmount result is %zi != %zi\n".as_ptr(), res, expect);
        return;
    }

    i = 0;
    while i < size as size_t - step as size_t {
        res = listmount(
            LSMT_ROOT,
            0,
            if i != 0 { list2[i - 1] } else { 0 },
            list2.as_mut_ptr().add(i),
            step as size_t,
            0,
        );
        if res == -1 {
            ksft_test_result_fail(c"short listmount: %s\n".as_ptr(), strerror(errno));
        }
        i += res as size_t;
        if res < step as ssize_t {
            break;
        }
    }
    if i != num {
        ksft_test_result_fail(c"different number of entries: %zu != %zu\n".as_ptr(), i, num);
        return;
    }
    i = 0;
    while i < num {
        if list2[i] != list[i] {
            ksft_test_result_fail(c"different value for entry %zu: 0x%llx != 0x%llx\n".as_ptr(), i, list2[i] as u64, list[i] as u64);
        }
        i += 1;
    }

    ksft_test_result_pass(c"listmount tree\n".as_ptr());
}

unsafe fn test_statmount_by_fd() {
    let mut sm: *mut statmount = ptr::null_mut();
    let mut tmpdir: [c_char; 22] = *b"/statmount.fd.XXXXXX\0";
    let root: [c_char; 6] = *b"/test\0";
    let mut subdir = [0 as c_char; PATH_MAX];
    let mut tmproot = [0 as c_char; PATH_MAX];
    let fd: c_int;

    if mkdtemp(tmpdir.as_mut_ptr()).is_null() {
        ksft_perror(c"mkdtemp".as_ptr());
        return;
    }

    if mount(c"statmount.test".as_ptr(), tmpdir.as_ptr(), c"tmpfs".as_ptr(), 0, ptr::null()) != 0 {
        ksft_perror(c"mount".as_ptr());
        rmdir(tmpdir.as_ptr());
        return;
    }

    snprintf(subdir.as_mut_ptr(), PATH_MAX, c"%s%s".as_ptr(), tmpdir.as_ptr(), root.as_ptr());
    snprintf(tmproot.as_mut_ptr(), PATH_MAX, c"%s/%s".as_ptr(), tmpdir.as_ptr(), c"chroot".as_ptr());

    if mkdir(subdir.as_ptr(), 0o755) != 0 {
        ksft_perror(c"mkdir".as_ptr());
        goto_err_tmpdir(&mut sm, -1, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }
    if mount(subdir.as_ptr(), subdir.as_ptr(), ptr::null(), MS_BIND, ptr::null()) != 0 {
        ksft_perror(c"mount".as_ptr());
        goto_err_subdir(&mut sm, -1, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }
    if mkdir(tmproot.as_ptr(), 0o755) != 0 {
        ksft_perror(c"mkdir".as_ptr());
        goto_err_subdir(&mut sm, -1, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }

    fd = open(subdir.as_ptr(), O_PATH);
    if fd < 0 {
        ksft_perror(c"open".as_ptr());
        goto_err_tmproot(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }

    if chroot(tmproot.as_ptr()) != 0 {
        ksft_perror(c"chroot".as_ptr());
        goto_err_fd(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }

    sm = statmount_alloc_by_fd(fd, STATMOUNT_MNT_ROOT | STATMOUNT_MNT_POINT);
    if sm.is_null() {
        ksft_test_result_fail(c"statmount by fd failed: %s\n".as_ptr(), strerror(errno));
        chroot(c".".as_ptr());
        goto_out(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }
    if (*sm).size as usize  < size_of::<statmount>() {
        ksft_test_result_fail(c"unexpected size: %u < %u\n".as_ptr(), (*sm).size, size_of::<statmount>() as uint32_t);
        chroot(c".".as_ptr());
        goto_out(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }
    if (*sm).mask & STATMOUNT_MNT_POINT != 0 {
        ksft_test_result_fail(c"STATMOUNT_MNT_POINT unexpectedly set in statmount\n".as_ptr());
        chroot(c".".as_ptr());
        goto_out(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }
    if (*sm).mask & STATMOUNT_MNT_ROOT == 0 {
        ksft_test_result_fail(c"STATMOUNT_MNT_ROOT not set in statmount\n".as_ptr());
        chroot(c".".as_ptr());
        goto_out(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }
    if strcmp(root.as_ptr(), sm_str(sm, (*sm).mnt_root)) != 0 {
        ksft_test_result_fail(c"statmount returned incorrect mnt_root,statmount mnt_root: %s != %s\n".as_ptr(), sm_str(sm, (*sm).mnt_root), root.as_ptr());
        chroot(c".".as_ptr());
        goto_out(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }

    if chroot(c".".as_ptr()) != 0 {
        ksft_perror(c"chroot".as_ptr());
        goto_out(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }

    free(sm as *mut c_void);
    sm = statmount_alloc_by_fd(fd, STATMOUNT_MNT_ROOT | STATMOUNT_MNT_POINT);
    if sm.is_null() {
        ksft_test_result_fail(c"statmount by fd failed: %s\n".as_ptr(), strerror(errno));
        goto_err_fd(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }
    if (*sm).size as usize  < size_of::<statmount>() {
        ksft_test_result_fail(c"unexpected size: %u < %u\n".as_ptr(), (*sm).size, size_of::<statmount>() as uint32_t);
        goto_out(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }
    if (*sm).mask & STATMOUNT_MNT_POINT == 0 {
        ksft_test_result_fail(c"STATMOUNT_MNT_POINT not set in statmount\n".as_ptr());
        goto_out(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }
    if (*sm).mask & STATMOUNT_MNT_ROOT == 0 {
        ksft_test_result_fail(c"STATMOUNT_MNT_ROOT not set in statmount\n".as_ptr());
        goto_out(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }
    if strcmp(subdir.as_ptr(), sm_str(sm, (*sm).mnt_point)) != 0 {
        ksft_test_result_fail(c"statmount returned incorrect mnt_point,statmount mnt_point: %s != %s\n".as_ptr(), sm_str(sm, (*sm).mnt_point), subdir.as_ptr());
        goto_out(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }
    if strcmp(root.as_ptr(), sm_str(sm, (*sm).mnt_root)) != 0 {
        ksft_test_result_fail(c"statmount returned incorrect mnt_root,statmount mnt_root: %s != %s\n".as_ptr(), sm_str(sm, (*sm).mnt_root), root.as_ptr());
        goto_out(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
        return;
    }

    ksft_test_result_pass(c"statmount by fd\n".as_ptr());
    goto_out(&mut sm, fd, tmproot.as_ptr(), subdir.as_ptr(), tmpdir.as_ptr());
}

unsafe fn goto_out(sm: &mut *mut statmount, fd: c_int, tmproot: *const c_char, subdir: *const c_char, tmpdir: *const c_char) {
    free(*sm as *mut c_void);
    close(fd);
    rmdir(tmproot);
    umount2(subdir, MNT_DETACH);
    rmdir(subdir);
    umount2(tmpdir, MNT_DETACH);
    rmdir(tmpdir);
}

unsafe fn goto_err_fd(sm: &mut *mut statmount, fd: c_int, tmproot: *const c_char, subdir: *const c_char, tmpdir: *const c_char) {
    close(fd);
    goto_err_tmproot(sm, fd, tmproot, subdir, tmpdir);
}

unsafe fn goto_err_tmproot(sm: &mut *mut statmount, _fd: c_int, tmproot: *const c_char, subdir: *const c_char, tmpdir: *const c_char) {
    rmdir(tmproot);
    goto_err_subdir(sm, -1, tmproot, subdir, tmpdir);
}

unsafe fn goto_err_subdir(sm: &mut *mut statmount, _fd: c_int, _tmproot: *const c_char, subdir: *const c_char, tmpdir: *const c_char) {
    umount2(subdir, MNT_DETACH);
    rmdir(subdir);
    goto_err_tmpdir(sm, -1, ptr::null(), subdir, tmpdir);
}

unsafe fn goto_err_tmpdir(_sm: &mut *mut statmount, _fd: c_int, _tmproot: *const c_char, _subdir: *const c_char, tmpdir: *const c_char) {
    umount2(tmpdir, MNT_DETACH);
    rmdir(tmpdir);
}

unsafe fn test_statmount_by_fd_unmounted() {
    let root: [c_char; 16] = *b"/test.unmounted\0";
    let mut tmpdir: [c_char; 22] = *b"/statmount.fd.XXXXXX\0";
    let mut subdir = [0 as c_char; PATH_MAX];
    let fd: c_int;
    let mut sm: *mut statmount = ptr::null_mut();

    if mkdtemp(tmpdir.as_mut_ptr()).is_null() {
        ksft_perror(c"mkdtemp".as_ptr());
        return;
    }

    if mount(c"statmount.test".as_ptr(), tmpdir.as_ptr(), c"tmpfs".as_ptr(), 0, ptr::null()) != 0 {
        ksft_perror(c"mount".as_ptr());
        rmdir(tmpdir.as_ptr());
        return;
    }

    snprintf(subdir.as_mut_ptr(), PATH_MAX, c"%s%s".as_ptr(), tmpdir.as_ptr(), root.as_ptr());

    if mkdir(subdir.as_ptr(), 0o755) != 0 {
        ksft_perror(c"mkdir".as_ptr());
        umount2(tmpdir.as_ptr(), MNT_DETACH);
        rmdir(tmpdir.as_ptr());
        return;
    }

    if mount(subdir.as_ptr(), subdir.as_ptr(), ptr::null(), MS_BIND, ptr::null()) != 0 {
        ksft_perror(c"mount".as_ptr());
        umount2(subdir.as_ptr(), MNT_DETACH);
        rmdir(subdir.as_ptr());
        umount2(tmpdir.as_ptr(), MNT_DETACH);
        rmdir(tmpdir.as_ptr());
        return;
    }

    fd = open(subdir.as_ptr(), O_PATH);
    if fd < 0 {
        ksft_perror(c"open".as_ptr());
        umount2(subdir.as_ptr(), MNT_DETACH);
        rmdir(subdir.as_ptr());
        umount2(tmpdir.as_ptr(), MNT_DETACH);
        rmdir(tmpdir.as_ptr());
        return;
    }

    if umount2(tmpdir.as_ptr(), MNT_DETACH) != 0 {
        ksft_perror(c"umount2".as_ptr());
        close(fd);
        umount2(subdir.as_ptr(), MNT_DETACH);
        rmdir(subdir.as_ptr());
        umount2(tmpdir.as_ptr(), MNT_DETACH);
        rmdir(tmpdir.as_ptr());
        return;
    }

    sm = statmount_alloc_by_fd(fd, STATMOUNT_MNT_POINT | STATMOUNT_MNT_ROOT);
    if sm.is_null() {
        ksft_test_result_fail(c"statmount by fd unmounted: %s\n".as_ptr(), strerror(errno));
        free(sm as *mut c_void);
        close(fd);
        umount2(subdir.as_ptr(), MNT_DETACH);
        rmdir(subdir.as_ptr());
        umount2(tmpdir.as_ptr(), MNT_DETACH);
        rmdir(tmpdir.as_ptr());
        return;
    }
    if (*sm).size as usize  < size_of::<statmount>() {
        ksft_test_result_fail(c"unexpected size: %u < %u\n".as_ptr(), (*sm).size, size_of::<statmount>() as uint32_t);
        free(sm as *mut c_void);
        close(fd);
        umount2(subdir.as_ptr(), MNT_DETACH);
        rmdir(subdir.as_ptr());
        umount2(tmpdir.as_ptr(), MNT_DETACH);
        rmdir(tmpdir.as_ptr());
        return;
    }
    if (*sm).mask & STATMOUNT_MNT_POINT != 0 {
        ksft_test_result_fail(c"STATMOUNT_MNT_POINT unexpectedly set in mask\n".as_ptr());
        free(sm as *mut c_void);
        close(fd);
        umount2(subdir.as_ptr(), MNT_DETACH);
        rmdir(subdir.as_ptr());
        umount2(tmpdir.as_ptr(), MNT_DETACH);
        rmdir(tmpdir.as_ptr());
        return;
    }
    if (*sm).mask & STATMOUNT_MNT_ROOT == 0 {
        ksft_test_result_fail(c"STATMOUNT_MNT_ROOT not set in mask\n".as_ptr());
        free(sm as *mut c_void);
        close(fd);
        umount2(subdir.as_ptr(), MNT_DETACH);
        rmdir(subdir.as_ptr());
        umount2(tmpdir.as_ptr(), MNT_DETACH);
        rmdir(tmpdir.as_ptr());
        return;
    }
    if strcmp(sm_str(sm, (*sm).mnt_root), root.as_ptr()) != 0 {
        ksft_test_result_fail(c"statmount returned incorrect mnt_root,statmount mnt_root: %s != %s\n".as_ptr(), sm_str(sm, (*sm).mnt_root), root.as_ptr());
        free(sm as *mut c_void);
        close(fd);
        umount2(subdir.as_ptr(), MNT_DETACH);
        rmdir(subdir.as_ptr());
        umount2(tmpdir.as_ptr(), MNT_DETACH);
        rmdir(tmpdir.as_ptr());
        return;
    }

    ksft_test_result_pass(c"statmount by fd on unmounted mount\n".as_ptr());
    free(sm as *mut c_void);
    close(fd);
    umount2(subdir.as_ptr(), MNT_DETACH);
    rmdir(subdir.as_ptr());
    umount2(tmpdir.as_ptr(), MNT_DETACH);
    rmdir(tmpdir.as_ptr());
}

macro_rules! str_off {
    ($memb:ident) => {{
        let uninit = core::mem::MaybeUninit::<statmount>::uninit();
        let base = uninit.as_ptr();
        unsafe { (&raw const (*base).$memb as usize - base as usize) / size_of::<uint32_t>() }
    }};
}

fn main() {
    unsafe {
        let mut ret: c_int;
        let all_mask: uint64_t = STATMOUNT_SB_BASIC | STATMOUNT_MNT_BASIC |
            STATMOUNT_PROPAGATE_FROM | STATMOUNT_MNT_ROOT |
            STATMOUNT_MNT_POINT | STATMOUNT_FS_TYPE | STATMOUNT_MNT_NS_ID;

        ksft_print_header();

        ret = statmount(0, 0, 0, 0, ptr::null_mut(), 0, 0);
        assert!(ret == -1);
        if errno == ENOSYS {
            ksft_exit_skip(c"statmount() syscall not supported\n".as_ptr());
        }

        setup_namespace();

        ksft_set_plan(17);
        test_listmount_empty_root();
        test_statmount_zero_mask();
        test_statmount_mnt_basic();
        test_statmount_sb_basic();
        test_statmount_mnt_root();
        test_statmount_mnt_point();
        test_statmount_fs_type();
        test_statmount_mnt_opts();
        test_statmount_string(STATMOUNT_MNT_ROOT, str_off!(mnt_root), c"mount root".as_ptr());
        test_statmount_string(STATMOUNT_MNT_POINT, str_off!(mnt_point), c"mount point".as_ptr());
        test_statmount_string(STATMOUNT_FS_TYPE, str_off!(fs_type), c"fs type".as_ptr());
        test_statmount_string(all_mask, str_off!(mnt_root), c"mount root & all".as_ptr());
        test_statmount_string(all_mask, str_off!(mnt_point), c"mount point & all".as_ptr());
        test_statmount_string(all_mask, str_off!(fs_type), c"fs type & all".as_ptr());

        test_listmount_tree();
        test_statmount_by_fd_unmounted();
        test_statmount_by_fd();

        if ksft_get_fail_cnt() + ksft_get_error_cnt() > 0 {
            ksft_exit_fail();
        } else {
            ksft_exit_pass();
        }
    }
}
