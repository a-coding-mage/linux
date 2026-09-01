/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This is the most generic implementation of unaligned accesses
 * and should work almost anywhere.
 */
/* C dependency intent: #include <vdso/unaligned.h> */

pub type __le16 = u16;
pub type __le32 = u32;
pub type __le64 = u64;
pub type __be16 = u16;
pub type __be32 = u32;
pub type __be64 = u64;

unsafe extern "C" {
    pub fn le16_to_cpu(x: __le16) -> u16;
    pub fn le32_to_cpu(x: __le32) -> u32;
    pub fn le64_to_cpu(x: __le64) -> u64;
    pub fn cpu_to_le16(x: u16) -> __le16;
    pub fn cpu_to_le32(x: u32) -> __le32;
    pub fn cpu_to_le64(x: u64) -> __le64;
    pub fn be16_to_cpu(x: __be16) -> u16;
    pub fn be32_to_cpu(x: __be32) -> u32;
    pub fn be64_to_cpu(x: __be64) -> u64;
    pub fn cpu_to_be16(x: u16) -> __be16;
    pub fn cpu_to_be32(x: u32) -> __be32;
    pub fn cpu_to_be64(x: u64) -> __be64;
}

#[inline]
pub unsafe fn __get_unaligned_t<T: Copy>(ptr: *const T) -> T {
    unsafe { core::ptr::read_unaligned(ptr) }
}

#[inline]
pub unsafe fn __put_unaligned_t<T>(val: T, ptr: *mut T) {
    unsafe {
        core::ptr::write_unaligned(ptr, val);
    }
}

/* C macros:
 * #define get_unaligned(ptr) __get_unaligned_t(typeof(*(ptr)), (ptr))
 * #define put_unaligned(val, ptr) __put_unaligned_t(typeof(*(ptr)), (val), (ptr))
 */
#[inline]
pub unsafe fn get_unaligned<T: Copy>(ptr: *const T) -> T {
    unsafe { __get_unaligned_t(ptr) }
}

#[inline]
pub unsafe fn put_unaligned<T>(val: T, ptr: *mut T) {
    unsafe {
        __put_unaligned_t(val, ptr);
    }
}

#[inline]
pub unsafe fn get_unaligned_le16(p: *const core::ffi::c_void) -> u16 {
    unsafe { le16_to_cpu(__get_unaligned_t::<__le16>(p as *const __le16)) }
}

#[inline]
pub unsafe fn get_unaligned_le32(p: *const core::ffi::c_void) -> u32 {
    unsafe { le32_to_cpu(__get_unaligned_t::<__le32>(p as *const __le32)) }
}

#[inline]
pub unsafe fn get_unaligned_le64(p: *const core::ffi::c_void) -> u64 {
    unsafe { le64_to_cpu(__get_unaligned_t::<__le64>(p as *const __le64)) }
}

#[inline]
pub unsafe fn put_unaligned_le16(val: u16, p: *mut core::ffi::c_void) {
    unsafe {
        __put_unaligned_t::<__le16>(cpu_to_le16(val), p as *mut __le16);
    }
}

#[inline]
pub unsafe fn put_unaligned_le32(val: u32, p: *mut core::ffi::c_void) {
    unsafe {
        __put_unaligned_t::<__le32>(cpu_to_le32(val), p as *mut __le32);
    }
}

#[inline]
pub unsafe fn put_unaligned_le64(val: u64, p: *mut core::ffi::c_void) {
    unsafe {
        __put_unaligned_t::<__le64>(cpu_to_le64(val), p as *mut __le64);
    }
}

#[inline]
pub unsafe fn get_unaligned_be16(p: *const core::ffi::c_void) -> u16 {
    unsafe { be16_to_cpu(__get_unaligned_t::<__be16>(p as *const __be16)) }
}

#[inline]
pub unsafe fn get_unaligned_be32(p: *const core::ffi::c_void) -> u32 {
    unsafe { be32_to_cpu(__get_unaligned_t::<__be32>(p as *const __be32)) }
}

