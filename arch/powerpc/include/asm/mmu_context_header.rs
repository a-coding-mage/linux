/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the kernel header. C includes and header guards are omitted;
// referenced kernel types and functions are supplied by other dependencies.

#[cfg(kernel)]
extern "C" {
    pub fn init_new_context(tsk: *mut task_struct, mm: *mut mm_struct) -> i32;
    pub fn destroy_context(mm: *mut mm_struct);
    pub fn switch_slb(tsk: *mut task_struct, mm: *mut mm_struct);
    pub fn switch_mm_irqs_off(prev: *mut mm_struct, next: *mut mm_struct,
                               tsk: *mut task_struct);
    pub fn arch_exit_mmap(mm: *mut mm_struct);
}

#[cfg(all(kernel, CONFIG_SPAPR_TCE_IOMMU))]
extern "C" {
    pub fn mm_iommu_preregistered(mm: *mut mm_struct) -> bool;
    pub fn mm_iommu_new(mm: *mut mm_struct, ua: c_ulong, entries: c_ulong,
                        pmem: *mut *mut mm_iommu_table_group_mem_t) -> c_long;
    pub fn mm_iommu_newdev(mm: *mut mm_struct, ua: c_ulong, entries: c_ulong,
                           dev_hpa: c_ulong,
                           pmem: *mut *mut mm_iommu_table_group_mem_t) -> c_long;
    pub fn mm_iommu_put(mm: *mut mm_struct,
                        mem: *mut mm_iommu_table_group_mem_t) -> c_long;
    pub fn mm_iommu_init(mm: *mut mm_struct);
    pub fn mm_iommu_lookup(mm: *mut mm_struct, ua: c_ulong, size: c_ulong)
        -> *mut mm_iommu_table_group_mem_t;
    pub fn mm_iommu_get(mm: *mut mm_struct, ua: c_ulong, entries: c_ulong)
        -> *mut mm_iommu_table_group_mem_t;
    pub fn mm_iommu_ua_to_hpa(mem: *mut mm_iommu_table_group_mem_t, ua: c_ulong,
                              pageshift: c_uint, hpa: *mut c_ulong) -> c_long;
    pub fn mm_iommu_is_devmem(mm: *mut mm_struct, hpa: c_ulong,
                              pageshift: c_uint, size: *mut c_ulong) -> bool;
    pub fn mm_iommu_mapped_inc(mem: *mut mm_iommu_table_group_mem_t) -> c_long;
    pub fn mm_iommu_mapped_dec(mem: *mut mm_iommu_table_group_mem_t);
}

#[cfg(all(kernel, not(CONFIG_SPAPR_TCE_IOMMU)))]
pub unsafe fn mm_iommu_is_devmem(_mm: *mut mm_struct, _hpa: c_ulong,
                                 _pageshift: c_uint, _size: *mut c_ulong) -> bool { false }
#[cfg(all(kernel, not(CONFIG_SPAPR_TCE_IOMMU)))]
pub unsafe fn mm_iommu_init(_mm: *mut mm_struct) {}

#[cfg(all(kernel, CONFIG_PPC_BOOK3S_64))]
extern "C" {
    pub fn radix__switch_mmu_context(prev: *mut mm_struct, next: *mut mm_struct);
    pub fn hash__alloc_context_id() -> i32;
    pub fn hash__reserve_context_id(id: i32);
    pub fn __destroy_context(context_id: i32);
}

#[cfg(all(kernel, CONFIG_PPC_BOOK3S_64))]
pub unsafe fn switch_mmu_context(prev: *mut mm_struct, next: *mut mm_struct,
                                 tsk: *mut task_struct) {
    if radix_enabled() { radix__switch_mmu_context(prev, next); }
    else { switch_slb(tsk, next); }
}

#[cfg(all(kernel, CONFIG_PPC_BOOK3S_64))]
pub unsafe fn mmu_context_init() {}

#[cfg(all(kernel, CONFIG_PPC_BOOK3S_64, CONFIG_PPC_64S_HASH_MMU))]
pub unsafe fn alloc_extended_context(mm: *mut mm_struct, ea: c_ulong) -> i32 {
    let index = ea >> MAX_EA_BITS_PER_CONTEXT;
    let context_id = hash__alloc_context_id();
    if context_id < 0 { return context_id; }
    VM_WARN_ON((*mm).context.extended_id[index as usize] != 0);
    (*mm).context.extended_id[index as usize] = context_id;
    context_id
}

#[cfg(all(kernel, CONFIG_PPC_BOOK3S_64, CONFIG_PPC_64S_HASH_MMU))]
pub unsafe fn need_extra_context(mm: *mut mm_struct, ea: c_ulong) -> bool {
    get_user_context(&mut (*mm).context, ea) == 0
}

#[cfg(all(kernel, not(CONFIG_PPC_BOOK3S_64)))]
extern "C" {
    pub fn switch_mmu_context(prev: *mut mm_struct, next: *mut mm_struct,
                              tsk: *mut task_struct);
    pub fn __init_new_context() -> c_ulong;
    pub fn __destroy_context(context_id: c_ulong);
    pub fn mmu_context_init();
}

#[cfg(all(kernel, not(CONFIG_PPC_BOOK3S_64)))]
pub unsafe fn alloc_extended_context(_mm: *mut mm_struct, _ea: c_ulong) -> i32 {
    // non book3s_64 should never find this called
    WARN_ON(1);
    -ENOMEM
}
#[cfg(all(kernel, not(CONFIG_PPC_BOOK3S_64)))]
pub unsafe fn need_extra_context(_mm: *mut mm_struct, _ea: c_ulong) -> bool { false }

