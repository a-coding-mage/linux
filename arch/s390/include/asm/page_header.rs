/* SPDX-License-Identifier: GPL-2.0 */
/*
 * S390 version
 * Copyright IBM Corp. 1999, 2000
 * Author(s): Hartmut Penner (hp@de.ibm.com)
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// not reimplemented here: linux/const.h, asm/types.h, asm/asm.h, vdso/page.h,
// asm/setup.h, asm-generic/memory_model.h, and asm-generic/getorder.h.

pub const PAGE_DEFAULT_ACC: usize = 0;
/* storage-protection override */
pub const PAGE_SPO_ACC: usize = 9;
pub const PAGE_DEFAULT_KEY: usize = PAGE_DEFAULT_ACC << 4;

pub const HPAGE_SHIFT: usize = 20;
pub const HPAGE_SIZE: usize = 1usize << HPAGE_SHIFT;
pub const HPAGE_MASK: usize = !(HPAGE_SIZE - 1);
pub const HUGETLB_PAGE_ORDER: usize = HPAGE_SHIFT - PAGE_SHIFT;
pub const HUGE_MAX_HSTATE: usize = 2;

// ARCH_HAS_SETCLEAR_HUGE_PTE, ARCH_HAS_HUGE_PTE_TYPE,
// ARCH_HAS_PREPARE_HUGEPAGE, ARCH_HAS_HUGEPAGE_CLEAR_FLUSH,
// HAVE_ARCH_HUGETLB_UNMAPPED_AREA

extern "C" {
    pub fn __storage_key_init_range(start: usize, end: usize);
}

#[inline]
pub unsafe fn storage_key_init_range(start: usize, end: usize) {
    if PAGE_DEFAULT_KEY != 0 {
        __storage_key_init_range(start, end);
    }
}

#[inline]
pub unsafe fn clear_page(page: *mut core::ffi::c_void) {
    core::ptr::write_bytes(page as *mut u8, 0, PAGE_SIZE);
}

/*
 * copy_page uses the mvcl instruction with 0xb0 padding byte in order to
 * bypass caches when copying a page. Especially when copying huge pages
 * this keeps L1 and L2 data caches alive.
 */
#[inline]
pub unsafe fn copy_page(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void) {
    let mut dst: register_pair = register_pair { even: to as usize, odd: 0x1000 };
    let mut src: register_pair = register_pair { even: from as usize, odd: 0xb0001000 };
    core::arch::asm!("mvcl {dst}, {src}", dst = inout(reg) dst.pair, src = inout(reg) src.pair, options(nostack));
}

#[inline]
pub unsafe fn copy_user_page(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void, _vaddr: usize, _pg: *mut core::ffi::c_void) {
    copy_page(to, from);
}

// vma_alloc_zeroed_movable_folio(vma, vaddr) expands to
// vma_alloc_folio(GFP_HIGHUSER_MOVABLE | __GFP_ZERO, 0, vma, vaddr).

#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[repr(C)] pub struct pgprot_t { pub pgprot: usize }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[repr(C)] pub struct pte_t { pub pte: usize }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[repr(C)] pub struct pmd_t { pub pmd: usize }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[repr(C)] pub struct pud_t { pub pud: usize }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[repr(C)] pub struct p4d_t { pub p4d: usize }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[repr(C)] pub struct pgd_t { pub pgd: usize }

#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
pub type pgprot_t = usize;
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
pub type pte_t = usize;
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
pub type pmd_t = usize;
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
pub type pud_t = usize;
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
pub type p4d_t = usize;
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
pub type pgd_t = usize;

#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[inline] pub const fn pgprot_val(v: pgprot_t) -> usize { v.pgprot }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[inline] pub const fn pte_val(v: pte_t) -> usize { v.pte }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[inline] pub const fn pmd_val(v: pmd_t) -> usize { v.pmd }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[inline] pub const fn pud_val(v: pud_t) -> usize { v.pud }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[inline] pub const fn p4d_val(v: p4d_t) -> usize { v.p4d }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[inline] pub const fn pgd_val(v: pgd_t) -> usize { v.pgd }
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
#[inline] pub const fn pgprot_val(v: pgprot_t) -> usize { v }
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
#[inline] pub const fn pte_val(v: pte_t) -> usize { v }
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
#[inline] pub const fn pmd_val(v: pmd_t) -> usize { v }
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
#[inline] pub const fn pud_val(v: pud_t) -> usize { v }
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
#[inline] pub const fn p4d_val(v: p4d_t) -> usize { v }
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
#[inline] pub const fn pgd_val(v: pgd_t) -> usize { v }

pub type pgtable_t = *mut pte_t;

#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
#[inline] pub const fn __pgprot(x: usize) -> pgprot_t { x }
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
#[inline] pub const fn __pte(x: usize) -> pte_t { x }
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
#[inline] pub const fn __pmd(x: usize) -> pmd_t { x }
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
#[inline] pub const fn __pud(x: usize) -> pud_t { x }
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
#[inline] pub const fn __p4d(x: usize) -> p4d_t { x }
#[cfg(not(feature = "CONFIG_STRICT_MM_TYPECHECKS"))]
#[inline] pub const fn __pgd(x: usize) -> pgd_t { x }

#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[inline] pub const fn __pgprot(x: usize) -> pgprot_t { pgprot_t { pgprot: x } }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[inline] pub const fn __pte(x: usize) -> pte_t { pte_t { pte: x } }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[inline] pub const fn __pmd(x: usize) -> pmd_t { pmd_t { pmd: x } }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[inline] pub const fn __pud(x: usize) -> pud_t { pud_t { pud: x } }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[inline] pub const fn __p4d(x: usize) -> p4d_t { p4d_t { p4d: x } }
#[cfg(feature = "CONFIG_STRICT_MM_TYPECHECKS")]
#[inline] pub const fn __pgd(x: usize) -> pgd_t { pgd_t { pgd: x } }

