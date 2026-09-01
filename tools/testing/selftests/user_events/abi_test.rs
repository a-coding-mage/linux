// SPDX-License-Identifier: GPL-2.0
/*
 * User Events ABI Test Program
 *
 * Copyright (c) 2022 Beau Belgrave <beaub@linux.microsoft.com>
 */

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_void};
use std::ptr;

use libc;

type __u64 = u64;

type CloneFunc = extern "C" fn(*mut c_void) -> c_int;

#[repr(C)]
struct user_reg {
    size: c_uint,
    name_args: __u64,
    flags: c_uint,
    enable_bit: c_uint,
    enable_addr: __u64,
    enable_size: c_uint,
    _pad: c_uint,
}

#[repr(C)]
struct user_unreg {
    size: c_uint,
    disable_bit: c_uint,
    disable_addr: __u64,
    _pad: c_uint,
}

// These ioctl request codes and registration flags come from linux/user_events.h.
// Exact numeric values are provided by that header in the target build environment.
#[allow(dead_code)]
const DIAG_IOCSDEL: libc::c_ulong = 0;
#[allow(dead_code)]
const DIAG_IOCSREG: libc::c_ulong = 0;
#[allow(dead_code)]
const DIAG_IOCSUNREG: libc::c_ulong = 0;
#[allow(dead_code)]
const USER_EVENT_REG_MULTI_FORMAT: c_uint = 0;
#[allow(dead_code)]
const USER_EVENT_REG_PERSIST: c_uint = 0;
#[allow(dead_code)]
const USER_EVENT_REG_MAX: c_uint = u32::MAX;

static DATA_FILE: &[u8] = b"/sys/kernel/tracing/user_events_data\0";
static ENABLE_FILE: &[u8] = b"/sys/kernel/tracing/events/user_events/__abi_event/enable\0";
static MULTI_DIR_GLOB: &[u8] = b"/sys/kernel/tracing/events/user_events_multi/__abi_event.*\0";

#[inline]
unsafe fn errno() -> c_int {
    *libc::__errno_location()
}

fn wait_for_delete(dir: *const c_char) -> c_int {
    let mut i = 0;
    let mut buf: libc::stat = unsafe { std::mem::zeroed() };

    while i < 10000 {
        if unsafe { libc::stat(dir, &mut buf as *mut libc::stat) } == -1 && unsafe { errno() } == libc::ENOENT
        {
            return 0;
        }

        unsafe { libc::usleep(1000) };
        i += 1;
    }

    -1
}

fn find_multi_event_dir(unique_field: *const c_char, out_dir: *mut c_char, dir_len: c_int) -> c_int {
    let mut line_buf = [0i8; 256];
    let mut glob_buf: libc::glob_t = unsafe { std::mem::zeroed() };
    let mut ret: c_int;

    ret = unsafe {
        libc::glob(
            MULTI_DIR_GLOB.as_ptr() as *const c_char,
            libc::GLOB_ONLYDIR,
            None,
            &mut glob_buf,
        )
    };

    if ret != 0 {
        return -1;
    }

    ret = -1;

    let mut i = 0usize;
    while i < glob_buf.gl_pathc as usize {
        let event_dir = unsafe { *glob_buf.gl_pathv.as_ptr().add(i) };

        let path_len = unsafe {
            libc::snprintf(
                line_buf.as_mut_ptr(),
                line_buf.len(),
                b"%s/format\0".as_ptr() as *const c_char,
                event_dir,
            )
        };

        let _ = path_len;

        let fp = unsafe { libc::fopen(line_buf.as_ptr(), b"r\0".as_ptr() as *const c_char) };
        if fp.is_null() {
            i += 1;
            continue;
        }

        loop {
            let line = unsafe { libc::fgets(line_buf.as_mut_ptr(), line_buf.len() as c_int, fp) };
            if line.is_null() {
                break;
            }

            if unsafe { !libc::strstr(line_buf.as_ptr(), unique_field).is_null() } {
                unsafe {
                    libc::fclose(fp);
                }
                // strscpy is not available, use snprintf
                let _ = unsafe {
                    libc::snprintf(
                        out_dir,
                        dir_len as usize,
                        b"%s\0".as_ptr() as *const c_char,
                        event_dir,
                    )
                };
                ret = 0;
                unsafe { libc::globfree(&mut glob_buf) };
                return ret;
            }
        }

        unsafe { libc::fclose(fp) };
        i += 1;
    }

    unsafe { libc::globfree(&mut glob_buf) };
    ret
}

