// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 - Google LLC
 * Author: Quentin Perret <qperret@google.com>
 */

// Kernel headers and build-time configuration are supplied by the surrounding tree.

static mut KVM_PROTECTED_MODE_INITIALIZED: bool = false;

static mut HYP_MEMORY: *mut memblock_region = unsafe { kvm_nvhe_sym(hyp_memory) };
static mut HYP_MEMBLOCK_NR_PTR: *mut u32 = unsafe { &mut kvm_nvhe_sym(hyp_memblock_nr) };

static mut hyp_mem_base: phys_addr_t = 0;
static mut hyp_mem_size: phys_addr_t = 0;

unsafe fn register_memblock_regions() -> i32 {
    let mut reg: *mut memblock_region = core::ptr::null_mut();
    for_each_mem_region!(reg, {
        if *HYP_MEMBLOCK_NR_PTR >= HYP_MEMBLOCK_REGIONS { return -ENOMEM; }
        *HYP_MEMORY.add(*HYP_MEMBLOCK_NR_PTR as usize) = *reg;
        *HYP_MEMBLOCK_NR_PTR += 1;
    });
    0
}

pub unsafe fn kvm_hyp_reserve() {
    let mut hyp_mem_pages: u64 = 0;
    if !is_hyp_mode_available() || is_kernel_in_hyp_mode() || kvm_get_mode() != KVM_MODE_PROTECTED { return; }
    let ret = register_memblock_regions();
    if ret != 0 {
        *HYP_MEMBLOCK_NR_PTR = 0;
        kvm_err!("Failed to register hyp memblocks: %d\n", ret);
        return;
    }
    hyp_mem_pages += hyp_s1_pgtable_pages();
    hyp_mem_pages += host_s2_pgtable_pages();
    hyp_mem_pages += hyp_vm_table_pages();
    hyp_mem_pages += hyp_vmemmap_pages(STRUCT_HYP_PAGE_SIZE);
    hyp_mem_pages += pkvm_selftest_pages();
    hyp_mem_pages += hyp_ffa_proxy_pages();
    hyp_mem_size = hyp_mem_pages << PAGE_SHIFT;
    hyp_mem_base = memblock_phys_alloc(ALIGN!(hyp_mem_size, PMD_SIZE), PMD_SIZE);
    if hyp_mem_base == 0 {
        hyp_mem_base = memblock_phys_alloc(hyp_mem_size, PAGE_SIZE);
    } else {
        hyp_mem_size = ALIGN!(hyp_mem_size, PMD_SIZE);
    }
    if hyp_mem_base == 0 { kvm_err!("Failed to reserve hyp memory\n"); return; }
    kvm_info!("Reserved %lld MiB at 0x%llx\n", hyp_mem_size >> 20, hyp_mem_base);
}

unsafe fn __pkvm_destroy_hyp_vm(kvm: *mut kvm) {
    if pkvm_hyp_vm_is_created(kvm) {
        WARN_ON!(kvm_call_hyp_nvhe(__pkvm_finalize_teardown_vm, (*kvm).arch.pkvm.handle));
    } else if (*kvm).arch.pkvm.handle != 0 {
        kvm_call_hyp_nvhe(__pkvm_unreserve_vm, (*kvm).arch.pkvm.handle);
    }
    (*kvm).arch.pkvm.handle = 0;
    (*kvm).arch.pkvm.is_created = false;
    free_hyp_memcache(&mut (*kvm).arch.pkvm.teardown_mc);
    free_hyp_memcache(&mut (*kvm).arch.pkvm.stage2_teardown_mc);
}

unsafe fn __pkvm_create_hyp_vcpu(vcpu: *mut kvm_vcpu) -> i32 {
    let hyp_vcpu_sz = PAGE_ALIGN!(PKVM_HYP_VCPU_SIZE);
    let handle = (*vcpu).kvm.as_ref().unwrap().arch.pkvm.handle;
    (*vcpu).arch.pkvm_memcache.flags |= HYP_MEMCACHE_ACCOUNT_STAGE2;
    let hyp_vcpu = alloc_pages_exact(hyp_vcpu_sz, GFP_KERNEL_ACCOUNT);
    if hyp_vcpu.is_null() { return -ENOMEM; }
    let ret = kvm_call_hyp_nvhe(__pkvm_init_vcpu, handle, vcpu, hyp_vcpu);
    if ret == 0 { vcpu_set_flag(vcpu, VCPU_PKVM_FINALIZED); }
    else { free_pages_exact(hyp_vcpu, hyp_vcpu_sz); }
    ret
}

