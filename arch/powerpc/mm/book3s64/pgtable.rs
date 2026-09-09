// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015-2016, Aneesh Kumar K.V, IBM Corporation.
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct mmu_psize_def { pub _opaque: [u8; 0] }
pub static mut mmu_psize_defs: [mmu_psize_def; MMU_PAGE_COUNT] = [mmu_psize_def { _opaque: [] }; MMU_PAGE_COUNT];
pub static mut __pmd_frag_nr: c_ulong = 0;
pub static mut __pmd_frag_size_shift: c_ulong = 0;

#[cfg(feature = "CONFIG_SPARSEMEM_VMEMMAP")]
pub static mut mmu_vmemmap_psize: c_int = MMU_PAGE_4K;

#[cfg(feature = "CONFIG_KFENCE")]
extern "C" {
    static mut kfence_early_init: bool;
}

#[cfg(feature = "CONFIG_KFENCE")]
unsafe extern "C" fn parse_kfence_early_init(arg: *mut c_char) -> c_int {
    let mut val: c_int = 0;
    if get_option(&arg, &mut val) != 0 { kfence_early_init = val != 0; }
    0
}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn pmdp_set_access_flags(vma: *mut vm_area_struct, address: c_ulong, pmdp: *mut pmd_t, entry: pmd_t, _dirty: c_int) -> c_int {
    let changed = (!pmd_same(*pmdp, entry)) as c_int;
    if changed != 0 { __ptep_set_access_flags(vma, pmdp_ptep(pmdp), pmd_pte(entry), address, MMU_PAGE_2M); }
    changed
}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn pudp_set_access_flags(vma: *mut vm_area_struct, address: c_ulong, pudp: *mut pud_t, entry: pud_t, _dirty: c_int) -> c_int {
    let changed = (!pud_same(*pudp, entry)) as c_int;
    if changed != 0 { __ptep_set_access_flags(vma, pudp_ptep(pudp), pud_pte(entry), address, MMU_PAGE_1G); }
    changed
}

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn pmdp_test_and_clear_young(vma: *mut vm_area_struct, address: c_ulong, pmdp: *mut pmd_t) -> bool { __pmdp_test_and_clear_young((*vma).vm_mm, address, pmdp) }
#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn pudp_test_and_clear_young(vma: *mut vm_area_struct, address: c_ulong, pudp: *mut pud_t) -> bool { __pudp_test_and_clear_young((*vma).vm_mm, address, pudp) }

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn set_pmd_at(mm: *mut mm_struct, addr: c_ulong, pmdp: *mut pmd_t, pmd: pmd_t) { trace_hugepage_set_pmd(addr, pmd_val(pmd)); page_table_check_pmd_set(mm, addr, pmdp, pmd); set_pte_at_unchecked(mm, addr, pmdp_ptep(pmdp), pmd_pte(pmd)); }
#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn set_pud_at(mm: *mut mm_struct, addr: c_ulong, pudp: *mut pud_t, pud: pud_t) { trace_hugepage_set_pud(addr, pud_val(pud)); page_table_check_pud_set(mm, addr, pudp, pud); set_pte_at_unchecked(mm, addr, pudp_ptep(pudp), pud_pte(pud)); }

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn pmdp_invalidate(vma: *mut vm_area_struct, address: c_ulong, pmdp: *mut pmd_t) -> pmd_t { let old = __pmd(pmd_hugepage_update((*vma).vm_mm, address, pmdp, _PAGE_PRESENT, _PAGE_INVALID)); flush_pmd_tlb_range(vma, address, address + HPAGE_PMD_SIZE); page_table_check_pmd_clear((*vma).vm_mm, address, old); old }
#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn pudp_invalidate(vma: *mut vm_area_struct, address: c_ulong, pudp: *mut pud_t) -> pud_t { let old = __pud(pud_hugepage_update((*vma).vm_mm, address, pudp, _PAGE_PRESENT, _PAGE_INVALID)); flush_pud_tlb_range(vma, address, address + HPAGE_PUD_SIZE); page_table_check_pud_clear((*vma).vm_mm, address, old); old }

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn pmdp_huge_get_and_clear_full(vma: *mut vm_area_struct, addr: c_ulong, pmdp: *mut pmd_t, full: c_int) -> pmd_t { let was_present = pmd_present(*pmdp); let pmd = pmdp_huge_get_and_clear((*vma).vm_mm, addr, pmdp); if was_present && full == 0 { flush_pmd_tlb_range(vma, addr, addr + HPAGE_PMD_SIZE); } pmd }
#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn pudp_huge_get_and_clear_full(vma: *mut vm_area_struct, addr: c_ulong, pudp: *mut pud_t, full: c_int) -> pud_t { let pud = pudp_huge_get_and_clear((*vma).vm_mm, addr, pudp); if full == 0 { flush_pud_tlb_range(vma, addr, addr + HPAGE_PUD_SIZE); } pud }

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
unsafe fn pmd_set_protbits(pmd: pmd_t, pgprot: pgprot_t) -> pmd_t { __pmd(pmd_val(pmd) | pgprot_val(pgprot)) }
#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
unsafe fn pud_set_protbits(pud: pud_t, pgprot: pgprot_t) -> pud_t { __pud(pud_val(pud) | pgprot_val(pgprot)) }
#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn pfn_pmd(pfn: c_ulong, pgprot: pgprot_t) -> pmd_t { __pmd_mkhuge(pmd_set_protbits(__pmd((pfn << PAGE_SHIFT) & PTE_RPN_MASK), pgprot)) }
#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn pfn_pud(pfn: c_ulong, pgprot: pgprot_t) -> pud_t { __pud_mkhuge(pud_set_protbits(__pud((pfn << PAGE_SHIFT) & PTE_RPN_MASK), pgprot)) }
#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn pmd_modify(pmd: pmd_t, newprot: pgprot_t) -> pmd_t { pmd_set_protbits(__pmd(pmd_val(pmd) & _HPAGE_CHG_MASK), newprot) }
#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn pud_modify(pud: pud_t, newprot: pgprot_t) -> pud_t { pud_set_protbits(__pud(pud_val(pud) & _HPAGE_CHG_MASK), newprot) }

