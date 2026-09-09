/*
 * Copyright (C) 2014 Altera Corporation
 * Copyright (C) 2010 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependency intent from <linux/types.h> and <asm/pgtable-bits.h>.

/* PCI is not supported in nios2, set this to 0. */
pub const IO_SPACE_LIMIT: usize = 0;

macro_rules! readb_relaxed {
    ($addr:expr) => { readb($addr) };
}
macro_rules! readw_relaxed {
    ($addr:expr) => { readw($addr) };
}
macro_rules! readl_relaxed {
    ($addr:expr) => { readl($addr) };
}

macro_rules! writeb_relaxed {
    ($x:expr, $addr:expr) => { writeb($x, $addr) };
}
macro_rules! writew_relaxed {
    ($x:expr, $addr:expr) => { writew($x, $addr) };
}
macro_rules! writel_relaxed {
    ($x:expr, $addr:expr) => { writel($x, $addr) };
}

unsafe extern "C" {
    pub fn ioremap(physaddr: usize, size: usize) -> *mut core::ffi::c_void;
    pub fn iounmap(addr: *mut core::ffi::c_void);
}

/* Macros used for converting between virtual and physical mappings. */
pub unsafe fn phys_to_virt(vaddr: usize) -> *mut core::ffi::c_void {
    // CONFIG_NIOS2_KERNEL_REGION_BASE is supplied by the build configuration.
    ((vaddr | CONFIG_NIOS2_KERNEL_REGION_BASE) as *mut core::ffi::c_void)
}

/* Clear top 3 bits */
pub fn virt_to_phys(vaddr: usize) -> usize {
    vaddr & !0xE0000000usize
}

// Contents corresponding to <asm-generic/io.h> are supplied by the generic IO dependency.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
