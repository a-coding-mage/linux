// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2024 Michael Jeanson <mjeanson@efficios.com>

// C source defined _GNU_SOURCE and included:
// <assert.h>, <stdint.h>, <syscall.h>, <string.h>, <unistd.h>, and "rseq.h".

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rseq_abi {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static RSEQ_SIG: u32;
    static RSEQ_ABI_FLAG_UNREGISTER: c_int;

    fn syscall(num: c_long, ...) -> c_long;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn strerrorname_np(errnum: c_int) -> *mut c_char;
    fn rseq_get_abi() -> *mut rseq_abi;
    fn rseq_available() -> c_int;
    fn __errno_location() -> *mut c_int;
}

const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const EBUSY: c_int = 16;
const EPERM: c_int = 1;

unsafe fn sys_rseq(
    rseq_abi: *mut c_void,
    rseq_len: u32,
    flags: c_int,
    sig: u32,
) -> c_int {
    syscall(libc::SYS_rseq as c_long, rseq_abi, rseq_len, flags, sig) as c_int
}

unsafe fn errno_set(value: c_int) {
    *__errno_location() = value;
}

unsafe fn errno_get() -> c_int {
    *__errno_location()
}

/*
 * Check the value of errno on some expected failures of the rseq syscall.
 */

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    let global_rseq: *mut rseq_abi = rseq_get_abi();
    let mut ret: c_int;
    let mut errno_copy: c_int;

    if rseq_available() == 0 {
        fprintf(
            stderr,
            b"rseq syscall unavailable\0".as_ptr() as *const c_char,
        );
        return -1;
    }

    /* The current thread is NOT registered. */

    /* EINVAL */
    errno_set(0);
    ret = sys_rseq(global_rseq as *mut c_void, 32, -1, RSEQ_SIG);
    errno_copy = errno_get();
    fprintf(
        stderr,
        b"Registration with invalid flag fails with errno set to EINVAL (ret = %d, errno = %s)\n\0"
            .as_ptr() as *const c_char,
        ret,
        strerrorname_np(errno_copy),
    );
    if ret == 0 || errno_copy != EINVAL {
        return -1;
    }

    errno_set(0);
    ret = sys_rseq(
        (global_rseq as *mut c_char).add(1) as *mut c_void,
        32,
        0,
        RSEQ_SIG,
    );
    errno_copy = errno_get();
    fprintf(
        stderr,
        b"Registration with unaligned rseq_abi fails with errno set to EINVAL (ret = %d, errno = %s)\n\0"
            .as_ptr() as *const c_char,
        ret,
        strerrorname_np(errno_copy),
    );
    if ret == 0 || errno_copy != EINVAL {
        return -1;
    }

    errno_set(0);
    ret = sys_rseq(global_rseq as *mut c_void, 31, 0, RSEQ_SIG);
    errno_copy = errno_get();
    fprintf(
        stderr,
        b"Registration with invalid size fails with errno set to EINVAL (ret = %d, errno = %s)\n\0"
            .as_ptr() as *const c_char,
        ret,
        strerrorname_np(errno_copy),
    );
    if ret == 0 || errno_copy != EINVAL {
        return -1;
    }

    /*
     * C condition preserved:
     * #if defined(__LP64__) && (!defined(__s390__) && !defined(__s390x__))
     *
     * We haven't found a reliable way to find an invalid address when
     * running a 32bit userspace on a 64bit kernel, so only run this test
     * on 64bit builds for the moment.
     *
     * Also exclude architectures that select
     * CONFIG_ALTERNATE_USER_ADDRESS_SPACE where the kernel and userspace
     * have their own address space and this failure can't happen.
     */
    #[cfg(all(
        target_pointer_width = "64",
        not(any(target_arch = "s390x", target_arch = "s390"))
    ))]
    {
        /* EFAULT */
        errno_set(0);
        ret = sys_rseq((-4096isize) as *mut c_void, 32, 0, RSEQ_SIG);
        errno_copy = errno_get();
        fprintf(
            stderr,
            b"Registration with invalid address fails with errno set to EFAULT (ret = %d, errno = %s)\n\0"
                .as_ptr() as *const c_char,
            ret,
            strerrorname_np(errno_copy),
        );
        if ret == 0 || errno_copy != EFAULT {
            return -1;
        }
    }

    errno_set(0);
    ret = sys_rseq(global_rseq as *mut c_void, 32, 0, RSEQ_SIG);
    errno_copy = errno_get();
    fprintf(
        stderr,
        b"Registration succeeds for the current thread (ret = %d, errno = %s)\n\0"
            .as_ptr() as *const c_char,
        ret,
        strerrorname_np(errno_copy),
    );
    if ret != 0 && errno_get() != 0 {
        return -1;
    }

    /* The current thread is registered. */

    /* EBUSY */
    errno_set(0);
    ret = sys_rseq(global_rseq as *mut c_void, 32, 0, RSEQ_SIG);
    errno_copy = errno_get();
    fprintf(
        stderr,
        b"Double registration fails with errno set to EBUSY (ret = %d, errno = %s)\n\0"
            .as_ptr() as *const c_char,
        ret,
        strerrorname_np(errno_copy),
    );
    if ret == 0 || errno_copy != EBUSY {
        return -1;
    }

    /* EPERM */
    errno_set(0);
    ret = sys_rseq(
        global_rseq as *mut c_void,
        32,
        RSEQ_ABI_FLAG_UNREGISTER,
        RSEQ_SIG.wrapping_add(1),
    );
    errno_copy = errno_get();
    fprintf(
        stderr,
        b"Unregistration with wrong RSEQ_SIG fails with errno to EPERM (ret = %d, errno = %s)\n\0"
            .as_ptr() as *const c_char,
        ret,
        strerrorname_np(errno_copy),
    );
    if ret == 0 || errno_copy != EPERM {
        return -1;
    }

    errno_set(0);
    ret = sys_rseq(
        global_rseq as *mut c_void,
        32,
        RSEQ_ABI_FLAG_UNREGISTER,
        RSEQ_SIG,
    );
    errno_copy = errno_get();
    fprintf(
        stderr,
        b"Unregistration succeeds for the current thread (ret = %d, errno = %s)\n\0"
            .as_ptr() as *const c_char,
        ret,
        strerrorname_np(errno_copy),
    );
    if ret != 0 {
        return -1;
    }

    errno_set(0);
    ret = sys_rseq(
        global_rseq as *mut c_void,
        32,
        RSEQ_ABI_FLAG_UNREGISTER,
        RSEQ_SIG,
    );
    errno_copy = errno_get();
    fprintf(
        stderr,
        b"Double unregistration fails with errno set to EINVAL (ret = %d, errno = %s)\n\0"
            .as_ptr() as *const c_char,
        ret,
        strerrorname_np(errno_copy),
    );
    if ret == 0 || errno_copy != EINVAL {
        return -1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
