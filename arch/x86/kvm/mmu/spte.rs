// SPDX-License-Identifier: GPL-2.0-only
/* Kernel-based Virtual Machine driver for Linux
 * Macros and functions to access KVM PTEs (also known as SPTEs) */

pub static mut enable_mmio_caching: bool = true;
static mut allow_mmio_caching: bool = false;
pub static mut kvm_ad_enabled: bool = false;
pub static mut shadow_host_writable_mask: u64 = 0;
pub static mut shadow_mmu_writable_mask: u64 = 0;
pub static mut shadow_nx_mask: u64 = 0;
pub static mut shadow_user_mask: u64 = 0;
pub static mut shadow_xs_mask: u64 = 0;
pub static mut shadow_xu_mask: u64 = 0;
pub static mut shadow_accessed_mask: u64 = 0;
pub static mut shadow_dirty_mask: u64 = 0;
pub static mut shadow_mmio_value: u64 = 0;
pub static mut shadow_mmio_mask: u64 = 0;
pub static mut shadow_mmio_access_mask: u64 = 0;
pub static mut shadow_present_mask: u64 = 0;
pub static mut shadow_me_value: u64 = 0;
pub static mut shadow_me_mask: u64 = 0;
pub static mut shadow_acc_track_mask: u64 = 0;
pub static mut shadow_nonpresent_or_rsvd_mask: u64 = 0;
pub static mut shadow_nonpresent_or_rsvd_lower_gfn_mask: u64 = 0;

unsafe fn kvm_get_host_maxphyaddr() -> u8 {
    /* boot_cpu_data.x86_phys_bits is reduced for MKTME/SME; use raw MAXPHYADDR. */
    if likely(boot_cpu_data.extended_cpuid_level >= 0x80000008) {
        return (cpuid_eax(0x80000008) & 0xff) as u8;
    }
    boot_cpu_data.x86_phys_bits
}

pub unsafe fn kvm_mmu_spte_module_init() {
    allow_mmio_caching = enable_mmio_caching;
    kvm_host.maxphyaddr = kvm_get_host_maxphyaddr();
}

unsafe fn generation_mmio_spte_mask(gen: u64) -> u64 {
    WARN_ON_ONCE(gen & !MMIO_SPTE_GEN_MASK);
    let mut mask = (gen << MMIO_SPTE_GEN_LOW_SHIFT) & MMIO_SPTE_GEN_LOW_MASK;
    mask |= (gen << MMIO_SPTE_GEN_HIGH_SHIFT) & MMIO_SPTE_GEN_HIGH_MASK;
    mask
}

pub unsafe fn make_mmio_spte(vcpu: *mut kvm_vcpu, gfn: u64, mut access: u32) -> u64 {
    let gen = (*kvm_vcpu_memslots(vcpu)).generation & MMIO_SPTE_GEN_MASK;
    let mut spte = generation_mmio_spte_mask(gen);
    let gpa = gfn << PAGE_SHIFT;
    access &= shadow_mmio_access_mask as u32;
    spte |= (*vcpu).kvm.as_ref().unwrap().arch.shadow_mmio_value | access as u64;
    spte |= gpa | shadow_nonpresent_or_rsvd_mask;
    spte |= (gpa & shadow_nonpresent_or_rsvd_mask) << SHADOW_NONPRESENT_OR_RSVD_MASK_LEN;
    spte
}

unsafe fn __kvm_is_mmio_pfn(pfn: kvm_pfn_t) -> bool {
    if pfn_valid(pfn) {
        return !is_zero_pfn(pfn) && PageReserved(pfn_to_page(pfn)) &&
            (!pat_enabled() || pat_pfn_immune_to_uc_mtrr(pfn));
    }
    !e820__mapped_raw_any(pfn_to_hpa(pfn), pfn_to_hpa(pfn + 1) - 1, E820_TYPE_RAM)
}

unsafe fn kvm_is_mmio_pfn(pfn: kvm_pfn_t, is_host_mmio: *mut i32) -> bool {
    if *is_host_mmio < 0 { *is_host_mmio = __kvm_is_mmio_pfn(pfn) as i32; }
    *is_host_mmio != 0
}

