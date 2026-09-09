/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * OpenRISC implementation:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 * et al.
 */

/* C header guard: __ASM_OPENRISC_PAGE_H */
/* C dependencies: <vdso/page.h>, <asm/setup.h>,
 * <asm-generic/memory_model.h>, and <asm-generic/getorder.h>.
 */

pub const PAGE_OFFSET: usize = 0xc0000000;
pub const KERNELBASE: usize = PAGE_OFFSET;

/* This is not necessarily the right place for this, but it's needed by
 * drivers/of/fdt.c
 */

/* The following items are omitted by the C __ASSEMBLER__ condition. */

#[macro_export]
macro_rules! clear_page {
    ($page:expr) => {
        memset($page, 0, PAGE_SIZE)
    };
}

#[macro_export]
macro_rules! copy_page {
    ($to:expr, $from:expr) => {
        memcpy($to, $from, PAGE_SIZE)
    };
}

#[macro_export]
macro_rules! copy_user_page {
    ($to:expr, $from:expr, $vaddr:expr, $pg:expr) => {
        copy_page!($to, $from)
    };
}

/*
 * These are used to make use of C type-checking..
 */
#[repr(C)]
pub struct pte_t {
    pub pte: usize,
}

#[repr(C)]
pub struct pgd_t {
    pub pgd: usize,
}

#[repr(C)]
pub struct pgprot_t {
    pub pgprot: usize,
}

pub type pgtable_t = *mut page;

#[macro_export]
macro_rules! pte_val {
    ($x:expr) => {
        $x.pte
    };
}

#[macro_export]
macro_rules! pgd_val {
    ($x:expr) => {
        $x.pgd
    };
}

#[macro_export]
macro_rules! pgprot_val {
    ($x:expr) => {
        $x.pgprot
    };
}

#[inline]
pub const fn __pte(x: usize) -> pte_t {
    pte_t { pte: x }
}

#[inline]
pub const fn __pgd(x: usize) -> pgd_t {
    pgd_t { pgd: x }
}

#[inline]
pub const fn __pgprot(x: usize) -> pgprot_t {
    pgprot_t { pgprot: x }
}

#[macro_export]
macro_rules! __va {
    ($x:expr) => {
        (($x as usize).wrapping_add(PAGE_OFFSET)) as *mut core::ffi::c_void
    };
}

#[macro_export]
macro_rules! __pa {
    ($x:expr) => {
        ($x as usize).wrapping_sub(PAGE_OFFSET)
    };
}

#[inline]
pub unsafe fn virt_to_pfn(kaddr: *const core::ffi::c_void) -> usize {
    __pa!(kaddr) >> PAGE_SHIFT
}

#[macro_export]
macro_rules! virt_to_page {
    ($addr:expr) => {
        mem_map.add((($addr as usize).wrapping_sub(PAGE_OFFSET)) >> PAGE_SHIFT)
    };
}

#[macro_export]
macro_rules! virt_addr_valid {
    ($kaddr:expr) => {
        pfn_valid(virt_to_pfn($kaddr))
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
