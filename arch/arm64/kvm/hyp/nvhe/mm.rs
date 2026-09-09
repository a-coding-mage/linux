// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 Google LLC
 * Author: Quentin Perret <qperret@google.com>
 */

// Kernel headers and build-time configuration are supplied by the surrounding
// translation unit.

#[repr(C)]
pub struct hyp_fixmap_slot {
    pub addr: u64,
    pub ptep: *mut kvm_pte_t,
}

pub static mut pkvm_pgtable: kvm_pgtable = unsafe { core::mem::zeroed() };
pub static mut pkvm_pgd_lock: hyp_spinlock_t = unsafe { core::mem::zeroed() };
pub static mut hyp_memory: [memblock_region; HYP_MEMBLOCK_REGIONS] = unsafe { core::mem::zeroed() };
pub static mut hyp_memblock_nr: c_uint = 0;

static mut __io_map_base: u64 = 0;
static mut fixmap_slots: hyp_fixmap_slot = unsafe { core::mem::zeroed() };

unsafe fn __pkvm_create_mappings(
    start: c_ulong,
    size: c_ulong,
    phys: c_ulong,
    prot: kvm_pgtable_prot,
) -> c_int {
    hyp_spin_lock(&raw mut pkvm_pgd_lock);
    let err = kvm_pgtable_hyp_map(&raw mut pkvm_pgtable, start, size, phys, prot);
    hyp_spin_unlock(&raw mut pkvm_pgd_lock);
    err
}

unsafe fn __pkvm_alloc_private_va_range(start: c_ulong, size: usize) -> c_int {
    hyp_assert_lock_held(&raw mut pkvm_pgd_lock);
    if start == 0 || start < __io_map_base { return -EINVAL; }
    let cur = start + PAGE_ALIGN(size as c_ulong);
    if cur > __hyp_vmemmap { return -ENOMEM; }
    __io_map_base = cur;
    0
}

pub unsafe fn pkvm_alloc_private_va_range(size: usize, haddr: *mut c_ulong) -> c_int {
    hyp_spin_lock(&raw mut pkvm_pgd_lock);
    let addr = __io_map_base;
    let ret = __pkvm_alloc_private_va_range(addr, size);
    hyp_spin_unlock(&raw mut pkvm_pgd_lock);
    *haddr = addr;
    ret
}

pub unsafe fn __pkvm_create_private_mapping(
    phys: phys_addr_t, mut size: usize, prot: kvm_pgtable_prot, haddr: *mut c_ulong,
) -> c_int {
    let off = offset_in_page(phys);
    size = PAGE_ALIGN(size + off as usize);
    let mut addr = 0;
    let mut err = pkvm_alloc_private_va_range(size, &mut addr);
    if err != 0 { return err; }
    err = __pkvm_create_mappings(addr, size as c_ulong, phys, prot);
    if err != 0 { return err; }
    *haddr = addr + off;
    err
}

pub unsafe fn pkvm_create_mappings_locked(from: *mut core::ffi::c_void, to: *mut core::ffi::c_void, prot: kvm_pgtable_prot) -> c_int {
    let start = (from as c_ulong) & PAGE_MASK;
    let end = PAGE_ALIGN(to as c_ulong);
    hyp_assert_lock_held(&raw mut pkvm_pgd_lock);
    let mut virt_addr = start;
    while virt_addr < end {
        let phys = hyp_virt_to_phys(virt_addr as *mut core::ffi::c_void);
        let err = kvm_pgtable_hyp_map(&raw mut pkvm_pgtable, virt_addr, PAGE_SIZE, phys, prot);
        if err != 0 { return err; }
        virt_addr += PAGE_SIZE;
    }
    0
}

pub unsafe fn pkvm_create_mappings(from: *mut core::ffi::c_void, to: *mut core::ffi::c_void, prot: kvm_pgtable_prot) -> c_int {
    hyp_spin_lock(&raw mut pkvm_pgd_lock);
    let ret = pkvm_create_mappings_locked(from, to, prot);
    hyp_spin_unlock(&raw mut pkvm_pgd_lock);
    ret
}

