/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation: asm-generic/access_ok.h

/* These transfer routines select the access width from the pointed-to type. */
macro_rules! put_user {
    ($x:expr, $ptr:expr) => {
        __put_user_check($x as _, $ptr, core::mem::size_of_val(unsafe { &*$ptr }))
    };
}

macro_rules! get_user {
    ($x:expr, $ptr:expr) => {
        __get_user_check($x, $ptr, core::mem::size_of_val(unsafe { &*$ptr }))
    };
}

/* The __xxx versions do not perform address-space checking. */
macro_rules! __put_user {
    ($x:expr, $ptr:expr) => {
        __put_user_nocheck($x as _, $ptr, core::mem::size_of_val(unsafe { &*$ptr }))
    };
}

macro_rules! __get_user {
    ($x:expr, $ptr:expr) => {
        __get_user_nocheck($x, $ptr, core::mem::size_of_val(unsafe { &*$ptr }))
    };
}

// The C EXC macro emits Alpha exception-table assembler.
macro_rules! EXC {
    ($label:tt, $cont:tt, $res:tt, $err:tt) => {
        ".section __ex_table,\"a\"\n\t.long " $label "-.\n\tlda " $res "," $cont "-" $label "(" $err ")\n.previous\n"
    };
}

unsafe extern "C" {
    pub fn __get_user_unknown();
    pub fn __put_user_unknown();
    pub fn __copy_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, len: i64) -> i64;
    pub fn __clear_user(to: *mut core::ffi::c_void, len: i64) -> i64;
    pub fn strncpy_from_user(dest: *mut i8, src: *const i8, count: i64) -> i64;
    pub fn strnlen_user(str_: *const i8, n: i64) -> i64;
}

#[repr(C)]
pub struct __large_struct {
    pub buf: [u64; 100],
}

macro_rules! __m {
    ($x:expr) => { &*($x as *const __large_struct) };
}

// Alpha inline assembly is preserved as source-level intent; these operations
// require the target-specific assembler and exception-table integration.
macro_rules! __get_user_64 { ($addr:expr) => {{ /* ldq + EXC */ }}; }
macro_rules! __get_user_32 { ($addr:expr) => {{ /* ldl + EXC */ }}; }
macro_rules! __get_user_16 { ($addr:expr) => {{ /* ldwu + EXC */ }}; }
macro_rules! __get_user_8  { ($addr:expr) => {{ /* ldbu + EXC */ }}; }

macro_rules! __get_user_nocheck {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __gu_err: i64 = 0;
        let mut __gu_val: u64 = 0;
        __chk_user_ptr!($ptr);
        match $size {
            1 => __get_user_8!($ptr),
            2 => __get_user_16!($ptr),
            4 => __get_user_32!($ptr),
            8 => __get_user_64!($ptr),
            _ => unsafe { __get_user_unknown() },
        }
        $x = __gu_val as _;
        __gu_err
    }};
}

macro_rules! __get_user_check {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __gu_err: i64 = -(EFAULT as i64);
        let mut __gu_val: u64 = 0;
        let __gu_addr = $ptr;
        if __access_ok(__gu_addr, $size) {
            __gu_err = 0;
            match $size {
                1 => __get_user_8!(__gu_addr),
                2 => __get_user_16!(__gu_addr),
                4 => __get_user_32!(__gu_addr),
                8 => __get_user_64!(__gu_addr),
                _ => unsafe { __get_user_unknown() },
            }
        }
        $x = __gu_val as _;
        __gu_err
    }};
}

macro_rules! __put_user_64 { ($x:expr, $addr:expr) => {{ /* stq + EXC */ }}; }
macro_rules! __put_user_32 { ($x:expr, $addr:expr) => {{ /* stl + EXC */ }}; }
macro_rules! __put_user_16 { ($x:expr, $addr:expr) => {{ /* stw + EXC */ }}; }
macro_rules! __put_user_8  { ($x:expr, $addr:expr) => {{ /* stb + EXC */ }}; }

macro_rules! __put_user_nocheck {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __pu_err: i64 = 0;
        __chk_user_ptr!($ptr);
        match $size {
            1 => __put_user_8!($x, $ptr),
            2 => __put_user_16!($x, $ptr),
            4 => __put_user_32!($x, $ptr),
            8 => __put_user_64!($x, $ptr),
            _ => unsafe { __put_user_unknown() },
        }
        __pu_err
    }};
}

macro_rules! __put_user_check {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __pu_err: i64 = -(EFAULT as i64);
        let __pu_addr = $ptr;
        if __access_ok(__pu_addr, $size) {
            __pu_err = 0;
            match $size {
                1 => __put_user_8!($x, __pu_addr),
                2 => __put_user_16!($x, __pu_addr),
                4 => __put_user_32!($x, __pu_addr),
                8 => __put_user_64!($x, __pu_addr),
                _ => unsafe { __put_user_unknown() },
            }
        }
        __pu_err
    }};
}

pub unsafe fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, len: usize) -> usize {
    __copy_user(to, from, len as i64) as usize
}

pub unsafe fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, len: usize) -> usize {
    __copy_user(to, from, len as i64) as usize
}

pub unsafe fn clear_user(to: *mut core::ffi::c_void, mut len: i64) -> i64 {
    if __access_ok(to, len) {
        len = __clear_user(to, len);
    }
    len
}

// Dependency supplied by the surrounding kernel translation: asm/extable.h

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
