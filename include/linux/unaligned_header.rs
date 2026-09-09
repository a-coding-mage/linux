/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the original header:
// linux/unaligned/packed_struct.h, asm/byteorder.h, vdso/unaligned.h

macro_rules! get_unaligned {
    ($ptr:expr) => {
        __get_unaligned_t($ptr)
    };
}

macro_rules! put_unaligned {
    ($val:expr, $ptr:expr) => {
        __put_unaligned_t($val, $ptr)
    };
}

#[inline]
pub unsafe fn get_unaligned_le16(p: *const core::ffi::c_void) -> u16 {
    le16_to_cpu(__get_unaligned_t(p as *const __le16))
}

#[inline]
pub unsafe fn get_unaligned_le32(p: *const core::ffi::c_void) -> u32 {
    le32_to_cpu(__get_unaligned_t(p as *const __le32))
}

#[inline]
pub unsafe fn get_unaligned_le64(p: *const core::ffi::c_void) -> u64 {
    le64_to_cpu(__get_unaligned_t(p as *const __le64))
}

#[inline]
pub unsafe fn put_unaligned_le16(val: u16, p: *mut core::ffi::c_void) {
    __put_unaligned_t(cpu_to_le16(val), p as *mut __le16);
}

#[inline]
pub unsafe fn put_unaligned_le32(val: u32, p: *mut core::ffi::c_void) {
    __put_unaligned_t(cpu_to_le32(val), p as *mut __le32);
}

#[inline]
pub unsafe fn put_unaligned_le64(val: u64, p: *mut core::ffi::c_void) {
    __put_unaligned_t(cpu_to_le64(val), p as *mut __le64);
}

#[inline]
pub unsafe fn get_unaligned_be16(p: *const core::ffi::c_void) -> u16 {
    be16_to_cpu(__get_unaligned_t(p as *const __be16))
}

#[inline]
pub unsafe fn get_unaligned_be32(p: *const core::ffi::c_void) -> u32 {
    be32_to_cpu(__get_unaligned_t(p as *const __be32))
}

#[inline]
pub unsafe fn get_unaligned_be64(p: *const core::ffi::c_void) -> u64 {
    be64_to_cpu(__get_unaligned_t(p as *const __be64))
}

#[inline]
pub unsafe fn put_unaligned_be16(val: u16, p: *mut core::ffi::c_void) {
    __put_unaligned_t(cpu_to_be16(val), p as *mut __be16);
}

#[inline]
pub unsafe fn put_unaligned_be32(val: u32, p: *mut core::ffi::c_void) {
    __put_unaligned_t(cpu_to_be32(val), p as *mut __be32);
}

#[inline]
pub unsafe fn put_unaligned_be64(val: u64, p: *mut core::ffi::c_void) {
    __put_unaligned_t(cpu_to_be64(val), p as *mut __be64);
}

#[inline]
pub unsafe fn __get_unaligned_be24(p: *const u8) -> u32 {
    ((*p as u32) << 16) | ((*p.add(1) as u32) << 8) | (*p.add(2) as u32)
}

#[inline]
pub unsafe fn get_unaligned_be24(p: *const core::ffi::c_void) -> u32 {
    __get_unaligned_be24(p as *const u8)
}

#[inline]
pub unsafe fn __get_unaligned_le24(p: *const u8) -> u32 {
    (*p as u32) | ((*p.add(1) as u32) << 8) | ((*p.add(2) as u32) << 16)
}

#[inline]
pub unsafe fn get_unaligned_le24(p: *const core::ffi::c_void) -> u32 {
    __get_unaligned_le24(p as *const u8)
}

#[inline]
pub unsafe fn __put_unaligned_be24(val: u32, mut p: *mut u8) {
    *p = ((val >> 16) & 0xff) as u8;
    p = p.add(1);
    *p = ((val >> 8) & 0xff) as u8;
    p = p.add(1);
    *p = (val & 0xff) as u8;
}

#[inline]
pub unsafe fn put_unaligned_be24(val: u32, p: *mut core::ffi::c_void) {
    __put_unaligned_be24(val, p as *mut u8);
}

#[inline]
pub unsafe fn __put_unaligned_le24(val: u32, mut p: *mut u8) {
    *p = (val & 0xff) as u8;
    p = p.add(1);
    *p = ((val >> 8) & 0xff) as u8;
    p = p.add(1);
    *p = ((val >> 16) & 0xff) as u8;
}

#[inline]
pub unsafe fn put_unaligned_le24(val: u32, p: *mut core::ffi::c_void) {
    __put_unaligned_le24(val, p as *mut u8);
}

#[inline]
pub unsafe fn __put_unaligned_be48(val: u64, mut p: *mut u8) {
    *p = (val >> 40) as u8;
    p = p.add(1);
    *p = (val >> 32) as u8;
    p = p.add(1);
    *p = (val >> 24) as u8;
    p = p.add(1);
    *p = (val >> 16) as u8;
    p = p.add(1);
    *p = (val >> 8) as u8;
    p = p.add(1);
    *p = val as u8;
}

#[inline]
pub unsafe fn put_unaligned_be48(val: u64, p: *mut core::ffi::c_void) {
    __put_unaligned_be48(val, p as *mut u8);
}

#[inline]
pub unsafe fn __get_unaligned_be48(p: *const u8) -> u64 {
    ((*p as u64) << 40)
        | ((*p.add(1) as u64) << 32)
        | ((*p.add(2) as u64) << 24)
        | ((*p.add(3) as u64) << 16)
        | ((*p.add(4) as u64) << 8)
        | (*p.add(5) as u64)
}

#[inline]
pub unsafe fn get_unaligned_be48(p: *const core::ffi::c_void) -> u64 {
    __get_unaligned_be48(p as *const u8)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
