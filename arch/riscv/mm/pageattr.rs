// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2019 SiFive */

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
struct pageattr_masks {
    set_mask: pgprot_t,
    clear_mask: pgprot_t,
}

unsafe fn set_pageattr_masks(val: c_ulong, walk: *mut mm_walk) -> c_ulong {
    let masks = (*walk).private as *mut pageattr_masks;
    let mut new_val = val;
    new_val &= !(pgprot_val((*masks).clear_mask));
    new_val |= pgprot_val((*masks).set_mask);
    new_val
}

unsafe fn pageattr_p4d_entry(p4d: *mut p4d_t, _addr: c_ulong, _next: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut val = p4dp_get(p4d);
    if p4d_leaf(val) { val = __p4d(set_pageattr_masks(p4d_val(val), walk)); set_p4d(p4d, val); }
    0
}
unsafe fn pageattr_pud_entry(pud: *mut pud_t, _addr: c_ulong, _next: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut val = pudp_get(pud);
    if pud_leaf(val) { val = __pud(set_pageattr_masks(pud_val(val), walk)); set_pud(pud, val); }
    0
}
unsafe fn pageattr_pmd_entry(pmd: *mut pmd_t, _addr: c_ulong, _next: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut val = pmdp_get(pmd);
    if pmd_leaf(val) { val = __pmd(set_pageattr_masks(pmd_val(val), walk)); set_pmd(pmd, val); }
    0
}
unsafe fn pageattr_pte_entry(pte: *mut pte_t, _addr: c_ulong, _next: c_ulong, walk: *mut mm_walk) -> c_int {
    let mut val = ptep_get(pte);
    val = __pte(set_pageattr_masks(pte_val(val), walk));
    set_pte(pte, val); 0
}
unsafe fn pageattr_pte_hole(_addr: c_ulong, _next: c_ulong, _depth: c_int, _walk: *mut mm_walk) -> c_int { 0 }

static pageattr_ops: mm_walk_ops = mm_walk_ops {
    p4d_entry: Some(pageattr_p4d_entry), pud_entry: Some(pageattr_pud_entry),
    pmd_entry: Some(pageattr_pmd_entry), pte_entry: Some(pageattr_pte_entry),
    pte_hole: Some(pageattr_pte_hole), walk_lock: PGWALK_RDLOCK,
};

#[cfg(target_pointer_width = "64")]
unsafe fn __split_linear_mapping_pmd(pudp: *mut pud_t, mut vaddr: c_ulong, end: c_ulong) -> c_int {
    let mut pmdp = pmd_offset(pudp, vaddr); let mut next;
    loop {
        next = pmd_addr_end(vaddr, end);
        if next - vaddr >= PMD_SIZE && vaddr <= (vaddr & PMD_MASK) && end >= next { }
        else if pmd_leaf(pmdp_get(pmdp)) {
            let pte_page = alloc_page(GFP_KERNEL); if pte_page.is_null() { return -ENOMEM; }
            let pfn = _pmd_pfn(pmdp_get(pmdp));
            let prot = __pgprot(pmd_val(pmdp_get(pmdp)) & !_PAGE_PFN_MASK);
            let mut ptep_new = page_address(pte_page) as *mut pte_t;
            for i in 0..PTRS_PER_PTE { set_pte(ptep_new, pfn_pte(pfn + i, prot)); ptep_new = ptep_new.add(1); }
            smp_wmb(); set_pmd(pmdp, pfn_pmd(page_to_pfn(pte_page), PAGE_TABLE));
        }
        pmdp = pmdp.add(1); vaddr = next; if vaddr == end { break; }
    } 0
}

