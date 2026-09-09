/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/types.h, linux/kvm_host.h, asm/kvm_host.h, and mmu.h.

/* Page table builder macros common to shadow (host) PTEs and guest PTEs. */
pub const __PT_BASE_ADDR_MASK: u64 = GENMASK_ULL(51, 12);
#[inline]
pub const fn __PT_LEVEL_SHIFT(level: i32, bits_per_level: i32) -> i32 {
    PAGE_SHIFT + (level - 1) * bits_per_level
}
#[inline]
pub const fn __PT_INDEX(address: u64, level: i32, bits_per_level: i32) -> u64 {
    (address >> __PT_LEVEL_SHIFT(level, bits_per_level)) & ((1u64 << bits_per_level) - 1)
}
#[inline]
pub const fn __PT_LVL_ADDR_MASK(base_addr_mask: u64, level: i32, bits_per_level: i32) -> u64 {
    base_addr_mask & !((1u64 << (PAGE_SHIFT + (level - 1) * bits_per_level)) - 1)
}
#[inline]
pub const fn __PT_LVL_OFFSET_MASK(base_addr_mask: u64, level: i32, bits_per_level: i32) -> u64 {
    base_addr_mask & ((1u64 << (PAGE_SHIFT + (level - 1) * bits_per_level)) - 1)
}
#[inline]
pub const fn __PT_ENT_PER_PAGE(bits_per_level: i32) -> i32 { 1 << bits_per_level }

pub const INVALID_PAE_ROOT: u64 = 0;
#[inline]
pub fn IS_VALID_PAE_ROOT(x: u64) -> bool { x != 0 }

pub type tdp_ptep_t = *mut u64;

#[repr(C)]
pub struct kvm_mmu_page {
    pub link: list_head,
    pub hash_link: hlist_node,
    pub tdp_mmu_page: bool,
    pub unsync: bool,
    pub mmu_valid_gen_or_tdp_mmu_scheduled_root_to_zap: kvm_mmu_page__u0,
    pub nx_huge_page_disallowed: bool,
    pub role: kvm_mmu_page_role,
    pub gfn: gfn_t,
    pub spt: *mut u64,
    pub shadowed_translation: *mut u64,
    pub root_count_or_tdp_mmu_root_count: kvm_mmu_page__u1,
    pub has_mapped_host_mmio: bool,
    pub unsync_children_or_external_spt: kvm_mmu_page__u2,
    pub parent_ptes_or_ptep: kvm_mmu_page__u3,
    pub unsync_child_bitmap: [u64; 8],
    pub possible_nx_huge_page_link: list_head,
    // CONFIG_X86_32: int clear_spte_count;
    // CONFIG_X86_64: struct rcu_head rcu_head;
}

#[repr(C)]
pub union kvm_mmu_page__u0 { pub mmu_valid_gen: u8, pub tdp_mmu_scheduled_root_to_zap: bool }
#[repr(C)]
pub union kvm_mmu_page__u1 { pub root_count: i32, pub tdp_mmu_root_count: refcount_t }
#[repr(C)]
pub union kvm_mmu_page__u2 {
    pub unsync_children_data: kvm_mmu_page__u2_data,
    pub external_spt: *mut core::ffi::c_void,
}
#[repr(C)]
pub struct kvm_mmu_page__u2_data { pub unsync_children: u32, pub write_flooding_count: atomic_t }
#[repr(C)]
pub union kvm_mmu_page__u3 { pub parent_ptes: kvm_rmap_head, pub ptep: tdp_ptep_t }

extern "C" {
    pub static mut mmu_page_header_cache: *mut kmem_cache;
    pub fn kvm_mmu_memory_cache_alloc(cache: *mut kvm_mmu_memory_cache) -> *mut core::ffi::c_void;
    pub fn kvm_gfn_direct_bits(kvm: *const kvm) -> gfn_t;
    pub fn kvm_flush_remote_tlbs_range(kvm: *mut kvm, gfn: gfn_t, nr_pages: u64);
    pub fn kvm_prepare_memory_fault_exit(vcpu: *mut kvm_vcpu, addr: u64, len: u64, write: bool, exec: bool, is_private: bool);
    pub static mut nx_huge_pages: i32;
}

