// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_ulong, c_void};

// C headers and the Linux binder/binderfs headers are external dependencies.
const CLONE_NEWNS: c_int = 0x0002_0000;
const MS_REC: c_ulong = 0x4000;
const MS_PRIVATE: c_ulong = 1 << 18;
const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2_000_000;
const EEXIST: c_int = 17;
const EXIT_FAILURE: c_int = 1;
const EXIT_SUCCESS: c_int = 0;
const BINDER_CTL_ADD: c_ulong = 0x4004_6201;

#[repr(C)]
pub struct binderfs_device {
    pub name: [c_char; 256],
    pub major: u32,
    pub minor: u32,
}

unsafe extern "C" {
    fn unshare(flags: c_int) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn mkdir(pathname: *const c_char, mode: u32) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn strerror(errnum: c_int) -> *const c_char;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn __errno_location() -> *mut c_int;
    static mut stderr: *mut c_void;
}

pub unsafe fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut fd: c_int;
    let mut ret: c_int;
    let mut saved_errno: c_int;
    let mut device: binderfs_device = core::mem::zeroed();

    ret = unshare(CLONE_NEWNS);
    if ret < 0 {
        fprintf(stderr, b"%s - Failed to unshare mount namespace\n\0".as_ptr() as *const c_char, strerror(*__errno_location()));
        exit(EXIT_FAILURE);
    }

    ret = mount(core::ptr::null(), b"/\0".as_ptr() as *const c_char, core::ptr::null(), MS_REC | MS_PRIVATE, core::ptr::null());
    if ret < 0 {
        fprintf(stderr, b"%s - Failed to mount / as private\n\0".as_ptr() as *const c_char, strerror(*__errno_location()));
        exit(EXIT_FAILURE);
    }

    ret = mkdir(b"/dev/binderfs\0".as_ptr() as *const c_char, 0o755);
    if ret < 0 && *__errno_location() != EEXIST {
        fprintf(stderr, b"%s - Failed to create binderfs mountpoint\n\0".as_ptr() as *const c_char, strerror(*__errno_location()));
        exit(EXIT_FAILURE);
    }

    ret = mount(core::ptr::null(), b"/dev/binderfs\0".as_ptr() as *const c_char, b"binder\0".as_ptr() as *const c_char, 0, core::ptr::null());
    if ret < 0 {
        fprintf(stderr, b"%s - Failed to mount binderfs\n\0".as_ptr() as *const c_char, strerror(*__errno_location()));
        exit(EXIT_FAILURE);
    }

    memcpy(device.name.as_mut_ptr() as *mut c_void, b"my-binder\0".as_ptr() as *const c_void, strlen(b"my-binder\0".as_ptr() as *const c_char));

    fd = open(b"/dev/binderfs/binder-control\0".as_ptr() as *const c_char, O_RDONLY | O_CLOEXEC);
    if fd < 0 {
        fprintf(stderr, b"%s - Failed to open binder-control device\n\0".as_ptr() as *const c_char, strerror(*__errno_location()));
        exit(EXIT_FAILURE);
    }

    ret = ioctl(fd, BINDER_CTL_ADD, &mut device);
    saved_errno = *__errno_location();
    close(fd);
    *__errno_location() = saved_errno;
    if ret < 0 {
        fprintf(stderr, b"%s - Failed to allocate new binder device\n\0".as_ptr() as *const c_char, strerror(*__errno_location()));
        exit(EXIT_FAILURE);
    }

    printf(b"Allocated new binder device with major %d, minor %d, and name %s\n\0".as_ptr() as *const c_char, device.major, device.minor, device.name.as_ptr());

    ret = unlink(b"/dev/binderfs/my-binder\0".as_ptr() as *const c_char);
    if ret < 0 {
        fprintf(stderr, b"%s - Failed to delete binder device\n\0".as_ptr() as *const c_char, strerror(*__errno_location()));
        exit(EXIT_FAILURE);
    }

    /* Cleanup happens when the mount namespace dies. */
    exit(EXIT_SUCCESS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
