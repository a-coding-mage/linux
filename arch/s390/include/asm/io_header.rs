/* SPDX-License-Identifier: GPL-2.0 */
/*
 * S390 version
 *   Copyright IBM Corp. 1999
 *   Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
 *
 * Derived from "include/asm-i386/io.h"
 */

use core::ffi::c_void;

/* Dependencies supplied by the surrounding kernel translation. */

// #define xlate_dev_mem_ptr xlate_dev_mem_ptr
// #define kc_xlate_dev_mem_ptr xlate_dev_mem_ptr
extern "C" {
    pub fn xlate_dev_mem_ptr(phys: phys_addr_t) -> *mut c_void;
}

// #define unxlate_dev_mem_ptr unxlate_dev_mem_ptr
// #define kc_unxlate_dev_mem_ptr unxlate_dev_mem_ptr
extern "C" {
    pub fn unxlate_dev_mem_ptr(phys: phys_addr_t, addr: *mut c_void);
}

pub const IO_SPACE_LIMIT: u64 = 0;

/* I/O memory mapping functions. */
// #define ioremap_prot ioremap_prot
// #define iounmap iounmap

pub const _PAGE_IOREMAP: _ = pgprot_val(PAGE_KERNEL);

#[inline]
pub unsafe fn ioremap_wc(addr: usize, size: usize) -> *mut c_void {
    ioremap_prot(addr, size, pgprot_writecombine(PAGE_KERNEL))
}

#[inline]
pub unsafe fn ioport_map(_port: c_ulong, _nr: c_uint) -> *mut c_void {
    core::ptr::null_mut()
}

#[inline]
pub unsafe fn ioport_unmap(_p: *mut c_void) {
}

#[cfg(feature = "CONFIG_PCI")]
mod config_pci {
    use super::*;

    /*
     * s390 needs a private implementation of pci_iomap since ioremap with
     * its offset parameter isn't sufficient. BAR spaces are not disjunctive
     * on s390, so the bar parameter is needed to find the device and create
     * the mapping cookie.
     */
    // #define pci_iomap pci_iomap
    // #define pci_iomap_range pci_iomap_range
    // #define pci_iounmap pci_iounmap
    // #define pci_iomap_wc pci_iomap_wc
    // #define pci_iomap_wc_range pci_iomap_wc_range

    // #define memcpy_fromio(dst, src, count) zpci_memcpy_fromio(dst, src, count)
    // #define memcpy_toio(dst, src, count) zpci_memcpy_toio(dst, src, count)
    // #define memset_io(dst, val, count) zpci_memset_io(dst, val, count)
    // #define mmiowb() zpci_barrier()

    // #define __raw_readb zpci_read_u8
    // #define __raw_readw zpci_read_u16
    // #define __raw_readl zpci_read_u32
    // #define __raw_readq zpci_read_u64
    // #define __raw_writeb zpci_write_u8
    // #define __raw_writew zpci_write_u16
    // #define __raw_writel zpci_write_u32
    // #define __raw_writeq zpci_write_u64

    /* combine single writes by using store-block insn */
    #[inline]
    pub unsafe fn __iowrite32_copy(to: *mut c_void, from: *const c_void, count: usize) {
        zpci_memcpy_toio(to, from, count.wrapping_mul(4));
    }

    // #define __iowrite32_copy __iowrite32_copy

    #[inline]
    pub unsafe fn __iowrite64_copy(to: *mut c_void, from: *const c_void, count: usize) {
        zpci_memcpy_toio(to, from, count.wrapping_mul(8));
    }

    // #define __iowrite64_copy __iowrite64_copy
}

// #include <asm-generic/io.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