fn event_exists() -> bool {
    let fd = unsafe { libc::open(ENABLE_FILE.as_ptr() as *const c_char, libc::O_RDWR) };

    if fd < 0 {
        return false;
    }

    unsafe { libc::close(fd) };
    true
}

fn change_event(enable: bool) -> c_int {
    let fd = unsafe { libc::open(ENABLE_FILE.as_ptr() as *const c_char, libc::O_RDWR) };
    let ret: c_int;

    if fd < 0 {
        return -1;
    }

    if enable {
        ret = unsafe { libc::write(fd, b"1\0".as_ptr() as *const c_void, 1) };
    } else {
        ret = unsafe { libc::write(fd, b"0\0".as_ptr() as *const c_void, 1) };
    }

    unsafe { libc::close(fd) };

    if ret == 1 {
        0
    } else {
        -1
    }
}

fn event_delete() -> c_int {
    let fd = unsafe { libc::open(DATA_FILE.as_ptr() as *const c_char, libc::O_RDWR) };

    if fd < 0 {
        return -1;
    }

    let ret = unsafe { libc::ioctl(fd, DIAG_IOCSDEL, b"__abi_event\0".as_ptr()) };

    unsafe { libc::close(fd) };
    ret
}

/*
 * Deleting an event drops its last reference, but an unregister may defer
 * that put (and the freeing of the associated enabler) past an RCU grace
 * period. The delete can therefore transiently fail with -EBUSY while the
 * previous reference is still being dropped. Retry only on that transient
 * failure; treat an already-deleted event (-ENOENT) as success and return
 * any other error immediately rather than spinning for the full timeout.
 */
fn wait_for_event_delete() -> c_int {
    let mut i = 0;
    loop {
        let ret = event_delete();

        if ret == 0 || unsafe { errno() } == libc::ENOENT {
            return 0;
        }

        if unsafe { errno() } != libc::EBUSY {
            return ret;
        }

        if i >= 10000 {
            return ret;
        }

        unsafe { libc::usleep(1000) };
        i += 1;
    }
}

fn reg_enable_multi(enable: *mut c_void, size: c_int, bit: c_int, flags: c_uint, args: *const c_char) -> c_int {
    let mut reg: user_reg = unsafe { std::mem::zeroed() };
    let mut full_args = [0i8; 512];
    let fd = unsafe { libc::open(DATA_FILE.as_ptr() as *const c_char, libc::O_RDWR) };

    if fd < 0 {
        return -1;
    }

    let len = unsafe {
        libc::snprintf(
            full_args.as_mut_ptr(),
            full_args.len(),
            b"__abi_event %s\0".as_ptr() as *const c_char,
            args,
        )
    };

    if len > full_args.len() as c_int {
        unsafe { libc::close(fd) };
        return -libc::E2BIG;
    }

    reg.size = std::mem::size_of::<user_reg>() as c_uint;
    reg.name_args = full_args.as_ptr() as __u64;
    reg.flags = USER_EVENT_REG_MULTI_FORMAT | flags;
    reg.enable_bit = bit as c_uint;
    reg.enable_addr = enable as __u64;
    reg.enable_size = size as c_uint;

    let ret = unsafe { libc::ioctl(fd, DIAG_IOCSREG, &reg) };
    unsafe { libc::close(fd) };
    ret
}

fn reg_enable_flags(enable: *mut c_void, size: c_int, bit: c_int, flags: c_uint) -> c_int {
    let mut reg: user_reg = unsafe { std::mem::zeroed() };
    let fd = unsafe { libc::open(DATA_FILE.as_ptr() as *const c_char, libc::O_RDWR) };

    if fd < 0 {
        return -1;
    }

    reg.size = std::mem::size_of::<user_reg>() as c_uint;
    reg.name_args = b"__abi_event\0".as_ptr() as __u64;
    reg.flags = flags;
    reg.enable_bit = bit as c_uint;
    reg.enable_addr = enable as __u64;
    reg.enable_size = size as c_uint;

    let ret = unsafe { libc::ioctl(fd, DIAG_IOCSREG, &reg) };
    unsafe { libc::close(fd) };
    ret
}

