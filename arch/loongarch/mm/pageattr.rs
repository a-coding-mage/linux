// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct pageattr_masks {
    set_mask: pgprot_t,
    clear_mask: pgprot_t,
}

unsafe fn set_pageattr_masks(val: ::core::primitive::c_ulong, walk: *mut mm_walk) -> ::core::primitive::c_ulong {
    let mut new_val = val;
    let masks = (*walk).private as *mut pageattr_masks;

    new_val &= !(pgprot_val((*masks).clear_mask));
    new_val |= pgprot_val((*masks).set_mask);

    new_val
}

unsafe extern "C" fn pageattr_pgd_entry(pgd: *mut pgd_t, _addr: ::core::primitive::c_ulong,
                                         _next: ::core::primitive::c_ulong, walk: *mut mm_walk) -> ::core::primitive::c_int {
    let mut val = pgdp_get(pgd);

    if pgd_leaf(val) {
        val = __pgd(set_pageattr_masks(pgd_val(val), walk));
        set_pgd(pgd, val);
    }

    0
}

unsafe extern "C" fn pageattr_p4d_entry(p4d: *mut p4d_t, _addr: ::core::primitive::c_ulong,
                                         _next: ::core::primitive::c_ulong, walk: *mut mm_walk) -> ::core::primitive::c_int {
    let mut val = p4dp_get(p4d);

    if p4d_leaf(val) {
        val = __p4d(set_pageattr_masks(p4d_val(val), walk));
        set_p4d(p4d, val);
    }

    0
}

unsafe extern "C" fn pageattr_pud_entry(pud: *mut pud_t, _addr: ::core::primitive::c_ulong,
                                         _next: ::core::primitive::c_ulong, walk: *mut mm_walk) -> ::core::primitive::c_int {
    let mut val = pudp_get(pud);

    if pud_leaf(val) {
        val = __pud(set_pageattr_masks(pud_val(val), walk));
        set_pud(pud, val);
    }

    0
}

unsafe extern "C" fn pageattr_pmd_entry(pmd: *mut pmd_t, _addr: ::core::primitive::c_ulong,
                                         _next: ::core::primitive::c_ulong, walk: *mut mm_walk) -> ::core::primitive::c_int {
    let mut val = pmdp_get(pmd);

    if pmd_leaf(val) {
        val = __pmd(set_pageattr_masks(pmd_val(val), walk));
        set_pmd(pmd, val);
    }

    0
}

unsafe extern "C" fn pageattr_pte_entry(pte: *mut pte_t, _addr: ::core::primitive::c_ulong,
                                         _next: ::core::primitive::c_ulong, walk: *mut mm_walk) -> ::core::primitive::c_int {
    let val = __pte(set_pageattr_masks(pte_val(ptep_get(pte)), walk));
    set_pte(pte, val);

    0
}

unsafe extern "C" fn pageattr_pte_hole(_addr: ::core::primitive::c_ulong,
                                        _next: ::core::primitive::c_ulong, _depth: ::core::primitive::c_int,
                                        _walk: *mut mm_walk) -> ::core::primitive::c_int {
    0
}

static pageattr_ops: mm_walk_ops = mm_walk_ops {
    pgd_entry: Some(pageattr_pgd_entry),
    p4d_entry: Some(pageattr_p4d_entry),
    pud_entry: Some(pageattr_pud_entry),
    pmd_entry: Some(pageattr_pmd_entry),
    pte_entry: Some(pageattr_pte_entry),
    pte_hole: Some(pageattr_pte_hole),
    walk_lock: PGWALK_RDLOCK,
};

