/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of asm/uaccess_64.h.  C include dependencies remain external.

/* User space memory access functions. */

/// Test whether a block of memory is a valid user-space address.
#[inline]
pub fn __chk_range_not_ok(addr: usize, size: usize, limit: usize) -> bool {
    // `__builtin_constant_p(size)` is a build-time C property; callers may
    // use the constant-size path when applicable.
    if size == 0 {
        return addr > limit;
    }
    let end = addr.wrapping_add(size);
    if end < size {
        return true;
    }
    end > limit
}

// __range_not_ok also performs the architecture/compiler __chk_user_ptr check.
#[macro_export]
macro_rules! __range_not_ok {
    ($addr:expr, $size:expr, $limit:expr) => {
        $crate::__chk_range_not_ok($addr as usize, $size as usize, $limit as usize)
    };
}

extern "C" {
    pub fn __retl_efault();
    pub fn __put_user_bad() -> i32;
    pub fn __get_user_bad() -> i32;

    pub fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, size: usize) -> usize;
    pub fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, size: usize) -> usize;
    pub fn raw_copy_in_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, size: usize) -> usize;
    pub fn __clear_user(to: *mut core::ffi::c_void, size: usize) -> usize;
    pub fn strnlen_user(str_: *const core::ffi::c_char, n: isize) -> isize;
    pub fn compute_effective_address(regs: *mut pt_regs, insn: u32, rd: u32) -> usize;
}

pub struct __large_struct {
    pub buf: [usize; 100],
}

#[inline]
pub unsafe fn __m(x: usize) -> *mut __large_struct {
    x as *mut __large_struct
}

// The following operations correspond to the SPARC fault-recovering inline
// assembly in the C header.  The architecture-specific assembly is retained
// textually because it is meaningful only when compiled for sparc64.
#[macro_export]
macro_rules! __put_kernel_nofault {
    ($dst:expr, $src:expr, $ty:ty, $label:lifetime) => {{
        let addr = $dst as *mut $ty;
        let data = *(($src) as *const $ty);
        let __pu_ret: i32 = match core::mem::size_of::<$ty>() {
            1 | 2 | 4 | 8 => { core::ptr::write_volatile(addr, data); 0 },
            _ => unsafe { $crate::__put_user_bad() },
        };
        if __pu_ret != 0 { break $label; }
    }};
}

#[macro_export]
macro_rules! __put_user_nocheck {
    ($data:expr, $addr:expr, $size:expr) => {{
        let __pu_ret: i32 = match $size {
            1 | 2 | 4 | 8 => { core::ptr::write_volatile($addr as *mut _, $data); 0 },
            _ => unsafe { $crate::__put_user_bad() },
        };
        __pu_ret
    }};
}

#[macro_export]
macro_rules! put_user {
    ($x:expr, $ptr:expr) => {{
        let __pu_addr = $ptr as usize;
        $crate::__put_user_nocheck!($x, __pu_addr, core::mem::size_of_val(&*$ptr))
    }};
}

#[macro_export]
macro_rules! __put_user { ($x:expr, $ptr:expr) => { $crate::put_user!($x, $ptr) }; }

#[macro_export]
macro_rules! __get_user_nocheck {
    ($data:expr, $addr:expr, $size:expr, $ty:ty) => {{
        let (__gu_val, __gu_ret): ($ty, i32) = match $size {
            1 | 2 | 4 | 8 => (core::ptr::read_volatile($addr as *const $ty), 0),
            _ => (0 as $ty, unsafe { $crate::__get_user_bad() }),
        };
        $data = __gu_val;
        __gu_ret
    }};
}

#[macro_export]
macro_rules! get_user {
    ($x:expr, $ptr:expr) => {{
        let __gu_addr = $ptr as usize;
        $crate::__get_user_nocheck!($x, __gu_addr, core::mem::size_of_val(&*$ptr), _)
    }};
}

#[macro_export]
macro_rules! __get_user { ($x:expr, $ptr:expr) => { $crate::get_user!($x, $ptr) }; }

pub const INLINE_COPY_USER: bool = true;

pub type __user_ptr<T> = *mut T;

pub struct pt_regs;

#[inline]
pub unsafe fn clear_user(to: *mut core::ffi::c_void, size: usize) -> usize {
    __clear_user(to, size)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