unsafe fn kvm_track_host_mmio_mapping(vcpu: *mut kvm_vcpu) {
    let root = root_to_sp((*vcpu).arch.mmu.as_ref().unwrap().root.hpa);
    if !root.is_null() { WRITE_ONCE((*root).has_mapped_host_mmio, true); }
    else { WRITE_ONCE((*vcpu).kvm.as_ref().unwrap().arch.has_mapped_host_mmio, true); }
    kvm_make_all_cpus_request((*vcpu).kvm, KVM_REQ_OUTSIDE_GUEST_MODE);
}

pub unsafe fn spte_needs_atomic_update(spte: u64) -> bool {
    if !is_writable_pte(spte) && is_mmu_writable_spte(spte) { return true; }
    if !spte_ad_enabled(spte) { return true; }
    is_writable_pte(spte) && (spte & shadow_dirty_mask) == 0
}

pub unsafe fn make_spte(vcpu: *mut kvm_vcpu, sp: *mut kvm_mmu_page,
    slot: *const kvm_memory_slot, mut pte_access: u32, gfn: u64, pfn: kvm_pfn_t,
    old_spte: u64, prefetch: bool, synchronizing: bool, host_writable: bool,
    new_spte: *mut u64) -> bool {
    let level = (*sp).role.level;
    let mut spte = SPTE_MMU_PRESENT_MASK;
    let mut is_host_mmio: i32 = -1;
    let mut wrprot = false;
    WARN_ON_ONCE(((pte_access as u64) | shadow_present_mask) == SHADOW_NONPRESENT_VALUE);
    if (*sp).role.ad_disabled { spte |= SPTE_TDP_AD_DISABLED; }
    else if kvm_mmu_page_ad_need_write_protect((*vcpu).kvm, sp) { spte |= SPTE_TDP_AD_WRPROT_ONLY; }
    spte |= shadow_present_mask;
    if !prefetch || synchronizing { spte |= shadow_accessed_mask; }
    if level > PG_LEVEL_4K && is_nx_huge_page_enabled((*vcpu).kvm) {
        pte_access &= !ACC_EXEC_MASK;
        if shadow_xu_mask != 0 { pte_access &= !ACC_USER_EXEC_MASK; }
    }
    if pte_access & ACC_READ_MASK != 0 { spte |= PT_PRESENT_MASK; }
    if shadow_nx_mask != 0 {
        if pte_access & ACC_EXEC_MASK == 0 { spte |= shadow_nx_mask; }
        if pte_access & ACC_USER_MASK != 0 { spte |= shadow_user_mask; }
    } else {
        if pte_access & ACC_EXEC_MASK != 0 { spte |= shadow_xs_mask; }
        if pte_access & ACC_USER_EXEC_MASK != 0 { spte |= shadow_xu_mask; }
    }
    if level > PG_LEVEL_4K { spte |= PT_PAGE_SIZE_MASK; }
    if !kvm_x86_ops.get_mt_mask.is_none() { spte |= kvm_x86_call(get_mt_mask)(vcpu, gfn, kvm_is_mmio_pfn(pfn, &mut is_host_mmio)); }
    if host_writable { spte |= shadow_host_writable_mask; } else { pte_access &= !ACC_WRITE_MASK; }
    if shadow_me_value != 0 && !kvm_is_mmio_pfn(pfn, &mut is_host_mmio) { spte |= shadow_me_value; }
    spte |= (pfn as u64) << PAGE_SHIFT;
    if pte_access & ACC_WRITE_MASK != 0 {
        if (!is_last_spte(old_spte, level) || !is_writable_pte(old_spte)) && mmu_try_to_unsync_pages((*vcpu).kvm, slot, gfn, synchronizing, prefetch) { wrprot = true; }
        else { spte |= PT_WRITABLE_MASK | shadow_mmu_writable_mask | shadow_dirty_mask; }
    }
    if prefetch && !synchronizing { spte = mark_spte_for_access_track(spte); }
    WARN_ONCE(is_rsvd_spte(&(*vcpu).arch.mmu.as_ref().unwrap().fmt, spte, level), "spte = 0x%llx, level = %d", spte, level);
    if spte & PT_WRITABLE_MASK != 0 && kvm_slot_dirty_track_enabled(slot) { WARN_ON_ONCE(level > PG_LEVEL_4K); mark_page_dirty_in_slot((*vcpu).kvm, slot, gfn); }
    if cpu_feature_enabled(X86_FEATURE_CLEAR_CPU_BUF_VM_MMIO) && !kvm_vcpu_can_access_host_mmio(vcpu) && kvm_is_mmio_pfn(pfn, &mut is_host_mmio) { kvm_track_host_mmio_mapping(vcpu); }
    *new_spte = spte; wrprot
}

