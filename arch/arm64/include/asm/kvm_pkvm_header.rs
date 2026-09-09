// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 - Google LLC
 * Author: Quentin Perret <qperret@google.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

/* Maximum number of VMs that can co-exist under pKVM. */
pub const KVM_MAX_PVMS: usize = 255;
pub const HYP_MEMBLOCK_REGIONS: usize = 128;

extern "C" {
    pub fn pkvm_init_host_vm(kvm: *mut kvm, r#type: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn pkvm_create_hyp_vm(kvm: *mut kvm) -> ::core::ffi::c_int;
    pub fn pkvm_hyp_vm_is_created(kvm: *mut kvm) -> bool;
    pub fn pkvm_destroy_hyp_vm(kvm: *mut kvm);
    pub fn pkvm_create_hyp_vcpu(vcpu: *mut kvm_vcpu) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn kvm_pkvm_ext_allowed(kvm: *mut kvm, ext: ::core::ffi::c_long) -> bool {
    match ext {
        KVM_CAP_IRQCHIP | KVM_CAP_ARM_PSCI | KVM_CAP_ARM_PSCI_0_2 |
        KVM_CAP_NR_VCPUS | KVM_CAP_MAX_VCPUS | KVM_CAP_MAX_VCPU_ID |
        KVM_CAP_MSI_DEVID | KVM_CAP_ARM_VM_IPA_SIZE |
        KVM_CAP_ARM_PTRAUTH_ADDRESS | KVM_CAP_ARM_PTRAUTH_GENERIC => true,
        KVM_CAP_ARM_MTE => false,
        KVM_CAP_ARM_EAGER_SPLIT_CHUNK_SIZE | KVM_CAP_ARM_SUPPORTED_BLOCK_SIZES => false,
        _ => kvm.is_null() || !kvm_vm_is_protected(kvm),
    }
}

#[inline]
pub unsafe fn kvm_pkvm_ioctl_allowed(kvm: *mut kvm, ioctl: ::core::ffi::c_uint) -> bool {
    let mut ext: ::core::ffi::c_long = 0;
    let r = kvm_get_cap_for_kvm_ioctl(ioctl, &mut ext);
    if r < 0 {
        // WARN_ON_ONCE(r < 0)
        return false;
    }
    kvm_pkvm_ext_allowed(kvm, ext)
}

extern "C" {
    pub static mut hyp_memory: [memblock_region; HYP_MEMBLOCK_REGIONS];
    pub static mut hyp_memblock_nr: ::core::ffi::c_uint;
}

#[inline]
pub unsafe fn hyp_vmemmap_memblock_size(reg: *mut memblock_region, vmemmap_entry_size: usize) -> ::core::ffi::c_ulong {
    let nr_pages = ((*reg).size >> PAGE_SHIFT);
    let mut start = ((*reg).base >> PAGE_SHIFT) * vmemmap_entry_size;
    let mut end = start + nr_pages * vmemmap_entry_size;
    start = ALIGN_DOWN(start, PAGE_SIZE);
    end = ALIGN(end, PAGE_SIZE);
    end - start
}

#[inline]
pub unsafe fn hyp_vmemmap_pages(vmemmap_entry_size: usize) -> ::core::ffi::c_ulong {
    let mut res: ::core::ffi::c_ulong = 0;
    let mut i: usize = 0;
    while i < hyp_memblock_nr as usize {
        res += hyp_vmemmap_memblock_size(&mut hyp_memory[i], vmemmap_entry_size);
        i += 1;
    }
    res >> PAGE_SHIFT
}

#[inline]
pub fn hyp_vm_table_pages() -> ::core::ffi::c_ulong {
    PAGE_ALIGN(KVM_MAX_PVMS * ::core::mem::size_of::<*mut ::core::ffi::c_void>()) >> PAGE_SHIFT
}

#[inline]
pub fn __hyp_pgtable_max_pages(mut nr_pages: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let mut total = 0;
    let mut i = KVM_PGTABLE_FIRST_LEVEL;
    while i <= KVM_PGTABLE_LAST_LEVEL {
        nr_pages = DIV_ROUND_UP(nr_pages, PTRS_PER_PTE);
        total += nr_pages;
        i += 1;
    }
    total
}

#[inline]
pub unsafe fn __hyp_pgtable_total_pages() -> ::core::ffi::c_ulong {
    let mut res = 0;
    let mut i = 0usize;
    while i < hyp_memblock_nr as usize {
        res += __hyp_pgtable_max_pages(hyp_memory[i].size >> PAGE_SHIFT);
        i += 1;
    }
    res
}

#[inline]
pub unsafe fn hyp_s1_pgtable_pages() -> ::core::ffi::c_ulong {
    __hyp_pgtable_total_pages() + __hyp_pgtable_max_pages(SZ_1G >> PAGE_SHIFT)
}

#[inline]
pub unsafe fn host_s2_pgtable_pages() -> ::core::ffi::c_ulong {
    __hyp_pgtable_total_pages() + 16 + __hyp_pgtable_max_pages(SZ_1G >> PAGE_SHIFT)
}

#[inline]
pub const fn pkvm_selftest_pages() -> ::core::ffi::c_ulong {
    // CONFIG_NVHE_EL2_DEBUG selects the 32-page variant at build time.
    0
}

pub const KVM_FFA_MBOX_NR_PAGES: usize = 1;

#[inline]
pub unsafe fn hyp_ffa_proxy_pages() -> usize {
    let desc_max = ::core::mem::size_of::<ffa_mem_region>() +
        ::core::mem::size_of::<ffa_mem_region_attributes>() +
        ::core::mem::size_of::<ffa_composite_mem_region>() +
        SG_MAX_SEGMENTS * ::core::mem::size_of::<ffa_mem_region_addr_range>();
    (2 * KVM_FFA_MBOX_NR_PAGES) + DIV_ROUND_UP(desc_max, PAGE_SIZE)
}

#[inline]
pub unsafe fn pkvm_host_sve_state_size() -> usize {
    if !system_supports_sve() { return 0; }
    SVE_SIG_REGS_SIZE(sve_vq_from_vl(kvm_host_sve_max_vl))
}

#[repr(C)]
pub struct pkvm_mapping {
    pub node: rb_node,
    pub gfn: u64,
    pub pfn: u64,
    // C bitfields: nr_pages:48 and nc:1, represented in their containing word.
    pub nr_pages_nc: u64,
    pub __subtree_last: u64,
}

extern "C" {
    pub fn pkvm_pgtable_stage2_init(pgt: *mut kvm_pgtable, mmu: *mut kvm_s2_mmu, mm_ops: *mut kvm_pgtable_mm_ops) -> ::core::ffi::c_int;
    pub fn pkvm_pgtable_stage2_destroy_range(pgt: *mut kvm_pgtable, addr: u64, size: u64);
    pub fn pkvm_pgtable_stage2_destroy_pgd(pgt: *mut kvm_pgtable);
    pub fn pkvm_pgtable_stage2_map(pgt: *mut kvm_pgtable, addr: u64, size: u64, phys: u64, prot: kvm_pgtable_prot, mc: *mut ::core::ffi::c_void, flags: kvm_pgtable_walk_flags) -> ::core::ffi::c_int;
    pub fn pkvm_pgtable_stage2_unmap(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> ::core::ffi::c_int;
    pub fn pkvm_pgtable_stage2_wrprotect(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> ::core::ffi::c_int;
    pub fn pkvm_pgtable_stage2_flush(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> ::core::ffi::c_int;
    pub fn pkvm_pgtable_stage2_test_clear_young(pgt: *mut kvm_pgtable, addr: u64, size: u64, mkold: bool) -> bool;
    pub fn pkvm_pgtable_stage2_relax_perms(pgt: *mut kvm_pgtable, addr: u64, prot: kvm_pgtable_prot, flags: kvm_pgtable_walk_flags) -> ::core::ffi::c_int;
    pub fn pkvm_pgtable_stage2_mkyoung(pgt: *mut kvm_pgtable, addr: u64, flags: kvm_pgtable_walk_flags);
    pub fn pkvm_pgtable_stage2_split(pgt: *mut kvm_pgtable, addr: u64, size: u64, mc: *mut kvm_mmu_memory_cache) -> ::core::ffi::c_int;
    pub fn pkvm_pgtable_stage2_free_unlinked(mm_ops: *mut kvm_pgtable_mm_ops, pgtable: *mut ::core::ffi::c_void, level: i8);
    pub fn pkvm_pgtable_stage2_create_unlinked(pgt: *mut kvm_pgtable, phys: u64, level: i8, prot: kvm_pgtable_prot, mc: *mut ::core::ffi::c_void, force_pte: bool) -> *mut kvm_pte_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
