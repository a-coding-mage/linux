/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from x86/kvm/mmu.h.  Dependencies are supplied by other units. */

extern "C" {
    pub static mut tdp_enabled: bool;
    #[cfg(target_pointer_width = "64")]
    pub static mut tdp_mmu_enabled: bool;
    pub static mut enable_mmio_caching: bool;
    pub static mut eager_page_split: bool;
}
#[cfg(not(target_pointer_width = "64"))]
pub const tdp_mmu_enabled: bool = false;

pub const KVM_MEMSLOT_PAGES_TO_MMU_PAGES_RATIO: u64 = 50;
pub const KVM_MIN_ALLOC_MMU_PAGES: u64 = 64;
pub const KVM_MMU_HASH_SHIFT: u32 = 12;
pub const KVM_NUM_MMU_PAGES: u64 = 1 << KVM_MMU_HASH_SHIFT;
pub const KVM_MIN_FREE_MMU_PAGES: u64 = 5;
pub const KVM_REFILL_PAGES: u64 = 25;
pub const PT_WRITABLE_SHIFT: u32 = 1;
pub const PT_USER_SHIFT: u32 = 2;
pub const PT_PRESENT_MASK: u64 = 1 << 0;
pub const PT_WRITABLE_MASK: u64 = 1 << PT_WRITABLE_SHIFT;
pub const PT_USER_MASK: u64 = 1 << PT_USER_SHIFT;
pub const PT_PWT_MASK: u64 = 1 << 3;
pub const PT_PCD_MASK: u64 = 1 << 4;
pub const PT_ACCESSED_SHIFT: u32 = 5;
pub const PT_ACCESSED_MASK: u64 = 1 << PT_ACCESSED_SHIFT;
pub const PT_DIRTY_SHIFT: u32 = 6;
pub const PT_DIRTY_MASK: u64 = 1 << PT_DIRTY_SHIFT;
pub const PT_PAGE_SIZE_SHIFT: u32 = 7;
pub const PT_PAGE_SIZE_MASK: u64 = 1 << PT_PAGE_SIZE_SHIFT;
pub const PT_PAT_MASK: u64 = 1 << 7;
pub const PT_GLOBAL_MASK: u64 = 1 << 8;
pub const PT64_NX_SHIFT: u32 = 63;
pub const PT64_NX_MASK: u64 = 1 << PT64_NX_SHIFT;
pub const PT_PAT_SHIFT: u32 = 7;
pub const PT_DIR_PAT_SHIFT: u32 = 12;
pub const PT_DIR_PAT_MASK: u64 = 1 << PT_DIR_PAT_SHIFT;
pub const PT64_ROOT_5LEVEL: i32 = 5;
pub const PT64_ROOT_4LEVEL: i32 = 4;
pub const PT32_ROOT_LEVEL: i32 = 2;
pub const PT32E_ROOT_LEVEL: i32 = 3;
pub const ACC_READ_MASK: u32 = PT_PRESENT_MASK as u32;
pub const ACC_WRITE_MASK: u32 = PT_WRITABLE_MASK as u32;
pub const ACC_USER_MASK: u32 = PT_USER_MASK as u32;
pub const ACC_USER_EXEC_MASK: u32 = ACC_USER_MASK;
pub const ACC_EXEC_MASK: u32 = 8;
pub const ACC_ALL: u32 = ACC_EXEC_MASK | ACC_WRITE_MASK | ACC_USER_MASK | ACC_READ_MASK;

#[inline(always)]
pub const fn rsvd_bits(mut s: i32, mut e: i32) -> u64 {
    if e < s { return 0; }
    e &= 63;
    ((2u64 << ((e - s) as u32)) - 1) << (s as u32)
}

pub unsafe fn kvm_mmu_max_gfn() -> gfn_t {
    let max_gpa_bits = if likely(tdp_enabled) { kvm_host.maxphyaddr } else { 52 };
    (1u64 << ((max_gpa_bits - PAGE_SHIFT) as u32)) - 1
}

#[inline]
pub unsafe fn mmu_has_mbec(mmu: *mut kvm_mmu) -> bool { (*mmu).root_role.cr4_smep }

