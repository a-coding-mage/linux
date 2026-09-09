// SPDX-License-Identifier: GPL-2.0-only
/*
 * Access kernel or user memory without faulting.
 */

use core::ffi::c_void;

// Kernel dependencies supplied by other translation units.
extern "C" {
    fn pagefault_disable();
    fn pagefault_enable();
    fn kmsan_check_memory(src: *const c_void, size: usize);
    fn instrument_write(dst: *const c_void, size: usize);
    fn __access_ok(addr: *const c_void, size: usize) -> bool;
    fn access_ok(addr: *const c_void, size: usize) -> bool;
    fn nmi_uaccess_okay() -> bool;
    fn __copy_from_user_inatomic(dst: *mut c_void, src: *const c_void, size: usize) -> usize;
    fn __copy_to_user_inatomic(dst: *mut c_void, src: *const c_void, size: usize) -> usize;
    fn strncpy_from_user(dst: *mut u8, src: *const c_void, count: isize) -> isize;
    fn strnlen_user(src: *const c_void, count: isize) -> i32;
    fn warn(condition: i32, fmt: *const u8, ...);
}

#[no_mangle]
pub unsafe extern "C" fn copy_from_kernel_nofault_allowed(
    _unsafe_src: *const c_void,
    _size: usize,
) -> bool {
    true
}

#[inline]
unsafe fn copy_from_kernel_nofault_loop<T>(
    mut dst: *mut u8,
    mut src: *const u8,
    mut len: usize,
) -> Result<(*mut u8, *const u8, usize), ()> {
    while len >= core::mem::size_of::<T>() {
        core::ptr::copy_nonoverlapping(src, dst, core::mem::size_of::<T>());
        kmsan_check_memory(src as *const c_void, core::mem::size_of::<T>());
        dst = dst.add(core::mem::size_of::<T>());
        src = src.add(core::mem::size_of::<T>());
        len -= core::mem::size_of::<T>();
    }
    Ok((dst, src, len))
}

#[no_mangle]
pub unsafe extern "C" fn copy_from_kernel_nofault(
    dst: *mut c_void,
    src: *const c_void,
    size: usize,
) -> isize {
    let mut align: usize = 0;
    // CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS is a build-time kernel option.
    align = (dst as usize) | (src as usize);

    if !copy_from_kernel_nofault_allowed(src, size) {
        return -34;
    }

    pagefault_disable();
    let (mut d, mut s, mut l) = (dst as *mut u8, src as *const u8, size);
    if align & 7 == 0 { (d, s, l) = copy_from_kernel_nofault_loop::<u64>(d, s, l).unwrap(); }
    if align & 3 == 0 { (d, s, l) = copy_from_kernel_nofault_loop::<u32>(d, s, l).unwrap(); }
    if align & 1 == 0 { (d, s, l) = copy_from_kernel_nofault_loop::<u16>(d, s, l).unwrap(); }
    let _ = copy_from_kernel_nofault_loop::<u8>(d, s, l);
    pagefault_enable();
    0
}

#[inline]
unsafe fn copy_to_kernel_nofault_loop<T>(
    mut dst: *mut u8,
    mut src: *const u8,
    mut len: usize,
) -> Result<(*mut u8, *const u8, usize), ()> {
    while len >= core::mem::size_of::<T>() {
        core::ptr::copy_nonoverlapping(src, dst, core::mem::size_of::<T>());
        instrument_write(dst as *const c_void, core::mem::size_of::<T>());
        dst = dst.add(core::mem::size_of::<T>());
        src = src.add(core::mem::size_of::<T>());
        len -= core::mem::size_of::<T>();
    }
    Ok((dst, src, len))
}

#[no_mangle]
pub unsafe extern "C" fn copy_to_kernel_nofault(dst: *mut c_void, src: *const c_void, size: usize) -> isize {
    let align = (dst as usize) | (src as usize);
    pagefault_disable();
    let (mut d, mut s, mut l) = (dst as *mut u8, src as *const u8, size);
    if align & 7 == 0 { (d, s, l) = copy_to_kernel_nofault_loop::<u64>(d, s, l).unwrap(); }
    if align & 3 == 0 { (d, s, l) = copy_to_kernel_nofault_loop::<u32>(d, s, l).unwrap(); }
    if align & 1 == 0 { (d, s, l) = copy_to_kernel_nofault_loop::<u16>(d, s, l).unwrap(); }
    let _ = copy_to_kernel_nofault_loop::<u8>(d, s, l);
    pagefault_enable();
    0
}

#[no_mangle]
pub unsafe extern "C" fn strncpy_from_kernel_nofault(dst: *mut u8, unsafe_addr: *const c_void, count: isize) -> isize {
    if count <= 0 { return 0; }
    if !copy_from_kernel_nofault_allowed(unsafe_addr, count as usize) { return -34; }
    pagefault_disable();
    let mut i = 0isize;
    while i < count {
        let ch = *(unsafe_addr as *const u8).offset(i);
        *dst.offset(i) = ch;
        i += 1;
        if ch == 0 { break; }
    }
    pagefault_enable();
    *dst.offset(i - 1) = 0;
    i
}

#[no_mangle]
pub unsafe extern "C" fn copy_from_user_nofault(dst: *mut c_void, src: *const c_void, size: usize) -> isize {
    if !__access_ok(src, size) || !nmi_uaccess_okay() { return -14; }
    pagefault_disable();
    let ret = __copy_from_user_inatomic(dst, src, size);
    pagefault_enable();
    if ret != 0 { -14 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn copy_to_user_nofault(dst: *mut c_void, src: *const c_void, size: usize) -> isize {
    let mut ret = -14isize;
    if access_ok(dst, size) {
        pagefault_disable();
        ret = __copy_to_user_inatomic(dst, src, size) as isize;
        pagefault_enable();
    }
    if ret != 0 { -14 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn strncpy_from_user_nofault(dst: *mut u8, unsafe_addr: *const c_void, count: isize) -> isize {
    if count <= 0 { return 0; }
    pagefault_disable();
    let mut ret = strncpy_from_user(dst, unsafe_addr, count);
    pagefault_enable();
    if ret >= count { ret = count; *dst.offset(ret - 1) = 0; }
    else if ret >= 0 { ret += 1; }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn strnlen_user_nofault(unsafe_addr: *const c_void, count: isize) -> isize {
    pagefault_disable();
    let ret = strnlen_user(unsafe_addr, count);
    pagefault_enable();
    ret as isize
}

#[no_mangle]
pub unsafe extern "C" fn __copy_overflow(size: i32, count: usize) {
    warn(1, b"Buffer overflow detected (%d < %lu)!\n\0".as_ptr(), size, count);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
