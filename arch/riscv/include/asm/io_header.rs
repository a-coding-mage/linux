/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * {read,write}{b,w,l,q} based on arch/arm64/include/asm/io.h
 *   which was based on arch/arm/include/io.h
 *
 * Copyright (C) 1996-2000 Russell King
 * Copyright (C) 2012 ARM Ltd.
 * Copyright (C) 2014 Regents of the University of California
 */

// C dependencies: linux/types.h, linux/pgtable.h, asm/mmiowb.h,
// asm/early_ioremap.h, asm/mmio.h, and asm-generic/io.h.

#[cfg(feature = "CONFIG_MMU")]
pub const IO_SPACE_LIMIT: usize = PCI_IO_SIZE - 1;

#[cfg(feature = "CONFIG_MMU")]
pub const PCI_IOBASE: *mut core::ffi::c_void = PCI_IO_START as *mut core::ffi::c_void;

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn ioremap_wc(addr: usize, size: usize) -> *mut core::ffi::c_void {
    ioremap_prot(addr, size, __pgprot(_PAGE_KERNEL_NC))
}

/* Port-mapped I/O fences. */
#[inline]
unsafe fn __io_pbr() { RISCV_FENCE!(io, i); }
#[inline]
unsafe fn __io_par(_v: *const core::ffi::c_void) { RISCV_FENCE!(i, ior); }
#[inline]
unsafe fn __io_pbw() { RISCV_FENCE!(iow, o); }
#[inline]
unsafe fn __io_paw() { RISCV_FENCE!(o, io); }

/* Accesses from a single hart to a single I/O address must be ordered. */
#[inline]
pub unsafe fn __readsb(addr: *const core::ffi::c_void, buffer: *mut u8, mut count: u32) {
    __io_br!();
    if count != 0 {
        let mut buf = buffer;
        loop {
            *buf = __raw_readb(addr);
            buf = buf.add(1);
            count = count.wrapping_sub(1);
            if count == 0 { break; }
        }
    }
    __io_ar!(addr);
}

#[inline]
pub unsafe fn __readsw(addr: *const core::ffi::c_void, buffer: *mut u16, mut count: u32) {
    __io_br!();
    if count != 0 {
        let mut buf = buffer;
        loop {
            *buf = __raw_readw(addr);
            buf = buf.add(1);
            count = count.wrapping_sub(1);
            if count == 0 { break; }
        }
    }
    __io_ar!(addr);
}

#[inline]
pub unsafe fn __readsl(addr: *const core::ffi::c_void, buffer: *mut u32, mut count: u32) {
    __io_br!();
    if count != 0 {
        let mut buf = buffer;
        loop {
            *buf = __raw_readl(addr);
            buf = buf.add(1);
            count = count.wrapping_sub(1);
            if count == 0 { break; }
        }
    }
    __io_ar!(addr);
}

#[macro_export]
macro_rules! readsb { ($addr:expr, $buffer:expr, $count:expr) => { $crate::__readsb($addr, $buffer, $count) }; }
#[macro_export]
macro_rules! readsw { ($addr:expr, $buffer:expr, $count:expr) => { $crate::__readsw($addr, $buffer, $count) }; }
#[macro_export]
macro_rules! readsl { ($addr:expr, $buffer:expr, $count:expr) => { $crate::__readsl($addr, $buffer, $count) }; }

#[inline]
pub unsafe fn __writesb(addr: *mut core::ffi::c_void, buffer: *const u8, mut count: u32) {
    __io_bw!();
    if count != 0 {
        let mut buf = buffer;
        loop {
            __raw_writeb(*buf, addr);
            buf = buf.add(1);
            count = count.wrapping_sub(1);
            if count == 0 { break; }
        }
    }
    __io_aw!();
}

#[inline]
pub unsafe fn __writesw(addr: *mut core::ffi::c_void, buffer: *const u16, mut count: u32) {
    __io_bw!();
    if count != 0 {
        let mut buf = buffer;
        loop {
            __raw_writew(*buf, addr);
            buf = buf.add(1);
            count = count.wrapping_sub(1);
            if count == 0 { break; }
        }
    }
    __io_aw!();
}

#[inline]
pub unsafe fn __writesl(addr: *mut core::ffi::c_void, buffer: *const u32, mut count: u32) {
    __io_bw!();
    if count != 0 {
        let mut buf = buffer;
        loop {
            __raw_writel(*buf, addr);
            buf = buf.add(1);
            count = count.wrapping_sub(1);
            if count == 0 { break; }
        }
    }
    __io_aw!();
}

#[macro_export]
macro_rules! writesb { ($addr:expr, $buffer:expr, $count:expr) => { $crate::__writesb($addr, $buffer, $count) }; }
#[macro_export]
macro_rules! writesw { ($addr:expr, $buffer:expr, $count:expr) => { $crate::__writesw($addr, $buffer, $count) }; }
#[macro_export]
macro_rules! writesl { ($addr:expr, $buffer:expr, $count:expr) => { $crate::__writesl($addr, $buffer, $count) }; }

#[cfg(feature = "CONFIG_64BIT")]
#[inline]
pub unsafe fn __readsq(addr: *const core::ffi::c_void, buffer: *mut u64, mut count: u32) {
    __io_br!();
    if count != 0 {
        let mut buf = buffer;
        loop { *buf = __raw_readq(addr); buf = buf.add(1); count = count.wrapping_sub(1); if count == 0 { break; } }
    }
    __io_ar!(addr);
}
#[cfg(feature = "CONFIG_64BIT")]
#[macro_export]
macro_rules! readsq { ($addr:expr, $buffer:expr, $count:expr) => { $crate::__readsq($addr, $buffer, $count) }; }

#[cfg(feature = "CONFIG_64BIT")]
#[inline]
pub unsafe fn __writesq(addr: *mut core::ffi::c_void, buffer: *const u64, mut count: u32) {
    __io_bw!();
    if count != 0 {
        let mut buf = buffer;
        loop { __raw_writeq(*buf, addr); buf = buf.add(1); count = count.wrapping_sub(1); if count == 0 { break; } }
    }
    __io_aw!();
}
#[cfg(feature = "CONFIG_64BIT")]
#[macro_export]
macro_rules! writesq { ($addr:expr, $buffer:expr, $count:expr) => { $crate::__writesq($addr, $buffer, $count) }; }

// CONFIG_HAS_IOPORT variants use PCI_IOBASE + the supplied port offset and
// the same raw access loops with port-specific fences.

#[cfg(feature = "CONFIG_MMU")]
#[inline]
pub unsafe fn arch_memremap_wb(addr: usize, size: usize, _flags: usize) -> *mut core::ffi::c_void {
    ioremap_prot(addr, size, __pgprot(_PAGE_KERNEL)) as *mut core::ffi::c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
