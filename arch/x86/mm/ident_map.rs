// SPDX-License-Identifier: GPL-2.0
/*
 * Helper routines for building identity mapping page tables. This is
 * included by both the compressed kernel and the regular kernel.
 */

// Types, constants, and helper operations below are supplied by the x86
// paging headers and are intentionally left as external dependencies.
extern "C" {
    fn pte_offset_kernel(pmd: *mut pmd_t, address: usize) -> *mut pte_t;
    fn pmd_offset(pud: *mut pud_t, address: usize) -> *mut pmd_t;
    fn pud_offset(p4d: *mut p4d_t, address: usize) -> *mut pud_t;
    fn p4d_offset(pgd: *mut pgd_t, address: usize) -> *mut p4d_t;
    fn pmd_present(pmd: pmd_t) -> bool;
    fn pud_present(pud: pud_t) -> bool;
    fn p4d_present(p4d: p4d_t) -> bool;
    fn pgd_present(pgd: pgd_t) -> bool;
    fn pmd_leaf(pmd: pmd_t) -> bool;
    fn pud_leaf(pud: pud_t) -> bool;
    fn pgtable_l5_enabled() -> bool;
    fn pmd_index(address: usize) -> usize;
    fn pud_index(address: usize) -> usize;
    fn p4d_index(address: usize) -> usize;
    fn pgd_index(address: usize) -> usize;
    fn pud_addr_end(address: usize, end: usize) -> usize;
    fn p4d_addr_end(address: usize, end: usize) -> usize;
    fn pgd_addr_end(address: usize, end: usize) -> usize;
    fn set_pmd(entry: *mut pmd_t, value: pmd_t);
    fn set_pud(entry: *mut pud_t, value: pud_t);
    fn set_p4d(entry: *mut p4d_t, value: p4d_t);
    fn set_pgd(entry: *mut pgd_t, value: pgd_t);
    fn __pmd(value: usize) -> pmd_t;
    fn __pud(value: usize) -> pud_t;
    fn __p4d(value: usize) -> p4d_t;
    fn __pgd(value: usize) -> pgd_t;
    fn __pa(value: *mut core::ffi::c_void) -> usize;
}

#[repr(C)]
pub struct x86_mapping_info {
    pub alloc_pgt_page: unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
    pub free_pgt_page: unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub context: *mut core::ffi::c_void,
    pub offset: usize,
    pub kernpg_flag: usize,
    pub page_flag: usize,
    pub direct_gbpages: bool,
}

#[repr(transparent)] #[derive(Copy, Clone)] pub struct pte_t(pub usize);
#[repr(transparent)] #[derive(Copy, Clone)] pub struct pmd_t(pub usize);
#[repr(transparent)] #[derive(Copy, Clone)] pub struct pud_t(pub usize);
#[repr(transparent)] #[derive(Copy, Clone)] pub struct p4d_t(pub usize);
#[repr(transparent)] #[derive(Copy, Clone)] pub struct pgd_t(pub usize);

extern "C" {
    static PTRS_PER_PMD: usize;
    static PTRS_PER_PUD: usize;
    static PTRS_PER_P4D: usize;
    static PTRS_PER_PGD: usize;
    static PMD_MASK: usize;
    static PUD_MASK: usize;
    static PMD_SIZE: usize;
    static _PAGE_NOPTISHADOW: usize;
    static _KERNPG_TABLE: usize;
    static __default_kernel_pte_mask: usize;
}

unsafe fn free_pte(info: *mut x86_mapping_info, pmd: *mut pmd_t) {
    let pte = pte_offset_kernel(pmd, 0);
    ((*info).free_pgt_page)(pte.cast(), (*info).context);
}

unsafe fn free_pmd(info: *mut x86_mapping_info, pud: *mut pud_t) {
    let pmd = pmd_offset(pud, 0);
    for i in 0..PTRS_PER_PMD {
        if !pmd_present(*pmd.add(i)) || pmd_leaf(*pmd.add(i)) { continue; }
        free_pte(info, pmd.add(i));
    }
    ((*info).free_pgt_page)(pmd.cast(), (*info).context);
}

unsafe fn free_pud(info: *mut x86_mapping_info, p4d: *mut p4d_t) {
    let pud = pud_offset(p4d, 0);
    for i in 0..PTRS_PER_PUD {
        if !pud_present(*pud.add(i)) || pud_leaf(*pud.add(i)) { continue; }
        free_pmd(info, pud.add(i));
    }
    ((*info).free_pgt_page)(pud.cast(), (*info).context);
}