#[cfg(target_pointer_width = "64")]
unsafe fn __split_linear_mapping_pud(p4dp: *mut p4d_t, mut vaddr: c_ulong, end: c_ulong) -> c_int {
    let mut pudp = pud_offset(p4dp, vaddr); let mut next;
    loop {
        next = pud_addr_end(vaddr, end);
        if next - vaddr >= PUD_SIZE && vaddr <= (vaddr & PUD_MASK) && end >= next { }
        else if pud_leaf(pudp_get(pudp)) {
            let pmd_page = alloc_page(GFP_KERNEL); if pmd_page.is_null() { return -ENOMEM; }
            let pfn = _pud_pfn(pudp_get(pudp)); let prot = __pgprot(pud_val(pudp_get(pudp)) & !_PAGE_PFN_MASK);
            let mut pmdp_new = page_address(pmd_page) as *mut pmd_t;
            for i in 0..PTRS_PER_PMD { set_pmd(pmdp_new, pfn_pmd(pfn + ((i * PMD_SIZE) >> PAGE_SHIFT), prot)); pmdp_new = pmdp_new.add(1); }
            smp_wmb(); set_pud(pudp, pfn_pud(page_to_pfn(pmd_page), PAGE_TABLE));
        }
        let ret = __split_linear_mapping_pmd(pudp, vaddr, next); if ret != 0 { return ret; }
        pudp = pudp.add(1); vaddr = next; if vaddr == end { break; }
    } 0
}

#[cfg(target_pointer_width = "64")]
unsafe fn __split_linear_mapping_p4d(pgdp: *mut pgd_t, mut vaddr: c_ulong, end: c_ulong) -> c_int {
    let mut p4dp = p4d_offset(pgdp, vaddr); let mut next;
    loop {
        next = p4d_addr_end(vaddr, end);
        if next - vaddr >= P4D_SIZE && vaddr <= (vaddr & P4D_MASK) && end >= next { }
        else if p4d_leaf(p4dp_get(p4dp)) {
            let pud_page = alloc_page(GFP_KERNEL); if pud_page.is_null() { return -ENOMEM; }
            let pfn = _p4d_pfn(p4dp_get(p4dp)); let prot = __pgprot(p4d_val(p4dp_get(p4dp)) & !_PAGE_PFN_MASK);
            let mut pudp_new = page_address(pud_page) as *mut pud_t;
            for i in 0..PTRS_PER_PUD { set_pud(pudp_new, pfn_pud(pfn + ((i * PUD_SIZE) >> PAGE_SHIFT), prot)); pudp_new = pudp_new.add(1); }
            smp_wmb(); set_p4d(p4dp, pfn_p4d(page_to_pfn(pud_page), PAGE_TABLE));
        }
        let ret = __split_linear_mapping_pud(p4dp, vaddr, next); if ret != 0 { return ret; }
        p4dp = p4dp.add(1); vaddr = next; if vaddr == end { break; }
    } 0
}

#[cfg(target_pointer_width = "64")]
unsafe fn __split_linear_mapping_pgd(pgdp: *mut pgd_t, mut vaddr: c_ulong, end: c_ulong) -> c_int {
    loop { let next = pgd_addr_end(vaddr, end); let ret = __split_linear_mapping_p4d(pgdp, vaddr, next); if ret != 0 { return ret; } pgdp = pgdp.add(1); vaddr = next; if vaddr == end { break; } } 0
}
#[cfg(target_pointer_width = "64")]
unsafe fn split_linear_mapping(start: c_ulong, end: c_ulong) -> c_int { __split_linear_mapping_pgd(pgd_offset_k(start), start, end) }

unsafe fn __set_memory(addr: c_ulong, numpages: c_int, set_mask: pgprot_t, clear_mask: pgprot_t) -> c_int {
    let start = addr; let end = start + PAGE_SIZE * numpages as c_ulong;
    let mut masks = pageattr_masks { set_mask, clear_mask }; if numpages == 0 { return 0; }
    mmap_write_lock(&mut init_mm);
    let ret = walk_kernel_page_table_range(start, end, &pageattr_ops, core::ptr::null_mut(), &mut masks);
    mmap_write_unlock(&mut init_mm); flush_tlb_kernel_range(start, end); ret
}

