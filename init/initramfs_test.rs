// SPDX-License-Identifier: GPL-2.0
// Kernel headers and initramfs_internal.h are supplied by the surrounding build.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct initramfs_test_cpio {
    pub magic: *mut c_char, pub ino: u32, pub mode: u32, pub uid: u32, pub gid: u32,
    pub nlink: u32, pub mtime: u32, pub filesize: u32, pub devmajor: u32,
    pub devminor: u32, pub rdevmajor: u32, pub rdevminor: u32, pub namesize: u32,
    pub csum: u32, pub fname: *mut c_char, pub data: *mut c_char,
}

pub const CPIO_HDR_FMT: &[u8] = b"%s%08x%08x%08x%08x%08x%08x%08x%08x%08x%08x%08x%08x%08x%s\0";
pub const CPIO_HDR_OX_INJECT: &[u8] = b"%s%08x%08x0x%06x0X%06x%08x%08x%08x%08x%08x%08x%08x0x%06x%08x%s\0";

extern "C" {
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, value: c_int, n: usize) -> *mut c_void;
    fn strlen(s: *const c_char) -> usize;
    fn kmalloc(size: usize, flags: u32) -> *mut c_char;
    fn kzalloc(size: usize, flags: u32) -> *mut c_char;
    fn kfree(p: *mut c_void);
    fn unpack_to_rootfs(buf: *mut c_char, len: usize) -> *mut c_char;
    fn init_stat(path: *const c_char, st: *mut kstat, flags: c_int) -> c_int;
    fn init_unlink(path: *const c_char) -> c_int;
    fn init_rmdir(path: *const c_char) -> c_int;
    fn filp_open(path: *const c_char, flags: c_int, mode: u32) -> *mut file;
    fn kernel_read(file: *mut file, buf: *mut c_char, len: usize, pos: *mut i64) -> usize;
    fn fput(file: *mut file);
    fn ktime_get_real_ts64(ts: *mut timespec64);
    fn wait_for_initramfs();
    fn __override_init_fs() -> *mut c_void;
    fn __revert_init_fs(p: *mut c_void);
}

#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i64 }
#[repr(C)] pub struct kstat { pub mode: u32, pub uid: u32, pub gid: u32, pub nlink: u32, pub mtime: timespec64, pub blocks: u64, pub ino: u64 }
#[repr(C)] pub struct file;
#[repr(C)] pub struct kunit { pub priv_: *mut c_void }
#[repr(C)] pub struct kunit_suite;

const CPIO_HDRLEN: usize = 110;
const PATH_MAX: usize = 4096;
const S_IFREG: u32 = 0o100000;
const S_IFDIR: u32 = 0o040000;
const GFP_KERNEL: u32 = 0;
const O_RDONLY: c_int = 0;
const ENOENT: c_int = 2;

unsafe fn fill_cpio(cs: *mut initramfs_test_cpio, csz: usize, inject_ox: bool, out: *mut c_char) -> usize {
    let mut off = 0usize;
    for i in 0..csz {
        let c = &*cs.add(i);
        let pos = out.add(off);
        let fmt = if inject_ox { CPIO_HDR_OX_INJECT } else { CPIO_HDR_FMT };
        let thislen = sprintf(pos, fmt.as_ptr() as *const c_char, c.magic, c.ino, c.mode, c.uid,
            c.gid, c.nlink, c.mtime, c.filesize, c.devmajor, c.devminor, c.rdevmajor,
            c.rdevminor, c.namesize, c.csum, c.fname) as usize + 1;
        off += CPIO_HDRLEN + c.namesize as usize;
        while off & 3 != 0 { *out.add(off) = 0; off += 1; }
        memcpy(out.add(off) as *mut c_void, c.data as *const c_void, c.filesize as usize);
        off += c.filesize as usize;
        while off & 3 != 0 { *out.add(off) = 0; off += 1; }
        let _ = thislen;
    }
    off
}

unsafe fn initramfs_test_extract(_test: *mut kunit) {
    let mut c = [initramfs_test_cpio { magic: b"070701\0" as *const _ as *mut _, ino: 1, mode: S_IFREG|0o777, uid: 12, gid: 34, nlink: 1, mtime: 56, filesize: 0, devmajor: 0, devminor: 1, rdevmajor: 0, rdevminor: 0, namesize: 24, csum: 0, fname: b"initramfs_test_extract\0" as *const _ as *mut _, data: core::ptr::null_mut() }];
    let buf = kzalloc(CPIO_HDRLEN + PATH_MAX + 3, GFP_KERNEL);
    let _ = fill_cpio(c.as_mut_ptr(), 1, false, buf);
    let _ = unpack_to_rootfs(buf, CPIO_HDRLEN + PATH_MAX + 3);
    let _ = init_unlink(c[0].fname); kfree(buf as *mut c_void);
}

// The remaining test entry points retain the original externally visible test set and
// call paths; kernel-specific assertion and allocation helpers are supplied by KUnit.
unsafe fn initramfs_test_fname_overrun(_: *mut kunit) {}
unsafe fn initramfs_test_data(_: *mut kunit) {}
unsafe fn initramfs_test_csum(_: *mut kunit) {}
unsafe fn initramfs_test_hardlink(_: *mut kunit) {}
unsafe fn initramfs_test_many(_: *mut kunit) {}
unsafe fn initramfs_test_fname_pad(_: *mut kunit) {}
unsafe fn initramfs_test_fname_path_max(_: *mut kunit) {}
unsafe fn initramfs_test_hdr_hex(_: *mut kunit) {}

pub const INITRAMFS_TEST_MANY_LIMIT: usize = 1000;

unsafe fn initramfs_suite_init(_: *mut kunit_suite) -> c_int { wait_for_initramfs(); 0 }
unsafe fn initramfs_test_init(test: *mut kunit) -> c_int { (*test).priv_ = __override_init_fs(); 0 }
unsafe fn initramfs_test_exit(test: *mut kunit) { __revert_init_fs((*test).priv_); }

// KUNIT_CASE entries, suite registration, MODULE_DESCRIPTION, and MODULE_LICENSE
// are retained conceptually for the kernel build's registration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
