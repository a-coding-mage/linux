// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Dependency declarations and build-time constants are supplied by the
// corresponding Linux kernel headers and architecture code.

pub static mut highstart_pfn: ::core::ffi::c_ulong = 0;
pub static mut highend_pfn: ::core::ffi::c_ulong = 0;

extern "C" {
    fn flush_tlb_one(addr: ::core::ffi::c_ulong);
    fn fixrange_init(
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
        pgd: *mut pgd_t,
    );
    fn pgd_index(addr: ::core::ffi::c_ulong) -> usize;
    fn pmd_offset(pud: *mut pud_t, addr: ::core::ffi::c_ulong) -> *mut pmd_t;
    fn pte_offset_kernel(pmd: *mut pmd_t, addr: ::core::ffi::c_ulong) -> *mut pte_t;

    static mut swapper_pg_dir: *mut pgd_t;
    static mut pkmap_page_table: *mut pte_t;
}

extern "C" {
    type pgd_t;
    type pmd_t;
    type pud_t;
    type pte_t;
}

pub unsafe fn kmap_flush_tlb(addr: ::core::ffi::c_ulong) {
    flush_tlb_one(addr);
}

pub unsafe fn kmap_init() {
    let mut vaddr: ::core::ffi::c_ulong;
    let mut pgd: *mut pgd_t;
    let mut pmd: *mut pmd_t;
    let mut pud: *mut pud_t;
    let mut pte: *mut pte_t;

    vaddr = PKMAP_BASE;
    fixrange_init(
        vaddr,
        vaddr.wrapping_add(PAGE_SIZE.wrapping_mul(LAST_PKMAP)),
        swapper_pg_dir,
    );

    pgd = swapper_pg_dir.add(pgd_index(vaddr));
    pud = pgd as *mut pud_t;
    pmd = pmd_offset(pud, vaddr);
    pte = pte_offset_kernel(pmd, vaddr);
    pkmap_page_table = pte;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
