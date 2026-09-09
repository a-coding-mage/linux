/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/cpu-sh4/sq.h
 *
 * Copyright (C) 2001, 2002, 2003  Paul Mundt
 * Copyright (C) 2001, 2002  M. R. Brown
 */

// Dependencies supplied by the surrounding architecture bindings:
// asm/addrspace.h and asm/page.h

/*
 * Store queues range from e0000000-e3fffffc, allowing approx. 64MB to be
 * mapped to any physical address space. Since data is written (and aligned)
 * to 32-byte boundaries, we need to be sure that all allocations are aligned.
 */
pub const SQ_SIZE: usize = 32;
pub const SQ_ALIGN_MASK: usize = !(SQ_SIZE - 1);

#[inline]
pub const fn sq_align(addr: usize) -> usize {
    (addr + SQ_SIZE - 1) & SQ_ALIGN_MASK
}

// P4SEG_REG_BASE and P4SEG_STORE_QUE are supplied by asm/addrspace.h.
pub const SQ_QACR0: usize = P4SEG_REG_BASE + 0x38;
pub const SQ_QACR1: usize = P4SEG_REG_BASE + 0x3c;
pub const SQ_ADDRMAX: usize = P4SEG_STORE_QUE + 0x04000000;

// arch/sh/kernel/cpu/sh4/sq.c
extern "C" {
    pub fn sq_remap(
        phys: ::core::ffi::c_ulong,
        size: ::core::ffi::c_uint,
        name: *const ::core::ffi::c_char,
        prot: pgprot_t,
    ) -> ::core::ffi::c_ulong;
    pub fn sq_unmap(vaddr: ::core::ffi::c_ulong);
    pub fn sq_flush_range(start: ::core::ffi::c_ulong, len: ::core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