pub unsafe fn set_memory_rw_nx(addr: c_ulong, n: c_int) -> c_int { __set_memory(addr,n,__pgprot(_PAGE_READ|_PAGE_WRITE),__pgprot(_PAGE_EXEC)) }
pub unsafe fn set_memory_ro(addr: c_ulong, n: c_int) -> c_int { __set_memory(addr,n,__pgprot(_PAGE_READ),__pgprot(_PAGE_WRITE)) }
pub unsafe fn set_memory_rw(addr: c_ulong, n: c_int) -> c_int { __set_memory(addr,n,__pgprot(_PAGE_READ|_PAGE_WRITE),__pgprot(0)) }
pub unsafe fn set_memory_x(addr: c_ulong, n: c_int) -> c_int { __set_memory(addr,n,__pgprot(_PAGE_EXEC),__pgprot(0)) }
pub unsafe fn set_memory_nx(addr: c_ulong, n: c_int) -> c_int { __set_memory(addr,n,__pgprot(0),__pgprot(_PAGE_EXEC)) }
pub unsafe fn set_direct_map_invalid_noflush(page: *mut page) -> c_int { __set_memory(page_address(page) as c_ulong,1,__pgprot(0),__pgprot(_PAGE_PRESENT)) }
pub unsafe fn set_direct_map_default_noflush(page: *mut page) -> c_int { __set_memory(page_address(page) as c_ulong,1,PAGE_KERNEL,__pgprot(_PAGE_EXEC)) }
pub unsafe fn set_direct_map_valid_noflush(page: *mut page, nr: c_uint, valid: bool) -> c_int { if valid { __set_memory(page_address(page) as c_ulong,nr as c_int,PAGE_KERNEL,__pgprot(_PAGE_EXEC)) } else { __set_memory(page_address(page) as c_ulong,nr as c_int,__pgprot(0),__pgprot(_PAGE_PRESENT)) } }

#[cfg(CONFIG_DEBUG_PAGEALLOC)]
unsafe fn debug_pagealloc_set_page(pte: *mut pte_t, _addr: c_ulong, data: *mut c_void) -> c_int {
    let enable = *(data as *mut c_int); let mut val = pte_val(ptep_get(pte));
    if enable != 0 { val |= _PAGE_PRESENT; } else { val &= !_PAGE_PRESENT; }
    set_pte(pte, __pte(val)); 0
}

#[cfg(CONFIG_DEBUG_PAGEALLOC)]
pub unsafe fn __kernel_map_pages(page: *mut page, numpages: c_int, enable: c_int) {
    if !debug_pagealloc_enabled() { return; }
    let start = page_address(page) as c_ulong; let size = PAGE_SIZE * numpages as c_ulong;
    apply_to_existing_page_range(&mut init_mm, start, size, Some(debug_pagealloc_set_page), &enable as *const _ as *mut c_void);
    flush_tlb_kernel_range(start, start + size);
}

pub unsafe fn kernel_page_present(page: *mut page) -> bool {
    let addr = page_address(page) as c_ulong; let pgd = pgd_offset_k(addr); if !pgd_present(pgdp_get(pgd)) { return false; } if pgd_leaf(pgdp_get(pgd)) { return true; }
    let p4d = p4d_offset(pgd,addr); if !p4d_present(p4dp_get(p4d)) { return false; } if p4d_leaf(p4dp_get(p4d)) { return true; }
    let pud = pud_offset(p4d,addr); if !pud_present(pudp_get(pud)) { return false; } if pud_leaf(pudp_get(pud)) { return true; }
    let pmd = pmd_offset(pud,addr); if !pmd_present(pmdp_get(pmd)) { return false; } if pmd_leaf(pmdp_get(pmd)) { return true; }
    pte_present(ptep_get(pte_offset_kernel(pmd,addr)))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
