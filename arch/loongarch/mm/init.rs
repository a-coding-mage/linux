// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// C header dependencies are supplied by the surrounding kernel translation.

pub unsafe fn page_is_ram(pfn: ::core::ffi::c_ulong) -> bool {
    let addr = PFN_PHYS(pfn);
    memblock_is_memory(addr) && !memblock_is_reserved(addr)
}

pub unsafe fn arch_zone_limits_init(max_zone_pfns: *mut ::core::ffi::c_ulong) {
    #[cfg(CONFIG_ZONE_DMA32)]
    {
        *max_zone_pfns.add(ZONE_DMA32 as usize) = MAX_DMA32_PFN;
    }
    *max_zone_pfns.add(ZONE_NORMAL as usize) = max_low_pfn;
    #[cfg(CONFIG_HIGHMEM)]
    {
        *max_zone_pfns.add(ZONE_HIGHMEM as usize) = max_pfn;
    }
}

pub unsafe fn free_initmem() {
    free_initmem_default(POISON_FREE_INITMEM);
}

#[cfg(CONFIG_HIGHMEM)]
pub unsafe fn fixrange_init(
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
    pgd_base: *mut pgd_t,
) {
    let mut vaddr = start;
    let mut i = pgd_index(vaddr);
    let mut j = pud_index(vaddr);
    let mut k = pmd_index(vaddr);
    let mut pgd = pgd_base.add(i as usize);
    let ptrs_per_pgd = core::cmp::min(1usize << (BITS_PER_LONG - PGDIR_SHIFT), PTRS_PER_PGD);

    while (i < ptrs_per_pgd) && (vaddr < end) {
        let mut pud = pgd.cast::<pud_t>();
        while (j < PTRS_PER_PUD) && (vaddr < end) {
            let mut pmd = pud.cast::<pmd_t>();
            while (k < PTRS_PER_PMD) && (vaddr < end) {
                if pmd_none(*pmd) {
                    let pte = memblock_alloc_low(PAGE_SIZE, PAGE_SIZE).cast::<pte_t>();
                    if pte.is_null() {
                        panic!("{}: Failed to allocate {} bytes align={:x}\n", "fixrange_init", PAGE_SIZE, PAGE_SIZE);
                    }
                    kernel_pte_init(pte);
                    set_pmd(pmd, __pmd(pte as ::core::ffi::c_ulong));
                    BUG_ON(pte != pte_offset_kernel(pmd, 0));
                }
                pmd = pmd.add(1);
                k += 1;
                vaddr += PMD_SIZE;
            }
            pud = pud.add(1);
            k = 0;
            j += 1;
        }
        pgd = pgd.add(1);
        j = 0;
        i += 1;
    }
}

#[cfg(CONFIG_MEMORY_HOTPLUG)]
pub unsafe fn arch_add_memory(nid: ::core::ffi::c_int, start: u64, size: u64, params: *mut mhp_params) -> ::core::ffi::c_int {
    let start_pfn = start >> PAGE_SHIFT;
    let nr_pages = size >> PAGE_SHIFT;
    let ret = __add_pages(nid, start_pfn, nr_pages, params);
    if ret != 0 {
        pr_warn!("{}: Problem encountered in __add_pages() as ret={}\n", "arch_add_memory", ret);
    }
    ret
}

#[cfg(CONFIG_MEMORY_HOTPLUG)]
pub unsafe fn arch_remove_memory(start: u64, size: u64, altmap: *mut vmem_altmap, pgmap: *mut dev_pagemap) {
    let start_pfn = start >> PAGE_SHIFT;
    let nr_pages = size >> PAGE_SHIFT;
    __remove_pages(start_pfn, nr_pages, altmap, pgmap);
}

#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
pub unsafe fn vmemmap_set_pmd(pmd: *mut pmd_t, p: *mut ::core::ffi::c_void, _node: ::core::ffi::c_int, addr: ::core::ffi::c_ulong, _next: ::core::ffi::c_ulong) {
    let mut entry = pfn_pmd(virt_to_pfn(p), PAGE_KERNEL);
    pmd_val(entry) |= _PAGE_HUGE | _PAGE_HGLOBAL;
    set_pmd_at(&init_mm, addr, pmd, entry);
}

#[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
pub unsafe fn vmemmap_populate(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong, node: ::core::ffi::c_int, _altmap: *mut vmem_altmap) -> ::core::ffi::c_int {
    #[cfg(CONFIG_PGTABLE_LEVELS_2)]
    { vmemmap_populate_basepages(start, end, node, core::ptr::null_mut()) }
    #[cfg(not(CONFIG_PGTABLE_LEVELS_2))]
    { vmemmap_populate_hugepages(start, end, node, core::ptr::null_mut()) }
}

