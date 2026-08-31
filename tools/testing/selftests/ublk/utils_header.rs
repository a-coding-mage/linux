/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from testing/selftests/ublk/utils.h.
 * C include/header guards are intentionally omitted. This header depends on
 * libc/POSIX CPU set and stdio/varargs facilities supplied by surrounding code.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

pub const fn min<T: Ord + Copy>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

#[macro_export]
macro_rules! ARRAY_SIZE {
    ($x:expr) => {
        ::core::mem::size_of_val(&$x) / ::core::mem::size_of_val(&$x[0])
    };
}

#[macro_export]
macro_rules! offsetof {
    ($type:ty, $member:tt) => {
        ::core::ptr::addr_of!((*(::core::ptr::null::<$type>())).$member) as usize
    };
}

#[macro_export]
macro_rules! container_of {
    ($ptr:expr, $type:ty, $member:tt) => {{
        let __mptr: c_ulong = $ptr as c_ulong;
        (__mptr - offsetof!($type, $member)) as *mut $type
    }};
}

#[macro_export]
macro_rules! round_up {
    ($val:expr, $rnd:expr) => {
        (($val + ($rnd - 1)) & !(($rnd) - 1))
    };
}

/* small sized & per-thread allocator */
#[repr(C)]
pub struct allocator {
    pub size: c_uint,
    pub set: *mut libc::cpu_set_t,
}

pub unsafe fn allocator_init(a: *mut allocator, size: c_uint) -> c_int {
    unsafe {
        (*a).set = libc::CPU_ALLOC(size as libc::c_int);
        (*a).size = size;

        if !(*a).set.is_null() {
            return 0;
        }
        -libc::ENOMEM
    }
}

pub unsafe fn allocator_deinit(a: *mut allocator) {
    unsafe {
        libc::CPU_FREE((*a).set);
        (*a).set = ptr::null_mut();
        (*a).size = 0;
    }
}

pub unsafe fn allocator_get(a: *mut allocator) -> c_int {
    let mut i: c_int;

    unsafe {
        i = 0;
        while i < (*a).size as c_int {
            let set_size: libc::size_t = libc::CPU_ALLOC_SIZE((*a).size as libc::c_int);

            if libc::CPU_ISSET_S(i as libc::c_int, set_size, (*a).set) == 0 {
                libc::CPU_SET_S(i as libc::c_int, set_size, (*a).set);
                return i;
            }

            i += 1;
        }
    }

    -1
}

pub unsafe fn allocator_put(a: *mut allocator, i: c_int) {
    unsafe {
        let set_size: libc::size_t = libc::CPU_ALLOC_SIZE((*a).size as libc::c_int);

        if i >= 0 && i < (*a).size as c_int {
            libc::CPU_CLR_S(i as libc::c_int, set_size, (*a).set);
        }
    }
}

pub unsafe fn allocator_get_val(a: *mut allocator, i: c_int) -> c_int {
    unsafe {
        let set_size: libc::size_t = libc::CPU_ALLOC_SIZE((*a).size as libc::c_int);

        libc::CPU_ISSET_S(i as libc::c_int, set_size, (*a).set)
    }
}

pub fn ilog2(x: c_uint) -> c_uint {
    if x == 0 {
        return 0;
    }
    ((size_of::<c_uint>() * 8 - 1) as c_uint) - x.leading_zeros()
}

pub const UBLK_DBG_DEV: c_uint = 1_u32 << 0;
pub const UBLK_DBG_THREAD: c_uint = 1_u32 << 1;
pub const UBLK_DBG_IO_CMD: c_uint = 1_u32 << 2;
pub const UBLK_DBG_IO: c_uint = 1_u32 << 3;
pub const UBLK_DBG_CTRL_CMD: c_uint = 1_u32 << 4;
pub const UBLK_LOG: c_uint = 1_u32 << 5;

unsafe extern "C" {
    pub static mut ublk_dbg_mask: c_uint;
    pub static mut stderr: *mut libc::FILE;
    pub static mut stdout: *mut libc::FILE;

    pub fn vfprintf(stream: *mut libc::FILE, format: *const c_char, ap: libc::va_list) -> c_int;
    pub fn assert(expression: c_int);
}

/*
 * C varargs cannot be represented as a normal safe Rust function. These keep
 * the source-level signatures and bodies in C-variadic Rust form.
 */
pub unsafe extern "C" fn ublk_err(fmt: *const c_char, mut args: ...) {
    unsafe {
        vfprintf(stderr, fmt, args.as_va_list());
    }
}

pub unsafe extern "C" fn ublk_log(fmt: *const c_char, mut args: ...) {
    unsafe {
        if (ublk_dbg_mask & UBLK_LOG) != 0 {
            vfprintf(stdout, fmt, args.as_va_list());
        }
    }
}

pub unsafe extern "C" fn ublk_dbg(level: c_int, fmt: *const c_char, mut args: ...) {
    unsafe {
        if (level as c_uint & ublk_dbg_mask) != 0 {
            vfprintf(stdout, fmt, args.as_va_list());
        }
    }
}

#[macro_export]
macro_rules! ublk_assert {
    ($x:expr) => {{
        if !$x {
            unsafe {
                ublk_err(
                    concat!("%s %d: assert!\n", "\0").as_ptr() as *const c_char,
                    concat!(module_path!(), "\0").as_ptr() as *const c_char,
                    line!() as c_int,
                );
                assert($x as c_int);
            }
        }
    }};
}
