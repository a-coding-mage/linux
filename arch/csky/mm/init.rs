// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Kernel and architecture dependencies are supplied by the surrounding build.

const PTRS_KERN_TABLE: usize = (PTRS_PER_PGD - USER_PTRS_PER_PGD) * PTRS_PER_PTE;

#[no_mangle]
pub static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD] = [unsafe { core::mem::zeroed() }; PTRS_PER_PGD];
#[no_mangle]
pub static mut invalid_pte_table: [pte_t; PTRS_PER_PTE] = [unsafe { core::mem::zeroed() }; PTRS_PER_PTE];
#[no_mangle]
pub static mut kernel_pte_tables: [pte_t; PTRS_KERN_TABLE] = [unsafe { core::mem::zeroed() }; PTRS_KERN_TABLE];

extern "C" {
    static va_pa_offset: unsigned_long;

    fn free_initmem_default(level: i32);
    fn flush_tlb_all();
    fn local_icache_inv_all(addr: *mut core::ffi::c_void);
    fn __pa(addr: *const core::ffi::c_void) -> unsigned_long;
    fn set_pte(ptep: *mut pte_t, pte: pte_t);
    fn __pte(value: unsigned_long) -> pte_t;
    fn pfn_pte(pfn: unsigned_long, prot: pgprot_t) -> pte_t;
    fn write_mmu_pagemask(value: unsigned_long);
    fn setup_pgd(pgd: *mut pgd_t, asid: unsigned_long);
    fn pgd_index(addr: unsigned_long) -> usize;
    fn pud_index(addr: unsigned_long) -> usize;
    fn pmd_index(addr: unsigned_long) -> usize;
    fn pmd_none(pmd: pmd_t) -> bool;
    fn memblock_alloc_low(size: unsigned_long, align: unsigned_long) -> *mut core::ffi::c_void;
    fn panic(format: *const u8, ...);
    fn set_pmd(pmdp: *mut pmd_t, pmd: pmd_t);
    fn __pmd(value: unsigned_long) -> pmd_t;
    fn pte_offset_kernel(pmd: *mut pmd_t, address: unsigned_long) -> *mut pte_t;
    fn __fix_to_virt(index: unsigned_long) -> unsigned_long;
}

// These aliases correspond to the kernel's architecture-sized unsigned long type.
type unsigned_long = usize;

#[no_mangle]
pub unsafe extern "C" fn free_initmem() {
    free_initmem_default(-1);
}

#[no_mangle]
pub unsafe extern "C" fn pgd_init(p: *mut unsigned_long) {
    let mut i = 0;
    while i < PTRS_PER_PGD {
        *p.add(i) = __pa(invalid_pte_table.as_ptr() as *const core::ffi::c_void);
        i += 1;
    }

    flush_tlb_all();
    local_icache_inv_all(core::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn mmu_init(min_pfn: unsigned_long, max_pfn: unsigned_long) {
    let mut i = 0;

    while i < USER_PTRS_PER_PGD {
        swapper_pg_dir[i].pgd = __pa(invalid_pte_table.as_ptr() as *const core::ffi::c_void);
        i += 1;
    }

    while i < PTRS_PER_PGD {
        swapper_pg_dir[i].pgd = __pa(
            kernel_pte_tables
                .as_ptr()
                .add(PTRS_PER_PTE * (i - USER_PTRS_PER_PGD)) as *const core::ffi::c_void,
        );
        i += 1;
    }

    i = 0;
    while i < PTRS_KERN_TABLE {
        set_pte(kernel_pte_tables.as_mut_ptr().add(i), __pte(_PAGE_GLOBAL));
        i += 1;
    }

    i = min_pfn;
    while i < max_pfn {
        set_pte(
            kernel_pte_tables
                .as_mut_ptr()
                .add(i - PFN_DOWN(va_pa_offset)),
            pfn_pte(i, PAGE_KERNEL),
        );
        i += 1;
    }

    flush_tlb_all();
    local_icache_inv_all(core::ptr::null_mut());

    // Setup page mask to 4k
    write_mmu_pagemask(0);

    setup_pgd(swapper_pg_dir.as_mut_ptr(), 0);
}

#[no_mangle]
pub unsafe extern "C" fn fixrange_init(
    start: unsigned_long,
    end: unsigned_long,
    pgd_base: *mut pgd_t,
) {
    let mut vaddr = start;
    let mut i = pgd_index(vaddr);
    let mut j = pud_index(vaddr);
    let mut k = pmd_index(vaddr);
    let mut pgd = pgd_base.add(i);

    while i < PTRS_PER_PGD && vaddr != end {
        let mut pud = pgd as *mut pud_t;
        while j < PTRS_PER_PUD && vaddr != end {
            let mut pmd = pud as *mut pmd_t;
            while k < PTRS_PER_PMD && vaddr != end {
                if pmd_none(*pmd) {
                    let pte = memblock_alloc_low(PAGE_SIZE, PAGE_SIZE) as *mut pte_t;
                    if pte.is_null() {
                        panic(
                            b"%s: Failed to allocate %lu bytes align=%lx\0".as_ptr(),
                            b"fixrange_init\0".as_ptr(),
                            PAGE_SIZE,
                            PAGE_SIZE,
                        );
                    }

                    set_pmd(pmd, __pmd(__pa(pte as *const core::ffi::c_void)));
                    BUG_ON(pte != pte_offset_kernel(pmd, 0));
                }
                vaddr += PMD_SIZE;
                pmd = pmd.add(1);
                k += 1;
            }
            pud = pud.add(1);
            j += 1;
            k = 0;
        }
        pgd = pgd.add(1);
        i += 1;
        j = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn fixaddr_init() {
    let vaddr = __fix_to_virt(__end_of_fixed_addresses - 1) & PMD_MASK;
    fixrange_init(vaddr, vaddr + PMD_SIZE, swapper_pg_dir.as_mut_ptr());
}

pub static protection_map: [pgprot_t; 16] = [
    PAGE_NONE, PAGE_READ, PAGE_READ, PAGE_READ,
    PAGE_READ, PAGE_READ, PAGE_READ, PAGE_READ,
    PAGE_NONE, PAGE_READ, PAGE_WRITE, PAGE_WRITE,
    PAGE_READ, PAGE_READ, PAGE_WRITE, PAGE_WRITE,
];

// DECLARE_VM_GET_PAGE_PROT

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
