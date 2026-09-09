// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Google LLC
 * Author: Will Deacon <will@kernel.org>
 */

// Translated from the C header. Kernel-provided symbols and types remain external dependencies.

pub const KVM_PGTABLE_FIRST_LEVEL: i8 = -1;
pub const KVM_PGTABLE_LAST_LEVEL: i8 = 3;
// CONFIG_ARM64_4K_PAGES selects level 1; otherwise level 2.
pub const KVM_PGTABLE_MIN_BLOCK_LEVEL: i8 = 2;

pub type kvm_pte_t = u64;

pub const KVM_PTE_VALID: u64 = 1 << 0;
pub const KVM_PTE_ADDR_MASK: u64 = (((1u64 << 48) - 1) & !((1u64 << PAGE_SHIFT) - 1));
pub const KVM_PTE_ADDR_51_48: u64 = 0xf000;
pub const KVM_PTE_ADDR_MASK_LPA2: u64 = (((1u64 << 50) - 1) & !((1u64 << PAGE_SHIFT) - 1));
pub const KVM_PTE_ADDR_51_50_LPA2: u64 = 0x300;
pub const KVM_PHYS_INVALID: u64 = u64::MAX;
pub const KVM_PTE_TYPE: u64 = 1 << 1;
pub const KVM_PTE_TYPE_BLOCK: u64 = 0;
pub const KVM_PTE_TYPE_PAGE: u64 = 1;
pub const KVM_PTE_TYPE_TABLE: u64 = 1;
pub const KVM_PTE_LEAF_ATTR_LO: u64 = 0xffc;
pub const KVM_PTE_LEAF_ATTR_LO_S1_ATTRIDX: u64 = 0x1c;
pub const KVM_PTE_LEAF_ATTR_LO_S1_AP: u64 = 0xc0;
pub const KVM_PTE_LEAF_ATTR_LO_S1_SH: u64 = 0x300;
pub const KVM_PTE_LEAF_ATTR_LO_S1_SH_IS: u64 = 3;
pub const KVM_PTE_LEAF_ATTR_LO_S1_AF: u64 = 1 << 10;
pub const KVM_PTE_LEAF_ATTR_LO_S2_MEMATTR: u64 = 0x3c;
pub const KVM_PTE_LEAF_ATTR_LO_S2_S2AP_R: u64 = 1 << 6;
pub const KVM_PTE_LEAF_ATTR_LO_S2_S2AP_W: u64 = 1 << 7;
pub const KVM_PTE_LEAF_ATTR_LO_S2_SH: u64 = 0x300;
pub const KVM_PTE_LEAF_ATTR_LO_S2_SH_IS: u64 = 3;
pub const KVM_PTE_LEAF_ATTR_LO_S2_AF: u64 = 1 << 10;
pub const KVM_PTE_LEAF_ATTR_HI: u64 = ((1u64 << 64) - 1) & !((1u64 << 50) - 1);
pub const KVM_PTE_LEAF_ATTR_HI_SW: u64 = 0x0780_0000_0000_0000;
pub const KVM_PTE_LEAF_ATTR_HI_S1_XN: u64 = 1 << 54;
pub const KVM_PTE_LEAF_ATTR_HI_S1_UXN: u64 = 1 << 54;
pub const KVM_PTE_LEAF_ATTR_HI_S1_PXN: u64 = 1 << 53;
pub const KVM_PTE_LEAF_ATTR_HI_S2_XN: u64 = 0x0060_0000_0000_0000;
pub const KVM_PTE_LEAF_ATTR_HI_S1_GP: u64 = 1 << 50;
pub const KVM_PTE_LEAF_ATTR_S2_PERMS: u64 = KVM_PTE_LEAF_ATTR_LO_S2_S2AP_R | KVM_PTE_LEAF_ATTR_LO_S2_S2AP_W | KVM_PTE_LEAF_ATTR_HI_S2_XN;
pub const KVM_INVALID_PTE_TYPE_MASK: u64 = 0xf000_0000_0000_0000;
pub const KVM_INVALID_PTE_ANNOT_MASK: u64 = !(KVM_PTE_VALID | KVM_INVALID_PTE_TYPE_MASK);

#[repr(C)]
#[derive(Copy, Clone)]
pub enum kvm_invalid_pte_type { KVM_INVALID_PTE_TYPE_LOCKED = 1, KVM_HOST_INVALID_PTE_TYPE_DONATION, KVM_GUEST_INVALID_PTE_TYPE_POISONED }

