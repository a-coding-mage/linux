// SPDX-License-Identifier: GPL-2.0-only
/*
 * Hibernation support for x86-64
 *
 * Copyright (c) 2007 Rafael J. Wysocki <rjw@sisk.pl>
 * Copyright (c) 2002 Pavel Machek <pavel@ucw.cz>
 * Copyright (c) 2001 Patrick Mochel <mochel@osdl.org>
 */

// Linux kernel headers and architecture dependencies are supplied externally.

extern "C" {
    static mut temp_pgt: ::core::ffi::c_ulong;
    static nr_pfn_mapped: ::core::ffi::c_int;
    static mut pfn_mapped: [PfnMapped; 0];
    static restore_jump_address: ::core::ffi::c_ulong;
    static jump_address_phys: ::core::ffi::c_ulong;
    static __default_kernel_pte_mask: ::core::ffi::c_ulong;

    fn pgtable_l5_enabled() -> bool;
    fn get_safe_page(flags: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
    fn set_pmd(entry: *mut Pmd, value: Pmd);
    fn set_pud(entry: *mut Pud, value: Pud);
    fn set_p4d(entry: *mut P4d, value: P4d);
    fn set_pgd(entry: *mut Pgd, value: Pgd);
    fn kernel_ident_mapping_init(
        info: *mut X86MappingInfo,
        pgd: *mut Pgd,
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    fn relocate_restore_code() -> ::core::ffi::c_int;
    fn restore_image();
}

// These types, constructors, accessors, constants, and macros are provided by
// the corresponding kernel headers.
use crate::{
    __PAGE_KERNEL_LARGE_EXEC, __PAGE_OFFSET, __KERNPG_TABLE, GFP_ATOMIC,
    PAGE_SHIFT, PMD_MASK, P4d, Pgd, Pmd, Pud, PfnMapped, X86MappingInfo,
    __default_kernel_pte_mask, __pa, __p4d, __pgd, __pmd, __pgprot,
    pgprot_val, pgd_index, p4d_index, pfn_mapped, pmd_index, pud_index,
};

unsafe fn set_up_temporary_text_mapping(pgd: *mut Pgd) -> ::core::ffi::c_int {
    let pmd: *mut Pmd;
    let pud: *mut Pud;
    let mut p4d: *mut P4d = core::ptr::null_mut();
    let mut pgtable_prot = __pgprot(__KERNPG_TABLE);
    let mut pmd_text_prot = __pgprot(__PAGE_KERNEL_LARGE_EXEC);

    /* Filter out unsupported __PAGE_KERNEL* bits: */
    pgprot_val(&mut pmd_text_prot) &= __default_kernel_pte_mask;
    pgprot_val(&mut pgtable_prot) &= __default_kernel_pte_mask;

    /*
     * The new mapping only has to cover the page containing the image
     * kernel's entry point (jump_address_phys), because the switch over to
     * it is carried out by relocated code running from a page allocated
     * specifically for this purpose and covered by the identity mapping, so
     * the temporary kernel text mapping is only needed for the final jump.
     * Moreover, in that mapping the virtual address of the image kernel's
     * entry point must be the same as its virtual address in the image
     * kernel (restore_jump_address), so the image kernel's restore_registers()
     * code doesn't find itself in a different area of the virtual address
     * space after switching over to the original page tables used by the image
     * kernel.
     */
    if pgtable_l5_enabled() {
        p4d = get_safe_page(GFP_ATOMIC) as *mut P4d;
        if p4d.is_null() { return -12; }
    }

    pud = get_safe_page(GFP_ATOMIC) as *mut Pud;
    if pud.is_null() { return -12; }

    pmd = get_safe_page(GFP_ATOMIC) as *mut Pmd;
    if pmd.is_null() { return -12; }

    set_pmd(pmd.add(pmd_index(restore_jump_address) as usize),
        __pmd((jump_address_phys & PMD_MASK) | pgprot_val(&mut pmd_text_prot)));
    set_pud(pud.add(pud_index(restore_jump_address) as usize),
        __pud(__pa(pmd) | pgprot_val(&mut pgtable_prot)));
    if !p4d.is_null() {
        let new_p4d = __p4d(__pa(pud) | pgprot_val(&mut pgtable_prot));
        let new_pgd = __pgd(__pa(p4d) | pgprot_val(&mut pgtable_prot));
        set_p4d(p4d.add(p4d_index(restore_jump_address) as usize), new_p4d);
        set_pgd(pgd.add(pgd_index(restore_jump_address) as usize), new_pgd);
    } else {
        /* No p4d for 4-level paging: point the pgd to the pud page table */
        let new_pgd = __pgd(__pa(pud) | pgprot_val(&mut pgtable_prot));
        set_pgd(pgd.add(pgd_index(restore_jump_address) as usize), new_pgd);
    }

    0
}

unsafe fn alloc_pgt_page(_context: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    get_safe_page(GFP_ATOMIC)
}

unsafe fn set_up_temporary_mappings() -> ::core::ffi::c_int {
    let mut info = X86MappingInfo {
        alloc_pgt_page: Some(alloc_pgt_page),
        page_flag: __PAGE_KERNEL_LARGE_EXEC,
        offset: __PAGE_OFFSET,
    };
    let mut pgd: *mut Pgd;

    pgd = get_safe_page(GFP_ATOMIC) as *mut Pgd;
    if pgd.is_null() { return -12; }

    /* Prepare a temporary mapping for the kernel text */
    let mut result = set_up_temporary_text_mapping(pgd);
    if result != 0 { return result; }

    /* Set up the direct mapping from scratch */
    for i in 0..nr_pfn_mapped {
        let mstart = pfn_mapped[i as usize].start << PAGE_SHIFT;
        let mend = pfn_mapped[i as usize].end << PAGE_SHIFT;
        result = kernel_ident_mapping_init(&mut info, pgd, mstart, mend);
        if result != 0 { return result; }
    }

    temp_pgt = __pa(pgd);
    0
}

#[no_mangle]
pub unsafe extern "C" fn swsusp_arch_resume() -> ::core::ffi::c_int {
    /* We have got enough memory and from now on we cannot recover */
    let mut error = set_up_temporary_mappings();
    if error != 0 { return error; }

    error = relocate_restore_code();
    if error != 0 { return error; }

    restore_image();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