pub unsafe extern "C" fn mmu_cleanup_all() { if radix_enabled() { radix__mmu_cleanup_all(); } else if mmu_hash_ops.hpte_clear_all.is_some() { (mmu_hash_ops.hpte_clear_all.unwrap())(); } reset_sprs(); }

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
pub unsafe extern "C" fn create_section_mapping(start: c_ulong, end: c_ulong, nid: c_int, prot: pgprot_t) -> c_int { if radix_enabled() { radix__create_section_mapping(start, end, nid, prot) } else { hash__create_section_mapping(start, end, nid, prot) } }
#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
pub unsafe extern "C" fn remove_section_mapping(start: c_ulong, end: c_ulong) -> c_int { if radix_enabled() { radix__remove_section_mapping(start, end) } else { hash__remove_section_mapping(start, end) } }

pub unsafe extern "C" fn mmu_partition_table_init() { let patb_size = 1usize << PATB_SIZE_SHIFT; let ptcr; partition_tb = memblock_alloc_or_panic(patb_size, patb_size); ptcr = __pa(partition_tb) | (PATB_SIZE_SHIFT - 12); set_ptcr_when_no_uv(ptcr); powernv_set_nmmu_ptcr(ptcr); }

unsafe fn flush_partition(lpid: c_uint, radix: bool) { if radix { radix__flush_all_lpid(lpid); radix__flush_all_lpid_guest(lpid); } else { asm!("ptesync", options(nostack)); asm!("eieio; tlbsync; ptesync", options(nostack)); trace_tlbie(lpid, 0, TLBIEL_INVAL_SET_LPID, lpid, 2, 0, 0); } }

pub unsafe extern "C" fn mmu_partition_table_set_entry(lpid: c_uint, dw0: c_ulong, dw1: c_ulong, flush: bool) { let old = be64_to_cpu(partition_tb[lpid as usize].patb0); partition_tb[lpid as usize].patb0 = cpu_to_be64(dw0); partition_tb[lpid as usize].patb1 = cpu_to_be64(dw1); if firmware_has_feature(FW_FEATURE_ULTRAVISOR) { uv_register_pate(lpid, dw0, dw1); pr_info!("PATE registered by ultravisor: dw0 = 0x%lx, dw1 = 0x%lx\n", dw0, dw1); } else if flush { flush_partition(lpid, (old & PATB_HR) != 0); } }

unsafe fn get_pmd_from_cache(mm: *mut mm_struct) -> *mut pmd_t { if PMD_FRAG_NR == 1 { return core::ptr::null_mut(); } spin_lock(&mut (*mm).page_table_lock); let ret = (*mm).context.pmd_frag; if !ret.is_null() { let mut frag = ret.add(PMD_FRAG_SIZE as usize); if ((frag as usize) & !(PAGE_MASK as usize)) == 0 { frag = core::ptr::null_mut(); } (*mm).context.pmd_frag = frag; } spin_unlock(&mut (*mm).page_table_lock); ret as *mut pmd_t }
unsafe fn __alloc_for_pmdcache(mm: *mut mm_struct) -> *mut pmd_t { let mut gfp = GFP_KERNEL_ACCOUNT | __GFP_ZERO; if mm == &mut init_mm { gfp &= !__GFP_ACCOUNT; } let ptdesc = pagetable_alloc(gfp, 0); if ptdesc.is_null() { return core::ptr::null_mut(); } if !pagetable_pmd_ctor(mm, ptdesc) { pagetable_free(ptdesc); return core::ptr::null_mut(); } atomic_set(&mut (*ptdesc).pt_frag_refcount, 1); let ret = ptdesc_address(ptdesc); if PMD_FRAG_NR != 1 { spin_lock(&mut (*mm).page_table_lock); if likely((*mm).context.pmd_frag.is_null()) { atomic_set(&mut (*ptdesc).pt_frag_refcount, PMD_FRAG_NR); (*mm).context.pmd_frag = (ret as *mut u8).add(PMD_FRAG_SIZE as usize) as *mut c_void; } spin_unlock(&mut (*mm).page_table_lock); } ret as *mut pmd_t }
pub unsafe extern "C" fn pmd_fragment_alloc(mm: *mut mm_struct, _vmaddr: c_ulong) -> *mut pmd_t { let pmd = get_pmd_from_cache(mm); if !pmd.is_null() { pmd } else { __alloc_for_pmdcache(mm) } }
pub unsafe extern "C" fn pmd_fragment_free(pmd: *mut c_ulong) { let ptdesc = virt_to_ptdesc(pmd as *mut c_void); if pagetable_is_reserved(ptdesc) { free_reserved_ptdesc(ptdesc); return; } BUG_ON(atomic_read(&(*ptdesc).pt_frag_refcount) <= 0); if atomic_dec_and_test(&mut (*ptdesc).pt_frag_refcount) { pagetable_dtor(ptdesc); pagetable_free(ptdesc); } }

