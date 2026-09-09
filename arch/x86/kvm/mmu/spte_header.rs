// SPDX-License-Identifier: GPL-2.0-only
// Translated from spte.h. Dependencies are supplied by other translation units.

pub const SPTE_MMU_PRESENT_MASK: u64 = 1u64 << 11;
pub const SPTE_TDP_AD_SHIFT: u32 = 60;
pub const SPTE_TDP_AD_MASK: u64 = 3u64 << SPTE_TDP_AD_SHIFT;
pub const SPTE_TDP_AD_ENABLED: u64 = 0u64 << SPTE_TDP_AD_SHIFT;
pub const SPTE_TDP_AD_DISABLED: u64 = 1u64 << SPTE_TDP_AD_SHIFT;
pub const SPTE_TDP_AD_WRPROT_ONLY: u64 = 2u64 << SPTE_TDP_AD_SHIFT;

// CONFIG_DYNAMIC_PHYSICAL_MASK selects the dynamic physical mask in the C build.
#[cfg(feature = "CONFIG_DYNAMIC_PHYSICAL_MASK")]
pub const SPTE_BASE_ADDR_MASK: u64 = physical_mask & !(PAGE_SIZE as u64 - 1);
#[cfg(not(feature = "CONFIG_DYNAMIC_PHYSICAL_MASK"))]
pub const SPTE_BASE_ADDR_MASK: u64 = ((1u64 << 52) - 1) & !(PAGE_SIZE as u64 - 1);

pub const SPTE_LEVEL_BITS: u32 = 9;
pub const SHADOW_ACC_TRACK_SAVED_BITS_MASK: u64 =
    VMX_EPT_READABLE_MASK | VMX_EPT_EXECUTABLE_MASK | VMX_EPT_USER_EXECUTABLE_MASK;
pub const SHADOW_ACC_TRACK_SAVED_BITS_SHIFT: u32 = 52;
pub const SHADOW_ACC_TRACK_SAVED_MASK: u64 =
    SHADOW_ACC_TRACK_SAVED_BITS_MASK << SHADOW_ACC_TRACK_SAVED_BITS_SHIFT;
pub const DEFAULT_SPTE_HOST_WRITABLE: u64 = 1u64 << 9;
pub const DEFAULT_SPTE_MMU_WRITABLE: u64 = 1u64 << 10;
pub const EPT_SPTE_HOST_WRITABLE: u64 = 1u64 << 53;
pub const EPT_SPTE_MMU_WRITABLE: u64 = 1u64 << 55;

pub const MMIO_SPTE_GEN_LOW_START: u32 = 3;
pub const MMIO_SPTE_GEN_LOW_END: u32 = 9;
pub const MMIO_SPTE_GEN_HIGH_START: u32 = 52;
pub const MMIO_SPTE_GEN_HIGH_END: u32 = 62;
pub const MMIO_SPTE_GEN_LOW_MASK: u64 = ((1u64 << (MMIO_SPTE_GEN_LOW_END - MMIO_SPTE_GEN_LOW_START + 1)) - 1) << MMIO_SPTE_GEN_LOW_START;
pub const MMIO_SPTE_GEN_HIGH_MASK: u64 = ((1u64 << (MMIO_SPTE_GEN_HIGH_END - MMIO_SPTE_GEN_HIGH_START + 1)) - 1) << MMIO_SPTE_GEN_HIGH_START;
pub const SPTE_MMIO_ALLOWED_MASK: u64 = (1u64 << 63) | (((1u64 << 40) - 1) << 12) | (1u64 << 10) | 7;
pub const MMIO_SPTE_GEN_LOW_BITS: u32 = MMIO_SPTE_GEN_LOW_END - MMIO_SPTE_GEN_LOW_START + 1;
pub const MMIO_SPTE_GEN_HIGH_BITS: u32 = MMIO_SPTE_GEN_HIGH_END - MMIO_SPTE_GEN_HIGH_START + 1;
pub const MMIO_SPTE_GEN_LOW_SHIFT: u32 = MMIO_SPTE_GEN_LOW_START;
pub const MMIO_SPTE_GEN_HIGH_SHIFT: u32 = MMIO_SPTE_GEN_HIGH_START - MMIO_SPTE_GEN_LOW_BITS;
pub const MMIO_SPTE_GEN_MASK: u64 = (1u64 << (MMIO_SPTE_GEN_LOW_BITS + MMIO_SPTE_GEN_HIGH_BITS)) - 1;