fn reg_enable(enable: *mut c_void, size: c_int, bit: c_int) -> c_int {
    reg_enable_flags(enable, size, bit, 0)
}

fn reg_disable(enable: *mut c_void, bit: c_int) -> c_int {
    let mut reg: user_unreg = unsafe { std::mem::zeroed() };
    let fd = unsafe { libc::open(DATA_FILE.as_ptr() as *const c_char, libc::O_RDWR) };

    if fd < 0 {
        return -1;
    }

    reg.size = std::mem::size_of::<user_unreg>() as c_uint;
    reg.disable_bit = bit as c_uint;
    reg.disable_addr = enable as __u64;

    let ret = unsafe { libc::ioctl(fd, DIAG_IOCSUNREG, &reg) };
    unsafe { libc::close(fd) };
    ret
}

#[repr(C)]
struct User {
    check: c_int,
    check_long: c_long,
    umount: bool,
}

macro_rules! ASSERT_EQ {
    ($a:expr, $b:expr) => {
        assert_eq!($a, $b);
    };
}

macro_rules! ASSERT_NE {
    ($a:expr, $b:expr) => {
        assert_ne!($a, $b);
    };
}

macro_rules! ASSERT_TRUE {
    ($a:expr) => {
        assert!($a);
    };
}

macro_rules! ASSERT_FALSE {
    ($a:expr) => {
        assert!(!$a);
    };
}

fn user_fixture_setup(user: &mut User) {
    // USER_EVENT_FIXTURE_SETUP(return, self->umount);
    change_event(false);
    user.check = 0;
    user.check_long = 0;
}

fn user_fixture_teardown(user: &mut User) {
    // USER_EVENT_FIXTURE_TEARDOWN(self->umount);
    let _ = user;
}

fn test_f_enablement(user: &mut User) {
    /* Changes should be reflected immediately */
    ASSERT_EQ!(0, user.check);
    ASSERT_EQ!(
        0,
        reg_enable(
            &mut user.check as *mut c_int as *mut c_void,
            std::mem::size_of::<c_int>() as c_int,
            0,
        ),
    );
    ASSERT_EQ!(0, change_event(true));
    ASSERT_EQ!(1, user.check);
    ASSERT_EQ!(0, change_event(false));
    ASSERT_EQ!(0, user.check);

    /* Ensure kernel clears bit after disable */
    ASSERT_EQ!(0, change_event(true));
    ASSERT_EQ!(1, user.check);
    ASSERT_EQ!(
        0,
        reg_disable(&mut user.check as *mut c_int as *mut c_void, 0),
    );
    ASSERT_EQ!(0, user.check);

    /* Ensure doesn't change after unreg */
    ASSERT_EQ!(0, change_event(true));
    ASSERT_EQ!(0, user.check);
    ASSERT_EQ!(0, change_event(false));
}

fn test_f_flags(user: &mut User) {
    /* USER_EVENT_REG_PERSIST is allowed */
    ASSERT_EQ!(
        0,
        reg_enable_flags(
            &mut user.check as *mut c_int as *mut c_void,
            std::mem::size_of::<c_int>() as c_int,
            0,
            USER_EVENT_REG_PERSIST,
        ),
    );
    ASSERT_EQ!(0, reg_disable(&mut user.check as *mut c_int as *mut c_void, 0));

    /* Ensure it exists after close and disable */
    ASSERT_TRUE!(event_exists());

    /* Ensure we can delete it */
    ASSERT_EQ!(0, wait_for_event_delete());

    /* USER_EVENT_REG_MAX or above is not allowed */
    ASSERT_EQ!(
        -1,
        reg_enable_flags(
            &mut user.check as *mut c_int as *mut c_void,
            std::mem::size_of::<c_int>() as c_int,
            0,
            USER_EVENT_REG_MAX,
        ),
    );

    /* Ensure it does not exist after invalid flags */
    ASSERT_FALSE!(event_exists());
}