#[inline]
pub unsafe fn page_set_storage_key(addr: usize, skey: u8, mapped: core::ffi::c_int) {
    if mapped == 0 {
        core::arch::asm!(".insn rrf,0xb22b0000,{skey},{addr},8,0", skey = in(reg) skey, addr = in(reg) addr, options(nostack));
    } else {
        core::arch::asm!("sske {skey},{addr}", skey = in(reg) skey, addr = in(reg) addr, options(nostack));
    }
}

#[inline]
pub unsafe fn page_get_storage_key(addr: usize) -> u8 {
    let skey: u8;
    core::arch::asm!("iske {skey},{addr}", skey = out(reg_byte) skey, addr = in(reg) addr, options(nostack));
    skey
}

#[inline]
pub unsafe fn page_reset_referenced(addr: usize) -> core::ffi::c_int {
    let mut cc: core::ffi::c_int;
    core::arch::asm!("rrbe 0,{addr}", "ipm {cc}", addr = in(reg) addr, cc = lateout(reg) cc, options(nostack));
    cc
}

extern "C" { pub fn split_pud_page(pudp: *mut pud_t, addr: usize) -> core::ffi::c_int; }

/* Bits in the storage key */
pub const _PAGE_CHANGED: u8 = 0x02;
pub const _PAGE_REFERENCED: u8 = 0x04;
pub const _PAGE_FP_BIT: u8 = 0x08;
pub const _PAGE_ACC_BITS: u8 = 0xf0;

#[repr(C)] pub struct page;
#[repr(C)] pub struct folio;
extern "C" {
    pub fn arch_free_page(page: *mut page, order: core::ffi::c_int);
    pub fn arch_alloc_page(page: *mut page, order: core::ffi::c_int);
    pub fn arch_make_folio_accessible(folio: *mut folio) -> core::ffi::c_int;
}

#[inline] pub const fn devmem_is_allowed(_pfn: usize) -> core::ffi::c_int { 0 }

#[repr(C)] pub struct vm_layout {
    pub kaslr_offset: usize,
    pub kaslr_offset_phys: usize,
    pub identity_base: usize,
    pub identity_size: usize,
}
extern "C" { pub static mut vm_layout: vm_layout; }

#[inline] pub unsafe fn __kaslr_offset() -> usize { vm_layout.kaslr_offset }
#[inline] pub unsafe fn __kaslr_offset_phys() -> usize { vm_layout.kaslr_offset_phys }
#[cfg(feature = "CONFIG_RANDOMIZE_IDENTITY_BASE")]
#[inline] pub unsafe fn __identity_base() -> usize { vm_layout.identity_base }
#[cfg(not(feature = "CONFIG_RANDOMIZE_IDENTITY_BASE"))]
#[inline] pub const fn __identity_base() -> usize { 0 }
#[inline] pub unsafe fn ident_map_size() -> usize { vm_layout.identity_size }

#[inline] pub unsafe fn kaslr_offset() -> usize { __kaslr_offset() }
extern "C" { pub static mut __kaslr_enabled: core::ffi::c_int; }
#[inline] pub unsafe fn kaslr_enabled() -> core::ffi::c_int {
    // IS_ENABLED(CONFIG_RANDOMIZE_BASE) is supplied by the build configuration.
    __kaslr_enabled
}

#[inline] pub unsafe fn __pa_nodebug(x: usize) -> usize {
    if x < __kaslr_offset() { x.wrapping_sub(__identity_base()) }
    else { x.wrapping_sub(__kaslr_offset()).wrapping_add(__kaslr_offset_phys()) }
}
#[inline] pub unsafe fn __pa(x: *const core::ffi::c_void) -> usize { __pa_nodebug(x as usize) }
#[inline] pub unsafe fn __pa32(x: *const core::ffi::c_void) -> usize { __pa_nodebug(x as usize) }
#[inline] pub unsafe fn __va(x: usize) -> *mut core::ffi::c_void { (x.wrapping_add(__identity_base())) as *mut _ }

#[inline] pub const fn phys_to_pfn(phys: usize) -> usize { phys >> PAGE_SHIFT }
#[inline] pub const fn pfn_to_phys(pfn: usize) -> usize { pfn << PAGE_SHIFT }
#[inline] pub unsafe fn pfn_to_virt(pfn: usize) -> *mut core::ffi::c_void { __va(pfn_to_phys(pfn)) }
#[inline] pub unsafe fn virt_to_pfn(kaddr: *const core::ffi::c_void) -> usize { phys_to_pfn(__pa(kaddr)) }
#[inline] pub unsafe fn pfn_to_kaddr(pfn: usize) -> *mut core::ffi::c_void { pfn_to_virt(pfn) }

// virt_to_page(kaddr) expands to pfn_to_page(virt_to_pfn(kaddr)).
// page_to_virt(page) expands to pfn_to_virt(page_to_pfn(page)).
// phys_to_folio(phys) and folio_to_phys(page) retain their source macro forms.

pub const AMODE31_SIZE: usize = 3 * PAGE_SIZE;
pub const KERNEL_IMAGE_SIZE: usize = 512 * 1024 * 1024;
// __NO_KASLR_START_KERNEL = CONFIG_KERNEL_IMAGE_BASE
// __NO_KASLR_END_KERNEL = __NO_KASLR_START_KERNEL + KERNEL_IMAGE_SIZE
pub const TEXT_OFFSET: usize = 0x100000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
