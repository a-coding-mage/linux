/* SPDX-License-Identifier: GPL-2.0 */

// Translated from include/asm-generic/io.h.
// Original include dependencies: asm/barrier.h, asm/byteorder.h,
// linux/compiler.h, linux/kernel.h, linux/types.h.

pub type u8 = ::core::ffi::c_uchar;
pub type u16 = ::core::ffi::c_ushort;
pub type u32 = ::core::ffi::c_uint;
pub type u64 = ::core::ffi::c_ulonglong;
pub type c_void = ::core::ffi::c_void;
pub type c_ulong = ::core::ffi::c_ulong;

unsafe extern "C" {
    fn barrier();
}

// Fallback for: #ifndef mmiowb_set_pending
#[inline]
pub unsafe fn mmiowb_set_pending() {}

// Fallback for: #ifndef __io_br
#[inline]
pub unsafe fn __io_br() {
    unsafe { barrier() };
}

/*
 * prevent prefetching of coherent DMA data ahead of a dma-complete
 *
 * Fallback translation of the C preprocessor choice:
 * if rmb is defined, __io_ar(v) maps to rmb(); otherwise barrier().
 */
#[inline]
pub unsafe fn __io_ar<T>(_v: T) {
    unsafe { barrier() };
}

/*
 * flush writes to coherent DMA data before possibly triggering a DMA read
 *
 * Fallback translation of the C preprocessor choice:
 * if wmb is defined, __io_bw() maps to wmb(); otherwise barrier().
 */
#[inline]
pub unsafe fn __io_bw() {
    unsafe { barrier() };
}

/* serialize device access against a spin_unlock, usually handled there. */
#[inline]
pub unsafe fn __io_aw() {
    unsafe { mmiowb_set_pending() };
}

#[inline]
pub unsafe fn __io_pbw() {
    unsafe { __io_bw() };
}

#[inline]
pub unsafe fn __io_paw() {
    unsafe { __io_aw() };
}

#[inline]
pub unsafe fn __io_pbr() {
    unsafe { __io_br() };
}

#[inline]
pub unsafe fn __io_par<T>(v: T) {
    unsafe { __io_ar(v) };
}

pub const _THIS_IP_: c_ulong = 0;
pub const _RET_IP_: c_ulong = 0;

#[inline]
pub unsafe fn log_write_mmio(
    _val: u64,
    _width: u8,
    _addr: *mut c_void,
    _caller_addr: c_ulong,
    _caller_addr0: c_ulong,
) {
}

#[inline]
pub unsafe fn log_post_write_mmio(
    _val: u64,
    _width: u8,
    _addr: *mut c_void,
    _caller_addr: c_ulong,
    _caller_addr0: c_ulong,
) {
}

#[inline]
pub unsafe fn log_read_mmio(
    _width: u8,
    _addr: *const c_void,
    _caller_addr: c_ulong,
    _caller_addr0: c_ulong,
) {
}

#[inline]
pub unsafe fn log_post_read_mmio(
    _val: u64,
    _width: u8,
    _addr: *const c_void,
    _caller_addr: c_ulong,
    _caller_addr0: c_ulong,
) {
}

/*
 * __raw_{read,write}{b,w,l,q}() access memory in native endianness.
 *
 * On some architectures memory mapped IO needs to be accessed differently.
 * On the simple architectures, we just read/write the memory location
 * directly.
 */

#[inline]
pub unsafe fn __raw_readb(addr: *const c_void) -> u8 {
    unsafe { ::core::ptr::read_volatile(addr as *const u8) }
}

#[inline]
pub unsafe fn __raw_readw(addr: *const c_void) -> u16 {
    unsafe { ::core::ptr::read_volatile(addr as *const u16) }
}

#[inline]
pub unsafe fn __raw_readl(addr: *const c_void) -> u32 {
    unsafe { ::core::ptr::read_volatile(addr as *const u32) }
}