fn test_f_bit_sizes(user: &mut User) {
    /* Allow 0-31 bits for 32-bit */
    ASSERT_EQ!(
        0,
        reg_enable(
            &mut user.check as *mut c_int as *mut c_void,
            std::mem::size_of::<c_int>() as c_int,
            0,
        ),
    );
    ASSERT_EQ!(
        0,
        reg_enable(
            &mut user.check as *mut c_int as *mut c_void,
            std::mem::size_of::<c_int>() as c_int,
            31,
        ),
    );
    ASSERT_NE!(
        0,
        reg_enable(
            &mut user.check as *mut c_int as *mut c_void,
            std::mem::size_of::<c_int>() as c_int,
            32,
        ),
    );
    ASSERT_EQ!(0, reg_disable(&mut user.check as *mut c_int as *mut c_void, 0));
    ASSERT_EQ!(0, reg_disable(&mut user.check as *mut c_int as *mut c_void, 31));

    #[cfg(target_pointer_width = "64")]
    {
        /* Allow 0-64 bits for 64-bit */
        ASSERT_EQ!(
            0,
            reg_enable(
                &mut user.check_long as *mut c_long as *mut c_void,
                std::mem::size_of::<c_long>() as c_int,
                63,
            ),
        );
        ASSERT_NE!(
            0,
            reg_enable(
                &mut user.check_long as *mut c_long as *mut c_void,
                std::mem::size_of::<c_long>() as c_int,
                64,
            ),
        );
        ASSERT_EQ!(
            0,
            reg_disable(&mut user.check_long as *mut c_long as *mut c_void, 63),
        );
    }

    /* Disallowed sizes (everything beside 4 and 8) */
    ASSERT_NE!(0, reg_enable(&mut user.check as *mut c_int as *mut c_void, 1, 0));
    ASSERT_NE!(0, reg_enable(&mut user.check as *mut c_int as *mut c_void, 2, 0));
    ASSERT_NE!(0, reg_enable(&mut user.check as *mut c_int as *mut c_void, 3, 0));
    ASSERT_NE!(0, reg_enable(&mut user.check as *mut c_int as *mut c_void, 5, 0));
    ASSERT_NE!(0, reg_enable(&mut user.check as *mut c_int as *mut c_void, 6, 0));
    ASSERT_NE!(0, reg_enable(&mut user.check as *mut c_int as *mut c_void, 7, 0));
    ASSERT_NE!(0, reg_enable(&mut user.check as *mut c_int as *mut c_void, 9, 0));
    ASSERT_NE!(0, reg_enable(&mut user.check as *mut c_int as *mut c_void, 128, 0));
}

fn test_f_multi_format(user: &mut User) {
    let mut first_dir = [0i8; 256];
    let mut second_dir = [0i8; 256];
    let mut buf: libc::stat = unsafe { std::mem::zeroed() };

    /* Multiple formats for the same name should work */
    ASSERT_EQ!(
        0,
        reg_enable_multi(
            &mut user.check as *mut c_int as *mut c_void,
            std::mem::size_of::<c_int>() as c_int,
            0,
            0,
            b"u32 multi_first\0".as_ptr() as *const c_char,
        ),
    );

    ASSERT_EQ!(
        0,
        reg_enable_multi(
            &mut user.check as *mut c_int as *mut c_void,
            std::mem::size_of::<c_int>() as c_int,
            1,
            0,
            b"u64 multi_second\0".as_ptr() as *const c_char,
        ),
    );

    /* Same name with same format should also work */
    ASSERT_EQ!(
        0,
        reg_enable_multi(
            &mut user.check as *mut c_int as *mut c_void,
            std::mem::size_of::<c_int>() as c_int,
            2,
            0,
            b"u64 multi_second\0".as_ptr() as *const c_char,
        ),
    );

    ASSERT_EQ!(
        0,
        find_multi_event_dir(
            b"multi_first\0".as_ptr() as *const c_char,
            first_dir.as_mut_ptr(),
            first_dir.len() as c_int,
        ),
    );

    ASSERT_EQ!(
        0,
        find_multi_event_dir(
            b"multi_second\0".as_ptr() as *const c_char,
            second_dir.as_mut_ptr(),
            second_dir.len() as c_int,
        ),
    );

    /* Should not be found in the same dir */
    let first_dir_c = unsafe { CStr::from_ptr(first_dir.as_ptr()) };
    let second_dir_c = unsafe { CStr::from_ptr(second_dir.as_ptr()) };
    ASSERT_NE!(first_dir_c, second_dir_c);

    /* First dir should still exist */
    ASSERT_EQ!(0, unsafe { libc::stat(first_dir.as_ptr(), &mut buf as *mut libc::stat) });

    /* Disabling first register should remove first dir */
    ASSERT_EQ!(0, reg_disable(&mut user.check as *mut c_int as *mut c_void, 0));
    ASSERT_EQ!(0, wait_for_delete(first_dir.as_ptr()));

    /* Second dir should still exist */
    ASSERT_EQ!(0, unsafe { libc::stat(second_dir.as_ptr(), &mut buf as *mut libc::stat) });

    /* Disabling second register should remove second dir */
    ASSERT_EQ!(0, reg_disable(&mut user.check as *mut c_int as *mut c_void, 1));
    /* Ensure bit 1 and 2 are tied together, should not delete yet */
    ASSERT_EQ!(0, unsafe { libc::stat(second_dir.as_ptr(), &mut buf as *mut libc::stat) });
    ASSERT_EQ!(0, reg_disable(&mut user.check as *mut c_int as *mut c_void, 2));
    ASSERT_EQ!(0, wait_for_delete(second_dir.as_ptr()));
}

