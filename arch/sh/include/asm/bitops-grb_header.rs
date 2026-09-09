/* SPDX-License-Identifier: GPL-2.0 */

// The C header included <asm-generic/bitops/non-atomic.h>; that dependency is
// supplied by the surrounding build.

#[inline]
pub unsafe fn set_bit(nr: i32, addr: *mut core::ffi::c_void) {
    let a = (addr as *mut u32).offset((nr >> 5) as isize);
    let mask: u32 = (1i32 << (nr & 0x1f)) as u32;
    let value = core::ptr::read_volatile(a);
    core::ptr::write_volatile(a, value | mask);
}

#[inline]
pub unsafe fn clear_bit(nr: i32, addr: *mut core::ffi::c_void) {
    let a = (addr as *mut u32).offset((nr >> 5) as isize);
    let mask: u32 = (!(1i32 << (nr & 0x1f))) as u32;
    let value = core::ptr::read_volatile(a);
    core::ptr::write_volatile(a, value & mask);
}

#[inline]
pub unsafe fn change_bit(nr: i32, addr: *mut core::ffi::c_void) {
    let a = (addr as *mut u32).offset((nr >> 5) as isize);
    let mask: u32 = (1i32 << (nr & 0x1f)) as u32;
    let value = core::ptr::read_volatile(a);
    core::ptr::write_volatile(a, value ^ mask);
}

#[inline]
pub unsafe fn test_and_set_bit(nr: i32, addr: *mut core::ffi::c_void) -> i32 {
    let a = (addr as *mut u32).offset((nr >> 5) as isize);
    let mask: u32 = (1i32 << (nr & 0x1f)) as u32;
    let value = core::ptr::read_volatile(a);
    let retval = if value & mask != 0 { 1 } else { 0 };
    core::ptr::write_volatile(a, value | mask);
    retval
}

#[inline]
pub unsafe fn test_and_clear_bit(nr: i32, addr: *mut core::ffi::c_void) -> i32 {
    let a = (addr as *mut u32).offset((nr >> 5) as isize);
    let mask: u32 = (1i32 << (nr & 0x1f)) as u32;
    let value = core::ptr::read_volatile(a);
    let retval = if value & mask != 0 { 1 } else { 0 };
    core::ptr::write_volatile(a, value & !mask);
    retval
}

#[inline]
pub unsafe fn test_and_change_bit(nr: i32, addr: *mut core::ffi::c_void) -> i32 {
    let a = (addr as *mut u32).offset((nr >> 5) as isize);
    let mask: u32 = (1i32 << (nr & 0x1f)) as u32;
    let value = core::ptr::read_volatile(a);
    let retval = if value & mask != 0 { 1 } else { 0 };
    core::ptr::write_volatile(a, value ^ mask);
    retval
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
