/* SPDX-License-Identifier: GPL-2.0 */

pub unsafe fn __get_unaligned_le16(p: *const u8) -> u16 {
    unsafe { (*p.add(0) as u16) | ((*p.add(1) as u16) << 8) }
}

pub unsafe fn __get_unaligned_le32(p: *const u8) -> u32 {
    unsafe {
        (*p.add(0) as u32)
            | ((*p.add(1) as u32) << 8)
            | ((*p.add(2) as u32) << 16)
            | ((*p.add(3) as u32) << 24)
    }
}

pub unsafe fn __get_unaligned_le64(p: *const u8) -> u64 {
    unsafe { ((__get_unaligned_le32(p.add(4)) as u64) << 32) | (__get_unaligned_le32(p) as u64) }
}

pub unsafe fn __put_unaligned_le16(val: u16, mut p: *mut u8) {
    unsafe {
        *p = val as u8;
        p = p.add(1);
        *p = (val >> 8) as u8;
    }
}

pub unsafe fn __put_unaligned_le32(val: u32, p: *mut u8) {
    unsafe {
        __put_unaligned_le16((val >> 16) as u16, p.add(2));
        __put_unaligned_le16(val as u16, p);
    }
}

pub unsafe fn __put_unaligned_le64(val: u64, p: *mut u8) {
    unsafe {
        __put_unaligned_le32((val >> 32) as u32, p.add(4));
        __put_unaligned_le32(val as u32, p);
    }
}

pub unsafe fn get_unaligned_le16(p: *const core::ffi::c_void) -> u16 {
    unsafe { __get_unaligned_le16(p as *const u8) }
}

pub unsafe fn get_unaligned_le32(p: *const core::ffi::c_void) -> u32 {
    unsafe { __get_unaligned_le32(p as *const u8) }
}

pub unsafe fn get_unaligned_le64(p: *const core::ffi::c_void) -> u64 {
    unsafe { __get_unaligned_le64(p as *const u8) }
}

pub unsafe fn put_unaligned_le16(val: u16, p: *mut core::ffi::c_void) {
    unsafe {
        __put_unaligned_le16(val, p as *mut u8);
    }
}

pub unsafe fn put_unaligned_le32(val: u32, p: *mut core::ffi::c_void) {
    unsafe {
        __put_unaligned_le32(val, p as *mut u8);
    }
}

pub unsafe fn put_unaligned_le64(val: u64, p: *mut core::ffi::c_void) {
    unsafe {
        __put_unaligned_le64(val, p as *mut u8);
    }
}
