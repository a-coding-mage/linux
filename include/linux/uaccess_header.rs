/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/uaccess.h. Required architecture and kernel symbols
// are intentionally left as external dependencies.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub type size_t = usize;

extern "C" {
    fn raw_copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn raw_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn instrument_copy_from_user_before(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize);
    fn instrument_copy_from_user_after(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize, res: usize);
    fn instrument_copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize);
    fn check_object_size(ptr: *const core::ffi::c_void, n: usize, to_user: bool);
    fn should_fail_usercopy() -> bool;
    fn might_fault();
    fn access_ok(ptr: *const core::ffi::c_void, n: usize) -> bool;
    fn barrier_nospec();
    fn barrier();
    fn check_copy_size(ptr: *const core::ffi::c_void, n: usize, to_user: bool) -> bool;
    fn check_zeroed_user(from: *const core::ffi::c_void, size: usize) -> i32;
    fn clear_user(dst: *mut core::ffi::c_void, n: usize) -> usize;
    fn __get_user<T>(val: *mut T, ptr: *const T) -> i32;
    fn __put_user<T>(val: T, ptr: *mut T) -> i32;
    fn in_atomic() -> bool;
    fn smp_mb();
    fn memchr_inv(s: *const core::ffi::c_void, c: i32, n: usize) -> *const core::ffi::c_void;
    fn memset(dst: *mut core::ffi::c_void, c: i32, n: usize);
    static mut current: *mut TaskStruct;
}

#[repr(C)]
pub struct TaskStruct { pub pagefault_disabled: i32 }

#[inline(always)]
pub unsafe fn __copy_from_user_inatomic(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
    instrument_copy_from_user_before(to, from, n);
    check_object_size(to, n, false);
    let res = raw_copy_from_user(to, from, n);
    instrument_copy_from_user_after(to, from, n, res);
    res
}

#[inline(always)]
pub unsafe fn __copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
    might_fault();
    instrument_copy_from_user_before(to, from, n);
    if should_fail_usercopy() { return n; }
    check_object_size(to, n, false);
    let res = raw_copy_from_user(to, from, n);
    instrument_copy_from_user_after(to, from, n, res);
    res
}

#[inline(always)]
pub unsafe fn __copy_to_user_inatomic(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
    if should_fail_usercopy() { return n; }
    instrument_copy_to_user(to, from, n);
    check_object_size(from, n, true);
    raw_copy_to_user(to, from, n)
}

#[inline(always)]
pub unsafe fn __copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
    might_fault();
    if should_fail_usercopy() { return n; }
    instrument_copy_to_user(to, from, n);
    check_object_size(from, n, true);
    raw_copy_to_user(to, from, n)
}

extern "C" {
    fn _copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn _copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    pub fn copy_from_kernel_nofault_allowed(src: *const core::ffi::c_void, size: usize) -> bool;
    pub fn copy_from_kernel_nofault(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> isize;
    pub fn copy_to_kernel_nofault(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> isize;
    pub fn copy_from_user_nofault(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> isize;
    pub fn copy_to_user_nofault(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> isize;
    pub fn strncpy_from_kernel_nofault(dst: *mut i8, src: *const core::ffi::c_void, count: isize) -> isize;
    pub fn strncpy_from_user_nofault(dst: *mut i8, src: *const core::ffi::c_void, count: isize) -> isize;
    pub fn strnlen_user_nofault(src: *const core::ffi::c_void, count: isize) -> isize;
}

#[inline(always)]
pub unsafe fn copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
    if !check_copy_size(to, n, false) { n } else { _copy_from_user(to, from, n) }
}

#[inline(always)]
pub unsafe fn copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize {
    if !check_copy_size(from, n, true) { n } else { _copy_to_user(to, from, n) }
}

#[inline(always)]
pub unsafe fn copy_mc_to_kernel(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, cnt: usize) -> usize {
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, cnt); 0
}

#[inline(always)]
pub unsafe fn copy_struct_from_user(dst: *mut u8, ksize: usize, src: *const u8, usize_: usize) -> i32 {
    let size = core::cmp::min(ksize, usize_);
    let rest = core::cmp::max(ksize, usize_) - size;
    if usize_ < ksize { core::ptr::write_bytes(dst.add(size), 0, rest); }
    else if usize_ > ksize {
        let ret = check_zeroed_user(src.add(size) as *const core::ffi::c_void, rest);
        if ret <= 0 { return if ret != 0 { ret } else { -7 }; }
    }
    if copy_from_user(dst as *mut core::ffi::c_void, src as *const core::ffi::c_void, size) != 0 { return -14; }
    0
}

#[inline(always)]
pub unsafe fn copy_struct_to_user(dst: *mut u8, usize_: usize, src: *const u8, ksize: usize, ignored_trailing: *mut bool) -> i32 {
    let size = core::cmp::min(ksize, usize_);
    let rest = core::cmp::max(ksize, usize_) - size;
    if usize_ > ksize && clear_user(dst.add(size) as *mut core::ffi::c_void, rest) != 0 { return -14; }
    if !ignored_trailing.is_null() { *ignored_trailing = usize_ < ksize && !memchr_inv(src.add(size) as *const core::ffi::c_void, 0, rest).is_null(); }
    if copy_to_user(dst as *mut core::ffi::c_void, src as *const core::ffi::c_void, size) != 0 { return -14; }
    0
}

#[inline]
pub unsafe fn pagefault_disabled_inc() { (*current).pagefault_disabled += 1; }
#[inline]
pub unsafe fn pagefault_disabled_dec() { (*current).pagefault_disabled -= 1; }
#[inline]
pub unsafe fn pagefault_disable() { pagefault_disabled_inc(); barrier(); }
#[inline]
pub unsafe fn pagefault_enable() { barrier(); pagefault_disabled_dec(); }
#[inline]
pub unsafe fn pagefault_disabled() -> bool { (*current).pagefault_disabled != 0 }
#[inline]
pub unsafe fn faulthandler_disabled() -> bool { pagefault_disabled() || in_atomic() }

#[inline]
pub unsafe fn probe_subpage_writeable(_uaddr: *mut i8, _size: usize) -> usize { 0 }

#[inline]
pub unsafe fn copy_struct_from_bounce_buffer(dst: *mut u8, dstsize: usize, src: *const u8, srcsize: usize) -> i32 {
    let size = core::cmp::min(dstsize, srcsize);
    if dstsize > srcsize { core::ptr::write_bytes(dst.add(size), 0, dstsize - size); }
    core::ptr::copy_nonoverlapping(src, dst, size);
    0
}

#[inline]
pub unsafe fn copy_struct_to_bounce_buffer(dst: *mut u8, dstsize: usize, src: *const u8, srcsize: usize, ignored_trailing: *mut bool) -> i32 {
    let size = core::cmp::min(dstsize, srcsize);
    if dstsize > srcsize { core::ptr::write_bytes(dst.add(size), 0, dstsize - size); }
    if !ignored_trailing.is_null() { *ignored_trailing = false; }
    core::ptr::copy_nonoverlapping(src, dst, size);
    0
}

#[inline]
pub unsafe fn user_access_save() -> usize { 0 }
#[inline]
pub unsafe fn user_access_restore(_flags: usize) {}

// C preprocessor conditionals and scoped-access cleanup macros are preserved
// as externally supplied architecture/kernel facilities.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