#[inline]
pub unsafe fn __raw_readq(addr: *const c_void) -> u64 {
    unsafe { ::core::ptr::read_volatile(addr as *const u64) }
}

#[inline]
pub unsafe fn __raw_writeb(value: u8, addr: *mut c_void) {
    unsafe { ::core::ptr::write_volatile(addr as *mut u8, value) };
}

#[inline]
pub unsafe fn __raw_writew(value: u16, addr: *mut c_void) {
    unsafe { ::core::ptr::write_volatile(addr as *mut u16, value) };
}

#[inline]
pub unsafe fn __raw_writel(value: u32, addr: *mut c_void) {
    unsafe { ::core::ptr::write_volatile(addr as *mut u32, value) };
}

#[inline]
pub unsafe fn __raw_writeq(value: u64, addr: *mut c_void) {
    unsafe { ::core::ptr::write_volatile(addr as *mut u64, value) };
}

/*
 * {read,write}{b,w,l,q}() access little endian memory and return result in
 * native endianness.
 */

#[inline]
pub unsafe fn readb(addr: *const c_void) -> u8 {
    let val: u8;

    unsafe { log_read_mmio(8, addr, _THIS_IP_, _RET_IP_) };
    unsafe { __io_br() };
    val = unsafe { __raw_readb(addr) };
    unsafe { __io_ar(val) };
    unsafe { log_post_read_mmio(val as u64, 8, addr, _THIS_IP_, _RET_IP_) };
    val
}

#[inline]
pub unsafe fn readw(addr: *const c_void) -> u16 {
    let val: u16;

    unsafe { log_read_mmio(16, addr, _THIS_IP_, _RET_IP_) };
    unsafe { __io_br() };
    val = u16::from_le(unsafe { __raw_readw(addr) });
    unsafe { __io_ar(val) };
    unsafe { log_post_read_mmio(val as u64, 16, addr, _THIS_IP_, _RET_IP_) };
    val
}

#[inline]
pub unsafe fn readl(addr: *const c_void) -> u32 {
    let val: u32;

    unsafe { log_read_mmio(32, addr, _THIS_IP_, _RET_IP_) };
    unsafe { __io_br() };
    val = u32::from_le(unsafe { __raw_readl(addr) });
    unsafe { __io_ar(val) };
    unsafe { log_post_read_mmio(val as u64, 32, addr, _THIS_IP_, _RET_IP_) };
    val
}

#[inline]
pub unsafe fn readq(addr: *const c_void) -> u64 {
    let val: u64;

    unsafe { log_read_mmio(64, addr, _THIS_IP_, _RET_IP_) };
    unsafe { __io_br() };
    val = u64::from_le(unsafe { __raw_readq(addr) });
    unsafe { __io_ar(val) };
    unsafe { log_post_read_mmio(val, 64, addr, _THIS_IP_, _RET_IP_) };
    val
}

#[inline]
pub unsafe fn writeb(value: u8, addr: *mut c_void) {
    unsafe { log_write_mmio(value as u64, 8, addr, _THIS_IP_, _RET_IP_) };
    unsafe { __io_bw() };
    unsafe { __raw_writeb(value, addr) };
    unsafe { __io_aw() };
    unsafe { log_post_write_mmio(value as u64, 8, addr, _THIS_IP_, _RET_IP_) };
}

#[inline]
pub unsafe fn writew(value: u16, addr: *mut c_void) {
    unsafe { log_write_mmio(value as u64, 16, addr, _THIS_IP_, _RET_IP_) };
    unsafe { __io_bw() };
    unsafe { __raw_writew(value.to_le(), addr) };
    unsafe { __io_aw() };
    unsafe { log_post_write_mmio(value as u64, 16, addr, _THIS_IP_, _RET_IP_) };
}