pub unsafe fn hyp_back_vmemmap(back: phys_addr_t) -> c_int {
    let mut end = 0;
    for i in 0..hyp_memblock_nr as usize {
        let mut start = ALIGN_DOWN(hyp_phys_to_page(hyp_memory[i].base), PAGE_SIZE);
        start = max(start, end);
        end = PAGE_ALIGN(hyp_phys_to_page(hyp_memory[i].base + hyp_memory[i].size));
        if start >= end { continue; }
        let size = end - start;
        let ret = __pkvm_create_mappings(start, size, back, PAGE_HYP);
        if ret != 0 { return ret; }
        memset(hyp_phys_to_virt(back), 0, size);
        back += size;
    }
    0
}

static mut __hyp_bp_vect_base: *mut core::ffi::c_void = core::ptr::null_mut();
pub unsafe fn pkvm_cpu_set_vector(slot: arm64_hyp_spectre_vector) -> c_int {
    let vector = match slot {
        HYP_VECTOR_DIRECT => __kvm_hyp_vector,
        HYP_VECTOR_SPECTRE_DIRECT => __bp_harden_hyp_vecs,
        HYP_VECTOR_INDIRECT | HYP_VECTOR_SPECTRE_INDIRECT => __hyp_bp_vect_base,
        _ => return -EINVAL,
    };
    let vector = __kvm_vector_slot2addr(vector, slot);
    *this_cpu_ptr(&raw mut kvm_hyp_vector) = vector as c_ulong;
    0
}

pub unsafe fn hyp_map_vectors() -> c_int {
    if !kvm_system_needs_idmapped_vectors() {
        __hyp_bp_vect_base = __bp_harden_hyp_vecs;
        return 0;
    }
    let phys = __hyp_pa(__bp_harden_hyp_vecs);
    let mut bp_base = 0;
    let ret = __pkvm_create_private_mapping(phys, __BP_HARDEN_HYP_VECS_SZ, PAGE_HYP_EXEC, &mut bp_base);
    if ret != 0 { return ret; }
    __hyp_bp_vect_base = bp_base as *mut _;
    0
}

unsafe fn fixmap_map_slot(slot: *mut hyp_fixmap_slot, phys: phys_addr_t) -> *mut core::ffi::c_void {
    let ptep = (*slot).ptep;
    let mut pte = *ptep;
    pte &= !kvm_phys_to_pte(KVM_PHYS_INVALID);
    pte |= kvm_phys_to_pte(phys) | KVM_PTE_VALID;
    WRITE_ONCE(ptep, pte);
    dsb(ishst);
    (*slot).addr as *mut _
}

pub unsafe fn hyp_fixmap_map(phys: phys_addr_t) -> *mut core::ffi::c_void {
    fixmap_map_slot(this_cpu_ptr(&raw mut fixmap_slots), phys).add(offset_in_page(phys) as usize)
}

unsafe fn fixmap_clear_slot(slot: *mut hyp_fixmap_slot) {
    let ptep = (*slot).ptep;
    let level = if FIELD_GET(KVM_PTE_TYPE, *ptep) == KVM_PTE_TYPE_PAGE { KVM_PGTABLE_LAST_LEVEL } else { KVM_PGTABLE_LAST_LEVEL - 1 };
    WRITE_ONCE(ptep, *ptep & !KVM_PTE_VALID);
    dsb(ishst);
    __tlbi_level(vale2is, (*slot).addr, level);
    __tlbi_sync_s1ish_hyp();
    isb();
}

pub unsafe fn hyp_fixmap_unmap() { fixmap_clear_slot(this_cpu_ptr(&raw mut fixmap_slots)); }

unsafe extern "C" fn __create_fixmap_slot_cb(ctx: *const kvm_pgtable_visit_ctx, _visit: kvm_pgtable_walk_flags) -> c_int {
    let slot = (*ctx).arg as *mut hyp_fixmap_slot;
    if !kvm_pte_valid((*ctx).old) || (*ctx).end - (*ctx).start != kvm_granule_size((*ctx).level) { return -EINVAL; }
    (*slot).addr = (*ctx).addr;
    (*slot).ptep = (*ctx).ptep;
    fixmap_clear_slot(slot);
    0
}

