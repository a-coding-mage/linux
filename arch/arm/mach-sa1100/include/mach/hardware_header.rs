/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/arm/mach-sa1100/include/mach/hardware.h
 *
 * Copyright (C) 1998 Nicolas Pitre <nico@fluxnic.net>
 *
 * This file contains the hardware definitions for SA1100 architecture
 *
 * 2000/05/23 John Dorsey <john+@cs.cmu.edu>
 *      Definitions for SA1111 added.
 */

// The original header guard was: __ASM_ARCH_HARDWARE_H

pub const UNCACHEABLE_ADDR: usize = 0xfa05_0000; // ICIP

/*
 * SA1100 internal I/O mappings
 *
 * We have the following mapping:
 *      phys            virt
 *      80000000        f8000000
 *      90000000        fa000000
 *      a0000000        fc000000
 *      b0000000        fe000000
 */

pub const VIO_BASE: usize = 0xf800_0000; // virtual start of IO space
pub const VIO_SHIFT: usize = 3; // x = IO space shrink power
pub const PIO_START: usize = 0x8000_0000; // physical start of IO space

#[inline]
pub const fn io_p2v(x: usize) -> usize {
    ((x & 0x00ff_ffff) | ((x & 0x3000_0000) >> VIO_SHIFT)).wrapping_add(VIO_BASE)
}

#[inline]
pub const fn io_v2p(x: usize) -> usize {
    ((x & 0x00ff_ffff) | ((x & (0x3000_0000 >> VIO_SHIFT)) << VIO_SHIFT))
        .wrapping_add(PIO_START)
}

// Supplied by the platform's I/O-memory abstraction (the C header used IOMEM).
extern "C" {
    pub fn IOMEM(addr: usize) -> *mut u8;
}

#[inline]
pub unsafe fn __MREG(x: usize) -> *mut u8 {
    IOMEM(io_p2v(x))
}

// Non-assembly form: __REG(x) denotes a volatile unsigned-long register lvalue.
#[inline]
pub unsafe fn __REG(x: usize) -> *mut usize {
    IOMEM(io_p2v(x)) as *mut usize
}

#[inline]
pub fn __PREG(x: *const usize) -> usize {
    io_v2p(x as usize)
}

// Assembly form uses __REG(x) = io_p2v(x) and __PREG(x) = io_v2p(x).


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
