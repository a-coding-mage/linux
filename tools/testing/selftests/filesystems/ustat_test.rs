// SPDX-License-Identifier: GPL-2.0
/*
 * Test ustat(2): looking up superblocks by device number.
 *
 * ustat() resolves a device number to a mounted superblock via
 * user_get_super(). Check that the device number of a mounted tmpfs (an
 * anonymous device) resolves, that it stops resolving once the filesystem
 * is unmounted and that bogus device numbers report EINVAL.
 */

// C dependencies:
// errno.h, fcntl.h, sched.h, stdio.h, stdlib.h, string.h, sys/mount.h,
// sys/stat.h, sys/syscall.h, unistd.h, and ../kselftest_harness.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

type ssize_t = isize;
type uid_t = c_uint;
type gid_t = c_uint;

extern "C" {
    static mut errno: c_int;

    fn syscall(num: c_long, ...) -> c_long;
    fn strlen(s: *const c_char) -> usize;
    fn open(path: *const c_char, oflag: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn getuid() -> uid_t;
    fn getgid() -> gid_t;
    fn unshare(flags: c_int) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn rmdir(path: *const c_char) -> c_int;
    fn stat(path: *const c_char, statbuf: *mut stat) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn umount(target: *const c_char) -> c_int;
}

const O_WRONLY: c_int = 1;
const CLONE_NEWNS: c_int = 0x0002_0000;
const CLONE_NEWUSER: c_int = 0x1000_0000;
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
const EINVAL: c_int = 22;

// The original C file conditionally compiles this section with `#ifdef
// __NR_ustat`; keep the same intent with a Rust cfg name supplied by the build.
#[cfg(__NR_ustat)]
const __NR_ustat: c_long = __NR_ustat as c_long;

/* struct ustat is not exported through UAPI, mirror include/linux/types.h. */
#[repr(C)]
struct ustat_buf {
    f_tfree: c_int,
    f_tinode: c_ulong,
    f_fname: [c_char; 6],
    f_fpack: [c_char; 6],
    /* slack in case an architecture lays the struct out differently */
    pad: [c_char; 64],
}

#[repr(C)]
struct stat {
    st_dev: c_ulong,
}

#[cfg(__NR_ustat)]
/*
 * The kernel decodes @dev with new_decode_dev(), which matches the low 32
 * bits of the st_dev encoding stat(2) returns for any major below 4096.
 */
unsafe fn sys_ustat(dev: c_uint, buf: *mut ustat_buf) -> c_int {
    syscall(__NR_ustat, dev, buf) as c_int
}

#[cfg(__NR_ustat)]
unsafe fn write_string(path: *const c_char, string: *const c_char) -> c_int {
    let len: ssize_t = strlen(string) as ssize_t;
    let fd: c_int;

    fd = open(path, O_WRONLY);
    if fd < 0 {
        return -1;
    }
    if write(fd, string as *const c_void, len as usize) != len {
        close(fd);
        return -1;
    }
    close(fd)
}

#[cfg(__NR_ustat)]
/* Enter namespaces in which mounting a tmpfs instance is allowed. */
unsafe fn setup_namespaces() -> c_int {
    let uid: uid_t = getuid();
    let gid: gid_t = getgid();
    let mut map: [c_char; 64] = [0; 64];

    if unshare(CLONE_NEWNS | if uid != 0 { CLONE_NEWUSER } else { 0 }) != 0 {
        return -1;
    }

    if uid != 0 {
        if write_string(
            b"/proc/self/setgroups\0".as_ptr() as *const c_char,
            b"deny\0".as_ptr() as *const c_char,
        ) != 0
        {
            return -1;
        }
        snprintf(
            map.as_mut_ptr(),
            core::mem::size_of_val(&map),
            b"0 %d 1\0".as_ptr() as *const c_char,
            uid,
        );
        if write_string(
            b"/proc/self/uid_map\0".as_ptr() as *const c_char,
            map.as_ptr(),
        ) != 0
        {
            return -1;
        }
        snprintf(
            map.as_mut_ptr(),
            core::mem::size_of_val(&map),
            b"0 %d 1\0".as_ptr() as *const c_char,
            gid,
        );
        if write_string(
            b"/proc/self/gid_map\0".as_ptr() as *const c_char,
            map.as_ptr(),
        ) != 0
        {
            return -1;
        }
    }

    mount(
        core::ptr::null(),
        b"/\0".as_ptr() as *const c_char,
        core::ptr::null(),
        MS_REC | MS_PRIVATE,
        core::ptr::null(),
    )
}

#[cfg(__NR_ustat)]
unsafe fn resolves_mounted_superblock() {
    let mut dir = *b"/tmp/ustat_test.XXXXXX\0";
    let mut ub: ustat_buf = core::mem::zeroed();
    let mut st: stat = core::mem::zeroed();

    ASSERT_NE!(core::ptr::null_mut::<c_char>(), mkdtemp(dir.as_mut_ptr() as *mut c_char));

    if setup_namespaces() != 0 {
        rmdir(dir.as_ptr() as *const c_char);
        SKIP!(
            return,
            "cannot set up namespaces: %s",
            strerror(errno)
        );
    }

    ASSERT_EQ!(
        0,
        mount(
            b"ustat_test\0".as_ptr() as *const c_char,
            dir.as_ptr() as *const c_char,
            b"tmpfs\0".as_ptr() as *const c_char,
            0,
            core::ptr::null(),
        )
    );
    ASSERT_EQ!(0, stat(dir.as_ptr() as *const c_char, &mut st));

    memset(
        &mut ub as *mut ustat_buf as *mut c_void,
        0xff,
        core::mem::size_of_val(&ub),
    );
    ASSERT_EQ!(0, sys_ustat(st.st_dev as c_uint, &mut ub))
        .TH_LOG(
            "ustat(%u): %s",
            st.st_dev as c_uint,
            strerror(errno),
        );

    ASSERT_EQ!(0, umount(dir.as_ptr() as *const c_char));

    /* The unmount removed the superblock, the device is gone. */
    ASSERT_EQ!(-1, sys_ustat(st.st_dev as c_uint, &mut ub));
    ASSERT_EQ!(EINVAL, errno);

    rmdir(dir.as_ptr() as *const c_char);
}

#[cfg(__NR_ustat)]
unsafe fn bogus_device_numbers() {
    let mut ub: ustat_buf = core::mem::zeroed();

    ASSERT_EQ!(-1, sys_ustat(0, &mut ub));
    ASSERT_EQ!(EINVAL, errno);

    /* major 4095, minor 1048575: nothing plausible lives there */
    ASSERT_EQ!(
        -1,
        sys_ustat((0xfffu32 << 8) | 0xffu32 | (0xfff00u32 << 12), &mut ub)
    );
    ASSERT_EQ!(EINVAL, errno);
}

#[cfg(not(__NR_ustat))]
unsafe fn unsupported() {
    SKIP!(
        return,
        "ustat(2) is not available on this architecture"
    );
}

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