pub const KVM_PGTABLE_S2_IDMAP: u32 = 1 << 0;
pub const KVM_PGTABLE_S2_AS_S1: u32 = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum kvm_pgtable_stage2_flags { KVM_PGTABLE_STAGE2_FLAGS_IDMAP = KVM_PGTABLE_S2_IDMAP as isize, KVM_PGTABLE_STAGE2_FLAGS_AS_S1 = KVM_PGTABLE_S2_AS_S1 as isize }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum kvm_pgtable_prot {
    KVM_PGTABLE_PROT_PX = 1 << 0, KVM_PGTABLE_PROT_UX = 1 << 1,
    KVM_PGTABLE_PROT_X = 3, KVM_PGTABLE_PROT_W = 1 << 2, KVM_PGTABLE_PROT_R = 1 << 3,
    KVM_PGTABLE_PROT_DEVICE = 1 << 4, KVM_PGTABLE_PROT_NORMAL_NC = 1 << 5,
    KVM_PGTABLE_PROT_SW0 = 1 << 55, KVM_PGTABLE_PROT_SW1 = 1 << 56,
    KVM_PGTABLE_PROT_SW2 = 1 << 57, KVM_PGTABLE_PROT_SW3 = 1 << 58,
}
pub const KVM_PGTABLE_PROT_RW: u64 = 0x18;
pub const KVM_PGTABLE_PROT_RWX: u64 = 0x1b;
pub const PKVM_HOST_MEM_PROT: u64 = KVM_PGTABLE_PROT_RWX;
pub const PKVM_HOST_MMIO_PROT: u64 = KVM_PGTABLE_PROT_RW;
pub const PAGE_HYP: u64 = KVM_PGTABLE_PROT_RW;
pub const PAGE_HYP_EXEC: u64 = 0x9;
pub const PAGE_HYP_RO: u64 = 0x8;
pub const PAGE_HYP_DEVICE: u64 = PAGE_HYP | 0x10;

pub type kvm_pgtable_force_pte_cb_t = unsafe extern "C" fn(u64, u64, kvm_pgtable_prot) -> bool;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum kvm_pgtable_walk_flags { KVM_PGTABLE_WALK_LEAF=1, KVM_PGTABLE_WALK_TABLE_PRE=2, KVM_PGTABLE_WALK_TABLE_POST=4, KVM_PGTABLE_WALK_SHARED=8, KVM_PGTABLE_WALK_IGNORE_EAGAIN=16, KVM_PGTABLE_WALK_SKIP_BBM_TLBI=32, KVM_PGTABLE_WALK_SKIP_CMO=64 }

#[repr(C)]
pub struct kvm_pgtable_mm_ops {
    pub zalloc_page: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void>,
    pub zalloc_pages_exact: Option<unsafe extern "C" fn(usize) -> *mut core::ffi::c_void>,
    pub free_pages_exact: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize)>,
    pub free_unlinked_table: Option<unsafe extern "C" fn(*mut core::ffi::c_void, i8)>,
    pub get_page: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pub put_page: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub page_count: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
    pub phys_to_virt: Option<unsafe extern "C" fn(phys_addr_t) -> *mut core::ffi::c_void>, pub virt_to_phys: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> phys_addr_t>,
    pub dcache_clean_inval_poc: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize)>, pub icache_inval_pou: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize)>,
}

pub type kvm_pgtable_visitor_fn_t = unsafe extern "C" fn(*const kvm_pgtable_visit_ctx, kvm_pgtable_walk_flags) -> i32;
#[repr(C)] pub struct kvm_pgtable_visit_ctx { pub ptep: *mut kvm_pte_t, pub old: kvm_pte_t, pub arg: *mut core::ffi::c_void, pub mm_ops: *mut kvm_pgtable_mm_ops, pub start: u64, pub addr: u64, pub end: u64, pub level: i8, pub flags: kvm_pgtable_walk_flags }
#[repr(C)] pub struct kvm_pgtable_walker { pub cb: kvm_pgtable_visitor_fn_t, pub arg: *mut core::ffi::c_void, pub flags: kvm_pgtable_walk_flags }
pub type kvm_pteref_t = *mut kvm_pte_t;

