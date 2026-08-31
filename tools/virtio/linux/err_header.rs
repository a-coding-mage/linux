/* SPDX-License-Identifier: GPL-2.0 */
/* Dependency intent from C: #include <linux/kernel.h> for unlikely(). */

pub const MAX_ERRNO: c_ulong = 4095;

#[inline]
pub unsafe fn IS_ERR_VALUE(x: c_ulong) -> bool {
    unsafe { unlikely(x >= (-(MAX_ERRNO as c_long)) as c_ulong) }
}

#[inline]
pub unsafe fn ERR_PTR(error: c_long) -> *mut c_void {
    error as *mut c_void
}

#[inline]
pub unsafe fn PTR_ERR(ptr: *const c_void) -> c_long {
    ptr as c_long
}

#[inline]
pub unsafe fn IS_ERR(ptr: *const c_void) -> c_long {
    unsafe { IS_ERR_VALUE(ptr as c_ulong) as c_long }
}

#[inline]
pub unsafe fn IS_ERR_OR_NULL(ptr: *const c_void) -> c_long {
    (ptr.is_null() || unsafe { IS_ERR_VALUE(ptr as c_ulong) }) as c_long
}
