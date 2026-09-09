/* SPDX-License-Identifier: GPL-2.0 */

/*
 * User space memory access functions, for machines with kernel and user data
 * in the same address space.
 *
 * C dependencies from linux/string.h, asm-generic/access_ok.h, and
 * asm/extable.h are supplied by other translated files.
 */

/* CONFIG_UACCESS_MEMCPY conditional section. */
#[cfg(CONFIG_UACCESS_MEMCPY)]
#[inline(always)]
pub unsafe fn __get_user_fn(size: usize, from: *const core::ffi::c_void, to: *mut core::ffi::c_void) -> i32 {
    // BUILD_BUG_ON(!__builtin_constant_p(size));
    match size {
        1 => {
            *(to as *mut u8) = *(from as *const u8);
            0
        }
        2 => {
            *(to as *mut u16) = core::ptr::read_unaligned(from as *const u16);
            0
        }
        4 => {
            *(to as *mut u32) = core::ptr::read_unaligned(from as *const u32);
            0
        }
        8 => {
            *(to as *mut u64) = core::ptr::read_unaligned(from as *const u64);
            0
        }
        _ => {
            // BUILD_BUG();
            0
        }
    }
}

#[cfg(CONFIG_UACCESS_MEMCPY)]
#[inline(always)]
pub unsafe fn __put_user_fn(size: usize, to: *mut core::ffi::c_void, from: *mut core::ffi::c_void) -> i32 {
    // BUILD_BUG_ON(!__builtin_constant_p(size));
    match size {
        1 => {
            *(to as *mut u8) = *(from as *mut u8);
            0
        }
        2 => {
            core::ptr::write_unaligned(to as *mut u16, *(from as *mut u16));
            0
        }
        4 => {
            core::ptr::write_unaligned(to as *mut u32, *(from as *mut u32));
            0
        }
        8 => {
            core::ptr::write_unaligned(to as *mut u64, *(from as *mut u64));
            0
        }
        _ => {
            // BUILD_BUG();
            0
        }
    }
}

#[cfg(CONFIG_UACCESS_MEMCPY)]
#[inline(always)]
pub unsafe fn __get_kernel_nofault<T: Copy>(dst: *mut T, src: *const T) {
    *dst = core::ptr::read_unaligned(src);
}

#[cfg(CONFIG_UACCESS_MEMCPY)]
#[inline(always)]
pub unsafe fn __put_kernel_nofault<T: Copy>(dst: *mut T, src: *const T) {
    core::ptr::write_unaligned(dst, *src);
}

#[cfg(CONFIG_UACCESS_MEMCPY)]
#[inline]
pub unsafe fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, n);
    0
}

#[cfg(CONFIG_UACCESS_MEMCPY)]
#[inline]
pub unsafe fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, n);
    0
}

#[cfg(CONFIG_UACCESS_MEMCPY)]
pub const INLINE_COPY_USER: bool = true;

/* The following macros preserve the C interfaces and require the supplied
 * access_ok, might_fault, __chk_user_ptr, and bad-user helpers. */
#[macro_export]
macro_rules! __put_user {
    ($x:expr, $ptr:expr) => {{
        let mut __x = $x;
        let mut __pu_err: i32 = -EFAULT;
        __chk_user_ptr!($ptr);
        match core::mem::size_of_val(&$ptr) {
            1 | 2 | 4 | 8 => {
                __pu_err = unsafe { __put_user_fn(core::mem::size_of_val(&$ptr), $ptr as *mut _, &mut __x as *mut _ as *mut core::ffi::c_void) };
            }
            _ => { __put_user_bad(); }
        }
        __pu_err
    }};
}

#[macro_export]
macro_rules! put_user {
    ($x:expr, $ptr:expr) => {{
        let __p = $ptr as *mut core::ffi::c_void;
        might_fault!();
        if access_ok!(__p, core::mem::size_of_val(&$ptr)) { __put_user!($x, __p) } else { -EFAULT }
    }};
}

#[cfg(not(CONFIG_UACCESS_MEMCPY))]
#[inline]
pub unsafe fn __put_user_fn(size: usize, ptr: *mut core::ffi::c_void, x: *mut core::ffi::c_void) -> i32 {
    if raw_copy_to_user(ptr, x, size) != 0 { -EFAULT } else { 0 }
}

extern "C" {
    pub fn __put_user_bad() -> !;
}

#[macro_export]
macro_rules! __get_user {
    ($x:expr, $ptr:expr) => {{
        let mut __gu_err: i32 = -EFAULT;
        __chk_user_ptr!($ptr);
        match core::mem::size_of_val(&$ptr) {
            1 | 2 | 4 | 8 => {
                __gu_err = unsafe { __get_user_fn(core::mem::size_of_val(&$ptr), $ptr as *const _, &mut $x as *mut _ as *mut core::ffi::c_void) };
            }
            _ => { __get_user_bad(); }
        }
        __gu_err
    }};
}

#[macro_export]
macro_rules! get_user {
    ($x:expr, $ptr:expr) => {{
        let __p = $ptr as *const core::ffi::c_void;
        might_fault!();
        if access_ok!(__p, core::mem::size_of_val(&$ptr)) { __get_user!($x, __p) } else { $x = 0; -EFAULT }
    }};
}

#[cfg(not(CONFIG_UACCESS_MEMCPY))]
#[inline]
pub unsafe fn __get_user_fn(size: usize, ptr: *const core::ffi::c_void, x: *mut core::ffi::c_void) -> i32 {
    if raw_copy_from_user(x, ptr, size) != 0 { -EFAULT } else { 0 }
}

extern "C" {
    pub fn __get_user_bad() -> !;
}

#[cfg(not(__clear_user))]
#[inline]
pub unsafe fn __clear_user(to: *mut core::ffi::c_void, n: usize) -> usize {
    core::ptr::write_bytes(to as *mut u8, 0, n);
    0
}

#[inline]
pub unsafe fn clear_user(to: *mut core::ffi::c_void, n: usize) -> usize {
    might_fault!();
    if !access_ok!(to, n) { return n; }
    __clear_user(to, n)
}

extern "C" {
    pub fn strncpy_from_user(dst: *mut i8, src: *const i8, count: isize) -> isize;
    pub fn strnlen_user(src: *const i8, n: isize) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