unsafe fn create_fixmap_slot(addr: u64, cpu: u64) -> c_int {
    let walker = kvm_pgtable_walker { cb: Some(__create_fixmap_slot_cb), flags: KVM_PGTABLE_WALK_LEAF, arg: per_cpu_ptr(&raw mut fixmap_slots, cpu) as *mut _ };
    kvm_pgtable_walk(&raw mut pkvm_pgtable, addr, PAGE_SIZE, &walker)
}

// The source compiles the following block when PAGE_SHIFT < 16.
#[cfg(any())]
static mut hyp_fixblock_slot: hyp_fixmap_slot = unsafe { core::mem::zeroed() };
#[cfg(any())]
static mut hyp_fixblock_lock: hyp_spinlock_t = unsafe { core::mem::zeroed() };

unsafe fn create_fixblock() -> c_int {
    0
}

pub unsafe fn hyp_fixblock_map(phys: phys_addr_t, size: *mut usize) -> *mut core::ffi::c_void {
    *size = PAGE_SIZE as usize;
    hyp_fixmap_map(phys)
}

pub unsafe fn hyp_fixblock_unmap() { hyp_fixmap_unmap(); }

pub unsafe fn hyp_create_fixmap() -> c_int {
    let mut addr = 0;
    for i in 0..hyp_nr_cpus {
        let mut ret = pkvm_alloc_private_va_range(PAGE_SIZE as usize, &mut addr);
        if ret != 0 { return ret; }
        ret = kvm_pgtable_hyp_map(&raw mut pkvm_pgtable, addr, PAGE_SIZE, __hyp_pa(__hyp_bss_start), PAGE_HYP);
        if ret != 0 { return ret; }
        ret = create_fixmap_slot(addr, i);
        if ret != 0 { return ret; }
    }
    create_fixblock()
}

pub unsafe fn hyp_create_idmap(hyp_va_bits: u32) -> c_int {
    let mut start = ALIGN_DOWN(hyp_virt_to_phys(__hyp_idmap_text_start as *mut _), PAGE_SIZE);
    let end = ALIGN(hyp_virt_to_phys(__hyp_idmap_text_end as *mut _), PAGE_SIZE);
    __io_map_base = start & BIT(hyp_va_bits - 2);
    __io_map_base ^= BIT(hyp_va_bits - 2);
    __hyp_vmemmap = __io_map_base | BIT(hyp_va_bits - 3);
    __pkvm_create_mappings(start, end - start, start, PAGE_HYP_EXEC)
}

pub unsafe fn pkvm_create_stack(phys: phys_addr_t, haddr: *mut c_ulong) -> c_int {
    hyp_spin_lock(&raw mut pkvm_pgd_lock);
    let prev_base = __io_map_base;
    let size = NVHE_STACK_SIZE * 2;
    let addr = ALIGN(__io_map_base, size);
    let mut ret = __pkvm_alloc_private_va_range(addr, size as usize);
    if ret == 0 {
        ret = kvm_pgtable_hyp_map(&raw mut pkvm_pgtable, addr + NVHE_STACK_SIZE, NVHE_STACK_SIZE, phys, PAGE_HYP);
        if ret != 0 { __io_map_base = prev_base; }
    }
    hyp_spin_unlock(&raw mut pkvm_pgd_lock);
    *haddr = addr + size;
    ret
}

unsafe fn admit_host_page(arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let host_mc = arg as *mut kvm_hyp_memcache;
    if (*host_mc).nr_pages == 0 { return core::ptr::null_mut(); }
    if __pkvm_host_donate_hyp(hyp_phys_to_pfn((*host_mc).head), 1) != 0 { return core::ptr::null_mut(); }
    pop_hyp_memcache(host_mc, hyp_phys_to_virt)
}

pub unsafe fn refill_memcache(mc: *mut kvm_hyp_memcache, min_pages: c_ulong, host_mc: *mut kvm_hyp_memcache) -> c_int {
    let mut tmp = *host_mc;
    let ret = __topup_hyp_memcache(mc, min_pages, admit_host_page, hyp_virt_to_phys, &mut tmp);
    *host_mc = tmp;
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
