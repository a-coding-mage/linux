/* SPDX-License-Identifier: GPL-2.0 */
/* Translation of asm/nohash/32/pte-8xx.h. */

/* The PowerPC MPC8xx uses hardware-assisted, software tablewalk TLBs.
 * These flags describe the software PTE and PMD layouts used by that TLB. */
pub const _PAGE_PRESENT: u32 = 0x0001;
pub const _PAGE_NO_CACHE: u32 = 0x0002;
pub const _PAGE_SH: u32 = 0x0004;
pub const _PAGE_SPS: u32 = 0x0008;
pub const _PAGE_DIRTY: u32 = 0x0100;
pub const _PAGE_GUARDED: u32 = 0x0010;
pub const _PAGE_ACCESSED: u32 = 0x0020;
pub const _PAGE_EXEC: u32 = 0x0040;
pub const _PAGE_SPECIAL: u32 = 0x0080;
pub const _PAGE_NA: u32 = 0x0200;
pub const _PAGE_RO: u32 = 0x0600;
pub const _PAGE_HUGE: u32 = 0x0800;
pub const _PAGE_NAX: u32 = _PAGE_NA | _PAGE_EXEC;
pub const _PAGE_ROX: u32 = _PAGE_RO | _PAGE_EXEC;
pub const _PAGE_RW: u32 = 0;
pub const _PAGE_RWX: u32 = _PAGE_EXEC;
pub const _PAGE_COHERENT: u32 = 0;
pub const _PAGE_WRITETHRU: u32 = 0;
pub const _PAGE_KERNEL_RO: u32 = _PAGE_SH | _PAGE_RO;
pub const _PAGE_KERNEL_ROX: u32 = _PAGE_SH | _PAGE_RO | _PAGE_EXEC;
pub const _PAGE_KERNEL_RW: u32 = _PAGE_SH | _PAGE_DIRTY;
pub const _PAGE_KERNEL_RWX: u32 = _PAGE_SH | _PAGE_DIRTY | _PAGE_EXEC;

pub const _PMD_PRESENT: u32 = 0x0001;
pub const _PMD_PRESENT_MASK: u32 = _PMD_PRESENT;
pub const _PMD_BAD: u32 = 0x0f90;
pub const _PMD_PAGE_MASK: u32 = 0x000c;
pub const _PMD_PAGE_8M: u32 = 0x000c;
pub const _PMD_PAGE_512K: u32 = 0x0004;
pub const _PMD_ACCESSED: u32 = 0x0020;
pub const _PMD_USER: u32 = 0x0040;
pub const _PTE_NONE_MASK: u32 = 0;

#[cfg(feature = "CONFIG_PPC_16K_PAGES")]
pub const _PAGE_BASE_NC: u32 = _PAGE_PRESENT | _PAGE_ACCESSED | _PAGE_SPS;
#[cfg(not(feature = "CONFIG_PPC_16K_PAGES"))]
pub const _PAGE_BASE_NC: u32 = _PAGE_PRESENT | _PAGE_ACCESSED;
pub const _PAGE_BASE: u32 = _PAGE_BASE_NC;

extern "C" {
    pub fn pte_val(pte: pte_t) -> pte_basic_t;
    pub fn __pte(val: pte_basic_t) -> pte_t;
    pub fn pmd_val(pmd: pmd_t) -> u32;
    pub fn pte_update(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t,
                      clr: usize, set: usize, huge: i32) -> pte_basic_t;
    pub fn flush_tlb_page(vma: *mut vm_area_struct, address: usize);
    pub fn pmd_off(mm: *mut mm_struct, addr: usize) -> *mut pmd_t;
    pub fn pte_offset_kernel(pmd: *mut pmd_t, address: usize) -> *mut pte_t;
}

#[repr(C)] pub struct pte_t { pub pte: pte_basic_t }
#[repr(C)] pub struct pmd_t { pub val: u32 }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { pub vm_mm: *mut mm_struct }
pub type pte_basic_t = usize;

pub const SZ_4K: usize = 4096;
pub const SZ_16K: usize = 16384;
pub const SZ_512K: usize = 512 * 1024;
pub const SZ_4M: usize = 4 * 1024 * 1024;
pub const SZ_8M: usize = 8 * 1024 * 1024;
extern "C" { pub static mmu_virtual_psize: usize; }

