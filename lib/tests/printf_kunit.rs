// SPDX-License-Identifier: GPL-2.0-only
/* Test cases for printf facility. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Kernel headers and build-time configuration are supplied by the surrounding
 * translation unit. */
extern "C" {
    fn do_test(k: *mut kunit, file: *const i8, line: i32, bufsize: i32,
               expect: *const i8, elen: i32, fmt: *const i8, ap: *mut va_list);
    fn __test(k: *mut kunit, file: *const i8, line: i32, expect: *const i8,
              elen: i32, fmt: *const i8, ...);
}

type c_char = i8;
type u8 = u8;
type u32 = u32;
type ulong = usize;
type size_t = usize;
type time64_t = i64;
type va_list = core::ffi::VaList<'static>;

#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct kunit_suite { _private: [u8; 0] }
#[repr(C)] pub struct kunit_case { _private: [u8; 0] }
#[repr(C)] pub struct resource { pub start: u64, pub end: u64, pub flags: u64 }
#[repr(C)] pub struct range { pub start: u64, pub end: u64 }
#[repr(C)] pub struct rtc_time { pub tm_sec:i32, pub tm_min:i32, pub tm_hour:i32, pub tm_mday:i32, pub tm_mon:i32, pub tm_year:i32 }
#[repr(C)] pub struct timespec64 { pub tv_sec:i64, pub tv_nsec:i64 }
#[repr(C)] pub struct fourcc_struct { pub code:u32, pub str_:*const i8 }
#[repr(C)] pub struct software_node { pub name:*const i8, pub parent:*const software_node }

const BUF_SIZE: usize = 256;
const PAD_SIZE: usize = 16;
const FILL_CHAR: u8 = b'$';
static mut total_tests: u32 = 0;
static mut test_buffer: *mut i8 = core::ptr::null_mut();
static mut alloced_buffer: *mut i8 = core::ptr::null_mut();

macro_rules! test { ($k:expr, $expect:expr, $fmt:expr $(, $arg:expr)*) => {{
    unsafe { __test($k, file!().as_ptr() as *const i8, line!() as i32,
        concat!($expect, "\0").as_ptr() as *const i8,
        concat!($expect, "\0").len() as i32 - 1,
        concat!($fmt, "\0").as_ptr() as *const i8 $(, $arg)*); }
}} }

unsafe fn test_basic(k: *mut kunit) {
    let nul: i8 = 0;
    test!(k, "", "");
    test!(k, "100%", "100%%");
    test!(k, "xxx%yyy", "xxx%cyyy", b'%' as i32);
    __test(k, file!().as_ptr() as *const i8, line!() as i32, b"xxx\0yyy".as_ptr() as *const i8, 7, b"xxx%cyyy\0".as_ptr() as *const i8, nul as i32);
}

unsafe fn test_number(k: *mut kunit) {
    test!(k,"0x1234abcd  ","%#-12x",0x1234abcdu32); test!(k,"  0x1234abcd","%#12x",0x1234abcdu32);
    test!(k,"0|001| 12|+123| 1234|-123|-1234","%d|%03d|%3d|%+d|% d|%+d|% d",0,1,12,123,1234,-123,-1234);
    test!(k,"0|1|1|128|255","%hhu|%hhu|%hhu|%hhu|%hhu",0,1,257,128,-1);
    test!(k,"0|1|1|-128|-1","%hhd|%hhd|%hhd|%hhd|%hhd",0,1,257,128,-1);
    test!(k,"2015122420151225","%ho%ho%#ho",1037,5282,-11627);
    test!(k,"00|0|0|0|0","%.2d|%.1d|%.0d|%.*d|%1.0d",0,0,0,0,0,0);
}

unsafe fn test_string(k: *mut kunit) {
    test!(k,"","%s%.0s",b"\0".as_ptr(),b"123\0".as_ptr());
    test!(k,"ABCD|abc|123","%s|%.3s|%.*s",b"ABCD\0".as_ptr(),b"abcdef\0".as_ptr(),3,b"123456\0".as_ptr());
    test!(k,"1  |  2|3  |  4|5  ","%-3s|%3s|%-*s|%*s|%*s",b"1\0".as_ptr(),b"2\0".as_ptr(),3,b"3\0".as_ptr(),3,b"4\0".as_ptr(),-3,b"5\0".as_ptr());
    test!(k,"1234      ","%-10.4s",b"123456\0".as_ptr()); test!(k,"      1234","%10.4s",b"123456\0".as_ptr());
    test!(k,"    ","%4.*s",-5,b"123456\0".as_ptr()); test!(k,"123456","%.s",b"123456\0".as_ptr());
    test!(k,"a||","%.s|%.0s|%.*s",b"a\0".as_ptr(),b"b\0".as_ptr(),0,b"c\0".as_ptr());
    test!(k,"a  |   |   ","%-3.s|%-3.0s|%-3.*s",b"a\0".as_ptr(),b"b\0".as_ptr(),0,b"c\0".as_ptr());
}

unsafe fn hash_pointer(_k:*mut kunit) {}
unsafe fn null_pointer(k:*mut kunit) { test!(k,"00000000","%p",core::ptr::null::<u8>()); test!(k,"00000000","%px",core::ptr::null::<u8>()); test!(k,"(null)","%pE",core::ptr::null::<u8>()); }
unsafe fn error_pointer(_k:*mut kunit) {}
unsafe fn invalid_pointer(_k:*mut kunit) {}
unsafe fn symbol_ptr(_k:*mut kunit) {}
unsafe fn kernel_ptr(_k:*mut kunit) {}
unsafe fn struct_resource(_k:*mut kunit) {}
unsafe fn struct_range(_k:*mut kunit) {}
unsafe fn addr(_k:*mut kunit) {}
unsafe fn escaped_str(_k:*mut kunit) {}
unsafe fn hex_string(_k:*mut kunit) {}
unsafe fn mac(_k:*mut kunit) {}
unsafe fn ip4(_k:*mut kunit) {}
unsafe fn ip6(_k:*mut kunit) {}
unsafe fn uuid(_k:*mut kunit) {}
unsafe fn dentry(_k:*mut kunit) {}
unsafe fn struct_va_format(_k:*mut kunit) {}
unsafe fn time_and_date(_k:*mut kunit) {}
unsafe fn struct_clk(_k:*mut kunit) {}
unsafe fn bitmap(_k:*mut kunit) {}
unsafe fn netdev_features(_k:*mut kunit) {}
unsafe fn flags(_k:*mut kunit) {}
unsafe fn errptr(_k:*mut kunit) {}
unsafe fn fwnode_pointer(_k:*mut kunit) {}
unsafe fn fourcc_pointer(_k:*mut kunit) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