#[cfg(feature = "CONFIG_X86_64")]
pub const SHADOW_NONPRESENT_VALUE: u64 = 1u64 << 63;
#[cfg(not(feature = "CONFIG_X86_64"))]
pub const SHADOW_NONPRESENT_VALUE: u64 = 0;
pub const SHADOW_NONPRESENT_OR_RSVD_MASK_LEN: u32 = 5;
pub const FROZEN_SPTE: u64 = SHADOW_NONPRESENT_VALUE | 0x1a0;

extern "C" {
    pub static mut kvm_ad_enabled: bool;
    pub static mut shadow_host_writable_mask: u64;
    pub static mut shadow_mmu_writable_mask: u64;
    pub static mut shadow_nx_mask: u64;
    pub static mut shadow_user_mask: u64;
    pub static mut shadow_xs_mask: u64;
    pub static mut shadow_xu_mask: u64;
    pub static mut shadow_accessed_mask: u64;
    pub static mut shadow_dirty_mask: u64;
    pub static mut shadow_mmio_value: u64;
    pub static mut shadow_mmio_mask: u64;
    pub static mut shadow_mmio_access_mask: u64;
    pub static mut shadow_present_mask: u64;
    pub static mut shadow_me_value: u64;
    pub static mut shadow_me_mask: u64;
    pub static mut shadow_acc_track_mask: u64;
    pub static mut shadow_nonpresent_or_rsvd_mask: u64;
    pub static mut shadow_nonpresent_or_rsvd_lower_gfn_mask: u64;
}

pub fn spte_level_shift(level: u32) -> u32 { __PT_LEVEL_SHIFT(level, SPTE_LEVEL_BITS) }
pub fn spte_index_macro(address: u64, level: u32) -> u64 { __PT_INDEX(address, level, SPTE_LEVEL_BITS) }
pub const SPTE_ENT_PER_PAGE: u64 = __PT_ENT_PER_PAGE(SPTE_LEVEL_BITS);

#[inline]
pub const fn is_frozen_spte(spte: u64) -> bool { spte == FROZEN_SPTE }

#[inline]
pub unsafe fn spte_index(sptep: *mut u64) -> i32 {
    ((sptep as usize / core::mem::size_of::<u64>()) as u64 & (SPTE_ENT_PER_PAGE - 1)) as i32
}

#[inline]
pub unsafe fn kvm_mmu_get_dummy_root() -> hpa_t { zero_pfn(0) << PAGE_SHIFT }
#[inline]
pub unsafe fn kvm_mmu_is_dummy_root(shadow_page: hpa_t) -> bool { is_zero_pfn(shadow_page >> PAGE_SHIFT) }
#[inline]
pub unsafe fn to_shadow_page(shadow_page: hpa_t) -> *mut kvm_mmu_page {
    let page = pfn_to_page(shadow_page >> PAGE_SHIFT);
    page_private(page) as *mut kvm_mmu_page
}
#[inline]
pub unsafe fn spte_to_child_sp(spte: u64) -> *mut kvm_mmu_page { to_shadow_page(spte & SPTE_BASE_ADDR_MASK) }
#[inline]
pub unsafe fn sptep_to_sp(sptep: *mut u64) -> *mut kvm_mmu_page { to_shadow_page(__pa(sptep)) }
#[inline]
pub unsafe fn root_to_sp(root: hpa_t) -> *mut kvm_mmu_page {
    if kvm_mmu_is_dummy_root(root) { core::ptr::null_mut() } else { spte_to_child_sp(root) }
}

