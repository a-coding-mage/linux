// SPDX-License-Identifier: GPL-2.0

// C dependencies from:
// #define _GNU_SOURCE
// <check.h>, <stdio.h>, <stdlib.h>, <sched.h>, <limits.h>, <unistd.h>,
// <sys/sysinfo.h>, and "../../src/utils.h".

use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const INT_MAX: c_int = c_int::MAX;
const SCHED_OTHER: c_uint = 0;
const SCHED_FIFO: c_uint = 1;
const SCHED_RR: c_uint = 2;

#[repr(C)]
pub struct Suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct TCase {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpu_set_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sched_attr {
    pub size: u32,
    pub sched_policy: u32,
    pub sched_flags: u64,
    pub sched_nice: i32,
    pub sched_priority: u32,
    pub sched_runtime: u64,
    pub sched_deadline: u64,
    pub sched_period: u64,
}

unsafe extern "C" {
    static mut nr_cpus: c_int;

    fn strtoi(str_: *const c_char, value: *mut c_int) -> c_int;
    fn parse_cpu_set(cpu_list: *const c_char, set: *mut cpu_set_t) -> c_int;
    fn parse_prio(arg: *const c_char, sched_param: *mut sched_attr) -> c_int;

    fn snprintf(str_: *mut c_char, size: c_ulong, format: *const c_char, ...) -> c_int;

    fn CPU_ISSET(cpu: c_int, cpusetp: *const cpu_set_t) -> c_int;

    fn suite_create(name: *const c_char) -> *mut Suite;
    fn tcase_create(name: *const c_char) -> *mut TCase;
    fn tcase_add_test(tc: *mut TCase, test: unsafe extern "C" fn(c_int));
    fn suite_add_tcase(s: *mut Suite, tc: *mut TCase);
}

unsafe extern "C" fn test_strtoi(_i: c_int) {
    let mut result: c_int = 0;
    let mut buf = [0 as c_char; 64];

    assert_eq!(unsafe { strtoi(c"123".as_ptr(), &mut result) }, 0);
    assert_eq!(result, 123);
    assert_eq!(unsafe { strtoi(c" -456".as_ptr(), &mut result) }, 0);
    assert_eq!(result, -456);

    unsafe {
        snprintf(
            buf.as_mut_ptr(),
            buf.len() as c_ulong,
            c"%d".as_ptr(),
            INT_MAX,
        );
    }
    assert_eq!(unsafe { strtoi(buf.as_ptr(), &mut result) }, 0);
    unsafe {
        snprintf(
            buf.as_mut_ptr(),
            buf.len() as c_ulong,
            c"%ld".as_ptr(),
            INT_MAX as i64 + 1,
        );
    }
    assert_eq!(unsafe { strtoi(buf.as_ptr(), &mut result) }, -1);

    assert_eq!(unsafe { strtoi(c"".as_ptr(), &mut result) }, -1);
    assert_eq!(unsafe { strtoi(c"123abc".as_ptr(), &mut result) }, -1);
    assert_eq!(unsafe { strtoi(c"123 ".as_ptr(), &mut result) }, -1);
}

unsafe extern "C" fn test_parse_cpu_set(_i: c_int) {
    let mut set = std::mem::MaybeUninit::<cpu_set_t>::uninit();

    unsafe {
        nr_cpus = 8;
    }
    assert_eq!(unsafe { parse_cpu_set(c"0".as_ptr(), set.as_mut_ptr()) }, 0);
    assert!(unsafe { CPU_ISSET(0, set.as_ptr()) != 0 });
    assert!(unsafe { CPU_ISSET(1, set.as_ptr()) == 0 });

    assert_eq!(unsafe { parse_cpu_set(c"0,2".as_ptr(), set.as_mut_ptr()) }, 0);
    assert!(unsafe { CPU_ISSET(0, set.as_ptr()) != 0 });
    assert!(unsafe { CPU_ISSET(2, set.as_ptr()) != 0 });

    assert_eq!(unsafe { parse_cpu_set(c"0-3".as_ptr(), set.as_mut_ptr()) }, 0);
    assert!(unsafe { CPU_ISSET(0, set.as_ptr()) != 0 });
    assert!(unsafe { CPU_ISSET(1, set.as_ptr()) != 0 });
    assert!(unsafe { CPU_ISSET(2, set.as_ptr()) != 0 });
    assert!(unsafe { CPU_ISSET(3, set.as_ptr()) != 0 });

    assert_eq!(unsafe { parse_cpu_set(c"1-3,5".as_ptr(), set.as_mut_ptr()) }, 0);
    assert!(unsafe { CPU_ISSET(0, set.as_ptr()) == 0 });
    assert!(unsafe { CPU_ISSET(1, set.as_ptr()) != 0 });
    assert!(unsafe { CPU_ISSET(2, set.as_ptr()) != 0 });
    assert!(unsafe { CPU_ISSET(3, set.as_ptr()) != 0 });
    assert!(unsafe { CPU_ISSET(4, set.as_ptr()) == 0 });
    assert!(unsafe { CPU_ISSET(5, set.as_ptr()) != 0 });

    assert_eq!(unsafe { parse_cpu_set(c"-1".as_ptr(), set.as_mut_ptr()) }, 1);
    assert_eq!(unsafe { parse_cpu_set(c"abc".as_ptr(), set.as_mut_ptr()) }, 1);
    assert_eq!(unsafe { parse_cpu_set(c"9999".as_ptr(), set.as_mut_ptr()) }, 1);
}

unsafe extern "C" fn test_parse_prio(_i: c_int) {
    let mut attr = std::mem::MaybeUninit::<sched_attr>::uninit();

    assert_eq!(unsafe { parse_prio(c"f:50".as_ptr(), attr.as_mut_ptr()) }, 0);
    assert_eq!(unsafe { (*attr.as_ptr()).sched_policy }, SCHED_FIFO);
    assert_eq!(unsafe { (*attr.as_ptr()).sched_priority }, 50_u32);

    assert_eq!(unsafe { parse_prio(c"r:30".as_ptr(), attr.as_mut_ptr()) }, 0);
    assert_eq!(unsafe { (*attr.as_ptr()).sched_policy }, SCHED_RR);

    assert_eq!(unsafe { parse_prio(c"o:0".as_ptr(), attr.as_mut_ptr()) }, 0);
    assert_eq!(unsafe { (*attr.as_ptr()).sched_policy }, SCHED_OTHER);
    assert_eq!(unsafe { (*attr.as_ptr()).sched_nice }, 0);

    assert_eq!(
        unsafe { parse_prio(c"d:10ms:100ms".as_ptr(), attr.as_mut_ptr()) },
        0
    );
    assert_eq!(unsafe { (*attr.as_ptr()).sched_policy }, 6_u32);

    assert_eq!(unsafe { parse_prio(c"f:999".as_ptr(), attr.as_mut_ptr()) }, -1);
    assert_eq!(unsafe { parse_prio(c"o:-20".as_ptr(), attr.as_mut_ptr()) }, -1);
    assert_eq!(
        unsafe { parse_prio(c"d:100ms:10ms".as_ptr(), attr.as_mut_ptr()) },
        -1
    );
    assert_eq!(unsafe { parse_prio(c"x:50".as_ptr(), attr.as_mut_ptr()) }, -1);
}

#[no_mangle]
pub unsafe extern "C" fn utils_suite() -> *mut Suite {
    let s = unsafe { suite_create(c"utils".as_ptr()) };
    let tc = unsafe { tcase_create(c"core".as_ptr()) };

    unsafe {
        tcase_add_test(tc, test_strtoi);
        tcase_add_test(tc, test_parse_cpu_set);
        tcase_add_test(tc, test_parse_prio);

        suite_add_tcase(s, tc);
    }
    s
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