unsafe fn pgtable_free(table: *mut c_void, index: c_int) { match index { PTE_INDEX => pte_fragment_free(table, 0), PMD_INDEX => pmd_fragment_free(table as *mut c_ulong), PUD_INDEX => __pud_free(table), _ => BUG(), } }
pub unsafe extern "C" fn pgtable_free_tlb(tlb: *mut mmu_gather, table: *mut c_void, index: c_int) { BUG_ON(index > MAX_PGTABLE_INDEX_SIZE); tlb_remove_table(tlb, (table as usize | index as usize) as *mut c_void); }
pub unsafe extern "C" fn __tlb_remove_table(table: *mut c_void) { pgtable_free((table as usize & !(MAX_PGTABLE_INDEX_SIZE as usize)) as *mut c_void, table as usize & MAX_PGTABLE_INDEX_SIZE as usize); }

pub unsafe extern "C" fn ptep_modify_prot_start(vma: *mut vm_area_struct, addr: c_ulong, ptep: *mut pte_t) -> pte_t { __pte(pte_update((*vma).vm_mm, addr, ptep, _PAGE_PRESENT, _PAGE_INVALID, 0)) }
pub unsafe extern "C" fn ptep_modify_prot_commit(vma: *mut vm_area_struct, addr: c_ulong, ptep: *mut pte_t, old_pte: pte_t, pte: pte_t) { if radix_enabled() { radix__ptep_modify_prot_commit(vma, addr, ptep, old_pte, pte); } else { set_pte_at_unchecked((*vma).vm_mm, addr, ptep, pte); } }

#[cfg(feature = "CONFIG_TRANSPARENT_HUGEPAGE")]
pub unsafe extern "C" fn pmd_move_must_withdraw(new_pmd_ptl: *mut spinlock, old_pmd_ptl: *mut spinlock, vma: *mut vm_area_struct) -> c_int { if radix_enabled() { ((new_pmd_ptl != old_pmd_ptl) && vma_is_anonymous(vma)) as c_int } else { 1 } }

pub static mut tlbie_capable: bool = cfg!(feature = "CONFIG_PPC_RADIX_BROADCAST_TLBIE");
pub static mut tlbie_enabled: bool = cfg!(feature = "CONFIG_PPC_RADIX_BROADCAST_TLBIE");

#[cfg(any())]
pub unsafe extern "C" fn setup_disable_tlbie(_str: *mut c_char) -> c_int { if !radix_enabled() { pr_err!("disable_tlbie: Unable to disable TLBIE with Hash MMU.\n"); return 1; } tlbie_capable = false; tlbie_enabled = false; 1 }

pub unsafe extern "C" fn pgtable_debugfs_setup() -> c_int { if !tlbie_capable { return 0; } debugfs_create_bool(c"tlbie_enabled".as_ptr(), 0o600, arch_debugfs_dir, &mut tlbie_enabled); 0 }

#[cfg(all(feature = "CONFIG_ZONE_DEVICE", feature = "CONFIG_ARCH_HAS_MEMREMAP_COMPAT_ALIGN"))]
pub unsafe extern "C" fn memremap_compat_align() -> c_ulong { if !radix_enabled() { let shift = mmu_psize_defs[mmu_linear_psize as usize].shift; return max(SUBSECTION_SIZE, 1usize << shift) as c_ulong; } SUBSECTION_SIZE }

pub unsafe extern "C" fn vm_get_page_prot(mut vm_flags: vm_flags_t) -> pgprot_t { if !radix_enabled() && (vm_flags & VM_ACCESS_FLAGS) == VM_EXEC { vm_flags |= VM_READ; } let mut prot = pgprot_val(protection_map[(vm_flags & (VM_ACCESS_FLAGS | VM_SHARED)) as usize]); if vm_flags & VM_SAO != 0 { prot |= _PAGE_SAO; } prot |= vmflag_to_pte_pkey_bits(vm_flags); __pgprot(prot) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
