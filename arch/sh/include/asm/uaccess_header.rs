/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: <asm/extable.h>, <asm-generic/access_ok.h>, and
// <asm/uaccess_32.h> are supplied by other translation units.

/*
 * Uh, these should become the main single-value transfer routines ...
 * They automatically use the right size if we just have the right
 * pointer type ...
 *
 * As SuperH uses the same address space for kernel and user data, we
 * can just do these as direct assignments.
 *
 * Careful to not
 * (a) re-use the arguments for side effects (sizeof is ok)
 * (b) require any knowledge of processes at this stage
 */
macro_rules! put_user {
    ($x:expr, $ptr:expr) => { __put_user_check!($x, $ptr, core::mem::size_of_val(unsafe { &*$ptr })) };
}
macro_rules! get_user {
    ($x:expr, $ptr:expr) => { __get_user_check!($x, $ptr, core::mem::size_of_val(unsafe { &*$ptr })) };
}

/* The __xxx versions do not do address space checking. */
macro_rules! __put_user {
    ($x:expr, $ptr:expr) => { __put_user_nocheck!($x, $ptr, core::mem::size_of_val(unsafe { &*$ptr })) };
}
macro_rules! __get_user {
    ($x:expr, $ptr:expr) => { __get_user_nocheck!($x, $ptr, core::mem::size_of_val(unsafe { &*$ptr })) };
}

#[repr(C)]
pub struct __large_struct {
    pub buf: [core::ffi::c_ulong; 100],
}

// C's __user annotation is an address-space type qualifier.
macro_rules! __m {
    ($x:expr) => { *(($x) as *mut __large_struct) };
}

macro_rules! __get_user_nocheck {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __gu_err: core::ffi::c_long;
        let mut __gu_val: core::ffi::c_ulong;
        let __gu_addr = $ptr;
        __chk_user_ptr!($ptr);
        __get_user_size!(__gu_val, __gu_addr, $size, __gu_err);
        $x = __gu_val as _;
        __gu_err
    }};
}

macro_rules! __get_user_check {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __gu_err: core::ffi::c_long = -EFAULT;
        let mut __gu_val: core::ffi::c_ulong = 0;
        let __gu_addr = $ptr;
        if likely!(access_ok!(__gu_addr, $size)) {
            __get_user_size!(__gu_val, __gu_addr, $size, __gu_err);
        }
        $x = __gu_val as _;
        __gu_err
    }};
}

macro_rules! __put_user_nocheck {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __pu_err: core::ffi::c_long;
        let __pu_addr = $ptr;
        let __pu_val = $x;
        __chk_user_ptr!($ptr);
        __put_user_size!(__pu_val, __pu_addr, $size, __pu_err);
        __pu_err
    }};
}

macro_rules! __put_user_check {
    ($x:expr, $ptr:expr, $size:expr) => {{
        let mut __pu_err: core::ffi::c_long = -EFAULT;
        let __pu_addr = $ptr;
        let __pu_val = $x;
        if likely!(access_ok!(__pu_addr, $size)) {
            __put_user_size!(__pu_val, __pu_addr, $size, __pu_err);
        }
        __pu_err
    }};
}

extern "C" {
    pub fn strncpy_from_user(dest: *mut core::ffi::c_char, src: *const core::ffi::c_char, count: core::ffi::c_long) -> core::ffi::c_long;
    pub fn strnlen_user(str_: *const core::ffi::c_char, n: core::ffi::c_long) -> core::ffi::c_long;
    pub fn __copy_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
}

#[inline(always)]
pub unsafe fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: core::ffi::c_ulong) -> core::ffi::c_ulong {
    __copy_user(to, from, n as usize) as core::ffi::c_ulong
}

#[inline(always)]
pub unsafe fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: core::ffi::c_ulong) -> core::ffi::c_ulong {
    __copy_user(to, from, n as usize) as core::ffi::c_ulong
}

// C build marker: #define INLINE_COPY_USER

extern "C" {
    pub fn __clear_user(addr: *mut core::ffi::c_void, size: usize) -> usize;
    pub fn set_exception_table_vec(vec: core::ffi::c_uint, handler: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
}

macro_rules! clear_user {
    ($addr:expr, $n:expr) => {{
        let __cl_addr = $addr;
        let mut __cl_size = $n;
        if __cl_size != 0 && access_ok!(__cl_addr, __cl_size) {
            __cl_size = unsafe { __clear_user(__cl_addr, __cl_size) };
        }
        __cl_size
    }};
}

#[inline]
pub unsafe fn set_exception_table_evt(evt: core::ffi::c_uint, handler: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    set_exception_table_vec(evt >> 5, handler)
}

#[repr(C)]
pub struct mem_access {
    pub from: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void, core::ffi::c_ulong) -> core::ffi::c_ulong>,
    pub to: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_void, core::ffi::c_ulong) -> core::ffi::c_ulong>,
}

extern "C" {
    pub fn handle_unaligned_access(
        instruction: insn_size_t,
        regs: *mut pt_regs,
        ma: *mut mem_access,
        arg: core::ffi::c_int,
        address: core::ffi::c_ulong,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
