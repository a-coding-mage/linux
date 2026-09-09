// SPDX-License-Identifier: GPL-2.0
/*
 * Virtual DMA allocation
 *
 * (C) 1999 Thomas Bogendoerfer (tsbogend@alpha.franken.de)
 *
 * 11/26/2000 -- disabled the existing code because it didn't work for
 * me in 2.4.  Replaced with a significantly more primitive version
 * similar to the sun3 code.  the old functionality was probably more
 * desirable, but....   -- Sam Creasey (sammy@oh.verio.com)
 */

// Dependencies are supplied by the surrounding kernel translation.

const IOMMU_ADDR_MASK: usize = 0x03ffe000;
const IOMMU_CACHE_INHIBIT: usize = 0x00000040;
const IOMMU_FULL_BLOCK: usize = 0x00000020;
const IOMMU_MODIFIED: usize = 0x00000010;
const IOMMU_USED: usize = 0x00000008;
const IOMMU_WRITE_PROTECT: usize = 0x00000004;
const IOMMU_DT_MASK: usize = 0x00000003;
const IOMMU_DT_INVALID: usize = 0x00000000;
const IOMMU_DT_VALID: usize = 0x00000001;
const IOMMU_DT_BAD: usize = 0x00000002;

static mut iommu_pte: *mut usize = SUN3X_IOMMU as *mut usize;

#[inline]
unsafe fn dvma_entry_paddr(index: usize) -> usize {
    core::ptr::read_volatile(iommu_pte.add(index)) & IOMMU_ADDR_MASK
}

#[inline]
fn dvma_entry_vaddr(index: usize, paddr: usize) -> usize {
    (index << DVMA_PAGE_SHIFT) | (paddr & (DVMA_PAGE_SIZE - 1))
}

#[inline]
unsafe fn dvma_entry_set(index: usize, addr: usize) {
    core::ptr::write_volatile(
        iommu_pte.add(index),
        (addr & IOMMU_ADDR_MASK) | IOMMU_DT_VALID,
    );
}

#[inline]
unsafe fn dvma_entry_clr(index: usize) {
    core::ptr::write_volatile(iommu_pte.add(index), IOMMU_DT_INVALID);
}

#[inline]
fn dvma_entry_hash(addr: usize) -> usize {
    (addr >> DVMA_PAGE_SHIFT)
        ^ ((addr & 0x03c00000) >> (DVMA_PAGE_SHIFT + 4))
}

#[cfg(feature = "DEBUG")]
/* code to print out a dvma mapping for debugging purposes */
unsafe fn dvma_print(dvma_addr: usize) {
    let index = dvma_addr >> DVMA_PAGE_SHIFT;
    pr_info!("idx {:lx} dvma_addr {:08lx} paddr {:08lx}\n", index, dvma_addr,
             dvma_entry_paddr(index));
}

/* create a virtual mapping for a page assigned within the IOMMU
   so that the cpu can reach it easily */
#[inline]
unsafe fn dvma_map_cpu(mut kaddr: usize, mut vaddr: usize, len: i32) -> i32 {
    let mut ret: i32 = 0;

    kaddr &= PAGE_MASK;
    vaddr &= PAGE_MASK;

    let end = PAGE_ALIGN(vaddr + len as usize);

    pr_debug!("dvma: mapping kern {:08lx} to virt {:08lx}\n", kaddr, vaddr);
    let pgd = pgd_offset_k(vaddr);
    let p4d = p4d_offset(pgd, vaddr);
    let pud = pud_offset(p4d, vaddr);

    while vaddr < end {
        let pmd = match pmd_alloc(&mut init_mm, pud, vaddr) {
            Some(value) => value,
            None => { ret = -ENOMEM; break; }
        };

        let end2 = if (end & PGDIR_MASK) > (vaddr & PGDIR_MASK) {
            (vaddr + (PGDIR_SIZE - 1)) & PGDIR_MASK
        } else { end };

        while vaddr < end2 {
            let mut pte = match pte_alloc_kernel(pmd, vaddr) {
                Some(value) => value,
                None => { ret = -ENOMEM; return ret; }
            };

            let end3 = if (end2 & PMD_MASK) > (vaddr & PMD_MASK) {
                (vaddr + (PMD_SIZE - 1)) & PMD_MASK
            } else { end2 };

            while vaddr < end3 {
                pr_debug!("mapping {:08lx} phys to {:08lx}\n", __pa(kaddr), vaddr);
                set_pte(pte, pfn_pte(virt_to_pfn(kaddr as *mut core::ffi::c_void), PAGE_KERNEL));
                pte = pte.add(1);
                kaddr += PAGE_SIZE;
                vaddr += PAGE_SIZE;
            }
        }
    }

    flush_tlb_all();
    ret
}

unsafe fn dvma_map_iommu(mut kaddr: usize, baddr: usize, len: i32) -> i32 {
    let mut index = baddr >> DVMA_PAGE_SHIFT;
    let mut end = (baddr + len as usize) >> DVMA_PAGE_SHIFT;

    if (len as usize & !DVMA_PAGE_MASK) != 0 { end += 1; }

    while index < end {
        dvma_entry_set(index, __pa(kaddr));
        let pte = iommu_pte.add(index);
        core::ptr::write_volatile(pte, core::ptr::read_volatile(pte) | IOMMU_FULL_BLOCK);
        kaddr += DVMA_PAGE_SIZE;
        index += 1;
    }

    #[cfg(feature = "DEBUG")]
    {
        let mut index = baddr >> DVMA_PAGE_SHIFT;
        while index < end { dvma_print(index << DVMA_PAGE_SHIFT); index += 1; }
    }
    0
}

unsafe fn dvma_unmap_iommu(baddr: usize, len: i32) {
    let mut index = (baddr >> DVMA_PAGE_SHIFT) as i32;
    let end = (DVMA_PAGE_ALIGN(baddr + len as usize) >> DVMA_PAGE_SHIFT) as i32;

    while index < end {
        pr_debug!("freeing bus mapping {:08x}\n", (index as usize) << DVMA_PAGE_SHIFT);
        dvma_entry_clr(index as usize);
        index += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