fn test_f_forks(user: &mut User) {
    /* Ensure COW pages get updated after fork */
    ASSERT_EQ!(
        0,
        reg_enable(
            &mut user.check as *mut c_int as *mut c_void,
            std::mem::size_of::<c_int>() as c_int,
            0,
        ),
    );
    ASSERT_EQ!(0, user.check);

    if unsafe { libc::fork() } == 0 {
        /* Force COW */
        user.check = 0;

        /* Up to 1 sec for enablement */
        for _ in 0..10 {
            unsafe { libc::usleep(100_000) };

            if user.check != 0 {
                unsafe { libc::_exit(0) };
            }
        }

        unsafe { libc::_exit(1) };
    }

    /* Allow generous time for COW, then enable */
    unsafe { libc::usleep(100_000) };
    ASSERT_EQ!(0, change_event(true));

    let mut status: c_int = 0;
    ASSERT_NE!(-1, unsafe { libc::wait(&mut status as *mut c_int) });
    ASSERT_EQ!(0, (status >> 8) & 0xff);

    /* Ensure child doesn't disable parent */
    if unsafe { libc::fork() } == 0 {
        unsafe { libc::_exit(reg_disable(&mut user.check as *mut c_int as *mut c_void, 0)) };
    }

    let mut status2: c_int = 0;
    ASSERT_NE!(-1, unsafe { libc::wait(&mut status2 as *mut c_int) });
    ASSERT_EQ!(0, (status2 >> 8) & 0xff);
    ASSERT_EQ!(1, user.check);
    ASSERT_EQ!(0, change_event(false));
    ASSERT_EQ!(0, user.check);
}

/* Waits up to 1 sec for enablement */
extern "C" fn clone_check(check: *mut c_void) -> c_int {
    for _ in 0..10 {
        unsafe { libc::usleep(100_000) };

        if unsafe { *(check as *mut c_int) != 0 } {
            return 0;
        }
    }

    1
}

fn test_f_clones(user: &mut User) {
    let stack_size: usize = 4096;
    let stack = unsafe {
        libc::mmap(
            ptr::null_mut(),
            stack_size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_STACK,
            -1,
            0,
        )
    };

    ASSERT_NE!(libc::MAP_FAILED as usize as isize, stack as isize);
    ASSERT_EQ!(
        0,
        reg_enable(
            &mut user.check as *mut c_int as *mut c_void,
            std::mem::size_of::<c_int>() as c_int,
            0,
        ),
    );
    ASSERT_EQ!(0, user.check);

    /* Shared VM should see enablements */
    let child_stack = unsafe { (stack as *mut u8).add(stack_size) as *mut c_void };
    ASSERT_NE!(
        -1,
        unsafe {
            libc::clone(
                clone_check,
                child_stack,
                libc::CLONE_VM | libc::SIGCHLD,
                &mut user.check as *mut c_int as *mut c_void,
            )
        },
    );

    ASSERT_EQ!(0, change_event(true));
    let mut status: c_int = 0;
    ASSERT_NE!(-1, unsafe { libc::wait(&mut status as *mut c_int) });
    ASSERT_EQ!(0, (status >> 8) & 0xff);
    unsafe { libc::munmap(stack, stack_size) };
    ASSERT_EQ!(0, change_event(false));
}

unsafe extern "C" {
    fn test_harness_run(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

#[allow(unused_variables)]
fn main() -> i32 {
    unsafe { test_harness_run(0, ptr::null_mut()) }
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