unsafe fn modify_spte_protections(mut spte: u64, set: u64, clear: u64) -> u64 {
    let tracked = is_access_track_spte(spte);
    if tracked { spte = restore_acc_track_spte(spte); }
    KVM_MMU_WARN_ON(set & clear); spte = (spte | set) & !clear;
    if tracked { spte = mark_spte_for_access_track(spte); } spte
}

unsafe fn change_spte_executable(spte: u64, access: u8) -> u64 {
    let set = if shadow_nx_mask != 0 { if access & ACC_EXEC_MASK as u8 != 0 { 0 } else { shadow_nx_mask } } else { if access & ACC_EXEC_MASK as u8 != 0 { 0 } else { shadow_xs_mask } | if access & ACC_USER_EXEC_MASK as u8 != 0 { shadow_xu_mask } else { 0 } };
    modify_spte_protections(spte, set, set ^ (shadow_nx_mask | shadow_xs_mask | shadow_xu_mask))
}

pub unsafe fn make_small_spte(kvm: *mut kvm, huge_spte: u64, role: kvm_mmu_page_role, index: i32) -> u64 {
    let mut child = huge_spte; KVM_BUG_ON(!is_shadow_present_pte(huge_spte) || !is_large_pte(huge_spte), kvm);
    child |= ((index * KVM_PAGES_PER_HPAGE(role.level)) as u64) << PAGE_SHIFT;
    if role.level == PG_LEVEL_4K { child &= !PT_PAGE_SIZE_MASK; if is_nx_huge_page_enabled(kvm) { child = change_spte_executable(child, role.access); } }
    child
}

pub unsafe fn make_huge_spte(kvm: *mut kvm, small_spte: u64, level: i32) -> u64 {
    KVM_BUG_ON(!is_shadow_present_pte(small_spte) || level == PG_LEVEL_4K, kvm);
    let mut huge = (small_spte | PT_PAGE_SIZE_MASK) & (KVM_HPAGE_MASK(level) | !PAGE_MASK);
    if is_nx_huge_page_enabled(kvm) { huge = change_spte_executable(huge, 0); } huge
}

pub unsafe fn make_nonleaf_spte(child_pt: *mut u64, ad_disabled: bool) -> u64 {
    let mut spte = SPTE_MMU_PRESENT_MASK | __pa(child_pt) | shadow_present_mask | PT_WRITABLE_MASK | PT_PRESENT_MASK | shadow_user_mask | shadow_xs_mask | shadow_xu_mask | shadow_me_value;
    if ad_disabled { spte |= SPTE_TDP_AD_DISABLED; } else { spte |= shadow_accessed_mask; } spte
}

pub unsafe fn mark_spte_for_access_track(mut spte: u64) -> u64 {
    if spte_ad_enabled(spte) { return spte & !shadow_accessed_mask; }
    if is_access_track_spte(spte) { return spte; }
    check_spte_writable_invariants(spte);
    WARN_ONCE(spte & (SHADOW_ACC_TRACK_SAVED_BITS_MASK << SHADOW_ACC_TRACK_SAVED_BITS_SHIFT) != 0, "Access Tracking saved bit locations are not zero");
    spte |= (spte & SHADOW_ACC_TRACK_SAVED_BITS_MASK) << SHADOW_ACC_TRACK_SAVED_BITS_SHIFT;
    spte & !(shadow_acc_track_mask | shadow_accessed_mask)
}