#[cfg(all(CONFIG_SPARSEMEM_VMEMMAP, CONFIG_MEMORY_HOTPLUG))]
pub unsafe fn vmemmap_free(_start: ::core::ffi::c_ulong, _end: ::core::ffi::c_ulong, _altmap: *mut vmem_altmap) {}

pub unsafe fn populate_kernel_pte(addr: ::core::ffi::c_ulong) -> *mut pte_t {
    let pgd = pgd_offset_k(addr);
    let p4d = p4d_offset(pgd, addr);
    let pud;
    let pmd;
    if p4d_none(p4dp_get(p4d)) {
        pud = memblock_alloc_or_panic(PAGE_SIZE, PAGE_SIZE).cast::<pud_t>();
        p4d_populate(&init_mm, p4d, pud);
        #[cfg(not(__PAGETABLE_PUD_FOLDED))]
        pud_init(pud);
    }
    pud = pud_offset(p4d, addr);
    if pud_none(pudp_get(pud)) {
        pmd = memblock_alloc_or_panic(PAGE_SIZE, PAGE_SIZE).cast::<pmd_t>();
        pud_populate(&init_mm, pud, pmd);
        #[cfg(not(__PAGETABLE_PMD_FOLDED))]
        pmd_init(pmd);
    }
    pmd = pmd_offset(pud, addr);
    if !pmd_present(pmdp_get(pmd)) {
        let pte = memblock_alloc_or_panic(PAGE_SIZE, PAGE_SIZE).cast::<pte_t>();
        pmd_populate_kernel(&init_mm, pmd, pte);
        kernel_pte_init(pte);
    }
    pte_offset_kernel(pmd, addr)
}

pub unsafe fn __set_fixmap(idx: fixed_addresses, phys: phys_addr_t, flags: pgprot_t) {
    let addr = __fix_to_virt(idx);
    BUG_ON(idx <= FIX_HOLE || idx >= __end_of_fixed_addresses);
    let ptep = populate_kernel_pte(addr);
    if !pte_none(ptep_get(ptep)) {
        pte_ERROR(*ptep);
        return;
    }
    if pgprot_val(flags) != 0 {
        set_pte(ptep, pfn_pte(phys >> PAGE_SHIFT, flags));
    } else {
        pte_clear(&init_mm, addr, ptep);
        flush_tlb_kernel_range(addr, addr + PAGE_SIZE);
    }
}

/* Align swapper_pg_dir in to 64K, allows its address to be loaded
 * with a single LUI instruction in the TLB handlers.  If we used
 * __aligned(64K), its size would get rounded up to the alignment
 * size, and waste space.  So we place it in its own section and align
 * it in the linker script.
 */
#[link_section = ".bss..swapper_pg_dir"]
pub static mut swapper_pg_dir: [pgd_t; _PTRS_PER_PGD] = [pgd_t::default(); _PTRS_PER_PGD];
pub static mut invalid_pg_dir: [pgd_t; _PTRS_PER_PGD] = [pgd_t::default(); _PTRS_PER_PGD];
#[cfg(not(__PAGETABLE_PUD_FOLDED))]
pub static mut invalid_pud_table: [pud_t; PTRS_PER_PUD] = [pud_t::default(); PTRS_PER_PUD];
#[cfg(not(__PAGETABLE_PMD_FOLDED))]
pub static mut invalid_pmd_table: [pmd_t; PTRS_PER_PMD] = [pmd_t::default(); PTRS_PER_PMD];
pub static mut invalid_pte_table: [pte_t; PTRS_PER_PTE] = [pte_t::default(); PTRS_PER_PTE];

#[cfg(all(CONFIG_EXECMEM, MODULES_VADDR))]
const MODULES_TEXT_START: usize = MODULES_VADDR;
#[cfg(all(CONFIG_EXECMEM, MODULES_VADDR))]
const MODULES_TEXT_END: usize = MODULES_VADDR + SZ_256M;
#[cfg(all(CONFIG_EXECMEM, MODULES_VADDR))]
const MODULES_DATA_START: usize = MODULES_VADDR + SZ_256M;
#[cfg(all(CONFIG_EXECMEM, MODULES_VADDR))]
const MODULES_DATA_END: usize = MODULES_END;

#[cfg(all(CONFIG_EXECMEM, MODULES_VADDR))]
static mut execmem_info: execmem_info = execmem_info::default();

#[cfg(all(CONFIG_EXECMEM, MODULES_VADDR))]
pub unsafe fn execmem_arch_setup() -> *mut execmem_info {
    execmem_info = execmem_info {
        ranges: [
            (EXECMEM_MODULE_TEXT, execmem_range { start: MODULES_TEXT_START, end: MODULES_TEXT_END, pgprot: PAGE_KERNEL, alignment: 1 }),
            (EXECMEM_MODULE_DATA, execmem_range { start: MODULES_DATA_START, end: MODULES_DATA_END, pgprot: PAGE_KERNEL, alignment: 1 }),
        ],
    };
    &raw mut execmem_info
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
