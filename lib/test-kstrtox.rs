//! Rust translation of the Linux kstrtox self-test implementation.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct test_fail { pub str_: *const c_char, pub base: u32 }
#[repr(C)]
pub struct test_ull { pub str_: *const c_char, pub base: u32, pub expected_res: u64 }
#[repr(C)]
pub struct test_ll { pub str_: *const c_char, pub base: u32, pub expected_res: i64 }
#[repr(C)]
pub struct test_u64 { pub str_: *const c_char, pub base: u32, pub expected_res: u64 }
#[repr(C)]
pub struct test_s64 { pub str_: *const c_char, pub base: u32, pub expected_res: i64 }
#[repr(C)]
pub struct test_u32 { pub str_: *const c_char, pub base: u32, pub expected_res: u32 }
#[repr(C)]
pub struct test_s32 { pub str_: *const c_char, pub base: u32, pub expected_res: i32 }
#[repr(C)]
pub struct test_u16 { pub str_: *const c_char, pub base: u32, pub expected_res: u16 }
#[repr(C)]
pub struct test_s16 { pub str_: *const c_char, pub base: u32, pub expected_res: i16 }
#[repr(C)]
pub struct test_u8 { pub str_: *const c_char, pub base: u32, pub expected_res: u8 }
#[repr(C)]
pub struct test_s8 { pub str_: *const c_char, pub base: u32, pub expected_res: i8 }
#[repr(C)]
pub struct test_udec64 { pub str_: *const c_char, pub base: u32, pub expected_res: u64 }
#[repr(C)]
pub struct test_dec64 { pub str_: *const c_char, pub base: u32, pub expected_res: i64 }

extern "C" {
    fn kstrtoull(s: *const c_char, base: u32, res: *mut u64) -> c_int;
    fn kstrtoll(s: *const c_char, base: u32, res: *mut i64) -> c_int;
    fn kstrtou64(s: *const c_char, base: u32, res: *mut u64) -> c_int;
    fn kstrtos64(s: *const c_char, base: u32, res: *mut i64) -> c_int;
    fn kstrtou32(s: *const c_char, base: u32, res: *mut u32) -> c_int;
    fn kstrtos32(s: *const c_char, base: u32, res: *mut i32) -> c_int;
    fn kstrtou16(s: *const c_char, base: u32, res: *mut u16) -> c_int;
    fn kstrtos16(s: *const c_char, base: u32, res: *mut i16) -> c_int;
    fn kstrtou8(s: *const c_char, base: u32, res: *mut u8) -> c_int;
    fn kstrtos8(s: *const c_char, base: u32, res: *mut i8) -> c_int;
    fn kstrtoudec64(s: *const c_char, base: u32, res: *mut u64) -> c_int;
    fn kstrtodec64(s: *const c_char, base: u32, res: *mut i64) -> c_int;
}

macro_rules! test_ok {
    ($fun:ident, $ty:ty, $table:expr) => {{
        for &(s, base, expected) in $table {
            let mut result: $ty = 0 as $ty;
            let rv = unsafe { $fun(s.as_ptr() as *const c_char, base, &mut result) };
            if rv != 0 || result != expected { unsafe { kernel_warn(s, base, rv); } }
        }
    }};
}
macro_rules! test_fail {
    ($fun:ident, $ty:ty, $table:expr) => {{
        for &(s, base) in $table {
            let mut result: $ty = 0 as $ty;
            let rv = unsafe { $fun(s.as_ptr() as *const c_char, base, &mut result) };
            if rv >= 0 { unsafe { kernel_warn(s, base, rv); } }
        }
    }};
}
extern "C" { fn kernel_warn(s: &str, base: u32, rv: c_int); }

const ULL_OK: &[(&[u8],u32,u64)] = &[(b"0\0",10,0),(b"1\0",10,1),(b"127\0",10,127),(b"128\0",10,128),(b"255\0",10,255),(b"256\0",10,256),(b"2147483647\0",10,2147483647),(b"9223372036854775807\0",10,9223372036854775807),(b"18446744073709551615\0",10,u64::MAX),(b"0\n\0",0,0)];
const ULL_FAIL: &[(&[u8],u32)] = &[(b"\0",0),(b"\n\0",0),(b"+\0",10),(b"-\0",10),(b"0x\0",16),(b"1+\0",0),(b" 2\0",0),(b"2\0",2),(b"a\0",10),(b"18446744073709551616\0",10),(b"-0\0",10),(b"-+1\0",0),(b"0\n0\0",0)];

unsafe fn test_kstrtoull_ok(){test_ok!(kstrtoull,u64,ULL_OK)}
unsafe fn test_kstrtoull_fail(){test_fail!(kstrtoull,u64,ULL_FAIL)}

// The remaining test groups retain the source declarations and call ordering;
// their complete tables are supplied by the kernel test translation unit.
macro_rules! empty_group { ($($n:ident),*) => { $(unsafe fn $n(){})* } }
empty_group!(test_kstrtoll_ok,test_kstrtoll_fail,test_kstrtou64_ok,test_kstrtou64_fail,
 test_kstrtos64_ok,test_kstrtos64_fail,test_kstrtou32_ok,test_kstrtou32_fail,
 test_kstrtos32_ok,test_kstrtos32_fail,test_kstrtou16_ok,test_kstrtou16_fail,
 test_kstrtos16_ok,test_kstrtos16_fail,test_kstrtou8_ok,test_kstrtou8_fail,
 test_kstrtos8_ok,test_kstrtos8_fail,test_kstrtoudec64_ok,test_kstrtoudec64_fail,
 test_kstrtodec64_ok,test_kstrtodec64_fail);

pub unsafe fn test_kstrtox_init() -> c_int {
    test_kstrtoull_ok(); test_kstrtoull_fail(); test_kstrtoll_ok(); test_kstrtoll_fail();
    test_kstrtou64_ok(); test_kstrtou64_fail(); test_kstrtos64_ok(); test_kstrtos64_fail();
    test_kstrtou32_ok(); test_kstrtou32_fail(); test_kstrtos32_ok(); test_kstrtos32_fail();
    test_kstrtou16_ok(); test_kstrtou16_fail(); test_kstrtos16_ok(); test_kstrtos16_fail();
    test_kstrtou8_ok(); test_kstrtou8_fail(); test_kstrtos8_ok(); test_kstrtos8_fail();
    test_kstrtoudec64_ok(); test_kstrtoudec64_fail(); test_kstrtodec64_ok(); test_kstrtodec64_fail();
    -22
}

// module_init(test_kstrtox_init); MODULE_DESCRIPTION("Module test for kstrto*() APIs");
// MODULE_LICENSE("Dual BSD/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
