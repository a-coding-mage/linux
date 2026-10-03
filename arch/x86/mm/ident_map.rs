// SPDX-License-Identifier: GPL-2.0
/*
 * Helper routines for building identity mapping page tables. Like ident_map.c,
 * this module obtains configured page primitives from its including owner.
 * The compressed owner supplies native x86-64 Rust primitives and the generated
 * asm/init.h layout; a regular-kernel owner must supply its own __pa/__va and
 * configured page-table setters before including this module.
 */
use super::*;

unsafe fn free_pte(info: *mut x86_mapping_info, pmd: *mut pmd_t) {
    unsafe {
        let pte = pte_offset_kernel(pmd, 0);
        ((*info).free_pgt_page.unwrap())(pte.cast(), (*info).context);
    }
}

unsafe fn free_pmd(info: *mut x86_mapping_info, pud: *mut pud_t) {
    unsafe {
        let pmd = pmd_offset(pud, 0);
        for i in 0..PTRS_PER_PMD {
            if !pmd_present(*pmd.add(i)) || pmd_leaf(*pmd.add(i)) {
                continue;
            }
            free_pte(info, pmd.add(i));
        }
        ((*info).free_pgt_page.unwrap())(pmd.cast(), (*info).context);
    }
}

unsafe fn free_pud(info: *mut x86_mapping_info, p4d: *mut p4d_t) {
    unsafe {
        let pud = pud_offset(p4d, 0);
        for i in 0..PTRS_PER_PUD {
            if !pud_present(*pud.add(i)) || pud_leaf(*pud.add(i)) {
                continue;
            }
            free_pmd(info, pud.add(i));
        }
        ((*info).free_pgt_page.unwrap())(pud.cast(), (*info).context);
    }
}

unsafe fn free_p4d(info: *mut x86_mapping_info, pgd: *mut pgd_t) {
    unsafe {
        let p4d = p4d_offset(pgd, 0);
        for i in 0..ptrs_per_p4d() {
            if !p4d_present(*p4d.add(i)) {
                continue;
            }
            free_pud(info, p4d.add(i));
        }
        if pgtable_l5_enabled() {
            ((*info).free_pgt_page.unwrap())(p4d.cast(), (*info).context);
        }
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn kernel_ident_mapping_free(
    info: *mut x86_mapping_info,
    pgd: *mut pgd_t,
) {
    unsafe {
        for i in 0..PTRS_PER_PGD {
            if !pgd_present(*pgd.add(i)) {
                continue;
            }
            free_p4d(info, pgd.add(i));
        }
        ((*info).free_pgt_page.unwrap())(pgd.cast(), (*info).context);
    }
}

unsafe fn ident_pmd_init(
    info: *mut x86_mapping_info,
    pmd_page: *mut pmd_t,
    mut addr: c_ulong,
    end: c_ulong,
) {
    unsafe {
        addr &= PMD_MASK;
        while addr < end {
            let pmd = pmd_page.add(pmd_index(addr));
            if !pmd_present(*pmd) {
                set_pmd(
                    pmd,
                    __pmd(addr.wrapping_sub((*info).offset) | (*info).page_flag),
                );
            }
            addr = addr.wrapping_add(PMD_SIZE);
        }
    }
}

unsafe fn ident_pud_init(
    info: *mut x86_mapping_info,
    pud_page: *mut pud_t,
    mut addr: c_ulong,
    end: c_ulong,
) -> c_int {
    unsafe {
        while addr < end {
            let pud = pud_page.add(pud_index(addr));
            let next = pud_addr_end(addr, end);
            if pud_leaf(*pud) {
                addr = next;
                continue;
            }
            // A 1 GiB leaf may neither extend the range nor overwrite a table.
            let mut use_gbpage = (*info).direct_gbpages;
            use_gbpage &= addr & !PUD_MASK == 0;
            use_gbpage &= next & !PUD_MASK == 0;
            use_gbpage &= !pud_present(*pud);
            if use_gbpage {
                set_pud(
                    pud,
                    __pud(addr.wrapping_sub((*info).offset) | (*info).page_flag),
                );
                addr = next;
                continue;
            }
            if pud_present(*pud) {
                ident_pmd_init(info, pmd_offset(pud, 0), addr, next);
                addr = next;
                continue;
            }
            let pmd = ((*info).alloc_pgt_page.unwrap())((*info).context).cast::<pmd_t>();
            if pmd.is_null() {
                return -ENOMEM;
            }
            ident_pmd_init(info, pmd, addr, next);
            set_pud(pud, __pud(__pa(pmd.cast()) | (*info).kernpg_flag));
            addr = next;
        }
        0
    }
}

unsafe fn ident_p4d_init(
    info: *mut x86_mapping_info,
    p4d_page: *mut p4d_t,
    mut addr: c_ulong,
    end: c_ulong,
) -> c_int {
    unsafe {
        while addr < end {
            let p4d = p4d_page.add(p4d_index(addr));
            let next = p4d_addr_end(addr, end);
            if p4d_present(*p4d) {
                let result = ident_pud_init(info, pud_offset(p4d, 0), addr, next);
                if result != 0 {
                    return result;
                }
                addr = next;
                continue;
            }
            let pud = ((*info).alloc_pgt_page.unwrap())((*info).context).cast::<pud_t>();
            if pud.is_null() {
                return -ENOMEM;
            }
            let result = ident_pud_init(info, pud, addr, next);
            if result != 0 {
                return result;
            }
            set_p4d(
                p4d,
                __p4d(__pa(pud.cast()) | (*info).kernpg_flag | _PAGE_NOPTISHADOW),
            );
            addr = next;
        }
        0
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn kernel_ident_mapping_init(
    info: *mut x86_mapping_info,
    pgd_page: *mut pgd_t,
    pstart: c_ulong,
    pend: c_ulong,
) -> c_int {
    unsafe {
        let mut addr = pstart.wrapping_add((*info).offset);
        let end = pend.wrapping_add((*info).offset);
        if (*info).kernpg_flag == 0 {
            (*info).kernpg_flag = kernpg_table();
        }
        (*info).kernpg_flag &= __default_kernel_pte_mask;
        while addr < end {
            let pgd = pgd_page.add(pgd_index(addr));
            let next = pgd_addr_end(addr, end);
            if pgd_present(*pgd) {
                let result = ident_p4d_init(info, p4d_offset(pgd, 0), addr, next);
                if result != 0 {
                    return result;
                }
                addr = next;
                continue;
            }
            let p4d = ((*info).alloc_pgt_page.unwrap())((*info).context).cast::<p4d_t>();
            if p4d.is_null() {
                return -ENOMEM;
            }
            let result = ident_p4d_init(info, p4d, addr, next);
            if result != 0 {
                return result;
            }
            if pgtable_l5_enabled() {
                set_pgd(
                    pgd,
                    __pgd(__pa(p4d.cast()) | (*info).kernpg_flag | _PAGE_NOPTISHADOW),
                );
            } else {
                let pud = pud_offset(p4d, 0);
                set_pgd(
                    pgd,
                    __pgd(__pa(pud.cast()) | (*info).kernpg_flag | _PAGE_NOPTISHADOW),
                );
            }
            addr = next;
        }
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
