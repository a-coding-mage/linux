/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel.com})
 * Copyright 2003 PathScale, Inc.
 * Derived from include/asm-i386/pgtable.h
 */

// C header guard: __UM_PGTABLE_H
// Dependencies: asm/page.h, linux/mm_types.h

pub const _PAGE_PRESENT: usize = 0x001;
pub const _PAGE_NEEDSYNC: usize = 0x002;
pub const _PAGE_RW: usize = 0x020;
pub const _PAGE_USER: usize = 0x040;
pub const _PAGE_ACCESSED: usize = 0x080;
pub const _PAGE_DIRTY: usize = 0x100;
/* If _PAGE_PRESENT is clear, we use these. */
pub const _PAGE_PROTNONE: usize = 0x010;
/* We borrow bit 10 to store the exclusive marker in swap PTEs. */
pub const _PAGE_SWP_EXCLUSIVE: usize = 0x400;

// CONFIG_PGTABLE_LEVELS == 4 includes asm/pgtable-4level.h;
// CONFIG_PGTABLE_LEVELS == 2 includes asm/pgtable-2level.h.
// Other values are unsupported.

extern "C" {
    pub static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD];
    pub fn virt_to_pte(mm: *mut mm_struct, addr: c_ulong) -> *mut pte_t;
}

/* Just any arbitrary offset to the start of the vmalloc VM area. */
// COMPILE_OFFSETS may omit the as-layout.h dependency (for high_physmem).

pub const VMALLOC_OFFSET: usize = __va_space;
pub const VMALLOC_START: usize = (high_physmem + VMALLOC_OFFSET) & !(VMALLOC_OFFSET - 1);
pub const VMALLOC_END: usize = TASK_SIZE - 2 * PAGE_SIZE;
pub const MODULES_VADDR: usize = VMALLOC_START;
pub const MODULES_END: usize = VMALLOC_END;

pub const _PAGE_TABLE: usize = _PAGE_PRESENT | _PAGE_RW | _PAGE_USER | _PAGE_ACCESSED | _PAGE_DIRTY;
pub const _KERNPG_TABLE: usize = _PAGE_PRESENT | _PAGE_RW | _PAGE_ACCESSED | _PAGE_DIRTY;
pub const _PAGE_CHG_MASK: usize = PAGE_MASK | _PAGE_ACCESSED | _PAGE_DIRTY;
pub const __PAGE_KERNEL_EXEC: usize = _PAGE_PRESENT | _PAGE_RW | _PAGE_DIRTY | _PAGE_ACCESSED;

// The i386 cannot do page protection for execute, and write implies read.

#[inline]
pub unsafe fn pte_clear(_mm: *mut mm_struct, _addr: c_ulong, xp: *mut pte_t) {
    pte_set_val(&mut *xp, 0 as phys_t, __pgprot(_PAGE_NEEDSYNC));
}

#[inline] pub unsafe fn pmd_none(x: pmd_t) -> bool { (pmd_val(x) & !_PAGE_NEEDSYNC) == 0 }
#[inline] pub unsafe fn pmd_bad(x: pmd_t) -> bool { (pmd_val(x) & (!PAGE_MASK & !_PAGE_USER)) != _KERNPG_TABLE }
#[inline] pub unsafe fn pmd_present(x: pmd_t) -> usize { pmd_val(x) & _PAGE_PRESENT }
#[inline] pub unsafe fn pmd_clear(xp: *mut pmd_t) { pmd_val_mut(&mut *xp).write(_PAGE_NEEDSYNC); }
#[inline] pub unsafe fn pmd_needsync(x: pmd_t) -> usize { pmd_val(x) & _PAGE_NEEDSYNC }
#[inline] pub unsafe fn pmd_mkuptodate(x: &mut pmd_t) { *pmd_val_mut(x) &= !_PAGE_NEEDSYNC; }
#[inline] pub unsafe fn pud_needsync(x: pud_t) -> usize { pud_val(x) & _PAGE_NEEDSYNC }
#[inline] pub unsafe fn pud_mkuptodate(x: &mut pud_t) { *pud_val_mut(x) &= !_PAGE_NEEDSYNC; }
#[inline] pub unsafe fn p4d_needsync(x: p4d_t) -> usize { p4d_val(x) & _PAGE_NEEDSYNC }
#[inline] pub unsafe fn p4d_mkuptodate(x: &mut p4d_t) { *p4d_val_mut(x) &= !_PAGE_NEEDSYNC; }