#[inline]
pub unsafe fn writel(value: u32, addr: *mut c_void) {
    unsafe { log_write_mmio(value as u64, 32, addr, _THIS_IP_, _RET_IP_) };
    unsafe { __io_bw() };
    unsafe { __raw_writel(value.to_le(), addr) };
    unsafe { __io_aw() };
    unsafe { log_post_write_mmio(value as u64, 32, addr, _THIS_IP_, _RET_IP_) };
}

#[inline]
pub unsafe fn writeq(value: u64, addr: *mut c_void) {
    unsafe { log_write_mmio(value, 64, addr, _THIS_IP_, _RET_IP_) };
    unsafe { __io_bw() };
    unsafe { __raw_writeq(value.to_le(), addr) };
    unsafe { __io_aw() };
    unsafe { log_post_write_mmio(value, 64, addr, _THIS_IP_, _RET_IP_) };
}

/*
 * {read,write}{b,w,l,q}_relaxed() are like the regular version, but
 * are not guaranteed to provide ordering against spinlocks or memory
 * accesses.
 */

#[inline]
pub unsafe fn readb_relaxed(addr: *const c_void) -> u8 {
    let val: u8;

    unsafe { log_read_mmio(8, addr, _THIS_IP_, _RET_IP_) };
    val = unsafe { __raw_readb(addr) };
    unsafe { log_post_read_mmio(val as u64, 8, addr, _THIS_IP_, _RET_IP_) };
    val
}

#[inline]
pub unsafe fn readw_relaxed(addr: *const c_void) -> u16 {
    let val: u16;

    unsafe { log_read_mmio(16, addr, _THIS_IP_, _RET_IP_) };
    val = u16::from_le(unsafe { __raw_readw(addr) });
    unsafe { log_post_read_mmio(val as u64, 16, addr, _THIS_IP_, _RET_IP_) };
    val
}

#[inline]
pub unsafe fn readl_relaxed(addr: *const c_void) -> u32 {
    let val: u32;

    unsafe { log_read_mmio(32, addr, _THIS_IP_, _RET_IP_) };
    val = u32::from_le(unsafe { __raw_readl(addr) });
    unsafe { log_post_read_mmio(val as u64, 32, addr, _THIS_IP_, _RET_IP_) };
    val
}

// Original condition: #if defined(readq) && !defined(readq_relaxed)
#[inline]
pub unsafe fn readq_relaxed(addr: *const c_void) -> u64 {
    let val: u64;

    unsafe { log_read_mmio(64, addr, _THIS_IP_, _RET_IP_) };
    val = u64::from_le(unsafe { __raw_readq(addr) });
    unsafe { log_post_read_mmio(val, 64, addr, _THIS_IP_, _RET_IP_) };
    val
}

#[inline]
pub unsafe fn writeb_relaxed(value: u8, addr: *mut c_void) {
    unsafe { log_write_mmio(value as u64, 8, addr, _THIS_IP_, _RET_IP_) };
    unsafe { __raw_writeb(value, addr) };
    unsafe { log_post_write_mmio(value as u64, 8, addr, _THIS_IP_, _RET_IP_) };
}

#[inline]
pub unsafe fn writew_relaxed(value: u16, addr: *mut c_void) {
    unsafe { log_write_mmio(value as u64, 16, addr, _THIS_IP_, _RET_IP_) };
    unsafe { __raw_writew(value.to_le(), addr) };
    unsafe { log_post_write_mmio(value as u64, 16, addr, _THIS_IP_, _RET_IP_) };
}

#[inline]
pub unsafe fn writel_relaxed(value: u32, addr: *mut c_void) {
    unsafe { log_write_mmio(value as u64, 32, addr, _THIS_IP_, _RET_IP_) };
    unsafe { __raw_writel(value.to_le(), addr) };
    unsafe { log_post_write_mmio(value as u64, 32, addr, _THIS_IP_, _RET_IP_) };
}

// Original condition: #if defined(writeq) && !defined(writeq_relaxed)
#[inline]
pub unsafe fn writeq_relaxed(value: u64, addr: *mut c_void) {
    unsafe { log_write_mmio(value, 64, addr, _THIS_IP_, _RET_IP_) };
    unsafe { __raw_writeq(value.to_le(), addr) };
    unsafe { log_post_write_mmio(value, 64, addr, _THIS_IP_, _RET_IP_) };
}

