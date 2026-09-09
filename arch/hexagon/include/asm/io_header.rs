/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * IO definitions for the Hexagon architecture
 *
 * Copyright (c) 2010-2013, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the Linux and architecture headers:
// linux/types.h, asm/page.h, asm/cacheflush.h, and asm-generic/io.h.

extern "C" {
    pub fn remap_area_pages(
        start: c_ulong,
        phys_addr: c_ulong,
        end: c_ulong,
        flags: c_ulong,
    ) -> c_int;
}

/*
 * virt_to_phys - map virtual address to physical
 * @address:  address to map
 */
#[inline]
pub unsafe fn virt_to_phys(address: *mut core::ffi::c_void) -> c_ulong {
    __pa(address)
}

/*
 * phys_to_virt - map physical address to virtual
 * @address: address to map
 */
#[inline]
pub unsafe fn phys_to_virt(address: c_ulong) -> *mut core::ffi::c_void {
    __va(address)
}

/*
 * readb - read byte from memory mapped device
 * @addr:  pointer to memory
 *
 */
#[inline]
pub unsafe fn __raw_readb(addr: *const core::ffi::c_void) -> u8 {
    let mut val: u8;
    core::arch::asm!(
        "memb({1})",
        out(reg) val,
        in(reg) addr,
    );
    val
}

#[inline]
pub unsafe fn __raw_readw(addr: *const core::ffi::c_void) -> u16 {
    let mut val: u16;
    core::arch::asm!(
        "memh({1})",
        out(reg) val,
        in(reg) addr,
    );
    val
}

#[inline]
pub unsafe fn __raw_readl(addr: *const core::ffi::c_void) -> u32 {
    let mut val: u32;
    core::arch::asm!(
        "memw({1})",
        out(reg) val,
        in(reg) addr,
    );
    val
}

/*
 * writeb - write a byte to a memory location
 * @data: data to write to
 * @addr:  pointer to memory
 *
 */
#[inline]
pub unsafe fn __raw_writeb(data: u8, addr: *mut core::ffi::c_void) {
    core::arch::asm!(
        "memb({0}) = {1}",
        in(reg) addr,
        in(reg) data,
        options(nostack, preserves_flags),
    );
}

#[inline]
pub unsafe fn __raw_writew(data: u16, addr: *mut core::ffi::c_void) {
    core::arch::asm!(
        "memh({0}) = {1}",
        in(reg) addr,
        in(reg) data,
        options(nostack, preserves_flags),
    );
}

#[inline]
pub unsafe fn __raw_writel(data: u32, addr: *mut core::ffi::c_void) {
    core::arch::asm!(
        "memw({0}) = {1}",
        in(reg) addr,
        in(reg) data,
        options(nostack, preserves_flags),
    );
}

/* I/O memory mapping functions. */
pub const _PAGE_IOREMAP: c_ulong = _PAGE_PRESENT | _PAGE_READ | _PAGE_WRITE | (__HEXAGON_C_DEV << 6);

// The generic io.h supplies the remaining API contract and helpers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
