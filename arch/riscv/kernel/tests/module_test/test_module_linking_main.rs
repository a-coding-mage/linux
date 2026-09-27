// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2023 Rivos Inc.
 */

// Kernel module, KUnit, and conditional configuration dependencies are supplied
// by the surrounding kernel build.

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

extern "C" {
    pub fn test_set32() -> kernel::ffi::c_int;
    pub fn test_set16() -> kernel::ffi::c_int;
    pub fn test_set8() -> kernel::ffi::c_int;
    pub fn test_set6() -> kernel::ffi::c_int;
    pub fn test_sub64() -> kernel::ffi::c_long;
    pub fn test_sub32() -> kernel::ffi::c_int;
    pub fn test_sub16() -> kernel::ffi::c_int;
    pub fn test_sub8() -> kernel::ffi::c_int;
    pub fn test_sub6() -> kernel::ffi::c_int;

    // Present when CONFIG_AS_HAS_ULEB128 is enabled.
    #[cfg(CONFIG_AS_HAS_ULEB128)]
    pub fn test_uleb_basic() -> kernel::ffi::c_int;
    #[cfg(CONFIG_AS_HAS_ULEB128)]
    pub fn test_uleb_large() -> kernel::ffi::c_int;

    // KUnit assertion supplied by the kernel KUnit implementation.
    pub fn kunit_assert_eq(
        test: *mut kunit,
        lhs: isize,
        rhs: isize,
        lhs_name: *const kernel::ffi::c_char,
        rhs_name: *const kernel::ffi::c_char,
        file: *const kernel::ffi::c_char,
        line: kernel::ffi::c_uint,
    );
}

macro_rules! check_eq {
    ($test:expr, $lhs:expr, $rhs:expr) => {
        unsafe {
            kunit_assert_eq(
                $test,
                ($lhs) as isize,
                ($rhs) as isize,
                concat!(stringify!($lhs), "\0").as_ptr() as *const kernel::ffi::c_char,
                concat!(stringify!($rhs), "\0").as_ptr() as *const kernel::ffi::c_char,
                concat!(file!(), "\0").as_ptr() as *const kernel::ffi::c_char,
                line!(),
            );
        }
    };
}

pub unsafe extern "C" fn run_test_set(test: *mut kunit) {
    let val32 = test_set32();
    let val16 = test_set16();
    let val8 = test_set8();
    let val6 = test_set6();

    check_eq!(test, val32, 0);
    check_eq!(test, val16, 0);
    check_eq!(test, val8, 0);
    check_eq!(test, val6, 0);
}

pub unsafe extern "C" fn run_test_sub(test: *mut kunit) {
    let val64 = test_sub64();
    let val32 = test_sub32();
    let val16 = test_sub16();
    let val8 = test_sub8();
    let val6 = test_sub6();

    check_eq!(test, val64, 0);
    check_eq!(test, val32, 0);
    check_eq!(test, val16, 0);
    check_eq!(test, val8, 0);
    check_eq!(test, val6, 0);
}

#[cfg(CONFIG_AS_HAS_ULEB128)]
pub unsafe extern "C" fn run_test_uleb(test: *mut kunit) {
    let val_uleb = test_uleb_basic();
    let val_uleb2 = test_uleb_large();

    check_eq!(test, val_uleb, 0);
    check_eq!(test, val_uleb2, 0);
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct kunit_case {
    pub run_case: Option<unsafe extern "C" fn(*mut kunit)>,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct kunit_suite {
    pub name: *const kernel::ffi::c_char,
    pub test_cases: *mut kunit_case,
}

#[used]
#[link_section = ".rodata"]
pub static mut riscv_module_linking_test_cases: [kunit_case; 3] = [
    kunit_case { run_case: Some(run_test_set) },
    kunit_case { run_case: Some(run_test_sub) },
    #[cfg(CONFIG_AS_HAS_ULEB128)]
    kunit_case { run_case: Some(run_test_uleb) },
    kunit_case { run_case: None },
];

#[used]
pub static mut riscv_module_linking_test_suite: kunit_suite = kunit_suite {
    name: b"riscv_checksum\0".as_ptr() as *const kernel::ffi::c_char,
    test_cases: unsafe { riscv_module_linking_test_cases.as_mut_ptr() },
};

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Test module linking");
// kunit_test_suites(&riscv_module_linking_test_suite);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