// The following helpers retain the C header's inline behavior; architecture
// feature and bit-field helpers are supplied by the surrounding kernel.
extern "C" {
    fn kvm_lpa2_is_enabled() -> bool;
    fn kvm_get_parange_max() -> u64;
    fn cpuid_feature_extract_unsigned_field(value: u64, shift: u32) -> u64;
}
#[inline] pub unsafe fn kvm_get_parange(mmfr0: u64) -> u64 { let max = kvm_get_parange_max(); let mut p = cpuid_feature_extract_unsigned_field(mmfr0, ID_AA64MMFR0_EL1_PARANGE_SHIFT); if p > max { p = max; } p }
#[inline] pub unsafe fn kvm_pte_valid(pte: kvm_pte_t) -> bool { pte & KVM_PTE_VALID != 0 }
#[inline] pub unsafe fn kvm_pte_to_phys(pte: kvm_pte_t) -> u64 { let mut pa; if kvm_lpa2_is_enabled() { pa = pte & KVM_PTE_ADDR_MASK_LPA2; pa |= ((pte & KVM_PTE_ADDR_51_50_LPA2) >> 8) << 50; } else { pa = pte & KVM_PTE_ADDR_MASK; if PAGE_SHIFT == 16 { pa |= ((pte & KVM_PTE_ADDR_51_48) >> 12) << 48; } } pa }
#[inline] pub unsafe fn kvm_phys_to_pte(mut pa: u64) -> kvm_pte_t { let mut pte; if kvm_lpa2_is_enabled() { pte = pa & KVM_PTE_ADDR_MASK_LPA2; pte |= ((pa >> 50) & 3) << 8; } else { pte = pa & KVM_PTE_ADDR_MASK; if PAGE_SHIFT == 16 { pte |= ((pa >> 48) & 0xf) << 12; } } pte }
extern "C" { fn __phys_to_pfn(phys: u64) -> u64; fn ARM64_HW_PGTABLE_LEVEL_SHIFT(level: i8) -> u32; }
#[inline] pub unsafe fn kvm_pte_to_pfn(pte: kvm_pte_t) -> u64 { __phys_to_pfn(kvm_pte_to_phys(pte)) }
#[inline] pub unsafe fn kvm_granule_shift(level: i8) -> u32 { ARM64_HW_PGTABLE_LEVEL_SHIFT(level) }
#[inline] pub unsafe fn kvm_granule_size(level: i8) -> u64 { 1u64 << kvm_granule_shift(level) }
#[inline] pub fn kvm_level_supports_block_mapping(level: i8) -> bool { level >= KVM_PGTABLE_MIN_BLOCK_LEVEL }
#[inline] pub unsafe fn kvm_supported_block_sizes() -> u32 { let mut level = KVM_PGTABLE_MIN_BLOCK_LEVEL; let mut r = 0; while level <= KVM_PGTABLE_LAST_LEVEL { r |= 1u32 << kvm_granule_shift(level); level += 1; } r }
#[inline] pub unsafe fn kvm_is_block_size_supported(size: u64) -> bool { size != 0 && size.is_power_of_two() && (size & kvm_supported_block_sizes() as u64) != 0 }
#[inline] pub unsafe fn kvm_pgtable_walk_shared(ctx: *const kvm_pgtable_visit_ctx) -> bool { (*ctx).flags as u32 & KVM_PGTABLE_WALK_SHARED as u32 != 0 }
#[inline] pub unsafe fn kvm_pgtable_stage2_init(pgt: *mut kvm_pgtable, mmu: *mut kvm_s2_mmu, mm_ops: *mut kvm_pgtable_mm_ops) -> i32 { __kvm_pgtable_stage2_init(pgt, mmu, mm_ops, core::mem::transmute(0u32), None) }

#[repr(C)] pub union kvm_pgtable_union { pub pkvm_mappings: rb_root_cached, pub stage1: kvm_pgtable_stage1 }
#[repr(C)] pub struct kvm_pgtable_stage1 { pub ia_bits: u32, pub start_level: i8, pub pgd: kvm_pteref_t, pub mm_ops: *mut kvm_pgtable_mm_ops, pub flags: kvm_pgtable_stage2_flags, pub force_pte_cb: Option<kvm_pgtable_force_pte_cb_t> }
#[repr(C)] pub struct kvm_pgtable { pub root: kvm_pgtable_union, pub mmu: *mut kvm_s2_mmu }