#[inline] pub unsafe fn pmd_pfn(pmd: pmd_t) -> usize { pmd_val(pmd) >> PAGE_SHIFT }
#[inline] pub unsafe fn pmd_page(pmd: pmd_t) -> *mut page { phys_to_page(pmd_val(pmd) & PAGE_MASK) }
#[inline] pub unsafe fn pte_page(x: pte_t) -> *mut page { pfn_to_page(pte_pfn(x)) }
#[inline] pub unsafe fn pte_present(x: pte_t) -> bool { pte_get_bits(x, _PAGE_PRESENT | _PAGE_PROTNONE) }

#[inline] pub unsafe fn pte_none(pte: pte_t) -> i32 { pte_is_zero(pte) as i32 }
#[inline] pub unsafe fn pte_read(pte: pte_t) -> i32 { (!pte_get_bits(pte, _PAGE_PROTNONE)) as i32 }
#[inline] pub unsafe fn pte_exec(pte: pte_t) -> i32 { (!pte_get_bits(pte, _PAGE_PROTNONE)) as i32 }
#[inline] pub unsafe fn pte_write(pte: pte_t) -> i32 { (pte_get_bits(pte, _PAGE_RW) && !pte_get_bits(pte, _PAGE_PROTNONE)) as i32 }
#[inline] pub unsafe fn pte_dirty(pte: pte_t) -> usize { pte_get_bits(pte, _PAGE_DIRTY) }
#[inline] pub unsafe fn pte_young(pte: pte_t) -> usize { pte_get_bits(pte, _PAGE_ACCESSED) }
#[inline] pub unsafe fn pte_needsync(pte: pte_t) -> usize { pte_get_bits(pte, _PAGE_NEEDSYNC) }

#[inline] pub unsafe fn pte_mkclean(mut pte: pte_t) -> pte_t { pte_clear_bits(&mut pte, _PAGE_DIRTY); pte }
#[inline] pub unsafe fn pte_mkold(mut pte: pte_t) -> pte_t { pte_clear_bits(&mut pte, _PAGE_ACCESSED); pte }
#[inline] pub unsafe fn pte_wrprotect(mut pte: pte_t) -> pte_t { pte_clear_bits(&mut pte, _PAGE_RW); pte }
#[inline] pub unsafe fn pte_mkread(mut pte: pte_t) -> pte_t { pte_set_bits(&mut pte, _PAGE_USER); pte }
#[inline] pub unsafe fn pte_mkdirty(mut pte: pte_t) -> pte_t { pte_set_bits(&mut pte, _PAGE_DIRTY); pte }
#[inline] pub unsafe fn pte_mkyoung(mut pte: pte_t) -> pte_t { pte_set_bits(&mut pte, _PAGE_ACCESSED); pte }
#[inline] pub unsafe fn pte_mkwrite_novma(mut pte: pte_t) -> pte_t { pte_set_bits(&mut pte, _PAGE_RW); pte }
#[inline] pub unsafe fn pte_mkuptodate(mut pte: pte_t) -> pte_t { pte_clear_bits(&mut pte, _PAGE_NEEDSYNC); pte }
#[inline] pub unsafe fn pte_mkneedsync(mut pte: pte_t) -> pte_t { pte_set_bits(&mut pte, _PAGE_NEEDSYNC); pte }

#[inline] pub unsafe fn set_pte(pteptr: *mut pte_t, pteval: pte_t) {
    pte_copy(&mut *pteptr, pteval);
    *pteptr = pte_mkneedsync(*pteptr);
}

pub const PFN_PTE_SHIFT: usize = PAGE_SHIFT;

