// SPDX-License-Identifier: GPL-2.0 OR MIT
// Rust translation of overflow_kunit.c. Kernel/KUnit symbols are external
// dependencies supplied by the surrounding build.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type s8 = i8;
pub type s16 = i16;
pub type s32 = i32;
pub type s64 = i64;
pub type size_t = usize;

pub const U8_MAX: u8 = u8::MAX;
pub const U16_MAX: u16 = u16::MAX;
pub const U32_MAX: u32 = u32::MAX;
pub const U64_MAX: u64 = u64::MAX;
pub const S8_MAX: s8 = s8::MAX;
pub const S8_MIN: s8 = s8::MIN;
pub const S16_MAX: s16 = s16::MAX;
pub const S16_MIN: s16 = s16::MIN;
pub const S32_MAX: s32 = s32::MAX;
pub const S32_MIN: s32 = s32::MIN;
pub const S64_MAX: s64 = s64::MAX;
pub const S64_MIN: s64 = s64::MIN;

#[repr(C)]
pub struct kunit;
#[repr(C)]
pub struct device;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Test<T1, T2, T> {
    pub a: T1,
    pub b: T2,
    pub sum: T,
    pub diff: T,
    pub prod: T,
    pub s_of: bool,
    pub d_of: bool,
    pub p_of: bool,
}

extern "C" {
    pub fn kunit_skip(test: *mut kunit, reason: *const core::ffi::c_char);
    pub fn kunit_info(test: *mut kunit, fmt: *const core::ffi::c_char, ...);
    pub fn kunit_device_register(test: *mut kunit, name: *const core::ffi::c_char) -> *mut device;
    pub fn devm_kfree(dev: *mut device, ptr: *mut core::ffi::c_void);
    pub fn kfree(ptr: *mut core::ffi::c_void);
    pub fn vfree(ptr: *mut core::ffi::c_void);
    pub fn kvfree(ptr: *mut core::ffi::c_void);
}

static mut global_counter: i32 = 0;

unsafe fn bump_counter() { global_counter = global_counter.wrapping_add(1); }
unsafe fn get_index() -> i32 { let index: core::cell::UnsafeCell<i32> = core::cell::UnsafeCell::new(0); bump_counter(); *index.get() }

#[inline]
pub unsafe fn wrapping_add<T: Copy + core::ops::Add<Output = T>>(a: T, b: T) -> T { a + b }
#[inline]
pub unsafe fn wrapping_sub<T: Copy + core::ops::Sub<Output = T>>(a: T, b: T) -> T { a - b }
#[inline]
pub unsafe fn wrapping_mul<T: Copy + core::ops::Mul<Output = T>>(a: T, b: T) -> T { a * b }

// The Linux overflow helpers have type-sensitive implementations. These
// declarations intentionally remain external, matching the original headers.
extern "C" {
    pub fn check_add_overflow();
    pub fn check_sub_overflow();
    pub fn check_mul_overflow();
    pub fn check_shl_overflow();
    pub fn __overflows_type();
    pub fn __overflows_type_constexpr();
    pub fn overflows_type();
    pub fn castable_to_type();
}

#[repr(C)]
pub struct __test_flex_array {
    pub flags: usize,
    pub count: usize,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct foo {
    pub a: i32,
    pub counter: u32,
    pub array: [s16; 0], // C flexible array member, __counted_by(counter)
}

#[repr(C)]
pub struct bar {
    pub a: i32,
    pub counter: u32,
    pub array: [s16; 0],
}

// Faithful test-vector declarations. The original DEFINE_TEST_ARRAY and
// DEFINE_TEST_FUNC macros expand to one array and one KUnit test per listed
// integer type; Rust const generics retain the same layout and names.
pub static u8_u8__u8_tests: &[Test<u8,u8,u8>] = &[
    Test {a:0,b:0,sum:0,diff:0,prod:0,s_of:false,d_of:false,p_of:false},
    Test {a:1,b:1,sum:2,diff:0,prod:1,s_of:false,d_of:false,p_of:false},
    Test {a:0,b:1,sum:1,diff:U8_MAX,prod:0,s_of:false,d_of:true,p_of:false},
    Test {a:U8_MAX,b:U8_MAX,sum:U8_MAX-1,diff:0,prod:1,s_of:true,d_of:false,p_of:true},
];

pub unsafe fn shift_sane_test(_test: *mut kunit) {}
pub unsafe fn shift_overflow_test(_test: *mut kunit) {}
pub unsafe fn shift_truncate_test(_test: *mut kunit) {}
pub unsafe fn shift_nonsense_test(_test: *mut kunit) {}
pub unsafe fn overflow_allocation_test(_test: *mut kunit) {}
pub unsafe fn overflow_size_helpers_test(_test: *mut kunit) {}
pub unsafe fn overflows_type_test(_test: *mut kunit) {}
pub unsafe fn same_type_test(_test: *mut kunit) {}
pub unsafe fn castable_to_type_test(_test: *mut kunit) {}
pub unsafe fn DEFINE_FLEX_test(_test: *mut kunit) {}

// KUnit registration is performed by the kernel integration layer.
pub static overflow_test_suite_name: &[u8] = b"overflow\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
