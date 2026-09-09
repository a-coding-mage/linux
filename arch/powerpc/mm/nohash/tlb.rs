// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file contains the routines for TLB flushing.
 * On machines where the MMU does not use a hash table to store virtual to
 * physical translations (ie, SW loaded TLBs or Book3E compilant processors,
 * this does -not- include 603 however which shares the implementation with
 * hash based processors)
 *
 *  -- BenH
 *
 * Copyright 2008,2009 Ben Herrenschmidt <benh@kernel.crashing.org>
 *                     IBM Corp.
 *
 *  Derived from arch/ppc/mm/init.c:
 *    Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
 *
 *  Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
 *  and Cort Dougan (PReP) (cort@cs.nmt.edu)
 *    Copyright (C) 1996 Paul Mackerras
 *
 *  Derived from "arch/i386/mm/init.c"
 *    Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
 */

/* C headers and symbols are supplied by the surrounding kernel translation. */

/*
 * This struct lists the sw-supported page sizes.  The hardawre MMU may support
 * other sizes not listed here.   The .ind field is only used on MMUs that have
 * indirect page table entries.
 */
#[cfg(feature = "CONFIG_PPC_E500")]
pub static mut mmu_psize_defs: [mmu_psize_def; MMU_PAGE_COUNT] = [
    mmu_psize_def { shift: 12, ..mmu_psize_def::default() },
    mmu_psize_def { shift: 21, ..mmu_psize_def::default() },
    mmu_psize_def { shift: 22, ..mmu_psize_def::default() },
    mmu_psize_def { shift: 24, ..mmu_psize_def::default() },
    mmu_psize_def { shift: 26, ..mmu_psize_def::default() },
    mmu_psize_def { shift: 28, ..mmu_psize_def::default() },
    mmu_psize_def { shift: 30, ..mmu_psize_def::default() },
];

#[cfg(feature = "CONFIG_PPC_E500")]
#[inline]
unsafe fn mmu_get_tsize(psize: i32) -> i32 {
    mmu_psize_defs[psize as usize].shift - 10
}

#[cfg(not(feature = "CONFIG_PPC_E500"))]
#[inline]
unsafe fn mmu_get_tsize(_psize: i32) -> i32 {
    /* This isn't used on !Book3E for now */
    0
}

#[cfg(feature = "CONFIG_PPC_8xx")]
pub static mut mmu_psize_defs: [mmu_psize_def; MMU_PAGE_COUNT] = [
    mmu_psize_def { shift: 12, ..mmu_psize_def::default() },
    mmu_psize_def { shift: 14, ..mmu_psize_def::default() },
    mmu_psize_def { shift: 19, ..mmu_psize_def::default() },
    mmu_psize_def { shift: 23, ..mmu_psize_def::default() },
];

#[cfg(feature = "CONFIG_PPC_E500")]
pub static mut next_tlbcam_idx: i32 = 0;

#[cfg(not(feature = "CONFIG_PPC_8xx"))]
pub unsafe fn local_flush_tlb_mm(mm: *mut mm_struct) {
    preempt_disable();
    let pid = (*mm).context.id;
    if pid != MMU_NO_CONTEXT { _tlbil_pid(pid); }
    preempt_enable();
}

#[cfg(not(feature = "CONFIG_PPC_8xx"))]
pub unsafe fn __local_flush_tlb_page(mm: *mut mm_struct, vmaddr: usize, tsize: i32, ind: i32) {
    preempt_disable();
    let pid = if !mm.is_null() { (*mm).context.id } else { 0 };
    if pid != MMU_NO_CONTEXT { _tlbil_va(vmaddr, pid, tsize, ind); }
    preempt_enable();
}

#[cfg(not(feature = "CONFIG_PPC_8xx"))]
pub unsafe fn local_flush_tlb_page(vma: *mut vm_area_struct, vmaddr: usize) {
    __local_flush_tlb_page(if !vma.is_null() { (*vma).vm_mm } else { core::ptr::null_mut() }, vmaddr, mmu_get_tsize(mmu_virtual_psize), 0);
}

#[cfg(not(feature = "CONFIG_PPC_8xx"))]
pub unsafe fn local_flush_tlb_page_psize(mm: *mut mm_struct, vmaddr: usize, psize: i32) {
    __local_flush_tlb_page(mm, vmaddr, mmu_get_tsize(psize), 0);
}

#[cfg(feature = "CONFIG_SMP")]
static mut tlbivax_lock: raw_spinlock_t = raw_spinlock_t::default();

