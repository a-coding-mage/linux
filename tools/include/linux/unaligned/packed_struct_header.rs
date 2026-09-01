/* SPDX-License-Identifier: GPL-2.0 */

// C dependency intent: #include <linux/kernel.h> supplies u16/u32/u64 and __packed.

#[repr(C, packed)]
pub struct __una_u16 {
    pub x: u16,
}

#[repr(C, packed)]
pub struct __una_u32 {
    pub x: u32,
}

#[repr(C, packed)]
pub struct __una_u64 {
    pub x: u64,
}

#[inline]
pub unsafe fn __get_unaligned_cpu16(p: *const core::ffi::c_void) -> u16 {
    let ptr: *const __una_u16 = p as *const __una_u16;
    unsafe { core::ptr::addr_of!((*ptr).x).read_unaligned() }
}

#[inline]
pub unsafe fn __get_unaligned_cpu32(p: *const core::ffi::c_void) -> u32 {
    let ptr: *const __una_u32 = p as *const __una_u32;
    unsafe { core::ptr::addr_of!((*ptr).x).read_unaligned() }
}

#[inline]
pub unsafe fn __get_unaligned_cpu64(p: *const core::ffi::c_void) -> u64 {
    let ptr: *const __una_u64 = p as *const __una_u64;
    unsafe { core::ptr::addr_of!((*ptr).x).read_unaligned() }
}

#[inline]
pub unsafe fn __put_unaligned_cpu16(val: u16, p: *mut core::ffi::c_void) {
    let ptr: *mut __una_u16 = p as *mut __una_u16;
    unsafe { core::ptr::addr_of_mut!((*ptr).x).write_unaligned(val) };
}

#[inline]
pub unsafe fn __put_unaligned_cpu32(val: u32, p: *mut core::ffi::c_void) {
    let ptr: *mut __una_u32 = p as *mut __una_u32;
    unsafe { core::ptr::addr_of_mut!((*ptr).x).write_unaligned(val) };
}

#[inline]
pub unsafe fn __put_unaligned_cpu64(val: u64, p: *mut core::ffi::c_void) {
    let ptr: *mut __una_u64 = p as *mut __una_u64;
    unsafe { core::ptr::addr_of_mut!((*ptr).x).write_unaligned(val) };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
