/* SPDX-License-Identifier: GPL-2.0 */

pub const MAX_ERRNO: usize = 4095;

#[macro_export]
macro_rules! IS_ERR_VALUE {
    ($x:expr) => {
        ($x as *const core::ffi::c_void as usize) >= (0usize.wrapping_sub(MAX_ERRNO))
    };
}

#[macro_export]
macro_rules! __STR {
    ($x:tt) => {
        stringify!($x)
    };
}

#[macro_export]
macro_rules! set_if_not_errno_or_zero {
    ($x:expr, $y:tt) => {{
        unsafe {
            core::arch::asm!(
                "if {0} s< -4095 goto +1",
                "if {0} s<= 0 goto +1",
                concat!("{0} = ", stringify!($y)),
                inout(reg) $x,
                options(nostack, preserves_flags),
            );
        }
    }};
}

#[inline]
pub unsafe fn IS_ERR_OR_NULL(ptr: *const core::ffi::c_void) -> i32 {
    (ptr.is_null() || IS_ERR_VALUE!(ptr as usize)) as i32
}

#[inline]
pub unsafe fn PTR_ERR(ptr: *const core::ffi::c_void) -> isize {
    ptr as isize
}