unsafe fn __pkvm_create_hyp_vm(kvm: *mut kvm) -> i32 {
    if (*kvm).created_vcpus < 1 { return -EINVAL; }
    let pgd_sz = kvm_pgtable_stage2_pgd_size((*kvm).arch.mmu.vtcr);
    let pgd = alloc_pages_exact(pgd_sz, GFP_KERNEL_ACCOUNT);
    if pgd.is_null() { return -ENOMEM; }
    let hyp_vm_sz = PAGE_ALIGN!(size_add(PKVM_HYP_VM_SIZE, size_mul(core::mem::size_of::<*mut core::ffi::c_void>(), (*kvm).created_vcpus)));
    let hyp_vm = alloc_pages_exact(hyp_vm_sz, GFP_KERNEL_ACCOUNT);
    if hyp_vm.is_null() { free_pages_exact(pgd, pgd_sz); return -ENOMEM; }
    let ret = kvm_call_hyp_nvhe(__pkvm_init_vm, kvm, hyp_vm, pgd);
    if ret != 0 { free_pages_exact(hyp_vm, hyp_vm_sz); free_pages_exact(pgd, pgd_sz); return ret; }
    (*kvm).arch.pkvm.is_created = true;
    (*kvm).arch.pkvm.stage2_teardown_mc.flags |= HYP_MEMCACHE_ACCOUNT_STAGE2;
    kvm_account_pgtable_pages(pgd, pgd_sz / PAGE_SIZE);
    0
}

pub unsafe fn pkvm_hyp_vm_is_created(kvm: *mut kvm) -> bool { (*kvm).arch.pkvm.is_created }

pub unsafe fn pkvm_create_hyp_vm(kvm: *mut kvm) -> i32 {
    let mut ret = 0;
    mutex_lock(&mut (*kvm).slots_lock);
    mutex_lock(&mut (*kvm).arch.config_lock);
    if !pkvm_hyp_vm_is_created(kvm) { ret = __pkvm_create_hyp_vm(kvm); }
    mutex_unlock(&mut (*kvm).arch.config_lock); mutex_unlock(&mut (*kvm).slots_lock); ret
}

pub unsafe fn pkvm_create_hyp_vcpu(vcpu: *mut kvm_vcpu) -> i32 {
    let mut ret = 0; let kvm = (*vcpu).kvm;
    mutex_lock(&mut (*kvm).arch.config_lock);
    if !vcpu_get_flag(vcpu, VCPU_PKVM_FINALIZED) { ret = __pkvm_create_hyp_vcpu(vcpu); }
    mutex_unlock(&mut (*kvm).arch.config_lock); ret
}

pub unsafe fn pkvm_destroy_hyp_vm(kvm: *mut kvm) { mutex_lock(&mut (*kvm).arch.config_lock); __pkvm_destroy_hyp_vm(kvm); mutex_unlock(&mut (*kvm).arch.config_lock); }

pub unsafe fn pkvm_init_host_vm(kvm: *mut kvm, type_: c_ulong) -> i32 {
    let ret = kvm_call_hyp_nvhe(__pkvm_reserve_vm); if ret < 0 { return ret; }
    (*kvm).arch.pkvm.handle = ret; (*kvm).arch.pkvm.is_protected = (type_ & KVM_VM_TYPE_ARM_PROTECTED) != 0;
    if (*kvm).arch.pkvm.is_protected { pr_warn_once!("kvm: protected VMs are experimental and for development only, tainting kernel\n"); add_taint(TAINT_USER, LOCKDEP_STILL_OK); } 0
}

