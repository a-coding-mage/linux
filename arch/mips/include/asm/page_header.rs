/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994 - 1999, 2000, 03 Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 */

// Dependencies supplied by the surrounding kernel translation are referenced
// here but are not implemented in this header translation.

pub unsafe fn page_size_ftlb(mmuextdef: u32) -> u32 {
    match mmuextdef {
        MIPS_CONF4_MMUEXTDEF_FTLBSIZEEXT => {
            if PAGE_SIZE == (1u32 << 30) { return 5; }
            if PAGE_SIZE == (1u64 << 32) as u32 { return 6; }
            if PAGE_SIZE > (256u32 << 10) { return 7; } // reserved
            // fallthrough
            (PAGE_SHIFT - 10) / 2
        }
        MIPS_CONF4_MMUEXTDEF_VTLBSIZEEXT => (PAGE_SHIFT - 10) / 2,
        _ => panic!("Invalid FTLB configuration with Conf4_mmuextdef=%d value\n", mmuextdef >> 14),
    }
}

// CONFIG_MIPS_HUGE_TLB_SUPPORT selects these definitions; otherwise each
// expression is the BUILD_BUG()/0 form from the original header.
#[cfg(CONFIG_MIPS_HUGE_TLB_SUPPORT)]
pub const HPAGE_SHIFT: u32 = PAGE_SHIFT + PAGE_SHIFT - 3;
#[cfg(CONFIG_MIPS_HUGE_TLB_SUPPORT)]
pub const HPAGE_SIZE: usize = 1usize << HPAGE_SHIFT;
#[cfg(CONFIG_MIPS_HUGE_TLB_SUPPORT)]
pub const HPAGE_MASK: usize = !(HPAGE_SIZE - 1);
#[cfg(CONFIG_MIPS_HUGE_TLB_SUPPORT)]
pub const HUGETLB_PAGE_ORDER: u32 = HPAGE_SHIFT - PAGE_SHIFT;

extern "C" {
    pub fn build_clear_page();
    pub fn build_copy_page();
    pub fn clear_page(page: *mut core::ffi::c_void);
    pub fn copy_page(to: *mut core::ffi::c_void, from: *mut core::ffi::c_void);
    pub static mut shm_align_mask: libc::c_ulong;
}

#[cfg(CONFIG_MIPS_AUTO_PFN_OFFSET)]
extern "C" { pub static mut ARCH_PFN_OFFSET: libc::c_ulong; }

pub unsafe fn pages_do_alias(addr1: libc::c_ulong, addr2: libc::c_ulong) -> libc::c_ulong {
    (addr1 ^ addr2) & shm_align_mask
}

#[repr(C)]
pub struct page;

pub unsafe fn clear_user_page(addr: *mut core::ffi::c_void, vaddr: libc::c_ulong, _page: *mut page) {
    unsafe extern "C" { static mut flush_data_cache_page: Option<unsafe extern "C" fn(libc::c_ulong)>; }
    clear_page(addr);
    if pages_do_alias(addr as libc::c_ulong, vaddr & PAGE_MASK) != 0 {
        if let Some(f) = flush_data_cache_page { f(addr as libc::c_ulong); }
    }
}

pub struct vm_area_struct;
extern "C" { pub fn copy_user_highpage(to: *mut page, from: *mut page, vaddr: libc::c_ulong, vma: *mut vm_area_struct); }

#[repr(C)]
pub struct pte_t { pub pte: libc::c_ulong }
#[repr(C)]
pub struct pgd_t { pub pgd: libc::c_ulong }
#[repr(C)]
pub struct pgprot_t { pub pgprot: libc::c_ulong }
pub type pgtable_t = *mut page;

pub unsafe fn pte_val(x: pte_t) -> libc::c_ulong { x.pte }
pub fn __pte(x: libc::c_ulong) -> pte_t { pte_t { pte: x } }
pub unsafe fn pgd_val(x: pgd_t) -> libc::c_ulong { x.pgd }
pub fn __pgd(x: libc::c_ulong) -> pgd_t { pgd_t { pgd: x } }
pub unsafe fn pgprot_val(x: pgprot_t) -> libc::c_ulong { x.pgprot }
pub fn __pgprot(x: libc::c_ulong) -> pgprot_t { pgprot_t { pgprot: x } }
pub unsafe fn pte_pgprot(x: pte_t) -> pgprot_t { __pgprot(pte_val(x) & !_PFN_MASK) }
pub unsafe fn ptep_buddy(x: *mut pte_t) -> *mut pte_t { ((x as usize) ^ core::mem::size_of::<pte_t>()) as *mut pte_t }

pub unsafe fn ___pa(x: libc::c_ulong) -> libc::c_ulong {
    if IS_ENABLED_CONFIG_64BIT { return if x < CKSEG0 { XPHYSADDR(x) } else { CPHYSADDR(x) }; }
    if !IS_ENABLED_CONFIG_EVA { return CPHYSADDR(x); }
    x - PAGE_OFFSET + PHYS_OFFSET
}

pub unsafe fn __pa<T>(x: *const T) -> libc::c_ulong { ___pa(x as libc::c_ulong) }
pub unsafe fn __va(x: libc::c_ulong) -> *mut core::ffi::c_void { (x + PAGE_OFFSET - PHYS_OFFSET) as *mut core::ffi::c_void }
pub unsafe fn __pa_symbol_nodebug<T>(x: *const T) -> libc::c_ulong { __pa(x) }
pub unsafe fn pfn_to_kaddr(pfn: libc::c_ulong) -> *mut core::ffi::c_void { __va(pfn << PAGE_SHIFT) }
pub unsafe fn virt_to_pfn(kaddr: libc::c_ulong) -> libc::c_ulong { PFN_DOWN(virt_to_phys(kaddr as *mut core::ffi::c_void)) }
pub unsafe fn virt_to_page(kaddr: libc::c_ulong) -> *mut page { pfn_to_page(virt_to_pfn(kaddr)) }
extern "C" { pub fn __virt_addr_valid(kaddr: *const core::ffi::c_void) -> bool; }
pub const VMA_DATA_DEFAULT_FLAGS: libc::c_ulong = VMA_DATA_FLAGS_TSK_EXEC;
extern "C" { pub static mut __kaslr_offset: libc::c_ulong; }
pub unsafe fn kaslr_offset() -> libc::c_ulong { __kaslr_offset }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
