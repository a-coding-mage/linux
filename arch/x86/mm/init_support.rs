// SPDX-License-Identifier: GPL-2.0-only
// Canonical configured types and native inline/macro ABIs shared by both owners.
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/x86_mm_init_generated.rs"
    ));
}
use b::*;
use bindings as b;
use core::cmp::{max, min};
use core::mem::{size_of, MaybeUninit};
use core::ptr::{addr_of, addr_of_mut, null_mut, write_bytes};
use kernel::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
const PAGE_SHIFT: u32 = b::RUST_MM_PAGE_SHIFT;
const PMD_SHIFT: u32 = b::RUST_MM_PMD_SHIFT;
const PAGE_SIZE: c_ulong = b::RUST_MM_PAGE_SIZE;
const PAGE_MASK: c_ulong = b::RUST_MM_PAGE_MASK;
const PMD_SIZE: c_ulong = b::RUST_MM_PMD_SIZE;
const PMD_MASK: c_ulong = b::RUST_MM_PMD_MASK;
const PUD_SIZE: c_ulong = b::RUST_MM_PUD_SIZE;
const PUD_MASK: c_ulong = b::RUST_MM_PUD_MASK;
const P4D_SIZE: c_ulong = b::RUST_MM_P4D_SIZE;
const P4D_MASK: c_ulong = b::RUST_MM_P4D_MASK;
const PTRS_PER_PTE: usize = b::RUST_MM_PTRS_PER_PTE as usize;
const PTRS_PER_PMD: usize = b::RUST_MM_PTRS_PER_PMD as usize;
const PTRS_PER_PUD: usize = b::RUST_MM_PTRS_PER_PUD as usize;
const PTRS_PER_PGD: usize = b::RUST_MM_PTRS_PER_PGD as usize;
const _PAGE_PWT: c_ulong = b::RUST_MM_PAGE_PWT;
const _PAGE_PCD: c_ulong = b::RUST_MM_PAGE_PCD;
const _PAGE_PAT: c_ulong = b::RUST_MM_PAGE_PAT;
const _PAGE_PSE: c_ulong = b::RUST_MM_PAGE_PSE;
const _PAGE_USER: c_ulong = b::RUST_MM_PAGE_USER;
const _PAGE_GLOBAL: c_ulong = b::RUST_MM_PAGE_GLOBAL;
const _PAGE_CACHE_MASK: c_ulong = b::RUST_MM_PAGE_CACHE_MASK;
const _PAGE_NOPTISHADOW: c_ulong = b::RUST_MM_PAGE_NOPTISHADOW;
const ENOMEM: c_int = b::ENOMEM as c_int;
#[inline(always)]
fn round_down(value: c_ulong, align: c_ulong) -> c_ulong {
    value & !align.wrapping_sub(1)
}
#[inline(always)]
fn round_up(value: c_ulong, align: c_ulong) -> c_ulong {
    (value.wrapping_sub(1) | align.wrapping_sub(1)).wrapping_add(1)
}
#[inline(always)]
unsafe fn bug_on(condition: bool) {
    if condition {
        b::rust_mm_bug();
    }
}
#[cfg(CONFIG_X86_64)]
#[inline(always)]
unsafe fn pgd_index(addr: c_ulong) -> usize {
    b::rust_mm_pgd_index(addr) as usize
}
#[cfg(CONFIG_X86_64)]
#[inline(always)]
unsafe fn p4d_index(addr: c_ulong) -> usize {
    b::rust_mm_p4d_index(addr) as usize
}
#[cfg(CONFIG_X86_64)]
#[inline(always)]
unsafe fn pud_index(addr: c_ulong) -> usize {
    b::rust_mm_pud_index(addr) as usize
}
#[cfg(CONFIG_X86_64)]
#[inline(always)]
unsafe fn pmd_index(addr: c_ulong) -> usize {
    b::rust_mm_pmd_index(addr) as usize
}
#[cfg(CONFIG_X86_64)]
#[inline(always)]
unsafe fn pte_index(addr: c_ulong) -> usize {
    b::rust_mm_pte_index(addr) as usize
}
#[cfg(CONFIG_X86_64)]
#[inline(always)]
unsafe fn ptrs_per_p4d() -> usize {
    b::rust_mm_ptrs_per_p4d() as usize
}
#[cfg(CONFIG_MEMORY_HOTPLUG)]
use b::rust_mm_PageReserved as PageReserved;
use b::rust_mm___flush_tlb_all as __flush_tlb_all;
#[cfg(CONFIG_X86_64)]
use b::rust_mm___p4d as __p4d;
use b::rust_mm___pa as __pa;
#[cfg(CONFIG_X86_64)]
use b::rust_mm___pgd as __pgd;
use b::rust_mm___pgprot as __pgprot;
#[cfg(CONFIG_X86_64)]
use b::rust_mm___pmd as __pmd;
#[cfg(CONFIG_X86_64)]
use b::rust_mm___pte as __pte;
#[cfg(CONFIG_X86_64)]
use b::rust_mm___pud as __pud;
use b::rust_mm___va as __va;
use b::rust_mm_alloc_low_after_bootmem as alloc_low_after_bootmem;
use b::rust_mm_boot_bug_l1tf as boot_bug_l1tf;
use b::rust_mm_boot_has_gbpages as boot_has_gbpages;
use b::rust_mm_boot_has_pcid as boot_has_pcid;
use b::rust_mm_boot_has_pge as boot_has_pge;
use b::rust_mm_boot_has_pse as boot_has_pse;
use b::rust_mm_clear_page as clear_page;
use b::rust_mm_cr4_set_bits as cr4_set_bits;
use b::rust_mm_debug_pagealloc_enabled as debug_pagealloc_enabled;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_direct_map_physmem_end as direct_map_physmem_end;
use b::rust_mm_early_memtest as early_memtest;
use b::rust_mm_feature_hypervisor as feature_hypervisor;
use b::rust_mm_feature_pse as feature_pse;
use b::rust_mm_feature_pti as feature_pti;
use b::rust_mm_flush_tlb_all as flush_tlb_all;
#[cfg(CONFIG_MEMORY_HOTPLUG)]
use b::rust_mm_free_reserved_page as free_reserved_page;
#[cfg(CONFIG_MEMORY_HOTPLUG)]
use b::rust_mm_free_reserved_pages as free_reserved_pages;
use b::rust_mm_get_locked_pte as get_locked_pte;
use b::rust_mm_get_order as get_order;
#[cfg(CONFIG_EXECMEM)]
use b::rust_mm_get_random_u32_inclusive as get_random_u32_inclusive;
use b::rust_mm_init_trampoline_kaslr as init_trampoline_kaslr;
use b::rust_mm_kaslr_enabled as kaslr_enabled;
use b::rust_mm_kaslr_memory_enabled as kaslr_memory_enabled;
use b::rust_mm_kclist_add as kclist_add;
use b::rust_mm_kernpg_table as kernpg_table;
use b::rust_mm_kmemleak_free_part as kmemleak_free_part;
use b::rust_mm_l1tf_pfn_limit as l1tf_pfn_limit;
use b::rust_mm_load_swapper_cr3 as load_swapper_cr3;
use b::rust_mm_mem_encrypt_free_decrypted_mem as mem_encrypt_free_decrypted_mem;
use b::rust_mm_memblock_alloc_page as memblock_alloc_page;
use b::rust_mm_memblock_bottom_up as memblock_bottom_up;
use b::rust_mm_memory_block_advised_max_size as memory_block_advised_max_size;
#[cfg(CONFIG_EXECMEM)]
use b::rust_mm_modules_end as modules_end;
#[cfg(CONFIG_EXECMEM)]
use b::rust_mm_modules_vaddr as modules_vaddr;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_native_pte_val as native_pte_val;
use b::rust_mm_node_clear_state as node_clear_state;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_p4d_addr_end as p4d_addr_end;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_p4d_alloc as p4d_alloc;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_p4d_clear as p4d_clear;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_p4d_none as p4d_none;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_p4d_offset as p4d_offset;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_p4d_page as p4d_page;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_p4d_pgtable as p4d_pgtable;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_p4d_populate as p4d_populate;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_p4d_populate_safe as p4d_populate_safe;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_p4d_present as p4d_present;
use b::rust_mm_pa_symbol as pa_symbol;
use b::rust_mm_page_kernel as page_kernel;
use b::rust_mm_page_kernel_large as page_kernel_large;
use b::rust_mm_page_kernel_rox as page_kernel_rox;
use b::rust_mm_page_offset_base as page_offset_base;
#[cfg(CONFIG_MEMORY_HOTPLUG)]
use b::rust_mm_page_ptdesc as page_ptdesc;
#[cfg(CONFIG_MEMORY_HOTPLUG)]
use b::rust_mm_pagetable_free as pagetable_free;
use b::rust_mm_paravirt_enter_mmap as paravirt_enter_mmap;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pfn_pmd as pfn_pmd;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pfn_pte as pfn_pte;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pfn_pud as pfn_pud;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pgd_addr_end as pgd_addr_end;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pgd_none as pgd_none;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pgd_offset_k as pgd_offset_k;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pgd_page_vaddr as pgd_page_vaddr;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pgd_populate as pgd_populate;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pgd_populate_safe as pgd_populate_safe;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pgd_present as pgd_present;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pgd_val as pgd_val;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pgdir_mask as pgdir_mask;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pgdir_size as pgdir_size;
use b::rust_mm_pgprot_val as pgprot_val;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pgtable_l5_enabled as pgtable_l5_enabled;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pmd_addr_end as pmd_addr_end;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pmd_clear as pmd_clear;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pmd_leaf as pmd_leaf;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pmd_none as pmd_none;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pmd_offset as pmd_offset;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pmd_page as pmd_page;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pmd_page_vaddr as pmd_page_vaddr;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pmd_populate_kernel as pmd_populate_kernel;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pmd_populate_kernel_safe as pmd_populate_kernel_safe;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pmd_present as pmd_present;
use b::rust_mm_protval_4k_2_large as protval_4k_2_large;
use b::rust_mm_ptdesc_address as ptdesc_address;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pte_clear as pte_clear;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pte_clrhuge as pte_clrhuge;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pte_none as pte_none;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pte_offset_kernel as pte_offset_kernel;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pte_page as pte_page;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pte_pgprot as pte_pgprot;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pte_present as pte_present;
use b::rust_mm_pte_unmap_unlock as pte_unmap_unlock;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pte_val as pte_val;
use b::rust_mm_pti_check_boottime_disable as pti_check_boottime_disable;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pud_addr_end as pud_addr_end;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pud_alloc as pud_alloc;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pud_clear as pud_clear;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pud_leaf as pud_leaf;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pud_none as pud_none;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pud_offset as pud_offset;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pud_page as pud_page;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pud_populate as pud_populate;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pud_populate_safe as pud_populate_safe;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_pud_present as pud_present;
use b::rust_mm_set_ftrace_ops_ro as set_ftrace_ops_ro;
use b::rust_mm_set_notrack_mm as set_notrack_mm;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_set_p4d as set_p4d;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_set_p4d_safe as set_p4d_safe;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_set_pgd as set_pgd;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_set_pmd as set_pmd;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_set_pmd_safe as set_pmd_safe;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_set_pte as set_pte;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_set_pte_safe as set_pte_safe;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_set_pud as set_pud;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_set_pud_safe as set_pud_safe;
use b::rust_mm_spin_lock as spin_lock;
use b::rust_mm_spin_unlock as spin_unlock;
use b::rust_mm_spp_getpage_late as spp_getpage_late;
use b::rust_mm_task_size as task_size;
use b::rust_mm_task_unmapped_base as task_unmapped_base;
use b::rust_mm_update_page_count as update_page_count;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_vmalloc_start as vmalloc_start;
#[cfg(CONFIG_X86_64)]
use b::rust_mm_vmemory_end as vmemory_end;
use b::{
    rust_mm_assert_folded_pgd as assert_folded_pgd,
    rust_mm_assert_no_p4d_leaf as assert_no_p4d_leaf,
    rust_mm_debug_init_mapping as debug_init_mapping, rust_mm_debug_range as debug_range,
    rust_mm_debug_set_pte_vaddr as debug_set_pte_vaddr,
    rust_mm_debug_spp_getpage as debug_spp_getpage,
    rust_mm_debug_vmemmap_block as debug_vmemmap_block,
    rust_mm_error_altmap_unsupported as error_altmap_unsupported, rust_mm_vm_bug_on as vm_bug_on,
    rust_mm_warn_add_pages_end as warn_add_pages_end,
    rust_mm_warn_add_pages_ret as warn_add_pages_ret,
    rust_mm_warn_free_init_alignment as warn_free_init_alignment,
    rust_mm_warn_prot_pat as warn_prot_pat,
};
