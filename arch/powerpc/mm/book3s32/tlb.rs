// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file contains the routines for TLB flushing.
 * On machines where the MMU uses a hash table to store virtual to
 * physical translations, these routines flush entries from the hash
 * table also.
 *  -- paulus
 *
 *  Derived from arch/ppc/mm/init.c:
 *    Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *
 *  Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
 *  and Cort Dougan (PReP) (cort@cs.nmt.edu)
 *    Copyright (C) 1996 Paul Mackerras
 *
 *  Derived from "arch/i386/mm/init.c"
 *    Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
 */

/* Dependencies supplied by the surrounding kernel translation unit. */

/*
 * TLB flushing:
 *
 *  - flush_tlb_mm(mm) flushes the specified mm context TLB's
 *  - flush_tlb_page(vma, vmaddr) flushes one page
 *  - flush_tlb_range(vma, start, end) flushes a range of pages
 *  - flush_tlb_kernel_range(start, end) flushes kernel pages
 *
 * since the hardware hash table functions as an extension of the
 * tlb as far as the linux tables are concerned, flush it too.
 *    -- Cort
 */

/*
 * For each address in the range, find the pte for the address
 * and check _PAGE_HASHPTE bit; if it is set, find and destroy
 * the corresponding HPTE.
 */
pub unsafe fn hash__flush_range(
    mm: *mut mm_struct,
    mut start: c_ulong,
    mut end: c_ulong,
) {
    let mut pmd: *mut pmd_t;
    let mut pmd_end: c_ulong;
    let count: c_int;
    let ctx: c_uint = (*mm).context.id;

    start &= PAGE_MASK;
    if start >= end {
        return;
    }
    end = (end.wrapping_sub(1)) | !PAGE_MASK;
    pmd = pmd_off(mm, start);
    loop {
        pmd_end = ((start.wrapping_add(PGDIR_SIZE)) & PGDIR_MASK).wrapping_sub(1);
        if pmd_end > end {
            pmd_end = end;
        }
        if !pmd_none(*pmd) {
            count = (((pmd_end.wrapping_sub(start)) >> PAGE_SHIFT) + 1) as c_int;
            flush_hash_pages(ctx, start, pmd_val(*pmd), count);
        }
        if pmd_end == end {
            break;
        }
        start = pmd_end.wrapping_add(1);
        pmd = pmd.add(1);
    }
}

/* Flush all the (user) entries for the address space described by mm. */
pub unsafe fn hash__flush_tlb_mm(mm: *mut mm_struct) {
    let mut vmi = VmaIterator::new(mm, 0);

    /*
     * It is safe to iterate the vmas when called from dup_mmap,
     * holding mmap_lock.  It would also be safe from unmap_region
     * or exit_mmap, but not from vmtruncate on SMP - but it seems
     * dup_mmap is the only SMP case which gets here.
     */
    while let Some(mp) = vmi.next() {
        hash__flush_range((*mp).vm_mm, (*mp).vm_start, (*mp).vm_end);
    }
}

pub unsafe fn hash__flush_tlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong) {
    let mm: *mut mm_struct;
    let pmd: *mut pmd_t;

    mm = if vmaddr < TASK_SIZE { (*vma).vm_mm } else { &raw mut init_mm };
    pmd = pmd_off(mm, vmaddr);
    if !pmd_none(*pmd) {
        flush_hash_pages((*mm).context.id, vmaddr, pmd_val(*pmd), 1);
    }
}

pub unsafe fn hash__flush_gather(tlb: *mut mmu_gather) {
    if (*tlb).fullmm || (*tlb).need_flush_all {
        hash__flush_tlb_mm((*tlb).mm);
    } else {
        hash__flush_range((*tlb).mm, (*tlb).start, (*tlb).end);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