/*
 * {read,write}s{b,w,l,q}() repeatedly access the same memory address in
 * native endianness in 8-, 16-, 32- or 64-bit chunks (@count times).
 */

#[inline]
pub unsafe fn readsb(addr: *const c_void, buffer: *mut c_void, mut count: ::core::ffi::c_uint) {
    if count != 0 {
        let mut buf = buffer as *mut u8;

        loop {
            let x: u8 = unsafe { __raw_readb(addr) };
            unsafe { *buf = x };
            buf = unsafe { buf.add(1) };
            count = count.wrapping_sub(1);
            if count == 0 {
                break;
            }
        }
    }
}

#[inline]
pub unsafe fn readsw(addr: *const c_void, buffer: *mut c_void, mut count: ::core::ffi::c_uint) {
    if count != 0 {
        let mut buf = buffer as *mut u16;

        loop {
            let x: u16 = unsafe { __raw_readw(addr) };
            unsafe { *buf = x };
            buf = unsafe { buf.add(1) };
            count = count.wrapping_sub(1);
            if count == 0 {
                break;
            }
        }
    }
}

#[inline]
pub unsafe fn readsl(addr: *const c_void, buffer: *mut c_void, mut count: ::core::ffi::c_uint) {
    if count != 0 {
        let mut buf = buffer as *mut u32;

        loop {
            let x: u32 = unsafe { __raw_readl(addr) };
            unsafe { *buf = x };
            buf = unsafe { buf.add(1) };
            count = count.wrapping_sub(1);
            if count == 0 {
                break;
            }
        }
    }
}

#[inline]
pub unsafe fn readsq(addr: *const c_void, buffer: *mut c_void, mut count: ::core::ffi::c_uint) {
    if count != 0 {
        let mut buf = buffer as *mut u64;

        loop {
            let x: u64 = unsafe { __raw_readq(addr) };
            unsafe { *buf = x };
            buf = unsafe { buf.add(1) };
            count = count.wrapping_sub(1);
            if count == 0 {
                break;
            }
        }
    }
}

#[inline]
pub unsafe fn writesb(addr: *mut c_void, buffer: *const c_void, mut count: ::core::ffi::c_uint) {
    if count != 0 {
        let mut buf = buffer as *const u8;

        loop {
            unsafe { __raw_writeb(*buf, addr) };
            buf = unsafe { buf.add(1) };
            count = count.wrapping_sub(1);
            if count == 0 {
                break;
            }
        }
    }
}

#[inline]
pub unsafe fn writesw(addr: *mut c_void, buffer: *const c_void, mut count: ::core::ffi::c_uint) {
    if count != 0 {
        let mut buf = buffer as *const u16;

        loop {
            unsafe { __raw_writew(*buf, addr) };
            buf = unsafe { buf.add(1) };
            count = count.wrapping_sub(1);
            if count == 0 {
                break;
            }
        }
    }
}

#[inline]
pub unsafe fn writesl(addr: *mut c_void, buffer: *const c_void, mut count: ::core::ffi::c_uint) {
    if count != 0 {
        let mut buf = buffer as *const u32;

        loop {
            unsafe { __raw_writel(*buf, addr) };
            buf = unsafe { buf.add(1) };
            count = count.wrapping_sub(1);
            if count == 0 {
                break;
            }
        }
    }
}

#[inline]
pub unsafe fn writesq(addr: *mut c_void, buffer: *const c_void, mut count: ::core::ffi::c_uint) {
    if count != 0 {
        let mut buf = buffer as *const u64;

        loop {
            unsafe { __raw_writeq(*buf, addr) };
            buf = unsafe { buf.add(1) };
            count = count.wrapping_sub(1);
            if count == 0 {
                break;
            }
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
