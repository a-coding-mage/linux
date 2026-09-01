/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

pub unsafe fn __get_unaligned_be16(p: *const u8) -> u16 {
    unsafe { (((*p.add(0)) as u16) << 8) | ((*p.add(1)) as u16) }
}

pub unsafe fn __get_unaligned_be32(p: *const u8) -> u32 {
    unsafe {
        (((*p.add(0)) as u32) << 24)
            | (((*p.add(1)) as u32) << 16)
            | (((*p.add(2)) as u32) << 8)
            | ((*p.add(3)) as u32)
    }
}

pub unsafe fn __get_unaligned_be64(p: *const u8) -> u64 {
    unsafe { ((__get_unaligned_be32(p) as u64) << 32) | (__get_unaligned_be32(p.add(4)) as u64) }
}

pub unsafe fn __put_unaligned_be16(val: u16, p: *mut u8) {
    unsafe {
        *p.add(0) = (val >> 8) as u8;
        *p.add(1) = val as u8;
    }
}

pub unsafe fn __put_unaligned_be32(val: u32, p: *mut u8) {
    unsafe {
        __put_unaligned_be16((val >> 16) as u16, p);
        __put_unaligned_be16(val as u16, p.add(2));
    }
}

pub unsafe fn __put_unaligned_be64(val: u64, p: *mut u8) {
    unsafe {
        __put_unaligned_be32((val >> 32) as u32, p);
        __put_unaligned_be32(val as u32, p.add(4));
    }
}

pub unsafe fn get_unaligned_be16(p: *const c_void) -> u16 {
    unsafe { __get_unaligned_be16(p as *const u8) }
}

pub unsafe fn get_unaligned_be32(p: *const c_void) -> u32 {
    unsafe { __get_unaligned_be32(p as *const u8) }
}

pub unsafe fn get_unaligned_be64(p: *const c_void) -> u64 {
    unsafe { __get_unaligned_be64(p as *const u8) }
}

pub unsafe fn put_unaligned_be16(val: u16, p: *mut c_void) {
    unsafe {
        __put_unaligned_be16(val, p as *mut u8);
    }
}

pub unsafe fn put_unaligned_be32(val: u32, p: *mut c_void) {
    unsafe {
        __put_unaligned_be32(val, p as *mut u8);
    }
}

pub unsafe fn put_unaligned_be64(val: u64, p: *mut c_void) {
    unsafe {
        __put_unaligned_be64(val, p as *mut u8);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