unsafe fn __set_memory(addr: ::core::primitive::c_ulong, numpages: ::core::primitive::c_int,
                       set_mask: pgprot_t, clear_mask: pgprot_t) -> ::core::primitive::c_int {
    let mut ret: ::core::primitive::c_int;
    let start = addr;
    let end = start + PAGE_SIZE * numpages as ::core::primitive::c_ulong;
    let masks = pageattr_masks { set_mask, clear_mask };

    if numpages == 0 {
        return 0;
    }

    mmap_write_lock(&init_mm);
    ret = walk_kernel_page_table_range(start, end, &pageattr_ops, core::ptr::null_mut(), &masks as *const _ as *mut _);
    mmap_write_unlock(&init_mm);

    flush_tlb_kernel_range(start, end);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn set_memory_x(addr: ::core::primitive::c_ulong, numpages: ::core::primitive::c_int) -> ::core::primitive::c_int {
    if addr < vm_map_base { return 0; }
    __set_memory(addr, numpages, __pgprot(0), __pgprot(_PAGE_NO_EXEC))
}

#[no_mangle]
pub unsafe extern "C" fn set_memory_nx(addr: ::core::primitive::c_ulong, numpages: ::core::primitive::c_int) -> ::core::primitive::c_int {
    if addr < vm_map_base { return 0; }
    __set_memory(addr, numpages, __pgprot(_PAGE_NO_EXEC), __pgprot(0))
}

#[no_mangle]
pub unsafe extern "C" fn set_memory_ro(addr: ::core::primitive::c_ulong, numpages: ::core::primitive::c_int) -> ::core::primitive::c_int {
    if addr < vm_map_base { return 0; }
    __set_memory(addr, numpages, __pgprot(0), __pgprot(_PAGE_WRITE | _PAGE_DIRTY))
}

#[no_mangle]
pub unsafe extern "C" fn set_memory_rw(addr: ::core::primitive::c_ulong, numpages: ::core::primitive::c_int) -> ::core::primitive::c_int {
    if addr < vm_map_base { return 0; }
    __set_memory(addr, numpages, __pgprot(_PAGE_WRITE | _PAGE_DIRTY), __pgprot(0))
}

#[no_mangle]
pub unsafe extern "C" fn kernel_page_present(page: *mut page) -> bool {
    let addr = page_address(page) as ::core::primitive::c_ulong;

    if addr < vm_map_base { return memblock_is_memory(__pa(addr)); }

    let pgd = pgd_offset_k(addr);
    if pgd_none(pgdp_get(pgd)) { return false; }
    if pgd_leaf(pgdp_get(pgd)) { return true; }
    let p4d = p4d_offset(pgd, addr);
    if p4d_none(p4dp_get(p4d)) { return false; }
    if p4d_leaf(p4dp_get(p4d)) { return true; }
    let pud = pud_offset(p4d, addr);
    if pud_none(pudp_get(pud)) { return false; }
    if pud_leaf(pudp_get(pud)) { return true; }
    let pmd = pmd_offset(pud, addr);
    if pmd_none(pmdp_get(pmd)) { return false; }
    if pmd_leaf(pmdp_get(pmd)) { return true; }
    let pte = pte_offset_kernel(pmd, addr);
    pte_present(ptep_get(pte))
}

#[no_mangle]
pub unsafe extern "C" fn set_direct_map_default_noflush(page: *mut page) -> ::core::primitive::c_int {
    let addr = page_address(page) as ::core::primitive::c_ulong;
    if addr < vm_map_base { return 0; }
    __set_memory(addr, 1, PAGE_KERNEL, __pgprot(0))
}

#[no_mangle]
pub unsafe extern "C" fn set_direct_map_invalid_noflush(page: *mut page) -> ::core::primitive::c_int {
    let addr = page_address(page) as ::core::primitive::c_ulong;
    if addr < vm_map_base { return 0; }
    __set_memory(addr, 1, __pgprot(0), __pgprot(_PAGE_PRESENT | _PAGE_VALID))
}

#[no_mangle]
pub unsafe extern "C" fn set_direct_map_valid_noflush(page: *mut page, nr: ::core::primitive::c_uint, valid: bool) -> ::core::primitive::c_int {
    let addr = page_address(page) as ::core::primitive::c_ulong;
    if addr < vm_map_base { return 0; }

    let (set, clear) = if valid {
        (PAGE_KERNEL, __pgprot(0))
    } else {
        (__pgprot(0), __pgprot(_PAGE_PRESENT | _PAGE_VALID))
    };
    __set_memory(addr, nr as ::core::primitive::c_int, set, clear)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