extern "C" {
    pub fn kvm_mmu_get_max_tdp_level() -> u8;
    pub fn kvm_mmu_x86_module_init();
    pub fn kvm_mmu_vendor_module_init() -> i32;
    pub fn kvm_mmu_vendor_module_exit();
    pub fn kvm_mmu_destroy(vcpu: *mut kvm_vcpu);
    pub fn kvm_mmu_create(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_mmu_init_vm(kvm: *mut kvm) -> i32;
    pub fn kvm_mmu_uninit_vm(kvm: *mut kvm);
    pub fn kvm_mmu_init_memslot_memory_attributes(kvm: *mut kvm, slot: *mut kvm_memory_slot);
    pub fn kvm_mmu_after_set_cpuid(vcpu: *mut kvm_vcpu);
    pub fn kvm_mmu_reset_context(vcpu: *mut kvm_vcpu);
    pub fn kvm_mmu_slot_remove_write_access(kvm: *mut kvm, memslot: *const kvm_memory_slot, start_level: i32);
    pub fn kvm_mmu_slot_try_split_huge_pages(kvm: *mut kvm, memslot: *const kvm_memory_slot, target_level: i32);
    pub fn kvm_mmu_try_split_huge_pages(kvm: *mut kvm, memslot: *const kvm_memory_slot, start: u64, end: u64, target_level: i32);
    pub fn kvm_mmu_recover_huge_pages(kvm: *mut kvm, memslot: *const kvm_memory_slot);
    pub fn kvm_mmu_slot_leaf_clear_dirty(kvm: *mut kvm, memslot: *const kvm_memory_slot);
    pub fn kvm_mmu_invalidate_mmio_sptes(kvm: *mut kvm, gen: u64);
    pub fn kvm_mmu_change_mmu_pages(kvm: *mut kvm, pages: ulong);
    pub fn kvm_zap_gfn_range(kvm: *mut kvm, start: gfn_t, end: gfn_t);
    pub fn kvm_mmu_set_mmio_spte_mask(mmio_value: u64, mmio_mask: u64, access_mask: u64);
    pub fn kvm_mmu_set_mmio_spte_value(kvm: *mut kvm, mmio_value: u64);
    pub fn kvm_mmu_set_me_spte_mask(me_value: u64, me_mask: u64);
    pub fn kvm_mmu_set_ept_masks(has_ad_bits: bool);
    pub fn kvm_init_mmu(vcpu: *mut kvm_vcpu);
    pub fn kvm_init_shadow_npt_mmu(vcpu: *mut kvm_vcpu, cr4: ulong, efer: u64, nested_cr3: gpa_t, misc_ctl: u64);
    pub fn kvm_init_shadow_ept_mmu(vcpu: *mut kvm_vcpu, execonly: bool, huge_page_level: i32, accessed_dirty: bool, mbec: bool, new_eptp: gpa_t);
    pub fn kvm_mmu_page_fault(vcpu: *mut kvm_vcpu, cr2_or_gpa: gpa_t, error_code: u64, insn: *mut core::ffi::c_void, insn_len: i32) -> i32;
    pub fn kvm_mmu_print_sptes(vcpu: *mut kvm_vcpu, gpa: gpa_t, msg: *const i8);
    pub fn kvm_mmu_invlpg(vcpu: *mut kvm_vcpu, gva: gva_t);
    pub fn kvm_mmu_new_pgd(vcpu: *mut kvm_vcpu, new_pgd: gpa_t);
    pub fn kvm_configure_mmu(enable_tdp: bool, forced: i32, max: i32, huge: i32);
    pub fn kvm_mmu_load(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_mmu_unload(vcpu: *mut kvm_vcpu);
    pub fn kvm_mmu_free_obsolete_roots(vcpu: *mut kvm_vcpu);
    pub fn kvm_mmu_sync_roots(vcpu: *mut kvm_vcpu);
    pub fn kvm_mmu_sync_prev_roots(vcpu: *mut kvm_vcpu);
    pub fn kvm_mmu_free_roots(kvm: *mut kvm, mmu: *mut kvm_mmu, roots: ulong);
    pub fn kvm_mmu_free_guest_mode_roots(kvm: *mut kvm, mmu: *mut kvm_mmu);
    pub fn kvm_mmu_post_init_vm(kvm: *mut kvm) -> i32;
    pub fn kvm_mmu_pre_destroy_vm(kvm: *mut kvm);
    pub fn kvm_tdp_mmu_map_private_pfn(vcpu: *mut kvm_vcpu, gfn: gfn_t, pfn: kvm_pfn_t) -> i32;
    pub fn kvm_mmu_invalidate_addr(vcpu: *mut kvm_vcpu, w: *mut kvm_pagewalk, addr: u64, roots: ulong);
    pub fn kvm_mmu_invpcid_gva(vcpu: *mut kvm_vcpu, gva: gva_t, pcid: ulong);
    pub fn kvm_can_do_async_pf(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_handle_page_fault(vcpu: *mut kvm_vcpu, error_code: u64, fault_address: u64, insn: *mut i8, insn_len: i32) -> i32;
    pub fn __kvm_mmu_refresh_passthrough_bits(vcpu: *mut kvm_vcpu, pw: *mut kvm_pagewalk);
    pub fn kvm_mmu_track_write(vcpu: *mut kvm_vcpu, gpa: gpa_t, new: *const u8, bytes: i32);
    pub fn __kvm_mmu_unprotect_gfn_and_retry(vcpu: *mut kvm_vcpu, gpa: gpa_t, always_retry: bool) -> bool;
    pub fn kvm_mmu_gva_to_gpa_read(vcpu: *mut kvm_vcpu, gva: gva_t, exception: *mut x86_exception) -> gpa_t;
    pub fn kvm_mmu_gva_to_gpa_write(vcpu: *mut kvm_vcpu, gva: gva_t, exception: *mut x86_exception) -> gpa_t;
    pub fn kvm_mmu_gva_to_gpa_system(vcpu: *mut kvm_vcpu, gva: gva_t, exception: *mut x86_exception) -> gpa_t;
}

#[inline]
pub unsafe fn kvm_mmu_unprotect_gfn_and_retry(vcpu: *mut kvm_vcpu, gpa: gpa_t) -> bool {
    __kvm_mmu_unprotect_gfn_and_retry(vcpu, gpa, false)
}
#[inline]
pub unsafe fn kvm_shadow_root_allocated(kvm: *mut kvm) -> bool { smp_load_acquire(&(*kvm).arch.shadow_root_allocated) }
#[inline]
pub unsafe fn kvm_memslots_have_rmaps(kvm: *mut kvm) -> bool { !tdp_mmu_enabled || kvm_shadow_root_allocated(kvm) }
#[inline]
pub unsafe fn gfn_to_index(gfn: gfn_t, base_gfn: gfn_t, level: i32) -> gfn_t { (gfn >> KVM_HPAGE_GFN_SHIFT(level)) - (base_gfn >> KVM_HPAGE_GFN_SHIFT(level)) }
#[inline]
pub unsafe fn kvm_mmu_slot_lpages(slot: *mut kvm_memory_slot, level: i32) -> ulong { gfn_to_index((*slot).base_gfn + (*slot).npages - 1, (*slot).base_gfn, level) + 1 }
#[inline]
pub unsafe fn mmu_is_nested(vcpu: *mut kvm_vcpu) -> bool { (*vcpu).arch.mmu == &mut (*vcpu).arch.guest_mmu }
#[inline]
pub unsafe fn kvm_has_mirrored_tdp(kvm: *const kvm) -> bool { (*kvm).arch.vm_type == KVM_X86_TDX_VM }
#[inline]
pub unsafe fn kvm_gfn_direct_bits(kvm: *const kvm) -> gfn_t { (*kvm).arch.gfn_direct_bits }
#[inline]
pub unsafe fn kvm_is_addr_direct(kvm: *mut kvm, gpa: gpa_t) -> bool { let bits = gfn_to_gpa(kvm_gfn_direct_bits(kvm)); bits == 0 || (gpa & bits) != 0 }
#[inline]
pub unsafe fn kvm_is_gfn_alias(kvm: *mut kvm, gfn: gfn_t) -> bool { (gfn & kvm_gfn_direct_bits(kvm)) != 0 }

#[inline]
pub unsafe fn kvm_mmu_reload(vcpu: *mut kvm_vcpu) -> i32 {
    if kvm_check_request(KVM_REQ_MMU_FREE_OBSOLETE_ROOTS, vcpu) { kvm_mmu_free_obsolete_roots(vcpu); }
    if likely((*(*vcpu).arch.mmu).root.hpa != INVALID_PAGE) { return 0; }
    kvm_mmu_load(vcpu)
}
#[inline]
pub unsafe fn kvm_get_pcid(vcpu: *mut kvm_vcpu, cr3: gpa_t) -> ulong {
    if kvm_is_cr4_bit_set(vcpu, X86_CR4_PCIDE) { cr3 & X86_CR3_PCID_MASK } else { 0 }
}
#[inline]
pub unsafe fn kvm_get_active_pcid(vcpu: *mut kvm_vcpu) -> ulong { kvm_get_pcid(vcpu, kvm_read_cr3(vcpu)) }
#[inline]
pub unsafe fn kvm_get_active_cr3_lam_bits(vcpu: *mut kvm_vcpu) -> ulong {
    if !guest_cpu_cap_has(vcpu, X86_FEATURE_LAM) { return 0; }
    kvm_read_cr3(vcpu) & (X86_CR3_LAM_U48 | X86_CR3_LAM_U57)
}
#[inline]
pub unsafe fn kvm_mmu_load_pgd(vcpu: *mut kvm_vcpu) {
    let root_hpa = (*(*vcpu).arch.mmu).root.hpa;
    if !VALID_PAGE(root_hpa) { return; }
    kvm_x86_call_load_mmu_pgd(vcpu, root_hpa, (*(*vcpu).arch.mmu).root_role.level);
}
#[inline]
pub unsafe fn kvm_mmu_refresh_passthrough_bits(vcpu: *mut kvm_vcpu, w: *mut kvm_pagewalk) {
    if !tdp_enabled || w == &mut (*vcpu).arch.ngpa_walk { return; }
    __kvm_mmu_refresh_passthrough_bits(vcpu, w);
}
#[inline]
pub unsafe fn kvm_translate_gpa(vcpu: *mut kvm_vcpu, w: *mut kvm_pagewalk, gpa: gpa_t, access: u64, exception: *mut x86_exception, pte_access: u64) -> gpa_t {
    if !mmu_is_nested(vcpu) || w == &mut (*vcpu).arch.ngpa_walk { return gpa; }
    kvm_nested_translate_nested_gpa(vcpu, gpa, access, exception, pte_access)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