unsafe fn __pkvm_pgtable_stage2_unshare(_pgt: *mut kvm_pgtable, _start: u64, _end: u64) -> i32 { 0 }
pub unsafe fn pkvm_pgtable_stage2_init(pgt: *mut kvm_pgtable, mmu: *mut kvm_s2_mmu, _mm_ops: *mut kvm_pgtable_mm_ops) -> i32 { (*pgt).pkvm_mappings = RB_ROOT_CACHED; (*pgt).mmu = mmu; 0 }
pub unsafe fn pkvm_pgtable_stage2_destroy_range(pgt: *mut kvm_pgtable, addr: u64, size: u64) { let kvm = kvm_s2_mmu_to_kvm((*pgt).mmu); let handle = (*kvm).arch.pkvm.handle; if handle == 0 { return; } if pkvm_hyp_vm_is_created(kvm) && !(*kvm).arch.pkvm.is_dying { WARN_ON!(kvm_call_hyp_nvhe(__pkvm_start_teardown_vm, handle)); (*kvm).arch.pkvm.is_dying = true; } if kvm_vm_is_protected(kvm) { __pkvm_pgtable_stage2_reclaim(pgt, addr, addr + size); } else { __pkvm_pgtable_stage2_unshare(pgt, addr, addr + size); } }
pub unsafe fn pkvm_pgtable_stage2_destroy_pgd(pgt: *mut kvm_pgtable) { WARN_ON_ONCE!(!RB_EMPTY_ROOT!((*pgt).pkvm_mappings.rb_root)); }
unsafe fn __pkvm_pgtable_stage2_reclaim(_pgt: *mut kvm_pgtable, _start: u64, _end: u64) -> i32 { 0 }
pub unsafe fn pkvm_pgtable_stage2_map(pgt: *mut kvm_pgtable, addr: u64, size: u64, phys: u64, prot: kvm_pgtable_prot, _mc: *mut c_void, _flags: kvm_pgtable_walk_flags) -> i32 {
    let kvm = kvm_s2_mmu_to_kvm((*pgt).mmu);
    if kvm_vm_is_protected(kvm) && (size != PAGE_SIZE || prot != KVM_PGTABLE_PROT_RWX) { return -EINVAL; }
    if !kvm_vm_is_protected(kvm) && size != PAGE_SIZE && size != PMD_SIZE { return -EINVAL; }
    let gfn = addr >> PAGE_SHIFT; let pfn = phys >> PAGE_SHIFT;
    if kvm_vm_is_protected(kvm) { kvm_call_hyp_nvhe(__pkvm_host_donate_guest, pfn, gfn) }
    else { kvm_call_hyp_nvhe(__pkvm_host_share_guest, pfn, gfn, size / PAGE_SIZE, prot) }
}
pub unsafe fn pkvm_pgtable_stage2_wrprotect(pgt: *mut kvm_pgtable, _addr: u64, _size: u64) -> i32 { let kvm = kvm_s2_mmu_to_kvm((*pgt).mmu); if WARN_ON!(kvm_vm_is_protected(kvm)) { -EPERM } else { 0 } }
pub unsafe fn pkvm_pgtable_stage2_flush(_pgt: *mut kvm_pgtable, _addr: u64, _size: u64) -> i32 { 0 }
pub unsafe fn pkvm_pgtable_stage2_test_clear_young(pgt: *mut kvm_pgtable, _addr: u64, _size: u64, _mkold: bool) -> bool { let kvm = kvm_s2_mmu_to_kvm((*pgt).mmu); !kvm_vm_is_protected(kvm) && false }
pub unsafe fn pkvm_pgtable_stage2_relax_perms(pgt: *mut kvm_pgtable, addr: u64, prot: kvm_pgtable_prot, _flags: kvm_pgtable_walk_flags) -> i32 { if WARN_ON!(kvm_vm_is_protected(kvm_s2_mmu_to_kvm((*pgt).mmu))) { -EPERM } else { kvm_call_hyp_nvhe(__pkvm_host_relax_perms_guest, addr >> PAGE_SHIFT, prot) } }
pub unsafe fn pkvm_pgtable_stage2_mkyoung(pgt: *mut kvm_pgtable, addr: u64, _flags: kvm_pgtable_walk_flags) { if !kvm_vm_is_protected(kvm_s2_mmu_to_kvm((*pgt).mmu)) { WARN_ON!(kvm_call_hyp_nvhe(__pkvm_host_mkyoung_guest, addr >> PAGE_SHIFT)); } }
pub unsafe fn pkvm_pgtable_stage2_create_unlinked(_pgt: *mut kvm_pgtable, _phys: u64, _level: i8, _prot: kvm_pgtable_prot, _mc: *mut c_void, _force_pte: bool) -> *mut kvm_pte_t { WARN_ON_ONCE!(true); core::ptr::null_mut() }
pub unsafe fn pkvm_pgtable_stage2_unmap(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> i32 { let kvm = kvm_s2_mmu_to_kvm((*pgt).mmu); if WARN_ON!(kvm_vm_is_protected(kvm)) { return -EPERM; } __pkvm_pgtable_stage2_unshare(pgt, addr, addr + size) }
pub unsafe fn pkvm_pgtable_stage2_free_unlinked(_mm_ops: *mut kvm_pgtable_mm_ops, _pgtable: *mut c_void, _level: i8) { WARN_ON_ONCE!(true); }
pub unsafe fn pkvm_pgtable_stage2_split(_pgt: *mut kvm_pgtable, _addr: u64, _size: u64, _mc: *mut kvm_mmu_memory_cache) -> i32 { WARN_ON_ONCE!(true); -EINVAL }
pub unsafe fn pkvm_force_reclaim_guest_page(phys: phys_addr_t) -> bool { let ret = kvm_call_hyp_nvhe(__pkvm_force_reclaim_guest_page, phys); ret == 0 || ret == -EAGAIN }

unsafe fn _kvm_host_prot_finalize(err: *mut i32) { if WARN_ON!(kvm_call_hyp_nvhe(__pkvm_prot_finalize)) { WRITE_ONCE!(*err, -EINVAL); } }
unsafe fn pkvm_drop_host_privileges() -> i32 { let mut ret = 0; static_branch_enable(&mut KVM_PROTECTED_MODE_INITIALIZED); on_each_cpu(_kvm_host_prot_finalize, &mut ret, 1); ret }
unsafe fn finalize_pkvm() -> i32 {
    if !is_protected_kvm_enabled() || !is_kvm_arm_initialised() { return 0; }
    kmemleak_free_part(__hyp_bss_start, __hyp_bss_end - __hyp_bss_start);
    kmemleak_free_part(__hyp_data_start, __hyp_data_end - __hyp_data_start);
    kmemleak_free_part(__hyp_rodata_start, __hyp_rodata_end - __hyp_rodata_start);
    kmemleak_free_part_phys(hyp_mem_base, hyp_mem_size);
    let ret = pkvm_drop_host_privileges(); if ret != 0 { pr_err!("Failed to finalize Hyp protection: %d\n", ret); } ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
