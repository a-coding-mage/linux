// SPDX-License-Identifier: GPL-2.0
//
// Translated from C source ./wp.c. Original includes:
// <stdlib.h>, <string.h>, <unistd.h>, <errno.h>, <sys/ioctl.h>,
// <linux/compiler.h>, <linux/hw_breakpoint.h>, <linux/kernel.h>,
// "tests.h", "debug.h", "event.h", "cloexec.h", "../perf-sys.h"

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;

const PERF_TYPE_BREAKPOINT: u32 = 5;
const PERF_SAMPLE_IP: u64 = 1 << 0;
const HW_BREAKPOINT_R: c_int = 1;
const HW_BREAKPOINT_W: c_int = 2;
const PERF_EVENT_IOC_ENABLE: c_ulong = 0x2400;
const PERF_EVENT_IOC_MODIFY_ATTRIBUTES: c_ulong = 0x4008240b;
const ENODEV: c_int = 19;
const ENOTTY: c_int = 25;
const TEST_SKIP: c_int = 2;

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample_period: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub flags: u64,
    pub bp_type: u32,
    pub bp_addr: u64,
    pub bp_len: u64,
}

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
    pub skip_reason: *const c_char,
}

#[repr(C)]
pub struct test_suite {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
}

unsafe extern "C" {
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn rand() -> c_int;
    fn sys_perf_event_open(
        attr: *mut perf_event_attr,
        pid: c_int,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int;
    fn perf_event_open_cloexec_flag() -> c_ulong;
    fn pr_debug(fmt: *const c_char, ...);
    fn __errno_location() -> *mut c_int;
    fn TEST_ASSERT_VAL(text: *const c_char, cond: bool);
}

#[cfg(target_arch = "x86")]
static mut data1: u32 = 0;

#[cfg(not(target_arch = "x86"))]
static mut data1: u64 = 0;

static mut data2: [u8; 3] = [0; 3];

#[cfg(not(target_arch = "s390x"))]
unsafe fn wp_read(fd: c_int, count: *mut i64, size: c_int) -> c_int {
    let ret = unsafe { read(fd, count as *mut c_void, size as usize) };

    if ret != size as isize {
        unsafe {
            pr_debug(c"failed to read: %d\n".as_ptr(), ret as c_int);
        }
        return -1;
    }
    0
}

#[cfg(not(target_arch = "s390x"))]
unsafe fn get__perf_event_attr(
    attr: *mut perf_event_attr,
    wp_type: c_int,
    wp_addr: *mut c_void,
    wp_len: c_ulong,
) {
    unsafe {
        ptr::write_bytes(attr as *mut u8, 0, mem::size_of::<perf_event_attr>());
        (*attr).type_ = PERF_TYPE_BREAKPOINT;
        (*attr).size = mem::size_of::<perf_event_attr>() as u32;
        (*attr).config = 0;
        (*attr).bp_type = wp_type as u32;
        (*attr).bp_addr = wp_addr as c_ulong as u64;
        (*attr).bp_len = wp_len as u64;
        (*attr).sample_period = 1;
        (*attr).sample_type = PERF_SAMPLE_IP;
        (*attr).flags |= 1 << 5;
        (*attr).flags |= 1 << 6;
    }
}

#[cfg(not(target_arch = "s390x"))]
unsafe fn __event(wp_type: c_int, wp_addr: *mut c_void, wp_len: c_ulong) -> c_int {
    let mut fd: c_int;
    let mut attr: perf_event_attr = unsafe { mem::zeroed() };

    unsafe {
        get__perf_event_attr(&mut attr, wp_type, wp_addr, wp_len);
        fd = sys_perf_event_open(
            &mut attr,
            0,
            -1,
            -1,
            perf_event_open_cloexec_flag(),
        );
        if fd < 0 {
            fd = -*__errno_location();
            pr_debug(c"failed opening event %x\n".as_ptr(), attr.bp_type);
        }
    }

    fd
}

#[cfg(not(target_arch = "s390x"))]
unsafe fn WP_TEST_ASSERT_VAL(fd: c_int, text: *const c_char, val: i64) {
    let mut count: i64 = 0;
    unsafe {
        wp_read(fd, &mut count, mem::size_of::<i64>() as c_int);
        TEST_ASSERT_VAL(text, count == val);
    }
}

unsafe extern "C" fn test__wp_ro(_test: *mut test_suite, _subtest: c_int) -> c_int {
    #[cfg(any(target_arch = "s390x", target_arch = "x86_64", target_arch = "x86"))]
    {
        return TEST_SKIP;
    }

    #[cfg(not(any(target_arch = "s390x", target_arch = "x86_64", target_arch = "x86")))]
    unsafe {
        let fd: c_int;
        let tmp: c_ulong;
        let tmp1: c_ulong = rand() as c_ulong;

        fd = __event(
            HW_BREAKPOINT_R,
            &raw mut data1 as *mut c_void,
            mem::size_of_val(&data1) as c_ulong,
        );
        if fd < 0 {
            return if fd == -ENODEV { TEST_SKIP } else { -1 };
        }

        tmp = ptr::read_volatile(&raw const data1) as c_ulong;
        WP_TEST_ASSERT_VAL(fd, c"RO watchpoint".as_ptr(), 1);

        ptr::write_volatile(&raw mut data1, (tmp1.wrapping_add(tmp)) as _);
        WP_TEST_ASSERT_VAL(fd, c"RO watchpoint".as_ptr(), 1);

        close(fd);
        0
    }
}

unsafe extern "C" fn test__wp_wo(_test: *mut test_suite, _subtest: c_int) -> c_int {
    #[cfg(target_arch = "s390x")]
    {
        return TEST_SKIP;
    }

    #[cfg(not(target_arch = "s390x"))]
    unsafe {
        let fd: c_int;
        let tmp: c_ulong;
        let tmp1: c_ulong = rand() as c_ulong;

        fd = __event(
            HW_BREAKPOINT_W,
            &raw mut data1 as *mut c_void,
            mem::size_of_val(&data1) as c_ulong,
        );
        if fd < 0 {
            return if fd == -ENODEV { TEST_SKIP } else { -1 };
        }

        tmp = ptr::read_volatile(&raw const data1) as c_ulong;
        WP_TEST_ASSERT_VAL(fd, c"WO watchpoint".as_ptr(), 0);

        ptr::write_volatile(&raw mut data1, (tmp1.wrapping_add(tmp)) as _);
        WP_TEST_ASSERT_VAL(fd, c"WO watchpoint".as_ptr(), 1);

        close(fd);
        0
    }
}

unsafe extern "C" fn test__wp_rw(_test: *mut test_suite, _subtest: c_int) -> c_int {
    #[cfg(target_arch = "s390x")]
    {
        return TEST_SKIP;
    }

    #[cfg(not(target_arch = "s390x"))]
    unsafe {
        let fd: c_int;
        let tmp: c_ulong;
        let tmp1: c_ulong = rand() as c_ulong;

        fd = __event(
            HW_BREAKPOINT_R | HW_BREAKPOINT_W,
            &raw mut data1 as *mut c_void,
            mem::size_of_val(&data1) as c_ulong,
        );
        if fd < 0 {
            return if fd == -ENODEV { TEST_SKIP } else { -1 };
        }

        tmp = ptr::read_volatile(&raw const data1) as c_ulong;
        WP_TEST_ASSERT_VAL(fd, c"RW watchpoint".as_ptr(), 1);

        ptr::write_volatile(&raw mut data1, (tmp1.wrapping_add(tmp)) as _);
        WP_TEST_ASSERT_VAL(fd, c"RW watchpoint".as_ptr(), 2);

        close(fd);
        0
    }
}

unsafe extern "C" fn test__wp_modify(test: *mut test_suite, subtest: c_int) -> c_int {
    #[cfg(target_arch = "s390x")]
    {
        return TEST_SKIP;
    }

    #[cfg(not(target_arch = "s390x"))]
    unsafe {
        let fd: c_int;
        let mut ret: c_int;
        let tmp: c_ulong = rand() as c_ulong;
        let mut new_attr: perf_event_attr = mem::zeroed();

        fd = __event(
            HW_BREAKPOINT_W,
            &raw mut data1 as *mut c_void,
            mem::size_of_val(&data1) as c_ulong,
        );
        if fd < 0 {
            return if fd == -ENODEV { TEST_SKIP } else { -1 };
        }

        ptr::write_volatile(&raw mut data1, tmp as _);
        WP_TEST_ASSERT_VAL(fd, c"Modify watchpoint".as_ptr(), 1);

        /* Modify watchpoint with disabled = 1 */
        get__perf_event_attr(
            &mut new_attr,
            HW_BREAKPOINT_W,
            &raw mut data2[0] as *mut c_void,
            (mem::size_of::<u8>() * 2) as c_ulong,
        );
        new_attr.flags |= 1;
        ret = ioctl(fd, PERF_EVENT_IOC_MODIFY_ATTRIBUTES, &mut new_attr);
        if ret < 0 {
            if *__errno_location() == ENOTTY {
                (*(*test).test_cases.add(subtest as usize)).skip_reason =
                    c"missing kernel support".as_ptr();
                ret = TEST_SKIP;
            }

            pr_debug(c"ioctl(PERF_EVENT_IOC_MODIFY_ATTRIBUTES) failed\n".as_ptr());
            close(fd);
            return ret;
        }

        ptr::write_volatile(&raw mut data2[1], tmp as u8); /* Not Counted */
        WP_TEST_ASSERT_VAL(fd, c"Modify watchpoint".as_ptr(), 1);

        /* Enable the event */
        ioctl(fd, PERF_EVENT_IOC_ENABLE, 0 as c_ulong);
        if ret < 0 {
            pr_debug(c"Failed to enable event\n".as_ptr());
            close(fd);
            return ret;
        }

        ptr::write_volatile(&raw mut data2[1], tmp as u8); /* Counted */
        WP_TEST_ASSERT_VAL(fd, c"Modify watchpoint".as_ptr(), 2);

        ptr::write_volatile(&raw mut data2[2], tmp as u8); /* Not Counted */
        WP_TEST_ASSERT_VAL(fd, c"Modify watchpoint".as_ptr(), 2);

        close(fd);
        0
    }
}

static mut wp_tests: [test_case; 5] = [
    test_case {
        name: c"Read Only Watchpoint".as_ptr(),
        run_case: Some(test__wp_ro),
        skip_reason: c"missing hardware support".as_ptr(),
    },
    test_case {
        name: c"Write Only Watchpoint".as_ptr(),
        run_case: Some(test__wp_wo),
        skip_reason: c"missing hardware support".as_ptr(),
    },
    test_case {
        name: c"Read / Write Watchpoint".as_ptr(),
        run_case: Some(test__wp_rw),
        skip_reason: c"missing hardware support".as_ptr(),
    },
    test_case {
        name: c"Modify Watchpoint".as_ptr(),
        run_case: Some(test__wp_modify),
        skip_reason: c"missing hardware support".as_ptr(),
    },
    test_case {
        name: ptr::null(),
        run_case: None,
        skip_reason: ptr::null(),
    },
];

#[unsafe(no_mangle)]
pub static mut suite__wp: test_suite = test_suite {
    desc: c"Watchpoint".as_ptr(),
    test_cases: &raw mut wp_tests as *mut test_case,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