unsafe fn free_p4d(info: *mut x86_mapping_info, pgd: *mut pgd_t) {
    let p4d = p4d_offset(pgd, 0);
    for i in 0..PTRS_PER_P4D {
        if !p4d_present(*p4d.add(i)) { continue; }
        free_pud(info, p4d.add(i));
    }
    if pgtable_l5_enabled() { ((*info).free_pgt_page)(p4d.cast(), (*info).context); }
}

#[no_mangle]
pub unsafe extern "C" fn kernel_ident_mapping_free(info: *mut x86_mapping_info, pgd: *mut pgd_t) {
    for i in 0..PTRS_PER_PGD {
        if !pgd_present(*pgd.add(i)) { continue; }
        free_p4d(info, pgd.add(i));
    }
    ((*info).free_pgt_page)(pgd.cast(), (*info).context);
}

unsafe fn ident_pmd_init(info: *mut x86_mapping_info, pmd_page: *mut pmd_t, mut addr: usize, end: usize) {
    addr &= PMD_MASK;
    while addr < end {
        let pmd = pmd_page.add(pmd_index(addr));
        if !pmd_present(*pmd) { set_pmd(pmd, __pmd((addr.wrapping_sub((*info).offset)) | (*info).page_flag)); }
        addr = addr.wrapping_add(PMD_SIZE);
    }
}

unsafe fn ident_pud_init(info: *mut x86_mapping_info, pud_page: *mut pud_t, mut addr: usize, end: usize) -> i32 {
    while addr < end {
        let pud = pud_page.add(pud_index(addr));
        let next = pud_addr_end(addr, end);
        if pud_leaf(*pud) { addr = next; continue; }
        let mut use_gbpage = (*info).direct_gbpages;
        use_gbpage &= (addr & !PUD_MASK) == 0;
        use_gbpage &= (next & !PUD_MASK) == 0;
        use_gbpage &= !pud_present(*pud);
        if use_gbpage {
            set_pud(pud, __pud(addr.wrapping_sub((*info).offset) | (*info).page_flag));
            addr = next; continue;
        }
        let pmd: *mut pmd_t;
        if pud_present(*pud) { pmd = pmd_offset(pud, 0); }
        else {
            pmd = ((*info).alloc_pgt_page)((*info).context).cast();
            if pmd.is_null() { return -12; }
            ident_pmd_init(info, pmd, addr, next);
            set_pud(pud, __pud(__pa(pmd.cast()) | (*info).kernpg_flag));
            addr = next; continue;
        }
        ident_pmd_init(info, pmd, addr, next);
        addr = next;
    }
    0
}

unsafe fn ident_p4d_init(info: *mut x86_mapping_info, p4d_page: *mut p4d_t, mut addr: usize, end: usize) -> i32 {
    while addr < end {
        let p4d = p4d_page.add(p4d_index(addr));
        let next = p4d_addr_end(addr, end);
        let pud: *mut pud_t;
        if p4d_present(*p4d) { pud = pud_offset(p4d, 0); }
        else {
            pud = ((*info).alloc_pgt_page)((*info).context).cast();
            if pud.is_null() { return -12; }
            let result = ident_pud_init(info, pud, addr, next); if result != 0 { return result; }
            set_p4d(p4d, __p4d(__pa(pud.cast()) | (*info).kernpg_flag | _PAGE_NOPTISHADOW));
            addr = next; continue;
        }
        let result = ident_pud_init(info, pud, addr, next); if result != 0 { return result; }
        addr = next;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn kernel_ident_mapping_init(info: *mut x86_mapping_info, pgd_page: *mut pgd_t, pstart: usize, pend: usize) -> i32 {
    let mut addr = pstart.wrapping_add((*info).offset);
    let end = pend.wrapping_add((*info).offset);
    if (*info).kernpg_flag == 0 { (*info).kernpg_flag = _KERNPG_TABLE; }
    (*info).kernpg_flag &= __default_kernel_pte_mask;
    while addr < end {
        let pgd = pgd_page.add(pgd_index(addr));
        let next = pgd_addr_end(addr, end);
        let p4d: *mut p4d_t;
        if pgd_present(*pgd) { p4d = p4d_offset(pgd, 0); }
        else {
            p4d = ((*info).alloc_pgt_page)((*info).context).cast();
            if p4d.is_null() { return -12; }
            let result = ident_p4d_init(info, p4d, addr, next); if result != 0 { return result; }
            if pgtable_l5_enabled() { set_pgd(pgd, __pgd(__pa(p4d.cast()) | (*info).kernpg_flag | _PAGE_NOPTISHADOW)); }
            else { let pud = pud_offset(p4d, 0); set_pgd(pgd, __pgd(__pa(pud.cast()) | (*info).kernpg_flag | _PAGE_NOPTISHADOW)); }
            addr = next; continue;
        }
        let result = ident_p4d_init(info, p4d, addr, next); if result != 0 { return result; }
        addr = next;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
