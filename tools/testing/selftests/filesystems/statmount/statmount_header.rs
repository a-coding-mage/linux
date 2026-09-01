/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: <errno.h>, <stdint.h>, <stdlib.h>,
// <linux/mount.h>, <asm/unistd.h>.

use core::ffi::{c_int, c_long, c_uint, c_void};

pub const STATMOUNT_BUFSIZE: usize = 1 << 15;

// Fallback syscall numbers used when the C headers do not define them.
#[cfg(target_arch = "alpha")]
pub const __NR_statmount: c_long = 567;
#[cfg(all(
    any(target_arch = "mips", target_arch = "mips64"),
    target_pointer_width = "32"
))]
pub const __NR_statmount: c_long = 4457; /* o32 */
#[cfg(all(target_arch = "mips64", target_pointer_width = "64"))]
pub const __NR_statmount: c_long = 5457; /* n64 */
#[cfg(not(any(target_arch = "alpha", target_arch = "mips", target_arch = "mips64")))]
pub const __NR_statmount: c_long = 457;

#[cfg(target_arch = "alpha")]
pub const __NR_listmount: c_long = 568;
#[cfg(all(
    any(target_arch = "mips", target_arch = "mips64"),
    target_pointer_width = "32"
))]
pub const __NR_listmount: c_long = 4458; /* o32 */
#[cfg(all(target_arch = "mips64", target_pointer_width = "64"))]
pub const __NR_listmount: c_long = 5458; /* n64 */
#[cfg(not(any(target_arch = "alpha", target_arch = "mips", target_arch = "mips64")))]
pub const __NR_listmount: c_long = 458;

// The C source distinguishes MIPS o32, n32, and n64 using _MIPS_SIM. Rust cfg
// does not expose that exact preprocessor value here; the n32 syscall numbers
// are therefore preserved only by this note:
// __NR_statmount = 6457 and __NR_listmount = 6458 for _MIPS_SIM_NABI32.

unsafe extern "C" {
    pub fn syscall(num: c_long, ...) -> c_long;
    pub fn malloc(size: usize) -> *mut c_void;
    pub fn free(ptr: *mut c_void);

    // errno is a C macro. This declaration matches the common libc accessor
    // used by the original errno checks.
    pub fn __errno_location() -> *mut c_int;
}

#[inline]
pub unsafe fn statmount(
    mnt_id: u64,
    mnt_ns_id: u64,
    fd: u32,
    mask: u64,
    buf: *mut statmount,
    bufsize: usize,
    flags: c_uint,
) -> c_int {
    let mut req: mnt_id_req = unsafe { core::mem::zeroed() };
    req.size = MNT_ID_REQ_SIZE_VER0;
    req.param = mask;

    if flags & STATMOUNT_BY_FD != 0 {
        req.size = MNT_ID_REQ_SIZE_VER1;
        req.mnt_fd = fd;
    } else {
        req.mnt_id = mnt_id;
        if mnt_ns_id != 0 {
            req.size = MNT_ID_REQ_SIZE_VER1;
            req.mnt_ns_id = mnt_ns_id;
        }
    }

    unsafe { syscall(__NR_statmount, &mut req as *mut mnt_id_req, buf, bufsize, flags) as c_int }
}

#[inline]
pub unsafe fn listmount(
    mnt_id: u64,
    mnt_ns_id: u64,
    last_mnt_id: u64,
    list: *mut u64,
    num: usize,
    flags: c_uint,
) -> isize {
    let mut req: mnt_id_req = unsafe { core::mem::zeroed() };
    req.size = MNT_ID_REQ_SIZE_VER0;
    req.mnt_id = mnt_id;
    req.param = last_mnt_id;

    if mnt_ns_id != 0 {
        req.size = MNT_ID_REQ_SIZE_VER1;
        req.mnt_ns_id = mnt_ns_id;
    }

    unsafe { syscall(__NR_listmount, &mut req as *mut mnt_id_req, list, num, flags) as isize }
}

#[inline]
pub unsafe fn statmount_alloc(
    mnt_id: u64,
    mnt_ns_id: u64,
    mask: u64,
    flags: c_uint,
) -> *mut statmount {
    let mut buf: *mut statmount;
    let mut bufsize: usize = STATMOUNT_BUFSIZE;
    let mut ret: c_int;

    loop {
        buf = unsafe { malloc(bufsize) as *mut statmount };
        if buf.is_null() {
            return core::ptr::null_mut();
        }

        ret = unsafe { statmount(mnt_id, mnt_ns_id, 0, mask, buf, bufsize, flags) };
        if ret == 0 {
            return buf;
        }

        unsafe { free(buf as *mut c_void) };
        if unsafe { *__errno_location() } != EOVERFLOW {
            return core::ptr::null_mut();
        }

        bufsize <<= 1;
    }
}

#[inline]
pub unsafe fn statmount_alloc_by_fd(fd: c_int, mask: u64) -> *mut statmount {
    let mut buf: *mut statmount;
    let mut bufsize: usize = STATMOUNT_BUFSIZE;
    let mut ret: c_int;

    loop {
        buf = unsafe { malloc(bufsize) as *mut statmount };
        if buf.is_null() {
            return core::ptr::null_mut();
        }

        ret = unsafe { statmount(0, 0, fd as u32, mask, buf, bufsize, STATMOUNT_BY_FD) };
        if ret == 0 {
            return buf;
        }

        unsafe { free(buf as *mut c_void) };
        if unsafe { *__errno_location() } != EOVERFLOW {
            return core::ptr::null_mut();
        }

        bufsize <<= 1;
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