#[cfg(all(kernel, CONFIG_PPC_BOOK3S_64))]
pub unsafe fn inc_mm_active_cpus(mm: *mut mm_struct) {
    atomic_inc(&mut (*mm).context.active_cpus);
}
#[cfg(all(kernel, CONFIG_PPC_BOOK3S_64))]
pub unsafe fn dec_mm_active_cpus(mm: *mut mm_struct) {
    VM_WARN_ON_ONCE(atomic_read(&(*mm).context.active_cpus) <= 0);
    atomic_dec(&mut (*mm).context.active_cpus);
}
#[cfg(all(kernel, CONFIG_PPC_BOOK3S_64))]
pub unsafe fn mm_context_add_copro(mm: *mut mm_struct) {
    if atomic_inc_return(&mut (*mm).context.copros) == 1 { inc_mm_active_cpus(mm); }
}
#[cfg(all(kernel, CONFIG_PPC_BOOK3S_64))]
pub unsafe fn mm_context_remove_copro(mm: *mut mm_struct) {
    if radix_enabled() {
        radix__flush_all_mm(mm);
        let c = atomic_dec_if_positive(&mut (*mm).context.copros);
        WARN_ON(c < 0);
        if c == 0 { dec_mm_active_cpus(mm); }
    }
}
#[cfg(all(kernel, CONFIG_PPC_BOOK3S_64))]
pub unsafe fn mm_context_add_vas_window(mm: *mut mm_struct) {
    atomic_inc(&mut (*mm).context.vas_windows);
    mm_context_add_copro(mm);
}
#[cfg(all(kernel, CONFIG_PPC_BOOK3S_64))]
pub unsafe fn mm_context_remove_vas_window(mm: *mut mm_struct) {
    mm_context_remove_copro(mm);
    let v = atomic_dec_if_positive(&mut (*mm).context.vas_windows);
    WARN_ON(v < 0);
}
#[cfg(all(kernel, not(CONFIG_PPC_BOOK3S_64)))]
pub unsafe fn inc_mm_active_cpus(_mm: *mut mm_struct) {}
#[cfg(all(kernel, not(CONFIG_PPC_BOOK3S_64)))]
pub unsafe fn dec_mm_active_cpus(_mm: *mut mm_struct) {}
#[cfg(all(kernel, not(CONFIG_PPC_BOOK3S_64)))]
pub unsafe fn mm_context_add_copro(_mm: *mut mm_struct) {}
#[cfg(all(kernel, not(CONFIG_PPC_BOOK3S_64)))]
pub unsafe fn mm_context_remove_copro(_mm: *mut mm_struct) {}

#[cfg(all(kernel, CONFIG_KVM_BOOK3S_HV_POSSIBLE, CONFIG_PPC_RADIX_MMU))]
extern "C" { pub fn do_h_rpt_invalidate_prt(pid: c_ulong, lpid: c_ulong,
    type_: c_ulong, pg_sizes: c_ulong, start: c_ulong, end: c_ulong); }
#[cfg(all(kernel, not(all(CONFIG_KVM_BOOK3S_HV_POSSIBLE, CONFIG_PPC_RADIX_MMU))))]
pub unsafe fn do_h_rpt_invalidate_prt(_pid: c_ulong, _lpid: c_ulong,
    _type: c_ulong, _pg_sizes: c_ulong, _start: c_ulong, _end: c_ulong) {}

pub unsafe fn switch_mm(prev: *mut mm_struct, next: *mut mm_struct,
                        tsk: *mut task_struct) {
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    switch_mm_irqs_off(prev, next, tsk);
    local_irq_restore(flags);
}

#[cfg(kernel)]
pub unsafe fn activate_mm(prev: *mut mm_struct, next: *mut mm_struct) {
    switch_mm_irqs_off(prev, next, current);
}

#[cfg(all(kernel, CONFIG_PPC_BOOK3E_64))]
pub unsafe fn enter_lazy_tlb(_mm: *mut mm_struct, _tsk: *mut task_struct) {
    // 64-bit Book3E keeps track of current PGD in the PACA
    (*get_paca()).pgd = core::ptr::null_mut();
}

#[cfg(all(kernel, CONFIG_PPC_MEM_KEYS))]
extern "C" {
    pub fn arch_vma_access_permitted(vma: *mut vm_area_struct, write: bool,
                                      execute: bool, foreign: bool) -> bool;
    pub fn arch_dup_pkeys(oldmm: *mut mm_struct, mm: *mut mm_struct);
}
#[cfg(all(kernel, not(CONFIG_PPC_MEM_KEYS)))]
pub unsafe fn arch_vma_access_permitted(_vma: *mut vm_area_struct, _write: bool,
                                        _execute: bool, _foreign: bool) -> bool { true }
#[cfg(all(kernel, not(CONFIG_PPC_MEM_KEYS)))]
pub unsafe fn pte_to_hpte_pkey_bits(_pteflags: u64, _flags: c_ulong) -> u64 { 0 }

pub unsafe fn arch_dup_mmap(oldmm: *mut mm_struct, mm: *mut mm_struct) -> i32 {
    #[cfg(all(kernel, CONFIG_PPC_MEM_KEYS))] arch_dup_pkeys(oldmm, mm);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
