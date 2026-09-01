/* SPDX-License-Identifier: GPL-2.0 */

// Translated from testing/selftests/user_events/user_events_selftests.h.
// C include dependencies: sys/stat.h, sys/types.h, sys/mount.h, unistd.h,
// errno.h, and kselftest.h.

use core::ptr;

unsafe extern "C" {
    fn stat(path: *const libc::c_char, buf: *mut libc::stat) -> libc::c_int;
    fn mount(
        source: *const libc::c_char,
        target: *const libc::c_char,
        filesystemtype: *const libc::c_char,
        mountflags: libc::c_ulong,
        data: *const libc::c_void,
    ) -> libc::c_int;
    fn umount(target: *const libc::c_char) -> libc::c_int;
    fn getuid() -> libc::uid_t;
    fn __errno_location() -> *mut libc::c_int;
}

#[inline]
pub unsafe fn tracefs_unmount() {
    unsafe {
        umount(c"/sys/kernel/tracing".as_ptr());
    }
}

#[inline]
pub unsafe fn tracefs_enabled(
    message: *mut *mut libc::c_char,
    fail: *mut bool,
    umount_out: *mut bool,
) -> bool {
    let mut buf: libc::stat = unsafe { core::mem::zeroed() };
    let mut ret: libc::c_int;

    unsafe {
        *message = c"".as_ptr() as *mut libc::c_char;
        *fail = false;
        *umount_out = false;
    }

    /* Ensure tracefs is installed */
    ret = unsafe { stat(c"/sys/kernel/tracing".as_ptr(), &mut buf) };

    if ret == -1 {
        unsafe {
            *message = c"Tracefs is not installed".as_ptr() as *mut libc::c_char;
        }
        return false;
    }

    /* Ensure mounted tracefs */
    ret = unsafe { stat(c"/sys/kernel/tracing/README".as_ptr(), &mut buf) };

    if ret == -1 && unsafe { *__errno_location() } == libc::ENOENT {
        if unsafe {
            mount(
                ptr::null(),
                c"/sys/kernel/tracing".as_ptr(),
                c"tracefs".as_ptr(),
                0,
                ptr::null(),
            )
        } != 0
        {
            unsafe {
                *message = c"Cannot mount tracefs".as_ptr() as *mut libc::c_char;
                *fail = true;
            }
            return false;
        }

        unsafe {
            *umount_out = true;
        }

        ret = unsafe { stat(c"/sys/kernel/tracing/README".as_ptr(), &mut buf) };
    }

    if ret == -1 {
        unsafe {
            *message = c"Cannot access tracefs".as_ptr() as *mut libc::c_char;
            *fail = true;
        }
        return false;
    }

    true
}

#[inline]
pub unsafe fn user_events_enabled(
    message: *mut *mut libc::c_char,
    fail: *mut bool,
    umount_out: *mut bool,
) -> bool {
    let mut buf: libc::stat = unsafe { core::mem::zeroed() };
    let ret: libc::c_int;

    unsafe {
        *message = c"".as_ptr() as *mut libc::c_char;
        *fail = false;
        *umount_out = false;
    }

    if unsafe { getuid() } != 0 {
        unsafe {
            *message = c"Must be run as root".as_ptr() as *mut libc::c_char;
            *fail = true;
        }
        return false;
    }

    if !unsafe { tracefs_enabled(message, fail, umount_out) } {
        return false;
    }

    /* Ensure user_events is installed */
    ret = unsafe { stat(c"/sys/kernel/tracing/user_events_data".as_ptr(), &mut buf) };

    if ret == -1 {
        match unsafe { *__errno_location() } {
            libc::ENOENT => {
                unsafe {
                    *message = c"user_events is not installed".as_ptr() as *mut libc::c_char;
                }
                return false;
            }

            _ => {
                unsafe {
                    *message = c"Cannot access user_events_data".as_ptr() as *mut libc::c_char;
                    *fail = true;
                }
                return false;
            }
        }
    }

    true
}

macro_rules! USER_EVENT_FIXTURE_SETUP {
    ($statement:expr, $umount:expr) => {{
        let mut message: *mut libc::c_char = core::ptr::null_mut();
        let mut fail: bool = false;
        if !unsafe { user_events_enabled(&mut message, &mut fail, &mut ($umount)) } {
            if fail {
                TH_LOG!(c"Setup failed due to: %s".as_ptr(), message);
                ASSERT_FALSE!(fail);
            }
            SKIP!($statement, c"Skipping due to: %s".as_ptr(), message);
        }
    }};
}

macro_rules! USER_EVENT_FIXTURE_TEARDOWN {
    ($umount:expr) => {{
        if $umount {
            unsafe {
                tracefs_unmount();
            }
        }
    }};
}

pub(crate) use USER_EVENT_FIXTURE_SETUP;
pub(crate) use USER_EVENT_FIXTURE_TEARDOWN;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
