// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *
 *  Misc memory accessors
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct iov_iter {
    _private: [u8; 0],
}

unsafe extern "C" {
    static ITER_DEST: c_int;
    static ITER_SOURCE: c_int;
    static EFAULT: c_int;

    fn import_ubuf(
        rw: c_int,
        buf: *mut c_void,
        len: usize,
        i: *mut iov_iter,
    ) -> c_int;
    fn copy_to_iter(addr: *const c_void, bytes: usize, i: *mut iov_iter) -> usize;
    fn copy_from_iter(addr: *mut c_void, bytes: usize, i: *mut iov_iter) -> usize;
    fn memcpy_fromio(to: *mut c_void, from: *mut c_void, count: usize);
    fn memcpy_toio(to: *mut c_void, from: *const c_void, count: usize);
}

/**
 * copy_to_user_fromio - copy data from mmio-space to user-space
 * @dst: the destination pointer on user-space
 * @src: the source pointer on mmio
 * @count: the data size to copy in bytes
 *
 * Copies the data from mmio-space to user-space.
 *
 * Return: Zero if successful, or non-zero on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn copy_to_user_fromio(
    dst: *mut c_void,
    src: *const c_void,
    count: usize,
) -> c_int {
    let mut iter = core::mem::MaybeUninit::<iov_iter>::uninit();

    if unsafe { import_ubuf(ITER_DEST, dst, count, iter.as_mut_ptr()) } != 0 {
        return unsafe { -EFAULT };
    }
    if unsafe { copy_to_iter_fromio(src, count, iter.as_mut_ptr()) } != count {
        return unsafe { -EFAULT };
    }
    0
}
// EXPORT_SYMBOL(copy_to_user_fromio);

/**
 * copy_to_iter_fromio - copy data from mmio-space to iov_iter
 * @src: the source pointer on mmio
 * @count: the data size to copy in bytes
 * @dst: the destination iov_iter
 *
 * Copies the data from mmio-space to iov_iter.
 *
 * Return: number of bytes to be copied
 */
#[no_mangle]
pub unsafe extern "C" fn copy_to_iter_fromio(
    mut src: *const c_void,
    mut count: usize,
    dst: *mut iov_iter,
) -> usize {
    // C conditional: defined(__i386__) || defined(CONFIG_SPARC32).
    #[cfg(any(target_arch = "x86", CONFIG_SPARC32))]
    {
        return unsafe { copy_to_iter(src, count, dst) };
    }

    #[cfg(not(any(target_arch = "x86", CONFIG_SPARC32)))]
    {
        let mut buf = [0 as c_char; 256];
        let mut res: usize = 0;

        while count != 0 {
            let mut c = count;
            if c > core::mem::size_of_val(&buf) {
                c = core::mem::size_of_val(&buf);
            }
            unsafe { memcpy_fromio(buf.as_mut_ptr() as *mut c_void, src as *mut c_void, c) };
            if unsafe { copy_to_iter(buf.as_ptr() as *const c_void, c, dst) } != c {
                return res;
            }
            count -= c;
            src = unsafe { (src as *const u8).add(c) as *const c_void };
            res += c;
        }
        res
    }
}
// EXPORT_SYMBOL(copy_to_iter_fromio);

/**
 * copy_from_user_toio - copy data from user-space to mmio-space
 * @dst: the destination pointer on mmio-space
 * @src: the source pointer on user-space
 * @count: the data size to copy in bytes
 *
 * Copies the data from user-space to mmio-space.
 *
 * Return: Zero if successful, or non-zero on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn copy_from_user_toio(
    dst: *mut c_void,
    src: *const c_void,
    count: usize,
) -> c_int {
    let mut iter = core::mem::MaybeUninit::<iov_iter>::uninit();

    if unsafe { import_ubuf(ITER_SOURCE, src as *mut c_void, count, iter.as_mut_ptr()) } != 0 {
        return unsafe { -EFAULT };
    }
    if unsafe { copy_from_iter_toio(dst, count, iter.as_mut_ptr()) } != count {
        return unsafe { -EFAULT };
    }
    0
}
// EXPORT_SYMBOL(copy_from_user_toio);

/**
 * copy_from_iter_toio - copy data from iov_iter to mmio-space
 * @dst: the destination pointer on mmio-space
 * @count: the data size to copy in bytes
 * @src: the source iov_iter
 *
 * Copies the data from iov_iter to mmio-space.
 *
 * Return: number of bytes to be copied
 */
#[no_mangle]
pub unsafe extern "C" fn copy_from_iter_toio(
    mut dst: *mut c_void,
    mut count: usize,
    src: *mut iov_iter,
) -> usize {
    // C conditional: defined(__i386__) || defined(CONFIG_SPARC32).
    #[cfg(any(target_arch = "x86", CONFIG_SPARC32))]
    {
        return unsafe { copy_from_iter(dst, count, src) };
    }

    #[cfg(not(any(target_arch = "x86", CONFIG_SPARC32)))]
    {
        let mut buf = [0 as c_char; 256];
        let mut res: usize = 0;

        while count != 0 {
            let mut c = count;
            if c > core::mem::size_of_val(&buf) {
                c = core::mem::size_of_val(&buf);
            }
            if unsafe { copy_from_iter(buf.as_mut_ptr() as *mut c_void, c, src) } != c {
                return res;
            }
            unsafe { memcpy_toio(dst, buf.as_ptr() as *const c_void, c) };
            count -= c;
            dst = unsafe { (dst as *mut u8).add(c) as *mut c_void };
            res += c;
        }
        res
    }
}
// EXPORT_SYMBOL(copy_from_iter_toio);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