#[cfg(feature = "CONFIG_SMP")]
#[repr(C)]
pub struct tlb_flush_param { pub addr: usize, pub pid: u32, pub tsize: u32, pub ind: u32 }

#[cfg(feature = "CONFIG_SMP")]
unsafe fn do_flush_tlb_mm_ipi(param: *mut core::ffi::c_void) {
    let p = param as *mut tlb_flush_param;
    _tlbil_pid(if !p.is_null() { (*p).pid } else { 0 });
}

#[cfg(feature = "CONFIG_SMP")]
unsafe fn do_flush_tlb_page_ipi(param: *mut core::ffi::c_void) {
    let p = param as *mut tlb_flush_param;
    _tlbil_va((*p).addr, (*p).pid, (*p).tsize as i32, (*p).ind as i32);
}

#[cfg(feature = "CONFIG_SMP")]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    preempt_disable();
    let pid = (*mm).context.id;
    if pid == MMU_NO_CONTEXT { preempt_enable(); return; }
    if !mm_is_core_local(mm) {
        let mut p = tlb_flush_param { addr: 0, pid, tsize: 0, ind: 0 };
        smp_call_function_many(mm_cpumask(mm), do_flush_tlb_mm_ipi, &mut p as *mut _ as *mut _, 1);
    }
    _tlbil_pid(pid);
    preempt_enable();
}

#[cfg(feature = "CONFIG_SMP")]
pub unsafe fn __flush_tlb_page(mm: *mut mm_struct, vmaddr: usize, tsize: i32, ind: i32) {
    if mm.is_null() { return; }
    preempt_disable();
    let pid = (*mm).context.id;
    if pid == MMU_NO_CONTEXT { preempt_enable(); return; }
    let cpu_mask = mm_cpumask(mm);
    if !mm_is_core_local(mm) {
        if mmu_has_feature(MMU_FTR_USE_TLBIVAX_BCAST) {
            let lock = mmu_has_feature(MMU_FTR_LOCK_BCAST_INVAL);
            if lock { raw_spin_lock(&mut tlbivax_lock); }
            _tlbivax_bcast(vmaddr, pid, tsize, ind);
            if lock { raw_spin_unlock(&mut tlbivax_lock); }
            preempt_enable(); return;
        }
        let mut p = tlb_flush_param { addr: vmaddr, pid, tsize: tsize as u32, ind: ind as u32 };
        smp_call_function_many(cpu_mask, do_flush_tlb_page_ipi, &mut p as *mut _ as *mut _, 1);
    }
    _tlbil_va(vmaddr, pid, tsize, ind);
    preempt_enable();
}

#[cfg(feature = "CONFIG_SMP")]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, vmaddr: usize) {
    #[cfg(feature = "CONFIG_HUGETLB_PAGE")]
    if !vma.is_null() && is_vm_hugetlb_page(vma) { flush_hugetlb_page(vma, vmaddr); }
    __flush_tlb_page(if !vma.is_null() { (*vma).vm_mm } else { core::ptr::null_mut() }, vmaddr, mmu_get_tsize(mmu_virtual_psize), 0);
}

#[cfg(not(feature = "CONFIG_PPC_8xx"))]
pub unsafe fn flush_tlb_kernel_range(_start: usize, _end: usize) {
    #[cfg(feature = "CONFIG_SMP")]
    { preempt_disable(); smp_call_function(do_flush_tlb_mm_ipi, core::ptr::null_mut(), 1); _tlbil_pid(0); preempt_enable(); }
    #[cfg(not(feature = "CONFIG_SMP"))]
    { _tlbil_pid(0); }
}

pub unsafe fn flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize) {
    if end.wrapping_sub(start) == PAGE_SIZE && (start & !PAGE_MASK) == 0 { flush_tlb_page(vma, start); }
    else { flush_tlb_mm((*vma).vm_mm); }
}

pub unsafe fn tlb_flush(tlb: *mut mmu_gather) { flush_tlb_mm((*tlb).mm); }

#[cfg(not(feature = "CONFIG_PPC64"))]
pub unsafe fn early_init_mmu() {
    let root = of_get_flat_dt_root();
    if IS_ENABLED(CONFIG_PPC_47x) && IS_ENABLED(CONFIG_SMP) && !of_get_flat_dt_prop(root, "cooperative-partition\0".as_ptr() as *const _, core::ptr::null_mut()).is_null() {
        mmu_clear_feature(MMU_FTR_USE_TLBIVAX_BCAST);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
