// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  This file contains pgtable related functions for 64-bit machines.
 *
 *  Derived from arch/ppc64/mm/init.c
 *    Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *
 *  Modifications by Paul Mackerras (PowerMac) (paulus@samba.org)
 *  and Cort Dougan (PReP) (cort@cs.nmt.edu)
 *    Copyright (C) 1996 Paul Mackerras
 *
 *  Derived from "arch/i386/mm/init.c"
 *    Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
 *
 *  Dave Engebretsen <engebret@us.ibm.com>
 *      Rework for PPC64 port.
 */

// C header dependencies are supplied by the surrounding kernel translation.

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
pub static mut process_tb: *mut prtb_entry = core::ptr::null_mut();
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
pub static mut partition_tb: *mut patb_entry = core::ptr::null_mut();

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __pte_index_size: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __pmd_index_size: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __pud_index_size: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __pgd_index_size: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __pud_cache_index: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __pte_table_size: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __pmd_table_size: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __pud_table_size: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __pgd_table_size: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __pmd_val_bits: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __pud_val_bits: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __pgd_val_bits: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __kernel_virt_start: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __vmalloc_start: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __vmalloc_end: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __kernel_io_start: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
pub static mut __kernel_io_end: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut vmemmap: *mut page = core::ptr::null_mut();
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __pte_frag_nr: c_ulong = 0;
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[no_mangle] pub static mut __pte_frag_size_shift: c_ulong = 0;

#[cfg(not(feature = "__PAGETABLE_PUD_FOLDED"))]
pub unsafe fn p4d_page(p4d: p4d_t) -> *mut page {
    if p4d_leaf(p4d) {
        if !cfg!(feature = "CONFIG_HAVE_ARCH_HUGE_VMAP") {
            VM_WARN_ON(!p4d_leaf(p4d));
        }
        return pte_page(p4d_pte(p4d));
    }
    virt_to_page(p4d_pgtable(p4d))
}

pub unsafe fn pud_page(pud: pud_t) -> *mut page {
    if pud_leaf(pud) {
        if !cfg!(feature = "CONFIG_HAVE_ARCH_HUGE_VMAP") {
            VM_WARN_ON(!pud_leaf(pud));
        }
        return pte_page(pud_pte(pud));
    }
    virt_to_page(pud_pgtable(pud))
}

/*
 * For hugepage we have pfn in the pmd, we use PTE_RPN_SHIFT bits for flags
 * For PTE page, we have a PTE_FRAG_SIZE (4K) aligned virtual address.
 */
pub unsafe fn pmd_page(pmd: pmd_t) -> *mut page {
    if pmd_leaf(pmd) {
        /*
         * vmalloc_to_page may be called on any vmap address (not only
         * vmalloc), and it uses pmd_page() etc., when huge vmap is
         * enabled so these checks can't be used.
         */
        if !cfg!(feature = "CONFIG_HAVE_ARCH_HUGE_VMAP") {
            VM_WARN_ON(!pmd_leaf(pmd));
        }
        return pte_page(pmd_pte(pmd));
    }
    virt_to_page(pmd_page_vaddr(pmd))
}

#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")]
pub unsafe fn mark_rodata_ro() {
    if !mmu_has_feature(MMU_FTR_KERNEL_RO) {
        pr_warn("Warning: Unable to mark rodata read only on this CPU.\n");
        return;
    }

    if radix_enabled() {
        radix__mark_rodata_ro();
    } else {
        hash__mark_rodata_ro();
    }
}

#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")]
pub unsafe fn mark_initmem_nx() {
    if radix_enabled() {
        radix__mark_initmem_nx();
    } else {
        hash__mark_initmem_nx();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