extern "C" {
    pub fn kvm_pgtable_hyp_init(pgt: *mut kvm_pgtable, va_bits: u32, mm_ops: *mut kvm_pgtable_mm_ops) -> i32;
    pub fn kvm_pgtable_hyp_destroy(pgt: *mut kvm_pgtable);
    pub fn kvm_pgtable_hyp_map(pgt: *mut kvm_pgtable, addr: u64, size: u64, phys: u64, prot: kvm_pgtable_prot) -> i32;
    pub fn kvm_pgtable_hyp_unmap(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> u64;
    pub fn kvm_get_vtcr(mmfr0: u64, mmfr1: u64, phys_shift: u32) -> u64;
    pub fn kvm_pgtable_stage2_pgd_size(vtcr: u64) -> usize;
    pub fn __kvm_pgtable_stage2_init(pgt: *mut kvm_pgtable, mmu: *mut kvm_s2_mmu, mm_ops: *mut kvm_pgtable_mm_ops, flags: kvm_pgtable_stage2_flags, force_pte_cb: Option<kvm_pgtable_force_pte_cb_t>) -> i32;
    pub fn kvm_pgtable_stage2_destroy(pgt: *mut kvm_pgtable);
    pub fn kvm_pgtable_stage2_destroy_range(pgt: *mut kvm_pgtable, addr: u64, size: u64);
    pub fn kvm_pgtable_stage2_destroy_pgd(pgt: *mut kvm_pgtable);
    pub fn kvm_pgtable_stage2_free_unlinked(mm_ops: *mut kvm_pgtable_mm_ops, pgtable: *mut core::ffi::c_void, level: i8);
    pub fn kvm_pgtable_stage2_create_unlinked(pgt: *mut kvm_pgtable, phys: u64, level: i8, prot: kvm_pgtable_prot, mc: *mut core::ffi::c_void, force_pte: bool) -> *mut kvm_pte_t;
    pub fn kvm_pgtable_stage2_map(pgt: *mut kvm_pgtable, addr: u64, size: u64, phys: u64, prot: kvm_pgtable_prot, mc: *mut core::ffi::c_void, flags: kvm_pgtable_walk_flags) -> i32;
    pub fn kvm_pgtable_stage2_annotate(pgt: *mut kvm_pgtable, addr: u64, size: u64, mc: *mut core::ffi::c_void, ty: kvm_invalid_pte_type, annotation: kvm_pte_t) -> i32;
    pub fn kvm_pgtable_stage2_unmap(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> i32;
    pub fn kvm_pgtable_stage2_wrprotect(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> i32;
    pub fn kvm_pgtable_stage2_mkyoung(pgt: *mut kvm_pgtable, addr: u64, flags: kvm_pgtable_walk_flags);
    pub fn kvm_pgtable_stage2_test_clear_young(pgt: *mut kvm_pgtable, addr: u64, size: u64, mkold: bool) -> bool;
    pub fn kvm_pgtable_stage2_relax_perms(pgt: *mut kvm_pgtable, addr: u64, prot: kvm_pgtable_prot, flags: kvm_pgtable_walk_flags) -> i32;
    pub fn kvm_pgtable_stage2_flush(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> i32;
    pub fn kvm_pgtable_stage2_split(pgt: *mut kvm_pgtable, addr: u64, size: u64, mc: *mut kvm_mmu_memory_cache) -> i32;
    pub fn kvm_pgtable_walk(pgt: *mut kvm_pgtable, addr: u64, size: u64, walker: *mut kvm_pgtable_walker) -> i32;
    pub fn kvm_pgtable_get_leaf(pgt: *mut kvm_pgtable, addr: u64, ptep: *mut kvm_pte_t, level: *mut i8) -> i32;
    pub fn kvm_pgtable_stage2_pte_prot(pte: kvm_pte_t) -> kvm_pgtable_prot;
    pub fn kvm_pgtable_hyp_pte_prot(pte: kvm_pte_t) -> kvm_pgtable_prot;
    pub fn kvm_tlb_flush_vmid_range(mmu: *mut kvm_s2_mmu, addr: phys_addr_t, size: usize);
}

// External kernel types used by this header.
extern "C" { type rb_root_cached; type kvm_s2_mmu; type kvm_mmu_memory_cache; type phys_addr_t; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
