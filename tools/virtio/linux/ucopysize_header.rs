// SPDX-License-Identifier: GPL-2.0
// C header dependency: <linux/bug.h>

#[inline]
pub unsafe fn check_object_size(_ptr: *const core::ffi::c_void, _n: u64, _to_user: bool) {}

#[inline]
pub fn copy_overflow(_size: core::ffi::c_int, _count: u64) {}

#[inline(always)]
#[must_use]
pub unsafe fn check_copy_size(
    _addr: *const core::ffi::c_void,
    _bytes: usize,
    _is_source: bool,
) -> bool {
    true
}