#[inline] pub unsafe fn pte_wrprotect(pte: pte_t) -> pte_t { __pte(pte_val(pte) | _PAGE_RO as usize) }
#[inline] pub unsafe fn pte_read(pte: pte_t) -> i32 { ((pte_val(pte) & _PAGE_RO as usize) != _PAGE_NA as usize) as i32 }
#[inline] pub unsafe fn pte_write(pte: pte_t) -> i32 { (!(pte_val(pte) & _PAGE_RO as usize) != 0) as i32 }
#[inline] pub unsafe fn pte_mkwrite_novma(pte: pte_t) -> pte_t { __pte(pte_val(pte) & !(_PAGE_RO as usize)) }
#[inline] pub unsafe fn pte_mkhuge(pte: pte_t) -> pte_t { __pte(pte_val(pte) | (_PAGE_SPS | _PAGE_HUGE) as usize) }

#[inline] pub unsafe fn ptep_set_wrprotect(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t) {
    pte_update(mm, addr, ptep, 0, _PAGE_RO as usize, 0);
}

#[inline] pub unsafe fn __ptep_set_access_flags(vma: *mut vm_area_struct, ptep: *mut pte_t,
                                                entry: pte_t, address: usize, psize: i32) {
    let set = pte_val(entry) & (_PAGE_DIRTY | _PAGE_ACCESSED | _PAGE_EXEC) as usize;
    let clr = !pte_val(entry) & _PAGE_RO as usize;
    let huge = if psize as usize > mmu_virtual_psize { 1 } else { 0 };
    pte_update((*vma).vm_mm, address, ptep, clr, set, huge);
    flush_tlb_page(vma, address);
}

#[inline] pub unsafe fn __pte_leaf_size(pmd: pmd_t, pte: pte_t) -> usize {
    let val = pte_val(pte);
    if pmd_val(pmd) & _PMD_PAGE_8M != 0 { SZ_8M }
    else if val & _PAGE_HUGE as usize != 0 { SZ_512K }
    else if val & _PAGE_SPS as usize != 0 { SZ_16K } else { SZ_4K }
}

#[inline] pub unsafe fn ptep_is_8m_pmdp(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t) -> bool {
    ptep as *mut pmd_t == pmd_off(mm, addr & !(SZ_8M - 1))
}

#[inline] pub unsafe fn number_of_cells_per_pte(pmd: *mut pmd_t, val: pte_basic_t, huge: i32) -> usize {
    if huge == 0 { PAGE_SIZE / SZ_4K }
    else if pmd_val(*pmd) & _PMD_PAGE_MASK != _PMD_PAGE_8M { SZ_4M / SZ_4K }
    else if cfg!(feature = "CONFIG_PPC_4K_PAGES") && val & _PAGE_HUGE as usize == 0 { SZ_16K / SZ_4K }
    else { SZ_512K / SZ_4K }
}

pub const PAGE_SIZE: usize = 4096;

#[inline] pub unsafe fn __pte_update(mm: *mut mm_struct, addr: usize, p: *mut pte_t,
                                     clr: usize, set: usize, huge: i32) -> pte_basic_t {
    let mut entry = p as *mut pte_basic_t;
    let old = pte_val(*p);
    let mut new = (old & !clr) | set;
    let num = number_of_cells_per_pte(pmd_off(mm, addr), new, huge);
    let mut i = 0;
    while i < num {
        *entry = new; entry = entry.add(1);
        if cfg!(feature = "CONFIG_PPC_16K_PAGES") { *entry = new; entry = entry.add(1); *entry = new; entry = entry.add(1); *entry = new; entry = entry.add(1); }
        i += PAGE_SIZE / SZ_4K; new = new.wrapping_add(PAGE_SIZE);
    }
    old
}

#[inline] pub unsafe fn pte_update_local(mm: *mut mm_struct, addr: usize, ptep: *mut pte_t,
                                        clr: usize, set: usize, huge: i32) -> pte_basic_t {
    let old;
    if huge != 0 && ptep_is_8m_pmdp(mm, addr, ptep) {
        let pmdp = ptep as *mut pmd_t;
        old = __pte_update(mm, addr, pte_offset_kernel(pmdp, 0), clr, set, huge);
        __pte_update(mm, addr, pte_offset_kernel(pmdp.add(1), 0), clr, set, huge);
    } else { old = __pte_update(mm, addr, ptep, clr, set, huge); }
    old
}

#[cfg(feature = "CONFIG_PPC_16K_PAGES")]
#[inline] pub unsafe fn ptep_get(ptep: *mut pte_t) -> pte_t {
    let val = core::ptr::read_volatile(&(*ptep).pte);
    pte_t { pte: val }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
