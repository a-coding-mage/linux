// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Dependencies supplied by the corresponding kernel headers.

#[allow(non_camel_case_types)]
pub enum vm_fault {}
#[allow(non_camel_case_types)]
pub enum vm_area_struct {}
#[allow(non_camel_case_types)]
pub enum pte_t {}
#[allow(non_camel_case_types)]
pub enum folio {}
#[allow(non_camel_case_types)]
pub enum mm_struct {}
#[allow(non_camel_case_types)]
pub enum cpumask_t {}
#[allow(non_camel_case_types)]
pub enum page {}

extern "C" {
    fn pte_pfn(pte: pte_t) -> ::core::ffi::c_ulong;
    fn flush_tlb_page(vma: *mut vm_area_struct, address: ::core::ffi::c_ulong);
    fn pfn_valid(pfn: ::core::ffi::c_ulong) -> bool;
    fn pfn_to_page(pfn: ::core::ffi::c_ulong) -> *mut page;
    fn page_folio(page: *mut page) -> *mut folio;
    fn test_and_set_bit(nr: ::core::ffi::c_ulong, addr: *mut ::core::ffi::c_ulong) -> bool;
    fn icache_inv_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    fn folio_nr_pages(folio: *mut folio) -> ::core::ffi::c_uint;
    fn kmap_local_folio(folio: *mut folio, offset: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void;
    fn dcache_wb_range(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    fn kunmap_local(addr: *mut ::core::ffi::c_void);
    fn smp_processor_id() -> ::core::ffi::c_uint;
    fn cpumask_test_cpu(cpu: ::core::ffi::c_uint, mask: *mut cpumask_t) -> bool;
    fn cpumask_clear_cpu(cpu: ::core::ffi::c_uint, mask: *mut cpumask_t);
    fn smp_mb();
    fn local_icache_inv_all(arg: *mut ::core::ffi::c_void);
    fn preempt_disable();
    fn preempt_enable();
    fn cpumask_setall(mask: *mut cpumask_t);
    fn mm_cpumask(mm: *mut mm_struct) -> *mut cpumask_t;
    fn cpumask_of(cpu: ::core::ffi::c_uint) -> *mut cpumask_t;
    fn cpumask_andnot(dst: *mut cpumask_t, src1: *mut cpumask_t, src2: *mut cpumask_t);
    fn cpumask_empty(mask: *mut cpumask_t) -> bool;
    fn on_each_cpu_mask(mask: *mut cpumask_t, func: unsafe extern "C" fn(*mut ::core::ffi::c_void), info: *mut ::core::ffi::c_void, wait: ::core::ffi::c_int);
}

const PG_DCACHE_CLEAN: ::core::ffi::c_ulong = 0;
const VM_EXEC: ::core::ffi::c_ulong = 0;

#[no_mangle]
pub unsafe extern "C" fn update_mmu_cache_range(
    _vmf: *mut vm_fault,
    vma: *mut vm_area_struct,
    address: ::core::ffi::c_ulong,
    pte: *mut pte_t,
    nr: ::core::ffi::c_uint,
) {
    let pfn = pte_pfn(*pte);
    let folio: *mut folio;

    flush_tlb_page(vma, address);

    if !pfn_valid(pfn) {
        return;
    }

    folio = page_folio(pfn_to_page(pfn));

    if test_and_set_bit(PG_DCACHE_CLEAN, folio as *mut ::core::ffi::c_ulong) {
        return;
    }

    icache_inv_range(address, address.wrapping_add((nr as usize).wrapping_mul(4096) as u64));
    for i in 0..folio_nr_pages(folio) {
        let addr = kmap_local_folio(folio, (i as usize).wrapping_mul(4096) as u64) as ::core::ffi::c_ulong;

        dcache_wb_range(addr, addr.wrapping_add(4096));
        // vma->vm_flags & VM_EXEC
        if false {
            icache_inv_range(addr, addr.wrapping_add(4096));
        }
        kunmap_local(addr as *mut ::core::ffi::c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn flush_icache_deferred(mm: *mut mm_struct) {
    let cpu = smp_processor_id();
    let mask = mm as *mut cpumask_t;

    if cpumask_test_cpu(cpu, mask) {
        cpumask_clear_cpu(cpu, mask);
        /*
         * Ensure the remote hart's writes are visible to this hart.
         * This pairs with a barrier in flush_icache_mm.
         */
        smp_mb();
        local_icache_inv_all(::core::ptr::null_mut());
    }
}

#[no_mangle]
pub unsafe extern "C" fn flush_icache_mm_range(
    mm: *mut mm_struct,
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
) {
    let mut cpu: ::core::ffi::c_uint;
    let mut others: *mut cpumask_t = ::core::ptr::null_mut();
    let mask: *mut cpumask_t;

    preempt_disable();

    // CONFIG_CPU_HAS_ICACHE_INS is a build-time condition supplied externally.
    // if (mm == current->mm) {
    //     icache_inv_range(start, end);
    //     preempt_enable();
    //     return;
    // }

    mask = mm as *mut cpumask_t;
    cpumask_setall(mask);

    cpu = smp_processor_id();
    cpumask_clear_cpu(cpu, mask);
    local_icache_inv_all(::core::ptr::null_mut());

    cpumask_andnot(&mut others, mm_cpumask(mm), cpumask_of(cpu));

    // mm != current->active_mm || !cpumask_empty(&others)
    if !cpumask_empty(others) {
        on_each_cpu_mask(others, local_icache_inv_all, ::core::ptr::null_mut(), 1);
        cpumask_setall(mask);
    }

    preempt_enable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
