/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from arch/x86/include/asm/io.h.
 * C include dependencies: <linux/compiler.h>, <linux/types.h>,
 * "special_insns.h", and <asm-generic/io.h>.
 */

use core::ffi::c_void;

pub type size_t = usize;
pub type u8 = core::ffi::c_uchar;
pub type u64 = core::ffi::c_ulonglong;

#[inline]
pub unsafe fn readb(addr: *const c_void) -> core::ffi::c_uchar {
    unsafe { core::ptr::read_volatile(addr as *const core::ffi::c_uchar) }
}

#[inline]
pub unsafe fn readw(addr: *const c_void) -> core::ffi::c_ushort {
    unsafe { core::ptr::read_volatile(addr as *const core::ffi::c_ushort) }
}

#[inline]
pub unsafe fn readl(addr: *const c_void) -> core::ffi::c_uint {
    unsafe { core::ptr::read_volatile(addr as *const core::ffi::c_uint) }
}

#[inline]
pub unsafe fn __readb(addr: *const c_void) -> core::ffi::c_uchar {
    unsafe { core::ptr::read_volatile(addr as *const core::ffi::c_uchar) }
}

#[inline]
pub unsafe fn __readw(addr: *const c_void) -> core::ffi::c_ushort {
    unsafe { core::ptr::read_volatile(addr as *const core::ffi::c_ushort) }
}

#[inline]
pub unsafe fn __readl(addr: *const c_void) -> core::ffi::c_uint {
    unsafe { core::ptr::read_volatile(addr as *const core::ffi::c_uint) }
}

#[inline]
pub unsafe fn writeb(val: core::ffi::c_uchar, addr: *mut c_void) {
    unsafe { core::ptr::write_volatile(addr as *mut core::ffi::c_uchar, val) }
}

#[inline]
pub unsafe fn writew(val: core::ffi::c_ushort, addr: *mut c_void) {
    unsafe { core::ptr::write_volatile(addr as *mut core::ffi::c_ushort, val) }
}

#[inline]
pub unsafe fn writel(val: core::ffi::c_uint, addr: *mut c_void) {
    unsafe { core::ptr::write_volatile(addr as *mut core::ffi::c_uint, val) }
}

#[inline]
pub unsafe fn __writeb(val: core::ffi::c_uchar, addr: *mut c_void) {
    unsafe { core::ptr::write_volatile(addr as *mut core::ffi::c_uchar, val) }
}

#[inline]
pub unsafe fn __writew(val: core::ffi::c_ushort, addr: *mut c_void) {
    unsafe { core::ptr::write_volatile(addr as *mut core::ffi::c_ushort, val) }
}

#[inline]
pub unsafe fn __writel(val: core::ffi::c_uint, addr: *mut c_void) {
    unsafe { core::ptr::write_volatile(addr as *mut core::ffi::c_uint, val) }
}

#[inline]
pub unsafe fn readb_relaxed(a: *const c_void) -> core::ffi::c_uchar {
    unsafe { __readb(a) }
}

#[inline]
pub unsafe fn readw_relaxed(a: *const c_void) -> core::ffi::c_ushort {
    unsafe { __readw(a) }
}

#[inline]
pub unsafe fn readl_relaxed(a: *const c_void) -> core::ffi::c_uint {
    unsafe { __readl(a) }
}

pub use __readb as __raw_readb;
pub use __readw as __raw_readw;
pub use __readl as __raw_readl;

#[inline]
pub unsafe fn writeb_relaxed(v: core::ffi::c_uchar, a: *mut c_void) {
    unsafe { __writeb(v, a) }
}

#[inline]
pub unsafe fn writew_relaxed(v: core::ffi::c_ushort, a: *mut c_void) {
    unsafe { __writew(v, a) }
}

#[inline]
pub unsafe fn writel_relaxed(v: core::ffi::c_uint, a: *mut c_void) {
    unsafe { __writel(v, a) }
}

pub use __writeb as __raw_writeb;
pub use __writew as __raw_writew;
pub use __writel as __raw_writel;

/* Original condition: #ifdef __x86_64__ */
#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn readq(addr: *const c_void) -> u64 {
    unsafe { core::ptr::read_volatile(addr as *const u64) }
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn __readq(addr: *const c_void) -> u64 {
    unsafe { core::ptr::read_volatile(addr as *const u64) }
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn writeq(val: u64, addr: *mut c_void) {
    unsafe { core::ptr::write_volatile(addr as *mut u64, val) }
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn __writeq(val: u64, addr: *mut c_void) {
    unsafe { core::ptr::write_volatile(addr as *mut u64, val) }
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn readq_relaxed(a: *const c_void) -> u64 {
    unsafe { __readq(a) }
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn writeq_relaxed(v: u64, a: *mut c_void) {
    unsafe { __writeq(v, a) }
}

#[cfg(target_arch = "x86_64")]
pub use __readq as __raw_readq;
#[cfg(target_arch = "x86_64")]
pub use __writeq as __raw_writeq;

/**
 * iosubmit_cmds512 - copy data to single MMIO location, in 512-bit units
 * @dst: destination, in MMIO space (must be 512-bit aligned)
 * @src: source
 * @count: number of 512 bits quantities to submit
 *
 * Submit data from kernel space to MMIO space, in units of 512 bits at a
 * time.  Order of access is not guaranteed, nor is a memory barrier
 * performed afterwards.
 *
 * Warning: Do not use this helper unless your driver has checked that the CPU
 * instruction is supported on the platform.
 */
#[inline]
pub unsafe fn iosubmit_cmds512(dst: *mut c_void, src: *const c_void, count: size_t) {
    let mut from = src as *const u8;
    let end = unsafe { from.add(count.wrapping_mul(64)) };

    while from < end {
        unsafe {
            movdir64b(dst, from as *const c_void);
            from = from.add(64);
        }
    }
}

unsafe extern "C" {
    pub fn movdir64b(dst: *mut c_void, src: *const c_void);
}