#[inline]
pub unsafe fn get_unaligned_be64(p: *const core::ffi::c_void) -> u64 {
    unsafe { be64_to_cpu(__get_unaligned_t::<__be64>(p as *const __be64)) }
}

#[inline]
pub unsafe fn put_unaligned_be16(val: u16, p: *mut core::ffi::c_void) {
    unsafe {
        __put_unaligned_t::<__be16>(cpu_to_be16(val), p as *mut __be16);
    }
}

#[inline]
pub unsafe fn put_unaligned_be32(val: u32, p: *mut core::ffi::c_void) {
    unsafe {
        __put_unaligned_t::<__be32>(cpu_to_be32(val), p as *mut __be32);
    }
}

#[inline]
pub unsafe fn put_unaligned_be64(val: u64, p: *mut core::ffi::c_void) {
    unsafe {
        __put_unaligned_t::<__be64>(cpu_to_be64(val), p as *mut __be64);
    }
}

#[inline]
pub unsafe fn __get_unaligned_be24(p: *const u8) -> u32 {
    unsafe { ((*p.add(0) as u32) << 16) | ((*p.add(1) as u32) << 8) | (*p.add(2) as u32) }
}

#[inline]
pub unsafe fn get_unaligned_be24(p: *const core::ffi::c_void) -> u32 {
    unsafe { __get_unaligned_be24(p as *const u8) }
}

#[inline]
pub unsafe fn __get_unaligned_le24(p: *const u8) -> u32 {
    unsafe { (*p.add(0) as u32) | ((*p.add(1) as u32) << 8) | ((*p.add(2) as u32) << 16) }
}

#[inline]
pub unsafe fn get_unaligned_le24(p: *const core::ffi::c_void) -> u32 {
    unsafe { __get_unaligned_le24(p as *const u8) }
}

#[inline]
pub unsafe fn __put_unaligned_be24(val: u32, p: *mut u8) {
    unsafe {
        *p.add(0) = ((val >> 16) & 0xff) as u8;
        *p.add(1) = ((val >> 8) & 0xff) as u8;
        *p.add(2) = (val & 0xff) as u8;
    }
}

#[inline]
pub unsafe fn put_unaligned_be24(val: u32, p: *mut core::ffi::c_void) {
    unsafe {
        __put_unaligned_be24(val, p as *mut u8);
    }
}

#[inline]
pub unsafe fn __put_unaligned_le24(val: u32, p: *mut u8) {
    unsafe {
        *p.add(0) = (val & 0xff) as u8;
        *p.add(1) = ((val >> 8) & 0xff) as u8;
        *p.add(2) = ((val >> 16) & 0xff) as u8;
    }
}

#[inline]
pub unsafe fn put_unaligned_le24(val: u32, p: *mut core::ffi::c_void) {
    unsafe {
        __put_unaligned_le24(val, p as *mut u8);
    }
}

#[inline]
pub unsafe fn __put_unaligned_be48(val: u64, p: *mut u8) {
    unsafe {
        *p.add(0) = ((val >> 40) & 0xff) as u8;
        *p.add(1) = ((val >> 32) & 0xff) as u8;
        *p.add(2) = ((val >> 24) & 0xff) as u8;
        *p.add(3) = ((val >> 16) & 0xff) as u8;
        *p.add(4) = ((val >> 8) & 0xff) as u8;
        *p.add(5) = (val & 0xff) as u8;
    }
}

#[inline]
pub unsafe fn put_unaligned_be48(val: u64, p: *mut core::ffi::c_void) {
    unsafe {
        __put_unaligned_be48(val, p as *mut u8);
    }
}

#[inline]
pub unsafe fn __get_unaligned_be48(p: *const u8) -> u64 {
    unsafe {
        ((*p.add(0) as u64) << 40)
            | ((*p.add(1) as u64) << 32)
            | ((*p.add(2) as u64) << 24)
            | ((*p.add(3) as u64) << 16)
            | ((*p.add(4) as u64) << 8)
            | (*p.add(5) as u64)
    }
}

#[inline]
pub unsafe fn get_unaligned_be48(p: *const core::ffi::c_void) -> u64 {
    unsafe { __get_unaligned_be48(p as *const u8) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