#[inline] pub unsafe fn um_tlb_mark_sync(mm: *mut mm_struct, start: c_ulong, end: c_ulong) {
    // guard(spinlock_irqsave)(&mm->context.sync_tlb_lock);
    if !(*mm).context.sync_tlb_range_to {
        (*mm).context.sync_tlb_range_from = start;
        (*mm).context.sync_tlb_range_to = end;
    } else {
        if start < (*mm).context.sync_tlb_range_from { (*mm).context.sync_tlb_range_from = start; }
        if end > (*mm).context.sync_tlb_range_to { (*mm).context.sync_tlb_range_to = end; }
    }
}

#[inline] pub unsafe fn set_ptes(mm: *mut mm_struct, addr: c_ulong, mut ptep: *mut pte_t, mut pte: pte_t, mut nr: i32) {
    let length = nr as usize * PAGE_SIZE;
    loop {
        set_pte(ptep, pte);
        nr -= 1;
        if nr == 0 { break; }
        ptep = ptep.add(1);
        pte = __pte(pte_val(pte) + (nr as usize * (1usize << PFN_PTE_SHIFT)));
    }
    um_tlb_mark_sync(mm, addr, addr + length);
}

#[inline] pub unsafe fn pte_same(pte_a: pte_t, pte_b: pte_t) -> bool { (pte_val(pte_a) ^ pte_val(pte_b)) & !_PAGE_NEEDSYNC == 0 }
#[inline] pub unsafe fn __virt_to_page(virt: c_ulong) -> *mut page { phys_to_page(__pa(virt)) }
#[inline] pub unsafe fn virt_to_page(addr: c_ulong) -> *mut page { __virt_to_page(addr) }

#[inline] pub unsafe fn pfn_pte(pfn: c_ulong, pgprot: pgprot_t) -> pte_t {
    let mut pte = core::mem::zeroed();
    pte_set_val(&mut pte, pfn_to_phys(pfn), pgprot);
    pte
}
#[inline] pub unsafe fn pte_modify(mut pte: pte_t, newprot: pgprot_t) -> pte_t {
    pte_set_val(&mut pte, pte_val(pte) & _PAGE_CHG_MASK, newprot); pte
}

#[inline] pub unsafe fn pmd_page_vaddr(pmd: pmd_t) -> c_ulong { __va(pmd_val(pmd) & PAGE_MASK) }
#[inline] pub unsafe fn update_mmu_cache(_vma: *mut vm_area_struct, _address: c_ulong, _ptep: *mut pte_t) {}
#[inline] pub unsafe fn update_mmu_cache_range(_vmf: *mut vm_fault, _vma: *mut vm_area_struct, _address: c_ulong, _ptep: *mut pte_t, _nr: i32) {}

#[inline] pub unsafe fn __swp_type(x: swp_entry_t) -> usize { (x.val >> 5) & 0x1f }
#[inline] pub unsafe fn __swp_offset(x: swp_entry_t) -> usize { x.val >> 11 }
#[inline] pub unsafe fn __swp_entry(type_: usize, offset: usize) -> swp_entry_t { swp_entry_t { val: ((type_ & 0x1f) << 5) | (offset << 11) } }
#[inline] pub unsafe fn __pte_to_swp_entry(pte: pte_t) -> swp_entry_t { swp_entry_t { val: pte_val(pte_mkuptodate(pte)) } }
#[inline] pub unsafe fn __swp_entry_to_pte(x: swp_entry_t) -> pte_t { __pte(x.val) }

#[inline] pub unsafe fn pte_swp_exclusive(pte: pte_t) -> bool { pte_get_bits(pte, _PAGE_SWP_EXCLUSIVE) }
#[inline] pub unsafe fn pte_swp_mkexclusive(mut pte: pte_t) -> pte_t { pte_set_bits(&mut pte, _PAGE_SWP_EXCLUSIVE); pte }
#[inline] pub unsafe fn pte_swp_clear_exclusive(mut pte: pte_t) -> pte_t { pte_clear_bits(&mut pte, _PAGE_SWP_EXCLUSIVE); pte }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