#[inline]
pub unsafe fn is_mirror_sptep(sptep: tdp_ptep_t) -> bool { is_mirror_sp(sptep_to_sp(rcu_dereference(sptep))) }
#[inline]
pub unsafe fn kvm_vcpu_can_access_host_mmio(vcpu: *mut kvm_vcpu) -> bool {
    let root = root_to_sp((*vcpu).arch.mmu.root.hpa);
    if !root.is_null() { READ_ONCE((*root).has_mapped_host_mmio) } else { READ_ONCE((*(*vcpu).kvm).arch.has_mapped_host_mmio) }
}
#[inline]
pub unsafe fn is_mmio_spte(kvm: *mut kvm, spte: u64) -> bool {
    (spte & shadow_mmio_mask) == (*kvm).arch.shadow_mmio_value && likely(enable_mmio_caching)
}
#[inline] pub fn is_shadow_present_pte(pte: u64) -> bool { pte & SPTE_MMU_PRESENT_MASK != 0 }
#[inline] pub unsafe fn is_ept_ve_possible(spte: u64) -> bool { shadow_present_mask & VMX_EPT_SUPPRESS_VE_BIT != 0 && spte & VMX_EPT_SUPPRESS_VE_BIT == 0 && spte & VMX_EPT_RWX_MASK != VMX_EPT_MISCONFIG_WX_VALUE }
#[inline] pub unsafe fn sp_ad_disabled(sp: *mut kvm_mmu_page) -> bool { (*sp).role.ad_disabled }
#[inline] pub unsafe fn spte_ad_enabled(spte: u64) -> bool { KVM_MMU_WARN_ON(!is_shadow_present_pte(spte)); spte & SPTE_TDP_AD_MASK != SPTE_TDP_AD_DISABLED }
#[inline] pub unsafe fn spte_ad_need_write_protect(spte: u64) -> bool { KVM_MMU_WARN_ON(!is_shadow_present_pte(spte)); spte & SPTE_TDP_AD_MASK != SPTE_TDP_AD_ENABLED }
#[inline] pub unsafe fn is_access_track_spte(spte: u64) -> bool { !spte_ad_enabled(spte) && spte & shadow_acc_track_mask == 0 }
#[inline] pub fn is_large_pte(pte: u64) -> bool { pte & PT_PAGE_SIZE_MASK != 0 }
#[inline] pub fn is_last_spte(pte: u64, level: i32) -> bool { level == PG_LEVEL_4K || is_large_pte(pte) }
#[inline] pub fn spte_to_pfn(pte: u64) -> kvm_pfn_t { (pte & SPTE_BASE_ADDR_MASK) >> PAGE_SHIFT }
#[inline] pub unsafe fn is_accessed_spte(spte: u64) -> bool { spte & shadow_accessed_mask != 0 }

#[inline]
pub unsafe fn get_rsvd_bits(fmt: *mut kvm_page_format, pte: u64, level: i32) -> u64 { (*fmt).rsvd_bits_mask[((pte >> 7) & 1) as usize][(level - 1) as usize] }
#[inline] pub unsafe fn __is_rsvd_bits_set(fmt: *mut kvm_page_format, pte: u64, level: i32) -> bool { pte & get_rsvd_bits(fmt, pte, level) != 0 }
#[inline] pub unsafe fn __is_bad_mt_xwr(fmt: *mut kvm_page_format, mut pte: u64) -> bool { if pte & VMX_EPT_USER_EXECUTABLE_MASK != 0 { pte |= VMX_EPT_EXECUTABLE_MASK; } (*fmt).bad_mt_xwr & (1u64 << (pte & 0x3f)) != 0 }
#[inline] pub unsafe fn is_rsvd_spte(fmt: *mut kvm_page_format, spte: u64, level: i32) -> bool { __is_bad_mt_xwr(fmt, spte) || __is_rsvd_bits_set(fmt, spte, level) }
#[inline] pub fn is_writable_pte(pte: usize) -> bool { pte & PT_WRITABLE_MASK as usize != 0 }
#[inline] pub unsafe fn check_spte_writable_invariants(spte: u64) {
    if spte & shadow_mmu_writable_mask != 0 { WARN_ONCE(spte & shadow_host_writable_mask == 0, KBUILD_MODNAME ": MMU-writable SPTE is not Host-writable: %llx", spte); }
    else { WARN_ONCE(is_writable_pte(spte as usize), KBUILD_MODNAME ": Writable SPTE is not MMU-writable: %llx", spte); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