#[inline] pub fn kvm_mmu_role_as_id(role: kvm_mmu_page_role) -> i32 { if role.smm { 1 } else { 0 } }
#[inline] pub unsafe fn kvm_mmu_page_as_id(sp: *mut kvm_mmu_page) -> i32 { kvm_mmu_role_as_id((*sp).role) }
#[inline] pub unsafe fn is_mirror_sp(sp: *const kvm_mmu_page) -> bool { (*sp).role.is_mirror }
#[inline] pub unsafe fn kvm_mmu_alloc_external_spt(vcpu: *mut kvm_vcpu, sp: *mut kvm_mmu_page) {
    (*sp).unsync_children_or_external_spt.external_spt = kvm_mmu_memory_cache_alloc(&mut (*vcpu).arch.mmu_external_spt_cache);
}
#[inline] pub unsafe fn kvm_gfn_root_bits(kvm: *const kvm, root: *const kvm_mmu_page) -> gfn_t {
    if is_mirror_sp(root) { 0 } else { kvm_gfn_direct_bits(kvm) }
}
#[inline] pub unsafe fn kvm_mmu_page_ad_need_write_protect(kvm: *mut kvm, sp: *mut kvm_mmu_page) -> bool {
    (*kvm).arch.cpu_dirty_log_size != 0 && (*sp).role.guest_mode
}
#[inline] pub fn gfn_round_for_level(gfn: gfn_t, level: i32) -> gfn_t { gfn & (-(KVM_PAGES_PER_HPAGE(level) as gfn_t)) }

extern "C" {
    pub fn mmu_try_to_unsync_pages(kvm: *mut kvm, slot: *const kvm_memory_slot, gfn: gfn_t, synchronizing: bool, prefetch: bool) -> i32;
    pub fn kvm_mmu_gfn_disallow_lpage(slot: *const kvm_memory_slot, gfn: gfn_t);
    pub fn kvm_mmu_gfn_allow_lpage(slot: *const kvm_memory_slot, gfn: gfn_t);
    pub fn kvm_mmu_slot_gfn_write_protect(kvm: *mut kvm, slot: *mut kvm_memory_slot, gfn: u64, min_level: i32) -> bool;
    pub fn pte_list_count(rmap_head: *mut kvm_rmap_head) -> u32;
}
#[inline] pub unsafe fn kvm_flush_remote_tlbs_gfn(kvm: *mut kvm, gfn: gfn_t, level: i32) {
    kvm_flush_remote_tlbs_range(kvm, gfn_round_for_level(gfn, level), KVM_PAGES_PER_HPAGE(level) as u64);
}
#[inline] pub unsafe fn is_nx_huge_page_enabled(kvm: *mut kvm) -> bool { READ_ONCE(&nx_huge_pages) != 0 && !(*kvm).arch.disable_nx_huge_pages }

#[repr(C)]
pub struct kvm_page_fault {
    pub addr: gpa_t, pub error_code: u64, pub prefetch: bool,
    pub exec: bool, pub write: bool, pub present: bool, pub rsvd: bool, pub user: bool,
    pub is_tdp: bool, pub is_private: bool, pub nx_huge_page_workaround_enabled: bool,
    pub huge_page_disallowed: bool, pub max_level: u8, pub req_level: u8, pub goal_level: u8,
    pub gfn: gfn_t, pub slot: *mut kvm_memory_slot, pub mmu_seq: c_ulong, pub pfn: kvm_pfn_t,
    pub refcounted_page: *mut page, pub map_writable: bool, pub write_fault_to_shadow_pgtable: bool,
}

pub const RET_PF_CONTINUE: i32 = 0;
pub const RET_PF_RETRY: i32 = 1;
pub const RET_PF_EMULATE: i32 = 2;
pub const RET_PF_WRITE_PROTECTED: i32 = 3;
pub const RET_PF_INVALID: i32 = 4;
pub const RET_PF_FIXED: i32 = 5;
pub const RET_PF_SPURIOUS: i32 = 6;

#[inline] pub unsafe fn kvm_mmu_prepare_memory_fault_exit(vcpu: *mut kvm_vcpu, fault: *mut kvm_page_fault) {
    kvm_prepare_memory_fault_exit(vcpu, (*fault).gfn << PAGE_SHIFT, PAGE_SIZE, (*fault).write, (*fault).exec, (*fault).is_private);
}

extern "C" {
    pub fn kvm_mmu_max_mapping_level(kvm: *mut kvm, fault: *mut kvm_page_fault, slot: *const kvm_memory_slot, gfn: gfn_t) -> i32;
    pub fn kvm_mmu_hugepage_adjust(vcpu: *mut kvm_vcpu, fault: *mut kvm_page_fault);
    pub fn disallowed_hugepage_adjust(fault: *mut kvm_page_fault, spte: u64, cur_level: i32);
    pub fn track_possible_nx_huge_page(kvm: *mut kvm, sp: *mut kvm_mmu_page, mmu_type: kvm_mmu_type);
    pub fn untrack_possible_nx_huge_page(kvm: *mut kvm, sp: *mut kvm_mmu_page, mmu_type: kvm_mmu_type);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
