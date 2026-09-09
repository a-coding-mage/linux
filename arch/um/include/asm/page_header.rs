/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2000 - 2003 Jeff Dike (jdike@addtoit.com)
 * Copyright 2003 PathScale, Inc.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/const.h, vdso/page.h, linux/pfn.h, linux/types.h,
// asm/vm-flags.h, mem.h, asm-generic/memory_model.h, asm-generic/getorder.h.

// These are used to make use of C type-checking.

#[inline]
pub unsafe fn clear_page(page: *mut libc::c_void) {
    core::ptr::write_bytes(page as *mut u8, 0, PAGE_SIZE as usize);
}

#[inline]
pub unsafe fn copy_page(to: *mut libc::c_void, from: *const libc::c_void) {
    core::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, PAGE_SIZE as usize);
}

#[inline]
pub unsafe fn copy_user_page(
    to: *mut libc::c_void,
    from: *const libc::c_void,
    _vaddr: libc::c_ulong,
    _pg: *mut Page,
) {
    copy_page(to, from);
}

#[repr(C)]
pub struct Page;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PteT {
    pub pte: libc::c_ulong,
}
pub type pte_t = PteT;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PgdT {
    pub pgd: libc::c_ulong,
}
pub type pgd_t = PgdT;

// Present when CONFIG_PGTABLE_LEVELS > 2.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PmdT {
    pub pmd: libc::c_ulong,
}
pub type pmd_t = PmdT;

#[inline]
pub fn pmd_val(x: pmd_t) -> libc::c_ulong { x.pmd }
#[inline]
pub fn __pmd(x: libc::c_ulong) -> pmd_t { PmdT { pmd: x } }

// Present when CONFIG_PGTABLE_LEVELS > 3.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PudT {
    pub pud: libc::c_ulong,
}
pub type pud_t = PudT;

#[inline]
pub fn pud_val(x: pud_t) -> libc::c_ulong { x.pud }
#[inline]
pub fn __pud(x: libc::c_ulong) -> pud_t { PudT { pud: x } }

#[inline]
pub fn pte_val(x: pte_t) -> libc::c_ulong { x.pte }

#[inline]
pub fn pte_get_bits(p: pte_t, bits: libc::c_ulong) -> libc::c_ulong { p.pte & bits }
#[inline]
pub fn pte_set_bits(p: &mut pte_t, bits: libc::c_ulong) { p.pte |= bits; }
#[inline]
pub fn pte_clear_bits(p: &mut pte_t, bits: libc::c_ulong) { p.pte &= !bits; }
#[inline]
pub fn pte_copy(to: &mut pte_t, from: pte_t) { to.pte = from.pte; }
#[inline]
pub fn pte_is_zero(p: pte_t) -> bool { (p.pte & !(_PAGE_NEEDSYNC as libc::c_ulong)) == 0 }
#[inline]
pub unsafe fn pte_set_val(p: &mut pte_t, phys: libc::c_ulong, prot: pgprot_t) {
    p.pte = phys | pgprot_val(prot);
}

pub type phys_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PgprotT {
    pub pgprot: libc::c_ulong,
}
pub type pgprot_t = PgprotT;

pub type pgtable_t = *mut Page;

#[inline]
pub fn pgd_val(x: pgd_t) -> libc::c_ulong { x.pgd }
#[inline]
pub fn pgprot_val(x: pgprot_t) -> libc::c_ulong { x.pgprot }

#[inline]
pub fn __pte(x: libc::c_ulong) -> pte_t { PteT { pte: x } }
#[inline]
pub fn __pgd(x: libc::c_ulong) -> pgd_t { PgdT { pgd: x } }
#[inline]
pub fn __pgprot(x: libc::c_ulong) -> pgprot_t { PgprotT { pgprot: x } }

unsafe extern "C" {
    pub static mut uml_physmem: libc::c_ulong;
}

// #define PAGE_OFFSET (uml_physmem)
#[inline]
pub unsafe fn PAGE_OFFSET() -> libc::c_ulong { uml_physmem }
// #define KERNELBASE PAGE_OFFSET
#[inline]
pub unsafe fn KERNELBASE() -> libc::c_ulong { PAGE_OFFSET() }

pub const __VA_SPACE: libc::c_ulong = 8 * 1024 * 1024;

extern "C" {
    pub fn uml_to_phys(virt: *mut libc::c_void) -> libc::c_ulong;
    pub fn uml_to_virt(phys: libc::c_ulong) -> *mut libc::c_void;
    pub fn pfn_valid(pfn: libc::c_ulong) -> bool;
}

// Cast to unsigned long before casting to void * to avoid a warning from
// mmap_kmem about cutting a long long down to a void *. Not sure that
// casting is the right thing, but 32-bit UML can't have 64-bit virtual addresses.
#[inline]
pub unsafe fn __pa(virt: libc::c_ulong) -> libc::c_ulong {
    uml_to_phys(virt as *mut libc::c_void)
}
#[inline]
pub unsafe fn __va(phys: libc::c_ulong) -> *mut libc::c_void {
    uml_to_virt(phys)
}

#[inline]
pub fn phys_to_pfn(p: libc::c_ulong) -> libc::c_ulong { p >> PAGE_SHIFT }
#[inline]
pub fn pfn_to_phys(pfn: libc::c_ulong) -> libc::c_ulong { PFN_PHYS(pfn) }
#[inline]
pub unsafe fn virt_addr_valid(v: libc::c_ulong) -> bool {
    pfn_valid(phys_to_pfn(__pa(v)))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
