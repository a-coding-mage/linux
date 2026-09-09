// SPDX-License-Identifier: GPL-2.0+

// Dependencies supplied by the Linux kernel, KUnit, and test-kprobes.h remain
// external to this translation unit.

use core::ffi::c_void;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kprobe {
    pub addr: *mut c_void,
    pub pre_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs) -> i32>,
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_case {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_suite {
    pub name: *const u8,
    pub test_cases: *mut kunit_case,
}

extern "C" {
    static mut test_kprobes_addresses: *mut *mut c_void;
    static mut test_kprobes_functions:
        *mut Option<unsafe extern "C" fn() -> libc::c_long>;
    static KPROBE_TEST_MAGIC: libc::c_long;

    fn register_kprobe(kp: *mut kprobe) -> i32;
    fn unregister_kprobe(kp: *mut kprobe);
    fn kfree(ptr: *mut kprobe);
    fn kzalloc_objs<T>(count: usize) -> *mut T;

    fn kunit_expect_true(test: *mut kunit, condition: bool);
    fn kunit_expect_eq(test: *mut kunit, expected: libc::c_long, actual: libc::c_long);
    fn kunit_expect_eq_msg(
        test: *mut kunit,
        expected: libc::c_long,
        actual: libc::c_long,
        message: *const u8,
        index: i32,
    );
}

unsafe extern "C" fn kprobe_dummy_handler(
    _kp: *mut kprobe,
    _regs: *mut pt_regs,
) -> i32 {
    0
}

unsafe fn test_kprobe_riscv(test: *mut kunit) {
    let mut num_kprobe: u32 = 0;
    let mut func: Option<unsafe extern "C" fn() -> libc::c_long>;
    let kp: *mut kprobe;
    let mut i: i32;

    while !(*test_kprobes_addresses.add(num_kprobe as usize)).is_null() {
        num_kprobe = num_kprobe.wrapping_add(1);
    }

    kp = kzalloc_objs::<kprobe>(num_kprobe as usize);
    kunit_expect_true(test, !kp.is_null());
    if kp.is_null() {
        return;
    }

    i = 0;
    while i < num_kprobe as i32 {
        (*kp.add(i as usize)).addr = *test_kprobes_addresses.add(i as usize);
        (*kp.add(i as usize)).pre_handler = Some(kprobe_dummy_handler);
        kunit_expect_eq(
            test,
            0,
            register_kprobe(kp.add(i as usize)) as libc::c_long,
        );
        i += 1;
    }

    i = 0;
    loop {
        func = *test_kprobes_functions.add(i as usize);
        if func.is_none() {
            break;
        }
        kunit_expect_eq_msg(
            test,
            KPROBE_TEST_MAGIC,
            func.unwrap()() as libc::c_long,
            b"function %d broken\0".as_ptr(),
            i,
        );
        i += 1;
    }

    i = 0;
    while i < num_kprobe as i32 {
        unregister_kprobe(kp.add(i as usize));
        i += 1;
    }
    kfree(kp);
}

// KUNIT_CASE(test_kprobe_riscv)
static mut kprobes_testcases: [kunit_case; 2] = unsafe { core::mem::zeroed() };

static mut kprobes_test_suite: kunit_suite = kunit_suite {
    name: b"kprobes_riscv\0".as_ptr(),
    test_cases: core::ptr::null_mut(),
};

// kunit_test_suites(&kprobes_test_suite);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("KUnit test for riscv kprobes");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