pub unsafe fn kvm_mmu_set_mmio_spte_mask(mut mmio_value: u64, mmio_mask: u64, access_mask: u64) {
    BUG_ON((access_mask as u32 as u64) != access_mask); WARN_ON(mmio_value & shadow_nonpresent_or_rsvd_lower_gfn_mask);
    enable_mmio_caching = allow_mmio_caching; if !enable_mmio_caching { mmio_value = 0; }
    if WARN_ON(mmio_mask & !SPTE_MMIO_ALLOWED_MASK) { mmio_value = 0; }
    if WARN_ON(mmio_value & (shadow_nonpresent_or_rsvd_mask << SHADOW_NONPRESENT_OR_RSVD_MASK_LEN)) { mmio_value = 0; }
    if WARN_ON((mmio_value & mmio_mask) != mmio_value) || WARN_ON(mmio_value != 0 && (FROZEN_SPTE & mmio_mask) == mmio_value) { mmio_value = 0; }
    if mmio_value == 0 { enable_mmio_caching = false; }
    shadow_mmio_value = mmio_value; shadow_mmio_mask = mmio_mask; shadow_mmio_access_mask = access_mask;
}

pub unsafe fn kvm_mmu_set_mmio_spte_value(kvm: *mut kvm, mmio_value: u64) { (*kvm).arch.shadow_mmio_value = mmio_value; }
pub unsafe fn kvm_mmu_set_me_spte_mask(mut me_value: u64, mut me_mask: u64) { if WARN_ON(me_value & !me_mask) { me_value = 0; me_mask = 0; } shadow_me_value = me_value; shadow_me_mask = me_mask; }

pub unsafe fn kvm_mmu_set_ept_masks(has_ad_bits: bool) {
    kvm_ad_enabled = has_ad_bits; shadow_user_mask = 0; shadow_accessed_mask = VMX_EPT_ACCESS_BIT; shadow_dirty_mask = VMX_EPT_DIRTY_BIT; shadow_nx_mask = 0; shadow_xs_mask = VMX_EPT_EXECUTABLE_MASK; shadow_xu_mask = VMX_EPT_USER_EXECUTABLE_MASK; shadow_present_mask = VMX_EPT_SUPPRESS_VE_BIT; shadow_acc_track_mask = VMX_EPT_RWX_MASK | VMX_EPT_USER_EXECUTABLE_MASK; shadow_host_writable_mask = EPT_SPTE_HOST_WRITABLE; shadow_mmu_writable_mask = EPT_SPTE_MMU_WRITABLE;
    kvm_mmu_set_mmio_spte_mask(VMX_EPT_MISCONFIG_WX_VALUE, VMX_EPT_RWX_MASK | VMX_EPT_SUPPRESS_VE_BIT, 0);
}

pub unsafe fn kvm_mmu_reset_all_pte_masks() {
    let mut low_phys_bits: u8; let mask: u64; kvm_ad_enabled = true; shadow_nonpresent_or_rsvd_mask = 0; low_phys_bits = boot_cpu_data.x86_phys_bits;
    if boot_cpu_has_bug(X86_BUG_L1TF) && !WARN_ON_ONCE(boot_cpu_data.x86_cache_bits >= 52 - SHADOW_NONPRESENT_OR_RSVD_MASK_LEN) { low_phys_bits = boot_cpu_data.x86_cache_bits - SHADOW_NONPRESENT_OR_RSVD_MASK_LEN; shadow_nonpresent_or_rsvd_mask = rsvd_bits(low_phys_bits, boot_cpu_data.x86_cache_bits - 1); }
    shadow_nonpresent_or_rsvd_lower_gfn_mask = GENMASK_ULL(low_phys_bits - 1, PAGE_SHIFT); shadow_user_mask = PT_USER_MASK; shadow_accessed_mask = PT_ACCESSED_MASK; shadow_dirty_mask = PT_DIRTY_MASK; shadow_nx_mask = PT64_NX_MASK; shadow_xs_mask = 0; shadow_xu_mask = 0; shadow_present_mask = PT_PRESENT_MASK; shadow_acc_track_mask = 0; shadow_me_mask = 0; shadow_me_value = 0; shadow_host_writable_mask = DEFAULT_SPTE_HOST_WRITABLE; shadow_mmu_writable_mask = DEFAULT_SPTE_MMU_WRITABLE;
    mask = if kvm_host.maxphyaddr < 52 { BIT_ULL(51) | PT_PRESENT_MASK } else { 0 }; kvm_mmu_set_mmio_spte_mask(mask, mask, ACC_WRITE_MASK as u64 | ACC_USER_MASK as u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
